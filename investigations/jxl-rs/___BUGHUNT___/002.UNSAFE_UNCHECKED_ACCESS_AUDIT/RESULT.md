# Unchecked accesses in WeightedPredictorState, AnsHistogram::read and ChannelsView/ChannelsMutView

`get_unchecked` / `get_unchecked_mut` in `jxl/src/frame/modular/predict.rs`, `jxl/src/entropy_coding/ans.rs` and `jxl/src/render/channels.rs` (the last one is new code from the 'for_each_chunk' redesign), plus open issue #985 about the predictor ones. Looked suspicious because their safety comments rely on invariants established elsewhere.

Base commit 7f539c8 (jxl-rs main); the main tree was not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of each unsafe site against its stated invariant | predict.rs predict_and_property, ans.rs AnsHistogram::read, channels.rs view()/load/store/store_interleaved | Every index is bounded by an assert (pos.0 < xsize, mask `& (num_rows-1)`, stride check) or by construction; no hole found |
| Test suite under AddressSanitizer + debug-assertions + overflow-checks | nightly `-Zsanitizer=address -Cdebug-assertions -Coverflow-checks`, `cargo test --release -p jxl`, detect_leaks=0, allocator_may_return_null=1; 836 + 724 tests (every test except `image::test::huge_image`, which deliberately asks for a 2^56 byte allocation and is aborted by ASAN regardless of options) | 1560 passed, 0 failed, no ASAN report |
| Repo `decode` fuzz target | cargo-fuzz (debug-assertions on), 8 forks, 2439 s, ~720k execs, seeded from jxl/resources/test, max_len 100000 | 0 crashes/timeouts/OOMs |
| Repo `decode_progressive_parallel` fuzz target | 4 forks, 1204 s, ~35k execs, seeded from the decode corpus | 0 crashes |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

The invariants documented next to each `unsafe` block hold: `pred_errors_buffer`/`error` have `(xsize+1)*2` entries and every index is at most `2*xsize+1`; `err >> shift < 64` for any u32 `err`; `ChannelsView::view()` asserts `x_offset >= RADIUS`, `x_offset + RADIUS + MAX_SIMD_LANES <= stride`, `buf.len() == num_rows * stride` and masks rows with `num_rows - 1` (power of two asserted).

## What's not done / open questions

- Not covered: `jxl_simd` intrinsics themselves (~300 unsafe sites, only exercised through the test-suite and fuzzing above); `RawImageBuffer` was only read, not audited line by line. ASAN does not model `get_unchecked` that stays inside the same allocation but crosses a row boundary.
