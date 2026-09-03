# Reproduction: OOB read in `Builder::shuffle_vector`'s mask-padding branch

`src/main.rs` exercises `std::simd::simd_swizzle!` (portable-simd's own public macro, backed by
`core::intrinsics::simd::simd_shuffle`) with three cases that broadcast a shorter vector to a
longer one:

- **Case A** (1 lane -> 2 lanes): does **not** trigger the bug (mask length <= 2x input length).
- **Case B** (1 lane -> 4 lanes): triggers the bug - this is the same shape as GitHub issue #605
  (`u8x4::splat` tripping `-fsanitize=undefined` inside `swizzle.rs`), reproduced here through the
  still-live `Swizzle::swizzle` path rather than `Simd::splat` itself (see RESULT.md's "Why not
  `Simd::splat` directly" for why the exact splat path issue #605 hit no longer applies on the
  toolchain pinned by this repo, while the underlying `shuffle_vector` bug still does).
- **Case C** (2 lanes -> 8 lanes, `u32` instead of `u8`): same bug class, different element width
  and lane count, to satisfy `CONFIRMATION.md` section 7 (vary the input, not just the one byte
  sequence that first triggered it).

In all three cases the **final numeric result is correct** (the bug is a pure out-of-bounds *read*
whose result is never actually selected by the shuffle mask - see RESULT.md's root cause) - so a
plain `cargo build && cargo run` will build and print `ok` whether or not the bug is present. The
bug only becomes observable with `-fsanitize=undefined` instrumentation on the *compiled program*,
which `rustc_codegen_gcc` has no built-in flag for (`-Zsanitizer=` is not implemented by this
backend). This repro adds a tiny opt-in hook (`ubsan_hook.patch`, gated behind an env var so it
changes nothing when unset) to `rustc_codegen_gcc`'s own `src/gcc_util.rs` to forward
`-fsanitize=undefined` into libgccjit's context when `CG_GCCJIT_UBSAN=1` is set - this is
**instrumentation for observing the bug, not part of the fix** (`patch.diff` is the fix, and is
independent of this hook).

## Prerequisites

- A checkout of `rustc_codegen_gcc` at `base_commit` (see `../justfile`), built once with
  `./y.sh prepare && ./y.sh build --sysroot --release` (`config.example.toml` copied to
  `config.toml`, i.e. `download-gccjit = true`).
- GCC's own `libubsan` available on the host (`gcc -print-file-name=libubsan.a`) - present on any
  ordinary Debian/Ubuntu box with `gcc` installed (`libubsan1` package). Not clang's compiler-rt
  (this repro explicitly forces `-Clinker=gcc` via `CG_RUSTFLAGS` below, because clang's own
  `libclang_rt.ubsan_standalone.a` is commonly missing even when clang itself is installed).

## Reproducing

```sh
# From a checkout of rustc_codegen_gcc with the sysroot already built:
patch -p1 < /path/to/this/repro/ubsan_hook.patch   # instrumentation hook, not the fix
./y.sh build --release                              # rebuild the backend .so (~10s)

cd /path/to/this/repro
# CG_RUSTFLAGS (not RUSTFLAGS/.cargo/config.toml - y.sh's own build_system unconditionally
# overwrites the RUSTFLAGS env var it passes to cargo, so only its own CG_RUSTFLAGS escape hatch
# actually reaches the final link step) forces linking with `gcc` + GCC's libubsan.
CG_GCCJIT_UBSAN=1 CG_RUSTFLAGS="-Clinker=gcc -Clink-arg=-fsanitize=undefined -Clink-arg=-lubsan" \
  CHANNEL=release /path/to/rustc_codegen_gcc/y.sh cargo build --manifest-path Cargo.toml
./target/debug/shuffle_oob_repro
```

On the unpatched (pre-`patch.diff`) backend, this prints multiple UBSan diagnostics like:

```
core/src/../../portable-simd/crates/core_simd/src/swizzle.rs:88:13: runtime error: load of address 0x7ffc5b8b84e1 with insufficient space for an object of type 'unsigned char'
```

interleaved with (and not affecting) the program's own correct output (`ok`, no panics). See
`../output.txt` for a full captured run. Applying `patch.diff` (`git apply -p1 < patch.diff` from
the `rustc_codegen_gcc` checkout root) and rebuilding the backend eliminates every diagnostic while
keeping the program's own output identical - see `../justfile`'s `reproduce-after` for the same
before/after cycle scripted end to end.

## Why this is a real, standalone repro (not a fake second-participant repro)

This is a single-process, single-participant repro (`CONFIRMATION.md` section 11 does not apply):
`simd_swizzle!` is portable-simd's genuine public API, called the same way any real user of
`#![feature(portable_simd)]` would call it - there is no second participant (peer/caller) being
impersonated here, the "input" is just the Rust source being compiled, exactly as a real program
using this API would be.
