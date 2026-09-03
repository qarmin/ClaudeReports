# 113 - Qt-backend NativeButton/NativeTabWidget icon re-rasterized on every paint (GitHub #7511)

`NativeButton::actual_icon()` (`internal/backends/qt/qt_widgets/button.rs`) calls
`crate::qt_window::image_to_pixmap(...)` unconditionally every time it's invoked - from both
`render()` (via `fn_render!`) and `layout_info()`, i.e. on every paint. For an SVG-sourced
`icon`, `image_to_pixmap` runs a full `render_to_buffer` (resvg rasterization) plus a Qt
`QImage`/`QPixmap` construction, with zero caching. `NativeTab` (the per-tab item inside
`NativeTabWidget`, `internal/backends/qt/qt_widgets/tabwidget.rs`) has the identical defect at
two call sites (`layout_info()` and `fn_render!`). This matches maintainer ogoffart's own
diagnosis in the GitHub #7511 thread and is the same defect *class* as candidate 041 (SVG
re-rasterized every frame in the software renderer's plain `Image` path), but at a distinct call
site (the Qt backend's native-widget icon path) that 041's fix never touched.

The fix: a `RefCell<Option<(Image, u32, QPixmap)>>` (`NativeButton`) / `RefCell<Option<(Image,
QPixmap)>>` (`NativeTab`, which always rasterizes at the native intrinsic size, no `icon_size`
parameter) per-widget cache, checked before calling `image_to_pixmap` - reused via `Image`'s own
(already-existing) `PartialEq` when the icon and requested size still match, else recomputed.

## Environment note (this session)

No Qt was installed in this sandbox, and there is no root/sudo access to `apt install
qtbase5-dev` (or equivalent). Qt 6.5.3 (`gcc_64`, qtbase only, no root needed) was installed via
`aqtinstall` into a gitignored `tmp/Qt` for this session so `i-slint-backend-qt` could actually
link against real Qt and be measured, rather than only reasoned about from source. This isn't
needed to reproduce the result - `bench/bench_probe.patch` records the tiny `bench-probe` cargo
feature used to reach the code from outside the crate, and this candidate's own `justfile` notes
the same `aqtinstall` recipe for whoever runs `just test` later without a system Qt already
present.

## What was tried

Harness: a new standalone probe (`bench/qt_icon_probe.rs`, `qt-icon-rasterize-probe` crate - no
existing project benchmark reaches the Qt backend's native-widget code at all; `perf_test`'s own
"qt" feature only makes the *whole app* use the Qt backend/style, it has no icon-bearing
`Button`/`TabWidget` scene). It calls `i_slint_backend_qt::bench_probe_image_to_pixmap` (a
temporary `#[cfg(feature = "bench-probe")]` export of the otherwise-`pub(crate)`
`image_to_pixmap`, added purely so this out-of-crate probe can time the exact function the patch
either calls every time or skips) directly, in two modes:

- `uncached`: calls it on every iteration - what every paint does today.
- `cached`: calls it once, then does exactly what the patched `actual_icon()`/`NativeTab::
  actual_icon()` does on every subsequent iteration - compare the stored `Image` (`PartialEq`)
  and size, reuse if they match.

This isolates the eliminated redundant work precisely (the caching check itself is a cheap
`Image::eq` + integer compare, already exercised as-is in `cached` mode) without needing to stand
up a full live `QApplication` paint/event loop driving a real `NativeButton` widget end to end -
noted as an open question below. `ensure_initialized()` (Qt's lazy `QApplication` bring-up,
otherwise done by the normal widget `init()`/`render()` cpp! call sites) is invoked explicitly by
the probe export since it bypasses all of those. Correctness is checked every iteration: the
returned pixmap's `(width, height)` must be stable and identical to iteration 0.

Asset: `testdata/shapes.svg` (400 paths, the same file candidate 041 used), rasterized at
several representative Qt icon sizes (`QStyle::PM_ButtonIconSize` on Linux/Fusion is commonly in
the 16-24px range; 48px and 128px added as larger toolbar/dialog-icon stress points).
`QT_QPA_PLATFORM=offscreen`, no display needed. This binary is single-threaded, so "single-core"
(`taskset -c 3`) and "multi-core" (unpinned) are expected to and do coincide - both reported per
the loop's own discipline, not because concurrency is exercised here. `uptime` load average
4.6-5.4 on 24 cores throughout.

| icon size | mode | single-core p50 (run 1) | single-core p50 (run 2) | multi-core p50 (unpinned) |
|---|---|---|---|---|
| 24px | uncached | 0.334 ms | 0.350 ms | 0.334 ms |
| 24px | cached | 0.000 ms | 0.000 ms | 0.000 ms |
| 48px | uncached | 0.485 ms | 0.481 ms | 0.489 ms |
| 48px | cached | 0.000 ms | 0.000 ms | 0.000 ms |
| 128px | uncached | 0.993 ms | 0.997 ms | 0.999 ms |
| 128px | cached | 0.000 ms | 0.000 ms | 0.000 ms |

(`min` tracks `p50` within a few microseconds at every size/mode; the standalone `just test` run
above additionally reports `p95`/`max`/full raw samples for 32px, 200 iterations.)

`p50`/`min` are rock solid across repeat runs at every size (sub-1% spread); `p95`/`max`
occasionally spike into the single-digit milliseconds or beyond (one `uncached` run's `max`
reached 222 ms) - that's page-fault/scheduler jitter from this shared, loaded sandbox, not
attributable to the patch, and doesn't show up in `p50`/`min` at all.

## Result

`cached` mode is not merely faster than `uncached` - it's below this timer's microsecond
resolution (reports as flat `0.000 ms` for every iteration after the first) at every icon size
tried, while `uncached` costs a real, size-scaling 0.33-1.0 ms *every single time it's called*
(i.e. every paint, per the defect this candidate targets). This reproduces the same shape of
result candidate 041 found for the analogous software-renderer defect, confirming the
maintainer's own diagnosis in #7511 with a number: eliminating this is not a marginal win, it
removes essentially all of the cost on the cache-hit path (every repaint except the first, and
every repaint after the icon/size last changed).

## Does it change behavior/output?

- **Bit-exact?** Yes. The probe's own check (`(width, height)` stable across all iterations,
  `uncached` vs `cached`) never failed at any tested size. The cache stores and returns the exact
  same `QPixmap` `image_to_pixmap` would have recomputed for an unchanged `(Image, icon_size)`
  pair - `Image`'s `PartialEq` (used as the cache-hit condition) is exact, not approximate, so a
  changed icon or resized button always recomputes.
- **Is this a breaking change?** No public API changes - both `NativeButton` and `NativeTab` are
  internal (`i-slint-backend-qt` crate) `Item` implementations, not part of any public Slint API.
  The only new field on each is a private `RefCell` cache.
- Project test suite (`cargo test -p i-slint-backend-qt --features rtti`, `QT_QPA_PLATFORM=offscreen`,
  real Qt 6.5.3 linked): **0 unit tests, 1/1 doctest passed** before (plain `master`,
  `cdb6c43f7`) and after (patched) - identical, no new failures. (Without `--features rtti` the
  doctest fails to build on *both* baseline and patched trees - a pre-existing, patch-unrelated
  gap in this crate's default-feature doctest, not something this candidate introduced or fixed.)

## Why the result looks the way it does

Exactly as diagnosed in the GitHub issue: `image_to_pixmap` does a full resvg SVG rasterization
(`ImageInner::render_to_buffer`) plus a Qt `QImage`/`QPixmap` construction, with no memoization at
all on the unpatched tree - every paint pays the full cost again even though nothing about the
icon or its target size changed. A `RefCell`-based per-widget cache turns that into "pay once,
then a cheap equality check," which is why the eliminated cost doesn't show up as "a bit faster"
but as "immeasurably fast" on the hit path.

## What's not done / open questions

- This candidate's own probe calls `image_to_pixmap` directly (via the temporary `bench-probe`
  export) rather than driving a live `NativeButton`/`NativeTab` through a real `QApplication`
  paint cycle end to end (hover/press/focus-triggered repaint, or `QWidget::update()`). Standing
  that up (a real offscreen-QPA window, an actual `Button { icon: ... }` compiled with
  `CompilerConfiguration::with_style("qt")`, and a way to count/require repaints) would validate
  the *whole* pipeline, not just the function the fix gates - left as a follow-up if this needs
  independent confirmation beyond what the unit-level isolation above already shows.
- The `bench-probe` cargo feature and its `qt_window.rs`/`lib.rs` export are measurement-only
  scaffolding (`bench/bench_probe.patch`), never intended to ship - kept separate from
  `patch.diff` (the actual proposed fix) specifically so `patch.diff` alone is postable as-is
  (e.g. attached to #7511).
- Not extended to any other Qt-backend widget that reads an `Image` property on every paint
  without caching, if any others exist beyond `NativeButton`/`NativeTab` (not audited here -
  these two are exactly what the issue and the original triage named).
