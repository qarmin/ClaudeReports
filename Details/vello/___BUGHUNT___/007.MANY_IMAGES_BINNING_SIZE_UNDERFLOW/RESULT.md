# MANY_IMAGES_BINNING_SIZE_UNDERFLOW

`vello_encoding::RenderConfig::new` (the classic GPU `vello` crate's CPU-side render-config
builder, `vello_encoding/src/config.rs`) computes `binning_size` as a plain subtraction of two
`u32`s, one of which (`layout.bin_data_start`) scales with the number of draw objects in the
scene and the other of which (`buffer_sizes.bin_data.len()`) is a hardcoded constant - so a scene
with enough draw objects (concretely: `vello::Scene::draw_image` called ~26,215+ times, well
below the issue's reported 1,000,000) makes the subtraction underflow. Source: GitHub issue
https://github.com/linebender/vello/issues/788 (candidate `IDEAS.md`/task source), still open,
unfixed upstream as of this session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug-assertions build + the project's own real public API (`vello::Scene::draw_image` in a loop, then `vello_encoding::Resolver::resolve` + `RenderConfig::new`, the same two calls `vello`'s own renderer makes internally) | `cargo run` (dev profile, default `overflow-checks = true`), `n_images = 1_000_000` (issue's own repro count) | Reproduced: `attempt to subtract with overflow` panic at `vello_encoding/src/config.rs:185:31`, exactly the line/panic the issue reports |
| Same repro, boundary-value sweep | debug build, `n_images` in `{26214, 26215, 100000, 2000000}` | `26214` (bin_data_start = 262140, just under the hardcoded `1<<18` = 262144 buffer size) does not panic; `26215` (bin_data_start = 262150) panics - confirms the exact threshold and that it is far below the issue's headline 1,000,000, i.e. this is easy to hit unintentionally with a moderately large scene, not just a deliberate 1M-image stress test |
| Same repro, release build | `cargo run --release` (`overflow-checks = false`), `n_images = 1_000_000` | No panic; `RenderConfig::new` completes but `binning_size` = 4285229440 (the wrapped `u32` value of `262144 - 10000000 mod 2^32`) - confirms the reporter's claim that release mode silently produces a bogus, huge `binning_size` instead of panicking |

Only one tool/approach was needed (a debug-assertions build plus manual boundary-value
construction, per `HEURISTICS.md`'s guidance for "arithmetic on untrusted sizes" bugs) - the first
attempt reproduced the exact reported panic, so no escalation to miri/ASAN/fuzzing was warranted.

## Status

**Confirmed.** Reproduces deterministically, 3/3 runs (and again after being rebuilt from a fresh
`git clone` via this candidate's own `justfile`), debug build (`overflow-checks = true`, which is
`cargo`'s default `dev` profile - no special flag needed). Also independently confirmed the
release-mode silent-wrap variant described in the original issue (see table above).

**Fully reproducible**: Yes. The repro (`repro/src/main.rs`) drives the exact same real,
public-API path a real caller/renderer takes: `vello::Scene::draw_image` (the same public method
the issue's own repro uses, called the same number of times) to build a real `Encoding`, then
`vello_encoding::Resolver::resolve` and `RenderConfig::new` - the identical two calls
`vello::render::Render::render_encoding_coarse` makes internally before any GPU work is submitted.
This is a single-process, single-participant bug (a library reacting to its own public API being
called with realistic input, not one participant reacting to another's wire input), so
`CONFIRMATION.md` section 11's "real second participant" bar does not apply here; no GPU/wgpu
device is needed to hit the panic since it happens entirely in CPU-side arithmetic before any GPU
resource is created.

## Class

crash (debug: deterministic panic / `u32` underflow trap; release: silent wraparound producing a
nonsensical, ~4.28-billion-element `binning_size` bound that a GPU shader would consequently trust
as a valid write-region size for a buffer that is actually far smaller - a likely GPU-side
out-of-bounds write, matching the reporter's own suspicion, though this session did not run an
actual GPU trace to directly observe the OOB write itself).

## Root cause

`vello_encoding::config::BufferSizes::new` allocates the CPU-side size record for the `bin_data`
GPU buffer as a hardcoded constant, `BufferSize::new(1 << 18)` (262,144 `u32` words) - per the
code's own comment, "hand picked to accommodate the vello test scenes as well as paris-30k", not
derived from the actual scene. Separately, `Layout::bin_data_start` (computed in
`vello_encoding/src/resolve.rs:137`) is `encoding.draw_tags.iter().map(|tag| tag.info_size()).sum()`
- the total number of `u32` words needed for every draw object's fixed "info" record, which grows
linearly with the number of draw calls (an image draw's `DrawTag::IMAGE` has `info_size() == 10`).
`RenderConfig::new` (`vello_encoding/src/config.rs:185`) then computes
`buffer_sizes.bin_data.len() - layout.bin_data_start` with no check that the (variable-size)
subtrahend is smaller than the (fixed-size) minuend. Once a scene has more than 26,214 image draws
(26,214 * 10 = 262,140 < 262,144; 26,215 * 10 = 262,150 > 262,144), `bin_data_start` exceeds the
buffer's hardcoded capacity and the subtraction underflows - panicking in a debug build via Rust's
checked-arithmetic trap, or silently wrapping to a huge `u32` in release. That wrapped value is
exactly the `binning_size` field written into `ConfigUniform`, which (per `config.wgsl`, not
inspected line-by-line this session but implied by the field's own doc comment "Size of binning
buffer allocation") is meant to bound how far past `bin_data_start` a GPU shader is allowed to
write into the physical `bin_data` buffer - a buffer whose actual allocated size
(`buffer_sizes.bin_data.size_in_bytes()`, see `vello/src/render.rs`'s `info_bin_data_buf`) never
changes regardless of scene size. So even independent of the subtraction's arithmetic behavior,
the deeper problem is that the fixed 262,144-word buffer is simply too small to hold `bin_data_start`
itself once a scene has enough draw objects - the subtraction underflow is the symptom that makes
this discoverable in a debug build, not the whole bug.

## Introduced by

0256d8a9 "[vello_encoding] Minor clean ups and correctness fixes" (2023-04-13). Before this
commit, `binning_size` was set unconditionally to `buffer_sizes.bin_data.len()` (no subtraction at
all, see the same file at the prior commit e1f5352a0bef94cea433cf2a528fa10548d4fca3, the type's
introducing commit from 2023-01-26, where `config.binning_size = buffer_sizes.bin_data.len();`
with no offset). 0256d8a9 introduced the `- layout.bin_data_start` subtraction with no bounds
check, while `bin_data`'s own hardcoded size and `bin_data_start`'s draw-object-count-dependent
growth were both already present beforehand - i.e. this commit is what turned an existing
capacity-mismatch risk into an actual unchecked subtraction that can underflow.

## The fix

Two changes to `vello_encoding/src/config.rs`, both verified together:

1. `binning_size: buffer_sizes.bin_data.len() - layout.bin_data_start` ->
   `buffer_sizes.bin_data.len().saturating_sub(layout.bin_data_start)` - removes the panic/UB-prone
   raw subtraction (defense in depth; should now be unreachable given change 2, but kept so a
   future divergence between `bin_data`'s size and `bin_data_start` degrades to `binning_size = 0`
   rather than panicking/wrapping again).
2. `let bin_data = BufferSize::new(1 << 18);` -> `let bin_data = BufferSize::new(layout.bin_data_start.saturating_add(1 << 18));`
   in `BufferSizes::new` - this is the actual fix for the underlying capacity mismatch: the
   physical `bin_data`/`info_bin_data_buf` GPU buffer is now always sized to hold the full
   variable-size "info" section (`bin_data_start` words) *plus* the same `1<<18`-word headroom
   for per-bin data the hardcoded constant used to provide unconditionally. `binning_size` is
   therefore always exactly `262144` after the fix, for any scene size tested, instead of
   underflowing/wrapping once `bin_data_start` exceeds the old fixed constant.

Verified per `CONFIRMATION.md` section 7 (varied beyond the one triggering input): re-ran the
repro at `n_images` = 26214 (just under the old threshold), 26215 (just over), 100000, 1000000
(the issue's own count), and 2000000 - all five complete without panicking after the fix, all
producing the same `binning_size = 262144`, both against the live worktree and against a fresh
`git clone` + `git apply patch.diff` via this candidate's own `justfile` (`just test`).

- Project test suite: `cargo test -p vello_encoding`: 31 passed, 0 failed before -> 31 passed, 0
  failed after (identical). `cargo test -p vello --no-default-features`: 0 passed, 0 failed before
  and after (this crate has no tests that run without a GPU/wgpu device in this sandbox; the wgpu
  feature was not exercised this session - see "What's not done" below).

## What's not done / open questions

- This session did not verify the GPU-side consequence directly (running the coarse/binning
  compute shaders against a real or software GPU adapter to observe an actual out-of-bounds write
  when `binning_size` wraps in a release build) - the CPU-side arithmetic bug and its immediate
  panic/wrapped-value output are what's confirmed; the "potential OOB read/write on the GPU" part
  of the original report is corroborated by code reading (the wrapped value is used as a shader
  bounds-check parameter for a buffer whose real allocated size never changes) but not directly
  observed via a GPU trace/validation layer this session.
- The fix keeps the same "hand-picked headroom" design the code's own comment already flags as a
  known shortcoming ("These should instead get derived from the scene layout using reasonable
  heuristics") - it makes the buffer large enough for the *info* section plus the same fixed
  262,144-word headroom for the *actual* per-bin data, but a scene that also needs more than
  262,144 words of per-bin data (as opposed to draw-object info) could in principle still exceed
  that headroom. That would show up as `BumpAllocators::binning` (the GPU-side atomic overflow
  counter used by the "robust dynamic memory" path, per that struct's own doc comment) reporting
  overflow rather than as an underflow at `RenderConfig::new` - a different, not-yet-implemented
  code path (`vello/src/lib.rs`'s own `// TODO: apply logic to determine whether we need to rerun
  coarse` next to where that counter is read back) that this session did not investigate further,
  since it is a distinct, already-flagged-as-incomplete robustness mechanism rather than part of
  this specific underflow bug.
- Other fixed-size, scene-size-independent buffers in the same `BufferSizes::new` function
  (`tiles`, `lines`, `seg_counts`, `segments`, `blend_spill`, `ptcl`, all hardcoded to `1 << 20`
  or `1 << 21` or `1 << 23`) were not individually checked for the same class of "hardcoded size
  vs. scene-dependent requirement" mismatch this session - `bin_data`/`bin_data_start` was the one
  directly implicated by the reported panic. A future session could check whether any of these
  have their own unchecked-subtraction or capacity-mismatch counterpart.
- The sparse-strips/CPU-and-Hybrid backends (`vello_cpu`, `vello_hybrid`, `sparse_strips/*`) do not
  use this classic `vello_encoding` GPU pipeline's `BufferSizes`/`RenderConfig` at all, so this
  finding is specific to the classic `vello` (GPU compute) crate.
