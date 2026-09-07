//! Zcash anchor-inclusion leg for the August capture-time anchor (2 Sep audit, finding 6).
//!
//! The statement it enforces: the supplied transaction is a fully shielded v6 transaction on the
//! pinned network carrying an Ironwood bundle and no other bundle; its ZIP-244 txid (five-digest
//! tree, read from zcash_primitives 0.30.0 / orchard 0.15.3) sits under the supplied header's
//! merkle root via the supplied branch; and the header hashes to the returned value. The Ironwood
//! actions are handed back so the memo leg decrypts exactly the bytes that were hashed into the
//! txid. Chain context (that this header is on canonical mainnet) is the verifier's out-of-band
//! check against the published hash.
//!
//! v6 facts a guess gets wrong: an ABSENT transparent bundle digests as the bare personalisation
//! with no inner digests; the bundle anchor is EXCLUDED from the v6 txid digest (it is included
//! only for OrchardV5, where it lives in the authorising digest).

use blake2b_simd::Params;
use sha2::{Digest, Sha256};

use crate::memo::IronwoodAction;
use crate::MembershipError;

pub const EXPECTED_VERSION: u32 = 0x8000_0006;
pub const EXPECTED_VERSION_GROUP: u32 = 0xd884_b698;
pub const EXPECTED_BRANCH: u32 = 0x37a5_165b;
const MAX_TX_BYTES: usize = 1_000_000;
const MAX_ACTIONS: u64 = 64;
const MAX_PROOF_BYTES: u64 = 1_000_000;
const MAX_BRANCH_LEVELS: usize = 64;
pub const HEADER_BYTES: usize = 1487;
pub const ZCASH_PUBLIC_MAGIC: &[u8; 8] = b"ZBZCINC3";
pub const ZCASH_PUBLIC_BYTES: usize = 8 + 32 + 32;

type Result<T> = core::result::Result<T, MembershipError>;

pub struct AnchorInclusion {
    pub txid: [u8; 32],
    pub header_hash: [u8; 32],
    pub ironwood_actions: Vec<IronwoodAction>,
}

fn b2(person: &[u8; 16], data: &[u8]) -> [u8; 32] {
    let mut out = [0_u8; 32];
    out.copy_from_slice(Params::new().hash_length(32).personal(person).hash(data).as_bytes());
    out
}

fn sha256d(data: &[u8]) -> [u8; 32] {
    let first: [u8; 32] = Sha256::digest(data).into();
    Sha256::digest(first).into()
}

struct Reader<'a> {
    b: &'a [u8],
    o: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize, what: &'static str) -> Result<&'a [u8]> {
        let end = self
            .o
            .checked_add(n)
            .filter(|end| *end <= self.b.len())
            .ok_or(MembershipError(what))?;
        let s = &self.b[self.o..end];
        self.o = end;
        Ok(s)
    }
    fn u32le(&mut self, what: &'static str) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4, what)?.try_into().unwrap()))
    }
    fn compact(&mut self, what: &'static str) -> Result<u64> {
        let tag = self.take(1, what)?[0];
        match tag {
            0..=0xfc => Ok(tag as u64),
            0xfd => {
                let v = u16::from_le_bytes(self.take(2, what)?.try_into().unwrap()) as u64;
                if v < 0xfd {
                    return Err(MembershipError("non-minimal CompactSize"));
                }
                Ok(v)
            }
            0xfe => {
                let v = u32::from_le_bytes(self.take(4, what)?.try_into().unwrap()) as u64;
                if v <= 0xffff {
                    return Err(MembershipError("non-minimal CompactSize"));
                }
                Ok(v)
            }
            _ => {
                let v = u64::from_le_bytes(self.take(8, what)?.try_into().unwrap());
                if v <= 0xffff_ffff {
                    return Err(MembershipError("non-minimal CompactSize"));
                }
                Ok(v)
            }
        }
    }
}

struct BundleDigests {
    compact: [u8; 32],
    memo: [u8; 32],
    noncompact: [u8; 32],
    flags: u8,
    value_balance: [u8; 8],
}

/// Parse a present v6 Ironwood bundle, computing the three action digests the txid tree uses
/// and returning the raw actions for the memo leg.
fn read_ironwood_bundle(r: &mut Reader<'_>, n: u64) -> Result<(BundleDigests, Vec<IronwoodAction>)> {
    if n > MAX_ACTIONS {
        return Err(MembershipError("anchor action count exceeds cap"));
    }
    let mut ch = Params::new().hash_length(32).personal(b"ZTxIdIrnActCH_v6").to_state();
    let mut mh = Params::new().hash_length(32).personal(b"ZTxIdIrnActMH_v6").to_state();
    let mut nh = Params::new().hash_length(32).personal(b"ZTxIdIrnActNH_v6").to_state();
    let mut actions = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let cv = r.take(32, "action cv")?;
        let nf = r.take(32, "action nullifier")?;
        let rk = r.take(32, "action rk")?;
        let cmx = r.take(32, "action cmx")?;
        let epk = r.take(32, "action ephemeral key")?;
        let enc = r.take(580, "action enc_ciphertext")?;
        let out = r.take(80, "action out_ciphertext")?;
        ch.update(nf);
        ch.update(cmx);
        ch.update(epk);
        ch.update(&enc[..52]);
        mh.update(&enc[52..564]);
        nh.update(cv);
        nh.update(rk);
        nh.update(&enc[564..]);
        nh.update(out);
        let mut a = IronwoodAction {
            cv: [0; 32], nf: [0; 32], rk: [0; 32], cmx: [0; 32], epk: [0; 32], enc: [0; 580], out: [0; 80],
        };
        a.cv.copy_from_slice(cv);
        a.nf.copy_from_slice(nf);
        a.rk.copy_from_slice(rk);
        a.cmx.copy_from_slice(cmx);
        a.epk.copy_from_slice(epk);
        a.enc.copy_from_slice(enc);
        a.out.copy_from_slice(out);
        actions.push(a);
    }
    let flags = r.take(1, "bundle flags")?[0];
    let mut value_balance = [0u8; 8];
    value_balance.copy_from_slice(r.take(8, "bundle value balance")?);
    let _anchor = r.take(32, "bundle anchor")?; // v6: NOT in the txid digest
    let proof_len = r.compact("proof length")?;
    if proof_len > MAX_PROOF_BYTES {
        return Err(MembershipError("bundle proof length exceeds cap"));
    }
    r.take(proof_len as usize, "bundle proof")?;
    for _ in 0..n {
        r.take(64, "spend auth signature")?;
    }
    r.take(64, "binding signature")?;
    let mut compact = [0u8; 32];
    compact.copy_from_slice(ch.finalize().as_bytes());
    let mut memo = [0u8; 32];
    memo.copy_from_slice(mh.finalize().as_bytes());
    let mut noncompact = [0u8; 32];
    noncompact.copy_from_slice(nh.finalize().as_bytes());
    Ok((BundleDigests { compact, memo, noncompact, flags, value_balance }, actions))
}

fn ironwood_bundle_digest(b: &BundleDigests) -> [u8; 32] {
    let mut h = Params::new().hash_length(32).personal(b"ZTxIdIronwd_H_v6").to_state();
    h.update(&b.compact);
    h.update(&b.memo);
    h.update(&b.noncompact);
    h.update(&[b.flags]);
    h.update(&b.value_balance);
    let mut out = [0u8; 32];
    out.copy_from_slice(h.finalize().as_bytes());
    out
}

/// ZIP-244 txid of a fully shielded, Ironwood-only v6 transaction on the pinned network.
fn shielded_txid(raw: &[u8]) -> Result<([u8; 32], Vec<IronwoodAction>)> {
    if raw.len() > MAX_TX_BYTES {
        return Err(MembershipError("anchor transaction too large"));
    }
    let mut r = Reader { b: raw, o: 0 };
    if r.u32le("version")? != EXPECTED_VERSION {
        return Err(MembershipError("anchor is not a v6 transaction"));
    }
    if r.u32le("version group")? != EXPECTED_VERSION_GROUP {
        return Err(MembershipError("anchor version group id differs"));
    }
    if r.u32le("branch id")? != EXPECTED_BRANCH {
        return Err(MembershipError("anchor consensus branch id differs"));
    }
    r.u32le("lock time")?;
    r.u32le("expiry height")?;
    let header_bytes = &raw[0..20];
    let branch_le: [u8; 4] = raw[8..12].try_into().unwrap();

    if r.compact("input count")? != 0 || r.compact("output count")? != 0 {
        return Err(MembershipError("anchor must carry no transparent inputs or outputs"));
    }
    if r.compact("sapling spend count")? != 0 || r.compact("sapling output count")? != 0 {
        return Err(MembershipError("anchor must carry no Sapling bundle"));
    }
    if r.compact("orchard action count")? != 0 {
        return Err(MembershipError("anchor must carry no Orchard V5 bundle"));
    }
    let n_iron = r.compact("ironwood action count")?;
    if n_iron == 0 {
        return Err(MembershipError("anchor must carry an Ironwood bundle"));
    }
    let (iron, actions) = read_ironwood_bundle(&mut r, n_iron)?;
    if r.o != raw.len() {
        return Err(MembershipError("trailing bytes after the anchor transaction"));
    }

    let mut person = [0_u8; 16];
    person[..12].copy_from_slice(b"ZcashTxHash_");
    person[12..].copy_from_slice(&branch_le);
    let txid = b2(
        &person,
        &[
            b2(b"ZTxIdHeadersHash", header_bytes),
            b2(b"ZTxIdTranspaHash", b""), // absent: bare personalisation
            b2(b"ZTxIdSaplingHash", b""),
            b2(b"ZTxIdOrchardH_v6", b""),
            ironwood_bundle_digest(&iron),
        ]
        .concat(),
    );
    Ok((txid, actions))
}

pub fn verify_anchor_inclusion(raw_tx: &[u8], branch: &[u8], header: &[u8]) -> Result<AnchorInclusion> {
    if header.len() != HEADER_BYTES {
        return Err(MembershipError("Zcash header must be 1487 bytes"));
    }
    if branch.len() % 33 != 0 || branch.len() / 33 > MAX_BRANCH_LEVELS {
        return Err(MembershipError("merkle branch malformed"));
    }
    let (txid, ironwood_actions) = shielded_txid(raw_tx)?;
    let mut node = txid;
    for level in branch.chunks_exact(33) {
        let sibling: [u8; 32] = level[..32].try_into().unwrap();
        let direction = level[32];
        if direction > 1 {
            return Err(MembershipError("branch direction byte must be 0 or 1"));
        }
        let mut cat = [0_u8; 64];
        if direction == 1 {
            cat[..32].copy_from_slice(&sibling);
            cat[32..].copy_from_slice(&node);
        } else {
            cat[..32].copy_from_slice(&node);
            cat[32..].copy_from_slice(&sibling);
        }
        node = sha256d(&cat);
    }
    if node[..] != header[36..68] {
        return Err(MembershipError("merkle branch does not reach the header root"));
    }
    Ok(AnchorInclusion { txid, header_hash: sha256d(header), ironwood_actions })
}
