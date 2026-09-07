//! SP1 guest: the WHOLE ZeeBeam session chain in one statement.
//!
//! Private inputs: the complete chain log (CSV bytes), the session identifier, the authority-manifest
//! SHA-256 and the terminal state S_N. The program verifies, for every row t of the log, that the
//! row's drand quicknet signature verifies for its round (each distinct round verified once), that
//! the logged beacon value is SHA-256 of that signature, and that the logged next state equals
//! advance_chain(S_t, raw_t, meta_t, round_t, value_t) (S_N for the last row). It then rebuilds the
//! ordered-session tree exactly as the per-row relation defines it (context, leaves, padding leaves,
//! nodes, wrapped root) and publishes: row count, tree depth, number of distinct rounds, first and
//! last rounds, S_0, S_N, BLAKE3 of the chain log, the manifest digest, the context digest, the root
//! and the session identifier. A per-row proof's context digest and session root must equal these.
//!
//! Domains and encodings are those of `zeebeam-row-binding-membership` (read, not recalled):
//! domain_hash(D, parts) = BLAKE3(D || for each part: u32be(len) || part).

#![no_main]
sp1_zkvm::entrypoint!(main);

use std::collections::BTreeMap;

const CONTEXT_DOMAIN: &[u8] = b"ZEEBEAM_ORDERED_SESSION_CONTEXT_V1\0";
const ROW_DOMAIN: &[u8] = b"ZEEBEAM_ORDERED_SESSION_ROW_V1\0";
const NODE_DOMAIN: &[u8] = b"ZEEBEAM_ORDERED_SESSION_NODE_V1\0";
const ROOT_DOMAIN: &[u8] = b"ZEEBEAM_ORDERED_SESSION_ROOT_V1\0";
const PADDING_DOMAIN: &[u8] = b"ZEEBEAM_ORDERED_SESSION_PADDING_V1\0";
const PROTOCOL: &[u8] = b"TB-v0.9";
const TERMINAL_COMMITTED: u8 = 1;
pub const PUBLIC_MAGIC: &[u8; 8] = b"ZBCHAIN1";

fn domain_hash(domain: &[u8], parts: &[&[u8]]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(domain);
    for p in parts {
        h.update(&(p.len() as u32).to_be_bytes());
        h.update(p);
    }
    *h.finalize().as_bytes()
}

fn hex_decode(h: &[u8]) -> Vec<u8> {
    assert!(h.len() % 2 == 0, "odd hex length");
    let val = |c: u8| -> u8 {
        match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            b'A'..=b'F' => c - b'A' + 10,
            _ => panic!("non-hex byte in chain log"),
        }
    };
    h.chunks(2).map(|p| (val(p[0]) << 4) | val(p[1])).collect()
}

fn parse_u64(d: &[u8]) -> u64 {
    assert!(!d.is_empty() && d.len() <= 20, "bad decimal length");
    let mut v: u64 = 0;
    for &c in d {
        assert!(c.is_ascii_digit(), "non-digit in decimal field");
        v = v.checked_mul(10).and_then(|x| x.checked_add((c - b'0') as u64)).expect("decimal overflow");
    }
    v
}

fn arr32(v: Vec<u8>, what: &str) -> [u8; 32] {
    v.try_into().unwrap_or_else(|_| panic!("{what} is not 32 bytes"))
}

struct Row {
    s_t: [u8; 32],
    raw: [u8; 32],
    emission: [u8; 32],
    meta: [u8; zeebeam_b3xof_relation::META_BYTES],
    round: u64,
    value: [u8; 32],
    signature: Vec<u8>,
}

pub fn main() {
    println!("cycle-tracker-report-start: chain_input");
    let csv = sp1_zkvm::io::read_vec();
    let session_id = sp1_zkvm::io::read_vec();
    let manifest_sha256 = arr32(sp1_zkvm::io::read_vec(), "authority manifest digest");
    let s_n = arr32(sp1_zkvm::io::read_vec(), "S_N");
    println!("cycle-tracker-report-end: chain_input");

    println!("cycle-tracker-report-start: chain_log_blake3");
    let chain_log_blake3 = *blake3::hash(&csv).as_bytes();
    println!("cycle-tracker-report-end: chain_log_blake3");

    println!("cycle-tracker-report-start: chain_parse");
    let mut rows: Vec<Row> = Vec::new();
    let mut header_seen = false;
    for line in csv.split(|&b| b == b'\n') {
        if line.is_empty() || line[0] == b'#' {
            continue;
        }
        if !header_seen {
            assert!(line.starts_with(b"t,S_t_hex,bayer_blake3_hex,"), "unexpected chain log header");
            header_seen = true;
            continue;
        }
        let f: Vec<&[u8]> = line.split(|&b| b == b',').collect();
        assert!(f.len() >= 13, "chain log row has too few fields");
        let t = parse_u64(f[0]);
        assert_eq!(t as usize, rows.len(), "chain log rows are not sequential from 0");
        rows.push(Row {
            s_t: arr32(hex_decode(f[1]), "S_t"),
            raw: arr32(hex_decode(f[2]), "raw digest"),
            emission: arr32(hex_decode(f[6]), "emission digest"),
            meta: hex_decode(f[8]).try_into().unwrap_or_else(|_| panic!("meta is not 28 bytes")),
            round: parse_u64(f[10]),
            value: arr32(hex_decode(f[11]), "drand value"),
            signature: hex_decode(f[12]),
        });
    }
    let row_count = rows.len() as u32;
    assert!(row_count >= 2, "a chain needs at least two rows");
    println!("chain_rows={row_count}");
    println!("cycle-tracker-report-end: chain_parse");

    // ---- every distinct beacon round verified once; every row's value must be SHA-256(sig) ----
    println!("cycle-tracker-report-start: chain_beacons");
    let mut verified: BTreeMap<u64, [u8; 32]> = BTreeMap::new();
    for (t, r) in rows.iter().enumerate() {
        let v = match verified.get(&r.round) {
            Some(v) => *v,
            None => {
                let v = zeebeam_row_binding_join::verify_quicknet_beacon(r.round, &r.signature)
                    .unwrap_or_else(|_| panic!("row {t}: beacon signature for round {} does not verify", r.round));
                verified.insert(r.round, v);
                v
            }
        };
        assert!(v == r.value, "row {t}: logged drand value is not SHA-256 of the verified signature");
    }
    let distinct_rounds = verified.len() as u32;
    let first_round = *verified.keys().next().unwrap();
    let last_round = *verified.keys().next_back().unwrap();
    println!("distinct_rounds={distinct_rounds} first_round={first_round} last_round={last_round}");
    println!("cycle-tracker-report-end: chain_beacons");

    // ---- every transition S_t -> S_{t+1} recomputed; the last one must land on S_N ----
    println!("cycle-tracker-report-start: chain_advances");
    for t in 0..rows.len() {
        let r = &rows[t];
        let expected_next = if t + 1 < rows.len() { rows[t + 1].s_t } else { s_n };
        let got = zeebeam_b3xof_relation::advance_chain(&r.s_t, &r.raw, &r.meta, r.round, &r.value);
        assert!(got == expected_next, "row {t}: advance does not produce the logged next state");
        assert!(r.round >= if t > 0 { rows[t - 1].round } else { r.round }, "row {t}: beacon round decreased");
    }
    println!("cycle-tracker-report-end: chain_advances");

    // ---- the ordered-session tree, exactly as the per-row relation defines it ----
    println!("cycle-tracker-report-start: chain_tree");
    let s_0 = rows[0].s_t;
    let row_count_be = row_count.to_be_bytes();
    let context = domain_hash(
        CONTEXT_DOMAIN,
        &[PROTOCOL, &session_id, &row_count_be, &[TERMINAL_COMMITTED], &s_0, &s_n, &manifest_sha256, &chain_log_blake3],
    );
    let mut depth: u16 = 1;
    while (1usize << depth) < rows.len() {
        depth += 1;
    }
    let width = 1usize << depth;
    let mut layer: Vec<[u8; 32]> = Vec::with_capacity(width);
    for (t, r) in rows.iter().enumerate() {
        let s_next = if t + 1 < rows.len() { rows[t + 1].s_t } else { s_n };
        layer.push(domain_hash(
            ROW_DOMAIN,
            &[&context, &(t as u32).to_be_bytes(), &r.s_t, &r.raw, &r.meta, &r.round.to_be_bytes(), &r.value, &s_next, &r.emission],
        ));
    }
    for i in rows.len()..width {
        layer.push(domain_hash(PADDING_DOMAIN, &[&context, &row_count_be, &(i as u32).to_be_bytes()]));
    }
    for level in 1..=depth {
        let mut next = Vec::with_capacity(layer.len() / 2);
        for parent in 0..layer.len() / 2 {
            next.push(domain_hash(
                NODE_DOMAIN,
                &[&context, &level.to_be_bytes(), &(parent as u32).to_be_bytes(), &layer[2 * parent], &layer[2 * parent + 1]],
            ));
        }
        layer = next;
    }
    assert_eq!(layer.len(), 1, "tree did not reduce to one root");
    let root = domain_hash(ROOT_DOMAIN, &[&context, &row_count_be, &depth.to_be_bytes(), &layer[0]]);
    println!("cycle-tracker-report-end: chain_tree");

    println!("cycle-tracker-report-start: public_commitment");
    let mut out = Vec::with_capacity(8 + 4 + 2 + 4 + 8 + 8 + 32 * 6 + 2 + session_id.len());
    out.extend_from_slice(PUBLIC_MAGIC);
    out.extend_from_slice(&row_count_be);
    out.extend_from_slice(&depth.to_be_bytes());
    out.extend_from_slice(&distinct_rounds.to_be_bytes());
    out.extend_from_slice(&first_round.to_be_bytes());
    out.extend_from_slice(&last_round.to_be_bytes());
    out.extend_from_slice(&s_0);
    out.extend_from_slice(&s_n);
    out.extend_from_slice(&chain_log_blake3);
    out.extend_from_slice(&manifest_sha256);
    out.extend_from_slice(&context);
    out.extend_from_slice(&root);
    out.extend_from_slice(&(session_id.len() as u16).to_be_bytes());
    out.extend_from_slice(&session_id);
    sp1_zkvm::io::commit_slice(&out);
    println!("cycle-tracker-report-end: public_commitment");
}
