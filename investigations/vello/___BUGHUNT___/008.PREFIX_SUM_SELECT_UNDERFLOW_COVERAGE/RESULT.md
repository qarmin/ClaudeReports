# Sibling-shader sweep for the prefix-sum select() underflow pattern (PR #1817 / #1816 / #1465)

Open PR #1817 (https://github.com/linebender/vello/pull/1817, not yet merged, fixes #1816)
clamps 6 sites in `vello_shaders/shader/{tile_alloc,backdrop_dyn,coarse,fine}.wgsl` where a
prefix-sum lookup has the shape `select(default, sh_array[ix - 1u], ix > 0u)`. WGSL's `select()`
evaluates both operands unconditionally, so the `ix == 0u` lane always computes
`sh_array[0u - 1u]` (a huge wrapped `u32` index) before the result is discarded; Mali lowers
`var<workgroup>` arrays to ordinary addressable memory, so the wrapped index becomes a wild address
and the driver reports a device loss. This candidate checks whether the same pattern exists
anywhere else PR #1817 didn't look - other `vello_shaders` shaders, `vello_hybrid`, or
`sparse_strips/vello_sparse_shaders` (the sparse-strips/CPU-tiling renderer's own WGSL/WESL
shaders and their Rust CPU-fallback mirrors) - and whether issue #1465 (a separate Mali-TKRX page
fault, `with_winit` example, "any scene containing a path" on first frame) is an independent,
uncovered instance of the same class.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Exhaustive grep of every `select(` call in the repo's WGSL/WESL shader sources, classified by hand | Static, no build; covered `vello_shaders/shader/**/*.wgsl` (all 20 files, ~60 total `select()` calls, including multi-line calls) and `sparse_strips/vello_sparse_shaders/shaders/**/*.wesl` (all 16 files, ~15 `select()` calls) | Exactly 6 `select()` calls anywhere in the repo have the vulnerable shape (an index expression `<var> - 1u` inside a `select()` guarded by `<var> > 0u`) - and those 6 are exactly the 6 sites PR #1817 already patches (`tile_alloc.wgsl:107`, `backdrop_dyn.wgsl:76`, `coarse.wgsl:251,315`, `fine.wgsl:223,579`). No 7th site, in `vello_shaders` or elsewhere. See `repro/check_sibling_sites.sh` steps 1-2 and `output.txt`. |
| Manual read of every other `select()` call not caught by the "`- 1u]`" grep (the remaining ~54+~15 calls) | Static | None of the rest index any array/buffer at all (they select between scalars/vectors: blend-mode math, sign flips, gradient-quadrant folding, MSAA sample-mask packing) - so none can exhibit this bug class regardless of guard shape. Separately confirmed the workgroup-array prefix-sum lookups elsewhere in the repo that *do* index with a `- 1u` offset (`pathtag_scan.wgsl:72`, `draw_leaf.wgsl:103`, `clip_leaf.wgsl:43,56`) are all guarded by a plain `if`, not `select()` - `if`/`else` in WGSL, like Rust, only evaluates the taken branch, so these were never at risk from this specific double-evaluation mechanism (`clip_leaf.wgsl`'s two sites are additionally structurally safe regardless: the guard `(ix >> j) & 1u != 0u` already implies `ix >> j >= 1u`, so `(ix >> j) - 1u` can't underflow even if it *were* reachable unconditionally). |
| Architectural check: does `sparse_strips/vello_sparse_shaders` (vello_hybrid's shader crate) even have compute-shader prefix-sum kernels of this shape at all? | `grep -rl 'var<workgroup>'` and `grep -rl '@compute'` across `sparse_strips/` | Zero matches for both. The sparse-strips/hybrid renderer does all tiling/strip-generation prefix-sum work on the CPU (`sparse_strips/vello_common`); its GPU shaders (`render.wesl`, `blend.wesl`, filters, etc.) are vertex/fragment-only, compositing already-computed strip data - there is no workgroup array or compute dispatch anywhere in that crate for this bug class to live in. See `repro/check_sibling_sites.sh` step 4. |
| CPU-shader mirror check: `vello_shaders/src/cpu/*.rs` (the CPU fallback implementations of the same classic-pipeline shaders) | `grep -n "el_ix - 1\|part_ix - 1\|- 1\]"` across `backdrop.rs`, `coarse.rs`, `fine.rs`, `tile_alloc.rs` | No matches - the CPU mirrors implement the equivalent prefix-sum logic with plain Rust loops/`if`, which (like WGSL `if`) only evaluates the taken branch, so this exact bug class (unconditional evaluation of a discarded branch) cannot occur there independent of index value. |
| Confirm the 6 known sites are genuinely live/exploitable in the current, unpatched tree (not dead code some other guard already neutralizes) | `grep -n "ShaderRuntimeChecks::unchecked\|create_shader_module_trusted" vello/src/wgpu_engine.rs` | Confirmed: the classic full-GPU pipeline's compute shaders are compiled via `device.create_shader_module_trusted(..., wgpu::ShaderRuntimeChecks::unchecked())` (`vello/src/wgpu_engine.rs:848-853`), i.e. no runtime bounds checks are inserted - matching PR #1817's own description ("the shaders run unchecked since #1093"). The 6 sites are real, currently-unpatched OOB reads in this checkout (`75168314`), not already-mitigated dead code. |
| Read issue #1465 (`with_winit` crashes on Mali-TKRX, MediaTek 8196, v1.r49p) for whether it's plausibly an independent/uncovered site vs. the same root cause | `gh issue view 1465` | Same symptom signature as #1816/PR #1817: a Mali-family GPU, the classic `with_winit` example (same full-GPU pipeline, same `ShaderRuntimeChecks::unchecked()` compute path examined above), and the reporter's own workaround ("enable bounds checks in the shader runtime 'fixes' this") point at exactly the same mechanism PR #1817 targets - an unchecked OOB read in one of the classic pipeline's compute shaders. Nothing in #1465's report narrows it to a *different* shader/site than the 6 PR #1817 already covers; most plausible explanation is it's the same underlying bug on a different Mali chip, not a 7th uncovered site. |

## Status

**Not reproducible** (as an *additional, uncovered* instance of this pattern - the specific
hypothesis this candidate investigated). The underlying bug PR #1817 targets is itself real and
currently unpatched in this checkout (confirmed by direct inspection of the 6 sites - none of them
have PR #1817's `max(ix, 1u) - 1u` clamp applied yet), but that is already tracked by upstream
issue #1816 and has an open, unmerged fix (PR #1817); this candidate specifically checked for
*sibling* occurrences the PR's own 6-site fix doesn't cover, and found none anywhere in the repo
(`vello_shaders`, `vello_hybrid`, `sparse_strips/vello_sparse_shaders`, or the CPU-shader mirrors).

Static analysis, deterministic, re-run 3/3 times with identical output: once directly against this
worktree, once directly against the main project checkout at the same commit, and once via `just
test`'s own fresh `git clone` of `7516831426fb70f36d62c521c5477f6a7265b36a` from GitHub (see
`output.txt` for the captured run; `repro/check_sibling_sites.sh` is pure `grep`/text search over
tracked source files, so there is no source of non-determinism to account for run-to-run).

**Fully reproducible**: N/A (no bug found *by this candidate* - the sibling-coverage question is
what was checked, and it came back negative; see Status). Actually triggering the known, already-
tracked 6-site bug itself would need real Mali hardware (this environment has none), which is why
this candidate deliberately stayed at the static-analysis level for the coverage question rather
than attempting a hardware-dependent runtime repro of a bug that's already reported, root-caused,
and has an open fix upstream.

## Class

N/A (not Confirmed - see Status).

## Root cause

Not a new bug in the sense originally suspected (an *uncovered* sibling site). The reason no
additional site exists: in `vello_shaders`, every other prefix-sum-style lookup that indexes a
`var<workgroup>` array with a `- 1u` offset uses a plain `if` guard (single evaluation, safe by
WGSL/Rust-like short-circuit semantics), not `select()` - PR #1817's author appears to have found
all of the `select()`-shaped ones already. In `sparse_strips/vello_sparse_shaders`
(`vello_hybrid`'s own shader crate), the bug class cannot occur at all: that renderer does its
tiling/strip prefix-sum work on the CPU (`vello_common`) and only ever runs vertex/fragment shaders
on the GPU - there is no `var<workgroup>` array or `@compute` entry point anywhere in that crate to
carry this pattern.

## Introduced by

N/A - not applicable, since Status is Not reproducible for the hypothesis this candidate checked
(no new bug found to trace to an introducing commit). For context: the 6 known, already-tracked
sites' own history is PR #1817's concern, not this candidate's; that PR's own description says the
shaders "run unchecked since #1093" ("No shader runtime checks", disabling `wgpu`/`naga` bounds
checks for performance - confirmed still in effect at `vello/src/wgpu_engine.rs:848-853`).

## The fix

No fix attempted from this candidate - there is nothing new to fix (see Status/Root cause); the 6
already-known sites already have a fix proposed upstream in the still-open PR #1817, which this
session did not need to duplicate. No source files in the main project were modified during this
investigation (worked entirely from a git worktree/read-only greps).

- Project test suite: N/A, no change was made. Sanity baseline only (pristine tree, no patch):
  `cargo test -p vello_shaders --release` -> 0 passed; 0 failed; 1 ignored (doctest only, no unit
  tests in this crate). `cargo test -p vello_sparse_shaders --release` -> 3 passed; 0 failed; 0
  ignored.

## What's not done / open questions

- This candidate did not attempt to actually trigger the Mali device-loss/page-fault symptom
  itself (issues #1816/#1465) - that would need real Mali hardware, which this environment doesn't
  have; PR #1817's own description already states it was verified on a Pixel 9 Pro
  (Immortalis-G715), which is sufficient confirmation of the underlying mechanism without this
  session repeating it.
- Did not check whether `vello_sparse_shaders`' own use of `naga::proc::BoundsCheckPolicy::Unchecked`
  for *all* of `index`/`buffer`/`image_load`/`binding_array` (`sparse_strips/vello_sparse_shaders/src/compile.rs:87-91`,
  used for its Metal/MSL compilation) could expose a *different* class of OOB access in that
  renderer's fragment shaders (e.g. an adversarial/degenerate paint/gradient/atlas index computed
  from untrusted scene data, rather than a `select()`-arm prefix-sum artifact) - noted as a new,
  separate suspect below rather than chased in this candidate, since it's a different bug shape
  (no `select()`/double-evaluation involved) and would need its own investigation.
- If PR #1817 merges, worth a quick follow-up re-grep (`repro/check_sibling_sites.sh`) against the
  post-merge tree to confirm the count of remaining vulnerable-shape sites drops to 0 (this
  candidate's own check already establishes the expected post-merge state: 6 sites patched, 0
  left).
