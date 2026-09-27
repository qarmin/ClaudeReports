# Zero-sized Pixmaps are constructible and panic on sampling (#1269)

`Pixmap::new`/`Pixmap::from_parts` place no lower bound on `width`/`height`, so a `0`-wide or
`0`-tall `Pixmap` is trivially constructible through the public API. Using such a `Pixmap` as an
`ImageSource::Pixmap` paint and rendering a fill with it panics deep inside `vello_cpu`'s SIMD
image-sampling path with an out-of-bounds index into the (empty) pixel buffer. This looked
suspicious because dimensions of a decoded image very often come straight from untrusted
input/decoders, and `ImageSource::external_texture` already has an explicit `assert!(!source_region
.is_empty())` guard for the analogous case, showing the project itself considers empty image
sources a case worth rejecting - just not for the `Pixmap` variant.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + real public API | `Pixmap::new(0, 3)` used as an `ImageSource::Pixmap` paint, filled via `vello_cpu::RenderContext::fill_rect` + `render`, debug build | Reproduced immediately: panic in `Pixmap::sample_idx` ("index out of bounds: the len is 0 but the index is 0") |
| Repeat-run check (`CONFIRMATION.md` section 1) | Same binary, 5 runs | 5/5 runs panicked identically |
| Same-class boundary variants | `Pixmap::new(4, 0)` (zero height) and `Pixmap::new(0, 0)` (both zero) | Both panic the same way before the fix; both render successfully (no panic) after the fix |

A debug-assertions build alone was sufficient to show the bug (an ordinary `Vec` index out of
bounds, not something needing miri/ASAN); no sanitizer or fuzzing was needed given how directly
the boundary value reproduces it, so the soft 3-approach cap was not exhausted (one approach plus
its mandated repeat-run/variant checks was already conclusive).

## Status

**Confirmed**. Reproduces 5/5 runs, debug build, deterministic (same panic location and message
every run, no dependency on ASLR/thread scheduling/hash order).

**Fully reproducible**: **Yes**. The repro drives the real, public `vello_cpu` API end-to-end
(`RenderContext::set_paint` + `fill_rect` + `flush` + `render`) - the same sequence any real
`vello_cpu` caller uses - with a `Pixmap` built through `Pixmap::new`, its own real public
constructor. There is no second participant to fake here: a `Pixmap` with attacker/decoder-
controlled dimensions is itself the single external artifact this finding is about (directly
analogous to "a malformed file handed to a parser's real public `open_file`" in `CONFIRMATION.md`
section 11), so a single-process repro is the correct shape, not a downgrade.

## Class

crash

## Root cause

`Pixmap::new`/`Pixmap::from_parts` (`sparse_strips/vello_common/src/pixmap.rs`) never validate
that `width`/`height` are non-zero, so `buf` can legitimately be an empty `Vec`. Nothing between
that construction and the image-sampling fast path revalidates this invariant:

- `ImagePainterData::new` (`sparse_strips/vello_cpu/src/fine/common/image.rs`) computes
  `width_inv = 1.0 / width as f32`, which becomes `+inf` when `width == 0`, and `width_u32 = 0`.
- The `extend()` helper's `Pad` branch computes `val.min(max - 1.0).max(0.0)`; with `max == 0.0`
  this collapses to `0.0` for any typical (non-negative) sample position, regardless of the actual
  requested pixel position.
- `sample()` then computes `idx = x.to_int() + y.to_int() * width_u32`, which becomes `0` (since
  `width_u32 == 0` zeroes out the y-term and the x-term was already clamped to `0`), and calls
  `pixmap.sample_idx(0)`, i.e. `self.buf[0]` - but `buf.len() == 0`, so indexing panics.

So the code silently "resolves" any sample position to index `0` for a zero-sized pixmap instead
of detecting the degenerate case, and then indexes into a buffer that has no element `0` at all.
The same class of gap exists for `ImageSource::OpaqueId` (an `ImageResolver` is free to return an
`Arc<Pixmap>` with zero dimensions the same way) - the fix below covers both, since both resolve to
a `&Pixmap` at the same call site before any painter is constructed.

## Introduced by

b7f82779 "Add support for rendering images (#919)" (2025-04-22) - this is the commit that first
added the image-sampling fast path (`sample()`/`ImagePainterData`) that assumes a non-empty pixel
buffer without checking it. `Pixmap` itself predates this commit and has never validated
`width`/`height` against zero (`git log -S "fn sample_idx" -- sparse_strips/vello_common/src/pixmap.rs`
also lands on the SIMD rewrite that introduced today's `sample_idx`, cce9d918 "Rewrite `vello_cpu`
to be more SIMD-friendly (#1078)", but the underlying assumption - a pixmap's buffer always has
`width * height > 0` elements when used as an image source - goes back to #919, the commit that
first started indexing into pixmap data with a computed sample index instead of only ever writing
through `set_pixel`/`data_mut`).

## The fix

Added an early return in `vello_cpu`'s `Fine::indexed_fill`, right after resolving `image.source`
to a concrete `&Pixmap` (covering both the `ImageSource::Pixmap` and `ImageSource::OpaqueId`
cases) and before any image painter is constructed: if the resolved pixmap has zero width or zero
height, skip painting this span entirely (leave the destination untouched), matching the existing
`solid_fill`'s own `if span.pixel_width() == 0 { return; }` early-return pattern elsewhere in the
same file. A zero-sized image has no pixels to contribute, so "leave the destination as it was" is
the correct, transparent-compositing-equivalent behavior, not just a crash-avoidance stopgap.

This was chosen over rejecting zero dimensions in `Pixmap::new`/`from_parts` themselves, because
`Pixmap` is also used as a general-purpose render *target* (e.g. `Pixmap::new(ctx.width(),
ctx.height())` in several examples/tests), where a `0`-sized target is a legitimate degenerate
case that should render as a no-op, not panic at construction time - so the check belongs at the
point where a `Pixmap` is specifically being *used as an image source*, not at general
construction time.

Verified against all three boundary variants in the same class (zero width, zero height, both
zero) - all three panicked before the fix and all three render successfully (no panic, `did not
reproduce`) after it. Re-ran `repro/` against the patched tree: no longer reproduces (was:
`REPRODUCED - panicked`, now: `did not reproduce - rendering with a zero-width Pixmap image
completed normally`).

- Project test suite (`cargo test -p vello_cpu -p vello_common --lib`, this sparse_strips
  workspace): 159 passed / 0 failed / 1 ignored (vello_common) and 81 passed / 0 failed / 0
  ignored (vello_cpu), identical before and after the patch - no regressions, no new failures.

## What's not done / open questions

- `vello_hybrid` (the GPU/WebGL renderer) does not support `ImageSource::Pixmap` at all yet
  (`unimplemented!("Unsupported image source")` in `sparse_strips/vello_hybrid/src/paint.rs`), so
  it was not checked for the same class of bug - there is currently nothing to check there for
  this specific `ImageSource` variant.
- `PixmapMut::new` still allows constructing a `0`-sized mutable pixmap view (`width * height * 4
  == 0` matches a `0`-length buffer just fine) - not fixed here since it is only used as a target
  view (via `Pixmap::as_mut`/`PixmapMut::from`), not as an image source, so it was not observed to
  reach the same panic; worth a maintainer's second look if `PixmapMut` ever grows a sampling path
  of its own.
- No fuzzing/miri run was performed - the bug was directly, deterministically reproducible from a
  single boundary-value construction, so escalating to those tools was not needed per the soft cap
  in `SKILL.md`.
