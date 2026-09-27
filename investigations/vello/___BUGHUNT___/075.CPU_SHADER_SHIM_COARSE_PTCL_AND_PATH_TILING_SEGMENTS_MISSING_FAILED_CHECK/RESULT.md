# CPU_SHADER_SHIM_COARSE_PTCL_AND_PATH_TILING_SEGMENTS_MISSING_FAILED_CHECK

Extends this workspace's own 066.CPU_SHADER_SHIM_BUMP_ALLOCATOR_MISSING_FAILED_CHECK (which
covered `tile_alloc.rs`/`path_count.rs`/`binning.rs`) to `vello_shaders/src/cpu/coarse.rs`'s `ptcl`
buffer - the two files 066 explicitly left open (`coarse.rs`'s `ptcl`, `path_tiling.rs`'s
`segments`) after observing its own fix cause exactly the same scenes to panic one stage later, at
`coarse.rs:68`/`coarse.rs:382`/`path_tiling.rs:164`. This workspace's own
047.PATH_TILING_CPU_SHIM_ASSERT_AND_INDEX_WRITE already investigated `path_tiling.rs`'s `segments`
buffer specifically and found it Not Reproducible in isolation (an earlier stage,
`path_count.rs`, always panics first for every construction tried) - so this candidate's scope is
`coarse.rs`'s `ptcl` buffer only, at its two call sites: `TileState::write` (used by
`write_path`/`write_color`/etc.) and the unconditional per-tile `CMD_END`/blend-offset writes at
the end of `coarse_main`'s tile loop.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `coarse.rs` against `coarse.wgsl` (its GPU counterpart) and `shader/shared/bump.wgsl`, following 066's established methodology for this bug class | Reading only | `coarse.wgsl`'s `alloc_cmd` has an explicit `new_cmd + PTCL_INCREMENT > config.ptcl_size` capacity check, resetting to slot 0 and `atomicOr(&bump.failed, STAGE_COARSE)` on overflow - this whole mechanism is completely absent from `coarse.rs`'s `alloc_cmd`/`TileState::write`. Separately, the *unconditional* per-tile `CMD_END`/blend-offset write near the end of `coarse.wgsl`'s tile loop has **no** explicit capacity check on the GPU side either - it relies purely on wgpu/naga's own storage-buffer bounds checks (an implicit silent clamp) to stay safe when a tile's own initial cursor (`tile_ix * PTCL_INITIAL_ALLOC`) already exceeds `config.ptcl_size`. A plain Rust slice has no equivalent implicit protection, so this second case needed its own explicit guard in the fix (not just mirroring an existing GPU-side check, since none exists there for this exact case). |
| Real end-to-end reproduction via `vello::Scene`/`Rect` (public API) → a real `wgpu::Device` (SwiftShader/Vulkan software ICD) → `vello::Renderer::render_to_texture` with `RendererOptions::use_cpu: true`, `catch_unwind` around the call, targeting the *static* per-tile-cursor overflow (a canvas large enough that `width_in_tiles * height_in_tiles * 64 > config.ptcl_size` (8,388,608), independent of scene content) | Release build, `WGPU_BACKEND=vulkan`, one tiny 4x4px fill on a 5824x5824 canvas (132,496 tiles > 131,072 threshold), 3/3 runs | Panics deterministically at exactly `coarse.rs:382` (`index out of bounds: the len is 8388608 but the index is 8409857`) - the first tool tried already reproduced the targeted bug on the first attempt. |
| Real end-to-end reproduction, same harness, targeting the *dynamic* `alloc_cmd`/chunk-jump overflow path specifically (a canvas just under the 131,072-tile direct-overflow threshold, so the static case above never fires, plus many small same-tile fills to force many `alloc_cmd` chunk-jumps concentrated on one tile, cheaply, without overflowing `tile_alloc.rs`'s/`path_count.rs`'s own much smaller (1<<21) buffers first) | Release build, `WGPU_BACKEND=vulkan`, 60,000 same-16x16px-tile fills on a 5776x5776 canvas (130,321 tiles, under threshold), 3/3 runs | Panics deterministically at exactly `coarse.rs:68` (`index out of bounds: the len is 8388608 but the index is 8388608`) - confirming the second, independent call site the candidate's scope names. Both call sites reproduced on the first real-repro attempt each; no third tool/approach was needed. |
| Best-effort commit history lookup (`git log -S`/`git show --stat`, `git log --follow -- vello_shaders/src/cpu/coarse.rs`) for when `coarse.rs`'s CPU shim and `coarse.wgsl`'s `bump.failed`/`STAGE_COARSE` mechanism were each introduced | `git log`, no build | See "Introduced by" below - unlike 066's three files (where the GPU mechanism predated the CPU shim by ~8 months), for `coarse.rs` specifically the *order is reversed*: the CPU shim (d2c2a267, Sep 2023) predates `coarse.wgsl`'s own `bump.failed`/`STAGE_COARSE` capacity check (aad292dd, Apr 2024) by about 6.5 months, and no commit between those two dates (confirmed via `git log --follow`) ever touched `vello_shaders/src/cpu/coarse.rs`'s `alloc_cmd`/`TileState` at all. |
| Attempt + verify a bounded fix mirroring `coarse.wgsl`'s own `alloc_cmd` capacity-check-and-`failed` pattern for the dynamic path, plus defensive `.get_mut()` bounds-checked writes on every `ptcl` write in the file (needed for the static per-tile-cursor case, which has no explicit GPU-side check to mirror at all - only wgpu/naga's implicit one) | Release build, both repro scenarios, 3/3 runs before and after; project test suite before/after; also verified via a from-scratch `just clone` + `just test` full cycle in `/tmp` (a scratch location, not this candidate folder) to confirm the standalone `repro/`+`justfile` genuinely reproduce and then fix the bug with no dependency on the development worktree | See "The fix" below - both originally-targeted panics are gone (verified 3/3, both via the development worktree and via a completely fresh clone), full test suite identical before/after. |

## Status

**Confirmed.** Reproduces deterministically, 3/3 runs each (for both call sites), release build,
`WGPU_BACKEND=vulkan` (SwiftShader software ICD) - see `output.txt` for the exact captured panics
(including a from-scratch `just clone` re-run), `repro/` for the runnable reproduction.

**Fully reproducible**: **Yes**. Both repros drive the real, public entry point
(`vello::Renderer::render_to_texture` with `RendererOptions::use_cpu: true`) with real
`Scene`/`Rect` geometry through a real (software) `wgpu::Device` - exactly the same call a real
application using `use_cpu: true` makes. There is no second participant being faked here (see
`CONFIRMATION.md` section 11): the "adversarial" input is a single external artifact (an
application-supplied canvas size and/or scene with unusually many draws), the same kind of input a
real caller of the public API can genuinely construct.

## Class

crash (index-out-of-bounds panic; the GPU path handles the identical condition safely - via an
explicit capacity-check-and-`failed` guard for the dynamic case, or wgpu/naga's own implicit
storage-buffer bounds clamp for the static case)

## Root cause

`vello_encoding::config::BufferSizes::new` hardcodes the `ptcl` buffer's capacity at `1 << 23`
(8,388,608 `u32`s), independent of the actual canvas size or scene content (same "hand-picked to
accommodate the vello test scenes" comment 066 already documented for the other bump-allocated
buffers). `vello_shaders/src/cpu/coarse.rs` writes into this fixed-size buffer at two structurally
different call sites, neither of which is protected against the buffer's actual, fixed capacity:

- **The *dynamic* path (`TileState::write`, `coarse.rs:68` before this fix)**: `alloc_cmd`
  computes `new_cmd = ptcl_dyn_start + bump.ptcl` (a running global counter across every tile's
  own chunk-overflow events) whenever a tile's local per-tile `ptcl` allocation is exhausted, and
  hands out `new_cmd` unconditionally - unlike `coarse.wgsl`'s own `alloc_cmd`, which explicitly
  checks `new_cmd + PTCL_INCREMENT > config.ptcl_size` and resets to slot 0 (plus sets
  `bump.failed |= STAGE_COARSE`) before ever indexing with it. Once enough tiles across the scene
  have each needed enough chunk-jumps that the cumulative `bump.ptcl` pushes `new_cmd` past
  `config.ptcl_size`, the very next `write()` call (used by `write_path`/`write_color`/etc.
  immediately after `alloc_cmd`) indexes past the end of the `ptcl` slice.
- **The *static* path (the unconditional per-tile `CMD_END`/blend-offset writes,
  `coarse.rs:382`/`:385` before this fix)**: `TileState::new` unconditionally seeds every tile's
  initial `cmd_offset` at `tile_ix * PTCL_INITIAL_ALLOC` (`PTCL_INITIAL_ALLOC = 64`), and
  `coarse_main` unconditionally writes `CMD_END`/the blend offset at that (possibly-updated)
  cursor for every real in-canvas tile, with no capacity check anywhere in this path - **on
  either side**. A canvas with more than `8,388,608 / 64 = 131,072` tiles overflows the fixed
  `ptcl` buffer purely from this per-tile bookkeeping, with **zero scene content required at all**
  (an empty scene at a large enough canvas would trigger it identically - this candidate's repro
  adds one trivial draw only to avoid relying on an untested "does the pipeline even dispatch for
  a literally empty scene" edge case, not because the bug needs any draw). Unlike the dynamic
  path above, `coarse.wgsl` has **no explicit check here either** - it is protected purely by
  wgpu/naga's own storage-buffer bounds checks (an implicit runtime clamp WGSL storage-buffer
  indexing gets automatically), which a plain Rust slice never gets for free.

Both call sites were reproduced independently and directly, using canvas sizes/scene shapes chosen
specifically to isolate each one from `tile_alloc.rs`'s/`path_count.rs`'s/`binning.rs`'s own
(much smaller, `1<<21`/`1<<18`) unrelated overflow conditions - see "What was tried" above for the
exact construction of each.

## Introduced by

d2c2a267 "Add CPU shaders" (2023-09-19) is when `coarse.rs`'s unchecked `ptcl` writes were
originally written, and no commit between then and the present ever added a capacity check to
either call site in this file (confirmed via `git log --follow -- vello_shaders/src/cpu/coarse.rs`
- the only touches to `alloc_cmd`/`TileState` in that whole history are unrelated feature work:
even-odd fill-rule support, sweep gradients, blurred-rounded-rect support). Notably, and unlike
066's three files (where the GPU-side safety mechanism already existed ~8 months before the CPU
shim was written), for `coarse.rs` specifically the temporal order is reversed: `coarse.wgsl`'s own
`bump.failed`/`STAGE_COARSE` capacity check for the dynamic path was added later, by
aad292dd "Add robustness to GPU shaders (#537)" (2024-04-01) - about 6.5 months *after* the CPU
shim already existed without any equivalent - and that commit never touched
`vello_shaders/src/cpu/coarse.rs` at all. So aad292dd is the commit that actually created the
GPU/CPU divergence for the dynamic-path call site specifically (before it, neither side had a
check); the static-path call site's gap (no explicit check on *either* side, GPU relying entirely
on wgpu/naga's implicit bounds clamp) has no single "introducing" commit in the same sense - it's
been present, unaddressed on the CPU side, since d2c2a267 itself.

## The fix

Added, to `vello_shaders/src/cpu/coarse.rs`, the same two-part mechanism this workspace's own 066
already established for the sibling files, adapted to `coarse.rs`'s two distinct call sites:

1. **Capacity check + offset reset + `bump.failed`** in `TileState::alloc_cmd`, mirroring
   `coarse.wgsl`'s own `alloc_cmd` exactly: before handing out a `new_cmd` that would push past
   `config.ptcl_size`, reset it to 0 and OR a new `STAGE_COARSE` constant (mirroring
   `shader/shared/bump.wgsl`'s value, `0x10`) into `bump.failed`.
2. **Defensive `.get_mut()`-based bounds-checked writes** on every remaining `ptcl` write in the
   file: `alloc_cmd`'s own two `CMD_JUMP`/new-offset writes, `TileState::write` (the generic
   helper backing `write_path`/`write_color`/`write_image`/`write_grad`/`write_blur_rect`/
   `write_begin_clip`/`write_end_clip`), and `coarse_main`'s unconditional per-tile
   `CMD_END`/blend-offset writes. This is needed everywhere, not just as a defense-in-depth
   measure: per "Root cause" above, the static per-tile-cursor case has no explicit GPU-side
   check to mirror at all (only wgpu/naga's own implicit clamp, which a plain Rust slice doesn't
   get), so an explicit bounds check is the *only* fix available for that call site.

Verified: re-running `repro/`'s two scenarios against the patched tree, 3/3 runs each, no longer
panics at `coarse.rs:68` or `coarse.rs:382` (both print "rendered OK, no panic" - see
`output.txt`). Also independently re-verified via a completely fresh `just clone` +
`just apply` + `just reproduce-after` cycle in a scratch location outside this repo, confirming
the standalone `repro/`/`justfile` genuinely reproduce and then fix the bug with zero dependency
on the development worktree used to find it.

- Project test suite: `vello_shaders`/`vello_encoding`/`vello` `--lib` (release): 31 passed / 0
  failed before, 31 passed / 0 failed after (all 31 real tests are in `vello_encoding`; `vello`/
  `vello_shaders` have 0 unit tests) - identical, no regressions. `vello_tests` (with
  `VELLO_SKIP_LFS_SNAPSHOTS=all`, see `___BUGHUNT___/README.md` for why this sandbox needs that
  flag): 64 passed / 0 failed / 1 ignored before, identical after. See `output.txt` for the full
  per-binary breakdown.

## What's not done / open questions

- **This is a scoped fix for `coarse.rs`'s `ptcl` buffer only.** `path_tiling.rs`'s `segments`
  buffer (the other file 066 left open) was not investigated here - it was already investigated
  by this workspace's own 047.PATH_TILING_CPU_SHIM_ASSERT_AND_INDEX_WRITE and found Not
  Reproducible in isolation (an earlier stage, `path_count.rs`, always panics first for every
  construction 047 tried). If a future candidate fixes `path_count.rs`'s own overflow (066's fix,
  not applied to the real source by design - see "the one rule that matters most" in `SKILL.md`),
  `path_tiling.rs`'s `segments` buffer would become newly reachable and should be re-investigated
  at that point, the same way this candidate's own two scenes only became reachable in `coarse.rs`
  once 066's fix (also not applied to real source) was hypothetically in place.
- **This fix does not attempt pixel-level parity with the GPU path's degraded output** in an
  overflow scenario - only that the CPU shim stops panicking, not that its degraded output
  visually matches what the GPU path would produce for the same over-capacity scene (same caveat
  066 already noted for its own three files).
- As with 066, `path_count_setup.rs`/`path_tiling_setup.rs`'s own CPU-shim equivalents still never
  check `bump.failed` before running (unlike `path_count_setup.wgsl`/`path_tiling_setup.wgsl`,
  which skip their own work when a prior stage already failed) - so setting `bump.failed` in this
  fix is parity/documentation with the GPU path, not something anything downstream in the CPU
  shim currently consumes. `path_tiling_setup.rs`'s own comment
  (`// binding 2 is ptcl, which we would need if we propagate failure`) already acknowledges this
  gap explicitly.
