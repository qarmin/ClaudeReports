# lto_driver_failure_segfault

Found incidentally while setting up this workspace's own build/test tooling (running the
project's own recommended smoke test, `./y.sh cargo build --manifest-path tests/hello-world/Cargo.toml`,
per Readme.md's "Quick start"). `tests/hello-world/Cargo.toml` sets `profile.dev.lto = "thin"` /
`profile.release.lto = "fat"`, and any crate built with an LTO-enabled profile through this
backend in this environment crashes the whole `rustc` process with SIGSEGV instead of a normal
compiler diagnostic.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Project's own documented smoke test (`tests/hello-world`, `CHANNEL=release ./y.sh cargo build --manifest-path tests/hello-world/Cargo.toml`) | debug profile (`profile.dev.lto = "thin"`), release-mode backend, freshly built sysroot | rustc dies with SIGSEGV (signal 11) every time, 2/2 runs |
| Minimal standalone repro crate (no dependency on the rest of the repo), same command | `[profile.dev] lto = "thin"`, single `fn main() { println!("hello"); }` | Same SIGSEGV, 3/3 runs |
| Reading `src/back/write.rs`'s fat-LTO codegen path | - | Found the mechanism: `-fuse-linker-plugin` is added unconditionally for fat LTO with no check that the plugin the flag names is actually available (see Root cause) |

## Status

**Confirmed** - fully deterministic, 5/5 runs total across two slightly different repro crates
(2/2 on `tests/hello-world`, 3/3 on the minimal standalone `repro/` crate in this folder), same
crash signature (SIGSEGV inside `libgccjit.so`, reached through
`gcc_jit_context_compile_to_file`) every time.

**Fully reproducible**: Yes - the repro runs the real, unmodified `rustc` + this backend through
the project's own build system (`y.sh cargo build`), on a completely ordinary Rust crate compiled
with an LTO-enabled profile (the same profile setting `tests/hello-world` itself, the project's
own documented "Verify setup" smoke test, already uses). No second participant/peer is involved -
this is a single local compiler invocation crashing on its own, so `CONFIRMATION.md` section 11
does not apply.

## Class

crash

## Root cause

When compiling with an LTO-enabled profile (`lto = "thin"` or `"fat"` - both hit this: the code
comments in `src/back/write.rs` and the rustc warning "ThinLTO is not supported by the codegen
backend, using fat LTO instead" show the backend maps both onto the same fat-LTO path), the fat-LTO
branch in `codegen()` (`src/back/write.rs:113-146`) unconditionally does:

```rust
context.add_driver_option("-fuse-linker-plugin");
```

(`src/back/write.rs:122`, introduced by commit c92f0540, see "Introduced by") with the adjacent
comment explaining *why* it's there: without it, GCC's `lto1` reports "internal compiler error:
decompressed stream: Destination buffer is too small". This flag tells GCC's driver to load its
LTO plugin (`liblto_plugin.so`) for the link/compile step that follows
(`context.compile_to_file(OutputKind::Executable, &lto_path)` a few lines later).

Nothing in this code path checks that `liblto_plugin.so` is actually present/loadable before
requesting it. In this environment - the project's own default `config.toml` setting
(`download-gccjit = true`, see `config.example.toml`) downloads a prebuilt `libgccjit.so` binary
only, with no accompanying `lib/gcc/<target>/<version>/` directory tree that a real GCC install
would normally ship `liblto_plugin.so` in - so the plugin genuinely does not exist anywhere
`libgccjit`'s internal driver-invocation code can find it. GCC's own driver correctly detects this
and reports it as a normal, well-formed error:

```
x86_64-pc-linux-gnu-gcc-17.0.0: fatal error: '-fuse-linker-plugin', but liblto_plugin.so not found
compilation terminated.
```

...but that failure is not surfaced to Rust as a normal error from `compile_to_file` - the process
instead dies with SIGSEGV a moment later, inside `libgccjit.so` itself (the crash address is inside
`libgccjit.so`, reached through `gcc_jit_context_compile_to_file`, per the backtrace in
`output.txt`). The actual memory-safety fault is therefore inside `libgccjit`/the bundled GCC
fork's own C++ driver-invocation code (not part of this repository's Rust sources), triggered by
this project's own choice to pass `-fuse-linker-plugin` without first checking whether doing so is
actually going to work in the current environment.

Separately, `lto_supported` (checked earlier in `lib.rs` via `gccjit::is_lto_supported()`, gating
whether the `-flto=auto`/`-fuse-linker-plugin` options get added at all) only asks "does this GCC
version support LTO", not "is the LTO *plugin* actually available in this installation" - so an
environment that passes the first check can still fail the second one, with no diagnostic in
between.

## Introduced by

c92f0540 "Merge commit '98ed962c7d3eebe12c97588e61245273d265e72f' into master" (2024-07-10) - a
subtree-sync merge commit (this project periodically merges from an internal/upstream subtree), so
it is the point this line entered this repo's own history rather than necessarily where it was
originally authored; `git blame` attributes `src/back/write.rs:122` to this commit and Guillaume
Gomez. The underlying missing-plugin-detection gap has existed since then; it is exposed by, not
introduced by, the `download-gccjit = true` default config path (that default itself predates this
workspace's own investigation and was not further dated).

## The fix

No bounded fix was attempted this session. Two different classes of fix exist, and picking between
them is a project-maintainer decision this session isn't positioned to make on its own:

1. **Environment/packaging fix** (outside this repo): make the `download-gccjit = true` path also
   fetch or otherwise provide a `liblto_plugin.so` at whatever path the bundled `libgccjit.so`
   expects to find it - if maintainers already know that path (likely computed from the bundled
   GCC fork's own build configuration), this is probably the "real" fix, but it isn't something
   discoverable purely by reading this repo's own Rust sources.
2. **Defensive code fix** (in this repo, `src/back/write.rs`): before calling
   `add_driver_option("-fuse-linker-plugin")` / `compile_to_file` for the fat-LTO path, verify the
   plugin is actually available (e.g. shell out to the same driver GCC would use with something
   like `-print-file-name=liblto_plugin.so` and check the result isn't just the bare filename
   echoed back) and, if it's missing, have rustc emit a clean, actionable `dcx.emit_err(...)`
   diagnostic instead of proceeding into a call already known to be capable of crashing. This
   converts a compiler crash into a normal compile error - it does not make LTO actually work in
   this environment, and was not attempted this session because reproducing GCC's own internal
   plugin-search logic (needed to make the check meaningful rather than environment-specific
   guesswork) would need more time than this session's per-candidate budget, and because a
   defensive check that itself doesn't match GCC's exact search rules risks producing false
   negatives (declaring the plugin present when GCC's own driver would still fail to find it).

- Project test suite: N/A - no fix was applied this session, so there is no before/after to state.

## What's not done / open questions

- Not checked whether the same crash occurs for every codegen unit count / every target triple, or
  is specific to the single-CGU debug build used here.
- Not checked whether building GCC from source (per `Readme.md`'s "Building with your own GCC
  version" section, i.e. not using `download-gccjit = true`) avoids this entirely by providing a
  real, complete GCC install tree (and therefore a real `liblto_plugin.so`) - if so, this is purely
  a `download-gccjit = true` quick-start-path issue, worth confirming and documenting prominently
  since `download-gccjit = true` is the project's own documented default/recommended quick-start.
- Whichever fix approach is chosen, the same "no precondition check before an operation known to be
  able to crash instead of error" pattern is worth checking for elsewhere in `src/back/write.rs`'s
  and `src/back/lto.rs`'s other `compile_to_file`/`add_driver_option` call sites (e.g. the
  bitcode-emission path a few lines above, which sets the same `-flto=auto`/`-flto-partition=one`
  options under a similar `lto_supported` condition) - not checked this session.
