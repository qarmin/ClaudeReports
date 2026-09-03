# COLR_CPAL_PALETTE_INDEX_UNCHECKED_SLICE

`glifo/src/colr.rs`'s `ColrPainter::palette_index_to_color` (lines 130-147, the specific indexing
at 138) indexes `.cpal().ok()?.color_records_array()?.ok()?[palette_index as usize]` with a raw,
unchecked `[]` slice index. `palette_index` is a `u16` forwarded unvalidated straight from a
COLRv1 `PaintSolid`/`PaintVarSolid` paint or a `ColorLine` color-stop record, via `skrifa` 0.44.0's
own `ColorPainter::fill(Brush::Solid { palette_index, .. })` callback - `skrifa` never checks it
against the font's actual CPAL record count before handing it to the callback (confirmed by
reading `skrifa-0.44.0/src/color/traversal.rs`: every `Brush::Solid` construction just forwards
the raw `PaintSolid::palette_index()`/`ColorStop::palette_index` field with no bounds check;
`skrifa`'s own bounds-checked equivalent, `ColorPalettes::get`/`ColorPalette`, exists in
`skrifa-0.44.0/src/color/mod.rs` but is a separate, unused-by-`glifo` public helper). Any
`palette_index` value in `0..=65534` (0xFFFF is already special-cased as "use foreground color")
that is `>=` the font's total `color_records_array` length panics the glyph-rendering path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Source reading of the dependency chain | Read `skrifa` 0.44.0 (pinned in `Cargo.lock`) `src/color/traversal.rs` (all `Brush::Solid` construction sites) and `src/color/mod.rs` (`ColorPalettes`/`ColorPalette`, the bounds-checked helper `glifo` doesn't use) | Confirmed structurally: `palette_index` is forwarded from raw font bytes with no upstream validation against CPAL's record count anywhere in the traversal; `glifo`'s own `color_records_array()?[palette_index as usize]` is a raw slice index with no bounds check of its own |
| Manual boundary-value construction + real, end-to-end, debug-build reproduction | A small `skrifa`-based tool (`repro/tools/font_patch/`) inspected the project's own `examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf` (CPAL: `numPaletteEntries=14`, `numPalettes=3`, so `color_records_array` has exactly 42 entries), located glyph 84's `PaintSolid.paletteIndex` field (originally `4`, in range) and patched it in place to `300` - a real, minimally-malformed COLRv1+CPAL font. Rendered glyph 84 through the real public entry point (`vello_cpu::RenderContext::glyph_run(..).fill_glyphs(..)`, the same path any real caller uses), debug build | Panicked every time: "index out of bounds: the len is 42 but the index is 300" at `glifo/src/colr.rs:132`. Repeated 3/3 runs, debug build, deterministic (no seed/layout dependency - a plain safe-Rust slice index) |
| Boundary-value variant + release build | Same tool, `palette_index` patched to `42` instead (the exact off-by-one boundary: valid indices are `0..=41`), `cargo run --release` | Panicked identically: "index out of bounds: the len is 42 but the index is 42" - confirms this isn't specific to the arbitrary value `300`, and that the panic is a real (always-checked) slice-bounds panic, present in both debug and release builds, not something `debug_assertions` manufactures |

No `miri`/fuzzing was needed past the second approach - a real end-to-end reproduction with a
mechanistically-clear root cause (a plain out-of-bounds slice index on a directly-computed `usize`)
was obtained on the first real attempt and confirmed at a second boundary value; this is a safe-Rust
panic (not memory-unsafety), so `miri`/ASAN would not add signal beyond what the debug/release
comparison already showed.

## Status

**Confirmed.**

Reproduces 3/3 runs (debug build) plus 1/1 at a second boundary value in release build - fully
deterministic (a plain, unconditional slice index, no seed/layout/timing dependency).

**Fully reproducible**: **Yes**. The repro is a single-participant case exempted from needing a
second real participant per `CONFIRMATION.md` section 11's own carve-out: the input is a single
external artifact (a malformed font file) handed to the library's real public entry point
(`vello_cpu::RenderContext::glyph_run(..).fill_glyphs(..)`), the same way a real embedding
application would load an untrusted/attacker-supplied font - there is no second "peer" being
impersonated here, so no second-participant fake is involved at all.

## Class

crash

## Root cause

`glifo::colr::ColrPainter::palette_index_to_color` receives a `palette_index: u16` straight from
`skrifa`'s COLR paint-graph traversal (`ColorPainter::fill(Brush::Solid { palette_index, .. })`,
also reused for gradient color stops via `convert_stops`). `skrifa` 0.44.0's traversal
(`skrifa-0.44.0/src/color/traversal.rs`) reads this value directly from the font's raw
`PaintSolid`/`PaintVarSolid`/`ColorStop` bytes and forwards it to the callback with **no**
validation against the font's actual CPAL table size - unlike the empty-color-stops case
investigated in candidate 038, there is no upstream guard here at all. `glifo` itself then does
`.cpal().ok()?.color_records_array()?.ok()?[palette_index as usize]` - a raw `[]` index with no
`.get()`/length check - so any font whose `PaintSolid`/`PaintVarSolid`/`ColorStop.palette_index`
exceeds `color_records_array().len()` (and isn't the `0xFFFF` "foreground color" sentinel, which
is the only value `palette_index_to_color` already special-cases) makes this line panic. Both
of `palette_index_to_color`'s two call sites (`ColrPainter::fill`'s `Brush::Solid` arm, and
`convert_stops`'s per-stop mapping) already handle `palette_index_to_color` returning `None` via
`.unwrap_or(AlphaColor::BLACK)`, showing the function's own signature was designed to allow a
graceful "couldn't resolve this color" fallback - the raw indexing at line 138 simply doesn't use
it, converting what should be a recoverable `None` into a hard panic instead.

## Introduced by

743efe11 "Add support for rendering bitmap and COLR glyphs (#937)" (2025-05-08) - `git log -S
"color_records_array"` on `glifo/src/colr.rs` (and its pre-workspace-migration ancestor path)
shows this is the commit that first added `palette_index_to_color` with this exact
`.color_records_array()?.ok()?[palette_index as usize]` shape; `git show 743efe11` confirms the
added code is byte-for-byte the same indexing pattern still present today (the function moved
files/structs across later refactors - `5796226b` "Import glifo into the vello workspace",
`46cc30b7` "Migrate vello sparse strips renderers to use glifo", `7848ec74` "glifo: Optimize COLR
glyph rendering", `bbe27c4c` "glifo: Make use of the outline cache for COLR glyphs" - but none of
those touched this specific indexing expression).

## The fix

Changed the raw `[]` index to a bounds-checked `.get(...).copied()?`, mirroring the `?`-propagating
style already used for every other step in the same expression chain (`.cpal().ok()?`,
`.color_records_array()?.ok()?`) - an out-of-range `palette_index` now makes
`palette_index_to_color` return `None` (already handled by both call sites via
`.unwrap_or(AlphaColor::BLACK)`) instead of panicking:

```diff
                 .color_records_array()?
-                .ok()?[palette_index as usize];
+                .ok()?
+                .get(palette_index as usize)
+                .copied()?;
```

Verified against both boundary values used during confirmation (`palette_index = 300` and the
exact off-by-one boundary `palette_index = 42`, CPAL `color_records_array` length 42) - both
render without panicking after the fix (glyph 84 falls back to `AlphaColor::BLACK` for that paint,
same as the existing `.unwrap_or(AlphaColor::BLACK)` fallback already used for other
`palette_index_to_color` failure modes such as a missing CPAL table).

- Project test suite (`cargo test -p glifo`): 22 passed, 0 failed - before *and* after the fix
  (identical result; the fix only changes behavior for the previously-panicking out-of-range case,
  which none of the existing unit tests exercise). Also attempted the broader
  `cargo test -p vello_sparse_tests --test tests` snapshot suite (includes several
  `glyphs_colr_test_glyphs_*`/`glyphs_colr_noto_*` cases): 120 failed identically **before and
  after** the fix, all with `Unsupported(UnsupportedError { format: Unknown, kind: Format(Unknown)
  })` - this is a pre-existing environment issue unrelated to this patch (this checkout's `git-lfs`
  binary isn't installed, so LFS-tracked PNG snapshot assets are checked out as plain-text LFS
  pointer files instead of real images; the `just clone`/`reproduce-*` recipes in this candidate's
  own `justfile` work around it for the standalone repro, but the main project's own local checkout
  used for the before/after comparison was not similarly patched), confirmed identical (same
  failure set, same panic message) with and without the patch applied.

## What's not done / open questions

- The same "unchecked `palette_index` from `skrifa`, unvalidated against CPAL size" input feeds
  into `convert_stops` too (every `ColorStop.palette_index` in a `LinearGradient`/`RadialGradient`/
  `SweepGradient`'s color line goes through the exact same `palette_index_to_color` function) - so
  this one fix in `palette_index_to_color` itself already covers both call sites (`ColrPainter::fill`'s
  `Brush::Solid` arm and `convert_stops`'s per-stop mapping), not just the `PaintSolid` case used to
  build the repro. This was not built into a second, separate repro (e.g. a malformed
  `ColorLine.paletteIndex` inside a gradient) since it's the exact same function and exact same
  fix, verified once.
- `GlyphInfoExtractor`'s `ColorPainter` impl (`glifo/src/colr.rs`, the bbox/blend-flag-only
  traversal used by `get_colr_info`) has an empty `fill(&mut self, _brush: Brush<'_>) {}` and never
  calls `palette_index_to_color` at all, so it was not affected by this bug and wasn't a candidate.
- Only `skrifa` 0.44.0 (the version currently pinned in this repo's `Cargo.lock`) was checked;
  behavior of other `skrifa` versions wasn't investigated, since the question is "does this repo's
  currently-shipped code have this bug", not "did/does `skrifa` ever validate this for any version".
- This wasn't run through a longer fuzzing campaign (e.g. via `fuzz-bug-repro`) - the manual
  boundary-value construction already gave a fully deterministic, mechanistically-understood
  reproduction, so a fuzz harness wasn't judged to add further signal for this specific bug.
