# VELLO_COLR_PAINT_CYCLE_UNWRAP_PANIC

This candidate set out to attempt an actual differential-testing pass for COLR/COLRv1 rendering
against a reference renderer (per `projects/vello.md`'s risk map, which flags this as a promising
next step after 048/064 found semantic gaps by manual code reading, and 065 found the project's
own `compare_gpu_cpu` harness is self-comparing and can't be used for this). `vello_cpu`/`glifo`
(already independently validated by this project's own COLR snapshot tests, and the reference used
by 048/064's own confirmed findings) was used as the reference, diffed against classic `vello`'s
real GPU render (via a real `wgpu::Device`, SwiftShader Vulkan ICD) for every one of the 221 glyphs
in Google/nanoemoji's `test_glyphs-glyf_colr_1.ttf` (already shipped in this repo, already used by
this project's own `vello_sparse_tests` COLR snapshot tests). This scan re-found 048's and 064's
already-known bugs (as a sanity check that the harness works) and, partway through, crashed
outright: `vello/src/scene.rs`'s COLR-glyph rendering path (`DrawGlyphs::draw`'s
`EmojiLikeGlyph::Colr` arm) calls `colr.paint(location, &mut DrawColorGlyphs { .. }).unwrap()` -
`colr.paint` is `skrifa::color::ColorGlyph::paint`, which returns `Err(PaintError::
PaintCycleDetected)` for the font's own `paintcolrglyph_cycle_first`/`paintcolrglyph_cycle_second`
test glyphs (glyph ids 178/179) - two glyphs the font's own authors deliberately encoded with a
self-referential `PaintColrGlyph` cycle specifically to exercise this exact edge case. `skrifa`
also has a sibling `PaintError::DepthLimitExceeded` variant for the same call, for a paint graph
that is merely very deep rather than cyclic. `glifo`'s own `ColorPainter` implementation (used by
`vello_cpu`/`vello_hybrid` for the same COLRv1 format) already handles this: `ColrPainter::paint`
ignores the `Result` (`let _ = skrifa_glyph.paint(location_ref, self);`) and explicitly drains any
still-open clip/blend layers afterwards - but classic `vello`'s own, separate implementation
unwraps unconditionally, so this real, spec-sanctioned failure signal panics the entire render call
for any font containing a legitimately-encoded COLRv1 paint cycle (or merely a very deep paint
graph).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Set up a real full-font differential-rendering harness: render every glyph (0..221) of `test_glyphs-glyf_colr_1.ttf` through both `vello_cpu` (reference, via `glifo`) and classic `vello` (real `wgpu::Device` via SwiftShader's software Vulkan ICD, `VK_ICD_FILENAMES=/tmp/swiftshader_icd/vk_swiftshader_icd.json`), comparing per-pixel straight (unpremultiplied) RGBA8 | Debug-assertions-off release build, `CANVAS=FONT_SIZE=1000` (the font's own `unitsPerEm`), `DIFF_THRESHOLD=10`/255 per channel | First pass (before correcting a repro bug below) re-found 048 (Mix blend modes, glyphs 133-147) and 064 (sweep-gradient Y-flip, glyphs 12-83, 152-153) as expected sanity checks, but also showed large, spurious-looking divergences on several *other* glyph families (foreground-color-with-alpha, rotate/skew/scale/transform_matrix, translate) |
| Investigated the "new" divergences by sampling individual pixels and tracing through `peniko`/`color`'s premultiplication math | `SAMPLE=1` env var added to the scan harness to print raw RGBA at fixed sample points for specific glyph ids | **Self-correction, not a real bug**: `vello_common::pixmap::Pixmap::sample()`/`take()` return *premultiplied* RGBA8 (per the type's own doc comment), while classic vello's real GPU output (`fine.wgsl`'s final `textureStore(output, ..., vec4(fg.rgb * a_inv, fg.a))`) is *straight* (unpremultiplied) alpha. The scan's reference-image builder was comparing `vello_cpu`'s premultiplied pixels directly against classic vello's straight pixels - a repro-harness bug, not a renderer divergence. Switched to `Pixmap::take_unpremultiplied()`; after the fix, the foreground-color-alpha/rotate/skew/scale/transform_matrix/translate glyphs all dropped to `n_diff` in the low hundreds out of 1,000,000 (max_diff up to 255 but on a tiny fraction of pixels - consistent with ordinary single-pixel edge/AA differences between two independent rasterizers, the same order of magnitude 064's own fixed-state baseline (632/40,000 = 1.6%) called "ordinary AA noise") |
| Re-ran the corrected full-font scan | Same config, corrected harness | Confirmed 048/064 are still the only *large-magnitude, systemic* divergences (n_diff in the hundreds of thousands out of 1,000,000) in categories unrelated to what this candidate found below; the scan then panicked outright partway through, at glyph 178 (`paintcolrglyph_cycle_first`), before reaching the rest of the font |
| Root-caused the panic, then built a minimal, focused, real end-to-end repro (no GPU/wgpu needed at all: the panic is CPU-side, inside `Scene::draw_glyphs(&font).draw(..)`'s own scene-construction call) calling classic vello's real public API on the font's own `paintcolrglyph_cycle_first`/`paintcolrglyph_cycle_second` glyphs, plus a negative control (`no_cycle_multi_colrglyph`, glyph 180, which nests `PaintColrGlyph` without a cycle) | Release build, 3 repeat runs per cycle glyph (6 total) + 1 negative-control run, real font already shipped in this repo | Deterministic 6/6 panics (`called Result::unwrap() on an Err value: PaintCycleDetected`, `vello/src/scene.rs:879:22`) for the two cycle glyphs; the negative control renders without panicking both before and after the fix, confirming the panic is specific to the cycle condition, not `PaintColrGlyph` nesting in general |

No `miri`/ASAN/fuzzing was needed - this is a plain `unwrap()` on a `Result` a well-behaved, real
font can legitimately produce (no `unsafe`, no arithmetic on untrusted sizes), so a real
font-driven differential/exhaustive-glyph scan against the correct sibling implementation
(`vello_cpu`/`glifo`) was the right tool, and surfaced the crash organically as a side effect of
exercising every glyph in the test font rather than requiring it to be hypothesized in advance.

## Status

**Confirmed.**

Reproduces 6/6 runs (2 distinct cycle glyphs x 3 repeat runs each), release build, fully
deterministic (no seed/layout/timing dependency - the repro just builds a fixed `Scene` from a
fixed font glyph, real font content, real public API, every run panics at the identical line).

**Fully reproducible**: **Yes**. This is a single-participant case exempted from needing a second
real participant per `CONFIRMATION.md` section 11's own carve-out: the input is a single external
artifact (a real, unmodified COLRv1 font file already shipped in this repo, containing a glyph its
own authors deliberately crafted with a self-referential `PaintColrGlyph` cycle) handed to the
library's real public entry point (`vello::Scene::draw_glyphs(&font).draw(..)`), exactly the way a
real embedding application renders a color-emoji glyph - there is no second "peer" being
impersonated here. No `wgpu`/GPU device is involved at all: the panic occurs while building the
`Scene` (pure CPU-side scene-graph construction), before any GPU rendering would occur.

## Class

crash

## Root cause

`vello::scene::DrawGlyphs::draw`'s `EmojiLikeGlyph::Colr` arm calls `colr.paint(location, &mut
DrawColorGlyphs { .. }).unwrap()`, where `colr` is a `skrifa::color::ColorGlyph` and `.paint(..)`
drives `skrifa`'s own COLRv1 paint-graph traversal, invoking the supplied `ColorPainter` callback
(`DrawColorGlyphs`, vello's `Scene`-backed implementation) for each paint node. `skrifa`'s
traversal returns `Err(PaintError::PaintCycleDetected)` when it detects that a `PaintColrGlyph`
node (COLRv1's "reference another color glyph by id" construct) forms a cycle back to a glyph
already being painted higher up the same paint-graph traversal - this is spec-mandated protection
against infinite recursion, not a sign of a corrupt/unparseable font; `skrifa` also has a sibling
`PaintError::DepthLimitExceeded` for the same call site, for a paint graph that is merely very deep
rather than cyclic. `vello`'s own sibling implementation for the identical COLRv1 format
(`glifo::colr::ColrPainter`, used by `vello_cpu`/`vello_hybrid`) already handles this class of
failure: its own `paint()` method ignores the `Result` entirely (`let _ =
skrifa_glyph.paint(location_ref, self);`) and then explicitly drains any clip/blend layers still
left open on its internal stack (with an accompanying comment: "In certain malformed fonts (i.e.
if there is a cycle), skrifa will not ensure that the push/pop count is the same, so we pop the
remaining ones here"). Classic `vello`'s `DrawColorGlyphs` had no equivalent handling: it
unconditionally unwraps the `Result`, so any font that legitimately triggers either `PaintError`
variant (a cycle, or hitting the depth limit) panics the entire `Scene::draw_glyphs(..).draw(..)`
call - a real, publicly reachable crash from a real, well-formed (if adversarially- or
deliberately-crafted) font, not just from a corrupt one.

## Introduced by

886604b2 "Render COLR Glyphs (Emoji) (#615)" (2024-06-21) - `git show 886604b2 -- vello/src/scene.rs`
shows the original COLR-rendering code this PR added already had this exact
`.paint(location, &mut DrawColorGlyphs { .. }).unwrap()` shape (the call was written against a
differently-named local variable/collection accessor at the time, later refactored). `git blame`
on the *current* line attributes it to 3c8dc792 "Support bitmap fonts (#641)" (2024-08-21), which
restructured the surrounding glyph-dispatch code (introducing the `EmojiLikeGlyph` enum) but kept
the same unconditional `.unwrap()` - `git log -S "colr.paint("` on the file matches 3c8dc792
because that's the commit that introduced this exact literal call-site text, not because it
introduced the underlying unwrap-on-Result pattern, which has been present, unbroken, since the
COLR-rendering feature was first added in 886604b2.

## The fix

Changed the call site to ignore the `Result` (mirroring `glifo`'s own handling of the identical
condition) instead of unwrapping it, and added an `open_layers: u32` counter to `DrawColorGlyphs`
(incremented in `push_clip_glyph`/`push_clip_box`/`push_layer`, decremented in `pop_clip`/
`pop_layer`) so that, after `colr.paint(..)` returns (successfully or with an error), any layers
still left open on the target `Scene` (from a paint graph that errored out partway through, after
already having pushed some clips/blend layers without matching pops) are drained with
`scene.pop_layer()` before returning - keeping the `Scene`'s own layer stack balanced for whatever
is drawn next in the same scene, the same invariant `glifo`'s own stack-drain loop preserves.
`clip_depth` (pre-existing, used only to know when to reset `clip_box`) was left as-is and
`open_layers` added alongside it, since `clip_depth` only counts clip pushes and `Scene::pop_layer`
is generic over both clip and blend layers (both go through the same `Encoding::encode_begin_clip`/
`encode_end_clip` pair internally).

Verified by re-running `repro/` against the patched tree: 0/3 panics for each of the two cycle
glyphs (previously 3/3 each), and the negative-control glyph (`no_cycle_multi_colrglyph`, which
uses `PaintColrGlyph` nesting without a cycle) continues to render without panicking both before
and after the fix - confirming the fix doesn't just suppress the panic for this specific input but
correctly distinguishes "paint graph errored out" from "paint graph nested normally." Separately,
re-running the full 0..221-glyph differential scan against the patched tree completes without any
panic (versus crashing at glyph 178 before), and no other glyph's diff-count against the
`vello_cpu` reference changed relative to the pre-fix scan - i.e. draining the layer stack after a
cycle-glyph error does not disturb rendering of subsequent glyphs drawn later in the same `Scene`.

- Project test suite (`cargo test -p vello_tests --release -- --include-ignored`,
  `VELLO_SKIP_LFS_SNAPSHOTS=all` - the LFS snapshot PNGs aren't checked out in this sandbox;
  unrelated to this bug): **65 passed, 0 failed before -> 65 passed, 0 failed after** (identical
  per-file breakdown, no test's outcome changed). `cargo test -p vello --release`: 0 unit tests, 1
  ignored doc test, unchanged either way. No existing test in this project's suite currently
  exercises a COLRv1 `PaintColrGlyph` cycle through classic vello's renderer at all - which is
  exactly why this has gone unnoticed since the COLR-rendering feature was first added in 2024.

## What's not done / open questions

- Only `PaintCycleDetected` was exercised end-to-end via a real font (the test font's own two
  `paintcolrglyph_cycle_*` glyphs); `PaintError::DepthLimitExceeded` (the sibling variant returned
  by the exact same `.paint(..)` call for a paint graph that is very deep but not cyclic) was not
  separately triggered with a real font this session, though the fix handles both identically
  (any `Err(_)` variant, not matched specifically on `PaintCycleDetected`), since both represent
  the same class of "traversal stopped early, safely" signal from `skrifa`.
- The broader differential-testing pass this candidate ran (comparing every glyph of
  `test_glyphs-glyf_colr_1.ttf` between `vello_cpu`/`glifo` and classic vello) re-confirmed 048 and
  064 are still present and unfixed in this checkout (expected, since this workspace's own fixes
  are never applied for real to the main project - see `SKILL.md`'s "the one rule that matters
  most") and did not surface any *other* systemic (non-AA-noise-level) divergence beyond those two
  plus this candidate's crash, across all 221 glyphs - i.e. this specific font's coverage of
  gradients/extends/transforms/composites/clips/palettes/cycles is, modulo 048/064/this candidate,
  consistent between the two renderers. This is not a proof that classic vello's COLR path has no
  further bugs beyond these three - only that this one test font's ~221 glyphs didn't surface any
  more within this session's time budget.
- No project-level regression test was added for this fix (per the workspace rule that a fix is
  never applied for real by this skill - see `SKILL.md`'s "the one rule that matters most"); a
  maintainer taking this patch may want to add a `vello_tests` test rendering
  `test_glyphs-glyf_colr_1.ttf`'s `paintcolrglyph_cycle_first`/`paintcolrglyph_cycle_second` glyphs
  through classic vello (asserting it does not panic) so this exact class of regression is caught
  by CI going forward.
- The premultiplied-vs-straight-alpha repro artifact this session hit and corrected (see "What was
  tried" above) is worth recording as a general gotcha for any future differential test comparing
  `vello_cpu`'s `Pixmap` output against classic vello's `wgpu` texture readback: always use
  `Pixmap::take_unpremultiplied()` (or equivalently un-premultiply `Pixmap::sample()`'s output),
  never compare `Pixmap::sample()`'s raw (premultiplied) bytes directly against a `wgpu` texture
  read back from a `Renderer::render_to_texture` call (straight/unpremultiplied) for any scene
  containing partial alpha - candidate 064's own repro happened not to hit this because its
  specific test glyph's gradient stops were all fully opaque.
