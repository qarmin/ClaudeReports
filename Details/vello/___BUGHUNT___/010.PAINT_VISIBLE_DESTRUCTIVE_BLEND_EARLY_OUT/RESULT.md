# vello_hybrid Scene's paint_visible early-out vs destructive blend modes

`sparse_strips/vello_hybrid/src/scene.rs`'s `Scene::set_paint_visible` sets `paint_visible = false`
whenever the current paint is a solid color with alpha `0.0`, and `fill_path`/`stroke_path`/
`fill_rect`/`fill_blurred_rounded_rect` all return immediately (recording zero strips) when
`!paint_visible`. Issue #1408 (source for this candidate) suspected this is incorrect for
"destructive" blend modes (`Compose::Clear`/`Compose::Copy` and similar), where a fully-transparent
source is still supposed to affect the destination (e.g. `Clear` should erase the destination
wherever the source shape covers, regardless of the source's own color/alpha) - silently dropping
required compositing work whenever the skipped draw's coverage would otherwise have mattered.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `vello_common::record::CommandRecorder` (bbox tracking for recorded layers) and `vello_hybrid::schedule::mod::schedule_child_layer`/`compose_layer` (how a popped layer's blend/compose is actually scheduled against its parent) | Static, no build | `schedule_child_layer` explicitly special-cases this exact scenario: `if layer.bbox.is_empty() { if layer.props.blend_mode.is_destructive() { /* use parent_bounds instead of returning early */ } else { return Ok(None); } }`, with the comment "Unlike in the non-destructive case, empty *destructive* layers are not a no-op, and we need to clear the whole parent region." `compose_layer` similarly picks `parent_region.layer_bbox` (not the child's own, possibly-empty, bbox) as the affected blend region whenever `blend_mode.is_destructive()`. Since a `fill_rect` skipped via `paint_visible` records zero strips - the same recorder-level effect as never calling it at all (confirmed via `record.rs`'s own `empty_draws_do_not_affect_layer_bounds` unit test) - this is exactly the case the destructive-mode special-casing above was written to handle. |
| Existing project test suite, pristine main checkout, release build | `cargo test -p vello_hybrid --release --lib` (unit tests, includes `schedule::schedule_tests::destructive_empty`, which builds exactly a `Compose::Clear` child layer with zero draws inside a non-empty parent and asserts a real blend op still gets scheduled) | **122 passed; 0 failed.** `cargo test -p vello_sparse_tests --release compose` (real GPU-rendered snapshot tests against checked-in reference PNGs, both `vello_cpu` and `vello_hybrid` backends): **258 passed; 0 failed; 72 ignored** (ignored = `neon` variants on this non-ARM machine). In particular `compose_clear_empty_layer_hybrid` and `compose_clear_empty_nested_layer_hybrid` (a `Compose::Clear` child layer with **literally zero draws inside it**, nested one and two levels deep inside a parent that already has visible content) both pass, directly demonstrating that an empty/invisible destructive-mode layer already correctly erases the parent's prior content in the real renderer. |
| Standalone, real end-to-end repro against a fresh pristine clone (own `Cargo.toml`, `vello_hybrid`'s real public `Scene`/`Renderer` API, a real `wgpu` device/adapter, actual GPU render + texture readback - the same code path `examples/render_to_file.rs` uses), release build | For each of the 6 `Compose` variants `peniko::BlendMode::is_destructive()` flags (`Clear`, `Copy`, `SrcIn`, `DestIn`, `SrcOut`, `DestAtop`): render a scene with an opaque yellow backdrop rect, then `push_blend_layer` with that destructive compose mode (bypassing `Scene::set_blend_mode`'s panic-on-destructive guard, the only public way to reach a destructive mode at all) and `fill_rect` a second rect with either an opaque `BLUE` paint (reference) or a fully-transparent paint (`TRANSPARENT`, alpha `0.0` - triggers the `paint_visible` early-out). Sampled the pixel at (20, 20), which lies under the yellow backdrop but **outside** the second rect's own footprint, so it only shows a difference from the backdrop if the destructive-blend layer's effect propagated beyond just the drawn shape. Ran 3 times total (see `output.txt` for one full run; 2 additional reruns produced byte-identical `OK`/`RESULT: NO BUG` output). | **No bug reproduced, 3/3 runs.** For all 6 destructive compose modes, the transparent-source layer erased the yellow backdrop at (20, 20) exactly like the opaque-source reference did (`[0, 0, 0, 0]` in both cases - the scene has no opaque background fill, so "erased" reads as fully transparent rather than white). `Compose::Copy`/`SrcOut`/`DestAtop` additionally show the expected *difference* at the inner rect's own footprint (80, 80): opaque source paints blue there, transparent source correctly reads back as transparent there too (since `Copy`-family composes are supposed to replace that region with the - transparent - source color, not just "no-op" it). |

## Status

**Not reproducible.**

Ran 3/3 times against a byte-identical, deterministic result (no randomness/seed involved - pure
GPU rasterization of axis-aligned rectangles). The renderer already special-cases exactly this
"draw call skipped -> layer's own recorded bbox is empty -> layer has a destructive blend mode"
situation at the scheduling stage (`vello_hybrid::schedule::mod::schedule_child_layer`), which is
covered by both a unit test (`destructive_empty`) and real GPU-rendered snapshot tests
(`compose_clear_empty_layer_hybrid`, `compose_clear_empty_nested_layer_hybrid`) that predate this
candidate and already exercise the "zero draws inside a destructive-mode layer" case the
`paint_visible` early-out degenerates into.

**Fully reproducible**: N/A (no bug to reproduce - see Status). The investigation itself is fully
reproducible on demand via `just test` (verified against a genuinely pristine fresh clone of the
base commit, not the main checkout used for exploration), 1/1 clone-and-run via `just test` plus 2
additional direct `cargo run --release` reruns against the same clone, 3/3 total, byte-identical
output each time.

## Class

N/A (not Confirmed).

## Root cause

Not a bug in the current codebase. The underlying concern (a transparent-alpha draw silently
skipped, even though a destructive blend mode needs it to still "count" for erasing/replacing the
destination) is a real and reasonable thing to worry about for a sparse-strip renderer that
otherwise aggressively skips work for invisible content - but `vello_hybrid`'s scheduler was
already built to handle it: `CommandRecorder::pop_layer`/`record_bbox` compute a layer's bbox
purely from what was actually drawn into it (an empty/invisible draw contributes nothing, per
`record.rs`'s own `empty_draws_do_not_affect_layer_bounds` test), so a `paint_visible`-skipped
`fill_rect` does leave the enclosing layer's bbox empty exactly as suspected. But
`schedule_child_layer` (in `sparse_strips/vello_hybrid/src/schedule/mod.rs`) explicitly checks
`layer.props.blend_mode.is_destructive()` for exactly this case: if the bbox is empty and the
blend mode is destructive, it substitutes the *parent's* bounds instead of treating the layer as a
no-op, and `compose_layer` picks the (larger) parent bbox rather than the (possibly empty) child
bbox as the affected blend region whenever the blend mode is destructive. The two are wired
together correctly, so an invisible-paint draw inside a `Compose::Clear`/`Copy`/etc. layer still
triggers a real blend operation over the correct region at render time, matching what an opaque
draw covering the same shape would have triggered. The only way to reach a destructive blend mode
at all through vello_hybrid's public `Scene` API is via `push_layer`/`push_blend_layer` (since
`Scene::set_blend_mode` explicitly panics on `is_destructive()` blend modes) - i.e. the caller must
already be deliberately using the lower-level layer API that this scheduling logic is written for.

## Introduced by

Not determined - not worth a full `git blame`/bisect this session given the Status is Not
reproducible (per `SKILL.md` step 5, this lookup is optional and skipped once a candidate resolves
to a non-bug). For context: `git log -S is_destructive -- sparse_strips/vello_hybrid/src/schedule/mod.rs`
shows the `is_destructive()` special-casing in `schedule_child_layer`/`compose_layer` has been
present since this scheduling module was introduced, i.e. this was designed in up front rather than
patched in later as a fix for an observed bug.

## The fix

No fix attempted or needed - Status is Not reproducible (no defect exists to fix in the current
codebase).

- Project test suite: N/A, no fix was applied. Baseline (`cargo test -p vello_hybrid --release
  --lib`, pristine main checkout, no changes): **122 passed; 0 failed.**
  (`cargo test -p vello_sparse_tests --release compose`, same checkout: **258 passed; 0 failed; 72
  ignored**.)

## What's not done / open questions

- This candidate only checked `vello_hybrid` (the crate the candidate description names -
  `vello_cpu`'s `Scene`/`RenderContext` has no `paint_visible`-style early-out at all, grep-
  confirmed, so this specific concern doesn't apply there).
- Only checked solid-color paints going fully transparent (`alpha == 0.0`, the literal condition
  `set_paint_visible` checks). Did not check whether a *gradient* or *image* paint that is
  effectively fully transparent everywhere (e.g. a gradient with all-transparent stops) has any
  equivalent early-out anywhere else in the pipeline that might not be covered by the same
  destructive-blend-mode handling - `set_paint_visible` itself only ever sets `paint_visible =
  false` for the `PaintType::Solid` case (`_ => true` for gradients/images), so this looks
  structurally out of scope for the same bug class, but wasn't independently verified with a
  concrete repro.
- Did not attempt `miri`/ASAN/a fuzz harness - the suspected failure mode was a deterministic logic
  question ("does a specific code path get exercised for a specific input"), not a memory-safety
  or crash-shaped one, so a real end-to-end pixel-comparison repro (the third approach above) is
  the correct tool here, not a sanitizer.
