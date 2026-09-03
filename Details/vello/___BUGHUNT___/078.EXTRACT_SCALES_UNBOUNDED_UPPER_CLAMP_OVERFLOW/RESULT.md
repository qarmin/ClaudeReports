# EXTRACT_SCALES_UNBOUNDED_UPPER_CLAMP_OVERFLOW

`vello_common::util::extract_scales` (`sparse_strips/vello_common/src/util.rs:126-145`) extracts
`(scale_x, scale_y)` from an affine transform via an SVD-style closed form in `f32`, feeding
`gaussian_blur::transform_blur_params` (`sparse_strips/vello_common/src/filter/gaussian_blur.rs:
26-33`). Both results are only lower-clamped (`.max(1e-6)`) - there is no upper clamp. A
non-uniform transform with one very large scale factor overflows the `f32` squaring inside
`extract_scales` to `+inf`, which `transform_blur_params` and `GaussianBlur::new` then feed
straight into `plan_decimated_blur`'s decimation loop, which never terminates for infinite
variance.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction against the real public API: standalone probe crate (path dependency on this worktree's `vello_common`) sweeping `vello_common::util::extract_scales` over uniform and non-uniform `Affine` scales from `1e18` up to `f64::MAX` | release build, debug-assertions + overflow-checks enabled, single run | A **uniform** scale overflowing `f32` (>=~1.9e19) produces `scale_x=scale_y=0.000001` (the lower floor) - masked by the already-known NaN-absorption quirk (candidate 019: `s1-s2` becomes `inf-inf=NaN` for a uniform scale, and `f32::max(NaN, 1e-6)` returns the floor). A **non-uniform** scale (huge on one axis only, e.g. `scale_x=1e19, scale_y=1.0`) avoids that cancellation: `scale_x` genuinely comes back `inf`, uncapped, confirming the described missing upper clamp. |
| Same probe, driving `extract_scales`'s `inf` result through the real `transform_blur_params`-equivalent arithmetic into the real public `vello_common::filter::gaussian_blur::GaussianBlur::new`, wrapped in a background thread with a 5s wall-clock timeout (a genuine hang can't be "waited out") | release build, 3 separate runs | **Hung 3/3 runs** - `GaussianBlur::new` never returned within the 5s timeout for any run. |
| Real end-to-end repro via the actual public rendering pipeline: standalone crate depending on `vello_cpu`/`vello_common`, driving `RenderContext::new` -> `set_transform` (the pathological non-uniform transform) -> `push_filter_layer(Filter::from_function(FilterFunction::Blur{..}))` -> `set_transform(IDENTITY)` -> `fill_rect` (ordinary on-canvas content) -> `pop_layer` -> `flush` -> `render` - the same sequence `vello_cpu/examples/basic.rs` uses for the rest of its API, wrapped in the same background-thread timeout | release build, 3 separate runs against pristine code | **Hung 3/3 runs** - `RenderContext::render()` never returned within a 10s timeout. A temporary one-line `eprintln!` (removed again before writing the fix) confirmed the real call path reaches `transform_blur_params` with `scale_x=inf` before hanging, ruling out a coincidental hang elsewhere. |
| Varied input within the same bug class after the fix (CONFIRMATION.md section 7): re-ran the direct-API probe with (a) this candidate's transform-driven `inf` vector, (b) a sibling vector - a directly huge `std_deviation` (`5e19`) with **no transform involved at all**, which overflows `f32` on its own square (noted as a "new suspect" by an earlier candidate, 033), and two sanity checks (a legitimate large-but-finite transform scale, and an ordinary small blur) | release build, patched tree, single run covering all 4 cases | Both pathological vectors (a) and (b) now return in ~20ms instead of hanging; both legitimate cases still produce a real, non-identity kernel (`kernel_size=13`, `n_decimations=10` for the large-but-finite scale; `kernel_size=13` for the ordinary blur) - the fix does not silently disable blurring in general, and also happens to close the sibling direct-`std_deviation` vector even though this candidate's own described trigger is transform-driven. |

## Status

**Confirmed**. Reproduces 3/3 runs (release build) for both the isolated `GaussianBlur::new`
call and the real end-to-end `vello_cpu::RenderContext::render()` call - fully deterministic (a
plain arithmetic-overflow bug, no memory-layout/seed dependency).

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls only the real, public, production
entry point a real application uses - `vello_cpu::RenderContext::push_filter_layer` (with a
`Filter` built from the public `FilterFunction::Blur`), `fill_rect`, `pop_layer`, `render` - the
same sequence `vello_cpu`'s own `examples/basic.rs` uses for the rest of its API. The
caller-supplied transform/`std_deviation` are a single external artifact (an ordinary, if
extreme, caller-controlled affine transform and filter parameter) with no second participant to
fake, the same class CONFIRMATION.md section 11 says a single-process repro against the real
public entry point is sufficient for.

## Class

crash

(A hang/infinite-loop, not a panic - functionally a denial-of-service against the calling
thread. `repro/` demonstrates this via a bounded wall-clock timeout on a background thread rather
than actually waiting forever, per the note in CONFIRMATION.md about not needing an
externally-visible crash specifically - the observed symptom here is "never returns", not "abort".)

## Root cause

Three pieces combine:

1. `extract_scales` (`util.rs:126-145`) downcasts the transform's `f64` coefficients to `f32`,
   computes `a2 = a*a` etc. and `s1 = a2+b2+c2+d2`, `s2 = sqrt((a2-b2+c2-d2)^2 + 4*(ab+cd)^2)`,
   then `scale_x = sqrt(0.5*(s1+s2)).max(1e-6)`, `scale_y = sqrt(0.5*(s1-s2)).max(1e-6)`. For a
   transform with one very large scale factor (e.g. `a=1e19`, `d=1.0`), `a2` alone
   (`1e38`-ish) already exceeds `f32::MAX` (`~3.4e38`) once `a` passes roughly `1.84e19`,
   overflowing to `+inf`. `s1` and `s2` both become `+inf`, and `scale_x = sqrt(0.5*(inf+inf)) =
   inf`. `.max(1e-6)` only ever raises a value that's too small - it never lowers one that's too
   large, so `inf` passes through unchanged. (For a *uniform* scale specifically, `s1-s2` happens
   to become `inf-inf=NaN`, and `f32::max(NaN, 1e-6)` returns the floor instead - this is a
   separate, already-confirmed bug, candidate 019 - but a non-uniform scale sidesteps that
   cancellation and reaches genuine unclamped `inf`.)
2. `gaussian_blur::transform_blur_params` (`gaussian_blur.rs:26-33`) computes `uniform_scale =
   (scale_x + scale_y) / 2.0` and returns `std_deviation * uniform_scale` - an `inf` `scale_x`
   makes this `inf` too (as long as the caller's `std_deviation` is a nonzero finite number;
   `0 * inf` would be `NaN` instead, but that's not the case here).
3. `plan_decimated_blur` (`gaussian_blur.rs:92-129`) computes `variance = std_deviation *
   std_deviation` and then loops `while remaining_variance > 4.0 { remaining_variance =
   (remaining_variance - 1.5) * 0.25; n_decimations += 1; }`. For `variance = inf`, `(inf - 1.5)
   * 0.25 == inf` again - the loop condition `inf > 4.0` is always true and `remaining_variance`
   never decreases, so the loop never exits. `n_decimations` (a `usize`) increments forever;
   with 64-bit `usize` this would take on the order of `2^64` iterations to reach an
   overflow-panic in a debug build, which in practice means an unbounded hang rather than a
   panic, consistent with what was observed (the calling thread simply never returns).

This is reachable end-to-end through the real public API: any caller that applies a Gaussian
blur filter (`FilterFunction::Blur`, or the lower-level `FilterPrimitive::GaussianBlur`) while a
sufficiently large non-uniform scale transform is active (e.g. from an SVG `transform="scale(x
y)"` with a very large `x` and an ordinary `y` - `Affine::scale_non_uniform` is exactly what
`vello_common::pico_svg` uses to parse that attribute) hangs the render call.

## Introduced by

ee4394aa "feat(vello_cpu): Image Filters (Filter Effects) implementation (#1286)" (2025-11-21) -
`git log -S "while remaining_variance > 4.0"` and `git log -S "fn extract_scales"` both point to
this commit as the one that introduced `extract_scales`, `transform_blur_params`, and
`plan_decimated_blur`'s decimation loop together, unchanged in their overflow-relevant logic
since (later commits 0961df0b and 3a0908d6 only moved/reused the code across crates). This
matches the introducing commit already identified for the related-but-distinct candidate 019.

## The fix

Added a non-finite check on `variance` (`std_deviation * std_deviation`) at the top of
`plan_decimated_blur`, immediately after computing it, returning the same identity kernel (no
blur) the function already returns for `std_deviation <= 0.0`:

```rust
let variance = std_deviation * std_deviation;
if !variance.is_finite() {
    let mut kernel = [0.0; MAX_KERNEL_SIZE];
    kernel[0] = 1.0;
    return (0, kernel, 1);
}
```

This was chosen over clamping `extract_scales`'s output alone because that would only close the
transform-driven vector this candidate is specifically about, while leaving the sibling vector
(a directly-huge `std_deviation`, independent of any transform, noted by candidate 033) open -
the exact same infinite loop is reachable that way too (verified above: "vector B"). Guarding
`variance` at the one place the loop actually lives closes both vectors with a single, minimal,
one-if-statement change, using the same "return an identity kernel for invalid input" idiom the
function already establishes for `std_deviation <= 0.0`.

Checked against more than the one triggering input (CONFIRMATION.md section 7): both the
transform-driven vector (this candidate) and the direct-`std_deviation` vector (the sibling noted
by 033) now return quickly instead of hanging, and two legitimate cases (an ordinary small blur,
and a real large-but-finite transform scale) still produce a real, non-identity kernel after the
fix - see `output.txt` for the full transcript.

- Project test suite (`cargo test -p vello_common --release`, the only crate that calls
  `plan_decimated_blur`/`extract_scales`): 159 passed; 0 failed; 1 ignored - before the fix. 159
  passed; 0 failed; 1 ignored - after the fix. No regressions, no new failures.

## What's not done / open questions

- `extract_scales` itself is still unbounded (no upper clamp on its own output) - this fix closes
  the observable hang at its one real consumer (`plan_decimated_blur`) without touching
  `extract_scales`, so any other future caller of `extract_scales` would need to independently
  guard against `inf`/`NaN` the same way. `extract_scales` currently has exactly one caller
  (`transform_blur_params`), so this is not an active gap today, but is worth noting for anyone
  adding a second caller later.
- Candidate 019's separate NaN-absorption bug in `extract_scales` (uniform near-singular *or*
  uniform-overflow transforms silently getting the `1e-6` floor instead of their true value) is
  untouched by this fix - it produces a wrong-but-finite result, not a hang, so it remains its own
  distinct, already-recorded candidate.
- Did not check whether `vello_hybrid`'s GPU-side blur setup (`filter.rs`, which also calls
  `plan_decimated_blur`/`compute_gaussian_kernel` per its own tests) has an equivalent
  transform-scale-driven path that could reach the same hang - `vello_hybrid`'s filter transform
  handling wasn't exercised this session; the fix is in the shared `vello_common` function both
  backends call, so it should cover this backend too, but that wasn't independently re-verified
  end-to-end through `vello_hybrid`'s own pipeline.
- Did not attempt a longer/dedicated fuzzing campaign (via `fuzz-bug-repro`, run separately) over
  `extract_scales`'/`plan_decimated_blur`'s combined input space for other non-finite-producing
  transform shapes beyond the one non-uniform-scale case constructed by hand here.
