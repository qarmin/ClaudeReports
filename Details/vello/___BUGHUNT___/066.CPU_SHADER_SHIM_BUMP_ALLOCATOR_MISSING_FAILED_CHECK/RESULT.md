# CPU_SHADER_SHIM_BUMP_ALLOCATOR_MISSING_FAILED_CHECK

`vello_shaders/src/cpu/tile_alloc.rs:52`, `path_count.rs:143/146/147/150`, and `binning.rs:102`
write into bump-allocated slices (`tiles`/`seg_counts`/`bin_data`, capacities fixed in
`vello_encoding/src/config.rs`'s `BufferSizes::new`) with no bounds check of their own - unlike the
real GPU path, where wgpu/naga's storage-buffer bounds checks silently clamp an equivalent WGSL OOB
index, *and* the WGSL shaders themselves already reset their bump-allocator offset to 0 and OR a
`STAGE_*` bit into `BumpAllocators::failed` whenever a draw's demand would overflow a buffer's fixed
capacity. This builds directly on this workspace's own
047.PATH_TILING_CPU_SHIM_ASSERT_AND_INDEX_WRITE, which already reproduced real end-to-end panics at
exactly `tile_alloc.rs:52` (many overlapping large-bbox draws) and `path_count.rs:150` (one path,
many long diagonal lines) but never gave those two their own Confirmed candidate since 047's own
target was `path_tiling.rs` specifically. This candidate confirms both, plus a third, newly
constructed `binning.rs:102` overflow, root-causes all three, and evaluates + verifies a bounded fix
mirroring the GPU path's clamp-and-`failed` behavior.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `tile_alloc.rs`/`path_count.rs`/`binning.rs` against their WGSL counterparts (`tile_alloc.wgsl`/`path_count.wgsl`/`binning.wgsl`) and `shader/shared/bump.wgsl`, to establish exactly what the GPU-side safety mechanism the CPU shim is missing looks like | Reading only | `tile_alloc.wgsl` and `binning.wgsl` each explicitly check `offset/chunk_offset + count > config.*_size`, reset the offset to 0, and `atomicOr(&bump.failed, STAGE_*)` before ever indexing into the buffer. `path_count.wgsl` has no `STAGE_PATH_COUNT`/`bump.failed` write of its own for its `seg_counts` write, but does have an explicit `if seg_ix < config.seg_counts_size` guard before the write (a plain bounds check, not a `failed`-flag report). All three of these safety checks are completely absent from their CPU-shim equivalents - confirming none of them are a coincidence of translation, but a genuine gap in the CPU shim specifically. Separately, `path_tiling_setup.rs` (a different, sibling CPU-shim file, not part of this candidate's target set) contains the literal comment `// binding 2 is ptcl, which we would need if we propagate failure` - direct, contemporaneous acknowledgement from the code itself that the CPU shim never implements `bump.failed` propagation anywhere. |
| Real end-to-end reproduction via `vello::Scene`/`BezPath`/`Stroke` (public API) -> a real `wgpu::Device` (SwiftShader/Vulkan software ICD) -> `vello::Renderer::render_to_texture` with `RendererOptions::use_cpu: true`, `catch_unwind` around the call - reusing 047's own two constructions (many overlapping full-canvas diagonal strokes; one path with many full-diagonal `line_to`s) for `tile_alloc.rs`/`path_count.rs`, plus a newly constructed third scene for `binning.rs` (see "Root cause" for why it needs a different shape) | Release build, `WGPU_BACKEND=vulkan`, `RUST_BACKTRACE=1`, 3/3 runs each | All three panic, deterministically, at exactly the targeted file/line: `tile_alloc.rs:52` (`the len is 2097152 but the index is 2097152`), `path_count.rs:150` (same), `binning.rs:102` (`the len is 262144 but the index is 262288`). See `output.txt`/`repro/`. |
| Best-effort commit history lookup (`git log --follow`/`git show --stat`) for when the CPU shim (and the GPU-side safety mechanism it's missing) were introduced | `git log`, no build | See "Introduced by" below - the CPU shim (`d2c2a267`, Sep 2023) was added roughly 8 months *after* the GPU-side `bump.failed` mechanism it never picked up (`1e8d194b`, Jan 2023, confirmed an ancestor of `d2c2a267` via `git merge-base --is-ancestor`). |
| Attempt + verify a bounded fix mirroring the GPU shaders' own offset-reset-and-`failed` pattern (plus a defensive bounds guard on the resulting writes, since the CPU shim has no wgpu/naga-style implicit storage-buffer clamp to fall back on for the "a single draw's own demand alone exceeds capacity" case) in all three target files | Release build, same 3 repro scenes, 3/3 runs before and after, project test suite before/after | See "The fix" below - the three originally-targeted panics are gone (verified 3/3), but each scene still panics one stage later in an already-known, unfixed sibling file with the identical bug class. |

## Status

**Confirmed.** Reproduces deterministically, 3/3 runs, release build, `WGPU_BACKEND=vulkan`
(SwiftShader software ICD) - see `output.txt` for the exact captured panics, `repro/` for the
runnable reproduction.

**Fully reproducible**: **Yes**. The repro drives the real, public entry point
(`vello::Renderer::render_to_texture` with `RendererOptions::use_cpu: true`) with real
`Scene`/`BezPath`/`Stroke` geometry through a real (software) `wgpu::Device` - this is the exact
same call a real application using `use_cpu: true` makes. There is no second participant being
faked here (see `CONFIRMATION.md` section 11): the "adversarial" input is a single external
artifact (an application-supplied scene with unusually many/large draws), the same kind of input a
real caller of the public API can genuinely construct, not bytes/state standing in for what some
other process would send.

## Class

crash (index-out-of-bounds panic; the GPU path handles the identical condition safely by design)

## Root cause

`vello_encoding::config::BufferSizes::new` hand-picks fixed capacities for several
"bump-allocated" buffers used across the render pipeline (`tiles`, `lines`, `seg_counts`,
`segments`: `1 << 21` each; `bin_data`: `1 << 18`; `ptcl`: `1 << 23`; `blend_spill`: `1 << 20`) -
sized, per the code's own comment, "to accommodate the vello test scenes as well as paris-30k",
not derived from the actual scene. On the real GPU path, every WGSL shader that claims a slice of
one of these buffers (an atomic add against a running total in `BumpAllocators`) explicitly checks
whether its claim exceeds the buffer's declared capacity (`config.tiles_size` /
`config.binning_size` / etc.); if so, it resets its own offset to 0 and (for `tile_alloc.wgsl`/
`binning.wgsl`) ORs a `STAGE_*` bit into `BumpAllocators::failed`, and *also* relies on wgpu/naga's
own storage-buffer bounds checks to silently clamp/discard any write that's still out of range
after that (e.g. a single draw object whose own demand alone exceeds the buffer, even after the
offset reset) - the shader itself is coded so overflow degrades to wrong-but-safe output, never a
crash. `vello_shaders/src/cpu/tile_alloc.rs`, `path_count.rs`, and `binning.rs` (the CPU-shader
shim used whenever `RendererOptions::use_cpu` is `true`) reimplement the *arithmetic* of these same
shaders line-for-line, but never reimplement either safety mechanism: no capacity check before
handing out an offset, no `bump.failed`, and (being plain Rust slices, not WGSL storage buffers) no
implicit bounds-check fallback either - so the exact same over-capacity condition that degrades
gracefully on GPU becomes a hard `index out of bounds` panic on CPU, in three different files:

- **`tile_alloc.rs:52`**: `tiles[(offset + i) as usize] = Tile::default();` - `offset` is
  `bump.tile`'s running total *before* this draw's own tile count is added; once the cumulative
  total across all draw objects so far exceeds `config.tiles_size` (`1 << 21` = 2,097,152), this
  indexes past the end of the `tiles` slice.
- **`path_count.rs:150`** (plus the closely related `tile[...]` writes at lines 129/143/146/147,
  which index into the *same* `tiles`-sized buffer using offsets ultimately derived from
  `tile_alloc.rs`'s own, equally-unchecked `path.tiles`): `seg_counts[(seg_base + i - imin) as
  usize] = seg_count;` - `seg_base` is `bump.seg_counts`'s running total before this line's own
  segment count is added; once the cumulative total exceeds `config.seg_counts_size` (also
  `1 << 21`), this indexes past the end of the `seg_counts` slice.
- **`binning.rs:102`**: `bin_data[ix as usize] = element_ix as u32;` - `ix` is
  `config.layout.bin_data_start + chunk_offset[bin_ix]`, where `chunk_offset[bin_ix]` is
  `bump.binning`'s running total (summed per-bin, across every draw object that touches that bin)
  before this bin's own share is added; once that per-bin cumulative total exceeds
  `config.binning_size` (`bin_data`'s capacity minus `bin_data_start`, i.e. up to `1 << 18` =
  262,144 minus a small `layout`-dependent prefix), this indexes past the end of the `bin_data`
  slice. This buffer's capacity is 8x *smaller* than `tiles`/`seg_counts`'s, but each bin is
  256x256 pixels (16x16 tiles) - normally many more tiles than bins get consumed per unit of
  drawing, so a typical large/complex scene exhausts `tiles`/`seg_counts` long before `bin_data`.
  This candidate's third repro scene defeats that by repeating the exact same tiny (2x2 pixel)
  stroked square - positioned to straddle a point where four bins meet - 90,000 times: each
  repeat costs `bump.binning` 4 (one per bin corner touched) against only 4 tiles of `bump.tile`
  cost (ratio 1:1, instead of the usual up-to-256:1 tiles-to-bins ratio for an ordinary shape),
  cheaply overflowing `bin_data` (262,144 capacity) while `tiles` (2,097,152 capacity) stays far
  under budget.

## Introduced by

d2c2a267 "Add CPU shaders" (2023-09-19) - the commit that added the entire CPU-shader-shim pipeline
(`tile_alloc.rs`, `path_count.rs`, `binning.rs` included, then under `src/cpu_shader/`), with no
`bump.failed`/capacity-check logic in any of the three files, from that commit onward. The GPU
side's own `bump.failed`/`STAGE_*`/offset-reset mechanism (`shader/bump.wgsl`,
`shader/tile_alloc.wgsl`, `shader/binning.wgsl`) predates this by about 8 months
(1e8d194b "initial GPU side work for robust memory", 2023-01-17; confirmed an ancestor of
d2c2a267 via `git merge-base --is-ancestor`) - so the omission was not a later regression, the CPU
shim simply never picked up a safety mechanism the GPU shaders it was modeled on already had at the
time it was written.

## The fix

Added, to `vello_shaders/src/cpu/tile_alloc.rs`, `path_count.rs`, and `binning.rs`, the same
two-part mechanism their WGSL counterparts already use:

1. **Capacity check + offset reset + `bump.failed`** (mirroring `tile_alloc.wgsl`/`binning.wgsl`
   exactly): before handing out an offset/chunk-offset that a draw/bin's own demand would push past
   the buffer's declared capacity (`config.tiles_size`/`config.binning_size`), reset that offset to
   0 and OR the corresponding `STAGE_TILE_ALLOC`/`STAGE_BINNING` bit (new `pub(crate)` constants in
   `cpu/util.rs`, mirroring `shader/shared/bump.wgsl`'s `STAGE_*` values) into `bump.failed`.
   `path_count.rs`'s own `seg_counts` write mirrors `path_count.wgsl` instead (which has no
   `STAGE_PATH_COUNT`/`bump.failed` write of its own): just an explicit `if seg_ix <
   config.seg_counts_size` bounds check before the write, silently dropping it otherwise.
2. **Defensive `.get_mut()`/bounds-checked writes** on every write this candidate's target lines
   perform (`tiles[...]`, `seg_counts[...]`, `bin_data[...]`, and the closely-related `tile[...]`
   writes in `path_count.rs` that share the same offset arithmetic) - needed because, unlike WGSL
   storage buffers, a plain Rust slice has no implicit bounds-check fallback for the case where a
   *single* draw/bin/line's own demand alone still exceeds capacity even after the offset reset
   (e.g. one path whose own bbox alone needs more tiles than `tiles_size` allows).

Verified: re-running `repro/`'s three scenarios against the patched tree, 3/3 runs, no longer
panics at `tile_alloc.rs:52`, `path_count.rs:150`, or `binning.rs:102` - confirming the specific
fix works as designed for all three targeted sites (see `output.txt`). **However**, each of the
three scenes still panics overall, now one pipeline stage later - `coarse.rs:68` (was
`tile_alloc.rs:52`), `coarse.rs:382` (was `path_count.rs:150`), `path_tiling.rs:164` (was
`binning.rs:102`) - because those two sibling files have the *same* unfixed "unchecked
bump-allocator write" bug class (already noted, unfixed, in this workspace's own
047.PATH_TILING_CPU_SHIM_ASSERT_AND_INDEX_WRITE `RESULT.md`'s "What's not done"), and this
candidate's fix was intentionally scoped to only its three named target files, not a systemic
sweep of the whole CPU shim. This is expected, not a fix failure: the corrupted/clamped state that
`tile_alloc.rs`'s fix now produces when `bump.failed` is set (e.g. every over-capacity draw object
getting `path.tiles = 0`) is consumed downstream by stages that still assume it's always
well-formed, because none of them check `bump.failed` before running (unlike GPU, where
`path_count_setup.wgsl`/`path_tiling_setup.wgsl`/`coarse.wgsl` all check `bump.failed` and skip or
gate their own work when a prior stage already failed).

- Project test suite: `vello_shaders`/`vello_encoding`/`vello` `--lib` (release): 31 passed / 0
  failed before, 31 passed / 0 failed after (all 31 real tests are in `vello_encoding`; `vello`/
  `vello_shaders` have 0 unit tests) - identical, no regressions. `vello_tests` (with
  `VELLO_SKIP_LFS_SNAPSHOTS=all`, see `___BUGHUNT___/README.md` for why this sandbox needs that
  flag): 64 passed / 0 failed / 1 ignored before, identical after. See `output.txt` for the full
  breakdown.

## What's not done / open questions

- **This is a scoped, not systemic, fix.** As demonstrated directly above, the identical bug class
  still lives, unfixed, in at least `coarse.rs` (`ptcl` buffer, panics observed at both line 68 and
  line 382 in this session) and `path_tiling.rs` (`segments` buffer, line 164) - both already
  flagged as open in 047's own "What's not done". A future candidate could reasonably fix those two
  the same way this one fixed `tile_alloc.rs`/`path_count.rs`/`binning.rs`, and/or go further and
  add the GPU path's other half of the mechanism (having `path_count_setup.rs`/
  `path_tiling_setup.rs`/`coarse.rs`'s CPU-shim equivalents actually check `bump.failed` and skip
  their own work when a prior stage already failed, the way `path_count_setup.wgsl`/
  `path_tiling_setup.wgsl`/`coarse.wgsl` do) - `path_tiling_setup.rs` already has a comment
  (`// binding 2 is ptcl, which we would need if we propagate failure`) acknowledging this gap
  explicitly.
- **This fix does not attempt pixel-level parity with the GPU path's degraded output** in an
  overflow scenario (e.g. matching exactly which draw objects silently get corrupted/dropped) -
  only that the CPU shim stops panicking and degrades no more, not that its degraded output
  visually matches what the GPU path would have produced for the same over-capacity scene. No
  claim is made either way about whether the two paths' *wrong* outputs would match once neither
  panics.
- `blend_spill`/`lines`/`segments`/`ptcl` were not independently re-derived with their own
  from-scratch overflow scenes this session (only `segments`/`ptcl` were *observed* incidentally,
  via the "after" run above, as the next stage each of this candidate's three scenes happens to
  reach) - a systematic sweep of every remaining bump-allocated buffer, not just the ones a
  particular scene happens to reach next, is future work.
