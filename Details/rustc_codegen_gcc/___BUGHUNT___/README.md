# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## What this project is

`rustc_codegen_gcc` is an alternative codegen backend for `rustc` (replacing LLVM with libgccjit).
Most of the interesting bug surface here is not "malformed adversarial input" in the usual
parser/decoder sense - it's **codegen correctness**: does a given Rust source construct get
translated into GCC IR that behaves the same way the LLVM backend's output would (division sign,
casts, SIMD, atomics, overflow checks, unwinding, enum layout, intrinsics), and does the backend
ever crash instead of emitting a clean diagnostic. `fuzz-bug-repro`-style raw byte fuzzing doesn't
apply well here; differential testing (does a small Rust program produce the same result compiled
with this backend as with normal rustc/LLVM) and boundary-value construction around known-tricky
codegen (see the division-sign class of bug below) are the two techniques that actually fit.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| Full project build (`./y.sh prepare && ./y.sh build --sysroot --release`) | Yes | ~2-3 min total in this environment (24 cores): downloads a prebuilt `libgccjit.so` via `download-gccjit = true` (the project's own default, see `config.example.toml`), then builds the backend (~15s) and the sysroot (`core`/`alloc`/`std`/... , ~1 min). |
| LTO-enabled builds (`profile.dev.lto = "thin"`, `profile.release.lto = "fat"` - i.e. `tests/hello-world`'s own Cargo.toml, the project's documented "Verify setup" smoke test) | **No - crashes rustc with SIGSEGV** | See candidate `001.lto_driver_failure_segfault`. Root cause: `download-gccjit = true` only downloads the bare `libgccjit.so`, with no accompanying GCC install tree, so `liblto_plugin.so` (needed for fat LTO) doesn't exist anywhere libgccjit's internal driver can find it - and the failure isn't handled gracefully, it segfaults. **Any candidate/repro that doesn't specifically target this bug must disable LTO** (set `[profile.dev] lto = "off"` / avoid `[profile.release] lto = "fat"` in its own `Cargo.toml`, or export `CARGO_PROFILE_DEV_LTO=off` when invoking `y.sh cargo build`) or it will hit this same crash instead of exercising whatever it's actually testing. |
| `miri` | Not applicable to most of this codebase in the usual sense - this backend doesn't run user code through `unsafe`/FFI in the way a typical crate does; its own `unsafe` usage is calling into `gccjit`/`gccjit_sys` (an FFI wrapper around libgccjit's C API), which `miri` cannot run through (it isn't pure Rust it can interpret - `gccjit_sys` links a real C library). Not attempted this session. |
| ASAN/UBSAN on the backend itself | Not attempted this session | Would require rebuilding rustc_codegen_gcc's own Rust code with a sanitizer, which is plausible in principle (`RUSTFLAGS=-Zsanitizer=address`) but the actual bug surface for FFI-into-libgccjit issues is more likely to live in libgccjit's own C++ code (not part of this repo, not this skill's target) than in this crate's own (mostly safe) Rust code. Lower priority than differential/codegen-correctness testing. |
| A short one-off fuzz harness | Not a good fit for most of this codebase (see "What this project is" above) - the interesting bugs are codegen-correctness bugs found via constructing a *specific, reasoned* input (a value whose GCC-level declared signedness doesn't match its semantic type, e.g. after a `transmute`/cast/SIMD reinterpretation), not via random byte mutation. `tests/run/ptr_to_int_div.rs` (added by commit 6f389ff) is the project's own template for this style of test. |

## Existing project harnesses worth reusing

- `tests/run/*.rs` - `lang_tester`-driven run tests (see `tests/lang_tests.rs`, `tests/lang_tests_common.rs`) - compile-and-run tests with a `// Run-time: status: N` header. This is the fastest way to construct and check a codegen-correctness repro that behaves like the project's own test suite - candidates should follow this convention (see `tests/run/ptr_to_int_div.rs` for the closest existing example to the division-sign bug class).
- `tests/hello-world` - the project's own documented smoke test (`Readme.md` "Quick start") - **do not use directly for a new candidate's `repro/` without disabling LTO first**, see the LTO row above.
- `./y.sh test --release` - runs the project's full test suite; several tests are pre-known-failing (`tests/failing-*.txt` lists) - don't treat every failure in those lists as a new finding, cross-check first.
- `___BUGHUNT___/tools/diffgen/` - this workspace's own differential-testing harness (casts/overflow/floatint/bitops/simd/atomics/enum_niche/panic_unwind/generics_vtable against plain rustc, plus Miri as a third independent oracle - `MIRI=0 ./run_diff.sh` to disable if `cargo miri setup` was never run) - see `~/.claude/skills/bug-hunt/projects/rustc_codegen_gcc.md`'s "Differential-testing tooling" section for full usage and results so far.
- **`./y.sh fuzz --count N -j <threads>`** - the project's own first-party `rustlantis` fuzzing integration (MIR-level, terminating/UB-free-by-construction generated programs, debug-LLVM vs release-GCC comparison, auto-reduces mismatches into `target/fuzz/runtime_error/`). **Needs a one-time setup step not mentioned in `./y.sh fuzz --help`**: `cp clones/rustlantis/config.toml.example clones/rustlantis/config.toml` (clone happens automatically on first run) - without it, `./y.sh fuzz` doesn't error, it silently loops printing `fuzzed 0 cases(0%)` forever (looks exactly like a hang). See `~/.claude/skills/bug-hunt/projects/rustc_codegen_gcc.md`'s "rustlantis fuzzing" section - this found a real LLVM-vs-GCC mismatch within the first ~10 generated cases the one time it was actually run this session, higher hit rate than either `diffgen` or manual code reading at this point.
- `~/Projekty/Rust/hodow` (external project, **read-only - copy to a scratch dir before building, never write there directly**) - `hodow_core`'s real `std::arch` x86/ARM intrinsic test suite (~1700 tests, own correctness oracle, `--isolate` crash safety) is dramatically more thorough than `diffgen` for intrinsic-level bugs and already found a new one this session (`_mm_subs_epi8`/`_mm_subs_epi16`, see `IDEAS.md`) - see the same "Differential-testing tooling" section for exact build/run commands.

## Instrumentation/tooling set up for this workspace

Nothing beyond the project's own documented `./y.sh prepare && ./y.sh build --sysroot --release`
(no custom sanitizer/miri/fuzz setup was built this session - see the toolchain table above for
why those didn't fit well for a first session on this project).

**Important for every candidate**: after the one-time `./y.sh build --sysroot --release` in the
main tree, running `<repro>`'s own compile only needs `CHANNEL=release ./y.sh cargo build
--manifest-path <repro>/Cargo.toml` (or `./y.sh cargo run ...`) - this reuses the already-built
sysroot (`build/build_sysroot/`) and only rebuilds the backend `.so` itself if `src/` changed
(fast: ~15s, since `[profile.dev.package.rustc_codegen_gcc] opt-level = 0` in the root
`Cargo.toml`). **Do not run `./y.sh build --sysroot --release` again per candidate** - the sysroot
almost never needs rebuilding unless the change under test actually affects how `core`/`alloc`/
`std` themselves get compiled (rare for a scoped bug fix).

**In a parallel-batch worktree, symlink only `build/libgccjit` and `build/build_sysroot`
individually** (not the whole `build/` directory) - copy `config.toml` into the worktree root, then
recreate just those two symlinks (`ln -s <main_tree>/build/libgccjit build/libgccjit`, same for
`build_sysroot`) before running `./y.sh build --release` there. **Do NOT symlink `build/rand`,
`build/regex`, or `build/simple-raytracer`** - unlike `build_sysroot`/`libgccjit`, these are the
fixture crates `./y.sh test --extended-tests` (part of the unqualified `./y.sh test --release`)
actually builds/tests *into their own `target/` directories*, so two worktrees sharing them via the
same symlink race each other and can produce a spurious hash-mismatch/link failure - **hit for
real** in this workspace's first parallel session (2026-09-02, candidate 005, see its RESULT.md).
**Correction, dated 2026-09-02 (hit for real a second time, during a `diffgen` session running
concurrently with candidate 032's own regression check)**: `--test-libcore` ALSO writes into the
shared `build/build_sysroot` itself, not just `build/rand`/`build/regex`/`build/simple-raytracer` -
it runs `cargo test` inside `build/build_sysroot/sysroot_src/library/coretests`, and something in
that path ends up rewriting/truncating the precompiled `libcore-*.rlib`/`librustc_std_workspace_core-*.rlib`
under `build/build_sysroot/sysroot/lib/rustlib/.../lib/` while it runs (observed directly: another
worktree's `./y.sh test --test-libcore --mini-tests --cargo-tests` run corrupted that exact file for
several seconds, causing an unrelated `rustc`/`rust-lld` invocation reading the shared symlinked
`build_sysroot` at the same moment to fail with `Archive::children failed: truncated or malformed
archive`). **The earlier claim that `--test-libcore`/`--mini-tests`/`--cargo-tests` are all safe to
run concurrently against a shared symlinked `build_sysroot` was wrong for `--test-libcore`
specifically** - treat `./y.sh test --test-libcore` (and its release variant) the same as
`--extended-tests`: never run it against a `build_sysroot` shared via symlink with another
concurrently-running candidate. `--mini-tests`/`--cargo-tests` alone (without `--test-libcore`) have
not been observed to touch `build_sysroot`'s own precompiled files and remain the safer default for
a candidate's regression check; if `--test-libcore` coverage is specifically needed, either don't
symlink `build_sysroot` for that candidate's worktree (give it its own private copy/rebuild instead)
or ensure no other candidate in the same batch is concurrently reading the shared one.

A candidate that specifically does need `--extended-tests` should either run it against a locally-cloned (not shared) copy of
just that fixture crate, or must not run it in the same batch window as any other candidate that
also needs it.

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
