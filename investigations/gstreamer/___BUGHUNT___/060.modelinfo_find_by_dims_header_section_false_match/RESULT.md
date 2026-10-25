# modelinfo_find_by_dims_header_section_false_match

`gst-plugins-bad/gst-libs/gst/analytics/modelinfo.c` `modelinfo_find_tensor_name_by_dims()`
iterates every group in the `.modelinfo` `GKeyFile`, including the mandatory `[modelinfo]` format-
version header section, with no exclusion - unlike its sibling
`modelinfo_find_tensor_name_by_index()`, which candidate #005 (session 1) already found has a
related but distinct bug (double-incremented loop counter). This was left in `IDEAS.md` as a
follow-up: "currently masked by its own `type`-field check... not a principled fix - a malformed
file that stray-sets `type=`/`dir=` on the header section could resurrect the same confusion."

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end repro: write a crafted `.modelinfo` file to disk whose `[modelinfo]` header section stray-sets `type=float32` and `dims=224,224` (in addition to the mandatory `version=1.1`), then call the real public API `gst_analytics_modelinfo_load()` + `gst_analytics_modelinfo_find_tensor_name()` (requesting a tensor index that doesn't exist, with `data_type`/`dims` matching the header's stray values) | Scoped meson build (core+base+bad, `libgstanalytics-1.0`), plain debug | **Reproduces immediately, no sanitizer needed** (pure logic bug, not memory-unsafe): `gst_analytics_modelinfo_find_tensor_name()` returns the string `"modelinfo"` - the format-version header section's own group name - as if it were a genuine tensor name match. |

## Status

**Confirmed** - reproduces 3/3 runs (fully deterministic, no memory corruption or timing
dependency involved).

**Fully reproducible**: **Yes**. The repro drives the real, unmodified
`gst_analytics_modelinfo_load()`/`gst_analytics_modelinfo_find_tensor_name()` public API against a
real file on disk, exactly as a genuine inference-element caller would (see e.g. the `tflite`
element, the original consumer of this API) - no internal function is called directly.

## Class

logic

## Root cause

`modelinfo_find_tensor_name_by_dims()`'s search loop calls `modelinfo_validate_internal()` on
every group name returned by `g_key_file_get_groups()`, including `GST_MODELINFO_SECTION_NAME`
(`"modelinfo"`, the mandatory format-version header). `modelinfo_validate_internal()` checks
`type` (via `key_file_string_matches()`) and `dir` (via `modelinfo_check_direction()`, which
**defaults to `TRUE` when the group has no `"dir"` key at all** - true for the header) and `dims`.
Ordinarily the header's `type` check fails (the header has a `version` key, not `type`), which
*incidentally* excludes it - but this is not a principled exclusion: a modelinfo file whose header
section additionally happens to declare `type`/`dims` keys matching what a caller searches for
(whether crafted maliciously, or just a careless copy-paste by whoever authored the file) causes
the header itself to be returned as a "found" tensor name. A caller receiving `"modelinfo"` as a
tensor name back from this API and using it to look up per-tensor metadata (normalization ranges,
`caps`, etc.) elsewhere would get nonsensical results, since no such tensor actually exists in the
model.

## Introduced by

05782229eeba4b8e81cca71e5cc12ebe954e38db "tflite: Add TensorFlow Lite element" (2024-03-09) - same
introducing commit as candidate #005's sibling bug in `modelinfo_find_tensor_name_by_index()`; both
functions were added in this commit without a `GST_MODELINFO_SECTION_NAME` exclusion.

## The fix

Add `if (!g_strcmp0(groups[i], GST_MODELINFO_SECTION_NAME)) continue;` at the top of the search
loop, explicitly skipping the header section - the same fix shape candidate #005 already applied
to the sibling `_by_index()` function.

Verified: rebuilt with the fix and re-ran the exact same crafted-file repro - the API now correctly
returns `NULL` (no tensor found) instead of falsely matching the header section.

- Project test suite: `meson test -C build-analytics-repro 'gst-plugins-bad:libs_analyticsmeta'` -
  1/1 passed both before and after the fix.

## What's not done / open questions

- `modelinfo_find_tensor_name_by_index()` (candidate #005's target) has this exact same missing-
  exclusion issue as a *second*, independent bug alongside its already-documented double-increment
  bug - not re-verified or re-patched in this session since #005 already covers that function's
  write-up; a future session fixing #005's finding for real should apply this same
  `GST_MODELINFO_SECTION_NAME` skip there too, not just the double-increment fix, to be complete.
- Not checked whether any other `modelinfo.c` group-iteration loop (e.g.
  `modelinfo_check_mandatory_input_fields()`, which *does* already correctly exclude the header)
  has a similar gap - only this one function was in scope for this candidate.
