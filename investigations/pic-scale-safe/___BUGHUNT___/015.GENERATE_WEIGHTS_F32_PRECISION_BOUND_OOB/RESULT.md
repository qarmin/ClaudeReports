# generate_weights float-domain in_size clamp lets FilterBounds.size exceed the true source width/height

Investigated as the backlog entry `BOUNDS_SIZE_CHANNELS_OVERFLOW` (`___BUGHUNT___/IDEAS.md`,
seeded while working candidate 014): `bounds_size * CHANNELS` in `src/fixed_point_horizontal.rs:73,141-144`
and `bounds.size * CHANNELS` in `src/floating_point_horizontal.rs:75,144-147`, used with plain
(non-overflow-checked) multiplication to slice `src`. The backlog note's open question was whether
a caller-controlled value can drive `bounds_size` large enough to overflow that multiplication
independently of the length checks already confirmed/fixed in candidate 009.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: trace `bounds.size`'s upper bound through `generate_weights` (`src/compute_weights.rs`) back to the `overflowing_mul`-checked `src_stride = width * CHANNELS` computed in `resize_fixed_point`/`resize_floating_point`/`convolve_row_fixed_point`/`convolve_trampoline_fixed_point` before `generate_weights` is ever called | Read-only | In exact real-number arithmetic, `bounds.size <= in_size` always holds (`end = min(ceil(...), in_size, start+kernel_size)`, `start >= 0`), so `bounds_size * CHANNELS <= in_size * CHANNELS`, which is already proven non-overflowing by the caller's own `overflowing_mul` check - **refutes** the literal "arithmetic multiplication overflow" hypothesis as originally framed |
| In-tree `#[cfg(test)]` probe in `src/compute_weights.rs` (`bughunt_015_probe`), `cargo test --lib`, debug build: called `generate_weights::<f32>` directly with `in_size` near `usize::MAX/CHANNELS` (fixed-kernel filters `Spline16`/`Spline64` to keep `kernel_size` small) | debug, `cargo test --lib bughunt_015_probe` | No overflow/panic - `bounds.size` stayed near `0`/small for these inputs, consistent with the reasoning above; did **not** by itself reveal the real bug |
| Follow-up probe: swept `out_size` for `in_size = 16_777_219` (chosen so `(in_size as f32) as usize == 16_777_220`, i.e. rounds UP by 1 - just above the `2^24` f32-precision boundary) against `ResamplingFunction::Bilinear`, checking whether any `FilterBounds{start,size}` has `start+size > in_size` | debug, same test binary | **Signal found**: 12 of 15 sampled `out_size` values produced a bound with `end > in_size` (e.g. `out_size=1` -> `FilterBounds{start:0, size:16777220}`, one past the true `in_size`) - see `output.txt` for the full sweep |
| Standalone `repro/` via the crate's public API (`resize_plane8`/`resize_plane_f32`, `width=16_777_219`, `height=1`, `dst_width=1`), `cargo run` and `cargo run --release` | debug 3x, release 1x, against a pristine checkout | Deterministic panic every run: `range end index 16777220 out of range for slice of length 16777219` at `src/fixed_point_horizontal.rs:73:28` (fixed-point path) and `src/floating_point_horizontal.rs:75:28` (floating-point path) |

## Status

**Confirmed** - reproduces deterministically, 4/4 runs (3 debug + 1 release), identical panic
location and message every time, via the crate's public API with an ordinary ~16 MB single-row
`u8`/`f32` image (no multi-gigabyte allocation needed).

The originally-suspected mechanism (`bounds_size * CHANNELS` overflowing `usize` arithmetically)
is **refuted** - that multiplication never overflows for any width the crate's own stride checks
(candidate 009's guard) already let through, since `bounds.size <= in_size` holds in exact
arithmetic. What *is* real, at the exact same code sites, is a different bug: `bounds.size` itself
can be computed larger than the true `in_size` due to floating-point precision loss inside
`generate_weights`, which then drives an out-of-bounds *slice* (not an overflowing multiplication)
in the very lines the backlog entry pointed at.

## Class

crash

## Root cause

`generate_weights` (`src/compute_weights.rs`, both the general/non-area branch and the `is_area`
branch) computes each destination pixel's source window as `usize` `start`/`end` values, but does
the actual window-clamping arithmetic in the crate's generic float type `T` - concretely `f32` for
every real public entry point (`resize_plane8`, `resize_rgba8`, `resize_plane_f32`, etc., all the
way down through `resize_fixed_point`/`resize_floating_point`). Two lines convert the integer
`in_size: usize` (source width, or height for the vertical pass) to `T` for a clamp, then cast the
clamped float value back to `usize`:

```rust
let center_x = ((i.as_() + 0.5.as_()) * scale).min(in_size.as_());
...
let end: usize = (center_x + filter_radius)
    .ceil()
    .min(in_size.as_())          // <- in_size converted to f32 here
    .min(start.as_() + kernel_size.as_())
    .as_();                      // <- clamped f32 value cast back to usize here
```

`usize -> f32 -> usize` is lossy for any value above `2^24` (16,777,216) since `f32` has only a
24-bit mantissa. For an `in_size` whose nearest representable `f32` rounds **up** (roughly half of
all integers above `2^24` do - e.g. `16_777_219.0` isn't representable, and the nearest `f32`,
`16_777_220.0`, is on the far side), the clamp bound used inside the function is that rounded-up
value, not the true `in_size`. This lets the computed `end` (and therefore
`bounds.size = end - start`, stored straight into the `FilterBounds` returned to every caller)
exceed the true, integer `in_size` by however much the rounding overshot - `1` in the case
exercised here, confirmed to reach as high as `4` for other `out_size` values in the same sweep
(see `output.txt`).

Downstream, `fixed_point_horizontal.rs`/`floating_point_horizontal.rs` slice the source row as
`&src[px..(px + bounds_size * CHANNELS)]` using that inflated `bounds_size`. Since the row's real
data is only `in_size * CHANNELS` elements long, the slice's end index lands past the end of the
row - `range end index 16777220 out of range for slice of length 16777219`, a slice-bounds panic
at exactly the lines the backlog entry named, but via a wrong-input-to-the-slice-operator
mechanism rather than the multiplication itself overflowing.

## Introduced by

5e72885 "Initial commit" (2024-10-16) - `git log -S"center_x = ((i.as_"` finds this as the commit that introduced the `in_size.as_()`-based float clamp in `generate_weights`, unchanged in shape since.

Still present in current HEAD (checked while filling in this section: the `center_x`/`end` computation is unchanged) - not yet fixed upstream.

## The fix

Reclamp `start` and `end` to the true, integer `in_size` immediately after each is cast back from
`T` to `usize`, in both branches of `generate_weights` (the general/non-area branch and the
`is_area` branch):

```rust
let start: usize = (center_x - filter_radius).floor().max(0f32.as_()).as_();
let start: usize = start.min(in_size);
let end: usize = (center_x + filter_radius)
    .ceil()
    .min(in_size.as_())
    .min(start.as_() + kernel_size.as_())
    .as_();
let end: usize = end.min(in_size);
```

(and the equivalent two lines in the `is_area` branch). This is a pure integer `usize::min`
against the function's own `in_size: usize` parameter - it sidesteps the float-precision question
entirely rather than trying to make the `f32` clamp itself exact, and cannot make `end < start`
(both are independently clamped down from values that already satisfied `start <= end` before this
change, and `min` is monotonic, so the ordering is preserved).

Verified against the repro (varying the input within the same bug class, per `CONFIRMATION.md`
section 7 - not just the exact triggering case):

- **Before fix** (pristine clone): `repro/` panics deterministically for both the fixed-point and
  floating-point cases (`width=16_777_219`, `dst_width=1`, `ResamplingFunction::Bilinear`) - 3/3
  debug runs, 1/1 release run. The in-tree `bughunt_015_probe` sweep independently confirms
  `FilterBounds.size` exceeding `in_size` for 12 of 15 sampled `out_size` values at
  `in_size=16_777_219`.
- **After fix**: `repro/` no longer panics for either case in either build (`Ok([0])`/`Ok([0.0])`),
  and the sanity case (`width=1_000_000`, well under the `2^24` boundary, `dst_width=10`) still
  produces the correct 10-element output. The `bughunt_015_probe` sweep reports `overflow_bound:
  None` for every one of the same 15 `out_size` values after the fix.
- Project test suite: `cargo test --all-features --workspace` before fix - `app` binary crate's 1
  test (`image_wrapper::tests::test_rescaling`) passed (1 passed, 0 failed); `pic-scale-safe` lib
  itself has 0 tests (N/A, no unit/integration tests in this package pre-existing in the repo).
  After fix: identical `app` result (1 passed, 0 failed); the lib shows 1 passed test, but that is
  this session's own temporary `bughunt_015_probe` scaffolding (removed before the working tree
  was reverted, not part of `patch.diff`) - no regression either way.

## What's not done / open questions

- Only the width/horizontal-pass direction was driven all the way through a public-API repro;
  `generate_weights` is also called with `in_size = source_size.height` for the vertical
  (column) pass (`convolve_column_fixed_point`/`floating_point_dispatch`'s equivalent), which
  shares the exact same `generate_weights` code path and is therefore fixed by the same patch, but
  was not separately reproduced with a >2^24-height image (would need a taller allocation to set
  up economically with `height` as the varying dimension instead of `width`).
- Not reswept: whether every one of the ~35 `ResamplingFunction` variants (each with its own
  `min_kernel_size`/window function) can reach the `end > in_size` condition at exactly the same
  `in_size` used here - the probe used `Bilinear` (a representative resizable-kernel filter) for
  the sweep and `Spline16`/`Spline64` (fixed-kernel) for the huge-`in_size` sanity check; the root
  cause (the `usize -> f32 -> usize` round-trip on `in_size`) is filter-independent, but this was
  not exhaustively re-verified for every filter.
- The fix is a defense-in-depth integer reclamp, not a fix to the underlying `f32` precision loss
  itself - `center_x`, weight positions, and the resampling kernel's own math still operate in
  lossy `f32` for source dimensions above `2^24`, so resize *quality* (not safety) for such huge
  single-axis dimensions was not evaluated and may still be inaccurate near the affected edge; only
  the out-of-bounds-panic consequence was in scope for this bounded fix.
- Same underlying "value that should stay `<= in_size` doesn't, due to a lossy conversion"
  pattern as this session's other size-overflow candidates (002/009/013/014), but arrived at via
  floating-point precision loss rather than integer-overflow wraparound - not independently
  swept for other `usize -> f32`/`usize -> f64` round-trips elsewhere in `src/compute_weights.rs`
  or `src/filter_weights.rs` beyond the two call sites patched here.
