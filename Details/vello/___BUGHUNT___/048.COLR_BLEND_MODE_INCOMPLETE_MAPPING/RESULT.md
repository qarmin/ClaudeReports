# COLR_BLEND_MODE_INCOMPLETE_MAPPING

`vello/src/scene.rs:1031`, `DrawColorGlyphs::push_layer` (the `skrifa::color::ColorPainter` impl
classic vello uses to render COLRv1 emoji glyphs), maps `skrifa::color::CompositeMode` to a
`peniko::Compose` value only, via a `match` that handles the 13 plain Porter-Duff modes explicitly
and routes everything else - all 15 CSS "Mix" blend modes (`Multiply`, `Screen`, `Overlay`,
`Darken`, `Lighten`, `ColorDodge`, `ColorBurn`, `HardLight`, `SoftLight`, `Difference`,
`Exclusion`, `HslHue`, `HslSaturation`, `HslColor`, `HslLuminosity`) - through a `_ =>
Compose::SrcOver` wildcard arm, silently discarding the intended blend function
(`skrifa`'s`ColorPainter::push_layer` fires once per `PaintComposite` COLRv1 paint node). This
looked suspicious because `glifo/src/colr.rs`'s `convert_composite_mode` (used by `vello_cpu`/
`vello_hybrid` for the exact same font format) correctly maps every one of these to the matching
`peniko::Mix` variant, and `clippy::match_same_arms` (pedantic) already flags the wildcard arm as
identical to the explicit `SrcOver` arm - a `// TODO:` marker has sat directly above the wildcard
since the code was first added.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Source reading + comparison against the correct sibling implementation | Read `vello/src/scene.rs`'s `DrawColorGlyphs::push_layer` alongside `glifo/src/colr.rs`'s `convert_composite_mode`, and `skrifa` 0.44.0's `push_layer`/`pop_layer_with_mode` traversal contract (`src/color/traversal.rs`) | Confirmed structurally: `glifo` maps all 28 `CompositeMode` variants (13 Porter-Duff + 15 Mix) to the matching `peniko::BlendMode`; `vello/src/scene.rs` maps only the 13 Porter-Duff variants and silently drops the Mix component for the other 15 |
| `cargo clippy -p vello --all-targets -- -W clippy::match_same_arms` | debug build, base_commit `7516831426fb70f36d62c521c5477f6a7265b36a` | Flags exactly the described spot: "these match arms have identical bodies" between `scene.rs:1036` (`CompositeMode::SrcOver => Compose::SrcOver`) and `scene.rs:1047` (`_ => Compose::SrcOver`) |
| Real, end-to-end reproduction: fontTools introspection of a real COLRv1 test font's `COLR` table to find a real `PaintComposite(mode=Multiply)` glyph, then driving classic vello's real public `Scene::draw_glyphs(..).draw(..)` entry point with it and decoding the real `Encoding` it produced (both `Encoding` and `DrawBeginClip`/`DrawTag` are public API of the separately-published `vello_encoding` crate) | Font: `examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf` (Google's nanoemoji COLRv1 test font, already used by this project's own `vello_sparse_tests` COLR snapshot test), glyph id 143 (`composite_MULTIPLY`), release build, 3 repeat runs | Deterministic 3/3: the real `PaintComposite(mode=Multiply)` layer is encoded with `blend_mode = 0x0003` (`Compose::SrcOver`, `Mix::Normal`) instead of the expected `0x0103` (`Compose::SrcOver`, `Mix::Multiply`) |

No `miri`/ASAN/fuzzing was needed - this is a pure logic bug in a plain, total `match` over a
`#[non_exhaustive]`-free-in-practice finite enum (no `unsafe`, no arithmetic on untrusted sizes,
no crash), so a real font-driven differential check against the correct sibling implementation
(`glifo`) was the right tool and gave a clean, deterministic signal on the first real attempt.

## Status

**Confirmed.**

Reproduces 3/3 runs, release build, fully deterministic (no seed/layout/timing dependency - the
repro just builds a fixed `Scene` from a fixed font glyph and decodes the resulting `Encoding`,
byte-for-byte the same every run).

**Fully reproducible**: **Yes**. This is a single-participant case exempted from needing a second
real participant per `CONFIRMATION.md` section 11's own carve-out: the input is a single external
artifact (a real, unmodified COLRv1 font file already shipped in this repo) handed to the
library's real public entry point (`vello::Scene::draw_glyphs(&font).draw(..)`), exactly the way a
real embedding application renders a color-emoji glyph - there is no second "peer" being
impersonated here.

## Class

logic

## Root cause

`skrifa`'s own COLRv1 paint-graph traversal (`skrifa-0.44.0/src/color/traversal.rs`,
`ResolvedPaint::Composite` arm) implements the CSS-Compositing-flavored `PaintComposite` node by
calling the `ColorPainter` callback's `push_layer` twice per node: first
`push_layer(CompositeMode::SrcOver)` as an isolation wrapper around the whole composite group
(draws the "backdrop" paint inside it), then `push_layer(*mode)` - the paint's *actual* declared
mode - around the "source" paint, before popping both. `vello`'s `DrawColorGlyphs::push_layer`
maps this to a `peniko::Compose` value via a `match` that explicitly handles the 13 Porter-Duff
`CompositeMode` variants (`Clear` through `Plus`) but falls through a bare `_ => Compose::SrcOver`
wildcard for every other variant - which is all 15 CSS "Mix" blend modes. Since `Scene::push_layer`
only receives a `Compose` (via `Compose`'s blanket `Into<BlendMode>`, which fixes `Mix::Normal`),
the intended blend function is thrown away entirely: a font author's `Multiply`/`Screen`/`Darken`/
etc. `PaintComposite` layer renders identically to a plain `SrcOver` layer - i.e., the "source"
paint just opaquely occludes the "backdrop" paint in their geometric overlap, instead of the two
being color-blended per the CSS Compositing and Blending spec. `glifo/src/colr.rs`'s
`convert_composite_mode` (used by `vello_cpu`/`vello_hybrid` for the same COLRv1 format) already
maps every one of these 15 modes to the matching `peniko::Mix` variant, so those two renderers do
not have this bug - only classic `vello`'s own, separate `DrawColorGlyphs` implementation does.

## Introduced by

886604b2 "Render COLR Glyphs (Emoji) (#615)" (2024-06-21) - `git log -S "skrifa::color::CompositeMode::Plus => Compose::Plus" -- vello/src/scene.rs`
shows this is the commit that first added `DrawColorGlyphs::push_layer` with this exact `match`
shape (Porter-Duff arms + `// TODO:` + wildcard `_ => Compose::SrcOver`); `git show 886604b2`
confirms the added code is byte-for-byte the same shape still present at base_commit. This matches
the candidate's own citation of PR #615 as the origin.

## The fix

Changed `DrawColorGlyphs::push_layer`'s `match` to produce a `peniko::BlendMode` instead of a bare
`Compose`: the 13 Porter-Duff arms now each produce `Compose::<X>.into()` (still `Mix::Normal`,
unchanged behavior), and the 15 previously-wildcarded `Mix` modes now each produce
`BlendMode::new(Mix::<X>, Compose::SrcOver)` - the exact same mapping `glifo::colr::
convert_composite_mode` already uses. The final wildcard arm (`_ => Compose::SrcOver.into()`) is
kept for forward compatibility with any future/unrecognized `CompositeMode` variant `skrifa` might
add, which is the only case it can still be reached for (`CompositeMode` is a real Rust enum with
no `#[non_exhaustive]`, but treating the match as if it could still see a new value costs nothing
and avoids a compile break on a future `skrifa` upgrade that adds one). `Mix`/`BlendMode` were
already available from the `peniko` re-export at the top of the file (just needed adding `Mix` to
the existing `use peniko::{..}` import).

Verified by re-running `repro/` against the patched tree: the previously-`0x0003` (`Mix::Normal`)
`BEGIN_CLIP` entry for the `PaintComposite(mode=Multiply)` layer now encodes `0x0103`
(`Mix::Multiply`), 3/3 repeat runs, and the repro's own pass/fail check flips from "REPRODUCED"
(exit 1) to "did not reproduce" (exit 0). The other two `BEGIN_CLIP` entries in the same glyph (the
glyph-level clip-box sentinel and `skrifa`'s own `CompositeMode::SrcOver` isolation wrapper) are
unchanged, confirming the fix only affects the intended layer.

- Project test suite (`cargo test -p vello_tests`, `VELLO_SKIP_LFS_SNAPSHOTS=all` - the LFS
  snapshot PNGs aren't checked out in this sandbox; unrelated to this bug): 64 passed, 0 failed, 1
  ignored (`compare_large_bin_count`, explicitly marked "takes a long time"), identical before and
  after the patch. `cargo test -p vello` itself has 0 unit tests and 1 ignored doc test, unchanged
  either way. No existing test exercises a COLR `PaintComposite` with a "Mix" blend mode at all -
  which is exactly why this has gone unnoticed since PR #615 landed in 2024.

## What's not done / open questions

- Only `Multiply` was exercised end-to-end via the real font (the one glyph the test font happens
  to name `composite_MULTIPLY`); the other 14 `Mix` variants were not separately driven through a
  real font glyph this session, though the fix applies the exact same, already-proven-correct
  mapping (`glifo::colr::convert_composite_mode`'s own arms) to all of them uniformly, so there is
  no reason to expect any of the other 14 to behave differently from `Multiply` - fixed and
  unfixed by the same single `match`, verified for one representative variant.
  `test_glyphs-glyf_colr_1.ttf` does contain a `composite_<MODE>` glyph for every one of the 15
  Mix modes (confirmed via the same `fontTools` introspection used to find glyph 143), so a future
  session wanting to exercise all 15 individually can reuse the same font and technique.
- No pixel-level (rendered-image) comparison was captured for this candidate - the repro instead
  asserts directly on the encoded `blend_mode` word vello's real public API produces, which is a
  more precise, deterministic check of the actual root cause (what blend gets encoded) rather than
  its downstream visual symptom, and was confirmed to work in this sandbox (a real wgpu device
  *is* available here - `cargo test -p vello_tests --test smoke_snapshots` passes both `_gpu` and
  `_cpu` variants) if a future session wants to add a `vello_tests`-style snapshot test for this
  specific glyph.
- No project-level regression test was added for this fix (per the workspace rule that a fix is
  never applied for real by this skill - see `SKILL.md`'s "the one rule that matters most"); a
  maintainer taking this patch may want to add a `vello_tests` snapshot/comparison test rendering
  `test_glyphs-glyf_colr_1.ttf`'s `composite_*` glyphs (mirroring `vello_sparse_tests/tests/
  glyph.rs`'s existing `glyphs_colr_test_glyphs` test for `vello_cpu`/`vello_hybrid`) so this
  exact class of regression is caught by CI going forward.
