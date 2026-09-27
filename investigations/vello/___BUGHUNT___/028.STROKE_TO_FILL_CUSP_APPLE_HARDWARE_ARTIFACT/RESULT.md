# STROKE_TO_FILL_CUSP_APPLE_HARDWARE_ARTIFACT

Source: [linebender/vello#581](https://github.com/linebender/vello/issues/581). The issue's own
reporter (a vello collaborator) states that a specific near-cusp cubic from the `tricky_strokes`
test scene (`[(0., 0.01), (128., 127.999), (128., 0.01), (0., 127.99)]`) produces a visible
flattening artifact "on Apple hardware... at certain scale factors", and that they already
root-caused it themselves: the artifact appears only for a uniform scale factor between `5.3332`
and `113.5797`, disappears when Metal's `MTLCompilerOptions.fastMathEnabled` is explicitly turned
off (verified independently via Skia-Graphite's own Metal backend), and is absent when running the
same algorithm as a plain CPU implementation (no fastmath there). This candidate checks whether the
artifact - or any part of the mechanism behind it - is observable in this (Linux, no Apple/Metal
hardware) sandbox.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end classic-vello pipeline via public API (`vello::Scene::stroke` -> `vello_encoding::Resolver::resolve` -> the real production `vello_shaders::cpu::{pathtag_reduce,pathtag_scan,bbox_clear,flatten}` functions, not a reimplementation), the exact near-cusp cubic, reproducing the real `tricky_strokes` scene's own transform shape (`Affine::new([s,0,0,s,tx,ty])`, uniform scale/no rotation - reduces `flatten_euler`'s own `scale` local var to exactly `s`) and stroke-width relationship (`Stroke::new(30./s)`), across 13 scale factors spanning and well outside the reporter's own affected range (1..200, including the exact boundaries 5.3332 and 113.5797) | Release build, `cargo run --release` in `repro/` (plain scalar `f32` Rust arithmetic - `rustc` never reassociates floating-point ops unless an explicit `fadd_fast`-style intrinsic is used, and none is used anywhere in `vello_shaders`) | No crash, no non-finite (`NaN`/`inf`) point emitted at any scale (assertion in the harness), and segment count / total polyline length grows smoothly and monotonically with `s` (66 to 408 lines, 667 to 93807 device px) - no sudden collapse, blow-up, or other structural anomaly. See `output.txt`, `repro/src/main.rs`. |
| Real GPU-shader dispatch: the actual `flatten.wgsl` compute shader (naga-compiled to real SPIR-V, dispatched through `wgpu`, not the CPU fallback) via a software Vulkan implementation available in this sandbox (SwiftShader, `/tmp/swiftshader_icd/vk_swiftshader_icd.json`, `WGPU_BACKEND=vulkan`) - not Metal, but a genuine non-CPU compute-shader execution, closer to the reported trigger path than the CPU-only check above. Rendered the same near-cusp cubic, view centered on the cubic's own cusp point (local `(96, 64)`, its t=0.5 De Casteljau point) so the interesting region stays on-canvas at every scale, at 7 scales spanning the reporter's range, and diffed the GPU-shader output pixel-by-pixel against the CPU-fallback output of the same scene | Release build, real `wgpu::Instance`/`Device`/`Queue` via `vello::util::RenderContext`, adapter = "SwiftShader Device (Subzero)" (Vulkan, CPU-backed software rasterizer) | Adapter successfully created, `render_to_texture` succeeded at every scale. GPU-vs-CPU pixel diff stayed at antialiasing-rounding-noise level at every scale (0-60 pixels out of 160,000 differing by >8/255, max single-channel diff 36/255) - visually identical (see `output_images/gpu_scale_*.png` vs `cpu_scale_*.png`), no structural glitch resembling the reported artifact. See `output.txt`, `repro/src/bin/gpu_check.rs`. |
| Manual boundary-value/mechanism inspection: temporarily instrumented (`println!`, gated behind a `BUGHUNT028_DEBUG` env var, reverted via `git checkout --` immediately after capturing output - never part of any patch) the real `CubicParams::from_points_derivs` (`vello_shaders/src/cpu/euler.rs`) and the `flatten_euler` ACCEPT/SUBDIVIDE decision site (`vello_shaders/src/cpu/flatten.rs`) to inspect the actual intermediate values for this exact cubic at scale 113.5797 (the reporter's own upper boundary) | Debug instrumentation on top of the release build, same cubic/transform as above, single scale (113.5797) | At several subdivision steps, `cubic_params.err * scale` lands within ~1.3% of the `tol = 0.25` ACCEPT/SUBDIVIDE decision boundary (e.g. `0.24686523` and `0.24690735` against `tol=0.25`) - and `err` itself is dominated by `aerr = (a - amin).abs()`, a subtraction of two nearly-equal `f32` values (e.g. `a = 0.0050436417`, `amin = 0.005086926`, difference `-0.000043284148`) that loses roughly 2 significant decimal digits to cancellation. See `output_debug_113.5797.txt` and the excerpt in `output.txt`. |

No `miri`/ASAN/UBSAN was used: this is not a memory-safety/UB question in the Rust code (the CPU
implementation is safe, ordinary, IEEE-754-compliant `f32` arithmetic with no `unsafe`); the
reporter's own diagnosis already pins the mechanism to a specific GPU shader compiler's
FP-reassociation behavior, which is what HEURISTICS.md's "Logic bugs" row points at
(differential testing / reasoning from the reporter's own documented finding), not a
memory-safety class of bug.

## Status

**Not reproducible** (in this sandbox). Neither the real production CPU-shader pipeline (2/2
identical runs, deterministic) nor a real non-Apple GPU-shader dispatch (SwiftShader/Vulkan, a
genuine compute-shader execution of the actual `flatten.wgsl` source) shows the reported artifact
at the reporter's own identified scale range or beyond it. This sandbox has no Apple hardware and
no Metal backend (Metal is macOS/iOS-only; `wgpu` cannot target it from Linux), so the one
environment the issue is actually reported against could not be tested directly. This matches -
and does not contradict - the reporter's own already-published finding that the artifact requires
Metal's `fastMathEnabled` specifically and is absent from CPU-only execution; this session
independently confirms that on real, current `main` code (not just taking the two-year-old issue
report's claim on faith) and additionally shows it's also absent on a second real GPU-shader
backend (SwiftShader's Vulkan/SPIR-V execution), narrowing the "why doesn't this reproduce
outside Metal" question to something specific to Metal's own shader compiler rather than "any GPU
shader dispatch in general".

**Fully reproducible**: N/A (no bug reproduced in this session - see Status).

## Class

N/A (not Confirmed - see Status).

## Root cause

Not independently established by this session (the reporter already did this work - see below),
and not something this sandbox could re-derive first-hand without Metal. Per the reporter's own
description on the issue: Apple's Metal shader compiler enables `fastMathEnabled` by default,
which permits IEEE-754-violating floating-point reassociation; this is absent in every path this
sandbox can exercise (plain Rust `f32` arithmetic never reassociates without an explicit
`fadd_fast`-style intrinsic, and none is used in this codebase; SwiftShader's Vulkan/SPIR-V
execution path also shows no equivalent divergence from the CPU reference in this session's own
testing).

This session's own instrumentation (see "What was tried", 3rd row, and
`output_debug_113.5797.txt`) identifies a concrete, plausible *point of sensitivity* consistent
with the reporter's diagnosis: for this exact near-cusp cubic at scale 113.5797, several of the
16 cubic-subdivision steps `flatten_euler` performs land within roughly 1.3% of the `err * scale
<= tol` ACCEPT/SUBDIVIDE decision boundary, and `err` is computed through `aerr = (a -
amin).abs()` - a subtraction of two `f32` values that agree to about 2 significant decimal digits
(`a = 0.0050436417` vs `amin = 0.005086926`), i.e. a textbook catastrophic-cancellation-prone
computation. A shader compiler permitted to reassociate the multiply-add chains that produce `a`
and `amin` (each is `0.15 * (2.*d*s + 2.*d*s - d*d*s01)`-shaped, exactly the kind of expression
Metal's fastmath is licensed to reorder/fuse via FMA) could plausibly shift `a`/`amin` by enough
ULPs to flip one or more of these near-boundary ACCEPT/SUBDIVIDE decisions relative to the strict
IEEE-754 CPU reference - changing how many line segments get emitted, and where, for this specific
geometry/scale combination. This is offered as supporting evidence for *why* this particular
cubic/scale-range is sensitive, not as an independently-confirmed root cause (it was not verified
against an actual Metal fastmath-enabled compilation - see "What's not done").

## Introduced by

not determined for this specific candidate. The reporter's own issue text attributes the
mechanism to `flatten_euler`'s cubic-to-Euler-spiral subdivision loop, which candidate
024 (`FLATTEN_SUBDIVISION_FP_PRECISION_ARTIFACTS`, this same workspace) already traced to fe3273ab
"Checkpoint of Euler spiral based stroke expansion (#496)" (2024-03-13) for a *different* failure
mode (large-magnitude local-coordinate cancellation) in the same function. Whether that same
commit is the meaningful "introduction point" for *this* issue's specific fastmath-sensitivity is
not established here - the sensitivity described in "Root cause" above is a property of the
`err`/`aerr` formula's arithmetic shape in general (present since that same commit, essentially
unchanged since - `git log -S "amin ="` shows no substantive rewrite of this expression), not a
regression introduced by a later, separate change; distinguishing "always been theoretically
fastmath-sensitive since #496" from "some later change made it worse" was not attempted this
session (would need bisecting actual Metal-compiled output across historical commits, which needs
Apple hardware this sandbox doesn't have).

## The fix

No fix attempted or applied. `Status` is `Not reproducible` in this sandbox - there is nothing to
verify a patch against here, and the reporter's own issue (open since 2024-05-16, no linked PR)
indicates the upstream maintainers have not landed one either. Even if a fix were drafted purely
from code reading (e.g. reformulating `aerr`'s computation to avoid the `a - amin` cancellation,
or clamping/widening intermediate precision), it could not be verified against the actual bug in
this session, since the actual bug requires Metal's `fastMathEnabled` to manifest at all - this
session's own repro shows the *current* code already behaves correctly under every arithmetic path
available here (plain `f32`, and SwiftShader's Vulkan/SPIR-V execution), so there is no local
signal to check a candidate fix against without introducing artificial fastmath-like reassociation
by hand (which would not be testing the real bug - see CONFIRMATION.md section 11's reasoning
against faking the very mechanism in question).

- Project test suite: not run - no change was made to the main project's working tree at any
  point in this session (all work happened in an isolated git worktree; the one temporary
  `BUGHUNT028_DEBUG`-gated instrumentation added to `vello_shaders/src/cpu/{euler.rs,flatten.rs}`
  was reverted via `git checkout --` immediately after capturing `output_debug_113.5797.txt` -
  confirmed via `git status`/`git diff --stat` showing no changes afterward).

## What's not done / open questions

- **No genuine Metal/Apple-hardware repro was attempted.** Per CONFIRMATION.md section 11's
  narrow exception: this finding's actual trigger condition (Apple's Metal shader compiler with
  `fastMathEnabled`, the default) needs hardware (a real Mac) that is not available in this
  sandbox, and there is no software Metal implementation to substitute (unlike Vulkan, where
  SwiftShader let this session at least exercise the real GPU-shader dispatch path on different
  hardware). Building a hand-constructed stand-in for "what Metal's fastmath reassociation would
  do" (e.g. manually reordering the `a`/`amin` formula, or forcing FMA contraction via
  `mul_add`) was deliberately not written up as a `Confirmed` reproduction, since it would fake
  the very mechanism the finding is about, rather than demonstrate it via the real compiler on
  real hardware - it is presented above only as "Root cause" supporting reasoning, clearly labeled
  as such, not as evidence sufficient for `Confirmed`.
- This session's own near-boundary observation (several subdivisions landing within ~1.3% of
  `tol`) was checked only for scale=113.5797, the reporter's own upper boundary; it was not swept
  across the full reported range (5.3332..113.5797) to see whether near-boundary decisions are a
  narrow-band phenomenon specific to the range's edges or present throughout it. A future session
  with access to real Apple hardware (or a Metal Shading Language cross-compiler + a way to force
  `fastMathEnabled` off/on for an A/B comparison, mirroring exactly what the reporter already did
  for Skia-Graphite) could close this out definitively.
- Not checked whether the *other* near-cusp cell in the same scene
  (`[(0., -0.01), (128., 128.001), (128., -0.01), (0., 128.001)]`, immediately following this one
  in `tricky_cubics`) or the "Perfect cusp" cell exhibit the same near-boundary sensitivity - only
  the exact cubic the issue names was checked.
- `IMAGE_MINIFICATION`/other GPU-vs-CPU-only-on-Apple classes of bug were not searched for
  elsewhere in the codebase as part of this candidate; this was scoped strictly to issue #581's
  own reported case.
