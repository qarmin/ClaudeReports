# UNPREMULTIPLY_WRAPPING_MUL_OVERFLOW

`vello_common::util::unpremultiply::scalar`/`simd` (`sparse_strips/vello_common/src/util.rs:450`
and `:454`, introduced by 1a6d0449 "vello_common: Optimize pixmap unpremultiplication (#1839)",
2026-08-26) compute `u16::from(component).wrapping_mul(reciprocal).div_256() as u8`, which is only
sound if `component <= alpha` holds for every premultiplied pixel it is ever called on - reached
from the public `Pixmap::take_unpremultiplied` (`pixmap.rs:402`, used by PNG/image export of any
scene with `may_have_transparency`). The suspicion was that the real f32 rendering pipeline
(`vello_cpu::fine::highp::F32Kernel::pack`, `fine/highp/mod.rs:270-282`, which rounds r/g/b/a to u8
independently via `(src[c] * 255.0 + 0.5) as u8` with no post-hoc clamp) could produce exactly such
a violating pixel, since a sibling PR's (#1834) own commit message documents rounding deltas up to
1 occurring during blending elsewhere in the pixmap-construction code.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction/reimplementation of the reciprocal + `div_256` arithmetic (not calling the crate's `pub(crate)` module directly - re-derived the identical formula from source) over all `(alpha, component)` pairs with `component > alpha` | Standalone `rustc -O` script, exhaustive over all 256x256 `(alpha, component)` pairs | Confirms the arithmetic is unsound in principle: e.g. `alpha=152, component=153` (delta of only 1, the exact class the PR admits happens) yields `scalar() == 0` instead of the correct near-255 (white) result - a near-white channel silently collapses to black. Also found that `Div256Ext::div_256`'s own `self + 128` (a plain `+`, not `wrapping_add`) can itself overflow `u16` before the final cast whenever `component` is far enough above `alpha` (e.g. `alpha=9, component=253`), which would panic under `debug-assertions` if ever actually reached with such an input, on top of the silent-truncation failure mode for smaller deltas. This step only shows what happens *if* the precondition is violated - it does not show the precondition is ever actually violated by real rendering. |
| Real end-to-end fuzz-style search via the actual public API: `vello_cpu::RenderContext::new` + random solid-color `fill_rect` (fractional/anti-aliased coverage), random `Mix`/`Compose` `BlendMode`s (all 16 `Mix` and all 12 `Compose` variants), random `push_layer`/`pop_layer` nesting with random opacity, alternating `RenderMode::OptimizeQuality`/`OptimizeSpeed`, then scanning the resulting `Pixmap::data()` (raw premultiplied buffer) for any pixel with `r>a \|\| g>a \|\| b>a`, on an 8x8 canvas | Release build, 2,000,000 randomized scenes (128,000,000 pixel checks total), fixed PRNG seed, real `vello_cpu`/`vello_common` public API only (no hand-constructed `Pixmap`/internal state) | **0 violations** in 2,000,000 renders. See `output.txt`/`repro/`. |
| Same real end-to-end search, debug-assertions build (`cargo build`, no `--release`), 500,000 iterations | Debug build (arithmetic-overflow checks on) | **0 violations, 0 panics** anywhere in the real render pipeline (including no panic from the overflow-prone `+` noted above, since it was never actually reached with a violating input) |

All repro code uses only the project's real public API (`vello_cpu::RenderContext`,
`vello_common::pixmap::Pixmap`) - no internal/private state was fabricated.

## Status

**Not reproducible.** Confirmed via two escalating real end-to-end searches (2,000,000 release-mode
renders + 500,000 debug-assertions renders, 2,500,000 total, ~160,000,000 pixel checks) across a
wide space of blend modes, layer nesting/opacity, and anti-aliased fractional coverage on both
render pipelines (`OptimizeQuality`/f32 and `OptimizeSpeed`/u8): the real rendering pipeline never
produced a premultiplied pixel violating `component <= alpha` in this search. This matches the
existing test coverage in `util.rs` itself (`scalar_exhaustive`/`simd_exhaustive`, `util.rs:483-513`),
which deliberately only exercises `component in 0..=alpha` - i.e. the precondition is treated as an
invariant maintained by every producer, not something defensively checked at the consumer.

**Fully reproducible**: N/A (not applicable - `Status` is `Not reproducible`, not `Confirmed`).

## Class

N/A (not applicable - see `Status` above; no bug was confirmed reachable).

## Root cause

Not a live bug in the reachable sense. The suspicion rested on two premises: (1)
`unpremultiply::scalar`/`simd` are unsound if `component > alpha` (verified true, in isolation -
see "What was tried" above), and (2) that the real `F32Kernel::pack`'s independent per-channel
`(src[c] * 255.0 + 0.5) as u8` rounding, combined with floating-point error accumulated across
blend-mode math (`fine/highp/blend.rs`'s `mix_inner`/`apply_alpha`, which unpremultiplies,
blends, then re-premultiplies each channel through several f32 multiply/add/divide steps), could
occasionally push a rounded color channel one integer above the rounded alpha channel. Premise (2)
was not observed in ~160M real pixel checks: the blend math's f32 rounding error (~1e-7 relative)
is far smaller than the ~1/510 gap needed to flip which integer bucket a color and its alpha
channel round to, and-for the cases actually reachable through `RenderContext`'s public API -
`color <= alpha` appears to hold with enough margin in the real-valued blend math that independent
per-channel rounding never flips the ordering in this search. The PR #1834 commit message cited in
the original suspicion (rounding deltas up to 1 "during blending") in fact refers to a *different*
pipeline - PNG-loading premultiplication (`premultiply_rgba8`, used by `Pixmap::from_parts`'s
already-premultiplied-input path), not `fine::highp::pack`'s blend-then-round path - so the
inference connecting that commit message to this candidate's suspected trigger does not hold up.

## Introduced by

N/A for a "Not reproducible" result (no bug was confirmed to exist reachably). For reference: the
`scalar`/`simd`/`reciprocal`/`div_256` arithmetic itself was introduced by 1a6d044941f446a6b384b32f0f34e2362d4bd25d
"vello_common: Optimize pixmap unpremultiplication (#1839)" (2026-08-26).

## The fix

N/A - `Status` is `Not reproducible`; no fix was attempted, and no `patch.diff` is included. The
main project's working tree was never modified during this candidate.

- Project test suite: N/A - no change was made to the source tree, so no before/after run was
  needed for this candidate.

## What's not done / open questions

- The search covered `RenderContext`'s solid-color fill + blend-mode + layer-opacity surface on
  small (8x8) canvases; it did not cover gradients (`peniko::Gradient` stop interpolation),
  images (bilinear/bicubic sampling), text/glyph rendering, or filter effects (blur, etc.) as
  sources of color/alpha channels that could round independently. Those paths involve additional
  floating-point interpolation that wasn't exercised here and could in principle behave
  differently, though there is no specific reason from reading the code to expect them to violate
  the invariant where the tested paths didn't.
- `Div256Ext::div_256`'s own `self + 128` (using a plain `+`, not `wrapping_add`) is itself
  fragile independently of this candidate's specific suspicion: if any *future* caller of
  `unpremultiply::scalar`/`simd` is ever added that doesn't uphold the `component <= alpha`
  precondition (or if a future producer bug elsewhere violates it), the result is either a silent,
  drastically wrong unpremultiplied color (this session's isolated demonstration: `alpha=152,
  component=153` -> output `0` instead of ~255) in release builds, or a debug-assertions panic for
  larger deltas - worth a defensive `debug_assert!(component <= alpha)` or a saturating
  add/clamp in `div_256`/`scalar`/`simd` even though no live trigger was found this session, purely
  as a guard against a future regression silently reintroducing a real-world trigger.
- A longer/more targeted fuzzing campaign (via `fuzz-bug-repro`, run separately) specifically
  targeting `fine::highp::blend.rs`'s per-`Mix`-mode arithmetic with adversarial (not
  uniformly random) input construction designed to push the blend result as close as possible to
  an integer-rounding boundary might still find a violation that uniform random sampling did not;
  this session's negative result is bounded by what uniform random search covers, not an exhaustive
  proof.
