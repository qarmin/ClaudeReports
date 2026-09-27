# 115 - Audit other Qt-backend native widgets for candidate 113's uncached per-paint `image_to_pixmap` shape

`IDEAS.md`'s top backlog entry (added while building candidate 113) asked whether any Qt native
widget besides `NativeButton`/`NativeTab` reads an `Image`/pixmap-producing property on every
paint with no cache. This candidate is that audit, scoped to
`internal/backends/qt/qt_widgets/*.rs` (the "native widget" file family the idea named), plus a
check of every `image_to_pixmap` call site anywhere in the Qt backend crate to see whether the
defect exists just outside that literal scope too.

## What was tried

No code change - this is a pure investigation. Method: grep every file in
`internal/backends/qt/qt_widgets/` for the shapes candidate 113 fixed, then read each hit's
surrounding code to classify it.

1. `grep -rn "Image\b" qt_widgets/*.rs` -> only `lineedit.rs` (`clear_icon: Property<Image>`),
   `button.rs` (`icon: Property<Image>`), `tabwidget.rs` (`icon: Property<Image>`), plus two
   unrelated `QImage` (C++ type, not Slint's `Image`) scratch buffers in `scrollview.rs` and
   `tableheadersection.rs`.
2. `grep -rn "image_to_pixmap|QPixmap|QIcon|standardIcon" qt_widgets/*.rs` -> confirms exactly
   three call sites in the whole `qt_widgets/` tree: `button.rs:174` (`NativeButton::actual_icon`)
   and `tabwidget.rs:409`/`tabwidget.rs:541` (`NativeTab`'s `layout_info`/`fn_render!`) - the same
   three call sites candidate 113's `RESULT.md` already names and patches.
3. `grep -rn "render_to_buffer|resvg|decode|to_rgba|SharedPixelBuffer" qt_widgets/*.rs` -> only
   `lineedit.rs`'s `get_clear_icon()`, read in full below.
4. Read `lineedit.rs`'s `NativeLineEdit::clear_icon` end to end: `get_clear_icon()` (which does the
   expensive `QStyleOptionFrame`/`QIcon::paint` rasterization into a fresh `SharedPixelBuffer`) is
   called exactly once, from `Item::init()` (line 111), and the resulting `Image` is stored with
   `self.clear_icon.set(...)`. Nothing in `layout_info()` or `fn_render!` (the actual per-paint
   methods) calls `get_clear_icon()` or reads `clear_icon` at all - grepping the whole file for
   `clear_icon` shows only the field declaration, the import, and this single `init()`-time
   assignment. This is not the same defect: it is already "compute once, reuse forever" by
   construction, with no per-paint re-rasterization to cache against.
5. Read the two `QImage` sites the first grep also caught (`scrollview.rs:427`,
   `tableheadersection.rs:140`): both are `#if defined(Q_OS_MAC)`-gated scratch buffers used to
   work around a macOS style-painting bug (the comment: "The mac style ignores painter
   translations... hence this hack with an intermediate buffer") - a cheap, empty
   `QImage(r.size(), Format_ARGB32_Premultiplied)` is allocated, painted into via the *same*
   `QStyle::drawComplexControl`/`drawControl` call the non-Mac branch already makes directly to the
   real painter, then blitted once. This is not a converted/cached *source* image (there is no
   `Image`/pixmap-producing property being read here at all) - it is a per-frame render target, the
   same shape as any renderer's normal frame buffer, not an instance of candidate 113's defect
   class.
6. Widened the check to every `image_to_pixmap` call site in the whole `i-slint-backend-qt` crate
   (not just `qt_widgets/`), to see whether the defect exists just outside the idea's literal
   scope: `qt_window.rs` has six more call sites. Five are not per-paint-uncached: the window-icon
   site (`qt_window.rs:2421`) explicitly checks an `ImageCacheKey` against a stored
   `window_icon_cache_key` before calling `image_to_pixmap`, only recomputing on an actual change;
   `start_drag`'s two sites (`:2532`, `:2544`) and `set_mouse_cursor` (`:2669`) only run once per
   drag-start/cursor-change event, not once per paint. The sixth, `draw_image_direct`
   (`qt_window.rs:1196`, backing the `Image { source: ... }` element's direct-render path, not a
   native widget), calls `image_to_pixmap` unconditionally on every call with no cache at all -
   unlike the femtovg (`itemrenderer.rs:736-763`, `TextureCacheKey`/`texture_cache.lookup_image_in_
   cache_or_create`) and Skia (`itemrenderer.rs:895-903`, `cached_image::as_skia_image`) renderers'
   own `draw_image_direct`, which both go through a per-source texture cache. This is a real,
   same-shape finding, but it is a general-rendering-path defect, not a "native widget" one - it
   falls outside this candidate's scope (see IDEAS.md addition below) rather than being folded into
   this audit's patch.

No patch was written: the audit's actual scope (`qt_widgets/*.rs`) turned up zero additional
occurrences of candidate 113's defect.

## Result

N/A - no change to measure. The literal audit target (`qt_widgets/*.rs`, Qt native widgets) has no
uncached-per-paint `image_to_pixmap`/pixmap-conversion sites beyond the three candidate 113 already
found and fixed (`NativeButton::actual_icon`, `NativeTab`'s two call sites). `NativeLineEdit`'s
`clear_icon` is an `Image`-typed property that superficially matches the pattern's shape
("`Image`/pixmap-producing property") but does not have the defect (computed once at `init()`, no
per-paint recomputation to cache against).

## Does it change behavior/output?

Not applicable - no source was changed (`patch.diff` is empty).

- **Bit-exact?** N/A, no patch.
- **Is this a breaking change?** N/A, no patch.
- Project test suite: not run. This is a pure investigation with no patch, which SKILL.md's
  "Correctness" section explicitly exempts (same precedent as candidates 105/106).

## Why the result looks the way it does

Candidate 113's own `RESULT.md` already stated it fixed "exactly the two call sites the GitHub
issue and prior triage named" - this audit confirms directly (rather than just trusting the prior
claim) that those two call sites (three counting `NativeTab`'s pair) are in fact the *only*
`image_to_pixmap`/pixmap-conversion call sites anywhere in `qt_widgets/*.rs`. `NativeLineEdit` has
the only other `Image`-typed property in the file family, but Qt's clear-button icon is a
constant, style-driven resource (same icon for the widget's whole lifetime, not something the
`.slint` author sets per-instance the way `Button::icon`/`TabWidget`'s per-tab `icon` are) - it was
correctly implemented as "rasterize once at `init()`" from the start, with no per-paint call site
to have missed caching on in the first place. The two `QImage` scratch-buffer sites are a
completely different pattern (a Mac-only render-target workaround, not a source-image conversion),
included above only because the first broad grep for `Image`/`QImage` caught them and they needed
to be read and ruled out rather than assumed clean.

## What's not done / open questions

- The real finding this audit surfaced - `qt_window.rs`'s `draw_image_direct` (general `Image`
  element rendering, not a native widget) lacking the texture-cache pattern femtovg's and Skia's
  equivalent `draw_image_direct` both have - was intentionally left unfixed here since it's outside
  this candidate's literal "native widget" scope; added to `IDEAS.md`'s backlog as its own item
  (see that file) for a future candidate to build a bench and patch it, the same way candidate 113
  did for the widget-icon case.
- Not independently re-verified whether `NativeButton`/`NativeTab`'s candidate-113 fix (or
  candidate 114's E2E verification) is actually applied anywhere permanent - per this workspace's
  own "the workspace IS the deliverable, main tree always reverted" rule, `patch.diff` files never
  stay applied to the working tree, so `internal/backends/qt/qt_widgets/button.rs`/`tabwidget.rs`
  on this checkout's `HEAD` (`cdb6c43f7`) still have the original uncached code today - confirmed
  directly while grepping for this audit (the exact `image_to_pixmap` call sites candidate 113
  described are still present, unpatched, on `master`).
