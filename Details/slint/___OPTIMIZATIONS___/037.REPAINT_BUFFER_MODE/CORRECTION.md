# Correction (from candidate 067, 2026-08-24)

Candidate 067 followed up on this candidate's "C2" backlog lead (bound instead of purge the
`NewBuffer`-mode `clear_cache()`) and found the mechanism description in this file's "What this
does to candidates 035 and 036" section needs a small refinement:

> **Fix 35 barely does anything in the default mode**, and the reason is mechanical: it serves a
> culled item's origin out of the partial-rendering cache, and `NewBuffer` clears that cache every
> frame, so the entry is almost never there and the old path is taken.

Instrumented `PartialRendererCache::clear()` directly: in `NewBuffer` mode the cache is not "almost
never" populated when `clear()` runs - it is **provably always empty** (`slab.len()=0` on every
`clear()` call, setup and per-frame, confirmed on `scroll` 20 000). The reason isn't that the clear
runs *before* fix 35's read - it's that fix 35's read (`PartialRenderer::filter_item`) is never
reached at all in `NewBuffer` mode, because `render_buffer_impl`/`prepare_scene` set
`partial = (repaint_buffer_type != NewBuffer)` and call `render_component_items` with
`&mut renderer.actual_renderer` directly when `partial == false`, bypassing the whole
`PartialRenderer` wrapper (not just its cache) for rendering.

The conclusion this file draws - "fix 35 is close to noise in the mode most applications get by
default" - still holds; only the causal mechanism changes (routing bypass, not a clear-before-read
race). See `067.NEWBUFFER_CACHE_PURGE_DEAD_CODE/RESULT.md` for the full instrumentation and the
new idea this surfaces (routing `NewBuffer` through `PartialRenderer` properly, logged in
`IDEAS.md`).
