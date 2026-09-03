# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

Project: `png` (image-rs/image-png), a pure-Rust PNG decoder/encoder. ~13k lines across `src/`.
Session baseline commit: `05ccdb9c8ae7fc50f4787ba2888a1c515adad944`.

## The single most important fact about this crate for bug-hunting

`src/lib.rs:62` declares `#![forbid(unsafe_code)]` - **there is not one `unsafe` block in the
entire crate** (verified 2026-09-02 by grep; the only match in `src/` is the `forbid` attribute
itself). That rules out the whole memory-safety/UB half of this skill's usual arsenal: `miri`,
ASAN, UBSAN and TSan have essentially nothing to find here, and time spent setting them up is
time wasted.

The bug classes that *are* reachable in this crate, in rough order of how much surface they have:

- **`crash`** - a panic reaching a caller from safe code: slice index out of range, slice range
  start > end, `unwrap()`/`expect()` on decoder state, arithmetic overflow (only in a
  debug-assertions build; silently wraps in release), capacity overflow / unbounded allocation
  from attacker-controlled dimensions, and unbounded recursion / stack overflow.
- **`logic`** - wrong-but-not-crashing output: a decoded image that differs from what the PNG
  spec mandates, an encoder that writes a stream it cannot itself read back, interlace (Adam7)
  row/pass math, bit-depth expansion, palette/tRNS/gAMA/sRGB handling, and APNG frame
  composition (`dispose_op`/`blend_op`).

Treat "does this panic on malformed input" and "does this decode to the wrong pixels" as the two
questions worth asking here. `security` in this crate means denial-of-service (panic or
unbounded allocation on a hostile file), not memory corruption.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `cargo test --lib` | yes | Baseline on the session commit: **91 passed, 0 failed, 1 ignored**, ~1.9 s. This is the before/after suite every candidate must run. |
| `cargo test` (all targets) | yes | Adds the `tests/` integration suite; slower. Use it when a candidate touches decode output, not just for a panic repro. |
| debug-assertions build | yes, primary tool | The default `cargo run`/`cargo test` profile. This is what catches integer overflow - a release build wraps silently, so **always reproduce arithmetic suspects in debug first**. |
| `cargo fuzz` | yes, 0.12.0 installed | Nightly toolchains available: `nightly`, `nightly-2026-08-04`, `nightly-2026-08-21`. See "Existing project harnesses" below - prefer reusing the in-repo targets over writing a standalone one. |
| `cargo clippy` | yes | Useful for discovery, but this crate is already clippy-clean at default levels; run with pedantic/nursery lints to get correctness-relevant signal. |
| `miri` | installed (`miri 0.1.0`, nightly-2026-08-21) but **not useful here** | No `unsafe` in the crate - see above. Do not spend a candidate's 3-approach budget on it unless the suspect somehow involves a dependency's `unsafe` (`miniz_oxide`'s `simd` feature, `crc32fast`, or `flate2`'s `zlib-rs` backend - all off the default path). |
| ASAN / UBSAN | buildable (nightly present) but **not useful here** | Same reason as miri. |

## Existing project harnesses worth reusing

The repo ships a cargo-fuzz workspace at `fuzz/` with three targets - reuse these rather than
building a standalone harness from scratch:

| target | path | what it does |
|---|---|---|
| `decode` | `fuzz/fuzz_targets/decode.rs` | Decodes arbitrary bytes with `Limits { bytes: 1 << 16 }`, skips images over 5 MB raw, loops `next_frame` to EOF. The general "does malformed input panic" target. |
| `roundtrip` | `fuzz/fuzz_targets/roundtrip.rs` | Encode-then-decode: catches encoder/decoder disagreement (a `logic` bug the `decode` target structurally cannot find). |
| `buf_independent` | `fuzz/fuzz_targets/buf_independent.rs` | Checks that decoding is independent of how the input is chunked across `Read` calls - i.e. the streaming state machine in `src/decoder/stream.rs` must not depend on buffer boundaries. |

Note the `decode` target's own limits (`1 << 16` bytes, the 5 MB `raw_bytes()` early-return): a
memory-exhaustion or huge-dimension bug can be structurally unreachable through that target while
still being reachable through the crate's real public API with default `Limits`. If a candidate is
about allocation size, write its own harness rather than assuming `decode`'s clean fuzz history
covers it.

There is also `png-afl/`, an AFL harness crate - not used by this workspace so far.

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
5. Debug build first for anything arithmetic - a release-only check hides every overflow this
   crate can have.
