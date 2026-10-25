# 32-bit (i686) overflow check of the test suite and the fuzz corpus

Re-check of 007 (this supersedes the Inconclusive 007 for the 32-bit question) with a working 32-bit target: `cargo zigbuild --target i686-unknown-linux-gnu` links (zig cc) and the binaries run natively on this kernel, built with debug-assertions + overflow-checks.

Base commit 7f539c8; main tree not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo test --release -p jxl` as i686 test binary | `--test-threads=8` first: 1512 passed, 19 failed, all `ImageOutOfMemory` errors from tests holding several 100+ MB images in a 4 GB address space; the 19 re-run with `--test-threads=1` | 18 pass; `test_compare_modular_buffers_tirr_photo` still returns `ImageOutOfMemory(71616, 3979)` (image alloc of ~285 MB fails gracefully with an Err, no panic/abort) |
| Replay of 3072 decode-fuzz corpus inputs (111 decodable), i686 vs x86_64 scalar build, f32 output compared | `harnesses/simdvar`, FuzzConfig default (sample limit 4M) | identical ok/err status for all inputs, outputs bit-identical (worst diff 0) |
| Same on 109 test + conformance files | as above | bit-identical |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

No 32-bit specific overflow or divergence. The only failures are memory exhaustion reported as `ImageOutOfMemory`, which is the intended graceful path.

## What's not done / open questions

- Large-dimension headers that reach frame decoding were not generated (see IDEAS.md).
