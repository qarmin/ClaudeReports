# COMPARE_GPU_CPU_SELF_COMPARISON_BLIND_HARNESS

Candidate 065 set out to build/run a real CPU-vs-GPU differential rendering harness in this
sandbox (previous shader bugs 011/014/017/020 were all found by manual code reading, never by
actually executing a real `wgpu` device here). This session first confirmed a real `wgpu` device
*is* obtainable here (both via Mesa's `llvmpipe` software rasterizer through the GL backend, and
via a SwiftShader software Vulkan ICD already staged at `tmp/swiftshader_icd/` from an earlier
session), then used it to actually run the project's own existing GPU-vs-CPU differential test
harness (`vello_tests`' `compare_gpu_cpu`/`compare_gpu_cpu.rs`, the exact mechanism candidate 065
was looking for) - and found that the harness itself is broken: `vello_tests/src/compare.rs`'s
`compare_gpu_cpu` builds *both* of the two images it feeds into its `nv_flip` perceptual-diff
computation from `cpu_rendered.data` - the real `gpu_rendered.data` is stored on the returned
`GpuCpuComparison` struct (public field) but is never read by the diff computation. This means the
harness's reported similarity score is a hardcoded 0.0 (self-comparison) for every scene, and it
can never detect a real GPU-vs-CPU rendering difference, however large.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Set up a real `wgpu` device in this GPU-less sandbox: tried Mesa's `llvmpipe` software rasterizer via the GL backend (`WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1`, standard Mesa packages already installed), and a SwiftShader software Vulkan ICD staged at `tmp/swiftshader_icd/vk_swiftshader_icd.json` (points at Playwright's bundled `libvk_swiftshader.so`) via `VK_ICD_FILENAMES`/`WGPU_BACKEND=vulkan` | `cargo test -p vello_tests --test smoke_snapshots`, debug and release builds | Both backends work: a real `vello::Renderer` device is obtained and real scenes render through the real WGSL shaders on both, confirming (independently of candidate 020's prior finding) that a real `wgpu` device is genuinely runnable here |
| Ran the project's own existing GPU-vs-CPU differential test suite (`vello_tests/tests/compare_gpu_cpu.rs`: `compare_splash`, `compare_funky_paths`, `compare_stroke_styles*`, `compare_fill_types`, `compare_deep_blend`, `compare_blurred_rounded_rect`, `compare_large_bin_count`) against real scenes, driven by the real `wgpu` device from the step above | `cargo test -p vello_tests --test compare_gpu_cpu --release -- --include-ignored`, both GL/llvmpipe and Vulkan/SwiftShader backends | All 10 tests pass - but reading `vello_tests/src/compare.rs`'s implementation of `compare_gpu_cpu` (the function every one of these tests calls) shows the FLIP score it reports is structurally incapable of ever being anything but the (mathematically guaranteed) result of comparing an image to itself, regardless of what the real `gpu_rendered` image actually contains |
| Built a standalone repro calling the real, unmodified, public `vello_tests::compare_gpu_cpu_sync` entry point on a real scene (`splash_with_tiger`, then `funky_paths` as a second scene in a different class), independently inspecting the `GpuCpuComparison` struct's own public `cpu_rendered`/`gpu_rendered` fields (the real rendered images) alongside the library's own reported `statistics.mean()` | release build, real `wgpu` device (both backends tried) | Confirmed: for `splash_with_tiger`, 1392-1398 of 1,440,000 raw bytes differ between the real CPU-rendered and real GPU-rendered images (varies slightly run to run, consistent with ordinary GPU-driver/compute non-determinism), yet `compare_gpu_cpu_sync`'s own `statistics.mean()` is exactly `0.0` every single time - the harness is blind to a real, repeatedly-observed difference it exists specifically to catch. Same qualitative result on `funky_paths` (6/1,440,000 bytes differ, `mean()` still exactly 0.0) |
| Repeatability (CONFIRMATION.md section 1) | Same repro binary, `splash_with_tiger` scene | 4/4 runs (1 initial + 3 repeats) via Vulkan/SwiftShader all show `statistics.mean() == 0.0` despite a nonzero (varying) real byte-diff count; separately 3/3 runs via GL/llvmpipe show the same |

## Status

**Confirmed**. Reproduces deterministically in the sense that matters (the reported `mean()` is
*always* exactly `0.0`, by construction of the buggy code - not layout/seed-dependent): 4/4 runs
(Vulkan/SwiftShader backend) + 3/3 runs (GL/llvmpipe backend), 7/7 total, release build, two
different scenes (`splash_with_tiger`, `funky_paths`).

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls the real, unmodified, public
`vello_tests::compare_gpu_cpu_sync`/`encode_test_scene`/`TestParams` API - exactly the entry point
the project's own `compare_gpu_cpu.rs` regression-test suite calls - driven by a real `vello`
`Renderer` and a real `wgpu` device end to end (no hand-authored stand-in for the renderer, the
GPU, or its output). This is a single-process, single-participant scenario (a scene handed to the
harness's real public API); there is no second peer/caller whose behavior needed to be faked, so
CONFIRMATION.md section 11's bar is satisfied the same way a file handed to a parser's real public
entry point would be.

## Class

logic (test-infrastructure correctness bug: a differential test whose comparison always trivially
"passes" regardless of the real inputs)

## Root cause

`vello_tests/src/compare.rs`'s `compare_gpu_cpu` renders a scene twice - once with
`params.use_cpu = false` (the real GPU/WGSL `fine` path, stored in `gpu_rendered`) and once with
`params.use_cpu = true` (the CPU/Rust `fine` path, stored in `cpu_rendered`) - then is supposed to
feed both real images into an `nv_flip` perceptual-diff computation. Instead, both of the
intermediate `RgbaImage`s built for that computation are constructed from `cpu_rendered.data`:

```rust
let gpu_rendered_data: DynamicImage = image::RgbaImage::from_raw(
    cpu_rendered.width,
    cpu_rendered.height,
    cpu_rendered.data.as_ref().to_vec(),   // <- should be gpu_rendered.*
)
...
let cpu_rendered_data: DynamicImage = image::RgbaImage::from_raw(
    cpu_rendered.width,
    cpu_rendered.height,
    cpu_rendered.data.as_ref().to_vec(),
)
```

`nv_flip::flip(cpu_flip, gpu_flip, ...)` is therefore always called with two byte-identical
images, which any perceptual-diff metric must score as zero difference everywhere - the reported
`FlipPool::mean()` is mathematically guaranteed to be exactly `0.0` regardless of what the real
`gpu_rendered` image actually contains. The real `gpu_rendered`/`cpu_rendered` `ImageData`s are
still stored correctly on the returned `GpuCpuComparison` struct's own public fields (used only for
writing debug PNGs on failure, which - because `mean()` can never exceed the threshold - can also
never actually fire), so the bug is purely in which local variable feeds the FLIP computation, not
in how the two renders themselves are produced. This means the `compare_gpu_cpu.rs` test suite
(`compare_splash`, `compare_funky_paths`, `compare_stroke_styles*`, etc.) has never actually been
able to catch a GPU-vs-CPU rendering regression since it was added, however large the real
divergence - it would pass identically even if the GPU `fine.wgsl` shader path were completely
broken.

## Introduced by

b103b03dbd08dc8ac00f2f1ac09f4af8c619dc5f "Add some snapshot test infrastructure (#610)"
(2024-07-15) - the commit that added `vello_tests/src/compare.rs` and this exact
`cpu_rendered.data`-used-twice construction, found via `git log --follow -p` on the file (the
buggy lines have never been touched since).

## The fix

Changed the `gpu_rendered_data` construction to read from `gpu_rendered.width`/`gpu_rendered.height`/
`gpu_rendered.data` instead of the corresponding `cpu_rendered.*` fields (and corrected the
matching error message from "cpu result" to "gpu result"), so the FLIP computation actually
compares the real GPU-rendered image against the real CPU-rendered image. Verified against two
different scenes (`splash_with_tiger`, `funky_paths`) and two different `wgpu` backends
(Vulkan/SwiftShader, GL/llvmpipe): after the fix, `statistics.mean()` reports a small nonzero value
that tracks the real (also independently measured) byte-level divergence between the two renders
(e.g. `0.00016` for `splash_with_tiger`, `0.0000008` for `funky_paths`), instead of a hardcoded
`0.0`.

- Project test suite (`cargo test -p vello_tests --release -- --include-ignored`,
  `VELLO_SKIP_LFS_SNAPSHOTS=all` to skip 6 unrelated tests that fail before and after solely
  because this checkout has no `git-lfs` install to fetch the real snapshot PNGs - a pre-existing,
  unrelated environment gap, not touched by this fix): **65 passed, 0 failed before -> 65 passed, 0
  failed after** (both counts identical; no test's outcome changed). In particular all 10 tests in
  `compare_gpu_cpu.rs` (`compare_splash`, `compare_funky_paths`, `compare_stroke_styles`,
  `compare_stroke_styles_non_uniform`, `compare_stroke_styles_skew`, `compare_tricky_strokes`,
  `compare_fill_types`, `compare_deep_blend`, `compare_blurred_rounded_rect`,
  `compare_large_bin_count`) still pass after the fix - the real GPU-vs-CPU divergence for every
  scene currently in that suite is small enough to stay under the existing `assert_mean_less_than`
  thresholds once the comparison is actually real, so the fix does not introduce any new test
  failures. Verified with both the GL/llvmpipe and Vulkan/SwiftShader backends.

## What's not done / open questions

- This fix makes the harness *capable* of detecting a real GPU-vs-CPU divergence; it does not, by
  itself, prove no such divergence currently exists anywhere in the project beyond what the 10
  scenes in `compare_gpu_cpu.rs` already exercise. Candidates 011/014/017/020 (confirmed WGSL
  shader bugs, found by code reading) are in the coarse/binning stages, which - per this file's own
  comment ("the fine shader still requires a GPU, and so we still get a wgpu device") and
  `vello_tests` doc-comment ("currently `fine` is shared") - run identically as real GPU compute
  shaders regardless of the `use_cpu` flag; only the final `fine` compositing stage differs between
  the two `TestParams::use_cpu` settings. So this specific harness, even fixed, would not have
  caught 011/014/017/020 (those aren't `fine`-stage bugs) - it is specifically useful for
  divergences between `vello_shaders/shader/fine.wgsl` (GPU) and `vello_shaders/src/cpu/fine.rs`
  (CPU), which is a narrower slice of the "differential CPU/GPU/vello_hybrid" scope candidate 065
  originally asked about. A broader differential harness that also compares `vello_hybrid`'s own
  sparse-strip GPU renderer against `vello_cpu`'s CPU renderer (a structurally independent pair of
  implementations, not a shared-coarse-stage pair like this one) was not attempted this session and
  is a promising next step for the same "run real shaders, don't just read them" goal.
- Whether any *other* scene not currently in `compare_gpu_cpu.rs`'s 10-scene list would exceed the
  `assert_mean_less_than(0.01)` threshold once the comparison is real (i.e., whether a genuine,
  previously-undetectable GPU-vs-`fine.wgsl`-vs-CPU-`fine.rs` divergence exists somewhere in the
  project's other example/test scenes) was not exhaustively checked this session - the existing
  10-scene suite all stayed within tolerance, but that is a sample, not a proof of the whole
  `fine`-stage's correctness.
- **Operational note, not part of this candidate's finding**: this session ran in a git worktree
  alongside several sibling candidates' own worktrees sharing the same underlying repository. `git
  stash` operates on state shared across *all* worktrees of a repository (`refs/stash` lives in the
  common `.git` dir, not per-worktree) - using it from this worktree briefly interfered with a
  concurrent sibling candidate's own stash usage (a stash entry belonging to another candidate was
  transiently displaced). No data was lost (the displaced entry was found and re-saved), and this
  candidate's own fix was re-applied directly by file edit rather than relying on the stash to avoid
  further risk, but future parallel-worktree sessions should avoid `git stash` entirely in a shared
  repository and use plain `git diff`/`git checkout --`/`git worktree`-local operations instead.
