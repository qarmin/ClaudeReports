# Fuzz-harness `ResamplingFunction` coverage

Candidate `001`'s tooling notes flagged, unverified, that only `ResamplingFunction::Bilinear` was
independently confirmed exercised by the 16 existing cargo-fuzz targets, matching what
`resize_rgba`'s harness hardcodes; candidate `014`'s longer sustained run (91M execs, 0 crashes)
didn't re-check this either and explicitly logged it as a plausible reason its own coverage
plateaued early. This candidate checks whether that's actually true, and whether closing the gap
surfaces anything the narrower harnesses couldn't reach.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static read of all 16 `fuzz/<name>/<name>.rs` harness sources (`grep -rn "ResamplingFunction" fuzz/`) | reading source, no build | Confirmed the premise exactly: every one of the 16 `[[bin]]` targets passes a single hardcoded `ResamplingFunction` variant to `Scaler::new(...)` - 11 hardcode `Bilinear` (`resize_rgba`, `resize_rgb`, `resize_rgb_f32`, `resize_rgba_f32`, `resize_cbcr8`, `resize_cbcr_f32`, `resize_cbcr_f16`, `resize_plane`, `resize_plane_f32`, plus 4 in `colorspaces.rs`), 5 hardcode `Lanczos3` (`resize_cbcr16`, `resize_rgba_u16`, `resize_rgb_u16`, `resize_rgb_f16`, `resize_rgba_f16`). None derive the sampler from the `Arbitrary`-generated `SrcImage` struct at all - the field doesn't exist in any of the 16 `SrcImage` structs. Of the 53 `ResamplingFunction` variants (`src/sampler.rs`), 51 (everything except `Bilinear`/`Lanczos3`) are structurally unreachable by any of the 16 existing targets, no matter how long they run - this fully explains candidate `014`'s observed early coverage plateau. |
| One-off local harness patch: added `pub sampler_variant: u32` to `SrcImage` and replaced the hardcoded `ResamplingFunction::Bilinear`/`::Lanczos3` with `ResamplingFunction::from(data.sampler_variant)` (the crate's own existing `impl From<u32> for ResamplingFunction`, `src/sampler.rs`, maps 0-52 to each variant 1:1 and anything out-of-range to `Bilinear` - no new mapping logic needed) in 3 of the 16 targets spanning different channel counts/bit depths: `resize_rgba` (u8, 4-channel, was `Bilinear`-only), `resize_plane` (u8, 1-channel, was `Bilinear`-only), `resize_rgb_u16` (u16, 3-channel, was `Lanczos3`-only). Built with `cargo +nightly fuzz build --sanitizer none --features avx,sse,nightly_avx512 <target>` (release, debug-assertions on, no ASan - see `README.md` for why plain ASan hangs in this sandbox) then ran `cargo +nightly fuzz run --sanitizer none --features avx,sse,nightly_avx512 <target> -- -max_total_time=<N> -rss_limit_mb=4096` from an empty per-target corpus, coverage-guided, no fixed seed. | `resize_rgba`: 300s, 365,102 execs. `resize_plane`: 180s, 284,593 execs. `resize_rgb_u16`: 180s, 58,925 execs. **708,620 total executions, zero crashes, panics, overflow aborts, timeouts, or OOM kills.** `fuzz/artifacts/{resize_rgba,resize_plane,resize_rgb_u16}/` empty after each run (checked directly, see `output.txt`). |
| Independent verification that the diversified harness *actually* reached most/all `ResamplingFunction` variants (not just carried the field without exploring it): wrote a standalone scratch binary (`/tmp/corpus_check{,2}`, `arbitrary` crate, not part of this repo) that re-parses every file libFuzzer kept in each target's `corpus/<name>/` directory (a file is only kept there if it triggered new code coverage) with the exact same `Arbitrary`-derived `SrcImage` struct, decodes `sampler_variant`, and reports the distinct set of resulting values (0-52, out-of-range folds to `Bilinear`=0, matching `ResamplingFunction::from`'s own fallback). | `resize_rgba` corpus (1536 files): **53/53** distinct `sampler_variant` values present - every `ResamplingFunction` variant genuinely triggered new coverage at least once. `resize_plane` corpus (1460 files, identical `SrcImage` field layout to `resize_rgba`): **53/53**. `resize_rgb_u16` corpus (930 files, its own distinct `SrcImage` layout - `value: u16`, no `premultiply_alpha`/`supersampling`/`multi_stage_upsampling` fields): **49/53** (missing 4 values in its shorter 180s/58,925-exec run) - see `output.txt` for the exact printed sets. This directly confirms the diversified harnesses did what they were meant to: reach code paths (`src/math/*.rs` kernel formulas, `generate_weights`'s per-window-size sizing) that the original Bilinear/Lanczos3-only harnesses structurally could not. |

This is one escalation (a short one-off fuzz harness, per `HEURISTICS.md`'s guidance for "crash on
adversarial input with a large input space and no reference oracle") past the static read that
confirmed the premise - no signal appeared, so per `SKILL.md`'s "3 clean negatives means move on"
a second/third distinct tool was not pursued this round (there's no crash/UB signal to chase
further with `miri`/ASAN; the coverage-gap fact itself was already fully confirmed by the static
read alone, with no further tool needed to establish that half of the finding).

## Status

**Not reproducible** (no pic-scale source bug found). The premise itself - that only `Bilinear`
and `Lanczos3` are reachable by the existing 16 harnesses - **is confirmed true** by the static
read (this is not itself a "bug" in pic-scale, it's a fact about the fuzz harnesses' own coverage).
Closing that gap for 3 of the 16 targets and fuzzing with full `ResamplingFunction` diversity for
708,620 executions total found no new crash, panic, or overflow that the narrower Bilinear/Lanczos3-only
harnesses were missing.

**Fully reproducible**: N/A (Not reproducible - no pic-scale bug to reproduce; the coverage-gap
fact itself is trivially and fully reproducible by reading the 16 harness source files, see
`justfile`).

## Class

N/A (Not reproducible - no pic-scale bug confirmed).

## Root cause

Not applicable to a "bug" in pic-scale's own code - nothing in the resize/kernel-selection code
itself was observed to misbehave. The *mechanism* behind the original suspicion (why coverage
plateaued early in candidates `001`/`014`) is confirmed: every one of the 16 fuzz harnesses passes
a single Rust-source-level hardcoded `ResamplingFunction` value to `Scaler::new(...)` instead of
deriving it from the `Arbitrary` input, so `generate_weights` (`src/math/weights.rs`) and every
kernel-formula function in `src/math/*.rs` other than the two hardcoded variants' own formulas
(bilinear's implicit linear interpolation, Lanczos3's windowed-sinc) have never been exercised by
any of this project's existing fuzzing, by construction - not because those code paths are hard to
reach, but because nothing in the harness ever asks for them.

## The fix

No source fix applicable - no bug found this session, so there is no `patch.diff` against
`src/`. The one-off harness patches used to test this candidate (adding `sampler_variant: u32` and
using `ResamplingFunction::from(data.sampler_variant)`) were applied only locally to explore the
premise, then reverted (`git checkout --`) per this skill's "always leave the main project's
working tree clean" rule - see "What's not done" below for why they weren't kept as a permanent
harness improvement in this session.

- Project test suite: not run - no source change was made in this candidate (nothing to verify a
  fix against).

## What's not done / open questions

- Only 3 of the 16 harnesses were patched and fuzzed this session (`resize_rgba`, `resize_plane`,
  `resize_rgb_u16`); the other 13 (`resize_rgb`, `resize_cbcr8`, `resize_cbcr16`, `resize_cbcr_f16`,
  `resize_cbcr_f32`, `resize_plane_f32`, `resize_plane_u16`, `resize_rgba_f16`, `resize_rgba_f32`,
  `resize_rgba_u16`, `resize_rgb_f16`, `resize_rgb_f32`, `colorspaces`) still hardcode a single
  `ResamplingFunction` and were not patched/re-fuzzed - the 3 chosen cover 3 different
  channel-count/bit-depth combinations (u8 4-channel, u8 1-channel, u16 3-channel) as a
  representative sample, not an exhaustive one.
- Each patched target only got 180-300s (708,620 execs total) - much shorter than candidate `014`'s
  900s/target sustained campaign; a longer run (or one seeded with a corpus that specifically
  targets `kernel_size`-heavy variants like `Lanczos6`/`EwaLanczos4Sharpest`, which produce much
  larger convolution windows than `Bilinear`/`Lanczos3` and so exercise different size/index math
  in `src/math/weights.rs`) is the natural next escalation if this candidate is revisited.
- The underlying coverage gap itself was **not fixed** in the actual `fuzz/` harnesses (deliberately
  reverted, per the "always leave the tree clean" rule) - a maintainer applying the same
  `sampler_variant: u32` + `ResamplingFunction::from(...)` pattern to all 16 targets permanently
  (not just as a one-off local test) would be a real, low-effort, low-risk fuzzing-infrastructure
  improvement; logged as a new `IDEAS.md` backlog item for a future session (or the project
  maintainer directly) rather than applied here, since no bug was found to justify a source-tree
  change in this session and this skill's role is to confirm bugs, not to permanently modify the
  test harnesses themselves without a maintainer's own choice to keep that change.
- Still no ASan (memory-safety) coverage in this sandbox for these newly-diversified runs either
  (same limitation as candidates `001`/`014`) - a real out-of-bounds/UAF bug reachable only via a
  non-Bilinear/Lanczos3 kernel size would not necessarily be caught by debug-assertions +
  coverage-guided fuzzing alone.
- NEON/SVE2/WASM32 code paths remain untested (x86_64 host), unchanged from `README.md`'s existing
  caveat.
