# modelinfo_find_tensor_name_by_index() double-increments its counter, breaking index-based tensor lookup

`gst_analytics_modelinfo_find_tensor_name()` (the public API tflite/onnx inference elements use
to figure out which `.modelinfo` file section describes which model input/output tensor) silently
returns the WRONG tensor's name whenever a model has two or more tensors of the same direction
that can't be told apart some other way - specifically, asking for the tensor at index N (N>=1)
can return the tensor at a *different* index instead of the requested one, or fail to find it at
all. Found via code reading while checking `gst-libs/gst/analytics/modelinfo.c` (new-ish,
custom parsing code for ML model metadata, flagged as an under-fuzzed area relative to the
long-established demuxers/parsers oss-fuzz already covers heavily).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `modelinfo_find_tensor_name_by_index()` (~line 512-537) | `for (i = 0, j = 0; groups[i]; i++) { ...; if (index == j++) { ...; break; } j++; }` - the post-increment inside the `if` condition already advances `j` once; falling through (no match) then hits the *unconditional* `j++` two lines later, incrementing `j` a SECOND time per non-matching, direction-qualifying group. `j` advances by 2 for every miss instead of 1, so `index` (which advances by 1 per real tensor) desyncs from `j` after the very first miss - index 0 still works (nothing has desynced yet), every index >= 1 is looked up against the wrong (already-doubled) counter value | Confirmed a real off-by-double-increment bug purely by inspection |
| Code reading: same function, `g_key_file_get_groups()` includes the mandatory `[modelinfo]` header section | `modelinfo_check_direction()` returns `TRUE` (treated as "matches any requested direction") for a group that simply has no `dir` key - which is exactly the header section's situation, since it only has `version`/`group-id` keys. Every real modelinfo file has this header (mandatory per `modelinfo_check_version()`), so the header section itself is incorrectly counted as index 0 of *every* direction, shifting every real tensor's effective index by one. `modelinfo_check_mandatory_input_fields()` elsewhere in the same file explicitly skips `GST_MODELINFO_SECTION_NAME` for exactly this reason - this function doesn't | Confirmed a second, independent, compounding bug in the same function |
| Built a real, standalone reproduction using the actual public API (`gst_analytics_modelinfo_load()` + `gst_analytics_modelinfo_find_tensor_name()`, linked against the real `libgstanalytics-1.0.so` from an ASAN-instrumented scoped build - see `repro/repro.c`) with a 2-output-tensor `.modelinfo` file (`out0`, `out1`, identical `dims`/`type` so index is the *only* way to disambiguate them, exactly the scenario the `index` parameter exists for) | `ci/fuzzing/*` targets don't touch this code path (tflite/onnx aren't in this session's ASAN build scope, and modelinfo.c isn't a `ci/fuzzing` target anyway) - built directly via `pkg-config --cflags/--libs gstreamer-analytics-1.0` against `build-asan-fuzz` | **Reproduced**: `find_tensor_name(dir=OUTPUT, index=1, ...)` returns `"out0"` instead of `"out1"` - see `output.txt` |

Two approaches (both code reading) plus a direct, successful runtime reproduction - well within the
soft cap of 3, no need to escalate further (fuzzing/ASAN weren't needed since this is a pure logic
bug, not a memory-safety one, and the code reading already pinpointed both root causes precisely).

## Status

**Confirmed.** Deterministic, 5/5 runs (`repro/repro.c` run standalone 5 times before the fix,
5 times after - see `output.txt` for one before/after pair; the ratio is 5/5 not because flakiness
was suspected but because `CONFIRMATION.md`'s bar was applied as a matter of course for a
first-time finding).

**Fully reproducible**: **Yes.** The repro calls the exact same two public API functions
(`gst_analytics_modelinfo_load()`, `gst_analytics_modelinfo_find_tensor_name()`) that
`gsttfliteinference.c`/`gstonnxinference.c` call internally - this is the real, only way any
caller reaches this code, not a fabricated stand-in for it. No second participant is involved
(this isn't a network/IPC-facing finding), so section 11 of `CONFIRMATION.md` doesn't apply.

## Class

`logic`

## Root cause

Two independent bugs in `modelinfo_find_tensor_name_by_index()`, both needed fixing to restore
correct behavior:

1. `if (index == j++) { ...; break; } ... j++;` - `j` advances twice per non-matching iteration
   (once via the post-increment in the comparison, once via the explicit `j++` two lines later)
   instead of once, desyncing the running counter from the real tensor count after the first miss.
2. The loop iterates `g_key_file_get_groups()`'s result directly with no exclusion for the
   mandatory `[modelinfo]` header section, which `modelinfo_check_direction()` treats as "matches
   any direction" (a group missing the `dir` key is accepted for any requested direction) - so the
   header consumes index slot 0 of every direction in every real modelinfo file.

Either bug alone already breaks index-based lookup for `index >= 1`; fixing only one still leaves
the other in place (confirmed empirically while iterating on the fix - fixing bug 1 alone still
returned the wrong tensor, because bug 2 was independently shifting the index by one).

## Introduced by

05782229ee "tflite: Add TensorFlow Lite element" (2024-03-09) - the file was originally
`subprojects/gst-plugins-bad/ext/tflite/modelinfo.c` in this commit (already containing both bugs
in their current form), later moved verbatim to `gst-libs/gst/analytics/modelinfo.c` by
e7214664fe "analytics: move modelinfo to analytics lib" (2025-12-03) when the code was shared with
the new onnx inference element.

## The fix

Two changes to `modelinfo_find_tensor_name_by_index()`, `subprojects/gst-plugins-bad/gst-libs/gst/analytics/modelinfo.c`:

1. Changed `if (index == j++)` to `if (index == j)`, keeping only the single unconditional `j++`
   already present a few lines below (in the non-matching path) as the sole increment.
2. Added `if (!g_strcmp0 (groups[i], GST_MODELINFO_SECTION_NAME)) continue;` at the top of the
   loop body, mirroring the same guard already used in `modelinfo_check_mandatory_input_fields()`
   in the same file.

Verified against the standalone repro (`output.txt`): index 0 correctly resolves to `out0` both
before and after (it was never affected - the very first qualifying entry can't have desynced
yet); index 1 incorrectly resolves to `out0` before the fix and correctly resolves to `out1` after.

- Project test suite: `libs_analyticsmeta` (the only check-test binary this session's scoped ASAN
  build produces that touches the `gst-plugins-bad` analytics library) - 52 failures before AND
  after the patch, identical count, all pre-existing `GST_STATE_IGNORE_ELEMENTS` environment-setup
  failures unrelated to `modelinfo.c` (that test file doesn't exercise `modelinfo.c` at all - it
  tests `GstAnalyticsMtd` metadata types). No dedicated `modelinfo.c` unit test exists in this
  session's build scope: real tflite/onnx element tests exist
  (`subprojects/gst-plugins-bad/tests/check/elements/{tfliteinference,onnxinference}_data/`) but
  require the tflite/onnx plugins themselves, which aren't in this session's scoped ASAN
  configuration (see `../README.md`) and weren't rebuilt for this candidate given the session's
  time budget.

## What's not done / open questions

- `modelinfo_find_tensor_name_by_dims()` (the fallback path, a few lines below the fixed function)
  has the *same* missing `[modelinfo]`-section-exclusion gap as bug 2 above, but is currently
  masked by its own `type`-field check (the header section has no `type` key, so it never
  spuriously matches there) - it isn't a live bug today, but a modelinfo file that stray-sets a
  `type=`/`dir=` key on the `[modelinfo]` header section itself (malformed, but not rejected by
  `modelinfo_check_version()`) could resurrect the same class of confusion there too. Not fixed
  this session since it isn't demonstrably reachable with a well-formed file - worth a defensive
  fix (add the same `GST_MODELINFO_SECTION_NAME` guard) the next time this file is touched.
- Did not rebuild/test the real `tfliteinference`/`onnxinference` elements end-to-end (would need
  their runtime dependencies, out of this session's scoped ASAN build and time budget) - the
  public-API-level repro is a faithful stand-in since both elements call
  `gst_analytics_modelinfo_find_tensor_name()` directly with no additional indirection, but a
  maintainer may want to confirm with a real multi-output `.tflite`/`.onnx` model + modelinfo pair
  regardless.
- Only checked this one function in `modelinfo.c` for this exact bug class (double-increment /
  missing header exclusion) - did not audit the rest of the file (`get_target_ranges`,
  `get_input_scales_offsets`, `get_dims_order`, etc.) beyond the reading already documented in
  candidate discovery; those were skimmed and no similar issue was found, but not to the same
  depth as this specific function.
