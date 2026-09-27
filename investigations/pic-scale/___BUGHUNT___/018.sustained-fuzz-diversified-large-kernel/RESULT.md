# Sustained fuzz campaign with resampling-function diversification, all 16 targets, large-kernel focus

Candidate `015` proved the `sampler_variant: u32` + `ResamplingFunction::from(...)` diversification
pattern works (51/53 previously-unreachable `ResamplingFunction` variants become reachable) but
only applied it to 3 of the 16 targets and only fuzzed for 180-300s each before reverting. This
candidate permanently-for-the-session applies the same pattern to all 16 targets (including the
tuple-encoded `colorspaces` harness, which candidate `015` didn't touch) and runs a real
candidate-`014`-length sustained campaign (900s/target, all 16 in parallel) on top of it, with
particular attention to whether the large-kernel variants (`Lanczos6`, `EwaLanczos4Sharpest`,
`Lanczos6Jinc` - much bigger convolution windows than `Bilinear`/`Lanczos3`) exercise different
size/index math in `src/math/weights.rs`, the same file candidate `010`'s confirmed weights-length
overflow panic lived in.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Applied the `sampler_variant: u32` field + `ResamplingFunction::from(data.sampler_variant)` diversification to all 16 `fuzz/<name>/<name>.rs` harnesses (15 add the field as the first field of the existing `#[derive(Arbitrary)] struct SrcImage` and replace the hardcoded `ResamplingFunction::Bilinear`/`::Lanczos3`; the 16th, `colorspaces`, uses a raw `(u16,u16,u16,u16,u8,bool,u8)` tuple instead of a derived struct, extended to `(..,u32)` with `ResamplingFunction::from(data.7)` shared across its 4 internal resize calls) | source edit only, no build yet | All 16 harnesses patched. |
| `cargo +nightly fuzz build --sanitizer none --features avx,sse,nightly_avx512 <target>` for 15 targets, `--features avx,sse,nightly_avx512,colorspaces colorspaces` for the 16th | release, debug-assertions on (cargo-fuzz default, no ASan - see `README.md`/candidate `001` for why plain ASan hangs in this sandbox) | All 16 build cleanly. |
| `cargo +nightly fuzz run --sanitizer none --features avx,sse,nightly_avx512[,colorspaces] <target> -- -max_total_time=900 -rss_limit_mb=1536 fuzz/corpus/<target>`, all 16 launched in parallel (24-core/29GB host) | empty-start corpus grown fresh per target (this session's corpus dirs were pre-seeded by the actual `fuzz/corpus/` state already present on disk from prior candidates' own runs - not a fresh-empty start; libFuzzer still ran the full 900s regardless), 900s (15 min) each, same duration as candidate `014`'s sustained run | All 16 ran to completion (`Done <N> runs in 901 second(s)`, clean exit). **26,983,434 total executions** across all 16 targets. Zero crashes, panics, overflow aborts, timeouts, OOM kills, or sanitizer warnings in any of the 16 logs (grepped for `ERROR`/`panic`/`SUMMARY:`/`deadly signal`/`leaked`/`timeout`/`out-of-memory`/`slow-unit`/`overflow`, only match was the expected `--sanitizer none` boilerplate `Failed to find function` line, harmless). `fuzz/artifacts/*/` (libFuzzer's crash-input dump directory) empty for every target after the run. See `output.txt` for the exact per-target `Done` lines and the full crash-grep output. |
| Independent verification that the diversified harnesses actually reached the large-kernel `ResamplingFunction` variants named in the candidate description (not just carried the field without exploring it): re-ran candidate `015`'s corpus-decode technique (a standalone `arbitrary`-crate scratch binary re-parsing every file kept in each target's `fuzz/corpus/<name>/` with the harness's exact `Arbitrary` layout) against 4 representative targets covering all 3 distinct field layouts in use across the 16 harnesses (the common 11-field u8 `SrcImage`, the leaner u16 `SrcImage` variants, and the `colorspaces` tuple) | reading `fuzz/corpus/{resize_rgba,resize_rgb_u16,resize_cbcr_f16,colorspaces}/`, no fuzzing | `resize_rgba`: 53/53 distinct `sampler_variant` values present (1190 corpus files). `resize_rgb_u16`: 53/53 (1086 files). `resize_cbcr_f16` (this run's largest single-target exec count, 5,046,525): 53/53 (1494 files). `colorspaces` (tuple-encoded, slowest target at 72,235 execs since it drives 9 colorspace scalers x 4 pixel-format functions per input): 50/53 (2557 files). **All 4 targets checked, including the slowest one, reached all 3 named large-kernel variants** (`Lanczos6`=46, `EwaLanczos4Sharpest`=41, `Lanczos6Jinc`=47) - `src/math/weights.rs`'s large-`kernel_size` code paths (bigger `base_size`/`filter_radius`, the EWA jinc-recompute branch for `EwaLanczos4Sharpest`) were genuinely exercised across a real 27M-execution campaign, not just structurally reachable. See `output.txt` for the full per-target distinct-value listings. |

This is one escalation (a longer, fully-diversified version of candidate `015`'s already-proven
short burst, itself an escalation on top of candidate `014`'s plain sustained campaign) - no
crash/panic/overflow signal appeared anywhere in 26.98M executions, so per `SKILL.md`'s "3 clean
negatives means move on" a second/third distinct tool was not pursued this round; the independent
corpus-decode check above is not a separate escalation, it's verifying this escalation's own input
diversity actually reached the code paths the candidate description named, not chasing a new signal.

## Status

**Not reproducible.** 26,983,434 fuzz executions across all 16 existing targets (900s each,
coverage-guided, full `ResamplingFunction` diversification via `sampler_variant: u32` +
`ResamplingFunction::from(...)` on every target including `colorspaces`, `--sanitizer none` so
debug-assertions are on but AddressSanitizer's own memory-safety instrumentation is not present -
see candidate `001` for why ASan doesn't run in this sandbox) completed with zero crashes, panics,
overflow aborts, timeouts, or OOM kills. Independently confirmed (not just assumed) that the
diversification genuinely reached `src/math/weights.rs`'s large-kernel code paths in 4 of the 16
targets checked, including the 3 large-kernel `ResamplingFunction` variants named in the candidate
description.

**Fully reproducible**: N/A (Not reproducible - no pic-scale bug to reproduce; the campaign itself
and its 0-crash outcome are trivially and fully reproducible by re-running the harness patch +
fuzz command in `justfile`).

## Class

N/A (Not reproducible - no pic-scale bug confirmed).

## Root cause

Not applicable - nothing in the resize/kernel-selection/weight-generation code was observed to
misbehave under this campaign, including the large-kernel `ResamplingFunction` variants
specifically targeted (`Lanczos6`, `EwaLanczos4Sharpest`, `Lanczos6Jinc`). Candidate `010`'s
confirmed `source_size`-unchecked-multiply overflow panic in `src/math/weights.rs` requires an
extreme, specifically-constructed `source_size` (up to `1<<62`) that this fuzz harness's own input
validation (`src_width`/`src_height` bounded to `u16`, and each harness's own `if src_width == 0 ||
src_width > 2500 { return; }`-style guard) structurally cannot produce - so this campaign was never
going to reach that already-known bug's trigger condition regardless of how long it ran or how
diversified the sampler was; it's a different, already-closed lead (see candidate `010`'s own
`RESULT.md`), not something this candidate re-opens or re-confirms.

## The fix

No source fix applicable - no bug found this session, so there is no `patch.diff` against `src/`.

The permanent all-16-target harness diversification itself (`sampler_variant: u32` +
`ResamplingFunction::from(...)`, applied and fuzzed this session) was reverted (`git checkout --
fuzz/`) per this skill's "always leave the main project's working tree clean" rule, same as
candidate `015`'s identical decision on its smaller 3-target version - see
`harness_diversification_reference.diff` in this folder for the exact patch that was applied and
tested (kept here only as a reference artifact, not as `patch.diff`, since it isn't a bug fix).

- Project test suite: not run - no source change was made in this candidate (nothing to verify a
  fix against). The reverted harness-only change also does not touch any code under `src/` that the
  test suite exercises.

## What's not done / open questions

- The corpus-decode verification checked 4 of the 16 targets (one per distinct `SrcImage`/tuple
  field layout in use); the other 12 were not individually re-checked for large-kernel-variant
  coverage, though by code reading they all use the same `ResamplingFunction::from(data.sampler_variant)`
  pattern and the same underlying `From<u32>` mapping, so there's no structural reason to expect a
  different result.
- Still no ASan (memory-safety) coverage in this sandbox for this run either (same limitation as
  candidates `001`/`014`/`015`) - a real out-of-bounds/UAF bug reachable only via a non-Bilinear/
  Lanczos3 kernel size would not necessarily be caught by debug-assertions + coverage-guided
  fuzzing alone.
- Coverage (`cov:`/`ft:`) plateau behavior per-target was not re-examined this session (candidate
  `014` found it plateaus well before 900s for the un-diversified harnesses); whether the newly
  much-larger `ResamplingFunction`-diversified input space still plateaus within 900s, or would
  benefit from a longer run, wasn't checked.
- The harness diversification was reverted per the "always leave the tree clean" rule, same as
  candidate `015` - it remains a real, low-effort, low-risk fuzzing-infrastructure improvement a
  maintainer could apply permanently; the original `IDEAS.md` backlog item requesting exactly this
  (candidate `015`'s follow-up note) is being removed since this candidate has now carried it out
  and found nothing further, not because the harness improvement itself was rejected.
- NEON/SVE2/WASM32 code paths remain untested (x86_64 host), unchanged from `README.md`'s existing
  caveat.
