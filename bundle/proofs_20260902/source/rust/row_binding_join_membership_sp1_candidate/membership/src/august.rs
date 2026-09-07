//! August capture-time anchor legs, folded into the membership guest.
//!
//! The legs are deliberately CHAINED so a prover cannot satisfy them independently:
//!   memo    -> the binding digest read out of the on-chain action (memo.rs) is the ONLY value the
//!              receipt may hash to; no host-supplied memo digest exists any more
//!   prefix  -> produces contentRoot, which is the ONLY contentRoot the chameleon leg may use
//!   receipt -> parses Y and C from bytes pinned by that memo digest; those exact points, validated
//!              on-curve and in the prime-order subgroup, are the ones the chameleon and trapdoor
//!              legs consume
//!   prefix  -> the proved row's chain fields are compared against the relation's OWN verified
//!              journal, passed in by the caller, never against constants baked in here
//!
//! Base8 is a hardcoded constant (2 Sep audit, finding 1): as a witness it admitted the forgery
//! B8 = m^-1 * C with opening = 0 and trapdoor = 0.
//!
//! Disclosed contingency: the commitment is a chameleon, so either the record existed when the
//! anchor was mined or the trapdoor holder equivocated. The trapdoor leg proves the prover holds
//! that trapdoor, which turns the caveat into an exhibited property rather than an open doubt.

use ark_bn254::Fr;
use ark_ff::{BigInteger, Field, PrimeField};
use sha2::{Digest, Sha256, Sha512};

use crate::MembershipError;

type Result<T> = core::result::Result<T, MembershipError>;

pub const CONTENT_ROOT_DOMAIN: &[u8] = b"ZKMAX_PROFILE_R_CONTENT_ROOT_V1\0";
pub const CONTENT_SCALAR_DOMAIN: &[u8] = b"ZKMAX_PROFILE_R_CONTENT_SCALAR_V1";
pub const NETWORK: &str = "zcash-testnet";
pub const PROFILE: &str = "ZKMAX_PROFILE_R_ZCASH_TERMINAL_V1";
pub const AUGUST_PUBLIC_MAGIC: &[u8; 8] = b"ZBAUGST4";
/// magic, contentRoot, receipt digest, C.x, C.y, previous row's beacon round (BE u64), the row's own
/// verified round (BE u64), trapdoor flag
pub const AUGUST_PUBLIC_BYTES: usize = 8 + 32 + 32 + 32 + 32 + 8 + 8 + 1;

/// BabyJubJub subgroup order L = 2736030358979909402780800718157159386076813972158567259200215660948447373041,
/// little-endian u64 limbs.
const SUBGROUP_L: [u64; 4] = [
    0x677297dc392126f1,
    0xab3eedb83920ee0a,
    0x370a08b6d0302b0b,
    0x060c89ce5c263405,
];

/// Canonical Base8 of the prime-order subgroup (iden3 / circomlib babyjub), little-endian
/// coordinate bytes. Decimal:
///   x = 5299619240641551281634865583518297030282874472190772894086521144482721001553
///   y = 16950150798460657717958625567821834550301663161624707787222815936182638968203
pub const BASE8_X_LE_HEX: &str = "517095bbf6f39328b6e0340501d8b82ac177629de0b2ac4e9b733ed66a7ab70b";
pub const BASE8_Y_LE_HEX: &str = "8b7d2d877a253c4b7733e1b91f05e0fcedf96bd11c2e572549b2a0f703727925";

/// BN254 scalar-field modulus as a 77-digit decimal, for canonical coordinate parsing.
const FR_MODULUS_DEC: &[u8] = b"21888242871839275222246405745257275088548364400416034343698204186575808495617";

const A_COEFF: u64 = 168700;
const D_COEFF: u64 = 168696;

type Pt = (Fr, Fr);

fn add_pt(p: Pt, q: Pt) -> Pt {
    let (x1, y1) = p;
    let (x2, y2) = q;
    let a = Fr::from(A_COEFF);
    let d = Fr::from(D_COEFF);
    let x1x2 = x1 * x2;
    let y1y2 = y1 * y2;
    let dxy = d * x1x2 * y1y2;
    (
        (x1 * y2 + y1 * x2) * (Fr::ONE + dxy).inverse().unwrap(),
        (y1y2 - a * x1x2) * (Fr::ONE - dxy).inverse().unwrap(),
    )
}

fn mul_pt(mut pt: Pt, k: Fr) -> Pt {
    let mut acc: Pt = (Fr::ZERO, Fr::ONE);
    for bit in k.into_bigint().to_bits_le() {
        if bit {
            acc = add_pt(acc, pt);
        }
        pt = add_pt(pt, pt);
    }
    acc
}

fn on_curve(p: Pt) -> bool {
    let (x, y) = p;
    let a = Fr::from(A_COEFF);
    let d = Fr::from(D_COEFF);
    let xx = x * x;
    let yy = y * y;
    a * xx + yy == Fr::ONE + d * xx * yy
}

fn subgroup_l_as_fr() -> Fr {
    let mut le = [0u8; 32];
    for (i, limb) in SUBGROUP_L.iter().enumerate() {
        le[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_le_bytes());
    }
    Fr::from_le_bytes_mod_order(&le)
}

/// A point is acceptable only if it is on the curve and in the prime-order subgroup.
fn validate_point(p: Pt, what: &'static str) -> Result<()> {
    if !on_curve(p) {
        return Err(MembershipError(what));
    }
    if mul_pt(p, subgroup_l_as_fr()) != (Fr::ZERO, Fr::ONE) {
        return Err(MembershipError(what));
    }
    Ok(())
}

/// True when the 32 little-endian bytes encode an integer strictly below L.
fn le_scalar_below_l(le: &[u8]) -> bool {
    if le.len() != 32 {
        return false;
    }
    for i in (0..4).rev() {
        let limb = u64::from_le_bytes(le[i * 8..(i + 1) * 8].try_into().unwrap());
        if limb != SUBGROUP_L[i] {
            return limb < SUBGROUP_L[i];
        }
    }
    false
}

/// Reduce a 64-byte big-endian digest modulo the BabyJubJub subgroup order.
/// Shift-and-subtract long division: the remainder stays below L < 2^251, so
/// `rem*2 + bit` always fits four u64 limbs.
fn reduce_mod_subgroup(be: &[u8; 64]) -> Fr {
    let mut rem = [0u64; 4];
    for byte in be.iter() {
        for shift in (0..8).rev() {
            let bit = (byte >> shift) & 1;
            let mut carry = bit as u64;
            for limb in rem.iter_mut() {
                let next = *limb >> 63;
                *limb = (*limb << 1) | carry;
                carry = next;
            }
            let mut ge = true;
            for i in (0..4).rev() {
                if rem[i] != SUBGROUP_L[i] {
                    ge = rem[i] > SUBGROUP_L[i];
                    break;
                }
            }
            if ge {
                let mut borrow = 0u64;
                for i in 0..4 {
                    let (d, b1) = rem[i].overflowing_sub(SUBGROUP_L[i]);
                    let (d, b2) = d.overflowing_sub(borrow);
                    rem[i] = d;
                    borrow = (b1 as u64) | (b2 as u64);
                }
            }
        }
    }
    let mut le = [0u8; 32];
    for (i, limb) in rem.iter().enumerate() {
        le[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_le_bytes());
    }
    Fr::from_le_bytes_mod_order(&le)
}

fn find(h: &[u8], n: &[u8], from: usize) -> Option<usize> {
    (from..=h.len().saturating_sub(n.len())).find(|&i| &h[i..i + n.len()] == n)
}

fn hex_le_32(hex: &str) -> [u8; 32] {
    let s = hex.as_bytes();
    let nib = |c: u8| -> u8 {
        match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            _ => panic!("fixed constant is not lowercase hex"),
        }
    };
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = (nib(s[2 * i]) << 4) | nib(s[2 * i + 1]);
    }
    out
}

/// Parse `"<marker>"<digits>"` as a CANONICAL decimal field element: no leading zero unless the
/// value is exactly "0", at most 77 digits, and strictly below the BN254 scalar modulus.
/// Returns the value and the closing-quote index.
fn parse_decimal_after(bytes: &[u8], marker: &[u8], from: usize) -> Result<(Fr, usize)> {
    let at = find(bytes, marker, from).ok_or(MembershipError("receipt marker absent"))?;
    let mut i = at + marker.len();
    if bytes.get(i) != Some(&b'"') {
        return Err(MembershipError("receipt coordinate is not quoted"));
    }
    i += 1;
    let start = i;
    loop {
        let c = *bytes.get(i).ok_or(MembershipError("receipt truncated in a coordinate"))?;
        if c == b'"' {
            break;
        }
        if !c.is_ascii_digit() {
            return Err(MembershipError("non-digit inside a receipt coordinate"));
        }
        i += 1;
    }
    let digits = &bytes[start..i];
    if digits.is_empty() {
        return Err(MembershipError("empty receipt coordinate"));
    }
    if digits.len() > 1 && digits[0] == b'0' {
        return Err(MembershipError("receipt coordinate has a leading zero"));
    }
    if digits.len() > FR_MODULUS_DEC.len()
        || (digits.len() == FR_MODULUS_DEC.len() && digits >= FR_MODULUS_DEC)
    {
        return Err(MembershipError("receipt coordinate is not below the field modulus"));
    }
    let mut v = Fr::ZERO;
    let ten = Fr::from(10u64);
    for &c in digits {
        v = v * ten + Fr::from((c - b'0') as u64);
    }
    Ok((v, i))
}

/// Extract the quoted string value following `marker`.
fn parse_string_after<'a>(bytes: &'a [u8], marker: &[u8]) -> Result<&'a [u8]> {
    let at = find(bytes, marker, 0).ok_or(MembershipError("receipt string marker absent"))?;
    let start = at + marker.len();
    let end = find(bytes, b"\"", start).ok_or(MembershipError("unterminated receipt string"))?;
    Ok(&bytes[start..end])
}

/// recordScope must be exactly `sha256:` followed by 64 lowercase hex characters, so the
/// in-circuit canonical JSON never has to escape anything.
fn validate_record_scope(scope: &[u8]) -> Result<()> {
    if scope.len() != 7 + 64 || &scope[..7] != b"sha256:" {
        return Err(MembershipError("recordScope is not a sha256 digest identifier"));
    }
    if !scope[7..].iter().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(c)) {
        return Err(MembershipError("recordScope digest is not lowercase hex"));
    }
    Ok(())
}

fn field_at<'a>(prefix: &'a [u8], row_tag: &[u8], idx: usize) -> Result<&'a [u8]> {
    let mut i = 0usize;
    let (s, e) = loop {
        if i >= prefix.len() {
            return Err(MembershipError("target row absent from the chain-log prefix"));
        }
        let end = prefix[i..]
            .iter()
            .position(|&c| c == b'\n')
            .map(|p| i + p)
            .unwrap_or(prefix.len());
        if prefix[i..end].starts_with(row_tag) {
            break (i, end);
        }
        i = end + 1;
    };
    let line = &prefix[s..e];
    let mut field = 0usize;
    let mut fs = 0usize;
    for (k, &c) in line.iter().enumerate() {
        if c == b',' {
            if field == idx {
                return Ok(&line[fs..k]);
            }
            field += 1;
            fs = k + 1;
        }
    }
    if field != idx {
        return Err(MembershipError("field index beyond the row"));
    }
    let mut end2 = line.len();
    while end2 > fs && line[end2 - 1] == b'\r' {
        end2 -= 1;
    }
    Ok(&line[fs..end2])
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut s = String::new();
    for b in bytes {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 15) as u32, 16).unwrap());
    }
    s
}


fn hex_decode(h: &[u8]) -> Result<Vec<u8>> {
    if h.len() % 2 != 0 {
        return Err(MembershipError("odd-length hex field in the chain-log prefix"));
    }
    let nib = |c: u8| -> Result<u8> {
        match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            _ => Err(MembershipError("chain-log prefix field is not lowercase hex")),
        }
    };
    let mut out = Vec::with_capacity(h.len() / 2);
    for i in 0..h.len() / 2 {
        out.push((nib(h[2 * i])? << 4) | nib(h[2 * i + 1])?);
    }
    Ok(out)
}

fn parse_u64(d: &[u8]) -> Result<u64> {
    if d.is_empty() || d.len() > 19 || (d.len() > 1 && d[0] == b'0') {
        return Err(MembershipError("chain-log round is not a canonical decimal"));
    }
    let mut v: u64 = 0;
    for &c in d {
        if !c.is_ascii_digit() {
            return Err(MembershipError("chain-log round has a non-digit"));
        }
        v = v * 10 + (c - b'0') as u64;
    }
    Ok(v)
}

pub struct PreviousAdvance {
    /// The beacon round folded into S_t at row t-1: the round that bounds the proved frame's
    /// emission from below, verified in circuit.
    pub previous_round: u64,
}

/// PREVIOUS-ROW ADVANCE LEG (2 Sep referee finding). The emission the proved frame saw is derived
/// from S_t, and S_t was produced at row t-1 from that row's beacon. This leg takes row t-1's record
/// out of the anchored chain-log prefix, verifies ITS beacon signature under the quicknet key,
/// requires its randomness to be SHA-256 of that signature, and recomputes S_t from row t-1's
/// state, Bayer digest, metadata, round and value. Only then does the verified beacon bound the
/// proved frame's emission, rather than the frame after it.
pub fn verify_previous_advance(chain_prefix: &[u8], row_index: u32, s_t: &[u8; 32]) -> Result<PreviousAdvance> {
    if row_index == 0 {
        return Err(MembershipError("row 0 has no previous row; its emission derives from the session seed"));
    }
    let tag = format!("{},", row_index - 1);
    let s_prev = hex_decode(field_at(chain_prefix, tag.as_bytes(), 1)?)?;
    let raw_prev = hex_decode(field_at(chain_prefix, tag.as_bytes(), 2)?)?;
    let meta_prev = hex_decode(field_at(chain_prefix, tag.as_bytes(), 8)?)?;
    let round_prev = parse_u64(field_at(chain_prefix, tag.as_bytes(), 10)?)?;
    let value_prev = hex_decode(field_at(chain_prefix, tag.as_bytes(), 11)?)?;
    let sig_prev = hex_decode(field_at(chain_prefix, tag.as_bytes(), 12)?)?;
    let s_prev: [u8; 32] = s_prev.try_into().map_err(|_| MembershipError("previous S_t is not 32 bytes"))?;
    let raw_prev: [u8; 32] = raw_prev.try_into().map_err(|_| MembershipError("previous Bayer digest is not 32 bytes"))?;
    let meta_prev: [u8; zeebeam_b3xof_relation::META_BYTES] =
        meta_prev.try_into().map_err(|_| MembershipError("previous metadata is not 28 bytes"))?;
    let value_prev: [u8; 32] = value_prev.try_into().map_err(|_| MembershipError("previous drand value is not 32 bytes"))?;
    let randomness = zeebeam_row_binding_join::verify_quicknet_beacon(round_prev, &sig_prev)
        .map_err(|_| MembershipError("previous row's beacon signature does not verify"))?;
    if randomness != value_prev {
        return Err(MembershipError("previous row's drand value is not sha256 of its verified signature"));
    }
    let recomputed = zeebeam_b3xof_relation::advance_chain(&s_prev, &raw_prev, &meta_prev, round_prev, &value_prev);
    if recomputed != *s_t {
        return Err(MembershipError("previous row's advance does not produce the proved row's S_t"));
    }
    Ok(PreviousAdvance { previous_round: round_prev })
}

/// Chain state the main relation has already verified; the prefix leg is checked against THESE,
/// never against constants, so the anchor is bound to the row the rest of the proof establishes.
pub struct RowFacts<'a> {
    pub row_tag: &'a [u8],
    pub s_t_hex: &'a [u8],
    pub bayer_hex: &'a [u8],
    pub round_ascii: &'a [u8],
    pub value_hex: &'a [u8],
}

pub struct AugustAnchor {
    pub content_root: [u8; 32],
    pub receipt_digest: [u8; 32],
    pub commitment_x: [u8; 32],
    pub commitment_y: [u8; 32],
    pub trapdoor_proved: bool,
}

/// All legs, chained. `anchored_binding` is the digest the memo leg decrypted from the on-chain
/// action; the receipt must hash to exactly that value.
pub fn verify_august_anchor(
    chain_prefix: &[u8],
    prefix_json: &[u8],
    receipt: &[u8],
    anchored_binding: &[u8; 32],
    opening_le: &[u8],
    trapdoor_le: &[u8],
    facts: &RowFacts<'_>,
) -> Result<AugustAnchor> {
    // ---- leg 1: the prefix, and the proved row's join against the relation's own facts ----
    let prefix_digest = blake3::hash(chain_prefix);
    let recorded = parse_string_after(prefix_json, b"\"chain_log_prefix_blake3\": \"")
        .or_else(|_| parse_string_after(prefix_json, b"\"chain_log_prefix_blake3\":\""))?;
    let got = hex_lower(prefix_digest.as_bytes());
    if got.as_bytes() != recorded {
        return Err(MembershipError("chain-log prefix digest differs from the anchored record"));
    }
    if field_at(chain_prefix, facts.row_tag, 1)? != facts.s_t_hex {
        return Err(MembershipError("anchored prefix row S_t differs from the proved row"));
    }
    if field_at(chain_prefix, facts.row_tag, 2)? != facts.bayer_hex {
        return Err(MembershipError("anchored prefix row Bayer digest differs from the proved row"));
    }
    if field_at(chain_prefix, facts.row_tag, 10)? != facts.round_ascii {
        return Err(MembershipError("anchored prefix row drand round differs from the proved row"));
    }
    if field_at(chain_prefix, facts.row_tag, 11)? != facts.value_hex {
        return Err(MembershipError("anchored prefix row drand value differs from the proved row"));
    }
    let mut h = Sha256::new();
    h.update(CONTENT_ROOT_DOMAIN);
    h.update((prefix_json.len() as u64).to_be_bytes());
    h.update(prefix_json);
    let content_root: [u8; 32] = h.finalize().into();

    // ---- leg 2: the receipt, pinned by the digest the memo leg decrypted from the chain ----
    let receipt_digest: [u8; 32] = Sha256::digest(receipt).into();
    if &receipt_digest != anchored_binding {
        return Err(MembershipError("receipt does not hash to the binding carried in the anchored memo"));
    }
    let (yx, i) = parse_decimal_after(receipt, br#""publicKey":{"x":"#, 0)?;
    let (yy, _) = parse_decimal_after(receipt, br#""y":"#, i)?;
    let (cx, j) = parse_decimal_after(receipt, br#""sameCommitment":{"x":"#, 0)?;
    let (cy, _) = parse_decimal_after(receipt, br#""y":"#, j)?;
    let y: Pt = (yx, yy);
    let c: Pt = (cx, cy);
    validate_point(y, "receipt public key is not a valid subgroup point")?;
    validate_point(c, "receipt commitment is not a valid subgroup point")?;
    let record_scope = parse_string_after(receipt, b"\"recordScope\":\"")?;
    validate_record_scope(record_scope)?;

    // ---- leg 3: the chameleon opening, over the contentRoot leg 1 produced ----
    // canonical statement JSON, keys sorted: contentRoot, network, profile, recordScope
    let mut statement = String::new();
    statement.push_str("{\"contentRoot\":\"sha256:");
    statement.push_str(&hex_lower(&content_root));
    statement.push_str("\",\"network\":\"");
    statement.push_str(NETWORK);
    statement.push_str("\",\"profile\":\"");
    statement.push_str(PROFILE);
    statement.push_str("\",\"recordScope\":\"");
    statement.push_str(core::str::from_utf8(record_scope).map_err(|_| MembershipError("recordScope is not UTF-8"))?);
    statement.push_str("\"}");
    let mut sh = Sha512::new();
    sh.update(CONTENT_SCALAR_DOMAIN);
    sh.update([0u8]);
    sh.update(statement.as_bytes());
    let wide: [u8; 64] = sh.finalize().into();
    let m = reduce_mod_subgroup(&wide);

    let b8: Pt = (
        Fr::from_le_bytes_mod_order(&hex_le_32(BASE8_X_LE_HEX)),
        Fr::from_le_bytes_mod_order(&hex_le_32(BASE8_Y_LE_HEX)),
    );
    if !on_curve(b8) {
        return Err(MembershipError("fixed Base8 constant is not on the curve"));
    }
    if !le_scalar_below_l(opening_le) {
        return Err(MembershipError("chameleon opening is not a canonical subgroup scalar"));
    }
    let r = Fr::from_le_bytes_mod_order(opening_le);
    if add_pt(mul_pt(b8, m), mul_pt(y, r)) != c {
        return Err(MembershipError("chameleon opening does not verify against the anchored commitment"));
    }

    // ---- leg 4: trapdoor knowledge over the SAME Y the receipt pinned ----
    if !le_scalar_below_l(trapdoor_le) {
        return Err(MembershipError("trapdoor is not a canonical subgroup scalar"));
    }
    let td = Fr::from_le_bytes_mod_order(trapdoor_le);
    let trapdoor_proved = td != Fr::ZERO;
    if trapdoor_proved && mul_pt(b8, td) != y {
        return Err(MembershipError("trapdoor does not generate the anchored public key"));
    }

    let mut cx_le = [0u8; 32];
    cx_le.copy_from_slice(&c.0.into_bigint().to_bytes_le());
    let mut cy_le = [0u8; 32];
    cy_le.copy_from_slice(&c.1.into_bigint().to_bytes_le());
    Ok(AugustAnchor { content_root, receipt_digest, commitment_x: cx_le, commitment_y: cy_le, trapdoor_proved })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p_minus_one() -> Vec<u8> {
        b"21888242871839275222246405745257275088548364400416034343698204186575808495616".to_vec()
    }

    #[test]
    fn base8_is_a_valid_subgroup_point() {
        let b8: Pt = (
            Fr::from_le_bytes_mod_order(&hex_le_32(BASE8_X_LE_HEX)),
            Fr::from_le_bytes_mod_order(&hex_le_32(BASE8_Y_LE_HEX)),
        );
        validate_point(b8, "base8").unwrap();
    }

    #[test]
    fn order_two_point_is_rejected_by_the_subgroup_check() {
        let minus_one = -Fr::ONE;
        let p: Pt = (Fr::ZERO, minus_one);
        assert!(on_curve(p));
        assert!(validate_point(p, "order two").is_err());
    }

    #[test]
    fn decimal_parser_requires_canonical_form() {
        let ok = |s: &[u8]| {
            let mut v = b"\"x\":\"".to_vec();
            v.extend_from_slice(s);
            v.push(b'"');
            parse_decimal_after(&v, b"\"x\":", 0)
        };
        assert!(ok(b"0").is_ok());
        assert!(ok(b"7").is_ok());
        assert!(ok(&p_minus_one()).is_ok());
        assert!(ok(b"07").is_err());
        assert!(ok(b"").is_err());
        assert!(ok(FR_MODULUS_DEC).is_err());
        let mut too_long = p_minus_one();
        too_long.push(b'0');
        assert!(ok(&too_long).is_err());
    }

    #[test]
    fn scalars_must_be_below_l() {
        let mut le = [0u8; 32];
        for (i, limb) in SUBGROUP_L.iter().enumerate() {
            le[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_le_bytes());
        }
        assert!(!le_scalar_below_l(&le));
        le[0] -= 1;
        assert!(le_scalar_below_l(&le));
        assert!(le_scalar_below_l(&[0u8; 32]));
        assert!(!le_scalar_below_l(&[0xffu8; 32]));
        assert!(!le_scalar_below_l(&[0u8; 31]));
    }

    #[test]
    fn record_scope_format_is_enforced() {
        let good = format!("sha256:{}", "ab".repeat(32));
        assert!(validate_record_scope(good.as_bytes()).is_ok());
        assert!(validate_record_scope(b"sha256:abc").is_err());
        let upper = format!("sha256:{}", "AB".repeat(32));
        assert!(validate_record_scope(upper.as_bytes()).is_err());
        let escaped = format!("sha256:{}\\\"", "ab".repeat(31));
        assert!(validate_record_scope(escaped.as_bytes()).is_err());
    }
}
