---
version: 1.6
date: 2026-09-09
status: third-party-notices
author: Cathal Ryan Hynes (author of record); drafted with BOSUN, the project's automated research assistant
---

# Third-party notices

The `LICENSE` of this repository covers the copyright holder's own work only. Third-party material in this tree,
and the third-party code incorporated in the compiled programs and rendered documents it ships:

| what | where | licence |
|------|-------|---------|
| `jubjub` 0.10.0, a modified copy (four `const` field initialisers made non-const; described in `bundle/proofs_20260902/source/README.md` and digested in `bundle/proofs_20260902/final_relation/DEPENDENCY_SHA256SUMS`) | `bundle/proofs_20260902/source/vendor/jubjub-0.10.0/` | `MIT/Apache-2.0` as declared in its `Cargo.toml`; its `LICENSE-APACHE` and `LICENSE-MIT` ship with the copy |
| compiled third-party code in the three guest ELFs (`bundle/proofs_20260902/box_guest.elf`, `bundle/proofs_20260902/final_relation/zeebeam_guest_final.elf`, `bundle/proofs_20260902/final_relation/chain/zeebeam_chain_guest.elf`): the 166 third-party packages in the union of the shipped guest-workspace Cargo.lock files, listed below; target-runtime crate code is incorporated into the ELFs, while build dependencies and procedural macros execute during compilation and are not target-linked | compiled into the ELFs (target-runtime crates); their sources are not in this tree (the offline kit vendors them) | as listed below, read from each crate's `Cargo.toml`; the applicable licence texts and required notices are in `licenses/`: `Apache-2.0.txt`, `MIT.txt` (the complete upstream MIT licence files of the eighteen MIT-only crates and of `byteorder` and `memchr`, for which MIT is elected), `BSD-2-Clause-arrayref.txt` (the `arrayref` notice), `BSD-3-Clause-subtle.txt` (the `subtle` notice), `Unicode-3.0-unicode-ident.txt` (the `unicode-ident` notice); the two `sp1-patches` forks (`sha2` 0.10.9, `bls12_381` 0.8.0) keep their upstream MIT or Apache-2.0 licences |
| Rust `core`, `alloc`, `std`, `panic_abort`, `compiler-builtins` and the linked standard-library dependencies compiled into the three ELFs by the `succinct-1.94.0-64bit` toolchain | compiled into the ELFs | `core`, `alloc`, `std` and `panic_abort`: MIT OR Apache-2.0; `compiler-builtins`: MIT AND Apache-2.0 WITH LLVM-exception; its complete licence and LLVM-derived notices are in `licenses/compiler-builtins-LICENSE.txt` (byte-exact `library/compiler-builtins/LICENSE.txt` of the toolchain's source commit); the other linked standard-library components' terms are reproduced in `licenses/Rust-COPYRIGHT-library.html`, the complete, unmodified library-only COPYRIGHT notice generated for the Rust 1.94.0 release (rustc 1.94.0, commit 4a4ef493e3a1488c6e321570238084b38948f6db, 2026-03-02; sha256 `af70aaabed1b73e872f14f9130db37e09f3f4d73a5f7c598b9173697a5d2729f`). The `succinct-1.94.0-64bit` toolchain (rustc 1.94.0-dev, built from succinctlabs/rust commit c7149403db5f6f72f410d6dffcee90378235f23b) ships no generated notice. Re-running that commit's generate-copyright tool over that commit and its exact vendored inputs produced `licenses/Rust-COPYRIGHT-library.html` byte-for-byte; sha256 af70aaabed1b73e872f14f9130db37e09f3f4d73a5f7c598b9173697a5d2729f (run on 5 September 2026 with `x.py run generate-copyright`; that commit's `library/Cargo.lock`, `COPYRIGHT` and `library/compiler-builtins/LICENSE.txt` are also byte-identical to the 1.94.0 release's); texts in `Apache-2.0.txt` and `MIT.txt` |
| the dependencies of the host programs and of the standalone verifier | not distributed; named by `Cargo.lock` files and fetched by cargo, or supplied by the offline kit | their own licences, listed per crate in the offline kit's `THIRD_PARTY_NOTICES.md` |
| SP1 6.4.0 (`sp1-sdk`, `sp1-zkvm`, `sp1-verifier`, `cargo prove`, the GPU server) | guest-side crates compiled into the ELFs as above; the tools are not distributed | MIT or Apache-2.0 per the `succinctlabs/sp1` repository; the GPU server's terms are those shipped with its archive |
| drand quicknet public key and chain hash; Zcash block 3456294 header hash and the transaction identifier | pinned constants and statement fields in the bundle | public protocol data |
| DejaVu Sans glyph outlines embedded in `figures/zeebeam.svg` (and rendered into `figures/zeebeam.png`); DejaVu Serif, DejaVu Serif Bold, DejaVu Serif Italic and DejaVu Sans Mono subsets in all three PDFs, plus DejaVu Sans Mono Bold in `companions/zeebeam_worked_examples.pdf` | `figures/`, `paper/`, `companions/` | Bitstream Vera licence with the DejaVu additions; text in `licenses/DejaVu-Bitstream-Vera.txt`; the full upstream font files are not separately distributed; the SVG and PDFs distribute embedded outlines or subsets |
| a Liberation Serif glyph subset embedded in `paper/zeebeam.pdf` (the renderer's fallback for one font request) | `paper/zeebeam.pdf` | SIL Open Font License 1.1 with the Liberation copyright notice; text in `licenses/OFL-1.1-Liberation.txt` |

The offline kit (archived separately; see `RELEASE_NOTES.md`) redistributes toolchain archives, vendored crates and
fetched API responses; its own `THIRD_PARTY_NOTICES.md` lists them with their licences and terms, and records two
open questions (the SP1 GPU-server archive's terms and Blockchair's terms for redistributing API responses).

## Third-party packages locked by the guest workspaces (166)

Licence expressions, as declared: MIT OR Apache-2.0: 103; MIT/Apache-2.0: 22; MIT: 18; Apache-2.0 OR MIT: 7; Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT: 3; Unlicense OR MIT: 2; CC0-1.0 OR MIT-0 OR Apache-2.0: 2; BSD-2-Clause OR Apache-2.0 OR MIT: 2; BSD-2-Clause: 1; CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception: 1; Apache-2.0/MIT: 1; MIT OR Apache-2.0 OR LGPL-2.1-or-later: 1; BSD-3-Clause: 1; (MIT OR Apache-2.0) AND Unicode-3.0: 1; Zlib OR MIT OR Apache-2.0: 1. Read mechanically from the
vendored copies' `Cargo.toml` files on 5 September 2026.

| crate | version | licence (Cargo.toml) | repository |
|---|---|---|---|
| `addchain` | 0.2.1 | MIT OR Apache-2.0 | https://github.com/str4d/addchain |
| `aead` | 0.5.2 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| `aes` | 0.8.4 | MIT OR Apache-2.0 | https://github.com/RustCrypto/block-ciphers |
| `ahash` | 0.8.12 | MIT OR Apache-2.0 | https://github.com/tkaitchuck/ahash |
| `ark-bn254` | 0.4.0 | MIT/Apache-2.0 | https://github.com/arkworks-rs/curves |
| `ark-ec` | 0.4.2 | MIT/Apache-2.0 | https://github.com/arkworks-rs/algebra |
| `ark-ff` | 0.4.2 | MIT/Apache-2.0 | https://github.com/arkworks-rs/algebra |
| `ark-ff-asm` | 0.4.2 | MIT/Apache-2.0 | https://github.com/arkworks-rs/algebra |
| `ark-ff-macros` | 0.4.2 | MIT/Apache-2.0 | https://github.com/arkworks-rs/algebra |
| `ark-poly` | 0.4.2 | MIT/Apache-2.0 | https://github.com/arkworks-rs/algebra |
| `ark-serialize` | 0.4.2 | MIT/Apache-2.0 | https://github.com/arkworks-rs/algebra |
| `ark-serialize-derive` | 0.4.2 | MIT/Apache-2.0 | https://github.com/arkworks-rs/algebra |
| `ark-std` | 0.4.0 | MIT/Apache-2.0 | https://github.com/arkworks-rs/std |
| `arrayref` | 0.3.9 | BSD-2-Clause | https://github.com/droundy/arrayref |
| `arrayvec` | 0.7.8 | MIT OR Apache-2.0 | https://github.com/bluss/arrayvec |
| `autocfg` | 1.5.1 | Apache-2.0 OR MIT | https://github.com/cuviper/autocfg |
| `bech32` | 0.11.1 | MIT | https://github.com/rust-bitcoin/rust-bech32 |
| `bincode` | 1.3.3 | MIT | https://github.com/servo/bincode |
| `bitvec` | 1.1.1 | MIT | https://github.com/bitvecto-rs/bitvec |
| `blake2b_simd` | 1.0.5 | MIT | https://github.com/oconnor663/blake2_simd |
| `blake3` | 1.8.1 | CC0-1.0 OR Apache-2.0 OR Apache-2.0 WITH LLVM-exception | https://github.com/BLAKE3-team/BLAKE3 |
| `block-buffer` | 0.10.4 | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| `bls12_381` | 0.8.0 | MIT/Apache-2.0 | https://github.com/sp1-patches/bls12_381 |
| `byteorder` | 1.5.0 | Unlicense OR MIT | https://github.com/BurntSushi/byteorder |
| `cbc` | 0.1.2 | MIT OR Apache-2.0 | https://github.com/RustCrypto/block-modes |
| `cc` | 1.4.4 | MIT OR Apache-2.0 | https://github.com/rust-lang/cc-rs |
| `cfg-if` | 1.0.4 | MIT OR Apache-2.0 | https://github.com/rust-lang/cfg-if |
| `chacha20` | 0.9.1 | Apache-2.0 OR MIT | https://github.com/RustCrypto/stream-ciphers |
| `chacha20poly1305` | 0.10.1 | Apache-2.0 OR MIT | https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305 |
| `cipher` | 0.4.4 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| `const-default` | 1.0.0 | MIT | https://github.com/AerialX/const-default.rs |
| `constant_time_eq` | 0.3.1 | CC0-1.0 OR MIT-0 OR Apache-2.0 | https://github.com/cesarb/constant_time_eq |
| `constant_time_eq` | 0.4.2 | CC0-1.0 OR MIT-0 OR Apache-2.0 | https://github.com/cesarb/constant_time_eq |
| `corez` | 0.1.1 | MIT OR Apache-2.0 | https://github.com/zcash/corez |
| `cpufeatures` | 0.2.17 | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| `critical-section` | 1.2.0 | MIT OR Apache-2.0 | https://github.com/rust-embedded/critical-section |
| `crypto-common` | 0.1.7 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| `derivative` | 2.2.0 | MIT/Apache-2.0 | https://github.com/mcarton/rust-derivative |
| `digest` | 0.10.7 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| `either` | 1.18.0 | MIT OR Apache-2.0 | https://github.com/rayon-rs/either |
| `elf` | 0.7.4 | MIT/Apache-2.0 | https://github.com/cole14/rust-elf/ |
| `embedded-alloc` | 0.6.0 | MIT OR Apache-2.0 | https://github.com/rust-embedded/embedded-alloc |
| `ff` | 0.13.1 | MIT/Apache-2.0 | https://github.com/zkcrypto/ff |
| `ff_derive` | 0.13.1 | MIT/Apache-2.0 | https://github.com/zkcrypto/ff |
| `find-msvc-tools` | 0.1.11 | MIT OR Apache-2.0 | https://github.com/rust-lang/cc-rs |
| `fpe` | 0.6.1 | MIT/Apache-2.0 | https://github.com/str4d/fpe |
| `funty` | 2.0.0 | MIT | https://github.com/myrrlyn/funty |
| `futures` | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-channel` | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-core` | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-executor` | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-io` | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-macro` | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-sink` | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-task` | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `futures-util` | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| `gcd` | 2.3.0 | MIT/Apache-2.0 | https://github.com/frewsxcv/rust-gcd |
| `generic-array` | 0.14.7 | MIT | https://github.com/fizyk20/generic-array.git |
| `getrandom` | 0.2.17 | MIT OR Apache-2.0 | https://github.com/rust-random/getrandom |
| `getrandom` | 0.3.4 | MIT OR Apache-2.0 | https://github.com/rust-random/getrandom |
| `getset` | 0.1.7 | MIT | https://github.com/jbaublitz/getset |
| `group` | 0.13.0 | MIT/Apache-2.0 | https://github.com/zkcrypto/group |
| `halo2_poseidon` | 0.1.0 | MIT OR Apache-2.0 | https://github.com/zcash/halo2 |
| `hashbrown` | 0.13.2 | MIT OR Apache-2.0 | https://github.com/rust-lang/hashbrown |
| `hex` | 0.4.3 | MIT OR Apache-2.0 | https://github.com/KokaKiwi/rust-hex |
| `incrementalmerkletree` | 0.8.2 | MIT OR Apache-2.0 | https://github.com/zcash/incrementalmerkletree |
| `inout` | 0.1.4 | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| `itertools` | 0.10.5 | MIT/Apache-2.0 | https://github.com/rust-itertools/itertools |
| `itertools` | 0.12.1 | MIT OR Apache-2.0 | https://github.com/rust-itertools/itertools |
| `itertools` | 0.14.0 | MIT OR Apache-2.0 | https://github.com/rust-itertools/itertools |
| `lazy_static` | 1.5.0 | MIT OR Apache-2.0 | https://github.com/rust-lang-nursery/lazy-static.rs |
| `libc` | 0.2.189 | MIT OR Apache-2.0 | https://github.com/rust-lang/libc |
| `libm` | 0.2.16 | MIT | https://github.com/rust-lang/compiler-builtins |
| `linked_list_allocator` | 0.10.6 | Apache-2.0/MIT | https://github.com/phil-opp/linked-list-allocator |
| `memchr` | 2.8.3 | Unlicense OR MIT | https://github.com/BurntSushi/memchr |
| `memuse` | 0.2.2 | MIT/Apache-2.0 | https://github.com/str4d/memuse |
| `nonempty` | 0.11.0 | MIT | https://github.com/cloudhead/nonempty |
| `num-bigint` | 0.3.3 | MIT OR Apache-2.0 | https://github.com/rust-num/num-bigint |
| `num-bigint` | 0.4.8 | MIT OR Apache-2.0 | https://github.com/rust-num/num-bigint |
| `num-integer` | 0.1.47 | MIT OR Apache-2.0 | https://github.com/rust-num/num-integer |
| `num-traits` | 0.2.19 | MIT OR Apache-2.0 | https://github.com/rust-num/num-traits |
| `once_cell` | 1.21.4 | MIT OR Apache-2.0 | https://github.com/matklad/once_cell |
| `opaque-debug` | 0.3.1 | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| `orchard` | 0.15.3 | MIT OR Apache-2.0 | https://github.com/zcash/orchard |
| `p3-bn254-fr` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-challenger` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-dft` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-field` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-koala-bear` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-matrix` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-maybe-rayon` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-mds` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-poseidon2` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-symmetric` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `p3-util` | 0.4.3-succinct | MIT OR Apache-2.0 | https://github.com/Plonky3/Plonky3 |
| `pairing` | 0.23.0 | MIT/Apache-2.0 | https://github.com/zkcrypto/pairing |
| `pasta_curves` | 0.5.2 | MIT OR Apache-2.0 | https://github.com/zcash/pasta_curves |
| `paste` | 1.0.15 | MIT OR Apache-2.0 | https://github.com/dtolnay/paste |
| `pin-project-lite` | 0.2.17 | Apache-2.0 OR MIT | https://github.com/taiki-e/pin-project-lite |
| `poly1305` | 0.8.0 | Apache-2.0 OR MIT | https://github.com/RustCrypto/universal-hashes |
| `ppv-lite86` | 0.2.21 | MIT OR Apache-2.0 | https://github.com/cryptocorrosion/cryptocorrosion |
| `proc-macro2` | 1.0.107 | MIT OR Apache-2.0 | https://github.com/dtolnay/proc-macro2 |
| `quote` | 1.0.47 | MIT OR Apache-2.0 | https://github.com/dtolnay/quote |
| `r-efi` | 5.3.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | https://github.com/r-efi/r-efi |
| `radium` | 0.7.0 | MIT | https://github.com/bitvecto-rs/radium |
| `rand` | 0.8.7 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `rand` | 0.8.8 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `rand_chacha` | 0.3.1 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `rand_core` | 0.6.4 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| `reddsa` | 0.5.2 | MIT OR Apache-2.0 | https://github.com/ZcashFoundation/reddsa |
| `rlsf` | 0.2.3 | MIT/Apache-2.0 | https://github.com/yvt/rlsf |
| `rustc_version` | 0.4.1 | MIT OR Apache-2.0 | https://github.com/djc/rustc-version-rs |
| `rustversion` | 1.0.23 | MIT OR Apache-2.0 | https://github.com/dtolnay/rustversion |
| `semver` | 1.0.28 | MIT OR Apache-2.0 | https://github.com/dtolnay/semver |
| `serde` | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| `serde_core` | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| `serde_derive` | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| `sha2` | 0.10.9 | MIT OR Apache-2.0 | https://github.com/RustCrypto/hashes |
| `shlex` | 2.0.1 | MIT OR Apache-2.0 | https://github.com/comex/rust-shlex |
| `sinsemilla` | 0.1.0 | MIT OR Apache-2.0 | https://github.com/zcash/sinsemilla |
| `slab` | 0.4.12 | MIT | https://github.com/tokio-rs/slab |
| `slop-algebra` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-algebra` | 6.6.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-bn254` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-bn254` | 6.6.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-challenger` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-challenger` | 6.6.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-koala-bear` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-koala-bear` | 6.6.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-poseidon2` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-poseidon2` | 6.6.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-primitives` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-primitives` | 6.6.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-symmetric` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `slop-symmetric` | 6.6.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `sp1-lib` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `sp1-lib` | 6.6.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `sp1-primitives` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `sp1-primitives` | 6.6.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `sp1-zkvm` | 6.4.0 | MIT OR Apache-2.0 | https://github.com/succinctlabs/sp1 |
| `spin` | 0.9.9 | MIT | https://github.com/mvdnes/spin-rs.git |
| `static_assertions` | 1.1.0 | MIT OR Apache-2.0 | https://github.com/nvzqz/static-assertions-rs |
| `subtle` | 2.6.1 | BSD-3-Clause | https://github.com/dalek-cryptography/subtle |
| `syn` | 1.0.109 | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| `syn` | 2.0.119 | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| `syn` | 3.0.4 | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| `tap` | 1.0.1 | MIT | https://github.com/myrrlyn/tap |
| `tracing` | 0.1.44 | MIT | https://github.com/tokio-rs/tracing |
| `tracing-attributes` | 0.1.31 | MIT | https://github.com/tokio-rs/tracing |
| `tracing-core` | 0.1.36 | MIT | https://github.com/tokio-rs/tracing |
| `typenum` | 1.20.1 | MIT OR Apache-2.0 | https://github.com/paholg/typenum |
| `unicode-ident` | 1.0.24 | (MIT OR Apache-2.0) AND Unicode-3.0 | https://github.com/dtolnay/unicode-ident |
| `universal-hash` | 0.5.1 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| `version_check` | 0.9.5 | MIT/Apache-2.0 | https://github.com/SergioBenitez/version_check |
| `visibility` | 0.1.1 | Zlib OR MIT OR Apache-2.0 | https://github.com/danielhenrymantilla/visibility.rs |
| `wasi` | 0.11.1+wasi-snapshot-preview1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/bytecodealliance/wasi |
| `wasip2` | 1.0.4+wasi-0.2.12 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/bytecodealliance/wasi-rs |
| `wit-bindgen` | 0.57.1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/bytecodealliance/wit-bindgen |
| `wyz` | 0.5.1 | MIT | https://github.com/myrrlyn/wyz |
| `zcash_note_encryption` | 0.4.2 | MIT OR Apache-2.0 | https://github.com/zcash/librustzcash |
| `zcash_spec` | 0.2.1 | MIT OR Apache-2.0 | https://github.com/zcash/zcash_spec |
| `zerocopy` | 0.8.56 | BSD-2-Clause OR Apache-2.0 OR MIT | https://github.com/google/zerocopy |
| `zerocopy-derive` | 0.8.56 | BSD-2-Clause OR Apache-2.0 OR MIT | https://github.com/google/zerocopy |
| `zeroize` | 1.9.0 | Apache-2.0 OR MIT | https://github.com/RustCrypto/utils |
| `zeroize_derive` | 1.5.0 | Apache-2.0 OR MIT | https://github.com/RustCrypto/utils |
| `zip32` | 0.2.1 | MIT OR Apache-2.0 | https://github.com/zcash/zip32 |

## Log

- 1.6 (2026-09-09, BOSUN) — authorship line, 9 September 2026.
- 1.5 (2026-09-05, BOSUN) — after Sol's fourth closing pass: the Rust library notice's provenance settled by running the toolchain source commit's own generate-copyright tool, whose library output is byte-identical to the shipped 1.94.0 release file.
- 1.4 (2026-09-05, BOSUN) — after Sol's third closing pass: the Rust library notice replaced by the byte-exact generated notice of the 1.94.0 release (same lockfile, COPYRIGHT and compiler-builtins licence as the toolchain's source commit); `compiler-builtins` stated as MIT AND Apache-2.0 WITH LLVM-exception with its complete licence file; the `blake2b_simd` upstream LICENSE with its copyright; the 166 packages described as the lockfile union, target-runtime crates distinguished from build-time ones.
- 1.3 (2026-09-05, BOSUN) — after Sol's second closing pass: Rust components named exactly (`std` and `panic_abort` included) with the complete library COPYRIGHT notice; MIT notices reproduced per crate; Liberation copyright added to the OFL text; DejaVu Sans Mono Bold and the embedding wording.
- 1.2 (2026-09-05, BOSUN) — after Sol's closing pass: BSD-2-Clause, BSD-3-Clause and Unicode-3.0 notices and the OFL text added; PDF fonts named per document; Rust runtime row; full ELF paths.
- 1.1 (2026-09-05, BOSUN) — after Sol's verification pass: compiled third-party code in the guest ELFs stated and listed per crate; licence texts added under `licenses/`; path corrected.
- 1.0 (2026-09-05, BOSUN) — written after Sol's release audit.
