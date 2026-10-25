# gstjpegparser.c: two suspected overflows, both already correctly guarded

Code review of `subprojects/gst-plugins-bad/gst-libs/gst/codecparsers/gstjpegparser.c` (used by
the in-scope, already-ASAN-built `jpegformat` plugin, and by the VA JPEG decoder). Two
classic-looking JPEG parser overflow shapes turned up while reading this file; both turned out to
already be correctly guarded, one at the macro level and one at the caller level.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `gst_jpeg_segment_parse_huffman_table()` (~line 417-459) and the `GstJpegHuffmanTable` struct (`gstjpegparser.h` line 254-257) | `value_count = sum(huf_bits[0..15])`, each byte attacker-controlled and unvalidated (0-255), so `value_count` can be up to 4080 - but `huf_values` is a fixed `guint8[256]` array. Looked like a clean, unguarded stack/struct buffer overflow of up to ~3824 bytes | Initially looked like a real, confirmable overflow |
| Code reading: the `READ_BYTES()` macro used for the copy (~line 78-89) | `#define READ_BYTES(reader, buf, length) { if (length > sizeof(buf)) { ...; goto failed; } ...; memcpy(buf, vals, length); }` - `buf` is substituted as the literal expression `huf_table->huf_values`, a genuine fixed-size-array lvalue (member access through a struct pointer), so `sizeof(buf)` correctly evaluates to 256 at the call site, not `sizeof(pointer)`. The macro already rejects any `value_count > 256` before the `memcpy`, returning `FALSE` via `goto failed` | **Rules out the bug**: the macro's own `sizeof()` check is the guard, and it's applied correctly here (no pointer decay, since it's an array member of a dereferenced struct pointer, not a parameter) |

One approach initially suggested a real bug; a second, closer look at the exact macro definition
(not just its call site) resolved it as a false alarm - worth recording precisely because the
shape is a classic, well-known overflow pattern that would be easy for a future session to
re-flag without checking the macro.

| Code reading: `gst_jpeg_parse()`'s top-level marker/length scan (~line 687-764) | `seg->size = length;` (the raw, attacker-controlled 2-byte segment length field) is set with **no check against how much data is actually left in the real buffer** - `gst_jpeg_parse()` itself trusts the declared length unconditionally. This looked like it could let a crafted length claim far more data than the buffer actually has, and any caller that trusted `segment->size` to bound a byte reader (as `gst_jpeg_segment_parse_frame_header()` etc. do, via `gst_byte_reader_init(&br, segment->data + segment->offset, segment->size)`) would then read out of bounds via the `_unchecked` read macros | Looked like a second real bug, one level up from the Huffman table |
| Code reading: both real callers of `gst_jpeg_parse()` in this codebase - `gstjpegparse.c` (`gst_jpeg_parse_handle_frame`, ~line 1166: `if (offset + seg.size > mapinfo.size) { ...; goto beach; }`) and the VA JPEG decoder `gstjpegdecoder.c` (`jpeg_segment_fits_input()`, ~line 374-383, an overflow-safe equivalent check: `if (seg->offset > size) return FALSE; return seg->size <= size - seg->offset;`) | Both callers explicitly validate `segment->offset + segment->size` against the real mapped buffer size **before** calling any of the segment-body parsers that trust `segment->size` - exactly matching `gst_jpeg_parse()`'s own documented contract ("the caller must make sure there is enough data for the whole segment available") | **Rules out the bug**: `gst_jpeg_parse()` deliberately leaves this validation to the caller, and both real callers in this codebase already do it correctly |

## Status

**Not reproducible** - no bug. The same `READ_BYTES()` macro (and its `sizeof()`-based guard) is
also used by `gst_jpeg_segment_parse_frame_header()`'s implicit array reads and elsewhere in this
file; the quantization-table parser (`gst_jpeg_segment_parse_quantization_table()`) uses an
explicit `gst_byte_reader_get_remaining() < GST_JPEG_MAX_QUANT_ELEMENTS * element_size` check
instead of `READ_BYTES` (it reads a fixed count either way, so it doesn't need the macro's guard),
also correct.

## Root cause

N/A (not a bug) - see "What was tried" above.

## What's not done / open questions

- Did not run this through the `mini_fuzz.py`/fuzzing pipeline (no `ci/fuzzing/*` target
  specifically drives `jpegparse`/`gstjpegparser.c`'s DHT-segment path - `gst-codec-utils`'s
  corpus only has raw H264/H265/AV1 samples, not JPEG) - a future session wanting empirical
  confirmation of this "safe" conclusion could write a tiny standalone harness calling
  `gst_jpeg_segment_parse_huffman_table()` directly with a crafted `GstJpegSegment` (public API in
  `gstjpegparser.h`), which would also make a good permanent addition to `ci/fuzzing/` given this
  file currently has no fuzz coverage in the tree at all.
- Only read the Huffman/frame/scan/quantization/restart-interval header parsers in this file, not
  `gst_jpeg_parse()` (the top-level marker-scanning entry point) or `gstjpegparse.c`/
  `gstjpegformat.c` (the actual GStreamer elements built on top of this parser) - those are natural
  next candidates from the same file/plugin if a future session wants to continue here.
