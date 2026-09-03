# VELLO_HYBRID_STAGING_BELT_TEXTURE_UPLOAD

`vello_hybrid`'s wgpu backend uploaded the alpha/encoded-paints/filter/gradient textures every
frame via `Queue::write_texture`, which internally allocates a fresh CPU-side staging buffer per
call. Open PR [#1532](https://github.com/linebender/vello/pull/1532) replaces this with a
`wgpu::util::StagingBelt` (a reusable ring buffer of staging chunks recycled across frames) and
reports "For 1200 frames of GhostScript tiger, ~479ms -> ~155ms spent in `render`" via a profiler
on real hardware. This candidate hand-ports that PR's texture-upload change (the four
`upload_*_texture` functions plus threading a `CommandEncoder` through `prepare()`) onto current
`HEAD`, since the PR's own diff (written against an older, since-refactored `wgpu.rs`) no longer
applies cleanly to today's `wgpu/mod.rs`. The PR's separate change to reuse (rather than recreate)
the strips/filter-instance/clear-slot-indices *buffers* was left out - out of scope for this
candidate, which is scoped to `write_texture` specifically.

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

## Environment caveat (read before the numbers)

No real GPU was available in the sandbox this was measured in (no `/dev/dri`, no `vulkaninfo`).
All measurements below use a *software* Vulkan adapter: Google SwiftShader (`AdapterInfo { name:
"SwiftShader Device (Subzero)", device_type: Cpu, backend: Vulkan }`), reached via
`VK_ICD_FILENAMES` pointing at the `libvk_swiftshader.so` bundled with a local Playwright/Chromium
install (same technique as `___BUGHUNT___/026`). This is a CPU-based software rasterizer, not a
real GPU driver's staging-heap allocator - the PR's own claimed win is about a *real* GPU driver's
staging-buffer allocation cost, which SwiftShader's software implementation does not necessarily
reproduce the same way. The numbers below should be read as "what this patch does under a slow,
CPU-emulated Vulkan backend", not as a confirmation or refutation of the PR's own hardware numbers.

The bench (`bench/src/main.rs`) is a new standalone crate, since nothing existing in this repo
drives `vello_hybrid`'s wgpu backend through a real `Device`/`Queue` in a scriptable, repeated-frame
loop (`examples/render_to_file.rs` renders exactly one frame). It renders the Ghostscript Tiger SVG
(`examples/assets/Ghostscript_Tiger.svg`) at 5x scale, matching the PR's own scene, and times only
the CPU-side `Renderer::render()` call (encoding + any staging allocation/copy) for each of N
frames, discarding the first 10% as warmup.
Every frame is hashed (SHA-256 of the final frame's RGBA8 readback); the hash is printed and
compared baseline vs. patched.

Two loop-pacing regimes were tried because they told different stories (see "Why the result looks
the way it does"):

- **Frame-paced**: `device.poll(PollType::wait_indefinitely())` after every frame's submit,
  emulating a real windowed app's vsync/present pacing (the CPU never gets more than ~1 frame
  ahead of the GPU).
- **Unthrottled**: `device.poll(PollType::Poll)` (non-blocking) after every submit - the CPU keeps
  submitting new frames regardless of whether SwiftShader has finished the previous one.

## What was tried

### Frame-paced (`PollType::wait_indefinitely`) - render() CPU time only, Ghostscript Tiger

Multi-core (unpinned, 24 cores):

| Variant | frames | min us | p50 us | mean us |
|---|---|---|---|---|
| baseline run1 | 100 | 214.0 | 358.5 | 487.2 |
| baseline run2 | 100 | 233.2 | 407.6 | 545.0 |
| baseline run3 | 60 | 270.9 | 369.4 | 482.5 |
| baseline run4 | 60 | 317.5 | 362.2 | 441.7 |
| patched run1 | 100 | 268.3 | 368.5 | 515.8 |
| patched run2 | 100 | 242.8 | 418.2 | 514.4 |
| patched run3 | 60 | 165.5 | 338.8 | 436.0 |
| patched run4 | 60 | 253.5 | 378.8 | 529.3 |

Baseline/patched runs were alternated (B, P, B, P, B, P, B, P), not run back-to-back, per
MEASUREMENT.md 2b/2c. p50 across baseline runs: 358.5/407.6/369.4/362.2 (mean of p50s ~374.4us).
p50 across patched runs: 368.5/418.2/338.8/378.8 (mean of p50s ~376.1us). The two distributions
overlap completely - no consistent direction.

Single-core (`taskset -c 0`, 30 frames - SwiftShader's own internal worker threads contend with
the pinned core, so absolute numbers here are not comparable to the multi-core ones above, only
baseline-vs-patched within this row):

| Variant | min us | p50 us | mean us | p95 us | max us |
|---|---|---|---|---|---|
| baseline | 221.0 | 233.6 | 241.1 | 281.6 | 426.0 |
| patched | 234.9 | 239.7 | 248.7 | 311.2 | 328.6 |

Within a few percent either way - no confident win or regression.

### Unthrottled (`PollType::Poll`, non-blocking) - multi-core, 200 frames

| Variant | min us | p50 us | mean us |
|---|---|---|---|
| baseline run1 | 630.5 | 741.7 | 756.6 |
| baseline run2 | 436.3 | 603.8 | 690.5 |
| patched run1 | 460.5 | 2619.6 | 2567.4 |
| patched run2 | 417.7 | 2347.7 | 2284.5 |

Patched is ~4x slower at p50/mean here (min is comparable - a handful of individual frames do hit
the fast/reused-chunk path). Reproduced twice, same direction both times.

Every one of the ~16 runs above (baseline and patched, both loop-pacing regimes, both
single-core and multi-core) printed the identical output hash:
`20094948c41fb4b7fac7204ac57dd28d1bfc44a0f897385340d63bca6353d79b`.

## Result

**Time.** Under a realistic frame-paced render loop (the only regime the PR's own claim is
actually about), this patch is a wash on this software-Vulkan backend: p50/mean overlap
completely between baseline and patched across 4 alternated runs each, single- and multi-core.
No win, no regression, within this measurement's own noise floor (see "Why" below for why this
plausibly differs from the PR's own reported ~3x win). Under an unthrottled loop that lets CPU
submissions outrun the (slow, software) GPU, this patch is a reproducible ~4x *regression*
(p50/mean; min is comparable) - a genuine, characterized failure mode of the staging-belt
recycling design under this specific condition, not noise (same direction in both runs, ratio
consistent to within ~10%).

**Peak RSS / allocation count**: not measured for this candidate (out of scope - the mechanism
being tested is upload-buffer *reuse*, and the interesting question was CPU-side call latency,
which is what the PR itself reports).

## Does it change behavior/output?

- **Bit-exact?** Yes. Every baseline and patched run (16 total, both loop-pacing regimes) produced
  the identical SHA-256 hash of the final rendered frame's RGBA8 readback:
  `20094948c41fb4b7fac7204ac57dd28d1bfc44a0f897385340d63bca6353d79b`.
- **Is this a breaking change?** No public API changes (`prepare`/`upload_*_texture` are private
  to the wgpu backend module).
- Project test suite: `cargo test -p vello_hybrid` (the only test suite that runs without a real
  GPU/snapshot data - see "What's not done"): **122 passed, 0 failed** before, **122 passed, 0
  failed** after (identical). No new failures.

## Why the result looks the way it does

The staging belt's own documented protocol is: `allocate()`/`write_buffer()` -> `finish()` ->
submit -> `recall()`. `recall()` issues `map_async()` on the just-closed chunks, but that mapping
only completes (making the chunk reusable via `free_chunks`) once the device is polled *again*
after `recall()` was called, and only once the GPU has actually finished reading from that chunk.
In a **frame-paced** loop (block until each frame's GPU work completes before starting the next),
that condition is met well within one frame, so chunk reuse is reliable and the numbers land
where a genuinely-reused small allocation vs. a `queue.write_texture`-managed one would be expected
to land: close to each other, since both are small, cheap, well within noise on a
compute-shader-dominated frame. This is consistent with the PR's own numbers being modest in
absolute terms per frame (~0.4ms baseline -> ~0.13ms patched, i.e. still sub-millisecond) - on a
compute-bound frame the CPU-side staging cost was apparently already a small fraction of total
frame time even before the fix, on real hardware; here, SwiftShader's own per-frame compute cost
(a full software rasterization of the tiger scene) dominates far more heavily (hundreds of
milliseconds of real GPU-side wall time per frame, per the `real` timings recorded while building
this bench), which most likely buries whatever the staging-buffer allocation saving actually is on
a real GPU.

In the **unthrottled** case, submissions queue up faster than SwiftShader (a slow, single
logical device processing everything serially) can drain them. `recall()`'s `map_async()` requests
pile up without ever completing before the next frame's `allocate()` call needs a chunk, so
`free_chunks`/`active_chunks` stay empty and every single frame falls through to
`device.create_buffer()` for a brand-new **4MB** chunk (`chunk_size` from the PR's own choice) with
`mapped_at_creation: true` - a real, uncached GPU buffer allocation, four times larger than
anything actually being uploaded (the tiger scene's alpha/paint/gradient data is on the order of
tens of KB, not megabytes). This is strictly worse than the baseline's `queue.write_texture`,
which lets wgpu-core manage its own right-sized internal staging pool without depending on
the caller polling at just the right cadence. This is a genuine, reproducible characteristic of
the staging-belt approach under CPU-ahead-of-GPU submission patterns - plausible for a headless
batch renderer, a benchmark loop (like this one without the added `PollType::wait_indefinitely`),
or any embedder whose CPU is simply much faster than its target GPU - not specific to the software
backend used here (SwiftShader just makes the underlying mechanism easy to trigger and observe).

## What's not done / open questions

- No real GPU was available to check the PR's own claimed ~3x win directly; the frame-paced
  numbers here neither confirm nor refute it - see the environment caveat above.
- `vello_sparse_tests`'s hybrid rendering snapshot tests (`hybrid_schedule.rs`, `hybrid_depth.rs`)
  were not run: their reference PNGs are Git LFS pointer files (`git-lfs` isn't installed in this
  sandbox), so they'd fail identically before and after this patch for reasons unrelated to it.
  `cargo test -p vello_hybrid`'s 122 tests (scheduler/allocator logic, no GPU device needed) were
  run instead and are unaffected either way.
- The PR's own separate change (reusing rather than recreating the strips/filter-instance/
  clear-slot-indices *buffers* every frame, via `queue.write_buffer_with` today) was intentionally
  left out of this candidate - it's a related but distinct mechanism (buffer reuse, not
  `write_texture`'s staging allocation) and would be its own candidate.
- The unthrottled-regression finding suggests the PR's fixed 4MB `chunk_size` may be oversized for
  `vello_hybrid`'s actual per-frame upload volume, and/or that the belt would benefit from a
  fallback to a direct `write_texture`-style path when no free chunk is available yet, instead of
  always allocating a full new `chunk_size` buffer - noted in `IDEAS.md` rather than investigated
  further here (out of scope for testing the PR as proposed).
