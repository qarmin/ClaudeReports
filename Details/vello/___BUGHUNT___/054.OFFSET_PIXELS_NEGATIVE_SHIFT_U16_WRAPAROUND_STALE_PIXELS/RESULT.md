# OFFSET_PIXELS_NEGATIVE_SHIFT_U16_WRAPAROUND_STALE_PIXELS

`sparse_strips/vello_cpu/src/filter/shift.rs`'s `process_offset_pixel` (the shared in-place
pixel-shift helper used by both the `feOffset` filter, `sparse_strips/vello_cpu/src/filter/offset.rs`,
and `feDropShadow`, `sparse_strips/vello_cpu/src/filter/drop_shadow.rs`) computes `should_clear` by
casting `width as i32 + dx_pixels` down to `u16`. When `dx_pixels` is negative and its magnitude
exceeds the pixmap's width, that sum is itself negative, and an `i32 -> u16` `as` cast wraps
(truncates) rather than saturating - this looked suspicious purely from reading the arithmetic
(HEURISTICS.md's "arithmetic on untrusted sizes/casts between integer widths" pattern).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug-assertions unit test added directly to `shift.rs` (`offset_pixels(&mut pixmap, -10.0, -10.0)` on a 4x4 `Pixmap`) | `cargo test -p vello_cpu --lib filter::shift`, debug build, run 4x | Reproduced every time: pixel (1,1) kept `a=255` instead of being cleared. Confirms the unit-level mechanism, but this test was hand-authored pixmap state, not driven through a real caller - reverted before finalizing, not used as the final repro. |
| Real end-to-end repro via `vello_cpu::RenderContext::push_filter_layer` + `Filter::from_primitive(FilterPrimitive::Offset { dx: -70000.0, .. })`, filling a real rectangle and rendering to a real `Pixmap` | Release build, `cargo run --release`, run 4x (plus once more via the candidate's own fresh-clone `just test`) | Reproduced every time: the rectangle's original location stayed fully opaque after an offset that should have moved it off-canvas, while a smaller (`dx = -30`) offset through the identical code path correctly clears it. This is the repro recorded in `repro/`. |

Manual boundary-value reasoning (no sanitizer/fuzzer needed - the bug is a plain, deterministic
integer-cast logic error, not memory-unsafety or a crash) was enough to find both the unit-level
trigger and, by reading how the filter pipeline sizes its internal pixmap
(`vello_common::filter::FilterLayerPlacement`/`FilterData`), the specific `dx` magnitude
(`|dx| > u16::MAX`) needed to reach it through the real public API.

## Status

**Confirmed.** Deterministic, 4/4 runs via the real `push_filter_layer` end-to-end path (plus an
additional 4/4 at the unit level during initial triage), including one run via the candidate's own
`just test` recipe against a completely fresh clone.

**Fully reproducible**: **Yes**. `repro/src/main.rs` drives the real, public `vello_cpu::RenderContext`
API end-to-end (`push_filter_layer` with a real `Filter`/`FilterPrimitive::Offset`, a real
`fill_rect`, a real `render_with` into a real `Pixmap`) - no hand-constructed pixmap/filter state.
This finding is not about one participant reacting to another's input (no peer/wire/caller
boundary involved), so CONFIRMATION.md section 11's second-participant requirement doesn't apply.

## Class

logic

## Root cause

In `process_offset_pixel` (`sparse_strips/vello_cpu/src/filter/shift.rs`):

```rust
let should_clear = (dx_pixels > 0 && x < dx_pixels as u16)
    || (dx_pixels < 0 && x >= (width as i32 + dx_pixels) as u16)
    || (dy_pixels > 0 && y < dy_pixels as u16)
    || (dy_pixels < 0 && y >= (height as i32 + dy_pixels) as u16);
```

For the negative-`dx_pixels` branch, `width as i32 + dx_pixels` is meant to be the smallest local
`x` that has been "exposed" (moved off-canvas) by the shift and therefore needs clearing. If
`dx_pixels`'s magnitude exceeds `width`, that sum is negative (e.g. `width = 4`, `dx_pixels = -10`
-> `-6`). Casting that negative `i32` to `u16` with `as` truncates/wraps instead of saturating
(`-6i32 as u16 == 65530`), and since a real `x: u16` can never be `>= 65530`, `should_clear`
becomes wrongly `false` for every pixel in that row - the source pixel keeps its original color
instead of becoming transparent.

This is reachable through the *real* filter pipeline (not just a hand-built pixmap) because the
guard that would normally prevent it - the filter layer's internal pixmap being sized with enough
left/top padding to always exceed `|dx_pixels|` - itself breaks down for large offsets. That
padding is computed in `vello_common`'s `filter::mod::FilterData`/`FilterLayerPlacement` via
`math::snap_up(value: f64, step: u16) -> f64` followed by `as u16` in `snapped_padding`. That's a
*float-to-int* cast, which *saturates* at `65535` (Rust's defined, non-wrapping behavior for
float->int `as` casts) rather than wrapping. So once `|dx|` exceeds `u16::MAX` (65535), the padding
added to the pixmap caps out at 65535 while `dx_pixels` itself (an `i32`, computed via
`dx.round() as i32`, not capped until roughly `i32::MAX`) keeps growing - the pixmap ends up
narrower than `|dx_pixels|`, and `process_offset_pixel`'s `i32 -> u16` (int-to-int, *wrapping*)
cast then produces the wrong, wrapped comparison bound described above.

The equivalent positive-`dx_pixels` branch (`x < dx_pixels as u16`) has the same class of cast
issue (`dx_pixels as u16` for `dx_pixels > u16::MAX` also wraps, e.g. `70000i32 as u16 == 4464`),
but the one concrete scenario tried this session (`dx = 70000`, otherwise identical setup)
happened to still render correctly, because the wrapped threshold (4464) still exceeded every `x`
in that particular small test pixmap, and the "moved" pixel also legitimately fell out of bounds
for an unrelated reason. This session did not find a concrete input that visibly breaks on the
positive side - see "What's not done" below - but the same defensive `i32`-throughout fix removes
the same class of cast in both branches regardless.

## Introduced by

ed5a7c65 "vello_cpu(filters): Add offset filter (#1351)" (2026-01-14) - this commit added
`shift.rs` (and `process_offset_pixel`'s `should_clear` computation) from scratch; `git log -S`
against the file shows no earlier revision.

## The fix

Compare in `i32` throughout instead of narrowing a possibly-negative `i32` sum down to `u16`
before comparing:

```rust
let should_clear = (dx_pixels > 0 && (x as i32) < dx_pixels)
    || (dx_pixels < 0 && (x as i32) >= width as i32 + dx_pixels)
    || (dy_pixels > 0 && (y as i32) < dy_pixels)
    || (dy_pixels < 0 && (y as i32) >= height as i32 + dy_pixels);
```

`x`/`y` are widened to `i32` (a real `u16 -> i32` widening cast, always exact) instead of ever
narrowing `dx_pixels`/`dy_pixels`/`width`/`height` down to `u16`, so there's no cast in this
expression that can wrap regardless of how large `|dx_pixels|`/`|dy_pixels|` are (short of
overflowing `i32` itself, which would need an offset in the billions of pixels - `dx.round() as
i32` already saturates at `i32::MAX`/`i32::MIN` for an `f32` input, per Rust's defined float->int
cast behavior, so this can't overflow `i32` either).

Verified: `repro/`'s `dx = -70000.0` case now reports "did not reproduce" (pixel fully transparent,
matching the `dx = -30.0` sanity case) both live in the working tree and via the candidate's own
`just test`/`just apply`/`just reproduce-after` against a fresh clone.

- Project test suite (`cargo test -p vello_cpu --lib`): 81 passed / 0 failed before -> 81 passed / 0
  failed after (no change, no regression).
- `cargo test -p vello_sparse_tests --test tests filter::`: 7 passed / 442 failed / 222 ignored
  before -> identical 7 passed / 442 failed / 222 ignored after. The 442 failures are pre-existing
  and environmental (this checkout's `git-lfs` binary is missing from `PATH`, so the LFS-tracked
  reference PNGs these snapshot tests compare against are unresolved pointer files, not real
  images) - confirmed identical by running the same command against the unpatched tree via `git
  stash`. Not caused or affected by this patch either way.

## What's not done / open questions

- The positive-`dx_pixels`/`dy_pixels` branches (`x < dx_pixels as u16`, `y < dy_pixels as u16`)
  have the same class of wrapping `i32 -> u16` cast for `dx_pixels`/`dy_pixels` magnitudes above
  `u16::MAX`, but this session did not find a concrete rendering scenario where that produces
  visibly wrong pixels (see "Root cause" above) - the fix removes the same cast there anyway as a
  defensive measure, but the positive-direction case itself was not independently confirmed as
  producing wrong output before the fix.
- `feDropShadow` (`drop_shadow.rs`) calls the same `offset_pixels` helper on its own shadow-copy
  pixmap with the same `dx`/`dy` and the same class of internal-pixmap sizing (plus extra padding
  for the blur radius, which only makes the safe margin larger, not smaller) - not independently
  exercised with an out-of-range `dx`/`dy` this session, but shares the exact same code path and
  root cause, so the fix covers it too.
- Not investigated: whether an offset with `|dx|`/`|dy|` in the billions (near `i32::MAX`) has any
  other failure mode (e.g. pixmap allocation size, `RectU16` arithmetic) independent of this
  specific cast bug - out of scope for this candidate.
