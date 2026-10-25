# h266parser (VVC) PPS tile/slice parsing hardening cluster - verified correct

`gsth266parser.c` (H.266/VVC bitstream parser, part of the in-scope, ASAN-built `videoparsers`
plugin) has an unusually dense recent history of security-relevant fixes - VVC is new codec
support, apparently having just gone through its first real hardening pass (multiple
`Fixes .../work_items/...` commits reporting real, externally-found bugs). Given the density,
checked several of the fixes in `gst_h266_parser_parse_picture_partition()` (PPS tile/slice
layout parsing) for completeness, following the same "does the fix actually cover the array it's
protecting" methodology that found candidate #010's bug.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `3a02a89e05` "Avoid out-of-bounds write when parsing PPS tile slices" (2026-04-15, real reported bug) - adds `if (i + j >= pps->num_slices_in_pic_minus1) goto error;` before writing `pps->exp_slice_height_in_ctus_minus1[i][j]` inside the "tile contains multi slices" loop | Traced `num_slices_in_pic_minus1`'s own bound (`READ_UE_MAX(..., GST_H266_MAX_SLICES_PER_AU - 1)`, i.e. capped at 999) against the array's declared size (`exp_slice_height_in_ctus_minus1[GST_H266_MAX_SLICES_PER_AU][GST_H266_MAX_TILE_ROWS]`, first dimension 1000) - `i+j <= num_slices_in_pic_minus1 <= 999` is exactly the valid index range | Correct |
| Code reading: `8a2cce4f77` "Check bounds before writing to CTU entrypoint array" (2026-06-22, in `gst_h266_parser_parse_slice_hdr()`) - moves a `sh->num_entry_points >= GST_H266_MAX_ENTRY_POINTS` check from *after* the loop that fills `sh->entry_point_start_ctu[]` to *before* the write inside the loop | The pre-fix code wrote `entry_point_start_ctu[num_entry_points]` unconditionally inside the loop and only checked the bound once the whole loop had finished - a real, reachable stack/struct buffer overflow write for streams with enough CTU boundaries to push `num_entry_points` past the array's capacity before the too-late check could fire | Correct as fixed - check now precedes the write |
| Code reading: `d37923ee59` "Avoid out-of-bounds read if too many exp slices are signalled" (2026-06-22) - adds `if (pps->num_exp_slices_in_tile[i] > GST_H266_MAX_TILE_ROWS) goto error;` before the `for (j = 0; j < pps->num_exp_slices_in_tile[i]; j++)` loop that indexes `exp_slice_height_in_ctus_minus1[i][j]`'s *second* dimension | `num_exp_slices_in_tile[i]` is bounded only by `READ_UE_MAX(..., pps->tile_row_height_minus1[tile_y])` (a semantic/content bound, not necessarily <= the array's actual `GST_H266_MAX_TILE_ROWS` capacity) before this fix - confirmed the array's second dimension is exactly `GST_H266_MAX_TILE_ROWS`, and the fix's `>` (not `>=`) comparison is correct since `num_exp_slices_in_tile[i] == GST_H266_MAX_TILE_ROWS` still only produces valid indices `j` in `[0, GST_H266_MAX_TILE_ROWS - 1]` | Correct |
| Code reading: `dbf50dce31` "Check aspect ratio index against lookup table length" (2026-06-17, in `gst_h266_parse_vui_parameters()`) - adds `else if (vui->aspect_ratio_idc <= 16)` before indexing `aspect_ratios[vui->aspect_ratio_idc]` | Confirmed `aspect_ratios[17]` (indices 0-16) and `EXTENDED_SAR == 255` handled by a separate, earlier branch - the new `<= 16` guard plus the pre-existing `== EXTENDED_SAR` branch together correctly partition all 256 possible 8-bit `aspect_ratio_idc` values (0-16 valid lookup, 255 extended/custom, 17-254 now correctly rejected instead of read out of bounds) | Correct |

| Code reading: `8a3be2b513` "fix SEI parsing error handler" (2026-07-16) - adds `gst_h266_sei_clear(sei)` on the `error:` path of `gst_h266_parser_parse_sei_message()`, freeing `sei->payload.registered_user_data.data` if `sei->payloadType == GST_H266_SEI_REGISTERED_USER_DATA` | Initial concern: `gst_h266_parser_parse_sei()` (the caller, ~line 6251) declares `GstH266SEIMessage sei;` as an **uninitialized stack local** - if an error occurred before `payloadType`/`data` were ever assigned real values, this looked like it could read/free garbage stack memory as a pointer | Checked `gst_h266_parser_parse_sei_message()`'s own first executable line: `memset (sei, 0, sizeof (*sei));` - the callee itself zeroes the whole struct before any parsing begins, so `payloadType` is `0` and `data` is `NULL` on every path, including every early-`goto error`, regardless of the caller's own uninitialized declaration. **Rules out the bug** - `g_free(NULL)` is always safe |

| Code reading: `2d7cf30b37` "Heap-allocate parameter sets before parse to avoid stack overflow" (2026-07-01) - converts `gst_h266_parser_parse_nal()`'s stack-allocated `GstH266VPS`/`SPS`/`PPS`/`APS` locals (each large, given the many `GST_H266_MAX_SLICES_PER_AU`/`GST_H266_MAX_TILE_ROWS`-sized fixed arrays seen above) to `g_new`/`g_free` heap allocations | Confirmed every `switch` case frees its own allocation exactly once, on every path (the `break` after `g_free` is unconditional, `res` captures the real return value instead of the old code's `return` skipping past the (nonexistent, in the old code) cleanup) | Correct - no leak, no double-free, no behavior change beyond the allocation strategy |

| Code reading: `d85160e59b` "Use long bit skipping function for potentially large values" (2026-06-22, in `gst_h266_parse_general_constraints_info()`) - swaps `nal_reader_skip()` (which asserts/aborts if asked to skip more than 64 bits) for `nal_reader_skip_long()` (no such limit) when skipping `num_additional_bits - num_additional_bits_used` reserved bits, a value that can legitimately exceed 64 for a crafted stream | Confirmed `num_additional_bits` is read as an attacker-controlled count with no upper bound tying it to 64 before this subtraction is used as a skip length | Correct, minimal, matches its stated purpose - an assertion-abort DoS, now a graceful (if large) skip |

Seven fixes checked in depth, all from the same recently-hardened file; none showed a gap.

## Status

**Not reproducible** - no bug found across the four commits examined. This is a well-executed
hardening pass; the density of fixes reflects VVC/H.266 being new codec support recently getting
real external scrutiny (multiple fixes credit external reporters), not carelessness.

## Root cause

N/A (not a bug in any of the four commits examined) - see each fix's own description above for
the historical bug it already closed.

## What's not done / open questions

- All commits identified via git log for `gsth266parser.c` around this hardening period were
  checked (7 total) - this line of investigation is closed out.
- Only `gst_h266_parser_parse_picture_partition()` and `gst_h266_parse_vui_parameters()` were
  checked - the file is large (VVC is a complex, recent codec) and covers many more syntax
  structures (SEI messages, reference picture lists, weighted prediction tables, etc.) not
  touched by this candidate.
- No runtime fuzzing was attempted against `h266parse`/`gsth266parser.c` specifically -
  `ci/fuzzing/gst-codec-utils`'s own corpus is H264/H265/AV1 samples, not VVC/H266, so this file
  has effectively zero existing fuzz coverage from this session's tooling. A dedicated seed corpus
  of real `.266`/VVC-in-MP4 samples plus a short `mini_fuzz.py` burst would be a good, currently
  entirely unexplored follow-up (this session ran out of time to build one).
