# aarch64 NEON build vs x86_64 scalar

Only x86_64 SIMD levels were diffed in 006; the NEON backend has its own unsafe code. aarch64 binaries run under qemu-user here.

Base commit 7f539c8; main tree not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| aarch64-unknown-linux-gnu build with `--features neon` (linker aarch64-linux-gnu-gcc), run under `qemu-aarch64 -L /usr/aarch64-linux-gnu`, f32 outputs compared with the scalar x86_64 build | 109 test + conformance files; plus 70 of the 111 decodable fuzz-corpus inputs (qemu timeout cut the rest) | all compared files agree; status identical; worst relative diff 9.9e-4 (same value as the SSE4.2 build, i.e. the same approximation) |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

NEON output matches the scalar output within the tolerance seen for SSE4.2.

## What's not done / open questions

- wasm32 simd128 was not run (no wasm runtime with SIMD tested); armv7 NEON is not merged (PR 792).
