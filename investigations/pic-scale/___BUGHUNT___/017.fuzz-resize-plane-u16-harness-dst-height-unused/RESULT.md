# fuzz/resize_plane_u16 harness never fuzzes rectangular destination sizes (dst_height unused)

`fuzz/resize_plane_u16/resize_plane_u16.rs`'s `fuzz_target!` closure calls its `resize_rgb`
helper with `data.dst_width as usize` for BOTH the `dst_width` and `dst_height` positional
arguments. The `Arbitrary`-derived `SrcImage` struct does generate a `dst_height: u16` field, so
libFuzzer's mutator genuinely explores different byte values for it every run - but that decoded
value is never read anywhere in the harness, so every execution can only ever resize to a SQUARE
destination (`dst_width == dst_height`). This is byte-for-byte the same copy-paste mistake
candidate `016` found and fixed in `fuzz/resize_plane/resize_plane.rs`, independently present in
this sibling harness - noticed and logged in `IDEAS.md`'s backlog while closing out `016`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static read of `fuzz/resize_plane_u16/resize_plane_u16.rs` (the exact call site inside `fuzz_target!`) | reading source, no build | Confirmed with certainty: lines 54-55 are the literal token sequence `data.dst_width as usize,` twice in a row, as the 3rd and 4th positional arguments to `resize_rgb`. Deterministic, non-runtime-dependent - true on every execution regardless of what bytes libFuzzer feeds it. |
| One-line local fix (`data.dst_width as usize` -> `data.dst_height as usize` on the 4th argument only) + `cargo +nightly fuzz build --sanitizer none --features avx,sse,nightly_avx512 resize_plane_u16`, then `cargo +nightly fuzz run` for `-max_total_time=300` from an empty corpus, coverage-guided, no fixed seed (same environment/workaround as candidates `001`/`014`/`015`/`016` - see `README.md`) | 300s wall time | 241,522 executions, **0 crashes, panics, overflow-aborts, timeouts, or OOM kills**. `fuzz/artifacts/resize_plane_u16/` empty afterward. This burst specifically targets the newly-unlocked input dimension (rectangular destinations) - found nothing new there this session. |
| Corpus check: standalone scratch binary (`/tmp/corpus_check_plane_u16`, `arbitrary` crate, not part of this repo) re-decoded every one of the 313 files libFuzzer kept in `fuzz/corpus/resize_plane_u16/` (kept only if it triggered new coverage) with the identical `Arbitrary`-derived `SrcImage` struct, and counted how many decode to a valid, genuinely rectangular destination | reading the real post-fix corpus this session's fuzzing produced | `total parsed: 313, dst_width != dst_height: 288, valid+rectangular: 288` - 92% of the corpus libFuzzer actually kept is genuinely rectangular, confirming the one-line fix really does unlock new coverage the unpatched harness could structurally never reach. |

This is two escalations past the static read that already fully established the harness defect
itself with certainty (a short one-off fuzz harness burst, per `HEURISTICS.md`'s guidance for
"crash on adversarial input with a large input space and no reference oracle", plus a corpus
sanity check) - no crash/UB signal appeared in either, so per `SKILL.md`'s "3 clean negatives means
move on" a third tool was not pursued.

## Status

**Confirmed** (the harness defect itself - a real, deterministic argument-passing mistake in
`fuzz/resize_plane_u16/resize_plane_u16.rs`). This is bit-for-bit repeatable on demand: the source
text itself is the proof, and `repro/` demonstrates it via the real public API (see below), re-run
3/3 times with identical output.

No pic-scale **library** bug was found by exercising the newly-unlocked rectangular-destination
input space (241,522 executions, 300s, 0 crashes) - that part of the investigation is `Not
reproducible`, stated here rather than as a separate row since it's directly downstream of fixing
this same harness.

**Fully reproducible**: **Yes**. This is a single-artifact finding (fuzzer-chosen dimension bytes
processed by a harness function), not one participant reacting to another's input across a
process/protocol boundary - directly analogous to CONFIRMATION.md section 11's "malformed file
handed to a parser's real entry point" carve-out, so a single-process repro is the correct shape.
`repro/` decodes a real corpus file libFuzzer itself kept during this session's fuzzing run
(`fuzzer_found_input.bin`, not hand-authored) via the real `arbitrary` derive on the exact
`SrcImage` struct, then calls the real pic-scale public API
(`Scaler::plan_planar_resampling16`/`resample`) through both the exact buggy and exact fixed
call-site argument shapes, and prints the real `target.size()` each one actually reaches.

## Class

logic (in fuzz-harness/test-infrastructure code, not the pic-scale library itself)

## Root cause

`fuzz/resize_plane_u16/resize_plane_u16.rs`, inside `fuzz_target!(|data: SrcImage| { ... })`:

```rust
resize_rgb(
    data.src_width as usize,
    data.src_height as usize,
    data.dst_width as usize,
    data.dst_width as usize,   // <- should be data.dst_height
    ResamplingFunction::Lanczos3,
    ...
);
```

The 3rd and 4th positional arguments of `resize_rgb` are `dst_width: usize` and
`dst_height: usize` respectively. Both call sites read `data.dst_width`, so `data.dst_height`
(present in the struct, genuinely decoded from fuzzer-controlled bytes) is dead: it can never
influence which `resize_rgb` call is actually made. Every execution of this fuzz target, no
matter what bytes libFuzzer generates, resizes to a square destination
(`dst_width == dst_height`) - never a rectangular one. Any size-dependent code that could behave
differently for non-square dimensions (e.g. `src/math/weights.rs`'s per-window-size/index math,
the same file candidate `010`'s confirmed overflow bug lived in) has never been exercised via this
specific harness for its whole history, by construction - and specifically not for the `u16`/`i16`
16-bit-plane code paths this harness alone covers (10-bit and 16-bit depth variants of both signed
and unsigned planar resampling).

## Introduced by

d34179a5 "Adding i8mm NEON" (2025-05-14) - same commit and same mechanism as sibling candidate 016 (`git log -S"dst_width as usize"` on this file also lands on this commit): it rewrote the harness from a tuple to the `Arbitrary`-derived `SrcImage` struct and introduced the identical copy-paste mistake in the process.

## The fix

Change the 4th argument (2nd occurrence of `data.dst_width as usize`, at line 55) to
`data.dst_height as usize` - one line, identical shape to candidate `016`'s fix in the sibling
`resize_plane` harness. Verified: `cargo +nightly fuzz build` succeeds against the patched
harness, and the harness now genuinely explores rectangular destinations - the post-fix
300s/241,522-exec run's kept corpus is 92% genuinely rectangular (288/313 files, see "What was
tried" above), versus 0% structurally possible before the fix (provable directly from the pre-fix
source text, not just observed). `repro/`'s `call_as_written_after_fix` (the exact fixed
call-site shape) reaches the real fuzzer-requested `dst_height` (`1x257`) instead of collapsing to
`1x1`, re-run 3/3 times with identical output; `call_as_written_before_fix` (the exact pre-fix
call-site shape) collapses to `1x1` every time, matching the root cause exactly.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after
  (`cargo test --lib --release`, main crate). This fix only touches
  `fuzz/resize_plane_u16/resize_plane_u16.rs`, which is excluded from the main workspace
  (`Cargo.toml`'s `exclude = [..., "fuzz", ...]`) and has no test suite of its own - the main
  crate's test suite cannot be affected by it either way; run anyway for completeness per this
  skill's mandatory verification step.

## What's not done / open questions

- Per candidate `016`'s own note, all 14 *other* fuzz targets besides `resize_plane` and
  `resize_plane_u16` were already confirmed correct by a grep-based static sweep
  (`grep -B1 -A1 "dst_width as usize\|dst_height as usize" fuzz/*/*.rs`) run while closing out
  `016` - `resize_plane_u16` (this candidate) was the one remaining sibling instance identified by
  that sweep, and is now fixed-and-verified here. No further instances of this exact copy-paste
  class are believed to remain in `fuzz/`, but that belief rests on the earlier grep sweep, not an
  independent re-check this session.
- The 300s/241,522-exec burst against the newly-unlocked rectangular-destination input space found
  nothing, but that's a short one-off burst, not a sustained campaign (candidate `014`'s
  90.9M-exec/16-target sustained run is the relevant comparison) - a longer run is the natural next
  escalation, same open item as candidate `016` left for its own sibling harness.
- The fix was applied, verified, and then reverted from the main project's working tree per this
  skill's "always leave the tree clean" rule - a maintainer would need to actually apply
  `patch.diff` for real to permanently close this gap in the project's own fuzzing.
