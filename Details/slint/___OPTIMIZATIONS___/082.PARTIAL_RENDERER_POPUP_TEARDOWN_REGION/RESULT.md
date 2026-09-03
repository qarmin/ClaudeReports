# PARTIAL_RENDERER_POPUP_TEARDOWN_REGION

`PartialRenderingState::free_graphics_resources` (`internal/core/partial_renderer.rs`)
unconditionally sets `force_screen_refresh` whenever any item tree is destroyed, because - per
its own comment - "we don't have a way to determine the screen region of the delete[d] items".
For a closing `ChildWindow` popup this comment is actually false: `WindowInner::close_popup_impl`
(`internal/core/window.rs`) already computes the popup's exact absolute screen rect and calls
`mark_dirty_region` with it, *before* the popup's `ItemTreeRc` is dropped a few lines later - the
precise region is thrown away the moment the drop reaches `free_graphics_resources`, and a full
window refresh happens anyway. This is GitHub #7432 triaged in `IDEAS.md`, narrowed to the one
teardown path (popup close) where the "small design decision" the issue names already has an
answer sitting right next to it in the code.

Repeater-row and `if`-condition-toggle teardowns have no equivalent precomputed region and are
untouched by this patch - see "What's not done" below.

## What was tried

Standalone bench (`bench/`, headless `MinimalSoftwareWindow`, `RepaintBufferType::ReusedBuffer`,
800x600): 10 000 small rectangles tiling the window plus one 100x100 `ChildWindow` `PopupWindow`,
opened and closed 300 times; timed is the `render()` call for the frame immediately after each
`close()` (~10% warmup discarded).

| Variant | single-core p50 (`taskset -c 3`, 2 runs) | multi-core p50 (unpinned, 2 runs) | close-frame dirty bounding box | notes |
|---|---|---|---|---|
| baseline | 2.951ms, 2.946ms | 2.944ms, 2.952ms | 800x600 (full screen) | `force_screen_refresh` fires on every popup close |
| patched | 1.978ms, 1.974ms | 1.933ms, 1.943ms | 100x100 (popup's own rect) | matches the region already marked by `close_popup_impl` |

At 40 000 rows (`taskset -c 3`, 1 run each): baseline 8.197ms p50, patched 7.214ms p50 - the
per-frame `compute_dirty_regions` tree walk (`O(total items)`, unaffected by this patch, see
`projects/slint.md`) dominates more at this item count, shrinking the *relative* win (~12% vs
~33% at 10 000 rows) while the *absolute* saving stays close to constant (~0.98ms at 10 000 rows,
~0.98ms at 40 000 rows) - consistent with the win being "one less full-screen rasterization pass",
a fixed cost independent of how many other items exist off to the side.

## Result

Real, reproducible, ~33% p50 reduction in the popup-close frame's `render()` time at 10 000
background items (2.95ms -> 1.98ms), both pinned and unpinned, consistent across two repeat runs
each (no overlap between baseline and patched ranges). The win's absolute size (~1ms here) is
governed by how many pixels the popup's own region covers vs. the full window, not by background
item count; the *relative* percentage shrinks as background item count grows because an unrelated,
pre-existing full-tree walk cost that this patch doesn't touch comes to dominate the frame instead.

## Does it change behavior/output?

- **Bit-exact?** Yes. The bench prints an FNV-1a hash of the full pixel buffer after the
  measured frame: `b86ec9620cd7c425` on both the baseline and patched binaries, at both 10 000
  and 40 000 rows. The patch changes only which pixels the renderer decides to touch, never what
  ends up in the buffer.
- **Is this a breaking change?** No public API changes. `close_popup_impl` moved from taking
  `&PopupWindow` to owning `PopupWindow` (private, `WindowInner`-internal); `WindowKind` gained
  `#[derive(Copy, Clone)]` (a plain C-like enum, already `#[repr(C)]` and exposed read-only via
  cbindgen) so the call sites could read `p.window_kind` before moving `p` into
  `close_popup_impl`. Two new default-no-op methods (`mark_teardown_region_known`,
  `clear_teardown_region_known`) were added to the internal (sealed, non-implementable-downstream)
  `RendererSealed` trait.
- Project test suite: `cargo test -p i-slint-core --lib`: **307 passed / 0 failed**, identical
  before and after. `cargo test -p slint --test partial_renderer` (includes
  `partial_rendering_popup_position_size_change`): **24 passed / 0 failed**, identical before and
  after.

## Why the result looks the way it does

`close_popup_impl` already had everything needed - the popup's absolute rect and the moment right
before its `ItemTreeRc` drops - because popup teardown is a single, well-defined choke point
(`WindowInner::close_popup_impl`) with no ambiguity about "where did this subtree used to be".
`free_graphics_resources`'s own comment explaining the full-refresh fallback is really about the
*generic* case (any item tree teardown, called from a `PinnedDrop` impl with no caller-supplied
context) - it just happened to also apply, needlessly, to the one caller that already knew better.
The fix is a one-shot flag (`teardown_region_known`) bracketing the exact `drop(current_popup)`
call, deliberately *not* identified by pointer/tree identity (which would risk an ABA hazard if a
freed address were reused before the flag was consumed) - instead it's cleared defensively right
after the bracketed drop regardless of whether it was actually consumed, so a non-synchronous drop
(refcount not reaching zero exactly there) can never leak the flag into a later, unrelated
teardown and wrongly suppress a full refresh that one actually needs.

## What's not done / open questions

- **Repeater row removal and `if`-condition toggling still force a full-screen refresh.** Neither
  has an equivalent "compute the absolute rect before drop" call site today; building one would
  need to walk that subtree's cached geometry up through its ancestors' cumulative
  transforms *before* the row/branch is detached from the live tree (the general form of the
  problem GitHub #7432 describes) - a materially larger change than this one, left as a follow-up
  in `IDEAS.md`.
- **`TopLevel` popups** (their own native OS window) don't call `mark_teardown_region_known` -
  their destruction tears down a whole separate window/renderer that's not going to be redrawn
  again anyway, so there is nothing to save there.
- Only the software and Skia renderers implement `PartialRenderingState`/this mechanism (matching
  `mark_dirty_region`'s existing pattern); FemtoVG has moved off `PartialRenderer` entirely (see
  `IDEAS.md`'s existing note on candidate 051) and Qt's own `free_graphics_resources` doesn't use
  this cache at all, so both are unaffected either way.
- Not measured on real GPU hardware (FemtoVG/Skia `--frames`) - the bench is software-renderer-only
  headless; the mechanism is renderer-agnostic (`Renderer` trait forwarding) but only the software
  path was exercised end-to-end here.
