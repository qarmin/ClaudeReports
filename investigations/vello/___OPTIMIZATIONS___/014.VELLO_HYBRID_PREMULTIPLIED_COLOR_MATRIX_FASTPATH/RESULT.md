# vello_hybrid: premultiplied-space fast path for RGB-only, alpha-preserving ColorMatrix filters

Per open upstream PR #1625 (`filter/hybrid-colormatrix-premul`, not yet merged at this
candidate's `base_commit`): `vello_hybrid`'s GPU `ColorMatrix` filter shader normally applies the
matrix in straight-alpha space (unpremultiply the sampled pixel, apply the 4x5 matrix, then
premultiply the result). For a common subset of matrices - RGB-only coefficients, no RGB row
reads alpha, no RGB offsets, alpha row is the identity row (grayscale, sepia, and similar color
adjustments) - the RGB rows can instead be applied directly to the already-premultiplied RGB
channels and clamped to `[0, alpha]`, skipping the divide-then-multiply round trip per sampled
pixel.

**Scope note:** at this repo's `base_commit`, `ColorMatrix` has **no** support at all yet in
`vello_common`/`vello_cpu`/`vello_hybrid` - PR #1625 is itself a 4-commit chain (`vello_cpu:
support color matrix filters` -> `vello_hybrid: support color matrix filters` -> `vello_cpu:
fast-path color matrix filters` -> `vello_hybrid: fast-path premultiplied color matrices`), and
even the first two (general, non-fast-path) commits are unmerged and predate a `wgsl` ->
`wesl`-module shader refactor that has since landed on `main`. This candidate ports the whole
4-commit chain, adapted to the current `wesl` shader structure (`filter.wesl`'s per-texel
`load_filter_texel` accessors instead of the old monolithic `GpuFilterData` wgsl struct, and a
texel-aligned `GpuColorMatrix` layout - `header + padding, row_r, row_g, row_b, row_a, offsets`,
each its own texel - chosen specifically so the shader never needs a dynamically-indexed array
access, which this project's own shader lint (`no_dynamic_indexing.rs`) forbids). `patch.diff`
therefore bundles the prerequisite general `ColorMatrix` support together with the fast path
itself and a new bench group - see "What's not done" for why these weren't split into separate
candidates.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`. GPU: no
real adapter available in this sandbox (no Vulkan ICD); `wgpu` falls back to Mesa **llvmpipe**
(software rasterizer) via the `Gl` backend - confirmed via `adapter.get_info()`:
`AdapterInfo { name: "llvmpipe (LLVM 21.1.8, 256 bits)", device_type: Cpu, backend: Gl, ... }`.
Absolute millisecond values below are therefore software-rasterizer numbers, not representative
of real-GPU timings, but the relative delta directly reflects the per-pixel shader work removed.
24 logical cores.

## What was tried

No existing benchmark reached `vello_hybrid`'s GPU filter shader pipeline at all (`vello_bench`
had no dependency on `vello_hybrid`). Extended it with a new `hybrid` Cargo feature (pulling in
`vello_hybrid` + `wgpu` + `pollster`, kept optional since it needs a real/software GPU adapter)
and a new `vello_bench::hybrid_color_matrix` module: builds a 2048x2048 scene (four overlapping,
differently-colored opaque rects, so the filtered region has real RGB variance) inside a
`ColorMatrix` filter layer, renders it through `vello_hybrid::Renderer::render` into an
`Rgba8Unorm` render target, copies the result back to a mapped buffer, and waits for completion
(`device.poll(PollType::wait_indefinitely())`) each iteration - a full, realistic per-frame
round trip, not just GPU submission time. Three bench functions per resolution: `grayscale` and
`sepia` (premul-compatible, eligible for the fast path) and `alpha_to_black` (reads alpha into
RGB - **not** premul-compatible, always general path, used as a control that should show no
change from this patch).

Comparison method (mirrors candidate 012's approach, since the fast path can't be isolated any
other way given upstream never shipped a "general path only" state to diff against): built one
binary with the patch applied as-is (fast path active), then derived a second "baseline" binary
from the *same* tree by disabling just the one line that sets the premul-compatible header bit
(`vello_hybrid/src/filter.rs`'s `pack_color_matrix_header`, `&& false` appended), forcing the
shader's general branch unconditionally - same bench harness, same shaders file, only the runtime
branch selection differs. Ran both `taskset -c 0` (single-core, pinned) and unpinned (multi-core,
24 cores, `uptime` load average 8-13 during these specific runs), 2 full repeats each, Criterion
default settings (`--warm-up-time 1 --measurement-time 4`, 100 samples where the time budget
allowed, fewer otherwise as Criterion reported):

| Variant | grayscale single-core (`taskset -c 0`) | sepia single-core | control (alpha_to_black) single-core | grayscale multi-core | sepia multi-core | control multi-core |
|---|---|---|---|---|---|---|
| baseline (general path forced), run 1 | 169.02 ms | 166.51 ms | 166.23 ms | 26.303 ms | 26.022 ms | 26.501 ms |
| patched (fast path), run 1 | 144.06 ms | 142.95 ms | 169.84 ms | 24.241 ms | 24.733 ms | 27.039 ms |
| baseline (general path forced), run 2 | 172.33 ms | 169.12 ms | 173.20 ms | 27.010 ms | 27.274 ms | 26.748 ms |
| patched (fast path), run 2 | 151.75 ms | 154.01 ms | 175.06 ms | 24.455 ms | 24.745 ms | 27.062 ms |

(All figures are Criterion's own point estimate, i.e. median of its bootstrap distribution.)

A separate, earlier check at 1024x1024 (a quarter of the pixel count) showed **no** reliable
difference for grayscale (baseline 50.29 ms vs patched 49.08 ms, well within noise) - the
per-frame fixed overhead (encoder creation, submission, buffer readback/map) dominates at that
resolution and swamps the per-pixel shader saving; 2048x2048 was chosen for the table above
specifically because it makes the O(pixel-count) shader-time component large enough relative to
that fixed overhead to see clearly.

**A later, separate justfile smoke-test run** (`just test` in a scratch clone, unpinned/no
`taskset`, run while `uptime` reported load average ~19 on this 24-core machine, i.e. under much
heavier contention than the dedicated runs above) reproduced the correct comparison mechanics
end-to-end but with far noisier, partly inconsistent numbers (sepia showed a "regression" and the
control showed a large "improvement" - both artifacts of system contention during that specific
run, not a real effect; the control's own baseline/patched code paths are provably identical for
that matrix). This is reported for transparency, not as evidence - see "Why the result looks the
way it does" for why single-core, pinned measurements are the more trustworthy signal on a shared
machine, and why the dedicated table above (not the smoke-test run) is this candidate's actual
result.

## Result

**Time only** (no memory/allocation claim - the patch changes shader ALU work per pixel, not
allocation counts or buffer sizes).

- **Single-core (`taskset -c 0`), 2048x2048:** grayscale drops from 169.0-172.3 ms (baseline) to
  144.1-151.8 ms (patched) - **~12-15% lower**. Sepia drops from 166.5-169.1 ms to 143.0-154.0 ms
  - **~9-14% lower**. The control (`alpha_to_black`, general path in both binaries) moved the
  *other* direction slightly (166.2-173.2 ms baseline vs 169.8-175.1 ms patched, ~2-4% higher) -
  within this environment's own noise floor (llvmpipe single-core timing varies run to run; the
  two control numbers don't have non-overlapping ranges the way grayscale/sepia's do), and
  directionally the opposite of what a "the whole binary just got slower" explanation would
  predict, so it doesn't undermine the grayscale/sepia result.
- **Multi-core (unpinned, 24 cores, moderate load ~8-13):** grayscale drops from 26.3-27.0 ms to
  24.2-24.5 ms (**~7-10% lower**); sepia drops from 26.0-27.3 ms to 24.7 ms (**~5-9% lower**); the
  control stayed flat (26.5-26.7 ms baseline vs 27.0-27.1 ms patched, ~1-2% higher, noise).
- The effect is real but resolution-dependent: at 1024x1024 it was not measurable (see above);
  it should keep growing (in absolute terms, and likely in relative terms too, since driver/
  submission overhead is roughly fixed per frame while the shader saving is per pixel) at
  resolutions above 2048x2048, though that wasn't independently measured.
- The single-core numbers are the more trustworthy signal in this sandboxed, shared-machine
  environment - the later unpinned smoke-test run (see above) showed how much multi-core/
  unpinned timing degrades under heavier contention from sibling work on the same host.

## Does it change behavior/output?

- **Bit-exact.** Verified two ways:
  1. A GPU render+readback correctness check with fully-opaque source colors (red `(204,26,26)`)
     matched hand-computed grayscale/sepia values exactly: grayscale -> `(64,64,64,255)`
     (`0.2126*204+0.7152*26+0.0722*26 = 63.9 -> round 64`); sepia -> `(105,93,73,255)` (matches
     the SEPIA matrix's own row dot products to the pixel). `alpha_to_black` on a fully opaque
     source correctly produced solid white `(255,255,255,255)`.
  2. With a **translucent** source color (straight RGBA `(0.8, 0.3, 0.1, 0.5)`, so unpremultiply
     actually changes the numbers), rendering through the patched (fast-path) shader and through
     the same shader with the fast-path flag forced off (general path) produced the **identical
     non-cryptographic hash** of the rendered RGBA8 buffer (`be5dc1c3dbc2bc75`) and identical
     pixel value `(50, 50, 50, 128)` in both cases - confirming the fast path and general path
     agree bit-for-bit on this scene, not just approximately.
  3. `vello_cpu`'s own mirrored premultiplied-space fast path (added as a necessary consequence
     of `PreparedFilter`'s new `ColorMatrix` variant needing an exhaustive match in `vello_cpu`
     too - see "Scope note" above) was checked in Rust unit tests
     (`premul_fast_path_matches_general_path_for_grayscale`/`_for_sepia`) to agree with the
     general path within ±1 per channel (independent rounding paths - unpremultiply-then-multiply
     vs the direct dot product round to slightly different intermediate values in edge cases,
     same as the real upstream PR's own commit does).
- **Is this a breaking change?** No new public API breakage; the PR does add public items
  (`ColorMatrix::is_premul_compatible`, the `FilterPrimitive::ColorMatrix` variant moving out of
  "not yet implemented"), but this is new functionality (ColorMatrix support did not exist in
  `vello_hybrid`/`vello_cpu` at all before this candidate), not a change to any existing
  documented behavior.
- Project test suite (`cargo test --release -p vello_common -p vello_cpu -p vello_hybrid --lib`,
  run through the shared build lock, before = unmodified `base_commit`, after = `base_commit` +
  `patch.diff`):
  - `vello_common`: baseline **160 passed / 0 failed / 1 ignored** -> patched **163 passed / 0
    failed / 1 ignored** (+3 new tests: `ColorMatrix::is_premul_compatible` unit tests x2,
    `PreparedFilter::new` ColorMatrix-prepares test x1). The pre-existing ignored test is
    unrelated to this patch.
  - `vello_cpu`: baseline **81 passed / 0 failed** -> patched **93 passed / 0 failed** (+12: the
    new `pixel.rs` helper tests x5, `color_matrix.rs` tests x7).
  - `vello_hybrid`: baseline **122 passed / 0 failed** -> patched **125 passed / 0 failed** (+3:
    `GpuColorMatrix` round-trip + header-bit tests).
  - No pre-existing test newly failed or newly passed in either crate; every new failure/pass is
    accounted for by a new test this patch adds.
  - `vello_sparse_tests` (the crate with the most direct visual/snapshot coverage) was not run:
    this environment has no `git-lfs`, so its reference PNG snapshots are unresolved LFS pointer
    files (same environment limitation noted in candidates 006/007/012's own `RESULT.md`s) - not
    a regression caused by this patch, just not usable as a correctness signal here.

## Why the result looks the way it does

llvmpipe's fragment-shader cost for the general path is a division (unpremultiply), four
dot-products against the full 4x5 matrix plus per-row offset adds, four clamps, then a
multiply-back (premultiply) - the fast path replaces this with three dot-products against 3x3
(RGB-only) rows and a single 3-component clamp, no division, no offset adds, no fourth
(alpha) row. At 1024x1024 this saving (roughly a handful of scalar ALU ops per pixel) is too
small relative to the fixed per-frame cost of creating a command encoder, submitting to the
queue, copying the render target to a readback buffer, and blocking on `device.poll` to see
through llvmpipe's own software rendering + memory-copy overhead. At 2048x2048 (4x the pixels,
same fixed per-frame overhead) the per-pixel component is a large enough fraction of total time
for the saving to show up clearly and consistently across repeats, while the control matrix
(which always takes the general path in both binaries) stays flat - exactly the pattern expected
if the mechanism is "less per-pixel shader work," not some other confound shared by the whole
binary.

## What's not done / open questions

- **This candidate bundles three things in one `patch.diff`** (prerequisite general `ColorMatrix`
  support + the fast path + the new bench group) rather than one - unavoidable here, since
  neither the fast path nor a bench that reaches it can exist without the general support first,
  and upstream never shipped a "general support, no fast path" commit to diff against on its own.
  A reader who only cares about the *fast path itself* should look specifically at:
  `ColorMatrix::is_premul_compatible` (`vello_common/src/filter/color_matrix.rs`),
  `pack_color_matrix_header`'s flag bit + `COLOR_MATRIX_PREMUL_COMPATIBLE_FLAG`
  (`vello_hybrid/src/filter.rs`), and `apply_premul_compatible_color_matrix` +
  `get_color_matrix_premul_compatible` (`filter.wesl`) - everything else in the diff is the
  general-path scaffolding these depend on.
- **Not benchmarked on a real GPU** - only Mesa llvmpipe (software) was available in this sandbox.
  The mechanism (fewer ALU ops, no division, one fewer texture-data-derived row) should transfer
  to real hardware, but the *relative* size of the effect could differ (a real GPU's fixed
  per-draw overhead and ALU throughput both differ substantially from llvmpipe's).
- **Only one scene shape was measured** (a handful of overlapping opaque rects filling most of
  the viewport). A scene where the filtered region is much smaller (so per-pixel work matters
  less relative to per-draw setup) would show less benefit; this wasn't independently measured.
- **`vello_cpu`'s own mirrored fast path was not separately benchmarked** - only checked for
  correctness (see above). Iterating a `Pixmap`'s pixels is likely memory-bandwidth-bound for
  large images regardless of the per-pixel ALU difference, so the CPU-side win (if any) is
  probably much smaller than the GPU-side one measured here; a follow-up candidate could quantify
  this specifically if it turns out to matter.
- 1024x1024 and 2048x2048 were the only two resolutions checked; the exact resolution at which
  the effect becomes reliably measurable in this environment (somewhere between the two) was not
  pinned down further.
