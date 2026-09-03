# GPU_VS_CPU_DIFFERENTIAL_FUZZED_SCENE_COVERAGE

Candidate 082 (`GPU_VS_CPU_DIFFERENTIAL_IMAGE_COMPARISON_HARNESS`, Not reproducible) swept the
project's own GPU-vs-CPU FLIP differential comparison (`vello_tests::compare_gpu_cpu`, patched per
candidate 065's already-recorded self-comparison fix) across all 33 hand-authored example scenes
and found no divergence, but explicitly left two follow-ups untried: (1) feeding the harness a
fuzzed/randomly-generated scene corpus instead of only the fixed example scenes, and (2) re-running
on real GPU hardware if it ever becomes available in this environment. This candidate is that
follow-up, per `projects/vello.md`'s "Next-session leads".

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Checked for real GPU hardware in this environment (`lspci`, `nvidia-smi`, `vulkaninfo`, `/dev/dri`) | - | None found - see "Real GPU hardware" below; same conclusion as 082's session, re-checked fresh this session rather than assumed |
| Applied candidate 065's fix locally to `vello_tests/src/compare.rs` (still not merged upstream as of this session - re-verified by reading the file before patching) | - | Makes `compare_gpu_cpu`'s FLIP score real instead of a hardcoded 0.0 |
| Wrote `harness/compare_gpu_cpu_fuzz.rs`: a small self-contained xorshift64* PRNG builds random `vello::Scene`s (random rect/rounded-rect/circle/polygon-with-curves shapes, random fill/stroke styles+joins+caps, random solid/linear/radial/sweep-gradient brushes including deliberately-injected degenerate cases - matching linear start==end, radial zero-radius, sweep start_angle==end_angle - random affine transforms including near-zero scale, random nested push_layer/pop_layer with random blend modes, occasional tiny random image-brush fills), then runs each through `compare_gpu_cpu_sync` | release build, real `wgpu` device | 5 runs (see table below and `output.txt`), 4610 total (scene, AA-config, backend) comparisons across 4050 distinct (seed, AA-config) combinations, 0 divergences |
| Same harness, real Mesa llvmpipe software GL backend instead of Vulkan/SwiftShader, disjoint+overlapping seed ranges for cross-backend confirmation | release build | 0 divergences (see `output.txt` runs B, C, E) |
| Added worst-FLIP-mean tracking (not just failures) to the harness and re-ran a smaller batch on both backends to get a concrete "how close did we get" data point | release build | Worst FLIP mean observed: 0.00057 (Vulkan/SwiftShader, seed 146, `AaConfig::Area`), 0.0001 (GL/llvmpipe, seed 186) - both far below the project's own 0.01 threshold and below 082's closest example-scene result (`mmark`, ~0.0095) |

Full per-run detail (backend, seed range, count, worst mean where tracked) is in `output.txt`.

## Status

**Not reproducible.** Across 4610 real GPU-vs-CPU differential renders (a real `vello::Renderer`
GPU path via a real `wgpu::Device`, and the real `vello_cpu` renderer, compared via the project's
own FLIP-based `compare_gpu_cpu`) spanning 4050 distinct randomly generated scenes (2 real software
`wgpu` backends x 2 `AaConfig`s x seeds 1-250/1000-1150/5000-8000), none exceeded the project's own
0.01 FLIP-mean divergence threshold; the worst observed mean (0.00057) was roughly 17x below it.
No further approaches were tried this session for the fuzzing dimension - a much larger campaign
(via `fuzz-bug-repro`, run separately, with a real coverage-guided fuzzer rather than this shallow
one-off PRNG-driven generator) is the natural next step if more assurance is wanted, but is out of
this skill's scope (see `SKILL.md`'s "When NOT to use").

### Real GPU hardware

Still not available in this sandbox, re-checked fresh this session (not assumed from 082's prior
note): `lspci` returned nothing (no `lspci` output at all - likely no PCI enumeration exposed to
this sandboxed environment), `nvidia-smi`/`vulkaninfo` are both absent from `PATH`, and `/dev/dri`
does not exist (no render nodes). The only usable `wgpu` backends remain the two already
established in `README.md`/candidate 082: SwiftShader (software Vulkan) and Mesa llvmpipe (software
GL, via `LIBGL_ALWAYS_SOFTWARE=1`). This follow-up is therefore still open, not closed - see
"What's not done" below.

**Fully reproducible**: N/A (Not reproducible - no bug to reproduce).

## Class

N/A

## Root cause

N/A - broad differential sweep, not an investigation of one suspected code path. A randomly
generated scene corpus (built from a small grammar of shapes/brushes/transforms/clips, including
deliberately-injected degenerate gradient/transform cases matching bug classes already found
elsewhere in this project's COLR/glyph code - candidates 076, 081, 086) does not expose any
GPU-vs-CPU rendering divergence in the classic `vello`/`vello_cpu` renderer pair, at least not
within the scale of scene complexity this harness's PRNG grammar reaches (2-8 shapes/layers per
scene, 400x400px canvas). If anything, these simple randomly composed scenes agreed *more* closely
between GPU and CPU than the hand-authored example scenes already swept in 082 (worst mean
0.00057 vs. 082's closest 0.0095) - plausibly because the hand-authored examples were specifically
designed to stress edge cases (`FunkyPathsScene`, `TrickyStrokesScene`, etc.) that this shallow
PRNG grammar doesn't reliably reconstruct.

## Introduced by

N/A - no bug found, nothing to attribute.

## The fix

N/A - no bug found, no patch.

- Project test suite: N/A for this candidate specifically. The harness itself
  (`compare_gpu_cpu_fuzz.rs`) is scratch investigation infrastructure, gated `#[ignore]`, not
  proposed for the permanent suite; no change was made to real project source in this candidate
  (the 065 fix was applied only in the isolated worktree used to run this investigation, then
  discarded - see `justfile`/`harness/065_fix_needed_for_harness.patch`), so there is nothing to
  re-run the project's own test suite before/after for.

## What's not done / open questions

- **Real GPU hardware was still not available in this environment** - this follow-up remains
  genuinely untried, not ruled out. If this sandbox (or a future session) ever gets access to a
  real GPU (any vendor/backend), re-running both candidate 082's example-scene sweep and this
  candidate's fuzz harness against it is the natural next step; neither harness has any
  environment-specific assumption that would need changing to do so (both already read
  `VK_ICD_FILENAMES`/`WGPU_BACKEND` from the environment rather than hardcoding SwiftShader/
  llvmpipe).
- This session's PRNG-driven scene grammar is deliberately shallow (per `SKILL.md`'s own scoping
  note that this isn't a real fuzzing campaign) - it covers basic shapes, gradients (including the
  three known degenerate cases from 076/081/086), strokes, nested clip/blend layers, and tiny image
  brushes, but not: text/glyph rendering (glifo's font-parsing path, where most of this project's
  confirmed bugs actually live per `projects/vello.md`'s risk map), blurred rounded rects
  (`draw_blurred_rounded_rect`), dashed strokes, or deeply nested/many-shape scenes (capped at 2-8
  shapes for practical runtime). A real coverage-guided fuzzer (`fuzz-bug-repro`, run separately)
  targeting the scene *encoding* itself (not just a hand-picked shape/brush grammar) would have a
  better chance of finding a genuine GPU-vs-CPU divergence if one exists, but is out of this
  skill's scope.
- Same caveat as 082: software rasterizers (SwiftShader/llvmpipe) may not reproduce hardware-
  specific driver quirks (candidate 073's GLES texture-array misclassification was exactly this
  kind of driver-heuristic bug) - this sweep, however large, still cannot rule those out.
