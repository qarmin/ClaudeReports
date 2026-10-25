# 32-bit (wasm32) usize overflow in size computations

The crate targets wasm32 where usize is 32 bits; arithmetic like width*height*channels that is fine on x86_64 can overflow there. i686-unknown-linux-gnu could not be linked (no 32-bit libgcc), so wasm32-wasip1 + node's WASI was used.

Base commit 7f539c8 (jxl-rs main); the main tree was not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Replay of 3189 inputs from the `decode` fuzz corpus on wasm32-wasip1 with debug-assertions + overflow-checks, compared with the native run | `harnesses/replay32` + node 22; one input per node process | no panic and no status difference for the 3176 inputs that ran; 13 inputs make node itself segfault only when stdout is redirected (reproducible with --stack-size=900 even on a passing input - a node/WASI artefact, not investigated further) |
| Hand-crafted headers with huge dimensions (up to 2^30 x 1, 2^19 x 512) spliced into a valid file | native build, sample limit off | decoder stops at the next header with `truncated`, peak RSS 11 MB - no allocation based on the declared size |

## Status

**Inconclusive**

**Fully reproducible**: n/a (no finding).

## Root cause

No 32-bit specific defect was seen but the oracle is weak: the corpus is mostly invalid files that fail early, and the crafted files never reach frame decoding because their frame header/TOC no longer matches the spliced image size.

## What's not done / open questions

- A real 32-bit run of the large test images (e.g. under a working i686 toolchain) is still open.
