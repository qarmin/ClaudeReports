# GPU_VS_CPU_DIFFERENTIAL_IMAGE_COMPARISON_HARNESS

Candidate 065 (`COMPARE_GPU_CPU_SELF_COMPARISON_BLIND_HARNESS`, Confirmed) found that
`vello_tests::compare_gpu_cpu`'s FLIP score was a hardcoded 0.0 self-comparison bug, meaning this
project's own GPU-vs-CPU differential test suite had never actually compared anything for as long
as that bug existed. With that harness fix now known and available, this candidate asks the
natural follow-up: once the comparison is real, does sweeping it across *every* example scene (not
just the ~10 `compare_gpu_cpu.rs` already exercises) surface a genuine GPU-vs-CPU rendering
divergence that the blind harness would have hidden until now (see `projects/vello.md`'s note that
GPU-shader-path bugs found so far all came from reading WGSL code, never from actually running the
shaders in this environment).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Applied candidate 065's fix locally to `vello_tests/src/compare.rs` (repo has not merged it - see that candidate's own patch.diff) | - | Makes `compare_gpu_cpu`'s FLIP score real instead of a hardcoded 0.0 |
| Wrote `harness/compare_gpu_cpu_sweep.rs`: runs `compare_gpu_cpu` against every scene in `scenes::test_scenes::test_scenes()` (33 scenes), reporting each scene's real FLIP mean | release build, real `wgpu` device (SwiftShader software Vulkan ICD) | 1 run, `AaConfig::Area`: 33/33 scenes under the project's own 0.01 FLIP-mean threshold (closest: `mmark`, 0.0095) |
| Same sweep, real Mesa llvmpipe software GL backend instead of Vulkan | release build | 1 run: 33/33 scenes under threshold, generally lower FLIP means than the Vulkan/SwiftShader run |
| Same sweep, `AaConfig::Msaa16` instead of the default `AaConfig::Area` (Vulkan/SwiftShader) | release build | 1 run: 32/33 scenes checked (1 filtered by `#[ignore]` gating), all under threshold (closest: `mmark`, 0.0098) |

## Status

**Not reproducible.** 33 example scenes x 3 renderer configurations (Vulkan/SwiftShader + Area AA,
GL/llvmpipe + Area AA, Vulkan/SwiftShader + MSAA16) = 99 real differential GPU-vs-CPU renders, all
via this project's own real `wgpu`-backed renderer and real `vello_cpu` renderer, none producing an
error or a FLIP mean indicating genuine rendering divergence. No further approaches were tried this
session (a real-GPU-hardware run, rather than software rasterizers, was not available in this
environment - see `README.md`'s environment notes).

**Fully reproducible**: N/A (Not reproducible - no bug to reproduce).

## Class

N/A

## Root cause

N/A - this was a broad differential sweep, not an investigation of one suspected code path.
Software-rasterizer-backed `wgpu` (SwiftShader/llvmpipe) rendering of these 33 example scenes
matches `vello_cpu`'s CPU rendering closely enough (all well under the project's own existing
0.01 FLIP-mean acceptance threshold) that no divergence surfaced.

## The fix

N/A - no bug found, no patch.

- Project test suite: N/A for this candidate specifically (the sweep test itself, gated
  `#[ignore]`, is scratch infrastructure for this investigation only - not proposed for the
  permanent suite; the project's normal test suite was not separately re-run since nothing was
  changed in real source).

## What's not done / open questions

- Only software rasterizer backends (SwiftShader Vulkan, Mesa llvmpipe GL) were exercised - a real
  GPU (any vendor, any of Metal/DX12/native Vulkan) was not available in this environment. Software
  rasterizers may not reproduce hardware-specific driver quirks (e.g. candidate 073's GLES texture-
  array misclassification was specifically a driver-heuristic bug that might behave differently, or
  not trigger at all, under a fully software GL implementation) - this sweep does not rule those out.
  Worth re-running on real hardware if that ever becomes available in this environment.
  the existing ~10-scene `compare_gpu_cpu.rs` and this sweep's 33 scenes are both still limited to
  this project's own example/test scenes - a fuzzed/randomly-generated scene corpus was not tried
  and might exercise renderer code paths the hand-authored example scenes don't (a genuine next step
  for `fuzz-bug-repro`-style differential fuzzing, run separately, rather than this skill's own
  shallow one-off harnesses).
- The `harness/065_fix_needed_for_harness.patch` dependency means this sweep's value is contingent
  on candidate 065's fix eventually landing upstream - until then, `vello_tests`' own real test
  suite still silently can't detect GPU-vs-CPU divergence at all.
