# Exact float equality gates font-hinting uniform-scale detection

`vello_encoding/src/resolve.rs` (classic `vello` crate's glyph-run resolution, `Resolver::resolve_patches`)
decides whether to hint a glyph run's transform by testing `transform.matrix[0] == transform.matrix[3]
&& transform.matrix[1] == 0.0 && transform.matrix[2] == 0.0` - `clippy::pedantic`'s `float_cmp` flags
this exact-equality check, and a transform hierarchy that composes to a mathematically pure uniform
scale via matrix multiplication (rather than a literal `Affine::scale(..)` call) can leave the
off-diagonal terms as a tiny nonzero epsilon, silently taking the `else` branch and disabling hinting
for a run that should have been hinted.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static analysis: `cargo clippy -p vello_encoding --all-features -- -W clippy::pedantic` | release build | Confirms `clippy::float_cmp` fires exactly on `vello_encoding/src/resolve.rs:443` (`if transform.matrix[0] == transform.matrix[3]`), suggesting a margin-of-error comparison. |
| Manual boundary-value construction: probed several kurbo `Affine` composition chains (`rotate*scale*rotate`, `rotate*rotate*scale*rotate` with angles summing to zero, 200-iteration forward/back rotation chains, pivoted rotate-scale-rotate) to find one whose matrix, mathematically a pure uniform scale, fails the exact-equality check once cast to `f32` | small standalone probe crate (`kurbo` only) | `Affine::rotate(t1) * Affine::rotate(t2) * Affine::scale(3.0) * Affine::rotate(-(t1+t2))` (angles chosen non-trivially, e.g. `t1=0.1234567`, `t2=0.7654321`) yields matrix `[3.0, 2.220446e-16, -2.220446e-16, 3.0]` after casting to `f32` - a genuine, deterministic FP residual, not contrived by hand-picking a literal epsilon. |
| Real end-to-end repro: `vello::Scene::draw_glyphs(..).transform(composed).hint(true).draw(..)` (real public API an application would use) followed by `vello_encoding::Resolver::resolve(..)` (the exact same call `vello::Renderer` makes internally) and inspecting the packed output's `Layout::transforms(..)` (also public API) | release build, real Roboto-Regular.ttf font, real glyph id for `'A'` | Reproduces deterministically: the glyph's y-translation is left un-rounded (`100.37` instead of snapped to `100.0`), proving hinting was silently disabled end-to-end through the real public entry point. |

## Status

**Confirmed**. Reproduces 5/5 runs, release build, bit-for-bit identical output every time (pure
deterministic floating-point arithmetic - no seed/thread/layout dependence).

**Fully reproducible**: **Yes**. The repro drives `vello::Scene::draw_glyphs(..)` - the real public
API an application (or a text-layout library building a transform via a composed hierarchy) would
call - with a transform built via genuine `kurbo::Affine` matrix composition, then resolves the
scene with `vello_encoding::Resolver::resolve(..)`, the identical call `vello::Renderer` performs
internally when rendering. This is a single-participant finding (one caller's transform, handled by
vello's own resolve step) with no second participant to fake, so `CONFIRMATION.md` section 11's
two-participant bar does not apply here.

## Class

logic

## Root cause

`resolve_patches` in `vello_encoding/src/resolve.rs` (around line 443) treats a glyph run's
transform as "just a uniform scale, safe to hint" only when its 2x2 matrix satisfies **exact**
floating-point equality: `matrix[0] == matrix[3] && matrix[1] == 0.0 && matrix[2] == 0.0`. A
transform built by an application (or a text-layout library) via matrix composition - e.g. a
transform hierarchy like `parent_rotation * content_scale * inverse_parent_rotation`, which is
mathematically exactly a pure scale for any rotation angle - accumulates ordinary floating-point
rounding noise during the actual matrix multiplications. The off-diagonal terms end up as a tiny
nonzero value (observed: `2.220446e-16`, i.e. very close to `f32::EPSILON`) instead of exact `0.0`,
so the exact-equality check fails and the `else` branch (`hint = false`) silently disables hinting
for a transform that is, to any reasonable tolerance, a pure uniform scale. The externally visible
effect: the glyph's y-translation, which would otherwise be rounded to the nearest whole pixel
(`xform.translation[1] = xform.translation[1].round()`, a few lines further down in the same
function) to grid-fit the glyph, is instead left at its raw fractional value - the same class of
"disabled hinting" behavior the code comment says should only happen for a genuinely non-scale
transform (rotation, skew, non-uniform scale).

## Introduced by

2ef58ada "Glyph run API" (2023-02-23) - this commit introduced the glyph-run resolution code in
`vello_encoding/src/resolve.rs` (then still part of the main `vello` crate, moved into its own
`vello_encoding` crate later the same year by db2fefdc), including this exact-equality
uniform-scale check, essentially unchanged since.

## The fix

Replaced the exact-equality check with a small relative tolerance scaled to the magnitude of the
matrix's diagonal terms (`transform.matrix[0].abs().max(transform.matrix[3].abs()).max(1.0) *
f32::EPSILON * 4.0`), and compared each term against that tolerance instead of `== `/`== 0.0`. This
keeps the check tight enough to only absorb genuine floating-point composition noise (a few ULPs),
not real skew/rotation/non-uniform-scale, while no longer rejecting a mathematically-pure uniform
scale that picked up FP noise during composition.

Verified:
- The original repro (`repro/src/main.rs`) no longer reproduces after the patch: the resolved
  glyph transform's matrix is now cancelled to identity and the y-translation is correctly
  hint-rounded to `100.0` (was `100.37`, un-rounded, before the fix). See `output.txt`.
- Checked against more than the one triggering input (per `CONFIRMATION.md` section 7):
  `repro/src/bin/check_no_overbroaden.rs` constructs a genuinely non-uniform-scale transform
  (`scale_non_uniform(2.0, 5.0)`) and a genuinely skewed transform, both composed with the same
  fractional translation - both still correctly have hinting disabled after the fix (y stays
  un-rounded at `100.37`), confirming the tolerance does not over-broaden the detection.
- `cargo clippy -p vello_encoding --all-features -- -W clippy::pedantic`: the `float_cmp` warning
  at this location is gone after the fix (no other warning appeared in its place).

- Project test suite (`cargo test -p vello_encoding -p vello --release`): 31 passed / 0 failed in
  `vello_encoding` both before and after the fix (identical); `vello`'s own lib target has 0 unit
  tests (only an ignored doctest). No new failures, no regressions.

## What's not done / open questions

- The chosen tolerance (`4 * f32::EPSILON` relative to the diagonal magnitude) is a reasonable,
  conservative bound for the specific 1-3-multiplication composition chains tested here, but was
  not exhaustively fuzzed against arbitrarily long/adversarial composition chains; a maintainer may
  want to pick a different constant or a documented rationale for this specific value.
- `vello`'s `DrawGlyphs::glyph_transform` (a second, per-glyph transform composed separately, see
  `resolve.rs`'s transform-stream packing loop) is multiplied together with the run transform
  *after* this uniform-scale check runs, so it is not itself gated by this exact-equality check;
  not investigated further since the finding here is specifically about `run.transform`.
- Did not check whether the newer `sparse_strips`/`vello_cpu`/`vello_hybrid` codepaths (the
  from-scratch CPU/hybrid renderer that coexists with the classic `vello` crate in this repo) have
  an equivalent uniform-scale-detection-for-hinting check with the same exact-equality issue - this
  candidate only covers the classic `vello`/`vello_encoding` crates named in the original suspect.
