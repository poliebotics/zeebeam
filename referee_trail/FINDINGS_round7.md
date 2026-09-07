Hoy.

# 1. Round-6 disposition

| Carried item | Disposition | Evidence and residue |
|---|---|---|
| Fail-closed build driver | PARTIALLY | [build_reproducible.sh](<packet>/bundle/source/build_reproducible.sh:8) has `set -euo pipefail`, `--locked`, one-ELF counting and digest/key checks. However, the pins are overridable through `EXPECT_ELF_SHA` and `EXPECT_VKEY`; [source/README.md](<packet>/bundle/source/README.md:20) still directs strangers to the old fail-open script; and the supplied transcript is a development-machine run while [Section 12](<packet>/paper.md:796) calls it clean-machine evidence. |
| Correct ceremony-1 disclosure | RESOLVED | [Appendix C](<packet>/paper.md:872) correctly says `ZBAUGST2` publishes no round. Both 1,085-byte statements contain no round key. The [standalone verifier comment](<packet>/bundle/verifier/standalone_verifier/src/main.rs:6) accepts either relation length. All four regenerated transcripts match their artifacts and end in both rejections, `VERIFIED`, `exit=0`. |
| Repair oracle provenance | RESOLVED | [row_072_manifest.json](<packet>/bundle/final_relation/ceremony2/row_072_manifest.json:34), [boundary README](<packet>/bundle/final_relation/boundary_row_095/README.md:10), and [Section 7.4](<packet>/paper.md:425) explicitly supersede the frozen stale note. |
| Package empirical evidence | PARTIALLY | Cycle logs and GPU observations are present and hashed. The generated-fakes raw record supports its headline numbers. Replay/splice and black-box ℓ2 still retain exact results without run records, contrary to Round 6’s “package or remove” requirement. The realness harness is incomplete; see R7-N3. |
| Clean metadata | PARTIALLY | Frontmatter correctly says Round 7 and units are now KiB/MiB. Ceremony-1 row 72 nevertheless has the wrong RSS, and the completed-referee count remains five instead of six. |
| DOI and licence | OPEN | Neither exists. [Section 12](<packet>/paper.md:801) says so. |

| Finding | Disposition | Residue |
|---|---|---|
| R6-N1 build driver | PARTIALLY | New driver logic is much better, but environment-overridable pins and a README that invokes the old driver prevent a genuinely fail-closed stranger workflow. |
| R6-N2 oracle note | RESOLVED | Correction sidecars and manuscript disclosure are adequate. |
| R6-N3 ceremony-1 rounds | RESOLVED | Statement contents, Appendix C, verifier comment and transcripts agree. |
| R6-N4 missing measurements | PARTIALLY | Cycle and GPU evidence supplied; generated-fakes evidence supplied; two exact realness results remain unarchived. |
| R6-N5 metadata and units | PARTIALLY | Units and frontmatter repaired; one RSS value and the referee count are wrong. |

# 2. New findings and command record

The prescribed commands produced:

```text
$ cd bundle && sha256sum -c SHA256SUMS
195/195 entries: OK
exit=0

$ cd source/rust && sha256sum -c ../../final_relation/SOURCE_TREE_SHA256SUMS
38/38 entries: OK
exit=0
```

Each also printed three sandbox warnings:

```text
Failed to create stream fd: Operation not permitted
```

The decoder printed:

```text
total_bytes=1101
sha256=9b560f900965b110cb7d1e67d142e8bc8f8bca8143134659a34d540a116a5190
row_index=72
row_count=712
previous_drand_round=31521616
row_drand_round=31521616
pose.verdict=0
coupling.score_numerator=56821525
beacon.verified=1
trapdoor_flag=1
```

## R7-N1, MUST-FIX: the released build workflow is still not pinned

The new driver says it verifies the final pins, but lines 9–10 accept replacements from the environment:

```text
EXPECT_ELF_SHA="${EXPECT_ELF_SHA:-8bcadf...11db}"
EXPECT_VKEY="${EXPECT_VKEY:-0x0019d16c...0490}"
```

A mismatching build can therefore pass when those variables are inherited or supplied. That is configurable comparison, not assertion against the release pins.

Worse, the bundle’s source README still instructs:

```text
build_reproducible_20260902.sh
```

That historical script contains:

```text
set -u
cargo build ... | grep ... | head -20
E=$(find ... | head -1)
echo BUILD_REPRODUCIBLE_END
```

The command

```bash
rg -n 'clean-machine|build_reproducible_20260902|build_reproducible\.sh' \
  paper.md bundle/source/README.md bundle/VERIFY.md
```

printed, among other lines:

```text
paper.md:797:... with its clean-machine transcript,
bundle/source/README.md:20:| `build_reproducible_20260902.sh` | ...
bundle/source/README.md:38:... then run `build_reproducible_20260902.sh`.
bundle/VERIFY.md:39:... fail-closed build driver `build_reproducible.sh` ...
```

The transcript does end correctly:

```text
guest_elf_bytes=1171416
guest_elf_sha256=8bcadf5373742e92eb36535680e4b9f092c00fdcf9a8f7171678c49b170f11db
home_paths_in_elf=0
sp1_vkey=0x0019d16cfe911d943315c04235c792fcccc3e0759aa3095a941b8508178e0490
BUILD_REPRODUCED_OK ...
exit=0
```

It is the development-machine transcript named by the user, not a fresh-machine run.

## R7-N2, MUST-FIX: ceremony-1 row 72 RSS is wrong

The manuscript reports `111.6 MiB (114,260 KiB)`. The manifest and both process logs report `114,160 KiB`, which is `111.5 MiB` to one decimal.

Exact command:

```bash
python3 - <<'PY'
import json
for f in ['bundle/row_072_manifest.json','bundle/row_096_manifest.json',
          'bundle/final_relation/ceremony2/row_096_manifest.json',
          'bundle/final_relation/ceremony2/row_072_manifest.json']:
    d=json.load(open(f))
    print(f,d['host_peak_rss_kib'],f"{d['host_peak_rss_kib']/1024:.1f} MiB")
PY
```

Output:

```text
bundle/row_072_manifest.json 114160 111.5 MiB
bundle/row_096_manifest.json 114260 111.6 MiB
bundle/final_relation/ceremony2/row_096_manifest.json 113760 111.1 MiB
bundle/final_relation/ceremony2/row_072_manifest.json 114240 111.6 MiB
```

## R7-N3, MUST-FIX FOR ARTIFACT CLAIMS: the realness “harness” is incomplete

The packaged scripts import an absent `realness` package. Exact command:

```bash
PYTHONDONTWRITEBYTECODE=1 python3 bundle/ml/realness/code/run_fixture.py --help
```

Output:

```text
ModuleNotFoundError: No module named 'realness'
exit=1
```

The result record pins a script digest absent from the bundle. Its config digest does match, but its run metadata conflicts with that config:

```text
record_script_sha256 fdb8745de31430b562d62e573b235052c973cb3323abfe1ae20204b42f90fe43
bundled_script_match False
record_forger_seeds [100000]
config_forger_seeds [11, 22, 33]
```

`100000` also names the checkpoint step, so the paper’s “one forger training seed” provenance is not cleanly supported. The raw result itself is sound: each of three evaluation seeds has six real and six fake scores, zero errors at its recorded threshold, and all 36 real/fake pairwise comparisons correctly ordered.

## R7-N4, EDITORIAL: the referee count is stale

```bash
rg -n 'five manuscript|refereed this manuscript|five referee|seventh referee' paper.md
```

printed:

```text
paper.md:14:... seventh referee round ...
paper.md:96:... five manuscript referee rounds ...
paper.md:415:... has refereed this manuscript five times.
paper.md:421:... the five referee reports ...
```

Rounds 1 through 6 are complete. Those three occurrences must say six.

## R7-N5, STATISTICAL WORDING

Section 8.1 says the bootstrap interval `[1.0, 1.0]` is narrow “only because the sample is.” Small samples do not intrinsically narrow intervals. This interval is degenerate because the empirical bootstrap resamples only five perfectly separated observed clusters per class. It does not express population uncertainty. Say that.

# 3. Numeric check of Sections 6–8

| Claim | Check against bundle |
|---|---|
| Row-96 instructions | 4,149,712,293 exactly |
| Row-96 table sum | 4,149,712,293; delta 0 |
| Five main shares | 51.596851%, 16.934086%, 10.146374%, 8.311808%, 8.169183% |
| “Other” | 27,764,103 + 65,149 + 4,137 + 13,780 = 27,847,169 |
| Row-72 instructions | 4,148,971,330 exactly |
| Host execution | 61.501 s row 96; 58.564 s row 72 |
| Proof times | 850.972, 769.575, 857.424, 749.813 s, all exact |
| Costs | 47/60 × $1.99 = $1.55883; 37/60 × $1.99 = $1.22717 |
| Artifact sizes | Framed 2,780/2,779/2,795/2,795 B; raw 356 B; statements 1,085/1,101 B |
| RSS | Three rows correct; ceremony-1 row 72 should be 114,160 KiB = 111.5 MiB |
| GPU | Sidecar records approximately 26.5 GB and exact ceremony-2 observations 25,615/27,279 MiB; explicitly observations, not peaks |
| Coupling split | 424 fit, 72 architecture-selection |
| Coupling separation | Paired 72/72; pooled 5,175 wins, 9 losses, 0 ties; AUROC 0.9982638889 |
| Coupling extrema | Minimum matched −1,892,996; maximum crossed 20,290,152; two crossed positive |
| Parity | Native/Python 144/144; guest/Python on rows 72, 95 and 96 |
| Pose split | 461/116, overlap 0, union 577 |
| Pose accuracy | Integer 101/116; float 102/116; agreement 114/116 |
| Seed spread | `[102, 93, 104, 102, 92]`; median 102 |
| Temporal medians | All ten manuscript values match |
| Direction gaps | 10.8158, 8.2266, 19.7897 and 13.9829 percentage points |
| Cue delays | Row 72: 14.932 s; row 96: 4.932 s |
| Generated realness | Three evaluation seeds, each 6 real/6 fake; zero fake passes, zero real rejections, raw AUROC 1.0 |
| Bootstrap CI | `[1,1]` appears in the record; independent reproduction is blocked by missing cluster assignments/modules |
| Replay/splice | Six versus eighteen and nine runs appear only in the narrative note; no raw record |
| Black-box ℓ2 | ε=16 and 1,000 queries appear only in the narrative note; no raw record |
| White-box | Correctly reported as unfinished |

Sections 6–8 have one numeric error: row-72 RSS. Generated-fakes arithmetic is supported. The other two exact realness results remain anecdotes.

# 4. Strongest and weakest claims

The strongest claim remains Theorem 1’s execution binding for rows 72 and 96: two valid final-relation proofs, exact 1,101-byte statements, a byte-reproduced ELF and key, complete hashes, and circuit-independent agreement on every statement byte.

The weakest substantive claim is the physical corollary. P1 assumes genuine sensor bytes, P2 assumes the discriminator means illumination, A8 is an undeveloped application premise, optical relay survives, the operator manufactures the session tree, and only one local transition is proved. The weakest factual claims are the unarchived replay/splice and black-box ℓ2 numbers.

# 5. Recommendation

For IEEE S&P or CCS: **weak reject**. Correctness is close to minor revision after the fixes below. Significance remains below the bar. The proved result is exact execution of one large integrated program; the security meaning depends on premises that assume away raw injection and assign physical meaning to an ad hoc model. The anchor binds nothing against its operator, and the session proof covers one transition.

For Section 8 as diagnostics inside a systems paper: **minor revision, retain**. The coupling and pose accounting is unusually candid. The generated-fakes record is a useful small-sample diagnostic after its provenance package is repaired. None of Section 8 supports a security guarantee.

Minimum correctness and evidence changes that can be made tonight:

| Required change | Minimum satisfactory repair |
|---|---|
| Build workflow | Remove environment pin overrides; make `source/README.md` invoke `build_reproducible.sh`; label the old driver historical; correct “clean-machine transcript,” or rerun the new driver on a fresh machine with command, script hash and exit recorded |
| RSS | Change ceremony-1 row 72 to `111.5 MiB (114,160 KiB)` |
| Realness evidence | Restore the missing generating script and `realness/` modules, explain the seed/config discrepancy, and correct the contradictory JSON label |
| Unarchived results | Recover the replay/splice and ℓ2 outputs, or delete their exact success/sample/ε/query numbers from Section 8.1 |
| Statistics | Replace the misleading bootstrap explanation |
| Audit metadata | Change five completed manuscript rounds to six |
| Release | Add an explicit licence before release and the DOI upon deposit |

The significance judgement cannot be repaired tonight: acquisition authenticity, model validity, global-chain consistency and a non-equivocable anchor remain outside this result. No new proof or ML experiment is required to make the present artifact honest and internally closed.

— BOSUN ⚓
