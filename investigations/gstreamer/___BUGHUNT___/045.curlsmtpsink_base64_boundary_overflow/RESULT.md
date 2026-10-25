# curlsmtpsink_base64_boundary_overflow

`subprojects/gst-plugins-bad/ext/curl/gstcurlsmtpsink.c`'s `add_final_boundary_unlocked()`
(~line 178-188) and `gst_curl_smtp_sink_flush_data_unlocked()` (~line 798-806) both call
`g_base64_encode_close()` into a 6-byte `g_malloc(6)` buffer, then unconditionally do
`data_out[len - 1] = '\r'; data_out[len] = '\n';`. `len` is `gsize` (unsigned) in the first
function and `gint` (signed) in the second. The candidate (findings.toml OVF_14) claimed
`g_base64_encode_close()` "legitimately returns 0 when there's no pending partial group" (an
attachment whose base64 byte count is a multiple of 3, or an immediate EOS with no data at all),
which would make `data_out[len - 1]` wrap to `data_out[SIZE_MAX]` (unsigned function) or read/write
`data_out[-1]` (signed function) - a heap buffer overflow/underflow.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the actual `g_base64_encode_close()` implementation (glib 2.88.0, the version installed and linked in this environment; cross-checked against upstream GNOME/glib `main` via `raw.githubusercontent.com`, and confirmed via GitLab commit history that the relevant code (`if (break_lines) *outptr++ = '\n';`, unconditional) has been unchanged since the function's introduction in 2006 commit 5cf8f1d4 and its OOB-write fix in ac059df7, i.e. predates gstcurlsmtpsink.c's own 2011 introduction) | static code reading | Both call sites in gstcurlsmtpsink.c hardcode `break_lines = TRUE`. With `break_lines == TRUE`, `g_base64_encode_close()` unconditionally appends a `'\n'` byte regardless of any pending partial group (`*save`), so it can never return 0 - only `break_lines == FALSE` (not used by this file) can return 0. |
| Direct empirical test against the real, currently-installed `libglib-2.0.so.0` (2.88.0): a small C program calling `g_base64_encode_step()`/`g_base64_encode_close(TRUE, ...)` with every input length from 0 to 62 bytes (all three residues mod 3: exact multiples of 3, and both 1-byte/2-byte remainders), including the "immediate EOS, zero bytes ever encoded" case (`state=0, save=0` from a freshly-allocated `Base64Chunk`, matching `gst_curl_smtp_sink_start()`'s own initialization at line 564-565) | gcc -O0, plain build, `harness_boundary_sweep.c` | `len` was **1** for every multiple-of-3 case (including the immediate-EOS case) and **5** for every 1/2-byte-remainder case, across the full 0-62 byte sweep - never 0, so `len - 1` is always in `[0, 4]`, never negative/wrapped |
| Same harness rebuilt under ASAN+UBSAN (`-fsanitize=address,undefined`), writing into a real `g_malloc(6)` buffer and performing the exact `data_out[len-1]='\r'; data_out[len]='\n';` pattern from both real call sites, for all 63 boundary inputs above | gcc -fsanitize=address,undefined -O0, `harness_boundary_sweep.c` | Zero ASAN/UBSAN diagnostics; every write landed at index `len-1` in `{0,4}` and `len` in `{1,5}`, both within the 6-byte allocation |
| Differential check: encoding 45 bytes in one `g_base64_encode_step()` call vs. in repeated 7-byte chunks (mirroring how `transfer_chunk()` actually feeds data incrementally, `block_size` at a time) against the same running `state`/`save`, then closing both | gcc -fsanitize=address,undefined -O0, `harness_chunked_vs_single.c` | Identical `close()` result (`len=1`) either way - confirms the streaming `state`/`save` accumulation is chunk-boundary-independent, so real GStreamer buffer-size-driven chunking doesn't change the conclusion above |

## Status

**Not reproducible.** The premise of the finding - that `g_base64_encode_close()` can return 0 in
this code - does not hold for the actual glib implementation given the arguments this file always
passes (`break_lines = TRUE`). Covered: every possible total base64-input-byte-count residue class
mod 3 (0, 1, 2) from 0 through 62 bytes, including the two scenarios the finding specifically named
("multiple of 3" and "immediate EOS with no attachment data"); both the unsigned-`len` call site's
and (via the same underlying `g_base64_encode_close()` behavior) the signed-`len` call site's
boundary condition; and confirmed the result is independent of how the input was chunked into
successive `g_base64_encode_step()` calls (relevant since the real code feeds data in
`block_size`-sized pieces, not as one call). No sanitizer diagnostic, no crash, no out-of-bounds
index in any of the 63+ boundary cases tried.

## Class

N/A (not reproducible).

## Root cause

Not a bug: `g_base64_encode_close(break_lines, out, state, save)` (glib source,
`glib/gbase64.c`) ends with `if (break_lines) *outptr++ = '\n';`, executed **unconditionally**
whenever `break_lines` is true, regardless of whether the preceding `switch (((char*)save)[0])`
had a pending 1- or 2-byte group to flush. Both call sites in `gstcurlsmtpsink.c` pass
`TRUE` as the first argument (`g_base64_encode_close (TRUE, data_out, &state, &save)` at both
line 181 and line 799), so the returned `len` is always at least 1 (just the newline) and at most
5 (up to 4 padding-group bytes + the newline) - never 0. The subsequent
`data_out[len - 1] = '\r'; data_out[len] = '\n';` therefore always indexes within `[0, 5]` of the
6-byte `g_malloc(6)` buffer, regardless of the `gsize` (unsigned) vs. `gint` (signed) type of
`len` in the two functions - the type distinction the finding flagged as consequential never
actually matters here because `len` never reaches the boundary value (0) that would expose it.

## Introduced by

N/A - not a bug in the current codebase; no fix to attribute. (For context only: the file itself,
including this exact `g_base64_encode_close(TRUE, ...)` call pattern, was introduced by
3f20bbdcf4 "curl: new curlsmtpsink element" and has not changed in the surrounding logic since.)

## The fix

None needed - no bounded fix was made, and the main project's working tree was never modified for
this candidate (all testing was done in a standalone harness against the system's real `libglib`,
built and run outside the main tree, and against a separate git worktree used only for reading
source/history, never for building/patching gstcurlsmtpsink.c itself).

- Project test suite: N/A, no code changed in the main project.

## What's not done / open questions

- This result depends on `break_lines == TRUE` always being passed by gstcurlsmtpsink.c to
  `g_base64_encode_close()`; both call sites were checked (grep confirmed exactly two call sites
  in the file, both hardcoded `TRUE`) and neither is reachable with `FALSE`, so this isn't a
  residual gap, just recorded for completeness.
- Not independently verified against every glib version GStreamer might be built against
  historically (only glib 2.88.0, the version installed in this environment, was tested directly;
  however, reading upstream glib's own commit history for `glib/gbase64.c` shows the specific
  `if (break_lines) *outptr++ = '\n';` unconditional-append behavior has been present since the
  function's original 2006 introduction, predating gstcurlsmtpsink.c's 2011 introduction by 5
  years, so this is not expected to be version-sensitive).
- The two functions were not exercised through a full, real GStreamer pipeline (curlsmtpsink
  element instantiated, real buffers pushed, real EOS) - not needed here since the negative result
  is a structural proof (glib's own source + an exhaustive boundary sweep of every reachable
  `state`/`save` combination the real code can produce), not a claim that depends on one specific
  unobserved run of the full pipeline.
- No other `g_base64_encode_close()` call sites with similar surrounding index math were found
  elsewhere in the tree during this candidate (only this file uses this exact 6-byte-buffer +
  manual `\r\n` workaround pattern); not exhaustively swept as a separate candidate.
