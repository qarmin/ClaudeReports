# COLR/COLRv1 differential-testing pass vs an independent reference renderer

Standing lead: candidate 048 (COLRv1 `Mix` blend modes collapsing to `SrcOver`, Confirmed)
suggested there may be more COLR/COLRv1 correctness gaps beyond crash-shaped bugs, and
`projects/vello.md`/the task brief called out that a differential-testing pass against a
reference COLR renderer had not yet been done. This candidate builds and runs that pass: a real,
standalone Rust program (`repro/vello_harness/`) that renders individual glyphs of Google's public
COLRv1 conformance test font (`test_glyphs-glyf_colr_1.ttf`, already vendored in this repo and
used by an existing snapshot test) through vello_cpu's real public `RenderContext::glyph_run` API,
compared side by side (`repro/compare.py`) against `resvg` - an independent COLR implementation
built on `ttf-parser`/`usvg`, a completely different codebase from the `skrifa`/fontations stack
vello uses.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| resvg-vs-vello_cpu differential sweep, ~90 individual glyphs across every named COLRv1 feature category in the test font (transforms: scale/rotate/skew/matrix/translate, ids 84-119; gradient extend modes linear+radial, ids 90-98; clip boxes, ids 156-160; foreground-color substitution, ids 148-155; nested paint-glyph composition, 16 combinations, ids 205-220; cyclic PaintColrGlyph, ids 178-180; circles/misc, ids 168-177; composite modes, ids 120-147; sweep-gradient degenerate cases, ids 181-204), release build, resvg 0.46.0 | One real divergence found (glyph 167, `gradient_p2_skewed`); a second systematic-looking pattern found (sweep `_reflect`/`_repeat` variants of "coincident angle/stops" glyphs rendering blank) turned out to not be usable via resvg at all (resvg has no sweep-gradient support) |
| Manual spec-formula verification (hand-computed the OpenType COLR "rotating and skewing gradients" P0/P1/P2 normalization from the font's own raw `COLR` table data, extracted via `fontTools`) for the one real divergence found | Confirms vello's rendering is spec-correct and resvg's is the one that's wrong (resvg's own source has a `// TODO: We ignore x2, y2` comment for exactly this) |
| Source-level tracing (temporary `std::eprintln!` instrumentation in `glifo/src/colr.rs`, reverted before this write-up) of the blank-rendering sweep pattern that resvg couldn't referee | Traced to `skrifa`'s own COLRv1 traversal choosing not to invoke the `fill()` callback at all when a sweep gradient's `start_angle == end_angle` and `extend != Pad` - already known and explicitly cited in vello's own source (a comment referencing `googlefonts/fontations#1017`), not a new finding |

## Status

**Not reproducible** (no new, previously-undocumented COLR/COLRv1 rendering bug found this
session). This is the first time this differential-testing infrastructure has been built and run
against this project (per the task brief) - see `repro/` for the harness itself, which is
reusable for future sessions to point at other glyph/feature ranges, or extend to compare against
a second/different reference renderer.

Two things that looked like they might be findings, in decreasing order of how close they came:

1. **Glyph 167 (`gradient_p2_skewed`) renders differently between vello and resvg** - but hand-
   verifying the actual spec formula against the font's raw `COLR` table data (see `output.txt`)
   shows vello's diagonal gradient is the mathematically correct result, and resvg's plain
   horizontal gradient is the one that's wrong (resvg's own source admits it ignores the `p2`
   perpendicular/skew point it would need to compute this correctly). Not a vello bug.
2. **All `sweep_coincident_angles_*`/`sweep_coincident_stops_*` glyphs with `Reflect`/`Repeat`
   extend (16 of 24 glyphs in that family) render as fully blank in vello**, while their `Pad`
   siblings render fine. resvg has no sweep-gradient support at all so it can't referee this one;
   source-level tracing instead showed `skrifa` itself (not glifo/vello) silently decides not to
   call the paint-fill callback for this exact case, and vello's own source already has a comment
   citing the relevant upstream fontations issue (`googlefonts/fontations#1017`) predicting exactly
   this. Already-known/tracked, not a new finding - see "What's not done" below for why this is
   still worth a second look by someone who can weigh in on whether "paint nothing" vs. "paint
   like Pad" is the actual spec-correct behavior for a degenerate-length Reflect/Repeat color line.

**Fully reproducible**: N/A (no bug is being claimed as Confirmed this session; the differential
*harness* itself was re-run from a truly fresh clone via `just test` and reproduced identically -
see `output.txt`).

## Class

N/A (not Confirmed).

## Root cause

Not applicable in the "bug root cause" sense - the two candidate divergences found both have a
root cause, but neither is a vello defect:

- Divergence 1 (glyph 167): root cause is in `resvg`/`usvg` (an external tool used only as the
  differential reference), which does not implement `PaintLinearGradient`'s `p2` field.
- Divergence 2 (sweep `_reflect`/`_repeat` blank glyphs): root cause is in `skrifa` (vello's font
  dependency), and is already a known, tracked upstream situation, referenced directly in vello's
  own `glifo/src/colr.rs` source next to the code that specifically works around the `Pad` case of
  the same condition.

Every other category checked (transforms of every kind, gradient extend modes, clip boxes,
foreground-color substitution, nested paint-glyph composition with all 16 identity/translate/
rotate-around-origin/rotate-around-center combinations, cyclic-glyph-reference detection) matched
between vello and the independent resvg implementation, within normal antialiasing/rasterizer
differences between two unrelated CPU rasterizers.

## Introduced by

N/A (not Confirmed - no vello-side defect was found to attribute to a commit).

## The fix

N/A - no bounded fix, because no new vello-side bug was confirmed this session.

- Project test suite: not run (no code change was made to the main project; the only
  instrumentation added during investigation, in `glifo/src/colr.rs`, was reverted - `git status`
  in the investigation worktree showed a clean tree before this folder was written).

## What's not done / open questions

- The sweep `_reflect`/`_repeat` blank-rendering pattern (divergence 2 above) is worth a second
  look by someone who can authoritatively answer "should a Reflect/Repeat sweep gradient with a
  zero-length angular span (`start_angle == end_angle`) paint like its Pad sibling (clamped to the
  nearest stop color, split at the single angle) instead of painting nothing?" If the answer is
  yes, the fix would need to live inside `skrifa` (or `glifo` would need to stop relying on
  `skrifa`'s own traversal deciding whether to call `fill()` at all, which is a much bigger
  change), not a small vello-local patch - out of scope for a bounded fix this session either way.
- Composite/Porter-Duff blend modes (`composite_*`, glyph ids 120-147) were **not** meaningfully
  differential-tested here - resvg's own source admits it cannot represent 11 of the 16 COLRv1
  composite modes (anything other than the CSS-blend-mode-representable ones) and silently falls
  back to normal alpha blending for those, which would produce false-positive "divergences" that
  are really just resvg's limitation, not vello's. Candidates 048 and 079 already found real vello
  bugs in this exact area via other means (code reading + targeted construction, not this
  harness) - a differential pass specifically for composite modes would need a different reference
  renderer that actually implements full Porter-Duty compositing for color fonts (e.g. a real
  Skia/Chromium build, not attempted this session - see `repro/README.md`'s "Known reference-
  renderer limitations" section for the exact list of what resvg can't referee).
- Variable-font (`COLRv1` + `fvar`/`gvar`) COLR rendering was not covered - this test font is a
  static (non-variable) COLRv1 font; a differential pass against a variable COLRv1 test font (also
  publicly available from the same Google Fonts test-glyphs family) would need a reference renderer
  that supports font variations through COLR, which resvg does (it passes
  `face.variation_coordinates()` into `ttf_parser::colr` gradient stop resolution) - this would be
  a reasonable next step for a future session using this same harness.
- This session's resvg version (0.46.0) is not the newest available in this environment (0.47.0/
  0.48.1 crates are cached but weren't tried) - a newer resvg might have gained sweep-gradient
  support, which would let a future session extend this differential harness to the
  `sweep_coincident_*` and `composite_*` families it currently can't referee.
