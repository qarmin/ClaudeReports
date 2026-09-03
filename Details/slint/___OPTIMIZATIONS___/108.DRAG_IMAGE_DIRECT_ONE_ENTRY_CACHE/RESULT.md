# DRAG_IMAGE_DIRECT_ONE_ENTRY_CACHE

Follow-up to `107.TEXTURE_CACHE_DRAIN_FULL_WALK`, from an `IDEAS.md` entry noticed while building
that candidate: `internal/renderers/femtovg/itemrenderer.rs`'s `draw_image_direct` (used only for
`DragArea.drag-image`'s overlay, drawn every frame a drag is in flight - the software/linuxkms
cursor-overlay callers hit the software renderer's own, unrelated `draw_image_direct`) has an
uncached `.or_else()` fallback for images with no stable `ImageCacheKey` (per
`ImageCacheKey::new`'s doc comment, this is *always* true for `Image::from_rgba8`/`from_rgb8`).
Unlike `draw_image_impl` (used for `Image {}` items), which is wrapped in a per-item
`ItemGraphicsCache` that already avoids re-running its update closure for an unchanged property
value, `draw_image_direct` has no such wrapper at all - so a `drag-image` built at runtime is
re-uploaded to the GPU from scratch on every single rendered frame for as long as the drag lasts.

## What was tried

Added a `direct_image_cache: RefCell<Option<(Image, Rc<Texture<R>>)>>` field to `FemtoVGRenderer`
(threaded through to `GLItemRenderer` the same way `texture_cache` already is). In
`draw_image_direct`'s `.or_else()` fallback: if the incoming `image` equals (`Image`'s own
`PartialEq`, a real content comparison for buffer-backed images - see "Why" below) the last-cached
one, reuse its texture; otherwise create a new one and replace the cached entry. Cleared alongside
`texture_cache` at the two existing invalidation points (`set_window_adapter`,
`clear_graphics_context`) so a stale texture reference can't survive a context/window-adapter
reset.

`perf_test` had no way to reach `draw_image_direct` repeatedly at all (it wraps `Image {}` items,
not `DragArea.drag-image`) - added a `--drag-probe` flag: an always-present `DragArea` in
`main.slint` (gated by `enabled:`, not `if`, so its id stays reachable), a `DragProbe` global
providing a non-empty `data-transfer` (required for a drag to start at all), and `main.rs` code
that shows the window, dispatches a `PointerPressed` + one `PointerMoved` past the 8px drag
threshold to start an in-window drag, and never releases it - so the drag (and its
`draw_image_direct` overlay) stays in flight for the whole `--frames` run. Confirmed via a
temporary upload counter in `Texture::new_from_image` that baseline grows linearly with frame count
(89 uploads at `--frames 30`, +30 more at `--frames 60` - one per rendered frame) while the patched
build stays flat regardless of frame count (same 89, the one initial upload plus a few during
window/canvas setup).

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 | notes |
|---|---|---|---|
| baseline, `--mode images --drag-probe --frames 40 --size 800x600` (default `--image-size 64`) | 0.788 ms | - | tiny drag image; upload cost negligible vs. frame overhead |
| patched, same | 0.786 ms (-0.3%) | - | within noise |
| baseline, `--image-size 1024`, same flags | 0.894 ms | 1.280 ms | |
| patched, same | 0.786 ms (-12.1%) | 1.170 ms (-8.6%) | |
| baseline, `--image-size 1024`, repeat | 0.903 ms | - | reproducibility check |
| patched, same | 0.782 ms (-13.4%) | - | |
| baseline, `--image-size 2048`, same flags | 2.270 ms | 2.890 ms | |
| patched, same | 0.786 ms (-65.4%) | 1.150 ms (-60.2%) | |

All numbers: `RUNS=9`, `paired_femtovg_later_frames.sh`, median of 9 alternating runs each side,
`--renderer femtovg`, `xvfb-run`, `LIBGL_ALWAYS_SOFTWARE=1` (Mesa llvmpipe). Machine: 24 cores,
`uptime` load 3-5 at measurement time (below core count). Noise floor (same baseline binary vs a
copy of itself, single-core, `--image-size 1024`): +1.0% - both the 1024px and 2048px deltas are
well outside it. Per-pair win count (not just medians, per MEASUREMENT.md): 1024px single-core
8/9 pairs favor the patched build; 2048px single-core and multi-core both 9/9.

## Result

A real, reproducible win that scales with the drag image's size: negligible at the harness's tiny
default (64x64, `--image-size` unset) since a GPU upload of a few KB is already cheap relative to
frame overhead, but -8.6% to -65.4% at 1024px/2048px - sizes a real drag-preview thumbnail (a
rendered widget snapshot, a decoded-in-app image) plausibly uses. The effect is one-sided and
monotonic with image size, consistent with the mechanism (a full GPU texture upload eliminated per
frame, and upload cost scales with byte size) rather than noise.

## Does it change behavior/output?

- **Bit-exact?** No - and the difference is a *fix*, not merely a difference. See "A latent bug
  this happens to fix" below.
- **Is this a breaking change?** No. Internal-only change to `i-slint-renderer-femtovg`'s private
  `GLItemRenderer`/`FemtoVGRenderer`; no public API change.
- **Non-pixel behavior change**: an evicted/replaced drag-image texture (i.e., once the app sets a
  *different* `drag-image` mid-drag) is held one generation longer than before (the just-replaced
  entry's `Rc` drops when the cache slot is overwritten on the *next* differing draw, not
  immediately) - bounded to at most one extra texture's worth of GPU memory, not unbounded growth.
- Project test suite: `i-slint-renderer-femtovg` has no unit tests, before or after (`cargo test -p
  i-slint-renderer-femtovg`: 0 passed; 0 failed, both builds). `tests/driver/rust`'s 4
  `dragarea_*` cases (`cargo test --release dragarea` in `tests/driver/rust`) pass identically
  before and after (4 passed; 0 failed both) - expected, since they run on the mock `slint_testing`
  platform, not femtovg, so they don't exercise the fixed/patched code path directly; they do
  confirm no regression to `DragArea`/drag-image's core (non-rendering) behavior.

### A latent bug this happens to fix

While building the harness, the patched and unpatched builds' `--snapshot` output differed - not
in a way explained by the drag image's real (screen-space) position, which led to finding a real,
pre-existing, standalone bug: on the unpatched tree, `DragArea.drag-image` renders as **solid
black** instead of the actual image whenever the image has no stable `ImageCacheKey` (i.e. always,
for `Image::from_rgba8`/`from_rgb8`). Root cause: the `.or_else()` fallback's `Rc<Texture>` is used
once and dropped at the end of `draw_image_direct`; `Texture::drop()` deletes the femtovg image ID
immediately, which happens *before* that frame's `canvas.flush_to_output()` call - the same
"delete after flush, not before" hazard `GLItemRenderer::textures_to_delete_after_flush` exists to
avoid elsewhere in this exact file, just not applied to this call site. This candidate's cache
happens to fix it as a side effect (the `Rc` now stays alive across frames instead of being dropped
every single call), confirmed by a standalone reproduction.

Filed as a standalone finding, independent of whether this performance patch is taken: see
`___OTHER___/003.DRAG_IMAGE_TEXTURE_DELETED_BEFORE_FLUSH/` (`BUG_WRITEUP.md`, a real runnable
`repro/`, and `backtrace.txt` with both builds' captured output). A fix scoped to *only* the bug
(not the performance goal here) would defer the delete via the same
`textures_to_delete_after_flush` mechanism instead of adding cross-frame caching.

Given this, the pixel-level correctness check for *this* candidate is a positive one: the patched
build's snapshot shows the correct image content where the unpatched build shows black - not a
"characterized difference to weigh" in the usual sense, but a bug fix. `cmp`-identical framing
doesn't apply here since the baseline itself is wrong.

## Why the result looks the way it does

`draw_image_direct`'s only two call sites (`internal/core/window.rs`'s
`render_drag_image_overlay`, and the linuxkms software-cursor injection, which uses the *software*
renderer's own unrelated `draw_image_direct`) both redraw the same image every single frame for as
long as an interaction (a drag, or the cursor being visible) lasts - the textbook case a cache
helps, once one exists at all. The magnitude scaling with image size confirms the mechanism is a
real GPU upload (bytes-proportional cost), not fixed per-call overhead - matching candidate 107's
finding that `TextureCache::drain()`'s own per-entry cost is negligible, while an actual
create+upload+destroy cycle (candidate 057's target, and this one) is not.

## What's not done / open questions

- Only femtovg's `draw_image_direct` is patched. The skia and software renderers' own
  `draw_image_direct` were not checked for the same uncached-fallback shape (the software
  renderer's version was read while investigating the bug above and works differently - it streams
  pixels directly via `draw_image_impl`'s scanline path, not a GPU texture, so this specific
  upload-elimination doesn't apply there, but wasn't otherwise audited for its own equivalent
  inefficiency).
- The one-entry cache only helps the "same image redrawn unchanged" case. A drag preview that's
  genuinely regenerated every frame (an animated/canvas-style drag image) gets no benefit and pays
  the same cost as before - not a regression, just outside this cache's design.
- Not measured against a real GPU driver (only Mesa llvmpipe via `LIBGL_ALWAYS_SOFTWARE=1`) -
  upload cost and the before-flush-deletion bug's manifestation could both differ with real
  hardware/driver timing, though the underlying ordering hazard is driver-independent in principle.
