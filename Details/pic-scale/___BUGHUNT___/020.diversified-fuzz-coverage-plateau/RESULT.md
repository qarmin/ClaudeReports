# Diversified-fuzz coverage-plateau check (follow-up to candidate 018)

Candidate `018` proved the `sampler_variant: u32` + `ResamplingFunction::from(...)` diversification
patch finds 0 crashes over a full 16-target/900s-each run, but only checked crash outcome and the
final reached-`ResamplingFunction`-variant-set, not whether libFuzzer's own coverage counters
(`cov:`/`ft:`) keep growing meaningfully past 900s for this much-larger diversified input space, or
plateau early the same way candidate `014`'s un-diversified run did. This candidate re-applies the
same harness diversification and runs a 2x-longer (1800s/target instead of 900s/target), cold-start
(empty corpus, not the pre-seeded `fuzz/corpus/` candidate `018` reused), timestamp-snapshotted
campaign on 8 of the 16 targets to directly measure the growth curve.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Re-applied candidate `018`'s saved `harness_diversification_reference.diff` (`sampler_variant: u32` + `ResamplingFunction::from(...)` on all 16 harnesses, unchanged from that candidate) | `git apply` against the current tree (same base commit as candidate 018, `7eec7cf`) | Applied cleanly, all 16 harnesses patched. |
| `cargo +nightly fuzz build --release --sanitizer none --features avx,sse,nightly_avx512` (15 targets) + `...,colorspaces colorspaces` (16th) | release, debug-assertions on, coverage-guided (cargo-fuzz default, `--sanitizer none` per this sandbox's ASan-hang limitation, see candidate `001`) | All 16 build cleanly (the first command's expected non-zero exit for `colorspaces` alone, same as candidate `018`'s own build recipe). |
| `./target/.../<target> -max_total_time=1800 -rss_limit_mb=1536 -print_final_stats=1 <fresh-empty-corpus-dir>`, 8 representative targets (one per distinct `SrcImage`/tuple field layout family plus the largest/most-complex-per-input target `colorspaces`) run in parallel, **fresh empty corpus** per target (not candidate `018`'s pre-seeded `fuzz/corpus/`, so the growth curve reflects a genuine cold start, same starting condition as candidate `014`'s original plateau observation) | `resize_rgba`, `resize_rgba_u16`, `resize_cbcr_f16`, `resize_plane_u16`, `resize_rgb_f32`, `resize_cbcr8`, `resize_rgba_f16`, `colorspaces`; 1800s each (2x candidate `018`'s 900s), coverage-guided; `cov:`/`ft:` snapshotted at ~420s, ~900s, ~1500s and 1800s (final) by grepping the last matching line in each target's live log at each checkpoint | All 8 ran to completion (`Done <N> runs in 1801 second(s)`). **96,269,163 total executions** across the 8 targets. Zero crashes, panics, overflow aborts, timeouts, OOM kills, or sanitizer warnings in any log (same grep as candidates `014`/`018`, only match the harmless `--sanitizer none` boilerplate line). `fuzz/artifacts/<target>/` empty for all 8 after the run. **Coverage growth curve**: `cov:` (edge coverage) was flat or within 1 unit of its ~420s value for 7 of 8 targets all the way to 1800s (e.g. `resize_rgb_f32` stayed at exactly `cov: 925` from ~420s through 1800s; `resize_cbcr8` at `cov: 775` the whole time). `ft:` (finer-grained feature counters) crept up slowly throughout for all targets but by small amounts relative to the initial burst (e.g. `resize_rgba`: `ft: 4106` at ~420s -> `ft: 4310` at 1800s, +5% over the remaining 23 minutes). The one target that kept growing measurably was `colorspaces` (the slowest-executing target by far, 189 exec/s vs 2,029-13,749 exec/s for the others, since it drives 9 colorspace scalers x 4 pixel-format functions per input): `cov: 2786 -> 2826 -> 2828 -> 2828` and `ft: 10399 -> 11029 -> 11328 -> 11448` - most of its growth also happened by ~900s, with the 900s-1800s window adding far less (`ft` +419 in the first 480s-900s window vs +120 in the 1500s-1800s window) - a decelerating, plateauing curve, not a sustained linear one. See `output.txt` for the full per-checkpoint table and final stats. |

This is one escalation past candidate `018` (a longer, cold-start, timestamp-instrumented version
of the same tool/config) - no crash/panic/overflow signal appeared anywhere in ~96.3M additional
executions, and the coverage-growth question itself resolved cleanly (plateau, not sustained
growth) on the first escalation, so per `SKILL.md`'s "3 clean negatives means move on" a second/
third distinct tool was not pursued.

## Status

**Not reproducible** (no crash/panic/overflow found) - **and the coverage-plateau question is
answered**: the `ResamplingFunction`-diversified input space plateaus in `cov:`/`ft:` within
roughly the same 900s window candidate `014`'s un-diversified run did, for 7 of 8 targets checked
essentially by minute 7 already; the 8th (`colorspaces`, the slowest per-input target) takes
somewhat longer to plateau (still growing modestly at 900s) but is also flat by 1500-1800s. A
longer run does not meaningfully change the picture - it does not surface a new crash and it does
not keep discovering new coverage at a rate that would justify running these particular 8 targets
longer than ~900-1500s in a future session.

**Fully reproducible**: N/A (Not reproducible - no pic-scale bug to reproduce; the coverage-growth
measurement itself is fully reproducible by re-running the harness patch + fuzz command with
`-print_final_stats=1` and grepping `cov:`/`ft:` at wall-clock checkpoints, exactly as this
candidate's `justfile` does).

## Class

N/A (Not reproducible - no pic-scale bug confirmed).

## Root cause

Not applicable - nothing in the resize/kernel-selection/weight-generation code misbehaved under
this campaign. The coverage-plateau behavior itself has a straightforward explanation, not a bug:
these are small, `Arbitrary`-driven harnesses over a bounded byte-layout space (even with 53
`ResamplingFunction` variants reachable via `sampler_variant: u32 % 53`-style mapping, the harness's
own `src_width`/`src_height` guards keep images small, and libFuzzer's mutation engine reliably
explores a bounded structural space within a few hundred thousand to a few million executions for
most of these targets, which candidates `014` and `018` already independently observed). `colorspaces`
plateaus later only because it executes ~10-70x fewer inputs/second than the other targets (189
exec/s vs 2,029-13,749 exec/s) - same total structural space, just reached more slowly in wall-clock
time, not a larger reachable space.

## The fix

No source fix applicable - no bug found this session, so there is no `patch.diff` against `src/`.

The harness diversification (re-applied from candidate `018`'s saved `harness_diversification_reference.diff`,
unmodified) was reverted (`git checkout -- fuzz/`) per this skill's "always leave the tree clean"
rule, same as candidates `015`/`018`'s identical decision.

- Project test suite: not run - no source change was made in this candidate (nothing to verify a
  fix against). The reverted harness-only change also does not touch any code under `src/` that the
  test suite exercises.

## What's not done / open questions

- Only 8 of the 16 diversified targets were coverage-snapshotted this session (one representative
  per distinct `SrcImage`/tuple field layout family, plus `colorspaces` as the known outlier); the
  other 8 were not individually checked, though by code reading and by candidate `018`'s own
  4-target corpus-diversity check they all share the same harness shape and mutation dynamics, so
  there's no structural reason to expect a different plateau curve.
- Still no ASan (memory-safety) coverage in this sandbox for this run either (same limitation as
  candidates `001`/`014`/`015`/`018`) - a real out-of-bounds/UAF bug reachable only via a
  non-Bilinear/Lanczos3 kernel size would not necessarily be caught by debug-assertions + coverage-
  guided fuzzing alone.
- Per candidate `018`'s own already-open backlog item, a **persistent/reused fuzz corpus** across
  sessions (rather than this candidate's own deliberately-cold-start empty corpus, chosen here
  specifically to measure the growth curve honestly) remains the highest-leverage way to make
  future runs explore genuinely new ground instead of re-discovering the same plateau each time -
  this candidate's finding (plateau within ~900-1500s from empty) reinforces why a persistent
  corpus would help more than simply running the same empty-start harnesses longer.
- NEON/SVE2/WASM32 code paths remain untested (x86_64 host), unchanged from `README.md`'s existing
  caveat.
