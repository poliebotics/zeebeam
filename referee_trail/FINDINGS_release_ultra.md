Hoy.

The staged tree is not safe to push. It has false publication status, an incomplete release ledger, a verification-to-commit gap, contradictory licence grants, potentially live infrastructure identifiers, and material scientific overclaims. No file was modified.

## Defects

1. **Must-fix | [README.md:5](../README.md?plain=1#L5) | proof scope.**  
   Quote: “every row’s recorded computation is proved”. Only rows 1-259 have final per-row proofs. Replace with:

   > For each of the 259 anchored rows released here, the recorded computation is proved in zero knowledge under one verification key.

2. **Should-fix | [README.md:7](../README.md?plain=1#L7) | unsupported location.**  
   Quote: “recorded on 22 August 2026 aboard CittaDel”. The bundle establishes the date, not that location. Delete “aboard CittaDel” or insert a bundled record proving it.

3. **Must-fix | [README.md:19](../README.md?plain=1#L19) | physical overclaim.**  
   Quote: “reproduces the state the proved frame was lit under”. Replace with:

   > the complete preceding-row record advances to \(S_t\), from which the nominal emission checked against the proved frame is derived

4. **Must-fix | [README.md:21](../README.md?plain=1#L21) | Table 1 is not stated faithfully.**

   | Current quote | Exact correction |
   |---|---|
   | “the projected pattern re-derived” | “the nominal emission pattern re-derived” |
   | “fixed camera and emission windows reduced to the network inputs” | “fixed camera and emission windows reduced to the coupling-network input; whole-plane camera reduction feeds pose; BLAKE3 tile tree and typed root recomputed” |
   | “commitment, verdict and saturation count are published” | “commitment, all eleven logit sums, verdict and saturation count are published” |
   | “a mainnet v6 transaction is parsed” | “a supplied v6 transaction is parsed, its ZIP-244 txid recomputed, its supplied Merkle branch checked to the supplied header, and the header hashed; canonical-mainnet status is checked out of band” |
   | “the anchored 260-row chain-log prefix hashes to the receipt’s contentRoot” | “BLAKE3 of the 260-row prefix equals the digest in `PREFIX.json`; `contentRoot` is SHA-256 of the domain, encoded length and canonical `PREFIX.json`” |
   | “the receipt opens the chameleon commitment” | Insert first: “SHA-256(receipt) equals the decrypted memo binding; validated \(Y,C\) then satisfy…” |

5. **Must-fix | [README.md:37](../README.md?plain=1#L37) | incomplete limitations.**  
   Replace the present P1/P2 sentence with:

   > P1, genuine sensor output, and P2, the inference from coupling score to recorded illumination, are unproved. Even with both, a live optical relay is not excluded, and neither scene identity nor location is authenticated. The chain proof establishes internal consistency of an operator-supplied log; it does not rehash all frame files or authenticate chronology or precommitment.

   Also replace “Rows 260 to 711 … are not proved” with:

   > Rows 260 to 711 have no proof under the full per-row relation, although their log transitions are covered by the session-chain proof.

   Replace “Section 10” with “Sections 9.4-9.7 and 11”.

6. **Must-fix | [README.md:70](../README.md?plain=1#L70) | beacon release-time claim.**  
   Quote: “rounds 31521605 to 31521705 were released when the schedule says.” Replace with:

   > the 66 distinct quicknet rounds used by the session, spanning 31521605 through 31521705, match the public schedule; no-early-release is assumption A4

7. **Should-fix | [README.md:53](../README.md?plain=1#L53) | incomplete inventory.**  
   Quote: “259 row proofs and the chain proof.” Insert “two previous-revision row proofs” between those groups.

8. **Blocking | [RELEASE_NOTES.md:14](../RELEASE_NOTES.md?plain=1#L14) | root ledger is incomplete.**  
   Quote: “lists every file in the tree but itself (3,080 entries).” There are 3,082 regular files. The root ledger also omits its nested ledger. For the current bytes, add:

   ```text
   bcd5a3135fa8b27b268a881211fc02f0d919f65db27923a6472041ca237b27c8  ./bundle/proofs_20260902/SHA256SUMS
   ```

   That produces 3,081 non-self entries. Add a one-file `SHA256SUMS` row to the contents table and regenerate all counts and hashes after the other repairs.

9. **Blocking | [RELEASE_NOTES.md:44](../RELEASE_NOTES.md?plain=1#L44) | missing artifact.**  
   Quote: “The archive location is not yet fixed.” Insert the actual stable download URL or DOI before publication.

10. **Blocking | [RELEASE_NOTES.md:48](../RELEASE_NOTES.md?plain=1#L48) | publication makes the staged text false.**  
    Quote: “Nothing in this tree has been published … will change at release.”

    The same defect appears in:

    - `paper/zeebeam.md`: “The private bundle…” and “Still to add before release: the DOI and licence”.
    - `companions/zeebeam_for_dummies.md`: “The bundle is private … a licence … still to be decided.”
    - `bundle/proofs_20260902/VERIFY.md`: “Nothing here is published; this bundle is a private backup.”

    Replace these with durable public-release statements. Insert the final authorship decision, the existing licence, and either the DOI or “No persistent identifier has yet been assigned.” Item 005 must be retired and restaged because its frozen digest requires the false text.

11. **Must-fix | [RELEASE_NOTES.md:35](../RELEASE_NOTES.md?plain=1#L35) | false offline-manifest description.**  
    Replace “gives the source URL, fetch date and digest of every file” with:

    > documents provenance and abbreviated digests for the fetched top-level artifacts and describes the vendor trees; `SHA256SUMS` gives full digests for its 33,753 listed files

    Add that an end-to-end air-gapped rebuild has not been rehearsed.

12. **Blocking | [LICENSE:12](../LICENSE#L12) | internally contradictory commercial grant.**  
    Sections 1(b) and 1(d) grant verification and quotation without qualification; §2(a) prohibits every commercial use. Replace line 12 with:

    > 1. Permitted uses. Subject to section 3, you may, without charge and solely for non-commercial purposes:

    Then define “non-commercial”. README’s licence summary must use the resulting exact scope.

13. **Blocking | [LICENSE:6](../LICENSE#L6) | undefined licensed corpus.**  
    Quote: “The materials means everything in this repository”, versus §3: “This licence covers only the copyright holder’s own work.” Replace the definition with:

    > “The licensed materials” means the copyright holder’s own work in this repository and the separately archived offline kit. It excludes all third-party code and data identified under section 3.

14. **Must-fix | [LICENSE:17](../LICENSE#L17) | modification, redistribution and downstream rights do not cohere.**  
    The licence permits modifying everything, including `LICENSE` and `CITATION.cff`, while §2(d) forbids altering them. It permits publishing “results”, prohibits substantial redistribution, then preserves downstream rights it never grants.

    Exclude `LICENSE` and `CITATION.cff` from modification. Define “published results” as descriptions, measurements and conclusions, or add an express modified-material redistribution and sublicensing grant.

15. **Should-fix | [LICENSE:19](../LICENSE#L19) | attribution and statutory rights.**  
    `CITATION.cff` does not define an attribution formula. Replace with “author, title, version and repository-code recorded in `CITATION.cff`”. Add:

    > Nothing in this licence restricts any use permitted by applicable law without the copyright holder’s permission.

16. **Must-fix | [LICENSE:32](../LICENSE#L32) | third-party licence inventory missing.**  
    `jubjub` 0.10.0 is adequately evidenced as MIT or Apache-2.0, and licence texts are listed for the named `sha2` and `bls12_381` copies. But 35 of 166 guest crate directories and 152 of 644 host directories lack a conventional licence/notice path: 156 unique crate-version directories.

    Add `THIRD_PARTY_NOTICES.md` mapping every crate, toolchain archive and fetched-data file to source, version, licence expression, notices and scope. Absence of a conventional file is an evidence gap, not proof that a crate is unlicensed.

17. **Must-fix | [CITATION.cff:3](../CITATION.cff#L3), .zenodo.json:2 (`.zenodo.json`, line 2) | manuscript metadata masquerades as software-release metadata.**  
    Both use the manuscript title and version 3.4 while declaring software. No repository/software version exists.

    If this is a software release, use:

    ```text
    title: ZeeBeam
    type: software
    version: "<repository release version>"
    ```

    Use the same release version in Zenodo and put the v3.4 manuscript in CFF `preferred-citation`. Official CFF guidance makes root fields describe the software. [CFF schema guide](https://github.com/citation-file-format/citation-file-format/blob/main/schema-guide.md)

18. **Must-fix | [CITATION.cff:6](../CITATION.cff#L6) | unresolved authorship already frozen.**  
    CFF and Zenodo declare Cathal alone, while the manuscript says authorship remains unsettled. Decide it, then make the manuscript, release notes, CFF and Zenodo identical.

19. **Must-fix | .zenodo.json:25 (`.zenodo.json`, line 25) | mixed deposit mislabeled by one licence.**  
    Quote: `"license": "other-nc"`. That cannot accurately govern MIT/Apache third-party files which `LICENSE` excludes. Append:

    > The ZeeBeam Research and Private Use Licence v1.0 applies only to Cathal Ryan Hynes’s own work; third-party code and data retain the licences and terms identified in LICENSE and THIRD_PARTY_NOTICES.md.

    Declare the custom and third-party licences in the Zenodo draft. `access_right: open` is not itself inconsistent with non-commercial reuse. [Zenodo licence guidance](https://help.zenodo.org/docs/deposit/describe-records/licenses/)

20. **Must-fix | CFF and Zenodo descriptions | circuit/mainnet conflation.**  
    Both say the proof establishes a “Zcash mainnet transaction”. Replace that clause with:

    > a supplied Zcash v6 transaction is recomputed from raw bytes and its branch checked to a supplied block header; canonical-mainnet status is an out-of-band check

21. **Must-fix | naming consistency.**  
    The former title phrase does not survive only at the two authorised locations. Remove it from:

    - `paper/zeebeam.md:19` and `:1037`, then regenerate HTML/PDF.
    - `extras/005_zeebeam_repo.preview.md:52`.
    - `extras/offline_kit_MANIFEST.md:8`, replacing the title with `# Offline kit for ZeeBeam`.
    - `bundle/.../all_rows/figures/all_rows_figures.py:4`, replacing `zk_all_the_things.py` with `the ZeeBeam light-surface palette`.

    Replace companion front matter exactly:

    ```text
    companion_to: ZeeBeam manuscript v3.4 (paper/zeebeam.md)
    ```

    and, for the plain-language companion:

    ```text
    companion_to: ZeeBeam manuscript v3.4 (paper/zeebeam.md); ZeeBeam, by Example v1.3 (companions/zeebeam_worked_examples.md)
    ```

    Historical logs saying “Dark Lantern the project” should say the intermediate naming was superseded and Dark Lantern is only the wider programme. Current non-log uses are otherwise consistent. The principal's crew name is absent.

22. **Must-fix | [zeebeam.md:108](../paper/zeebeam.md?plain=1#L108) | stale manuscript bookkeeping.**

    - “eight manuscript referee rounds” must be “ten”.
    - Line 476’s “eight referee reports” must be “ten”.
    - Appendix C’s `VERIFY.md (v2.4)` must be `v2.6`.
    - Regenerate HTML and PDF.

23. **Must-fix | verifier dependency descriptions.**  
    These quotes are false:

    - `VERIFY.md`: “`sp1-verifier` 6.4.0 only”.
    - `Cargo.toml`: “with sp1-verifier alone”.
    - Manuscript: “built `--locked` and nothing else”.

    The verifier directly depends on `sha2 = "0.10"`. Replace with:

    > Cryptographic verification uses `sp1-verifier` 6.4.0; the utility also directly uses `sha2` 0.10 for digest handling.

24. **Must-fix | [figures/zeebeam.py:12](../figures/zeebeam.py#L12) | source does not reproduce the staged figure and overclaims chronology.**

    - Default status still says “ceremony 2 in progress”; replace it with the status embedded in the staged PNG/SVG.
    - “captured 22 August” should be “from the operator-supplied session dated 22 August”.
    - “nothing here is trusted” should be “all displayed relations are circuit-checked against pinned constants”.
    - “nobody chose it after the fact” should be “the nominal emission is a pure function of the supplied chain state”.
    - The timing footer must include A4 and A8 and the fixed-predecessor condition.
    - Replace the nonexistent public source `docs/research/zeebeam_one_proof_architecture_20260901.md` with `paper/zeebeam.md §§5-7, 9`.

    The `ZeeBeam` title, ten-round count and 259-row/chain status in the rendered footer are correct.

25. **Blocking | [005_zeebeam_repo.sh:22](<extras>/005_zeebeam_repo.sh:22) | “no unlisted file” check is false.**  
    `! -name SHA256SUMS` hides every nested checksum file. After fixing the root ledger, replace it with:

    ```bash
    ! -path './SHA256SUMS'
    ```

    Preview “3,081 files” must become the final actual total, currently 3,082. “tree verified: 3080 files” currently means ledger entries, not files.

26. **Blocking | [005_zeebeam_repo.sh:15](<extras>/005_zeebeam_repo.sh:15) | verified bytes are not necessarily committed bytes.**  
    The script checks the worktree before `git add`, hashes the message before `git commit -F` rereads it, ignores modes and symlinks, permits Git filters/hooks, and never verifies the resulting commit tree or message.

    After `git add`, compare `git write-tree` with a frozen expected tree OID. After commit, verify `HEAD^{tree}` and the committed message, capture the commit OID, and push that OID explicitly:

    ```bash
    COMMIT=$(git rev-parse HEAD)
    test "$(git rev-parse "$COMMIT^{tree}")" = "$EXPECTED_TREE"
    git push origin "$COMMIT:refs/heads/main"
    ```

    Reject all non-directory/non-regular nodes; test `.git` with `[ -e .git ] || [ -L .git ]`. Set `LC_ALL=C`.

    For the actual current ledger, `cut -c67-`, quoting, the shared `C.UTF-8` sort order and `set -euo pipefail` work. There are currently no symlinks, special nodes or `.git`.

27. **Must-fix | [005 preview:64](<extras>/005_zeebeam_repo.preview.md:64) | preview and commit claims are knowingly false.**

    - It admits authorship, licence wording, status, DOI and archive location remain open.
    - “no names, contact details, addresses” is false: the trail contains machine paths and session IDs; the script publishes `bosun@cittadel.poliephleet.ie`.
    - “only money figures are the $1.99-per-hour box costs” is false. Derived figures include `$0.418`, `$1.22717`, `$1.55883`, `$1.56` and `$135.09`.
    - Commit title “release candidate 3.4” mistakes the manuscript version for a release version.
    - Repository description’s “one CittaDel session” is unsupported and “a drand round” understates the two-round final relation.

28. **Must-fix | [offline_kit_MANIFEST.md:48](<extras>/offline_kit_MANIFEST.md:48) | wrong digest.**  
    Quote: `2c6be8a0…6a2785`. Actual:

    ```text
    2c6be8a0eac0663b54378fe50b4d03b2a6f70790d50bc0f8ee6d121f16f2a785
    ```

29. **Must-fix | [offline_kit_MANIFEST.md:71](<extras>/offline_kit_MANIFEST.md:71) | offline rebuild claim is false.**  
    Quote: “Go 1.23.4 for the native Groth16 wrap.” The vendored `sp1-recursion-gnark-ffi` requires Go 1.24.0. The kit also omits Go modules and host Rust/Cargo 1.98. Under Go’s rules, 1.23.4 downloads a newer toolchain or refuses. [Go toolchain rules](https://go.dev/doc/toolchain)

    Replace the claim, including in `ENVIRONMENT.md` and `source/README.md`, with:

    > The crate vendors support an offline guest-ELF rebuild given a preinstalled Rust/Cargo 1.98 host toolchain. The kit does not contain the newer Go toolchain or Go modules required for an air-gapped native-Groth16 wrap, which has not been rehearsed.

30. **Must-fix | referee trail publication debris and internal metadata.**

    - `FINDINGS_round1.md` begins `codex`, ends its first copy with `tokens used` / `265,863`, then duplicates the report. Delete line 1 and lines 230-459.
    - Delete the two session IDs:
      - `[run identifier redacted]`
      - `[run identifier redacted]`
    - Delete 20 runner lines: `hook: Stop` and `hook: Stop Completed`.
    - There are 96 `/home/c/` path occurrences on 63 lines across all 11 files. Example:  
      `<packet>/...`  
      Replace with public relative links where targets exist; unlink unavailable snapshots.
    - `— [internal assistant call sign redacted] ⚓` occurs 12 times because of the duplicate. Replace public-facing signoffs with `BOSUN`, or knowingly approve disclosure of the internal run identity.
    - Change README/preview “unedited” to “mechanically redacted for runner metadata and machine paths; substantive findings unchanged.”

    I found no human personal names, postal addresses, email addresses or key material in the trail. Its money figures are legitimate research-cost evidence. The sharpest tone is “operationally vacuous”, “continue the same untidy tradition”, “Honesty grade: A”, and “tonight’s fixable residue”. Those are severe but professional; retain them if the point is an adversarial trail.

31. **Blocking | bundled infrastructure disclosure.**  
    `all_rows/logs/fleet_instances.json` publishes eight instance IDs and public IPs as `"status": "active"`, all with `"launched_by": "the principal..."`:

    ```text
    [provider identifier redacted]  [public address redacted]
    [provider identifier redacted]  [public address redacted]
    [provider identifier redacted]  [public address redacted]
    [provider identifier redacted]  [public address redacted]
    [provider identifier redacted]  [public address redacted]
    [provider identifier redacted]  [public address redacted]
    [provider identifier redacted]  [public address redacted]
    [provider identifier redacted]  [public address redacted]
    ```

    `ml/realness/code/RUNBOOK.md` additionally publishes:

    ```text
    [public address redacted]  [public address redacted]  [public address redacted]
    [public address redacted]  [public address redacted]  [public address redacted]
    ```

    Confirm termination before publication, pseudonymize IDs/IPs, replace `[principal call name redacted]` with `operator`, and insert verified termination dates. If these machines remain reachable, do not publish.

32. **Must-fix | unintended bytecode disclosure.**  
    Remove `verifier/__pycache__/decode_statement.cpython-314.pyc`. It embeds:

    ```text
    bundle/proofs_20260902/verifier/decode_statement.py
    ```

    Regenerate both ledgers. This also makes the source’s “portable, no absolute paths” claim true of the distributed verifier directory.

33. **Should-fix | internal names.**  
    `[internal assistant call sign redacted]` occurs 114 times across 33 public files; `[principal call name redacted]` appears in companion logs and fleet metadata. Keep frozen evidentiary run labels only if intentional and defined; replace public-facing authorship/log labels with `BOSUN` and “the principal”. In `LICENSE:28`, delete the needless new `Truth Beam, ` entry.

34. **Nit | remaining exactness.**

    - Companion “confirm, in about a second” should be “in about 1.6 seconds”.
    - Offline ledger description “lists every file” should be “lists every other file”.
    - “Lambda base image (not archivable)” should be “Lambda base image, not archived and not pinned”.

## Counts and versions checked

| Item | Value stated in tree | Established |
|---|---:|---:|
| Public regular files | 3,081 in preview | **3,082** |
| Root ledger entries | 3,080 | 3,080, all hash correctly; nested ledger omitted |
| Correct current non-self coverage | “every file” | **3,081** |
| Paper / companions / figures | 3 / 6 / 3 | 3 / 6 / 3 |
| Bundle files / ledger entries | 3,053 / 3,052 | 3,053 / 3,052, complete and all pass |
| Referee trail / named front matter / root ledger | 11 / 5 / omitted | 11 / 5 / 1 |
| Session / anchored-prefix rows | 712 / 260 | 712 / 260 |
| Final row proofs | 259 | 259 |
| Fleet / ceremony-2 final proofs | 257 / 2 | 257 / 2 |
| Previous-revision / chain proofs | unstated / 1 | 2 / 1 |
| Raw proof files | unstated | 262 |
| Final proof / statement size | 356 / 1,101 bytes | exact for all 259 |
| Used quicknet rounds | 66 | 66 |
| Round endpoints | 31521605-31521705 | correct; 66 used among a 101-integer span |
| Offline ledger / total files | 33,753 / implicit 33,754 | 33,753 complete and passing / 33,754 |
| Guest / host crate directories | 166 / 644 | 166 / 644 |
| Guest / host vendor files | unstated | 5,491 / 28,247 |
| Offline top-level artifact sizes | 10 quoted | 10 exact |
| Offline abbreviated digests | 10 quoted | 9 exact, 1 wrong |
| Offline archive size | 683,543,615 bytes | exact |
| Approximate proving cost | USD 140 | approximately USD 137.8 from listed components |
| Largest public file | under 5 MB | 1,171,416 bytes |
| Public logical / allocated size | 23 MB | 17,698,131 / about 24 MB |
| Trail manuscript reports / companion audit | 10 / 1 | 10 / 1 |
| Trail path occurrences / session IDs / hook lines | unstated | 96 / 2 / 20 |
| Infrastructure IDs / IP occurrences | unstated | 8 / 14, 13 unique |
| CFF authors / keywords | 1 / 11 | structurally valid; authorship unresolved |
| Zenodo creators / keywords | 1 / 11 | structurally valid; authorship unresolved |

| Version | Tree | Established |
|---|---:|---:|
| Manuscript | 3.4 | 3.4 in Markdown, HTML and PDF |
| Companions | 1.3 | 1.3 in all formats |
| Release notes | 1.2 | 1.2 |
| Custom licence | 1.0 | 1.0 |
| CFF schema | 1.2.0 | Requested fields schema-valid |
| CFF/Zenodo work version | 3.4 | Only manuscript 3.4 established; no release version |
| `VERIFY.md` | 2.6 | 2.6; manuscript incorrectly says 2.4 |
| `ENVIRONMENT.md` | 1.3 | 1.3 |
| Offline manifest | 1.0 | 1.0 |
| Standalone verifier crate | 0.1.0 | 0.1.0 |
| SP1 / circuit artifacts | 6.4.0 / 6.1.0 | correct |
| Guest Rust | 1.94.0 | rustc 1.94.0-dev |
| Host Rust/Cargo | 1.98.0 | used in assembly; absent from kit |
| Included / required Go | 1.23.4 / unstated | 1.23.4 / at least 1.24.0 |
| `jubjub` / `sha2` / `bls12_381` | 0.10.0 / 0.10.9 / 0.8.0 | correct |

## Hashes, keys and commands

| Check | Result |
|---|---|
| Bundle `sha256sum -c SHA256SUMS` | 3,052/3,052 pass; complete |
| Root `sha256sum -c SHA256SUMS` | 3,080/3,080 listed files pass; coverage incomplete |
| Root-ledger SHA-256 | `f07f8fe61f9adf13387c56316876c42c0ba221c8951399b78bbbcbc88259a7a3`, exact current bytes |
| Nested-ledger SHA-256 | `bcd5a3135fa8b27b268a881211fc02f0d919f65db27923a6472041ca237b27c8` |
| Commit-message SHA-256 | `6607b44d2c31a5343ceed937a136e9fafa4c159cb5677709603ebb8a3b888add`, exact |
| Offline archive SHA-256 | `b5b9d5f37bfedf9a9e7127f710e20bdfaa8a16bccce921490c50d3f79330b174`, exact; archive test passed |
| Row verification key | `0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490`, exact |
| Chain verification key | `0x005402a848fdc787e32c0a110ca2662dd61c580481b4e418d5400a52fbd7df50`, exact |
| Block hash / transaction ID | Exact matches to pins and bundle |
| README paths and binary name | Correct |
| Retained standalone verification | Row 001 and chain verify; byte-flip and wrong-key controls reject |
| CFF parsing | YAML valid; requested 1.2 fields valid |
| Zenodo parsing | JSON and requested field forms valid; semantic defects remain |

The read-only audit could not rerun `cargo build`, because it writes `target/`. The staged sources are byte-identical to the retained successful build source; the retained binary is 917,512 bytes.

## Licence and metadata open questions

This is drafting review, not legal advice.

1. What counts as commercial: paid teaching, grants, commercial collaborators, cloud use, consulting, publication fees or cost recovery?
2. May research groups share copies? Are public forks, mirrors and issue attachments permitted?
3. Are modified code, proofs, statements, models, outputs and independent reimplementations “published results” or prohibited derivatives?
4. Is non-commercial model training permitted, and may the resulting model be distributed?
5. Is any sublicensing intended?
6. What patent rights are “necessary” under §4?
7. Which exact files and passages are Cathal’s own work, given BOSUN drafting and third-party content?
8. Were redistribution rights for the complete Blockchair responses confirmed?
9. What licences govern each drand wrapper, toolchain tarball and vendor component?
10. Is Cathal the final sole creator? Is PolieBotics an affiliation, trading name or rights holder?
11. What repository/software release version should replace manuscript version 3.4?
12. Is the Zenodo record software, a manuscript deposit, or a mixed research compendium?
13. Can the Zenodo importer represent the required custom/mixed licences, or must the imported draft be corrected manually?
14. Does the licence need cure/reinstatement, severability, governing law or a liability limitation?
15. Is `bosun@cittadel.poliephleet.ie` intentionally being published as the permanent commit contact?
16. What stable URL or DOI will hold the offline archive?

Verdict: not yet.

— BOSUN ⚓
