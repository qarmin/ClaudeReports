# hlsdemux_util_handle_pat_pmt_oob_read

`gst-plugins-bad/ext/hls/gsthlsdemux-util.c` `handle_pat()`/`handle_pmt()` skip the MPEG-TS
`adaptation_field` (`data += 1 + p[4]`) and `pointer_field` (`data += 1 + data[0]`) using
attacker-controlled length bytes with no bounds check against the current 188-byte TS packet, and
the one nominal downstream bounds check (`slen > (gsize)(p + r->packet_size - (data+1+2))`)
computes a `ptrdiff_t` that can already be negative by the time it's taken, wrapping to a huge
value when cast to `(gsize)`, so it never rejects anything. Flagged by `findings.toml` (`OVF_17`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Standalone harness calling the real, non-static public entry point `gst_hlsdemux_tsreader_find_pcrs()` (which internally calls the static `handle_pat()`) on a single crafted 188-byte TS packet (PID 0, `has_adaptation_field` set, `adaptation_field_len=255`), under `valgrind` | Scoped meson build (core+base+bad/hls), plain debug | No crash/finding. `GST_DEBUG` tracing showed `handle_pat()` was never even called - `find_offset()`/`have_ts_sync()` require `CLAMP(size/188, 25, 100)` = 25 **consecutive** valid-looking TS packet headers before trusting any packet offset at all; a single 188-byte packet never satisfies this. |
| Same entry point, input widened to 25 packets (4700 bytes: 24 harmless filler packets with a null PID, followed by the crafted malicious PAT packet as the 25th/last), under `valgrind` | Same build | The vulnerable code path is reached (confirmed via temporary `g_printerr` tracing: `handle_pat()` computes `data` at offset 260 within its own 188-byte packet, i.e. 72 bytes past the packet/buffer end) - but `valgrind` still reported 0 errors for the resulting read, since that address landed inside other still-allocated heap memory rather than a detectable gap. |
| Same 25-packet input, rebuilt with a scoped `clang -fsanitize=address,undefined` build of just this one source file (`-fvisibility=hidden` removed so the real, unmodified `gst_hlsdemux_tsreader_find_pcrs()`/`handle_pat()` are externally linkable, compiled together with the harness) | Same plugin scope, ASAN+UBSAN | **Reproduces**: `AddressSanitizer: heap-buffer-overflow ... READ of size 1 ... in handle_pat gsthlsdemux-util.c:158`, "located 65 bytes after 4843-byte region" (4843 = 4700-byte crafted buffer + the default allocator's `GstMemorySystem` header), called via the real `gst_hlsdemux_tsreader_find_pcrs_mpegts()` -> `gst_hlsdemux_tsreader_find_pcrs()` chain. |

## Status

**Confirmed** - reproduces 3/3 runs, identical ASAN report each time.

**Fully reproducible**: **Yes**. The repro calls the real, unmodified, externally-linkable
`gst_hlsdemux_tsreader_find_pcrs()` entry point - the only non-standard part of the build is
compiling `gsthlsdemux-util.c` directly alongside the harness (rather than linking the prebuilt
plugin `.so`) because `handle_pat()`/`handle_pmt()` are `static` and the real plugin build applies
`-fvisibility=hidden`; this is the same "compile the real source file together with a small
harness" pattern already used for `gsth266parser.c` earlier in this workspace, not a
reimplementation of the vulnerable logic.

## Class

memory-safety (heap-buffer-overflow read)

## Root cause

`handle_pat()`/`handle_pmt()` skip past the (attacker-controlled) `adaptation_field` and
`pointer_field` sections of an MPEG-TS packet using raw pointer arithmetic with **no check** that
the resulting `data` pointer stays within the current 188-byte packet (`p` to `p + r->packet_size`)
before dereferencing it. For a packet claiming `has_adaptation_field` with
`adaptation_field_len = 255` (the field's full 8-bit range), `data` ends up 72 bytes past the end
of the packet after just the first skip, and every subsequent read (`data[0]` for the
`pointer_field`, then `data[0]`/`data[6]`/`data[7]`/`GST_READ_UINT16_BE(data+1)` for
`table_id`/`section_length`) reads further out-of-bounds heap memory. The sibling function
`handle_pcr()` in the same file explicitly validates `af_len < (1+6) || af_len > r->packet_size -
(4+1)` before using it - `handle_pat()`/`handle_pmt()` have no equivalent check. This is reachable
via the real, unmodified `gst_hlsdemux_tsreader_find_pcrs()` public entry point, which `hlsdemux`
calls while probing HLS media segments claiming to be `video/mpegts` - i.e. a malicious/compromised
HLS server (or MITM on a non-HTTPS playlist) can trigger this via ordinary segment content, no
authentication or key material needed.

## Introduced by

8cf50dbc441688f6b823b9ccc5ec57869e6344ef "hlsdemux: Do some reading and dumping of PCRs"
(2016-02-24) - this is when `handle_pat()`/`handle_pmt()` were first added in their current
unbounded-skip form.

## The fix

Add explicit bounds checks (against `end = p + r->packet_size`) before each pointer skip in both
`handle_pat()` and `handle_pmt()`: reject the packet (`return FALSE`) if the `adaptation_field`
skip or the `pointer_field` skip would advance `data` past `end`, and require at least 8 bytes
remaining after the `pointer_field` skip before reading `table_id`/`section_length`/CRC fields.
This also fixes the downstream `slen > (gsize)(end - (data+1+2))` check's wraparound, since `data`
is now guaranteed to never exceed `end` by the time that subtraction runs.

Verified: rebuilt with the fix and re-ran the exact same 25-packet repro (same malicious last
packet) - AddressSanitizer now reports 0 errors, and `gst_hlsdemux_tsreader_find_pcrs()` correctly
returns `FALSE` (rejecting the malformed PAT) instead of reading out of bounds.

- Project test suite: no dedicated meson test target covers `gsthlsdemux-util.c` in this scoped
  build (`meson test --list` returned no `hls`-related entries) - N/A, no tests in this package.

## What's not done / open questions

- Only `handle_pat()` was driven all the way to a captured ASAN crash in this session;
  `handle_pmt()` has the exact same unbounded-skip pattern (same fix applied to both in
  `patch.diff`) but wasn't independently exercised with its own dedicated crafted packet - the
  mechanism is identical enough that this wasn't considered necessary, but a future session could
  add a PMT-specific repro for completeness.
- The filler packets used to satisfy `find_offset()`'s 25-consecutive-sync-point requirement use a
  null PID (`0x1FFF`) with no adaptation field - not checked whether a real HLS server's actual
  TS stream content (with real audio/video PIDs) would need any different framing to reach the
  same vulnerable dispatch in practice; the crafted buffer here demonstrates the bug is reachable
  in principle through the real entry point, which is what CONFIRMATION.md requires, but isn't a
  byte-for-byte real-world capture.
