# tflite _guess_tensor_data_type() dims-to-format guessing - no issue found

IDEAS.md lead: b137c03b22 "tflite: fix gray format" (a one-line string-typo fix, `"GRAY"` ->
`"GRAY8"`, already merged) flagged as worth checking the surrounding dimension-guessing logic for
a similar class of issue.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `_guess_tensor_data_type()`, `subprojects/gst-plugins-bad/ext/tflite/gsttfliteinference.c` (~line 406-475) | Pure format-string-selection logic based on tensor dimension counts/values (2D/3D/4D cases, channel-count heuristics) - no array indexing beyond the already-bounds-checked `dims[0..3]` (guarded by the `dims_count < 2 || dims_count > 4` check at the top), no buffer sizing derived from these values in this function. The 4D case's `*channels`/`*planar` -> `*gst_format` mapping at the end correctly covers channels==1 and channels==3 (the only two values the preceding `if`/`else if` can produce), with `g_assert_not_reached()` for anything else | No issue found |

Single approach - this was due diligence on an already-fixed one-line typo commit, not a deep
investigation; the code is small and its logic is easy to verify to be free of the same or a
similar bug on a single read-through.

## Status

**Not reproducible** - no bug found.

## Root cause

N/A (not a bug).

## What's not done / open questions

- Only this one function was checked; the broader `tfliteinference`/`onnxinference` elements were
  not built in this session's ASAN scope (they need the TensorFlow Lite/ONNX runtime libraries,
  not available/installed here) and so couldn't be exercised at runtime regardless.
