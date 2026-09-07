# Referee trail

Twenty-one reports by a second model (OpenAI GPT-5.6 through `codex exec`, reasoning effort high for rounds 1 to 10 and
ultra for the rest), produced while the manuscript and its companions were drafted. The manuscript states that this is
not independent validation; the trail is published so that a reader can see what was found, in what order, and what
the authors did about it.

| file | what | effort |
|------|------|--------|
| `FINDINGS_round1.md` to `FINDINGS_round9.md` | manuscript referee rounds before v3.0 | high |
| `FINDINGS_round10.md` | verification pass on the additions of Sections 7.6 and 9.5 | high |
| `FINDINGS_companions_ultra.md` | audit of the two companions (v1.0) | ultra |
| `FINDINGS_round11.md` | eleventh pass on the publication bytes (manuscript v3.4, companions v1.3) | ultra |
| `FINDINGS_release_ultra.md` | audit of the staged release tree (README, notes, licence, metadata, publication script, kit manifest, trail) | ultra |
| `FINDINGS_round12_verification.md` | verification pass on the applied text: disposition of every earlier finding, regression sweep, source-closure and script checks | ultra |
| `FINDINGS_round13_closing.md` | closing pass on the tree after the verification pass was applied | ultra |
| `FINDINGS_round14_closing.md` | second closing pass, on the tree after the first closing pass was applied | ultra |
| `FINDINGS_round15_closing.md` | third closing pass, on the tree after the second closing pass was applied | ultra |
| `FINDINGS_round16_closing.md` | fourth closing pass, on the tree after the third closing pass was applied | ultra |
| `FINDINGS_round17_closing.md` | fifth closing pass, on the tree after the fourth closing pass was applied; it cleared that tree | ultra |
| `FINDINGS_round18_closing.md` | sixth closing pass, on the tree after the fifth closing pass's low items were applied; it cleared that tree too | ultra |
| `FINDINGS_round19_closing.md` | seventh closing pass, on the tree after the sixth closing pass's editorial items were applied; it found no defect, cleared that tree and accepted the stopping rule under which this report was added without a further pass | ultra |

Every report is reproduced with its substance unchanged. Where a report discusses a redacted token as evidence, the
token is shown as a bracketed placeholder and the report's proposed replacement is kept as written. The last report
in this directory examined the preceding staged tree; the present tree includes that report and the adjudicated
changes made in response to it, so its counts, digests, versions and cited line numbers describe the tree before
those changes. Mechanical redaction only: the runner's stray lines
(`codex`, `hook: Stop`, token counts, one duplicated copy of round 1), run identifiers, the rented machines' public
addresses and provider instance identifiers (which the reports quoted from the bundle's logs; the bundle's own copies
are redacted the same way), runner temporary paths and the development machine's absolute paths were removed or
replaced by bracketed placeholders; a path such as `<packet>/paper.md:512` refers to the packet of that round,
whose files are not shipped, and paths under `bundle/`, `paper/`, `companions/` and `referee_trail/` refer to this tree. One exception is
deliberate: `FINDINGS_round12_verification.md` quotes the absolute build paths that the round found in the bundle's
Cargo manifests, because the paths are the finding.
The referee wrote in the assistant's voice and signed as it; the sign-offs are normalised to `BOSUN`, the assistant's
internal call sign is bracketed where a report discusses it as a token, and the principal is named as such. Links
from the ten reports that examined this tree are relative to the tree root (`../`) and use GitHub's line-anchor form (`?plain=1#L<n>` for Markdown, `#L<n>` otherwise); a link whose target was later removed from the tree is given as an inline path; links quoted inside a report's fenced blocks are left as the referee wrote them.
