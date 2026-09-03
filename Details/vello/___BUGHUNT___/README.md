# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `git worktree add`/`git clone` | **Broken by default in this sandbox** - see below | `git-lfs` is not installed on `PATH`, but `.gitattributes` marks `vello_tests/snapshots/*.png` and `sparse_strips/vello_sparse_tests/snapshots/*.png` as `filter=lfs ... required=true`. Any fresh checkout that touches those paths (a new `git worktree add`, a fresh `git clone`) fails with `git-lfs filter-process: 1: git-lfs: not found` / `fatal: the remote end hung up unexpectedly` (exit 128), and can leave a partially- or non-created directory. The **already-checked-out main project tree is unaffected** (`git status` etc. work fine there) - this only bites *new* checkouts. |
| `cargo test`/`cargo build`/`miri`/`cargo clippy` | Works normally | Not affected by the LFS issue once a worktree/clone is actually checked out (see workaround below). |
| GPU-based tests (`vello_hybrid` via real `wgpu`) | **Works** (candidate 026, 2026-08-31) - see below | A real (software) Vulkan adapter *is* available in this sandbox: Google's SwiftShare CPU implementation, ICD JSON at `/tmp/swiftshader_icd/vk_swiftshader_icd.json` (found via `find / -iname 'libvk_swiftshader*'` - it's already installed, just not registered as the default Vulkan ICD). Set `VK_ICD_FILENAMES=/tmp/swiftshader_icd/vk_swiftshader_icd.json` and request a `wgpu::Backends::VULKAN` adapter to get a real `wgpu::Adapter`/`Device`/`Queue` (`AdapterInfo { name: "SwiftShader Device (Subzero)", device_type: Cpu, backend: Vulkan, .. }`, `max_texture_dimension_2d: 8192`). This supersedes an earlier (024) session's note that no GPU/Vulkan adapter was available - that session apparently didn't search for one. `cargo test -p vello_sparse_tests --test tests -- hybrid` *runs* under this adapter, but its image-diff assertions still fail en masse (536/610 in candidate 026's session) because the reference PNGs are unresolved `git-lfs` pointer stubs in this sandbox (see the `git-lfs` row above) - not a SwiftShader rendering-fidelity problem. Useful for exercising real `vello_hybrid` wgpu-backend code paths (resource limits, atlas allocation, encoder/submit behavior) even though full snapshot-based regression testing isn't usable here. |

### Workaround: creating a usable worktree/clone without `git-lfs`

Use a **worktree-private sparse-checkout** that excludes the two LFS-tracked snapshot directories,
so git never invokes the (missing) LFS smudge/clean filters at all:

```bash
# 1. Create the worktree/clone WITHOUT checking anything out yet.
git worktree add --no-checkout /tmp/vello_worktrees/NNN_name HEAD
# (for a plain `git clone`, use `git clone --no-checkout <url> clone` instead)

cd /tmp/vello_worktrees/NNN_name

# 2. Enable per-worktree config (harmless, additive, does not affect the main tree or other
#    worktrees - only needs doing once per repo, `git config extensions.worktreeConfig true`
#    in the MAIN tree first if not already set).
git config --worktree core.sparseCheckout true

# 3. Exclude the two LFS-tracked patterns (this file lives in the worktree's OWN private
#    info/ dir - `git rev-parse --git-path info/sparse-checkout` - never the shared .git/config).
SP_FILE=$(git rev-parse --git-path info/sparse-checkout)
mkdir -p "$(dirname "$SP_FILE")"
printf '/*\n!/vello_tests/snapshots/*.png\n!/sparse_strips/vello_sparse_tests/snapshots/*.png\n' > "$SP_FILE"

# 4. Materialize the working tree. NOTE: `git checkout HEAD -- .` does NOT respect sparse-checkout
#    (it's an explicit-pathspec restore, not a ref switch) and will still try to check out the
#    excluded LFS paths, failing the same way. Use `git read-tree -mu HEAD` instead.
git read-tree -mu HEAD
```

Caveats:
- This means `sparse_strips/vello_sparse_tests` snapshot-comparison tests (`cargo test -p
  vello_sparse_tests`) cannot actually run in a worktree set up this way (the reference PNGs are
  sparse-excluded) - use `vello_common`/`vello_cpu`'s own `--lib` unit tests instead for
  non-snapshot regression coverage, and note this limitation explicitly in any `RESULT.md` that
  would otherwise want to run the snapshot suite.
- A candidate's own `justfile` (`just clone`) does a plain `git clone` and will hit the same LFS
  failure in *this* sandbox - it is written the normal way (no workaround baked in) because a
  reader's own machine will typically have `git-lfs` installed. If validating a candidate's
  `justfile` end-to-end from *this* sandbox, apply the same `--no-checkout` + sparse-checkout +
  `read-tree` sequence manually to the `clone/` directory it creates, rather than editing the
  justfile itself.
- `extensions.worktreeConfig = true` was enabled once (2026-08-31) in the main project's shared
  `.git/config` to make step 2 above possible - this is additive/opt-in and does not change any
  existing worktree's behavior on its own.

## Existing project harnesses worth reusing

- `sparse_strips/vello_sparse_tests/tests/*.rs` - real end-to-end rendering snapshot tests via
  `vello_dev_macros::vello_test`, covering both `vello_cpu` and `vello_hybrid` backends. Requires
  the LFS-tracked reference PNGs (see caveat above).
- `sparse_strips/vello_example_scenes` - the actual example-scene gallery (`FunkyPathsScene`,
  `TrickyStrokesScene`, etc.) shared between the native winit demo, the wasm/webgl demo, and (with
  the `cpu` feature) directly embeddable in a standalone `cargo run` repro via
  `vello_cpu::RenderContext` - this is the real, public entry point to use for any candidate about
  a specific example scene's rendering, rather than hand-copying its `BezPath` construction.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every `Confirmed` candidate has a real,
   standalone `repro/` and the exact command that runs it.
2. Every candidate with a fix gets a real `patch.diff`, applied and built for real, plus the
   project's own tests where they cover the code.
3. `Not reproducible`/`Inconclusive` results are written down with the same care as confirmed
   bugs - a result with no file is indistinguishable from "never checked".
4. If a later candidate changes an earlier one's confirmation status, don't silently rewrite that
   candidate's `RESULT.md` - add a `CORRECTION.md` in its folder and update its `SUMMARY.md` row
   to point at it.
