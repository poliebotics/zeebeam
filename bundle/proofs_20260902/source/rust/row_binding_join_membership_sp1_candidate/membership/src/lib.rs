//! Ordered-session membership wrapper for the frozen ZeeBeam row join.
//!
//! The wrapper consumes the v1 row evaluator internally, verifies one private
//! ordered-tree opening, and emits a privacy-aligned v2 public descriptor plus
//! the unchanged 344-byte scorer block. The row state, digests, metadata,
//! drand values, and sibling path remain private.
//!
//! The committed root is post-capture and issuer-derived. This relation
//! enforces membership under that supplied root. It does not prove honest
//! whole-log construction, capture time, chronology, liveness, or reality.

use core::fmt;

use zeebeam_b3xof_relation::blake3p;
use zeebeam_row_binding_join as row_join;

pub const CLASSIFICATION: &str = "CANDIDATE_ORDERED_MEMBERSHIP";
pub const ROW_MEMBERSHIP_RELATION_ENFORCED: bool = true;
pub const SESSION_IDENTIFIER_IN_RELATION: bool = true;
pub const AUTHORITY_MANIFEST_DIGEST_IN_CONTEXT: bool = true;
pub const CHAIN_LOG_DIGEST_IN_CONTEXT: bool = true;

pub const PRIVATE_MAGIC: &[u8; 8] = b"ZBOSM001";
pub const PRIVATE_ABI_VERSION: u8 = 1;
pub const PRIVATE_FIXED_BYTES: usize = 184;
pub const SESSION_ID_MAX_BYTES: usize = 128;
pub const MAX_TREE_DEPTH: usize = 20;
pub const MAX_ROW_COUNT: u32 = 1_000_000;

pub const PUBLIC_MAGIC: &[u8; 8] = b"ZBROWM02";
pub const PUBLIC_ABI_VERSION: u8 = 3; // v3: base journal carries pose + beacon blocks (audit 1 Sep)
pub mod august;
pub mod memo;
pub mod zcash;

pub const PUBLIC_PREFIX_BYTES: usize = 352;
pub const PUBLIC_BYTES: usize =
    PUBLIC_PREFIX_BYTES + 344 + row_join::POSE_PUBLIC_BYTES + row_join::BEACON_PUBLIC_BYTES;

pub const CONTEXT_DOMAIN: &[u8] = b"ZEEBEAM_ORDERED_SESSION_CONTEXT_V1\0";
pub const ROW_DOMAIN: &[u8] = b"ZEEBEAM_ORDERED_SESSION_ROW_V1\0";
pub const NODE_DOMAIN: &[u8] = b"ZEEBEAM_ORDERED_SESSION_NODE_V1\0";
pub const ROOT_DOMAIN: &[u8] = b"ZEEBEAM_ORDERED_SESSION_ROOT_V1\0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MembershipError(pub(crate) &'static str);

impl MembershipError {
    pub const fn message(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for MembershipError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for MembershipError {}

pub type Result<T> = core::result::Result<T, MembershipError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MembershipWitness {
    pub protocol_code: u8,
    pub terminal_committed: u8,
    pub tree_depth: u16,
    pub row_count: u32,
    pub session_id: Vec<u8>,
    pub s_0: [u8; 32],
    pub s_n: [u8; 32],
    pub authority_manifest_sha256: [u8; 32],
    pub chain_log_blake3: [u8; 32],
    pub ordered_session_root_blake3: [u8; 32],
    pub siblings_blake3: Vec<[u8; 32]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MembershipDigests {
    pub context_digest_blake3: [u8; 32],
    pub row_leaf_blake3: [u8; 32],
    pub internal_root_blake3: [u8; 32],
    pub wrapped_root_blake3: [u8; 32],
}

#[derive(Clone, Copy)]
struct BaseRowPublic {
    row_index: u32,
    s_t: [u8; 32],
    s_next: [u8; 32],
    raw_blake3: [u8; 32],
    emission_blake3: [u8; 32],
    meta: [u8; 28],
    drand_round_be: [u8; 8],
    drand_value: [u8; 32],
}

fn exact_32(bytes: &[u8]) -> [u8; 32] {
    bytes.try_into().expect("fixed 32-byte slice")
}

fn identifier_byte(value: u8, first: bool) -> bool {
    value.is_ascii_alphanumeric() || (!first && matches!(value, b'.' | b'_' | b':' | b'-'))
}

fn validate_identifier(value: &[u8]) -> Result<()> {
    if value.is_empty() || value.len() > SESSION_ID_MAX_BYTES {
        return Err(MembershipError(
            "session identifier length is outside 1..128",
        ));
    }
    if !identifier_byte(value[0], true)
        || value[1..]
            .iter()
            .copied()
            .any(|byte| !identifier_byte(byte, false))
    {
        return Err(MembershipError("session identifier is not canonical ASCII"));
    }
    Ok(())
}

pub fn tree_depth_for_count(row_count: u32) -> Result<u16> {
    if row_count == 0 || row_count > MAX_ROW_COUNT {
        return Err(MembershipError("row count is outside 1..1000000"));
    }
    let mut remaining = row_count - 1;
    let mut depth = 0_u16;
    while remaining != 0 {
        depth += 1;
        remaining >>= 1;
    }
    if usize::from(depth) > MAX_TREE_DEPTH {
        return Err(MembershipError("minimal tree depth exceeds ABI ceiling"));
    }
    Ok(depth)
}

fn domain_hash(domain: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = blake3p::Hasher::new();
    hasher.update(domain);
    for part in parts {
        let length = u32::try_from(part.len()).expect("membership field length exceeds u32");
        hasher.update(&length.to_be_bytes());
        hasher.update(part);
    }
    hasher.finalize()
}

fn validate_witness_shape(witness: &MembershipWitness) -> Result<()> {
    if witness.protocol_code != row_join::PROTOCOL_V9_CODE {
        return Err(MembershipError("membership protocol must be TB-v0.9"));
    }
    if witness.terminal_committed != 1 {
        return Err(MembershipError("terminal committed must be one"));
    }
    validate_identifier(&witness.session_id)?;
    let expected_depth = tree_depth_for_count(witness.row_count)?;
    if witness.tree_depth != expected_depth {
        return Err(MembershipError("tree depth is not minimal for row count"));
    }
    if witness.siblings_blake3.len() != usize::from(witness.tree_depth) {
        return Err(MembershipError("sibling count differs from tree depth"));
    }
    Ok(())
}

impl MembershipWitness {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < PRIVATE_FIXED_BYTES {
            return Err(MembershipError(
                "membership witness is shorter than fixed header",
            ));
        }
        if &bytes[0..8] != PRIVATE_MAGIC {
            return Err(MembershipError("membership witness magic differs"));
        }
        if bytes[8] != PRIVATE_ABI_VERSION {
            return Err(MembershipError("membership private ABI version differs"));
        }
        if bytes[11] != 0 {
            return Err(MembershipError("membership reserved byte must be zero"));
        }
        let declared_length = usize::try_from(u32::from_le_bytes(
            bytes[12..16].try_into().expect("fixed length"),
        ))
        .expect("u32 length does not fit usize");
        let session_length = usize::from(u16::from_le_bytes(
            bytes[16..18].try_into().expect("fixed session length"),
        ));
        let tree_depth = u16::from_le_bytes(bytes[18..20].try_into().expect("fixed depth"));
        if session_length == 0 || session_length > SESSION_ID_MAX_BYTES {
            return Err(MembershipError(
                "session identifier length is outside 1..128",
            ));
        }
        if usize::from(tree_depth) > MAX_TREE_DEPTH {
            return Err(MembershipError("tree depth exceeds ABI ceiling"));
        }
        let expected_length = PRIVATE_FIXED_BYTES
            .checked_add(session_length)
            .and_then(|value| value.checked_add(usize::from(tree_depth) * 32))
            .ok_or(MembershipError("membership witness length overflows"))?;
        if bytes.len() != expected_length || declared_length != expected_length {
            return Err(MembershipError("membership witness length differs"));
        }

        let siblings_start = PRIVATE_FIXED_BYTES + session_length;
        let siblings = bytes[siblings_start..]
            .chunks_exact(32)
            .map(exact_32)
            .collect::<Vec<_>>();
        let witness = Self {
            protocol_code: bytes[9],
            terminal_committed: bytes[10],
            tree_depth,
            row_count: u32::from_le_bytes(bytes[20..24].try_into().expect("fixed row count")),
            s_0: exact_32(&bytes[24..56]),
            s_n: exact_32(&bytes[56..88]),
            authority_manifest_sha256: exact_32(&bytes[88..120]),
            chain_log_blake3: exact_32(&bytes[120..152]),
            ordered_session_root_blake3: exact_32(&bytes[152..184]),
            session_id: bytes[184..184 + session_length].to_vec(),
            siblings_blake3: siblings,
        };
        validate_witness_shape(&witness)?;
        Ok(witness)
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        validate_witness_shape(self)?;
        let total_length =
            PRIVATE_FIXED_BYTES + self.session_id.len() + self.siblings_blake3.len() * 32;
        let mut bytes = Vec::with_capacity(total_length);
        bytes.extend_from_slice(PRIVATE_MAGIC);
        bytes.push(PRIVATE_ABI_VERSION);
        bytes.push(self.protocol_code);
        bytes.push(self.terminal_committed);
        bytes.push(0);
        bytes.extend_from_slice(
            &u32::try_from(total_length)
                .expect("membership witness exceeds u32")
                .to_le_bytes(),
        );
        bytes.extend_from_slice(
            &u16::try_from(self.session_id.len())
                .expect("session identifier exceeds u16")
                .to_le_bytes(),
        );
        bytes.extend_from_slice(&self.tree_depth.to_le_bytes());
        bytes.extend_from_slice(&self.row_count.to_le_bytes());
        bytes.extend_from_slice(&self.s_0);
        bytes.extend_from_slice(&self.s_n);
        bytes.extend_from_slice(&self.authority_manifest_sha256);
        bytes.extend_from_slice(&self.chain_log_blake3);
        bytes.extend_from_slice(&self.ordered_session_root_blake3);
        bytes.extend_from_slice(&self.session_id);
        for sibling in &self.siblings_blake3 {
            bytes.extend_from_slice(sibling);
        }
        assert_eq!(bytes.len(), total_length);
        Ok(bytes)
    }
}

pub fn session_context_digest(witness: &MembershipWitness) -> Result<[u8; 32]> {
    validate_witness_shape(witness)?;
    let protocol = match witness.protocol_code {
        row_join::PROTOCOL_V9_CODE => b"TB-v0.9".as_slice(),
        _ => return Err(MembershipError("membership protocol must be TB-v0.9")),
    };
    let row_count = witness.row_count.to_be_bytes();
    let terminal = [witness.terminal_committed];
    Ok(domain_hash(
        CONTEXT_DOMAIN,
        &[
            protocol,
            &witness.session_id,
            &row_count,
            &terminal,
            &witness.s_0,
            &witness.s_n,
            &witness.authority_manifest_sha256,
            &witness.chain_log_blake3,
        ],
    ))
}

fn parse_base_public(bytes: &[u8], row_count: u32) -> Result<BaseRowPublic> {
    if bytes.len() != row_join::PUBLIC_BYTES {
        return Err(MembershipError(
            "base row journal length differs from the v2 layout",
        ));
    }
    if &bytes[0..8] != row_join::PUBLIC_MAGIC
        || bytes[8] != row_join::PUBLIC_ABI_VERSION
        || bytes[9] != row_join::PROTOCOL_V9_CODE
    {
        return Err(MembershipError("base row journal framing differs"));
    }
    if bytes[10..12] != [0; 2]
        || bytes[324..328] != [0; 4]
        || u32::from_le_bytes(bytes[12..16].try_into().expect("fixed public length"))
            != row_join::PUBLIC_BYTES as u32
    {
        return Err(MembershipError(
            "base row journal reserved or length fields differ",
        ));
    }
    if u32::from_le_bytes(bytes[20..24].try_into().expect("fixed raw length")) != 24_472_000
        || u32::from_le_bytes(bytes[24..28].try_into().expect("fixed emission length")) != 6_220_800
        || u32::from_le_bytes(bytes[28..32].try_into().expect("fixed primary length")) != 458_752
    {
        return Err(MembershipError("base row journal tensor lengths differ"));
    }
    let row_index = u32::from_le_bytes(bytes[16..20].try_into().expect("fixed row index"));
    if row_index >= row_count {
        return Err(MembershipError(
            "base row index is outside committed session",
        ));
    }
    let meta: [u8; 28] = bytes[256..284].try_into().expect("fixed metadata");
    if u32::from_be_bytes(meta[0..4].try_into().expect("fixed metadata row")) != row_index {
        return Err(MembershipError(
            "base metadata row differs from public row index",
        ));
    }
    Ok(BaseRowPublic {
        row_index,
        s_t: exact_32(&bytes[32..64]),
        s_next: exact_32(&bytes[64..96]),
        raw_blake3: exact_32(&bytes[96..128]),
        emission_blake3: exact_32(&bytes[128..160]),
        meta,
        drand_round_be: bytes[284..292].try_into().expect("fixed drand round"),
        drand_value: exact_32(&bytes[292..324]),
    })
}

fn row_leaf_hash(context: &[u8; 32], row: &BaseRowPublic) -> [u8; 32] {
    let row_index = row.row_index.to_be_bytes();
    domain_hash(
        ROW_DOMAIN,
        &[
            context,
            &row_index,
            &row.s_t,
            &row.raw_blake3,
            &row.meta,
            &row.drand_round_be,
            &row.drand_value,
            &row.s_next,
            &row.emission_blake3,
        ],
    )
}

fn node_hash(
    context: &[u8; 32],
    level: u16,
    parent_index: u32,
    left: &[u8; 32],
    right: &[u8; 32],
) -> [u8; 32] {
    let level = level.to_be_bytes();
    let parent_index = parent_index.to_be_bytes();
    domain_hash(NODE_DOMAIN, &[context, &level, &parent_index, left, right])
}

fn wrapped_root_hash(
    context: &[u8; 32],
    row_count: u32,
    tree_depth: u16,
    internal_root: &[u8; 32],
) -> [u8; 32] {
    let row_count = row_count.to_be_bytes();
    let tree_depth = tree_depth.to_be_bytes();
    domain_hash(
        ROOT_DOMAIN,
        &[context, &row_count, &tree_depth, internal_root],
    )
}

pub fn derive_membership_digests(
    base_public: &[u8],
    witness: &MembershipWitness,
) -> Result<MembershipDigests> {
    validate_witness_shape(witness)?;
    let context = session_context_digest(witness)?;
    let row = parse_base_public(base_public, witness.row_count)?;
    let leaf = row_leaf_hash(&context, &row);
    let mut current = leaf;
    let mut position = row.row_index;
    for (zero_level, sibling) in witness.siblings_blake3.iter().enumerate() {
        let parent_index = position / 2;
        let level = u16::try_from(zero_level + 1).expect("tree depth exceeds u16");
        current = if position & 1 == 1 {
            node_hash(&context, level, parent_index, sibling, &current)
        } else {
            node_hash(&context, level, parent_index, &current, sibling)
        };
        position = parent_index;
    }
    let wrapped = wrapped_root_hash(&context, witness.row_count, witness.tree_depth, &current);
    Ok(MembershipDigests {
        context_digest_blake3: context,
        row_leaf_blake3: leaf,
        internal_root_blake3: current,
        wrapped_root_blake3: wrapped,
    })
}

fn encode_public(
    base_public: &[u8],
    row_index: u32,
    witness: &MembershipWitness,
    context_digest: &[u8; 32],
) -> Vec<u8> {
    let mut public = Vec::with_capacity(PUBLIC_BYTES);
    public.extend_from_slice(PUBLIC_MAGIC);
    public.push(PUBLIC_ABI_VERSION);
    public.push(witness.protocol_code);
    public.push(witness.terminal_committed);
    public.push(0);
    public.extend_from_slice(&(PUBLIC_BYTES as u32).to_le_bytes());
    public.extend_from_slice(&row_index.to_le_bytes());
    public.extend_from_slice(&witness.row_count.to_le_bytes());
    public.extend_from_slice(&witness.tree_depth.to_le_bytes());
    public.extend_from_slice(
        &u16::try_from(witness.session_id.len())
            .expect("session identifier exceeds u16")
            .to_le_bytes(),
    );
    public.extend_from_slice(&witness.session_id);
    public.resize(156, 0);
    public.extend_from_slice(&witness.s_0);
    public.extend_from_slice(&witness.s_n);
    public.extend_from_slice(&witness.authority_manifest_sha256);
    public.extend_from_slice(&witness.chain_log_blake3);
    public.extend_from_slice(context_digest);
    public.extend_from_slice(&witness.ordered_session_root_blake3);
    public.extend_from_slice(&[0_u8; 4]);
    assert_eq!(public.len(), PUBLIC_PREFIX_BYTES);
    public.extend_from_slice(&base_public[row_join::PUBLIC_PREFIX_BYTES..]);
    assert_eq!(public.len(), PUBLIC_BYTES);
    public
}

pub fn verify_and_encode(base_public: &[u8], witness: &MembershipWitness) -> Result<Vec<u8>> {
    let digests = derive_membership_digests(base_public, witness)?;
    if digests.wrapped_root_blake3 != witness.ordered_session_root_blake3 {
        return Err(MembershipError("ordered session opening does not verify"));
    }
    let row_index = u32::from_le_bytes(base_public[16..20].try_into().expect("fixed row index"));
    Ok(encode_public(
        base_public,
        row_index,
        witness,
        &digests.context_digest_blake3,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_public() -> Vec<u8> {
        let mut public = vec![0_u8; row_join::PUBLIC_BYTES];
        public[0..8].copy_from_slice(row_join::PUBLIC_MAGIC);
        public[8] = row_join::PUBLIC_ABI_VERSION;
        public[9] = row_join::PROTOCOL_V9_CODE;
        public[12..16].copy_from_slice(&(row_join::PUBLIC_BYTES as u32).to_le_bytes());
        public[16..20].copy_from_slice(&96_u32.to_le_bytes());
        public[20..24].copy_from_slice(&24_472_000_u32.to_le_bytes());
        public[24..28].copy_from_slice(&6_220_800_u32.to_le_bytes());
        public[28..32].copy_from_slice(&458_752_u32.to_le_bytes());
        public[32..64].fill(1);
        public[64..96].fill(2);
        public[96..128].fill(3);
        public[128..160].fill(4);
        public[256..260].copy_from_slice(&96_u32.to_be_bytes());
        public[260..284].fill(5);
        public[284..292].copy_from_slice(&7_u64.to_be_bytes());
        public[292..324].fill(8);
        for (index, byte) in public[328..].iter_mut().enumerate() {
            *byte = (index % 251) as u8;
        }
        public
    }

    fn honest_witness(base: &[u8]) -> MembershipWitness {
        let mut witness = MembershipWitness {
            protocol_code: row_join::PROTOCOL_V9_CODE,
            terminal_committed: 1,
            tree_depth: 10,
            row_count: 712,
            session_id: b"SESSION_001".to_vec(),
            s_0: [9; 32],
            s_n: [10; 32],
            authority_manifest_sha256: [11; 32],
            chain_log_blake3: [12; 32],
            ordered_session_root_blake3: [0; 32],
            siblings_blake3: (0..10).map(|index| [index + 13; 32]).collect(),
        };
        witness.ordered_session_root_blake3 = derive_membership_digests(base, &witness)
            .unwrap()
            .wrapped_root_blake3;
        witness
    }

    #[test]
    fn private_witness_round_trips_with_exact_dynamic_length() {
        let base = base_public();
        let witness = honest_witness(&base);
        let encoded = witness.encode().unwrap();
        assert_eq!(encoded.len(), PRIVATE_FIXED_BYTES + 11 + 10 * 32);
        assert_eq!(MembershipWitness::parse(&encoded).unwrap(), witness);
        assert!(MembershipWitness::parse(&encoded[..encoded.len() - 1]).is_err());
        let mut appended = encoded.clone();
        appended.push(0);
        assert!(MembershipWitness::parse(&appended).is_err());
    }

    #[test]
    fn honest_opening_replaces_private_row_prefix_and_preserves_scorer() {
        let base = base_public();
        let witness = honest_witness(&base);
        let public = verify_and_encode(&base, &witness).unwrap();
        assert_eq!(public.len(), PUBLIC_BYTES);
        assert_eq!(&public[0..8], PUBLIC_MAGIC);
        assert_eq!(public[8], PUBLIC_ABI_VERSION);
        assert_eq!(u32::from_le_bytes(public[12..16].try_into().unwrap()), PUBLIC_BYTES as u32);
        assert_eq!(u32::from_le_bytes(public[16..20].try_into().unwrap()), 96);
        assert_eq!(&public[348..352], &[0; 4]);
        assert_eq!(&public[352..], &base[328..]);
        assert_ne!(&public[..328], &base[..328]);
    }

    #[test]
    fn framing_path_context_and_root_mutations_fail_closed() {
        let base = base_public();
        let witness = honest_witness(&base);
        let honest = witness.encode().unwrap();
        for index in [0_usize, 8, 9, 10, 11, 12, 16, 18] {
            let mut changed = honest.clone();
            changed[index] ^= 1;
            assert!(MembershipWitness::parse(&changed).is_err(), "byte {index}");
        }

        for index in [20_usize, 24, 56, 88, 120, 152, 184] {
            let mut changed = honest.clone();
            changed[index] ^= 1;
            match MembershipWitness::parse(&changed) {
                Ok(parsed) => assert!(verify_and_encode(&base, &parsed).is_err(), "byte {index}"),
                Err(_) => {}
            }
        }

        let mut changed_root = witness.clone();
        changed_root.ordered_session_root_blake3[0] ^= 1;
        assert!(verify_and_encode(&base, &changed_root).is_err());

        for index in 0..witness.siblings_blake3.len() {
            let mut changed_sibling = witness.clone();
            changed_sibling.siblings_blake3[index][index] ^= 1;
            assert!(
                verify_and_encode(&base, &changed_sibling).is_err(),
                "sibling {index}"
            );
        }

        let mut changed_context = witness.clone();
        changed_context.chain_log_blake3[0] ^= 1;
        assert!(verify_and_encode(&base, &changed_context).is_err());

        let mut changed_row = base.clone();
        changed_row[96] ^= 1;
        assert!(verify_and_encode(&changed_row, &witness).is_err());
    }

    #[test]
    fn row_count_depth_identifier_and_base_framing_are_closed() {
        let base = base_public();
        let witness = honest_witness(&base);

        let mut wrong_depth = witness.clone();
        wrong_depth.tree_depth = 9;
        wrong_depth.siblings_blake3.pop();
        assert!(wrong_depth.encode().is_err());

        let mut wrong_identifier = witness.clone();
        wrong_identifier.session_id = b"bad session".to_vec();
        assert!(wrong_identifier.encode().is_err());

        let mut wrong_base = base;
        wrong_base[12] ^= 1;
        assert!(verify_and_encode(&wrong_base, &witness).is_err());
    }

    #[test]
    fn every_leaf_field_and_path_order_are_bound() {
        let base = base_public();
        let witness = honest_witness(&base);

        for index in [32_usize, 64, 96, 128, 260, 284, 292] {
            let mut changed = base.clone();
            changed[index] ^= 1;
            assert!(
                verify_and_encode(&changed, &witness).is_err(),
                "base byte {index}"
            );
        }

        let mut coherent_row_index = base.clone();
        coherent_row_index[16..20].copy_from_slice(&97_u32.to_le_bytes());
        coherent_row_index[256..260].copy_from_slice(&97_u32.to_be_bytes());
        assert!(verify_and_encode(&coherent_row_index, &witness).is_err());

        let mut reversed = witness.clone();
        reversed.siblings_blake3.reverse();
        assert!(verify_and_encode(&base, &reversed).is_err());

        let mut rotated = witness.clone();
        rotated.siblings_blake3.rotate_left(1);
        assert!(verify_and_encode(&base, &rotated).is_err());

        let mut truncated = witness.clone();
        truncated.siblings_blake3.pop();
        assert!(verify_and_encode(&base, &truncated).is_err());

        let mut appended = witness;
        appended.siblings_blake3.push([0; 32]);
        assert!(verify_and_encode(&base, &appended).is_err());
    }

    #[test]
    fn row_count_depth_boundaries_and_depth_zero_are_exact() {
        for (row_count, expected_depth) in [
            (1_u32, 0_u16),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 3),
            (1_000_000, 20),
        ] {
            assert_eq!(tree_depth_for_count(row_count).unwrap(), expected_depth);
        }
        assert!(tree_depth_for_count(0).is_err());
        assert!(tree_depth_for_count(1_000_001).is_err());

        let mut one_row_base = base_public();
        one_row_base[16..20].copy_from_slice(&0_u32.to_le_bytes());
        one_row_base[256..260].copy_from_slice(&0_u32.to_be_bytes());
        let mut one_row = MembershipWitness {
            protocol_code: row_join::PROTOCOL_V9_CODE,
            terminal_committed: 1,
            tree_depth: 0,
            row_count: 1,
            session_id: b"ONE_ROW".to_vec(),
            s_0: [9; 32],
            s_n: [10; 32],
            authority_manifest_sha256: [11; 32],
            chain_log_blake3: [12; 32],
            ordered_session_root_blake3: [0; 32],
            siblings_blake3: Vec::new(),
        };
        one_row.ordered_session_root_blake3 = derive_membership_digests(&one_row_base, &one_row)
            .unwrap()
            .wrapped_root_blake3;
        assert_eq!(
            MembershipWitness::parse(&one_row.encode().unwrap()).unwrap(),
            one_row
        );
        assert!(verify_and_encode(&one_row_base, &one_row).is_ok());
    }

    #[test]
    fn session_identifier_grammar_and_limits_are_exact() {
        let base = base_public();
        let witness = honest_witness(&base);

        let mut allowed = witness.clone();
        allowed.session_id = b"A._:-z9".to_vec();
        allowed.ordered_session_root_blake3 = derive_membership_digests(&base, &allowed)
            .unwrap()
            .wrapped_root_blake3;
        assert!(allowed.encode().is_ok());

        for invalid in [
            Vec::new(),
            vec![b'A'; 129],
            vec![0xff],
            b"_invalid-first".to_vec(),
            b"bad session".to_vec(),
        ] {
            let mut changed = witness.clone();
            changed.session_id = invalid;
            assert!(changed.encode().is_err());
        }
    }

    #[test]
    fn classification_distinguishes_relation_from_an_actual_proof() {
        assert_eq!(CLASSIFICATION, "CANDIDATE_ORDERED_MEMBERSHIP");
        assert!(ROW_MEMBERSHIP_RELATION_ENFORCED);
        assert!(SESSION_IDENTIFIER_IN_RELATION);
        assert!(AUTHORITY_MANIFEST_DIGEST_IN_CONTEXT);
        assert!(CHAIN_LOG_DIGEST_IN_CONTEXT);
    }
}
