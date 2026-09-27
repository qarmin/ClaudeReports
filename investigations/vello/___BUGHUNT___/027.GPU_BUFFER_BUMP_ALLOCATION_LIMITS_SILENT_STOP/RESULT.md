# GPU_BUFFER_BUMP_ALLOCATION_LIMITS_SILENT_STOP

Investigated three GitHub reports (linebender/vello#720, #936, #1660) of the classic GPU `vello`
backend "just stopping" - a black/incomplete frame with no error, under stress with many paths, a
multi-second freeze on first frame in wasm/gRPC, and rendering stopping past a certain
window/grid size. `vello_encoding`'s coarse-rasterization buffers (`bin_data`, `tiles`, `lines`,
`segments`, `ptcl`, ...) are fixed-size, hand-picked constants (`vello_encoding/src/config.rs`'s
own comment: "hand picked to accommodate the vello test scenes as well as paris-30k") - not sized
from the actual scene. When a real scene exceeds one, the GPU shaders themselves detect it (an
atomic `bump.failed` bitmask, added by aad292dd "Add robustness to GPU shaders (#537)") and
correctly stop drawing that frame rather than corrupting memory - but the classic `vello` crate's
recommended, non-deprecated public API (`Renderer::render_to_texture`) never reads that bitmask
back, so it reports `Ok(())` and an incomplete/blank image with **no way for the caller to detect
anything went wrong**, exactly matching #720's own ask ("It would be helpful if Vello would
report an error in this condition rather than failing silently").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the three linked issues (`gh issue view 720/936/1660`) and their maintainer comments | n/a | #720: maintainer (DJMcNab) confirms this is *intentional* behaviour from #537 ("skip rendering that frame" to avoid corrupting memory), unresolved due to reviewer bandwidth, with a design sketch in #606; a reporter also hit a real wgpu validation panic ("Buffer access out of bounds") using `--async_pipeline`. #1660: a commenter's own theory ("Vello currently seems to not support more than 256 Bins") matches a `log::warn!`-only (not `Err`) check already present in `vello/src/render.rs` (`workgroup_counts.coarse.0 * .1 * .2 > 256`), i.e. the same class of "limit exists but isn't surfaced as an error" bug, just for a different specific limit. #936 is diagnosed by the maintainers as an unrelated Chrome/Tint `disable_workgroup_init` polyfill performance bug, not a Vello buffer-limit bug at all - not part of this candidate's confirmed finding (see "What's not done" below) |
| Manual boundary-value construction: a real GPU render (`Renderer::render_to_texture`, driven by a real wgpu device - swiftshader/Vulkan then Mesa llvmpipe/GL, both usable in this sandbox) of a scene with an increasing draw count `N` on a small (64x64, single-coarse-bin) canvas, reading back the output texture's pixels and (via the *separate*, already-public but deprecated `render_to_texture_async` debug API, which does read the bump buffer) the real `BumpAllocators.failed` bitmask | Debug and release builds, `WGPU_BACKEND=vulkan` (swiftshader ICD) and `WGPU_BACKEND=gl` (Mesa llvmpipe, `LIBGL_ALWAYS_SOFTWARE=1`) - both reproduce identically | Confirmed a real, GPU-observable overflow: at `N` between 130,000 and 135,000 draws (64x64 canvas, all draws landing in the same single coarse bin), `bump.failed` becomes `0x1` (`STAGE_BINNING`, from `binning.wgsl`'s own `atomicAdd(&bump.binning, element_count); if chunk_offset + element_count > config.binning_size { ...; atomicOr(&bump.failed, STAGE_BINNING); }`) and the scene's marker rectangle silently vanishes from the rendered output - see `output.txt` for the exact sweep. The real, non-deprecated `Renderer::render_to_texture` API returns `Ok(())` regardless |
| Same setup, much larger `N` (600,000) with the deprecated `render_to_texture_async` debug API (mirroring the issue's own `--async_pipeline` flag) | Debug build, swiftshader/Vulkan | Reproduced the *exact* failure independently reported in #720's own comment thread: a real `wgpu` panic, `slice offset 0 size 57600096 is out of range for buffer of size 50331648` here vs. the issue's own `Buffer::map_async error: Buffer access out of bounds: last index 50583552 would overrun the buffer (limit: 50331648)` - same class, same code path (`vello/src/lib.rs`'s `DebugDownloads::map`/`render_to_texture_async_internal`, which slices the `lines` buffer using an unvalidated `bump.lines` count downloaded from a *different* prior frame's stale/oversized allocation) |
| Ran the project's own workspace test suite before/after, to check for regressions from the attempted fix (see below) | `cargo test --workspace` (scoped to the classic-GPU-relevant crates), `WGPU_BACKEND=gl`/llvmpipe | See "The fix" below - caught a real regression from an initial version of the fix, which was corrected before finalizing |

Two escalating approaches (issue reading + manual boundary-value construction on the real public
API) were sufficient to get a clean, deterministic, real GPU reproduction - no `miri`/ASAN/fuzzing
needed, since this is a plain "GPU shader sets a flag the CPU side never reads" logic gap, not a
memory-safety/UB question.

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs, both debug and release builds, both
`WGPU_BACKEND=vulkan` (swiftshader) and `WGPU_BACKEND=gl` (Mesa llvmpipe) backends. See
`output.txt` for the literal 3x-before/3x-after `just test` output plus the full threshold sweep.

**Fully reproducible**: **Yes**. This is a single-participant finding (a library's own public
rendering API reacting to its own caller's scene input, not one participant reacting to a peer's
wire input), so `CONFIRMATION.md` section 11's "real second participant" bar doesn't apply. The
repro (`repro/src/main.rs`) drives the actual, real, recommended public entry point
(`vello::Renderer::render_to_texture`) through a genuine wgpu `Device`/`Queue` (no GPU hardware
required in this sandbox - Mesa's llvmpipe software rasterizer over the GL backend runs the exact
same GPU compute-shader pipeline any real GPU would), with a real `Scene` built entirely through
public API (`Scene::fill`), and reads back the real rendered texture's pixels via a real
`copy_texture_to_buffer` + `map_async` - nothing about the GPU pipeline execution is faked or
hand-constructed.

## Class

logic (an internal correctness/robustness signal - `BumpAllocators.failed` - exists and is
correctly computed by the GPU shaders, but is silently discarded rather than surfaced to the
caller by the recommended public API)

## Root cause

`vello_encoding::config::BufferSizes::new` (`vello_encoding/src/config.rs`) allocates every
coarse-rasterization intermediate buffer (`bin_data`, `tiles`, `lines`, `segments`, `ptcl`,
`blend_spill`) as a hardcoded constant (`1 << 18` through `1 << 23`), independent of the actual
scene's complexity - the code's own comment admits these are "hand picked to accommodate the
vello test scenes as well as paris-30k". Since aad292dd (2024-04-01, "Add robustness to GPU
shaders (#537)", written explicitly to address the predecessor of these reports, issue #366), each
GPU compute shader stage checks its own buffer's atomic bump counter against its configured
`_size` limit and, on overflow, sets a bit in `BumpAllocators.failed` and either substitutes a
dummy write location (`binning.wgsl`: `chunk_offset = 0u`) or skips its own work entirely
(`fine.wgsl`: "An earlier stage has failed, don't try to render"). This is a deliberate, correct
design choice to avoid a GPU-side out-of-bounds write/read once a fixed-size buffer is exhausted.

The gap: `vello`'s recommended, non-deprecated public API, `Renderer::render_to_texture`
(`vello/src/lib.rs`), builds its recording via `render::render_full`/`render_encoding_full`
(`vello/src/render.rs`), whose own doc comment already says "This function is not recommended
when the scene can be complex, as it does not implement robust dynamic memory" - but that comment
was, until this patch, purely aspirational: the function never actually downloaded or checked
`BumpAllocators.failed` at all, so `render_to_texture` always returned `Ok(())` regardless of
whether any stage failed. The *only* place that ever downloaded the bump buffer for CPU-side
inspection was the deprecated `render_to_texture_async` (`robust = cfg!(feature =
"debug_layers")` - off by default), which nothing in the ordinary rendering path (winit apps,
`bevy_vello`, wasm/canvas integrations) uses. The result: exactly the symptom #720 describes
("responds to user input but doesn't generate video ... no error messages are generated") - the
GPU-side robustness mechanism works exactly as designed, but nothing tells the caller it fired.

#1660's "not more than 256 Bins" theory is architecturally the *same class* of gap: `render.rs`
already has a `log::warn!` (not an `Err`) when `workgroup_counts.coarse.0 * .1 * .2 > 256` -
another internal limit that's silently non-fatal (a log line most production/wasm/embedded
consumers never see) rather than surfaced through the `Result` the caller actually checks. This
candidate's fix does not address that specific 256-bin limit (a different, unrelated buffer
constraint from `bin_data`/`binning_size`) but the same design gap it exposes is directly
relevant context for why #1660's silent failure looks superficially different from #720's while
being the same underlying "internal limit isn't surfaced as an error" pattern.

#936 (a multi-second first-frame GPU stall specifically on WebGPU/Chrome) was independently
diagnosed by the maintainers (raphlinus, DJMcNab) as a Chrome/Tint `disable_workgroup_init`
shader-compilation polyfill performance issue, unrelated to any Vello buffer/bump-allocation
limit - it is **not** part of this candidate's confirmed finding; see "What's not done" below.

## Introduced by

aad292dd "Add robustness to GPU shaders (#537)" (2024-04-01) - this is the commit that introduced
the `BumpAllocators.failed` bitmask and the GPU-side "abort this stage if a previous one failed"
mechanism whose result is never read back by the classic (non-debug_layers, non-deprecated)
rendering path. Found via `git log -S 'failed: atomic<u32>' -- vello_shaders/shader/shared/bump.wgsl`
and cross-checked against the PR's own stated goal ("Work towards #366", the direct predecessor of
issue #720). The specific gap this patch closes (`render_to_texture`/`render_full` never reading
that bitmask at all) has existed unchanged since that same commit - `render::render_encoding_full`
was calling `render_encoding_coarse(..., robust: false)` from the moment `robust` was introduced.

## The fix

`vello/src/render.rs`:
- `Render::render_encoding_coarse` gains a `download_bump: bool` parameter, independent of the
  pre-existing `robust: bool` (which now controls *only* the larger `debug_layers`-specific debug
  capture, unchanged from before). `download_bump` controls just the tiny (a few words)
  `BumpAllocators` summary buffer's CPU readback.
- `render_full`/`render_encoding_full` take and thread through a `download_bump: bool` argument,
  returning `Option<BufferProxy>` for the bump buffer accordingly.

`vello/src/lib.rs`:
- `Renderer::render_to_texture` now passes `download_bump = !self.options.use_cpu` (see the
  regression note below for why this condition is required), and, when a bump buffer was
  returned, synchronously downloads and reads its `.failed` bitmask right after the recording
  runs (the GPU-side coarse+fine stages already correctly skipped/no-op'd for the failed portion,
  by design, before this readback ever happens - this only adds *reporting*, not a behavior
  change to what gets drawn).
- A new `Error::GpuBufferOverflow(u32)` variant is returned instead of `Ok(())` whenever the
  bitmask is nonzero, with the raw failed-stages bitmask as its payload and a doc comment pointing
  at issue #720.

**A regression caught during verification, not shipped**: an initial version of this fix always
set `download_bump: true`, unconditionally. Running the project's own test suite immediately
caught a real problem: all 9 `vello_tests/tests/compare_gpu_cpu.rs` tests (which render the *same*
scene twice, once with `RendererOptions::use_cpu: false` and once `use_cpu: true`, to compare
outputs) started failing with `Error::UnavailableBufferUsed("vello.bump_buf", "download")` -
specifically for the `use_cpu: true` (CPU-shader-dispatch) render. Root cause (isolated via
targeted `eprintln!` diagnostics in `wgpu_engine.rs`, since reverted): when a shader's `select()`
resolves to `ShaderKind::Cpu` (i.e. the whole `Renderer` was constructed with `use_cpu: true`),
its bindings are resolved via `create_cpu_resources`, which never materializes a real
`wgpu::Buffer` for those bindings in the engine's `bind_map` - so a `Command::Download` for a
buffer *only* ever touched by CPU-dispatched shaders fails outright, every time, regardless of
whether the scene actually overflowed anything. This is a pre-existing limitation of the
CPU-shader-dispatch path (unrelated to this candidate's own bug), simply never exercised before
because nothing previously downloaded the bump buffer outside the `debug_layers`-gated path. The
final fix conditions the new download/check on `!self.options.use_cpu`, so `use_cpu: true` renders
keep their prior behavior exactly (no overflow detection, but also no new failure) - this is a
known, explicitly stated gap in the fix's coverage, not a silent omission (see "What's not done").

Verified against the standalone `repro/` (3/3 runs each, release build, `WGPU_BACKEND=gl` +
Mesa llvmpipe - see `output.txt`):
- Before the patch: a 200,000-draw scene on a 64x64 canvas silently drops content (marker pixel
  missing) while `render_to_texture` returns `Ok(())`; a smaller 1,000-draw control scene renders
  correctly with `Ok(())`.
- After the patch: the same 200,000-draw scene now returns `Err(GpuBufferOverflow(1))` instead of
  `Ok(())` (output image unchanged - still incomplete, since the fix is about reporting, not
  fixing the underlying fixed-size-buffer limitation); the 1,000-draw control scene is unaffected
  (`Ok(())`, correct pixels).
- Fix checked against a second point in the same bug class (not just the one triggering value):
  the boundary sweep in `output.txt` (`N` from 100,000 through 300,000) shows the same
  before/after behavior change holds across the whole range where `bump.failed` becomes nonzero,
  not just the specific `N = 200,000` the repro happens to use.

- Project test suite: `cargo test --workspace --exclude vello_sparse_tests --exclude vello_cpu
  --exclude vello_hybrid --exclude vello_common --exclude vello_bench` (scoped to the classic-GPU
  `vello` crate and its direct test/example dependents; the excluded crates are the separate
  `sparse_strips`/CPU-only rendering stack, unaffected by and irrelevant to this fix), with
  `VELLO_SKIP_LFS_SNAPSHOTS=all` (this sandbox has no `git-lfs`) and `WGPU_BACKEND=gl` +
  `LIBGL_ALWAYS_SOFTWARE=1` (no real GPU; swiftshader/Vulkan was also tried and works identically
  for the repro itself, but hit an unrelated environment-specific `SIGSEGV` in one pristine-tree
  test unrelated to this patch, so GL/llvmpipe was used for the full-suite runs for stability):
  **134 passed, 0 failed** before the patch, **134 passed, 0 failed** after (identical counts,
  same per-binary breakdown, same 1 pre-existing `#[ignore]`d test) - no regressions.

## What's not done / open questions

- The fix only detects/reports overflow for the default GPU-dispatch path
  (`RendererOptions::use_cpu: false`, what real callers use). `use_cpu: true` renders keep the
  prior fully-silent behavior - see "A regression caught during verification" above for why, and
  `vello/src/render.rs`'s updated doc comment for where this is recorded in-tree.
- The underlying limitation - fixed-size, hand-picked buffer capacities instead of a
  scene-derived or dynamically-grown allocation - is **not** fixed by this patch, only surfaced.
  A maintainer-acknowledged "sketch of a solution" already exists upstream (issue #606, referenced
  directly in #720's own thread) for actually reallocating and re-rendering on overflow instead of
  just reporting it; that is a substantially larger architecture change, explicitly out of scope
  for this session's bounded fix.
- The deprecated `render_to_texture_async`'s own *separate* bug (the real `wgpu` panic reproduced
  above, `vello/src/lib.rs:803`, "slice offset ... out of range for buffer") was observed but not
  fixed this session - it's a different code path (`DebugDownloads::map`) from the one this patch
  touches, and is already effectively deprecated/discouraged in the crate's own docs.
- #1660's specific "256 bins" `log::warn!`-not-`Err` gap (`vello/src/render.rs`'s existing "HACK"
  comment) was identified as the same class of issue but was **not** independently reproduced or
  fixed this session - it is a different specific limit (bin *count*, not `bin_data` buffer
  *capacity*) from the one this candidate's repro exercises, and #1660's own issue thread shows
  the reporters were still actively narrowing down whether bin count, a large negative transform,
  or something else entirely is the actual proximate cause for their specific case, so treating it
  as fully explained by this candidate would overstate what was verified here.
- #936 was investigated (issue history read) and found to be an already-diagnosed, unrelated
  browser/GPU-driver shader-compilation performance issue (Chrome/Tint's
  `disable_workgroup_init` polyfill) - explicitly **not** part of this candidate's confirmed
  finding, despite being one of the three reports the candidate description grouped together.
- `vello_hybrid`'s own bump-allocation/overflow handling (a separate, sparse-strips-based
  rendering backend) was not checked for whether it shares this same "detected internally but not
  surfaced" gap.
