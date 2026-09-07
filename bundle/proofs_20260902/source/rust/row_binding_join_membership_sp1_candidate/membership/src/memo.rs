//! MEMO leg: decrypt the anchor's Ironwood action under the holder's external Orchard incoming
//! viewing key (a private witness) and read the `binding=` digest out of the memo.
//!
//! The orchard crate is the reference implementation and runs here unmodified. What
//! `try_note_decryption` enforces (zcash_note_encryption 0.4.2, read rather than assumed): the
//! ephemeral key parses to a non-identity Pallas point; shared = ivk * epk; key = KDF^Orchard(shared,
//! epk); the ChaCha20-Poly1305 tag verifies over the 564-byte plaintext; the plaintext parses as a
//! V3 note; the recomputed note commitment equals the action's on-chain cmx; and derive_esk(note)
//! regenerates epk. The cmx check is what makes this leg key-committing: a second viewing key
//! cannot produce a plaintext whose note recomputes the committed cmx, so the memo bytes are
//! determined by the on-chain action alone once the transaction is pinned by its txid.

use orchard::keys::{IncomingViewingKey, PreparedIncomingViewingKey};
use orchard::note::{ExtractedNoteCommitment, Nullifier, TransmittedNoteCiphertext};
use orchard::note_encryption::IronwoodDomain;
use orchard::primitives::redpallas::{SpendAuth, VerificationKey};
use orchard::value::ValueCommitment;
use orchard::Action;

use crate::MembershipError;

type Result<T> = core::result::Result<T, MembershipError>;

/// Wire fields of one v6 Ironwood action, in transaction order.
pub struct IronwoodAction {
    pub cv: [u8; 32],
    pub nf: [u8; 32],
    pub rk: [u8; 32],
    pub cmx: [u8; 32],
    pub epk: [u8; 32],
    pub enc: [u8; 580],
    pub out: [u8; 80],
}

pub struct MemoBinding {
    /// The 32-byte digest written after `binding=` in the memo.
    pub binding: [u8; 32],
    /// The decrypted note value in zatoshi, reported for the record.
    pub value: u64,
}

fn hex_nibble(c: u8) -> Result<u8> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => Err(MembershipError("memo binding digest is not lowercase hex")),
    }
}

/// Decrypt `action` under `ivk_bytes` (64 bytes: dk || ivk) and extract the binding digest.
pub fn decrypt_binding(action: &IronwoodAction, ivk_bytes: &[u8]) -> Result<MemoBinding> {
    let ivk_arr: [u8; 64] = ivk_bytes
        .try_into()
        .map_err(|_| MembershipError("incoming viewing key must be 64 bytes"))?;
    let ivk = Option::<IncomingViewingKey>::from(IncomingViewingKey::from_bytes(&ivk_arr))
        .ok_or(MembershipError("incoming viewing key bytes are invalid"))?;
    let prepared = PreparedIncomingViewingKey::new(&ivk);

    let nf = Option::<Nullifier>::from(Nullifier::from_bytes(&action.nf))
        .ok_or(MembershipError("action nullifier is not a valid field element"))?;
    let rk = VerificationKey::<SpendAuth>::try_from(action.rk)
        .map_err(|_| MembershipError("action rk is not a valid verification key"))?;
    let cmx = Option::<ExtractedNoteCommitment>::from(ExtractedNoteCommitment::from_bytes(&action.cmx))
        .ok_or(MembershipError("action cmx is not a valid field element"))?;
    let cv = Option::<ValueCommitment>::from(ValueCommitment::from_bytes(&action.cv))
        .ok_or(MembershipError("action cv is not a valid point"))?;
    let enc = TransmittedNoteCiphertext {
        epk_bytes: action.epk,
        enc_ciphertext: action.enc,
        out_ciphertext: action.out,
    };
    let parsed: Action<()> = Action::from_parts(nf, rk, cmx, enc, cv, ())
        .map_err(|_| MembershipError("action fields do not form a valid Ironwood action"))?;

    let domain = IronwoodDomain::for_action(&parsed);
    let (note, _recipient, memo) = zcash_note_encryption::try_note_decryption(&domain, &prepared, &parsed)
        .ok_or(MembershipError("anchor action does not decrypt under the holder viewing key"))?;

    let m: &[u8] = &memo[..];
    let marker = b"binding=";
    let at = (0..=m.len() - marker.len())
        .find(|&i| &m[i..i + marker.len()] == marker)
        .ok_or(MembershipError("anchor memo carries no binding field"))?;
    let start = at + marker.len();
    if start + 64 > m.len() {
        return Err(MembershipError("anchor memo binding field is truncated"));
    }
    let hex = &m[start..start + 64];
    let mut binding = [0u8; 32];
    for i in 0..32 {
        binding[i] = (hex_nibble(hex[2 * i])? << 4) | hex_nibble(hex[2 * i + 1])?;
    }
    Ok(MemoBinding { binding, value: note.value().inner() })
}
