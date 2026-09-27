# filter_1d.rs arena-path OOB slice index on zero-height images

Investigated from an `IDEAS.md` backlog item claiming `src/imageops/filter_1d.rs:83`
(`make_arena_row`'s `source_row.first().unwrap()`/`.last().unwrap()`) panics on zero-*width*
images via `imageops::unsharpen()`. That literal claim turned out to be currently unreachable
through any public entry point (see "Root cause" below for why), but the same file's Gaussian-blur
arena machinery has a distinct, real, reproducible panic for zero-*height* images instead, driven
through the exact same named entry point (`unsharpen()`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (debug build, `cargo run --example`), zero-width case from the candidate description | `image::imageops::unsharpen(&GrayImage::new(0, 5), 1.0, 0)` | Panicked, but **not** in `filter_1d.rs` - panicked at `src/images/buffer.rs:1302` (`Image index (0, 1) out of bounds (0, 5)`), inside the default `GenericImageView::Pixels::next()` iterator used by `gaussian_blur_indirect_impl` to build its working buffer, *before* any `filter_1d.rs` code runs. This is the exact bug already investigated and Confirmed as candidate `001.PIXELS_ITERATOR_ZERO_WIDTH_OOB` (patch recorded there, reverted per protocol - not present on the tree tested here) - it preempts the `make_arena_row` code path entirely for `width == 0, height > 0`. |
| Manual boundary-value construction, zero-*height* case (`width > 0, height == 0`), varying sigma | `unsharpen(&GrayImage::new(5, 0), sigma, 0)` for a sweep of `sigma` values (debug build) | For `sigma` large enough that the internally-derived Gaussian kernel length reaches `RING_QUEUE_CIRCULAR_CUTOFF` (55) - e.g. `sigma = 30.0` - **panics** at `src/imageops/filter_1d.rs:119:35`: `range end index 5 out of range for slice of length 0`, inside `make_columns_arenas`. Reproduces 3/3 runs, deterministic (no randomness/layout dependence - a plain length-arithmetic bug). Also reproduced via `blur()` directly and via the anisotropic kernel-size constructor without needing an unusually large sigma. |
| Standalone `repro/` (release profile) against the crate's own public API (`image::imageops::unsharpen`), via `just test` end-to-end (real `git clone` of `repo_url` @ `base_commit`, real cargo build) | `cargo run --release` in `repro/`, `just test` from a fresh clone | Same panic before the patch (3/3 runs, see `output.txt`); after `patch.diff` is applied, `unsharpen` returns a `(5, 0)` image without panicking (2/2 runs). |
| Boundary sweep across neighboring inputs against the patched clone | `(5,0,σ=30)`, `(17,0,σ=45)`, `(5,0,σ=1,` below the arena cutoff`)`, `(0,0,σ=30)`, `blur()` on a 9x0 RGB image with radius 40, and a normal non-degenerate `(4,3,σ=30)` case, via a throwaway `examples/` binary in the patched clone (deleted before end of session, output captured in `output.txt`) | All six produce correct output after the fix; the normal `(4,3)` case is unaffected. |

Only one approach round was needed (soft cap of 3 not exceeded) - the bug reproduces immediately
and deterministically via direct boundary-value construction, no fuzzing/miri/sanitizer needed for
this class of bug (plain length/index arithmetic that doesn't account for a zero-length dimension,
not `unsafe`/UB - the crate is `#![forbid(unsafe_code)]` in this module).

## Status

**Confirmed.** Deterministic, reproduces 3/3 runs before the fix (both directly in this checkout
and via the standalone `repro/` against a fresh clone) and consistently stops reproducing after
the fix (2/2 runs), driven purely through the public `image::imageops::unsharpen`/`blur` free
functions. No randomness/timing/memory-layout dependence involved.

**Fully reproducible: Yes.** The repro calls the real public entry point named in the original
candidate (`image::imageops::unsharpen`) on a real `ImageBuffer` built through the crate's own
public constructor, exactly as a genuine caller would (e.g. an empty crop, or simply a
caller-supplied zero-height buffer, blurred with a large radius). There is no second
participant/peer/wire input involved in this finding - it's a pure logic bug in internal
size/index arithmetic triggered by a caller-supplied dimension - so `CONFIRMATION.md` section 11's
"real second participant" requirement does not apply here.

## Class

crash

## Root cause

Two related facts, both needed to understand what's actually reachable today:

**1. The originally-suspected zero-*width* path (`make_arena_row`, line 83) is not currently
reachable from any public entry point.** Every caller of `filter_2d_separable`/`make_arena_row` in
this crate goes through `gaussian_blur_indirect_impl` (`src/imageops/sample.rs`), which first
builds its `f32` working buffer via `image.pixels()` - the default `GenericImageView::Pixels`
iterator. For `width == 0, height > 0`, that iterator itself panics first (indexing
`get_pixel(0, 1)` on a row with zero valid columns) - this is the pre-existing, already
investigated `001.PIXELS_ITERATOR_ZERO_WIDTH_OOB` bug (Confirmed, patch recorded and reverted
there). Because `SUMMARY.md`/`patch.diff` fixes from prior candidates are never left applied to
the main tree between candidates (per `SKILL.md`'s "the one rule that matters most"), that panic
is what actually fires today for the exact `width == 0` scenario the original candidate
description named - `make_arena_row`'s `.unwrap()`s are never reached for that input. `filter_1d`'s
own callers therefore never see `image_size.width == 0` today; if candidate 001's fix is ever
applied on its own without also fixing `filter_1d.rs`, `make_arena_row`'s `source_row` would then
be a genuinely empty slice for `width == 0` and `.first().unwrap()`/`.last().unwrap()` would panic
- so that code is latently buggy too, just not exercisable standalone right now.

**2. `height == 0` (the "other half" of the same "zero dimension" class) *is* reachable today, and
does panic inside `filter_1d.rs` itself** - `Pixels::next()` returns `None` immediately for
`height == 0` regardless of `width` (see candidate 001's `RESULT.md`), so the working buffer is
built successfully as an empty `Vec`, and `filter_2d_separable` is entered with
`image_size = { width: W, height: 0 }`. If the row/column kernel is at least `RING_QUEUE_CIRCULAR_CUTOFF`
(55) elements long, `filter_2d_separable` takes its "arena" branch instead of the ring-queue
branch (`src/imageops/filter_1d.rs:673`) and calls `make_columns_arenas` with the (empty)
`transient_image` buffer and `image_size.height == 0`:

```rust
let stride = image_size.width * N;               // > 0
let mut top_pad = vec![T::default(); pad_h * stride];      // pad_h > 0 for a wide kernel
let mut bottom_pad = vec![T::default(); pad_h * stride];

for dst in top_pad.chunks_exact_mut(stride) {
    dst.copy_from_slice(&image[..stride]);        // image.len() == 0, stride > 0 -> PANICS
}
for dst in bottom_pad.chunks_exact_mut(stride) {
    dst.copy_from_slice(&image[(image_size.height - 1) * stride..]);  // 0usize - 1 also underflows
}
```

`image` (the padded row buffer built one row at a time by the earlier loop) is empty because
`image_size.height == 0` means that loop's `chunks_exact_mut` produced zero rows to process, but
`make_columns_arenas` unconditionally slices `&image[..stride]` assuming at least one row exists to
replicate into the top/bottom padding - `stride` (`image_size.width * N`) is nonzero whenever the
image has any columns at all, so this always panics with a "range end index out of range" message
for any `width > 0, height == 0` input that reaches the arena path. The `bottom_pad` loop right
after it has an independent bug of the same root cause (`image_size.height - 1` underflows when
`height == 0`), never reached in this repro only because the `top_pad` loop panics first.

Both `1` and `2` are the same underlying class the candidate was pointing at ("this file's Gaussian
blur machinery doesn't account for a degenerate image dimension"), just triggered by the two
different dimensions in two different functions.

## Introduced by

159d03e "Gaussian blur" (2025-06-18) - `make_columns_arenas`'s unchecked `image_size.height - 1`
was present from the function's first commit; the later 2cbe32ed ("Simplify code in `filter_1d`",
2026-05-19) refactor only changed the panic's exact shape (slice-index panic vs. the original's
underflow panic), it did not introduce the underlying flaw.

## The fix

Added an early return in `filter_2d_separable` (`src/imageops/filter_1d.rs`), right after the
existing length-validation checks and before any kernel/arena work begins:

```rust
// A zero-width or zero-height image has no rows/columns to pad or convolve;
// `image`/`destination` are already provably empty from the length check above.
if image_size.width == 0 || image_size.height == 0 {
    return Ok(());
}
```

This is sufficient and safe because the preceding check (`image.len() != width * height * N`)
already guarantees `image`/`destination` are both empty whenever either dimension is `0`, so
returning `Ok(())` without touching either buffer is a correct no-op for every code path inside
this function (`ring_queue` and `arena`, `make_arena_row` and `make_columns_arenas` alike) - it
also structurally covers the latent `make_arena_row` zero-width bug described in root-cause point 1
above, not just the `height == 0` case this candidate's repro exercises, since neither function is
ever entered anymore once either dimension is zero. Verified against six boundary combinations (see
"What was tried" above and `output.txt`), not just the one triggering input, including the normal
non-degenerate case to confirm no behavior change there.

- Project test suite (`cargo test --lib`): **331 passed, 0 failed** before the patch -> **331
  passed, 0 failed** after the patch (identical pass count, no new failures, no regressions).

`patch.diff` contains this fix as a diff against `base_commit` (`44ce9226c541dd2b11897b9ef07156a28871cdb7`).

## What's not done / open questions

- The latent zero-*width* bug in `make_arena_row` (root-cause point 1) is now also structurally
  fixed by this same early return (neither `make_arena_row` nor `make_columns_arenas` is reached
  once either dimension is 0), but it was not independently re-verified with `width == 0` input
  against *this* patch in isolation from candidate 001's separate `Pixels::next()` fix, since
  applying only this candidate's patch still leaves candidate 001's bug panicking first for
  `width == 0, height > 0` on the unpatched tree. A future session applying *both* candidates'
  patches together should re-verify the `width == 0` case end-to-end.
- `make_columns_arenas`'s `bottom_pad` loop has its own independent `image_size.height - 1`
  underflow for `height == 0` (noted in "Root cause" point 2) - not exercised directly in this
  repro (the `top_pad` loop panics first), but covered by the same fix since the function is never
  entered for `height == 0` anymore.
- Did not check whether `filter_2d_separable_ring_queue` (the other branch, taken when
  `column_kernel.len() < 55`) has its own independent zero-dimension bug that this early return
  now also happens to preempt - not investigated in isolation since the fix already makes it moot
  for this crate's current callers.
- Open upstream PR #3007 (not merged as of this session) fixes the broader "zero-size image" class
  differently, by introducing a non-zero image-size type across the crate; this session's fix is
  narrower - a single guard clause in one function - and does not attempt that larger refactor.
- Per `SKILL.md` "Critical pitfalls" #7: this is the same "zero dimension" bug class as candidate
  001, in a different function of the same file. No other `filter_1d.rs`-adjacent callers were
  found this session (`filter_2d_sep_*` functions in `filter_1d.rs` are only ever called from
  `sample.rs`'s `gaussian_blur_indirect_impl`/`blur`, per a full-crate grep) - `fast_blur.rs` only
  reuses `filter_1d`'s `SafeAdd`/`SafeMul` helpers, not its arena/convolution code, so it's a
  structurally separate implementation not covered by this fix and not checked for the same bug
  class this session.
