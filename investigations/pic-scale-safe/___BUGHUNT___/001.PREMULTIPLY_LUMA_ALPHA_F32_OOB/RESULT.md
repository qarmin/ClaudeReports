# premultiply_luma_alpha_f32 / premultiplied_luma_alpha_f32 out-of-bounds index

`src/alpha.rs:464-472` (`premultiply_luma_alpha_f32`) and `src/alpha.rs:483-493`
(`premultiplied_luma_alpha_f32`) operate on 2-element (luma, alpha) chunks
(`chunks_exact_mut(2)` / `chunks_exact(2)`) but index element `2`, which does
not exist in a 2-element window. This was flagged by a prior
`code-audit-structured` pass as looking copy-pasted from the 4-channel RGBA
sibling functions (`premultiply_rgba_f32`/`premultiplied_rgba_f32`), which
legitimately index `chunk[3]`/`src[3]` on `chunks_exact(4)`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + `cargo test` in-tree (debug build) | Added a throwaway `tests/bughunt_oob_check.rs` calling both functions with a minimal 2-element `f32` slice (`[1.0, 0.5]`) | Both calls panicked: `index out of bounds: the len is 2 but the index is 2` at `src/alpha.rs:470:9` and `src/alpha.rs:488:17` |
| Same repro, `RUST_BACKTRACE=1`, repeated 2x | debug build | Deterministic, identical panic location both times |
| Standalone `repro/` against a fresh clone at the base commit, `cargo run --release` | release build (Rust's slice indexing bounds checks are not gated by `debug-assertions`, so this also confirms it's not a debug-only artifact) | Reproduced 5/5 runs, identical panic sites both times per run |

No sanitizer/miri needed - this is safe-Rust checked-indexing panic, not memory-unsafety; the
language's own bounds check is the "tool" that catches it, and it fires unconditionally on any
non-empty input in both debug and release.

## Status

**Confirmed** - reproduces deterministically, 5/5 runs in release build against a fresh clone
(`repro/`), plus 2/2 runs in-tree under debug. This is not layout- or seed-dependent: any call with
a non-empty slice hits the exact same out-of-bounds index every time.

## Class

crash

## Root cause

Both functions were written by copy-pasting the 4-channel RGBA premultiply functions
(`premultiply_rgba_f32` / `premultiplied_rgba_f32`, which correctly use `chunks_exact(4)` and index
up to `chunk[3]`) without adjusting the indices for the 2-channel luma+alpha layout. The correct
luma+alpha layout is `[luma, alpha]` (indices 0 and 1 only) - confirmed by the sibling
`unpremultiply_luma_alpha_f32` a few lines below (`src/alpha.rs:547-556`), which already uses the
correct pattern: reads `chunk[1]` as alpha and never touches an index beyond 1. The buggy functions
instead read `chunk[1]`/`src[1]` correctly as alpha but then write to (or read from) index `2`,
which is out of bounds for a 2-element chunk and panics via Rust's checked slice indexing.

Both functions are reachable from any downstream user of the crate: `src/lib.rs` re-exports the
whole `alpha` module (`pub use alpha::*`), and neither function is called anywhere internally in
this crate (grepped for both names across all `.rs` files) - so the bug is otherwise silent (no
internal caller trips over it) until a library consumer calls either public function directly.

## Introduced by

5ee1e8c "Refactors" (2024-11-20) - this commit added `premultiply_luma_alpha_f32` (and its `premultiplied_` sibling) from scratch, already with the copy-pasted RGBA indexing (`chunk[2]`) instead of the correct 2-channel indexing.

**Already fixed upstream** (independently of this candidate's own reverted patch): commit aa0ae81 "Fix off-by-one indexing in premultiply" (2026-08-25, merged into current HEAD via PR #20) changed `chunk[2]` to `chunk[1]`, citing "As spotted by Claude in issue #18" - the current working tree (checked while filling in this section) no longer reproduces this bug.

## The fix

Change the out-of-bounds index to match the correct 2-element luma+alpha layout, mirroring the
already-correct `unpremultiply_luma_alpha_f32`:

- `premultiply_luma_alpha_f32`: `chunk[2] = a;` -> `chunk[1] = a;`
- `premultiplied_luma_alpha_f32`: `let a = src[2];` -> `let a = src[1];`

This is sufficient, not just a fix for the one literal input that was tried: re-verified with a
second input (`[2.0, 0.5]`) after the fix, producing the mathematically correct premultiplied
result (`[1.0, 0.5]`, i.e. `luma * alpha = 2.0 * 0.5`, alpha unchanged) rather than merely not
panicking. See `patch.diff` and `repro/`'s before/after run in `output.txt`.

- Project test suite: before fix - `cargo test --workspace`: `pic-scale-safe` lib itself has 0
  tests (N/A, no unit/integration tests in this package); the workspace's `app` binary crate has 1
  test (`image_wrapper::tests::test_rescaling`), which passed both before and after the fix (1
  passed, 0 failed, unchanged). No test in the existing suite exercises either buggy function
  directly (which is why this had gone unnoticed), so the before/after counts are identical - the
  regression-guarding value here is entirely from the new `repro/`, not a change in the existing
  suite's pass/fail counts.

## What's not done / open questions

- No test was added to the main project's own test suite (per this skill's workflow, the working
  tree is always reverted; adding a permanent regression test is a decision for whoever applies
  `patch.diff` for real).
- Other `chunks_exact`-based functions in `src/alpha.rs` were spot-checked by reading (the
  `u16`/`u8` LA and RGBA variants, `unpremultiply_luma_alpha_f32`) and all use correct indices for
  their channel count - this specific copy-paste mistake does not appear to recur elsewhere in this
  file. `has_non_constant_alpha`/`alpha_check.rs` (a separate suspect already in `IDEAS.md`) was not
  checked as part of this candidate.
- No fuzzing campaign was run - not needed here; the panic is unconditional on any non-empty input,
  so exhaustive fuzzing would add no information beyond the single boundary-value repro already
  captured.
