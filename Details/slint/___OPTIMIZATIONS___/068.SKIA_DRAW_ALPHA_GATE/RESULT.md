# 68 - alpha-gate Skia's individual `draw_*` methods (candidate 054 follow-up)

Follow-up to `054.OPACITY_ZERO_SKIP_RENDER`: that candidate added an ancestor-`Opacity`
early-out to `visit_opacity` on all three renderers, but its own writeup flagged that Skia's
`draw_rectangle`/`draw_border_rectangle`/`draw_image`/`draw_text`/`draw_text_input`/`draw_path`
(`internal/renderers/skia/itemrenderer.rs`) still read rendering-only properties (`background()`,
`fitted_path_events()`, ...) with **no alpha-gate at all** before doing so - unlike the software
renderer's `should_draw()` and femtovg's `global_alpha_transparent()`, which already gate every
one of those same call sites. This candidate adds the equivalent gate to Skia.

## The change

Added `fn global_alpha_transparent(&self) -> bool { self.current_state.alpha == 0.0 }` to
`SkiaItemRenderer`, and an early `if self.global_alpha_transparent() { return; }` to
`draw_rectangle`, `draw_border_rectangle`, `draw_image`, `draw_text`, `draw_text_input` and
`draw_path` - placed identically to where femtovg's `global_alpha_transparent()` check already
sits in each of those methods (i.e. after any cheap geometry-emptiness check, before the first
read of a rendering-only property). `draw_window_background` and `draw_box_shadow` are
deliberately left ungated, matching software/femtovg (the former only registers a dependency and
never draws; the latter already gates on its own `color().alpha()`). See `patch.diff` (23 lines,
6 call sites + the helper).

## A harness fact this candidate had to discover first: most opacity usage never reaches this code

`perf_test`'s only existing opacity knobs (`rects` mode's Rectangle - always gets a synthetic
`BoxShadow` sibling from its (functionally inert but structurally always-present) `drop-shadow-*`
bindings; its `Path` - always gets an injected `Clip` wrapper from its `visible:` binding) both
make `Opacity::need_layer` return `true` unconditionally, so Skia's `visit_opacity` takes the
`save_layer_alpha`/`render_and_blend_layer` path, which resets `current_state.alpha` to `1.0`
while rendering the layer's content - **this patch's gate cannot fire there by construction**,
because content genuinely has to be evaluated to populate the offscreen layer before it gets
blended out. This is real (confirmed with a temporary debug counter: `need_layer=true` at
`--opacity 0` for both existing opacity-capable scenes), and initially made the patch look like a
no-op against the existing harness.

Confirmed by code reading that this is *not* representative of the originally-reported bug
(GitHub #6056, a `Spinner` faded via `opacity: 0`): `compile_paths` (which folds a `Path`'s
`MoveTo`/`LineTo`/`ArcTo` children into its `commands` property) runs *before* opacity-lowering,
so a plain `Spinner { opacity: 0; }` - no `visible:` binding, no drop-shadow - ends up with an
`Opacity` wrapping a childless `Path`, i.e. `need_layer() == false`, taking the direct
`apply_opacity` path this patch actually gates.

**Harness fix**: added `MainWindow.probe-opacity`/`probe-animate`/`probe-count` to
`perf_test/ui/main.slint` - a `for i in root.probe-count: Rectangle { ... }`, absolutely
positioned outside any layout (so it scales to thousands of items with no layout-engine cost of
its own), with no `visible:` binding and no drop-shadow, i.e. structurally identical to the
Spinner case. `background` optionally depends on `animation-tick()` (`probe-animate`), mirroring
the Spinner's `Path.commands` depending on it - reproducing the reported bug pattern (a
content-only property read gated by nothing) on a plain `Rectangle` instead of a widget. Verified
`need_layer() == false` for this element with a temporary debug counter before benching. Also
fixed two unrelated pre-existing gaps while here: `--skia-bench` never wired `--opacity`
(`app.set_rect_opacity`) or `--path` (`app.set_rect_use_path`) for the `rects` scene at all -
those flags were silently no-ops under `--skia-bench` before this candidate; both are now wired
in `skia_headless.rs`. All of this lives permanently in `perf_test/` (untracked in the main repo,
per this workspace's usual model - see `README.md`); `perf_test_fixes/` in this folder has copies
of the four changed files for reproducibility from a fresh clone.

## What was tried

Bench: `perf_test --skia-bench --mode list --count 3 --probe-count 3000 --probe-animate
--probe-opacity <0|1> --frames 20` (headless, real Skia via its own `SoftwareSurface`, no
display/GL needed - see `README.md`'s "Instrumentation added" section on `--skia-bench`).
`--mode list --count 3` just needs *some* valid mode/content; the probe items (outside any mode
page) are what's actually measured.

| Variant | scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) |
|---|---|---|---|
| baseline | `--probe-count 3000 --probe-opacity 0 --probe-animate` | 2.474 ms | 2.458 ms |
| patched | `--probe-count 3000 --probe-opacity 0 --probe-animate` | 1.711 ms (**-30.8%**, 9/9) | 1.711 ms (**-30.4%**, 9/9) |
| baseline (repeat) | same | 2.482 ms | 2.936 ms |
| patched (repeat) | same | 1.674 ms (**-32.6%**, 9/9) | 1.687 ms (**-42.5%**, 9/9) |
| baseline | `--probe-count 3000 --probe-opacity 1 --probe-animate` (sanity: gate never fires) | 7.532 ms | - |
| patched | same | 7.496 ms (-0.5%, 5/9 - noise) | - |

Allocations/frame (deterministic, not timing-noise-prone), `--frames 20`:

| opacity | baseline | patched |
|---|---|---|
| 0 (`--probe-animate`) | 36 408 | 15 800-17 123 (~54-57% fewer) |
| 1 (`--probe-animate`) | 36 408 | 36 408 (identical - gate never fires, as expected) |

`RUNS=9`, `___OPTIMIZATIONS___/tools/paired_skia_headless.sh`, run twice per configuration
(single-core and multi-core) per `MEASUREMENT.md`.

## Result

A **strong, consistent win** (~-30% to -42%, 9/9 paired wins every time, single-core and
multi-core, reproduced twice) for the scenario the bug report actually describes: an
`Opacity`-wrapped leaf item (no layer needed) whose content depends on `animation-tick()`, at
`opacity: 0`. At `opacity: 1` (where the gate never fires) the delta is within noise (-0.5%,
5/9), confirming no regression on the unaffected path. The baseline's allocation count is
*identical* regardless of opacity (36 408 either way) - direct proof the bug is real: before this
patch, Skia evaluates `background()`/`animation-tick()` every frame no matter what the alpha is.
The patched build's allocation count drops by ~54-57% specifically when opacity is 0.

## Does it change behavior/output?

- **Bit-exact?** Yes, outside a small (~100-150 pixel, rows ~835-855 of a 1280x860 frame),
  **pre-existing** non-deterministic region confirmed unrelated to this patch: it reproduces
  identically running the *same unpatched binary* twice (`base-vs-base`, even with
  `--probe-count 0`, i.e. no probe items at all), and still shows up at `--probe-opacity 1` where
  base and patched execute byte-identical code paths. Outside that region, baseline and patched
  PPM snapshots (`--skia-bench --snapshot`, `--mode list --count 3 --probe-count 2900`, opacity
  0 / 0.5 / 1) are pixel-identical. The noisy region's cause was not investigated further (not
  reachable from this patch's own code paths); worth a note in `IDEAS.md` for whoever next needs
  bit-exact Skia snapshots from this harness.
- **Is this a breaking change?** No - invisible content (`opacity: 0`) already produced no
  visible pixels before this patch; this only skips otherwise-unobservable side work (property
  evaluation, allocations) that could never affect what's on screen, and stops
  `animation-tick()`-driven properties behind `opacity: 0` from re-arming the animation driver
  forever (the actual reported symptom: persistent CPU usage with nothing visibly animating).
- `RENDERER=skia cargo build --profile rdebug --bin perf_test --features skia` (from
  `perf_test/`) succeeds cleanly before and after - the only compile check available in this
  environment for the Skia renderer crate (see below).
- `cargo test -p i-slint-core --all-features`: 319+71+1 passed, 0 failed, unaffected either way
  (this crate doesn't depend on `i-slint-renderer-skia`).

## Why the result looks the way it does

Same underlying mechanism as `054`: an alpha-gate placed *before* reading a rendering-only
property stops that property's binding (and anything it depends on, including
`animation-tick()`) from being evaluated at all when nothing will be visible. Software and
femtovg already had this gate for these exact call sites; Skia did not, so - as 054's own writeup
predicted by code reading but couldn't measure (no Skia build available in that session) - Skia
was the one renderer where the reported bug (persistent CPU use from a faded `Spinner`) was fully
unmitigated before this patch. The reason it took real harness work to *show* rather than just
argue this: every existing opacity-capable scene in `perf_test` happens to trigger Skia's
offscreen-layer path (`Opacity::need_layer() == true`) for reasons unrelated to opacity itself
(a structurally-always-present `BoxShadow` sibling; a `Clip` wrapper injected by a `visible:`
binding), and this patch's gate is a no-op inside that path by design (the layer's content must
be evaluated to render into it before the composite decides how much of it becomes visible). The
probe element added here has neither structural quirk, matching the real `Spinner` case, and is
where the fix's effect shows up clearly.

## What's not done / open questions

- **The offscreen-layer path is unfixed and out of scope for this candidate**: any real
  `Opacity`-wrapped subtree with more than one item (the common case for anything but a single
  leaf) still fully renders into a layer at `opacity: 0` on Skia, wasting the same work this
  patch avoids for the leaf case - and, per code reading, still calling
  `animation-tick()`-driven bindings if the layer's content depends on it, re-arming the driver.
  That is squarely `054`'s own `visit_opacity` early-out (`opacity <= 0.0` ->
  `ContinueRenderingWithoutChildren`, skipping the layer entirely), not this candidate's - see
  `054`'s `RESULT.md`. `054`'s patch was never actually applied to the repository (reverted per
  this skill's own rule), so as of this writing the layer path is unmitigated on all three
  renderers, not just Skia.
- **Not tested against a real GPU-backed Skia surface** (only `--skia-bench`'s `SoftwareSurface`,
  no display/GL/Vulkan) - matches this workspace's existing constraint (see `README.md`), not
  new to this candidate.
- **The pre-existing snapshot noise** (see "Does it change behavior/output?") was characterized
  as unrelated but not root-caused; added to `IDEAS.md`.
- `draw_box_shadow` and `draw_window_background` were deliberately left ungated to match
  software/femtovg exactly (see "The change"); a `BoxShadow`'s own `color().alpha() == 0` check
  already covers its equivalent case on femtovg, and Skia's `draw_box_shadow` doesn't check
  `global_alpha_transparent()` for the same reason femtovg doesn't - not verified here that Skia's
  `draw_box_shadow` behaves identically to femtovg's own gate in the ancestor-opacity-zero case;
  worth a dedicated look if `BoxShadow` behind `opacity: 0` turns out to matter in practice.
