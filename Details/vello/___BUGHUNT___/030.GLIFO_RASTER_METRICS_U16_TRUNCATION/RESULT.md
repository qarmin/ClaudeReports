# GLIFO_RASTER_METRICS_U16_TRUNCATION

`glifo/src/renderer.rs`'s `calculate_raster_metrics` (lines 591-614 at the time this candidate was
picked up) computes a glyph's rasterized pixel width/height/bearings as `i32`, then narrows them to
`u16`/`i16` with bare `as` casts, under a blanket
`#[expect(clippy::cast_possible_truncation, reason = "glyph bounds fit in i32/u16/i16 at reasonable
ppem values")]`. `font_size` (`glifo/src/glyph.rs`) is a public, unclamped `f32`, and the only gate
on this code path is `draw_props.font_size <= config.max_cached_font_size` (default 128.0) - this
looked worth checking against a genuinely oversized rasterized glyph bbox, since the atlas allocator
sizes the slot from the (possibly wrapped) metrics while rasterization still targets the real size.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + a temporary diagnostic `eprintln!` in `calculate_raster_metrics` (reverted before the patch) | Debug build, `cargo run` against a hand-built pathological TrueType font (`unitsPerEm = 16`, the OpenType spec's own documented minimum, with a glyph outline near the format's own i16 coordinate limit), driven through `vello_cpu::RenderContext::glyph_run(...).atlas_cache(true).fill_glyphs(...)` (the real, public glyph-rendering entry point) at font_size 128 (exactly the default `max_cached_font_size`, an entirely ordinary size, not "extreme") | Confirmed: `calculate_raster_metrics` computed a real bbox of 65585x65584 physical pixels, wrapped to 49x48 by the `as u16` casts (and `bearing_x`/`bearing_y` also wrapped/flipped sign via the `as i16` cast). The atlas allocated a slot sized to the wrapped 49x48, and the glyph, still recorded/rasterized at its real huge scale, rendered as a fully blank pixmap instead of visible content. |
| Same repro re-run 3x | Same debug build/input | Reproduces deterministically 3/3 (no randomness in the path: fixed font bytes, fixed glyph id, fixed font_size). |
| Same repro with a second, different font (different `unitsPerEm`/outline-coordinate combination, landing on a different point inside the u16 wrap range - real bbox ~67201px, wraps to 1665) | Debug build | Also reproduces on the unpatched tree, and no longer reproduces after the fix - confirms the fix isn't narrowly tailored to the one triggering input (CONFIRMATION.md section 7). |

Never needed a 3rd escalation tool (miri/fuzz) - the defect is a plain, deterministic, boundary-value
arithmetic bug (a lossy `as` cast on values a caller can size), not something needing UB-detection
tooling to observe; manual construction plus a real end-to-end run through the actual public API was
sufficient to both trigger it and see its concrete effect.

## Status

**Confirmed.** Reproduces deterministically (3/3 runs, debug build, no fixed seed needed - the
computation and font/glyph inputs are fully deterministic) via a real, unmodified call through
`vello_cpu`'s actual public glyph-rendering API.

**Fully reproducible**: **Yes.** `repro/` drives the real end-to-end path a genuine application
would use - `vello_cpu::RenderContext::glyph_run(&mut resources, &font).font_size(128.0)
.atlas_cache(true).fill_glyphs(...)`, followed by `RenderContext::render_with(...)` into a real
`Pixmap` - with no internal-API shortcuts, no `GlyphCacheConfig` tampering (the default
`max_cached_font_size` of 128.0 is used untouched), and no atlas/renderer-internals introspection.
The only "unusual" input is the font file itself, which is a well-formed, spec-compliant TrueType
font (`unitsPerEm = 16` is the OpenType specification's own documented minimum; every glyf-table
coordinate/delta used is within the format's i16 range) - i.e. a single external artifact (a font
file) handed to the real public glyph-rendering entry point, the same shape of input a real
application loading an untrusted/unusual font would hand it. There is no second participant to fake
here (see CONFIRMATION.md section 11's own "malformed file handed to a parser's public entry point"
exception) - this is a single-process, single-artifact reproduction of a genuine bug in how the
renderer reacts to a legitimately-producible font.

## Class

logic (silent, deterministic mis-render / content loss caused by an unchecked lossy integer cast;
did not manifest as a memory-safety crash in this session's testing - see "What's not done" below).

## Root cause

`calculate_raster_metrics` computes the glyph's rasterized bounding box in font-agnostic physical
pixels as `i32` (`width = max_x - min_x`, `height = flipped_max_y - flipped_min_y`), then narrows
both to `u16` with a bare `as u16`, and narrows `min_x`/`flipped_min_y` (the bearings) to `i16` with
a bare `as i16`. Unlike a float-to-int `as` cast (which saturates in Rust), an int-to-int `as`
narrowing cast truncates/wraps silently - it does not saturate and does not panic even in a
debug-assertions build (`overflow-checks` only instruments arithmetic operators, not `as` casts).

The physical bbox size is `(font_size / units_per_em) * design_bbox_range`. Nothing in this code
path bounds that product: `font_size` is only checked against `max_cached_font_size` (128.0 by
default) to decide whether caching is attempted at all, and `units_per_em` is read straight from the
font's own `head` table with no lower-bound sanity check beyond what the OpenType spec itself
permits (as low as 16). A font with a small `units_per_em` and a glyph outline using coordinates
near the format's own i16 limit (which every well-formed glyf-table glyph is allowed to use,
completely independent of `units_per_em`) makes the scale factor large enough that the physical bbox
exceeds 65535px even at an entirely ordinary `font_size` like 128 - nothing about this requires an
"extreme" font_size the way the original suspicion assumed.

Once wrapped, the (too-small, and for the bearings, also possibly sign-flipped) `RasterMetrics` is
what `GlyphAtlas::insert_entry` (`glifo/src/atlas/cache.rs`) sizes the actual atlas slot allocation
with, while `render_outline_to_atlas` still positions/records the real, full-size glyph path using
the (also wrapped) bearings to compute the outline transform. In this session's concrete repro, the
combined effect (tiny slot + a `bearing_x`/`bearing_y` that wrapped from a large negative value to a
large *positive* one, i.e. not just truncated but sign-flipped) pushed the recorded glyph path
entirely outside the tiny allocated slot, so the CPU rasterizer's own per-page bounds effectively
already contain the corruption: the visible symptom is the glyph silently vanishing (a fully blank
render) rather than any observed out-of-bounds write.

## Introduced by

46cc30b7 "Migrate vello sparse strips renderers to use glifo (#1562)" (2026-04-14). This commit
introduced the `glifo` crate's `renderer.rs` wholesale (an integration of a previously-separate
`glifo` crate into the sparse-strips renderers); `calculate_raster_metrics` and its `as u16`/`as
i16` casts (plus the accompanying `#[expect(clippy::cast_possible_truncation, ...)]`) are present
verbatim in that commit's diff with no prior history in this repository (`git log -S
"calculate_raster_metrics" -- glifo/src/renderer.rs` returns exactly this one commit).

## The fix

Changed `calculate_raster_metrics`'s signature from `-> RasterMetrics` to `-> Option<RasterMetrics>`,
replacing the four truncating `as` casts (`as u16` x2, `as i16` x2) with `u16::try_from(...).ok()?`
/ `i16::try_from(...).ok()?`, so an out-of-range bbox now yields `None` instead of a silently wrapped
value. The lone call site (`insert_and_render_outline`) treats `None` the same as any other
atlas-capacity failure it already handles (`CacheResult::AtlasFull`), which its own caller
(`fill_glyph`/`stroke_glyph` in the same file) already falls back from to full-precision direct
rendering (`fill_uncached_outline_glyph`) - so an oversized glyph is now drawn correctly (uncached)
instead of corrupted. The remaining `f64 -> i32` casts (`.floor() as i32` / `.ceil() as i32`)
legitimately still need the `#[expect(clippy::cast_possible_truncation, ...)]` (float-to-int casts
saturate rather than wrap, so they were never the actual truncation risk); the attribute's `reason`
was updated to say so explicitly.

Verified:
- Re-ran `repro/` against the patched tree: exit code 0, "did not reproduce: pixmap has nonzero
  coverage, as expected" (was: exit code 1, "REPRODUCED: ... fully blank pixmap").
- Re-ran a second, differently-parameterized font/bbox input in the same class (a different point
  inside the u16 wrap range, real bbox ~67201px wrapping to 1665) against both the unpatched tree
  (reproduces) and the patched tree (does not reproduce) - the fix isn't narrowly tailored to the
  one triggering input.
- Project test suite (workspace-relative paths, run from the actual worktree the patch was applied
  to, not just the repro's own clone):
  - `cargo test -p glifo`: 22 passed / 0 failed, unchanged before and after the patch.
  - `cargo test -p vello_cpu --lib` (+ 1 doctest): 81 passed / 0 failed (+ 1/1 doctest), unchanged
    before and after the patch.
  - `cargo test -p vello_sparse_tests` (the crate's own glyph-rendering snapshot-comparison suite,
    which would be the most directly relevant integration coverage): **N/A in this sandbox** - it
    fails before even reaching any rendering code, in its own reference-image loading helper
    (`sparse_strips/vello_sparse_tests/tests/util.rs:403`, `Unsupported(UnsupportedError { format:
    Unknown, kind: Format(Unknown) })`), because the reference PNGs are Git LFS pointer files and
    `git-lfs` isn't installed in this environment (same warning `git checkout`/`git worktree add`
    themselves print here: "This repository is configured for Git LFS but 'git-lfs' was not found
    on your path"). Spot-checked that this failure is identical (same panic, same location) on the
    unpatched tree, confirming it's a pre-existing environment limitation unrelated to this patch,
    not a regression it introduced.

## What's not done / open questions

- This session's repro demonstrates content loss (a blank/corrupted render), not a memory-safety
  crash - the CPU rasterizer's own per-atlas-page bounds appear to already contain the effect of the
  wrapped/mispositioned geometry in the cases tried. Whether some other combination of wrapped
  width/height/bearing values (a different corner of the u16/i16 wrap space) can additionally cause
  an out-of-bounds write or panic somewhere downstream (the atlas command recorder, the hybrid/GPU
  backend's own handling of `AtlasSlot`, which was not exercised this session - only the CPU
  backend was) was not exhaustively explored; a longer fuzzing campaign (via `fuzz-bug-repro`, run
  separately) driving `calculate_raster_metrics`'s real caller with many bbox/font combinations
  could be worth pointing at this code.
- The hybrid (GPU) backend (`vello_hybrid`) shares the same `glifo::renderer::calculate_raster_metrics`
  and `GlyphAtlas` code, so the same defect is reachable there too via the same kind of font; not
  independently re-verified with a hybrid-specific repro this session (the CPU backend was
  sufficient to demonstrate and fix the shared root cause).
- `vello_sparse_tests`'s own snapshot suite (the most directly relevant integration coverage for
  glyph rendering) could not be run to completion in this sandbox due to a pre-existing,
  patch-unrelated Git LFS tooling gap - re-running it in an environment with `git-lfs` installed
  would be a stronger before/after signal than this session could obtain.
