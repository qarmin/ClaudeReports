# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

- Host: x86_64, real AVX2/AVX-512 (F/BW/CD/DQ/VL/VBMI/VNNI/...) and SSE4.1/4.2 hardware - not an
  emulated/QEMU environment, so SIMD kernel bugs reachable only via AVX2/AVX-512 code paths are
  directly exercisable here without cross-compiling. NEON/SVE2 (ARM) and WASM32 code paths in
  `src/neon`, `src/sve2`, `src/wasm32` **cannot** be exercised natively on this machine.
- `rust-toolchain.toml` pins `channel = "nightly"` for the whole repo already - no separate
  `+nightly` toolchain juggling needed.
- `cargo-fuzz 0.12.0` is installed and builds all 16 targets fine, but **its default ASan build
  hangs almost immediately when actually run in this sandbox** (see candidate `001` -
  pegged at ~100% CPU right after `INITED`, zero fuzzing progress, `-max_total_time` not honored,
  has to be force-killed). Confirmed via multiple angles that this is an environment/tooling
  limitation and not a pic-scale bug: pic-scale's own resize code runs correctly and fast both
  with and without `-Zsanitizer=address` when called directly (bypassing cargo-fuzz/libFuzzer); a
  trivial ASan "hello world" binary runs fine; the hang is specific to the
  {cargo-fuzz/libFuzzer driver} + {AddressSanitizer} combination. `ASAN_OPTIONS=detect_leaks=0`
  (the usual fix for LeakSanitizer hanging under ptrace-restricted sandboxes - this container has
  `yama/ptrace_scope=1`) only avoided the hang on 1 of 11 trials, so it's not a reliable fix.
  **Practical workaround: `cargo +nightly fuzz run --sanitizer none <target>`** - keeps
  libFuzzer's coverage-guided fuzzing and cargo-fuzz's default debug-assertions, drops ASan. This
  is what actually works for fuzzing this project in this sandbox; a real ASan-covered run would
  need a different host/sandbox (or further root-causing with `strace`/`perf`, neither installed
  nor installable here - no root).
- `miri` (`miri-x86_64-unknown-linux-gnu`) is installed, but expected low value here: most of this
  crate's `unsafe` is architecture-specific SIMD intrinsics (`core::arch::x86_64::*`,
  `core::arch::aarch64::*`), and miri does not support the majority of those intrinsics - it will
  usually just fail to run rather than produce a verdict. Worth a quick try only on `unsafe` that's
  plain pointer/slice manipulation without SIMD intrinsics in the same function.
- No ASAN/UBSAN standalone (non-fuzz) build config exists in this repo yet - `cargo-fuzz` is the
  practical way to get ASan coverage; a hand-rolled `RUSTFLAGS=-Zsanitizer=address` build against
  the library directly is possible if a candidate needs to sanitizer-check something outside the
  fuzz targets' reach.

| tool | works here? | notes |
|---|---|---|
| cargo-fuzz (ASan, default) | **no - hangs** | see above; use `--sanitizer none` instead |
| cargo-fuzz (`--sanitizer none`) | yes | 16 existing targets in `fuzz/`, see below; keeps coverage-guided fuzzing + debug-assertions, no ASan memory-safety coverage |
| miri | limited | fails on most SIMD intrinsic code; only useful for non-SIMD unsafe |
| ASAN/UBSAN standalone | possible, not pre-wired | `RUSTFLAGS="-Zsanitizer=address"` + nightly `-Z build-std` if needed outside fuzz |
| debug-assertions build | yes | plain `cargo build` (dev profile) - overflow checks on by default |
| NEON/SVE2/WASM32 paths | not testable here | ARM/WASM-only code, this host is x86_64 |

## Existing project harnesses worth reusing

`fuzz/` (cargo-fuzz, `fuzz/Cargo.toml`) already defines 16 targets, each its own binary under
`fuzz/<name>/<name>.rs`: `resize_rgba`, `resize_rgb`, `resize_cbcr8`, `resize_cbcr16`,
`resize_cbcr_f16`, `resize_cbcr_f32`, `resize_plane`, `resize_plane_u16`, `resize_plane_f32`,
`resize_rgba_u16`, `resize_rgba_f16`, `resize_rgba_f32`, `resize_rgb_u16`, `resize_rgb_f16`,
`resize_rgb_f32`, `colorspaces`. Prefer running these for a few minutes each over writing a new
harness from scratch - only write a new one-off harness for an entry point none of these 16 cover.
Run from `fuzz/`: `cargo +nightly fuzz run --sanitizer none <target> -- -max_total_time=<seconds>`
(plain `cargo +nightly fuzz run <target>` uses the default ASan build, which hangs - see above).
The `colorspaces` target additionally needs `--features colorspaces` to build at all (its
`pic-scale` dependency line in `fuzz/Cargo.toml` doesn't enable it by default). To exercise the
SIMD code paths this host can actually run, add `--features avx,sse,nightly_avx512`.

## Candidate-justfile gotcha: cargo workspace ancestor-climbing when `clone/` nests under this repo

A candidate `justfile`'s `clone` recipe that clones into a plain subdirectory of this same
checkout (e.g. `NNN.NAME/clone/`) can break `cargo` builds that touch `fuzz/`, because `fuzz/`
has no `[workspace]` table of its own and relies on climbing ancestor directories to discover it
isn't part of one. `pic-scale`'s own root `Cargo.toml` excludes `"fuzz"` from its workspace, but
cargo's ancestor search does not stop at a workspace root that excludes the current package - it
keeps climbing past it. Normally that's harmless (nothing further up `/home/.../pic-scale/` is a
workspace), but if the clone lives nested inside this very checkout's own directory tree, the
climb reaches *this* checkout's real root `Cargo.toml` several directories further up and wrongly
folds the cloned `fuzz/` crate into *this* checkout's workspace instead
(`error: current package believes it's in a workspace when it's not`). Hit and fixed in candidate
`014`'s `justfile` by cloning into `/tmp/<name>` instead, with a `clone` symlink inside the
candidate folder for convenience. Any other candidate `justfile` that clones nested under this
checkout and then builds `fuzz/` directly (not just a `repro/` crate, which has its own bare
`[workspace]` table per the template and so stops the climb immediately) should use the same
clone-outside-the-tree pattern - this was not retroactively checked against already-written
candidates' justfiles this session.

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
