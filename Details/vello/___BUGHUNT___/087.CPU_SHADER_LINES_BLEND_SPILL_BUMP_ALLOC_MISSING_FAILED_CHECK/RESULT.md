# CPU_SHADER_LINES_BLEND_SPILL_BUMP_ALLOC_MISSING_FAILED_CHECK

Checks the two bump-allocated buffers this candidate names by analogy to already-confirmed
066/075: `lines` (`vello_shaders/src/cpu/flatten.rs`) and `blend_spill` (used by `fine.wgsl`).
Result is split: `lines` reproduces the same missing-bounds-check class as 066/075; `blend_spill`
turns out not to apply at all, because the CPU-shader shim never implements the `fine`
(rasterization) stage in the first place.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `vello_shaders/src/cpu/flatten.rs`'s `write_line` against its GPU counterpart `vello_shaders/shader/flatten.wgsl`'s `write_line` | Reading only | GPU's `write_line` guards the write with `if line_ix < config.lines_size { lines[line_ix] = ... }` (added in aad292dd7, see "Introduced by"). The CPU shim's `write_line` (`flatten.rs:90`) indexes `lines[line_ix] = LineSoup { ... }` unconditionally, with no equivalent guard - confirming the same missing-check pattern as 066/075, one stage earlier in the pipeline (`flatten` runs before `binning`/`tile_alloc`/`path_count`/`coarse`/`path_tiling`, per the dispatch order in `vello/src/render.rs`). |
| Search for the `blend_spill` buffer's CPU-shim equivalent (`vello_shaders/src/cpu/fine.rs`) | Reading only | `fine.rs`'s only function, `fine_main`, is marked `#[expect(unused, reason = "Draft code as textures not wired up")]` - genuinely dead code, not `pub use`d from `vello_shaders/src/cpu.rs` (unlike every other stage's module). It also doesn't implement `CMD_BEGIN_CLIP`/`CMD_END_CLIP` at all (`todo!("unhandled ptcl command {tag}")` for anything else), so it has no blend-stack/`blend_spill` logic to be missing a check from, even in principle. Confirmed via `vello/src/shaders.rs`: `fine_area`/`fine_msaa8`/`fine_msaa16` are registered with a hardcoded `CpuShaderType::Missing` (unlike every other stage, which passes `CpuShaderType::Present(vello_shaders::cpu::$name)` when `use_cpu` is requested) - so the real GPU `fine.wgsl` shader (which already has its own `blend_spill` handling, protected the normal GPU way) always runs the fine-rasterization stage, even with `RendererOptions::use_cpu: true`. The crate's own module doc comment states this directly: "a full CPU fallback as an alternative to GPU shaders is not provided." |
| Real end-to-end reproduction of the `lines` overflow via `vello::Scene`/`BezPath` (public API) -> a real `wgpu::Device` (SwiftShader/Vulkan software ICD) -> `vello::Renderer::render_to_texture` with `RendererOptions::use_cpu: true`, `catch_unwind` around the call - one path with 2,200,000 plain `line_to` segments (comfortably above `config.lines_size` == 1<<21 == 2,097,152; plain lines need no curve subdivision or stroke join/cap multiplication, so each `line_to` maps 1:1 to one `LineSoup`) | Release build, `WGPU_BACKEND=vulkan`, `VK_ICD_FILENAMES` pointed at the SwiftShader ICD, `RUST_BACKTRACE=1`, 3/3 runs | Deterministic panic every run: `index out of bounds: the len is 2097152 but the index is 2097152` at `vello_shaders/src/cpu/flatten.rs:90` (inside `write_line`, called from `flatten_euler`, called from `flatten`). See `output.txt`. |
| Attempt + verify a bounded fix mirroring `flatten.wgsl`'s own `if line_ix < config.lines_size` guard, in `flatten.rs`'s `write_line` | Release build, same repro scene, 3/3 runs before and after, project test suite before/after | The originally-targeted panic at `flatten.rs:90` is gone (verified 3/3) - but, matching 066/075's own experience with this bug class, the same scene now panics one stage later at the already-known, unfixed sibling site `path_count.rs:150` (066's own confirmed `seg_counts`-buffer overflow) instead of completing successfully. See "The fix" below. |
| Best-effort commit-history lookup (`git log -S`/`git blame`) for when the GPU/CPU divergence for `write_line`'s bounds check was introduced | `git log`/`git blame`, no build | See "Introduced by" below - the commit that added the GPU-side `if line_ix < config.lines_size` guard (aad292dd7, 2024-04-01) explicitly states in its own commit message that it deliberately left the CPU shim un-updated: "The CPU shaders are minimally changed to be layout compatible. For the most part, they'll panic on a bounds check if sizes are exceeded. That's arguably useful for debugging, but a case can be made they should have the same behavior as the GPU shaders." |

## Status

**Confirmed** for the `lines` buffer (`vello_shaders/src/cpu/flatten.rs`'s `write_line`).
**Not reproducible / not applicable** for the `blend_spill` buffer - see the second row above: the
CPU-shader shim has no code path that ever touches `blend_spill` at all, since the `fine`
(rasterization) stage always runs on the real GPU shader regardless of `RendererOptions::use_cpu`.
There is no "missing check" to find in code that is never reached.

Reproduces deterministically, 3/3 runs, release build, `WGPU_BACKEND=vulkan` (SwiftShader software
ICD) - see `output.txt` for the exact captured panic, `repro/` for the runnable reproduction.

**Fully reproducible**: **Yes**. The repro drives the real, public entry point
(`vello::Renderer::render_to_texture` with `RendererOptions::use_cpu: true`) with a real
`Scene`/`BezPath` through a real (software) `wgpu::Device` - the exact same call a real application
using `use_cpu: true` makes. There is no second participant being faked here (see
`CONFIRMATION.md` section 11): the "adversarial" input is a single external artifact (an
application-supplied scene with an unusually large number of line segments), the same kind of
input a real caller of the public API can genuinely construct.

## Class

crash (index-out-of-bounds panic; the GPU path handles the identical condition safely by design)

## Root cause

`vello_encoding::config::BufferSizes::new` gives the `lines` buffer a fixed capacity of `1 << 21`
(2,097,152 `LineSoup` entries), independent of the actual scene - "sized... to accommodate the
vello test scenes as well as paris-30k" per the code's own comment, same as every other
bump-allocated buffer in this pipeline (066/075's own root cause). `flatten` is the *first* stage
in the pipeline (`bbox_clear` -> `flatten` -> `binning` -> `tile_alloc` -> `path_count_setup` ->
`path_count` -> `coarse` -> `path_tiling_setup` -> `path_tiling` -> `fine`, per the dispatch order
in `vello/src/render.rs::render_encoding_coarse`) to claim a slot in one of these buffers per
flattened line segment, via a running `line_ix` counter incremented once per `output_line`/
`output_line_with_transform`/`output_two_lines_with_transform` call and only written back into
`bump.lines` once, at the very end of the whole `flatten` shader (`bump.lines = line_ix as u32;`
in `flatten.rs`).

On GPU, `flatten.wgsl`'s own `write_line` explicitly checks `if line_ix < config.lines_size` before
ever indexing into the `lines` storage buffer (added by aad292dd7, see "Introduced by") - an
over-capacity write is silently dropped, degrading to wrong-but-safe output (missing line segments)
rather than crashing; downstream, `binning.wgsl` separately detects the overall `bump.lines >
config.lines_size` condition and sets `bump.failed |= STAGE_FLATTEN` so later stages can skip
their own now-meaningless work. `vello_shaders/src/cpu/flatten.rs`'s `write_line` reimplements the
same arithmetic but never picked up either of GPU's two safety nets: no bounds check before the
write, and (being a plain Rust slice, not a WGSL storage buffer) no implicit bounds-clamp fallback
either - so the identical over-capacity condition (here, one path whose own flattened-line count
alone exceeds `config.lines_size`) becomes a hard `index out of bounds` panic at `flatten.rs:90`
instead of degrading safely.

The `blend_spill` half of this candidate's premise does not hold: the CPU-shader shim's `fine.rs`
module (which would be the analogous site for a `blend_spill`-buffer bug) is dead/draft code, never
wired up to run (see "What was tried" above) - the real `fine.wgsl` GPU shader always runs instead,
even under `RendererOptions::use_cpu: true`, and it already has its own GPU-side `blend_spill`
handling (implicit wgpu/naga storage-buffer bounds clamp, plus the early-exit `if ptcl[0] == ~0u`
check for "an earlier stage has failed"). There is no CPU-shim `blend_spill` write to be missing a
check.

## Introduced by

The unbounded `write_line` form in `flatten.rs` itself was written at 1e30be1fd "[cpu_shaders]
[flatten] Incorporate stroke expansion changes" (2023-11-22), when the CPU shim's flatten stage was
rewritten around the new Euler-spiral stroke-expansion algorithm - but at that time the GPU
counterpart (`flatten.wgsl`) had no bounds check on this write either, so there was no divergence
yet. The actual GPU/CPU divergence - the bug as it exists today - was introduced by aad292dd7 "Add
robustness to GPU shaders (#537)" (2024-04-01), the same commit already cited in 075's own
"Introduced by" for the identical class of divergence in `coarse.wgsl`/`coarse.rs`. This commit
added the `if line_ix < config.lines_size` guard to `flatten.wgsl`'s `write_line` (and equivalent
guards to `binning.wgsl`/`tile_alloc.wgsl`/`path_count.wgsl`/`coarse.wgsl`/`fine.wgsl`) but left
`vello_shaders/src/cpu/flatten.rs` untouched - the commit's own message explicitly acknowledges
this: "The CPU shaders are minimally changed to be layout compatible. For the most part, they'll
panic on a bounds check if sizes are exceeded. That's arguably useful for debugging, but a case can
be made they should have the same behavior as the GPU shaders."

## The fix

Added the same bounds check `flatten.wgsl`'s `write_line` already has, to `flatten.rs`'s
`write_line`: replace the unconditional `lines[line_ix] = LineSoup { ... }` with
`if let Some(slot) = lines.get_mut(line_ix) { *slot = LineSoup { ... }; }`, silently dropping the
write when `line_ix` is out of range instead of panicking - mirroring the GPU shader's own
degrade-safely behavior exactly (this is a plain Rust slice with no implicit bounds-clamp fallback,
so the check has to be explicit, same reasoning as 066's fix).

Verified: re-running `repro/` against the patched tree, 3/3 runs, no longer panics at
`flatten.rs:90`. **However**, matching 066/075's own experience with this bug class, the same
repro scene still panics overall, now one stage later - `path_count.rs:150` (066's own confirmed,
unfixed `seg_counts`-buffer overflow) - because that sibling file has the same unfixed
"unchecked bump-allocator write" bug class, and this candidate's fix was intentionally scoped to
only `flatten.rs`'s `write_line`, not a systemic sweep of the whole CPU shim. This is expected, not
a fix failure, and is the same caveat 066's own `RESULT.md` states for its own three target sites.

- Project test suite: `vello_shaders`/`vello_encoding`/`vello` `--lib` (release): 31 passed / 0
  failed before, 31 passed / 0 failed after (all 31 real tests are in `vello_encoding`; `vello`/
  `vello_shaders` have 0 unit tests in this configuration) - identical, no regressions. See
  `output.txt` for the captured test output.

## What's not done / open questions

- This is a scoped fix for exactly `flatten.rs`'s `write_line` (the `lines` buffer), not a
  systemic fix for the whole CPU shim - the identical bug class is already known, unfixed, in
  `tile_alloc.rs`/`path_count.rs`/`binning.rs` (066), `coarse.rs`/`path_tiling.rs` (075), and (per
  066's own "What's not done") was never independently re-derived from scratch for `segments`
  either (only observed incidentally as a "next stage reached" panic).
- `blend_spill` is confirmed *not* to have this bug class, for the specific reason stated above
  (the CPU shim never runs the `fine` stage at all) - this is a structural fact about the current
  pipeline, not something that would change if a future PR ever did wire up `fine.rs`'s draft code;
  if that ever happens, `fine.rs`'s handling of `blend_spill`/`CMD_BEGIN_CLIP`/`CMD_END_CLIP` would
  need its own bounds check added at that time, since none of the draft code currently has one
  either (it doesn't even implement clip commands, so there's nothing to check yet).
- No attempt was made to give the CPU shim the *other* half of the GPU path's mechanism (having
  each stage actually check `bump.failed` and skip its own work when a prior stage already failed,
  the way `path_count_setup.wgsl`/`path_tiling_setup.wgsl`/`coarse.wgsl`/`fine.wgsl` do) - same
  open item already noted in 066's own "What's not done".
