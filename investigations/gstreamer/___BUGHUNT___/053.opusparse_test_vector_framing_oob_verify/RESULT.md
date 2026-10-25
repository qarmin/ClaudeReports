# opusparse_test_vector_framing_oob_verify

`gst-plugins-bad/ext/opus/gstopusparse.c` `gst_opus_parse_handle_frame()`'s "test vector framing"
fallback computes `packet_size = GST_READ_UINT32_BE(data)` from untrusted input and calls
`opus_packet_parse(data + 8, packet_size, ...)`, where `packet_size` is checked against
`MAX_PAYLOAD_BYTES` (1500) and `size - 4` but the actual read starts 4 bytes later (`data + 8`,
not `data + 4`) - flagged by `findings.toml` (`OVF_20`) as a likely heap OOB read.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end repro via `GstHarness("opusparse")` + `gst_harness_push()`, small crafted buffer (5 bytes) designed to fail the primary `opus_packet_parse()` and enter the fallback with a small `packet_size`, run under `valgrind` | Scoped build (core+base+bad+devtools), plain debug | No crash, no valgrind finding. Debug log showed the primary `opus_packet_parse()` call actually *succeeded* on the crafted bytes (interpreted as a valid single-frame "code 0" Opus packet) instead of failing into the fallback branch - the initial byte pattern was wrong. |
| Same harness, refined input: verified via direct standalone calls to the real bundled `opus_packet_parse()` (linked against `subprojects/opus-1.6.1`) that a `code 0` TOC byte (which any `packet_size <= MAX_PAYLOAD_BYTES` forces, since the most-significant byte of the big-endian `packet_size` becomes the TOC byte, and `packet_size <= 1500` forces that byte to `0x00`) is *always* accepted by `opus_packet_parse_impl()` as a structurally valid single-frame packet **unless** the implied frame length (`len - 1`) exceeds 1275 bytes (libopus's own hard per-frame size cap, checked purely arithmetically at the end of the function, `if (last_size > 1275) return OPUS_INVALID_PACKET`) | Same build; a small standalone C program (`/tmp/find_opus_case.c`) brute-force searched `(size, data[0..5))` combinations for one where `opus_packet_parse()` fails while `packet_size <= 1500` - none found for `size <= 10` (7168 combinations tried), consistent with the "code 0 always succeeds unless total len > 1276" finding above | Confirms the primary parse can only be forced to fail (while keeping `packet_size` small enough to survive the fallback's own bounds checks) by using a buffer `>= 1277` bytes long - i.e. the fallback branch is *only* reachable with a large real buffer, not a small one |
| Constructed a 1277-byte real buffer (`g_malloc0`, no slack) - large enough that the primary `opus_packet_parse()` fails via the 1275-byte cap - with `packet_size` set to 1273 (the maximum value that still passes `packet_size <= size - 4`), pushed through the real `GstHarness("opusparse")` pipeline, run under `valgrind` | Same build/tooling | No crash, no valgrind finding. Read `opus_packet_parse_impl()`'s source directly (`subprojects/opus-1.6.1/src/opus.c`) to understand why: for a `code 0` TOC at `data+8` (forced again, by the same `packet_size <= 1500` -> MSB-is-zero argument, now applied to the *inner* call), the function only ever dereferences the single TOC byte itself: frame content is recorded as a bare pointer (`frames[0] = data; data += size[0];`) and never read. The `code 1`/`code 2` paths similarly only dereference 1-2 bytes immediately after the TOC (a short size-prefix), never bytes near the *end* of the claimed `len`. The `code 3` padding-byte loop reads one byte per iteration but is bounded by *both* the claimed `len` and hitting a non-`0xFF` byte, and the VBR per-frame size-prefix loop only advances the read pointer by the (short) prefix size per frame, not by each frame's declared content size - none of these walk far enough into the buffer for the discrepancy between `size-4` (the check) and `data+8` (the actual base) to ever land a real dereference outside the mapped memory, given the buffer must independently be `>= 1277` bytes for the outer call to fail in the first place. |

## Status

**Not reproducible.** The static-analysis finding correctly identifies a real internal
inconsistency (`size - 4` is checked but `data + 8` is used, and `MAX_PAYLOAD_BYTES` is a
looser bound than the real remaining buffer in some cases) - but no code path in
`opus_packet_parse_impl()` (the real, unmodified `libopus` 1.6.1 as vendored in this repo)
actually dereferences memory far enough into its claimed `len` to turn that discrepancy into an
observable out-of-bounds *read*, given the structural constraint (documented above) that reaching
the vulnerable fallback branch at all requires a real buffer of at least 1277 bytes, which is
always far larger than the few bytes any of `opus_packet_parse_impl()`'s code paths actually touch
near the start of the claimed range. `gst_opus_parse_handle_frame()` itself also never reads the
frame *content* directly - it only uses the returned frame-count/size bookkeeping to tell
`GstBaseParse` where the frame boundary is, and `GstBaseParse` manages its own real byte
accounting independently, so an over-claimed frame size just makes it wait for more input rather
than reading garbage.

## Root cause

N/A (not a reproducible bug) - see "What was tried" for the detailed mechanism that prevents the
apparent overflow from ever landing a real memory access. This does not rule out every conceivable
input; see "What's not done" below.

## What's not done / open questions

- Not exhaustively fuzzed - a short coverage-guided or mutation-based fuzz burst directly against
  `opus_packet_parse_impl()` (with a real, minimally-sized backing allocation and an oversized
  claimed `len`, checked under ASAN) would be a stronger negative result than this session's
  manual/targeted construction, and might surface a code path this analysis missed (in particular,
  the `code 3` self-delimited framing branch - `self_delimited=1` - was not tried, since
  `opus_packet_parse()`'s public wrapper always passes `self_delimited=0`, but it's worth
  double-checking that assumption holds for every call site).
- Only the specific call site in `gst_opus_parse_handle_frame()`'s fallback branch was checked;
  not checked whether any *other* GStreamer code (e.g. `oggdemux`, `matroskademux`'s Opus
  handling, or the decoder itself) constructs a similarly inconsistent `(pointer, claimed-length)`
  pair when calling into `opus_packet_parse()` or the lower-level `opus_decode()`.
