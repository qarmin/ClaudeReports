# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it, `IDEAS.md` is the backlog that hasn't been.

The subject is `rustc_codegen_cranelift` itself (the compiler backend), not a library - so
"input" here means *Rust source that the backend has to codegen*, and "a bug" means the backend
crashes on it, refuses it, or emits code that computes the wrong answer.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| differential testing vs LLVM | **yes, the primary tool** | Same source, `cargo build` vs `dist/cargo-clif build`, compare run-time output. LLVM is the oracle. This is what found every confirmed miscompilation so far. |
| hodow CPU suite | **yes**, with a workaround | ~19k self-checking x86 intrinsic tests. See below - needs a `--cfg target_feature` hack under cg_clif. |
| `miri` | n/a for the backend | miri interprets MIR; it never reaches codegen, so it cannot see a cg_clif bug. Useful only for a bug *in* the backend's own Rust code, which is not what this workspace is about. |
| ASAN/UBSAN | not tried (2026-09-02) | Would instrument the compiler binary, not the generated code. Not obviously useful; the generated code's memory errors show up as wrong values/aborts under the differential test instead. |
| `./y.sh test` | yes | The project's own test suite (`config.txt` selects the items). Use it as the regression check for any patch. |
| `./test.sh` / `scripts/test_rustc_tests.sh` | yes, slow | Full rustc test-suite run against the backend. |

## Existing project harnesses worth reusing

- `example/` + `config.txt` - the project's own AOT/JIT smoke tests, run by `./y.sh test`.
- `scripts/test_bootstrap.sh`, `scripts/test_rustc_tests.sh` - heavier end-to-end checks.
- `tools/simd_probe/` (this workspace) - standalone crate, LLVM-vs-cg_clif differential probe for
  the specific SIMD gaps found so far. `cd tools/simd_probe && just probe`.
- `tools/hodow_audit/` (this workspace) - drives the full ~19k-test hodow suite, below.

## Instrumentation/tooling set up for this workspace

### Isolated worktrees (`tools/new_worktree.sh`) - verified 2026-09-03

The Bash sandbox mounts **everything outside the project root read-only**, so the usual
"put a git worktree in a temp dir" isolation does not work here: nothing can be built in it.
`tools/new_worktree.sh <NNN.name>` instead creates the worktree at `tmp/wt/<NNN.name>` (inside the
project, `/tmp/` is gitignored) and seeds it with a **copy** of the main tree's already-built
`build/` (~1.4G, ~3s on NVMe) and `download/` if present.

That copy is what makes per-candidate isolation affordable: the cargo caches for the Cranelift
dependency tree and the sysroot sources come along, so a first `./y.sh build` in a fresh worktree
took **25s** (measured 2026-09-03, 24 cores) instead of a full cold build. `./y.sh test` took a
further **43s** for the in-tree items.

A real copy, not `cp -al` hardlinks - a hardlinked target dir would let a candidate's `rustc`
truncate a file the main tree still depends on. `dist/` is deliberately *not* copied: `./y.sh
build` regenerates it, and its wrappers bake in an absolute path.

Each worktree is ~2.7G after a build, so remove them (`git worktree remove --force`) between
batches rather than accumulating them.

### `tools/diffcheck.sh` - the fast differential loop - verified 2026-09-03

Compiles a **single-file** Rust program with plain `rustc` and with `dist/rustc-clif`, runs both,
diffs the run-time output; exit 1 means they disagree or cg_clif refused to compile. ~2s per
iteration, no cargo, no crate skeleton - this is the loop to use for finding and minimising a
miscompilation.

```sh
bash ___BUGHUNT___/tools/diffcheck.sh repro.rs                       # main tree's dist/ = unpatched
CLIF_ROOT=tmp/wt/003.foo bash ___BUGHUNT___/tools/diffcheck.sh repro.rs   # a patched worktree
RUSTFLAGS_EXTRA='-Ctarget-cpu=native' bash ... ; OPT=1 bash ...      # native ISA / the -O path
```

It defaults to **opt-level 0** on purpose (see the gotcha above about MIR GVN const-folding
intrinsics at `-O` and hiding the bug). Compile diagnostics are printed but excluded from the
comparison, since cg_clif legitimately warns about unsupported intrinsics where LLVM does not.



### The hodow audit (`tools/hodow_audit/`)

[hodow](https://github.com/qarmin/hodow) (`~/Projekty/Rust/hodow`) generates ~19k self-checking
x86 intrinsic tests with expected values captured on real hardware. Pointing it at cg_clif gives a
direct implemented / unimplemented / miscompiled breakdown.

Two things have to be worked around:

1. **`cfg(target_feature)`.** hodow's test registry is gated on `#[cfg(target_feature = "...")]`,
   and cg_clif's `target_config` (`src/lib.rs`) reports only `fxsr/sse/sse2/x87` while ignoring
   `-C target-feature` entirely. Without a workaround only ~296 SSE/SSE2 tests register instead of
   19154. Fix: set the cfgs by hand -
   `RUSTFLAGS='-A explicit_builtin_cfgs_in_flags --cfg target_feature="avx2" ...'`. The `-A` is
   required: `--cfg target_feature=...` is denied by default
   (`explicit_builtin_cfgs_in_flags`). `build_hodow_clif.sh` derives the list from
   `rustc --print cfg -Ctarget-cpu=native` so the registered test set matches the LLVM baseline
   exactly.
2. **Build in debug, never `--release`.** At `-O`, rustc's MIR GVN can const-fold intrinsic calls
   through the const-eval interpreter, so a miscompiled intrinsic silently produces the *right*
   answer at compile time and the backend is never exercised.

Then `hodow_cli cpu --isolate` runs each test with per-test subprocess isolation, so an
unimplemented intrinsic (which aborts via `codegen_panic_nounwind`) is attributed to that one test
instead of killing the run.

```sh
# build hodow against cg_clif (CLIF/HODOW/OUTDIR/TARGET_CPU_FLAG are overridable env vars)
bash ___BUGHUNT___/tools/hodow_audit/build_hodow_clif.sh > build_clif.log 2>&1
cd ~/Projekty/Rust/hodow && <outdir>/debug/hodow_cli cpu --isolate > run_clif.log 2>&1
# LLVM baseline for comparison (must be 19154/19154 pass)
CARGO_TARGET_DIR=<dir> RUSTFLAGS=-Ctarget-cpu=native cargo build -p hodow_cli --no-default-features

python3 ___BUGHUNT___/tools/hodow_audit/summarize.py    run_clif.log run_llvm.log   # per-extension
python3 ___BUGHUNT___/tools/hodow_audit/per_intrinsic.py run_clif.log all_tests.txt # per-intrinsic
```

Compile-time gaps come straight out of the build log and are worth extracting every run:

```sh
grep -oP "unsupported (x86 )?llvm intrinsic \K[a-zA-Z0-9_.]+" build_clif.log | sort -u
grep -oP "Unknown SIMD intrinsic \K\w+"                       build_clif.log | sort -u
grep -oP "Error while defining \K[^:]+"                       build_clif.log | sort -u
```

**Three cg_clif bugs block a complete run** (see `IDEAS.md` rows 3-5): the missing
`simd_funnel_shl`/`_shr` is a hard error, the `_mm*_stream_*` family ICEs, and the AVX-512 mask
register inline asm doesn't assemble. `measurement_patches.diff` in this directory turns the first
two into a warning + run-time trap and fixes the third, purely so the remaining ~19k tests can be
measured. **It is a measurement aid, not a fix** - the first two are counted as "unimplemented" in
the results, so it does not inflate the numbers. Apply it, run, then `git checkout -- src/` and
rebuild (`./y.sh build`) - never leave it applied.

### 2026-09-02 baseline

Ryzen 9 7900 (Zen 4, full AVX-512), rustc nightly-2026-08-24, cg_clif @ 2bc05d45 (Cranelift 0.135),
hodow debug + `--no-default-features`. Raw logs are in this directory (gzipped).

| | tests | distinct intrinsics |
|---|---:|---:|
| works | 13833 (72.2%) | 3116 (60.1%) |
| unimplemented (traps at run time) | 5318 (27.8%) | 2066 (39.8%) |
| wrong value | 3 | 3 |
| total | 19154 | 5185 |

Compile-time: 701 distinct unsupported `llvm.x86.*` (518 avx512), 2 unsupported generic SIMD
intrinsics, 8 functions Cranelift can't compile at all. A second run on the default `x86-64` ISA
(no `-Ctarget-cpu=native`) was identical except for one extra trap
(`test_x86_64_cmpxchg16b_cmpxchg16b`).

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
5. For a backend, "verified" means the *generated code* was run and its output compared against
   LLVM's on the same source. A patch that makes the compiler stop erroring is not verified until
   the program it produced ran and printed the right answer.
