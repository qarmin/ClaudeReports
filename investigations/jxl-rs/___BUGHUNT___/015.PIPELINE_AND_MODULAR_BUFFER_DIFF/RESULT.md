# Low-memory vs simple render pipeline, and 16-bit vs 32-bit modular buffers (differential fuzz)

The repo only compares these in unit tests on fixed files (`test_compare_pipelines_*`, exact equality). Mutated inputs were never compared. The test-only options and `SimpleRenderPipeline` were exposed in a scratch worktree with `--cfg bh_diff` (`harnesses/fuzz_pipe/expose_test_options.patch`).

Base commit 7f539c8; main tree not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| cargo-fuzz `pipediff`: default pipeline vs `use_simple_pipeline`, exact f32 equality of every frame/channel | 11 forks, ~1500 s, ~244k execs, seeded with the 3072-input decode corpus | 1 crash: `assertion failed: origin.0 <= output_size.0 as isize` at render/simple_pipeline/extend.rs:33 in the test-only simple pipeline, on an input the production pipeline decodes without error (input saved as `crash-b049f904...` in this folder). No output mismatch between the two pipelines on any input where both finished. |
| cargo-fuzz `modbufdiff`: default (16-bit buffers where possible) vs `disable_16bit_modular_buffers`, exact equality | 11 forks, ~1500 s, ~272k execs | 0 crashes / mismatches |

## Status

**Inconclusive**

**Fully reproducible**: n/a (no finding).

## Root cause

The one divergence is an assertion inside the test-only simple pipeline (frame origin entirely outside the canvas); it is not reachable through the public API. Whether the production pipeline renders such a frame correctly (nothing visible) was not checked, and no output difference was observed, so this is not written as Confirmed.

## What's not done / open questions

- Triage of the crash input (frame header contents, what the low-memory pipeline outputs for it) was not done because the session was stopped; it is the first thing to look at.
