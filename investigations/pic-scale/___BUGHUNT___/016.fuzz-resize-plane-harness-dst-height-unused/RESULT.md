# fuzz/resize_plane harness never fuzzes rectangular destination sizes (dst_height unused)

`fuzz/resize_plane/resize_plane.rs`'s `fuzz_target!` closure calls its `resize_plane` helper with
`data.dst_width as usize` for BOTH the `dst_width` and `dst_height` positional arguments. The
`Arbitrary`-derived `SrcImage` struct does generate a `dst_height: u16` field, so libFuzzer's
mutator genuinely explores different byte values for it every run - but that decoded value is
never read anywhere in the harness, so every execution can only ever resize to a SQUARE
destination (`dst_width == dst_height`). This was noticed while reading this harness for candidate
`015` and logged in `IDEAS.md`'s backlog rather than investigated there.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static read of `fuzz/resize_plane/resize_plane.rs` (the exact call site inside `fuzz_target!`) | reading source, no build | Confirmed with certainty: lines 56-57 are the literal token sequence `data.dst_width as usize,` twice in a row, as the 3rd and 4th positional arguments to `resize_plane`. This is a deterministic, non-runtime-dependent fact - true on every single execution regardless of what bytes libFuzzer feeds it, not something that needs a repeat-run count to establish. |
| One-line local fix (`data.dst_width as usize` -> `data.dst_height as usize` on the 4th argument only) + `cargo +nightly fuzz build --sanitizer none --features avx,sse,nightly_avx512 resize_plane`, then `cargo +nightly fuzz run` for `-max_total_time=300` from an empty corpus, coverage-guided, no fixed seed (same environment/workaround as candidates `001`/`014`/`015` - see `README.md`) | 300s wall time | 1,119,017 executions, **0 crashes, panics, overflow-aborts, timeouts, or OOM kills**. `fuzz/artifacts/resize_plane/` empty afterward. This burst specifically targets the newly-unlocked input dimension (rectangular destinations, including extreme aspect ratios like `1x830 -> 1x257`) in exactly the size/index math (`src/math/weights.rs` and friends) candidate `010`'s prior confirmed bug lived in - found nothing new there this session. |
| Corpus check: standalone scratch binary (`/tmp/corpus_check_plane`, `arbitrary` crate, not part of this repo) re-decoded every one of the 403 files libFuzzer kept in `fuzz/corpus/resize_plane/` (kept only if it triggered new coverage) with the identical `Arbitrary`-derived `SrcImage` struct, and counted how many decode to a valid, genuinely rectangular destination | reading the real post-fix corpus this session's fuzzing produced | `total parsed: 403, dst_width != dst_height: 368, valid+rectangular: 367` - 91% of the corpus libFuzzer actually kept is genuinely rectangular, confirming the one-line fix really does unlock new coverage the unpatched harness could structurally never reach. |

This is two escalations past the static read that already fully established the harness defect
itself with certainty (a short one-off fuzz harness burst, per `HEURISTICS.md`'s guidance for
"crash on adversarial input with a large input space and no reference oracle", plus a corpus
sanity check) - no crash/UB signal appeared in either, so per `SKILL.md`'s "3 clean negatives means
move on" a third tool was not pursued (there is no live thread here to chase further - the harness
defect itself needed no further tool to confirm, and the fuzzing burst it unlocked found nothing).

## Status

**Confirmed** (the harness defect itself - a real, deterministic argument-passing mistake in
`fuzz/resize_plane/resize_plane.rs`). This is bit-for-bit repeatable on demand: the source text
itself is the proof, and `repro/` demonstrates it via the real public API (see below), re-run 3/3
times with identical output.

No pic-scale **library** bug was found by exercising the newly-unlocked rectangular-destination
input space (1,119,017 executions, 300s, 0 crashes) - that part of the investigation is `Not
reproducible`, stated here rather than as a separate row since it's directly downstream of fixing
this same harness.

**Fully reproducible**: **Yes**. This is a single-artifact finding (fuzzer-chosen dimension bytes
processed by a harness function), not one participant reacting to another's input across a
process/protocol boundary - directly analogous to CONFIRMATION.md section 11's "malformed file
handed to a parser's real entry point" carve-out, so a single-process repro is the correct shape.
`repro/` decodes a real corpus file libFuzzer itself kept during this session's fuzzing run
(`fuzzer_found_input.bin`, not hand-authored) via the real `arbitrary` derive on the exact
`SrcImage` struct, then calls the real pic-scale public API
(`Scaler::plan_planar_resampling`/`resample`) through both the exact buggy and exact fixed
call-site argument shapes, and prints the real `target.size()` each one actually reaches.

## Class

logic (in fuzz-harness/test-infrastructure code, not the pic-scale library itself)

## Root cause

`fuzz/resize_plane/resize_plane.rs`, inside `fuzz_target!(|data: SrcImage| { ... })`:

```rust
resize_plane(
    data.src_width as usize,
    data.src_height as usize,
    data.dst_width as usize,
    data.dst_width as usize,   // <- should be data.dst_height
    ResamplingFunction::Bilinear,
    ...
);
```

The 3rd and 4th positional arguments of `resize_plane` are `dst_width: usize` and
`dst_height: usize` respectively. Both call sites read `data.dst_width`, so `data.dst_height`
(present in the struct, genuinely decoded from fuzzer-controlled bytes) is dead: it can never
influence which `resize_plane` call is actually made. Every execution of this fuzz target, no
matter what bytes libFuzzer generates, resizes to a square destination
(`dst_width == dst_height`) - never a rectangular one. `src/math/weights.rs`'s
per-window-size/index math (the same file candidate `010`'s confirmed overflow bug lived in) and
any other size-dependent code that could behave differently for non-square dimensions has never
been exercised via this specific harness for its whole history, by construction.

## Introduced by

d34179a5 "Adding i8mm NEON" (2025-05-14) - `git show` confirms this commit is what converted the harness from a plain `(u16,u16,u16,u16)` tuple (whose 4th field, `data.3`, was correctly read for `dst_height`) to the `Arbitrary`-derived `SrcImage` struct, and in doing so introduced the copy-paste mistake (`data.dst_width` used twice instead of `data.dst_width`/`data.dst_height`). The harness existed earlier (from d335ebb8 "Adding fuzzing, rgba8", 2024-12-09) but read the correct field before this commit's rewrite.

## The fix

Change the 4th argument from `data.dst_width as usize` to `data.dst_height as usize` (one line).
Verified: `cargo +nightly fuzz build` succeeds against the patched harness, and the harness now
genuinely explores rectangular destinations - the post-fix 300s/1,119,017-exec run's kept corpus
is 91% genuinely rectangular (367/403 files, see "What was tried" above), versus 0% structurally
possible before the fix (provable directly from the pre-fix source text, not just observed).
`repro/`'s `call_as_written_after_fix` (the exact fixed call-site shape) reaches the real
fuzzer-requested `dst_height` (`1x257`) instead of collapsing to `1x1`, re-run 3/3 times with
identical output; `call_as_written_before_fix` (the exact pre-fix call-site shape) collapses to
`1x1` every time, matching the root cause exactly.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after
  (`cargo test --lib --release`, main crate). This fix only touches
  `fuzz/resize_plane/resize_plane.rs`, which is excluded from the main workspace
  (`Cargo.toml`'s `exclude = [..., "fuzz", ...]`) and has no test suite of its own - the main
  crate's test suite cannot be affected by it either way; run anyway for completeness per this
  skill's mandatory verification step.

## What's not done / open questions

- Only `resize_plane` was checked and fixed this session. The same "harness's fuzz_target! closure
  hardcodes/duplicates one argument instead of reading the corresponding `Arbitrary`-derived
  field" bug class was not independently re-checked across the other 15 existing `fuzz/<name>/
  <name>.rs` targets (`resize_rgba`, `resize_rgb`, `resize_cbcr8`, `resize_cbcr16`,
  `resize_cbcr_f16`, `resize_cbcr_f32`, `resize_plane_u16`, `resize_plane_f32`, `resize_rgba_u16`,
  `resize_rgba_f16`, `resize_rgba_f32`, `resize_rgb_u16`, `resize_rgb_f16`, `resize_rgb_f32`,
  `colorspaces`) - each has its own `SrcImage`-equivalent struct and its own call site, and this
  specific one-line copy-paste mistake could independently exist in any of them; a grep-based
  static sweep (`grep -B1 -A1 "dst_width as usize" fuzz/*/*.rs`) across all 16 would be a cheap
  follow-up before assuming this class is now closed everywhere.
- The 300s/1,119,017-exec burst against the newly-unlocked rectangular-destination input space
  found nothing, but that's a short one-off burst, not a sustained campaign (candidate `014`'s
  90.9M-exec/16-target sustained run is the relevant comparison) - a longer run, especially with
  the `ResamplingFunction` diversification candidate `015` prototyped (large-kernel variants like
  `Lanczos6`/`EwaLanczos4Sharpest` combined with extreme rectangular aspect ratios would exercise
  `src/math/weights.rs` size math more aggressively than `Bilinear`-only), is the natural next
  escalation.
- The fix was applied, verified, and then reverted from the main project's working tree per this
  skill's "always leave the tree clean" rule (see `git status` note below) - a maintainer would
  need to actually apply `patch.diff` for real to permanently close this gap in the project's own
  fuzzing.
