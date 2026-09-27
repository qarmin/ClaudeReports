# APNG decode's per-frame reset cost (ZlibStream::reset / UnfilteringBuffer::start_frame) - measurement only, no patch here

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

The lead (from `IDEAS.md`): an 8-frame, 512x512 RGBA APNG encodes in 8.77 ms but decodes in
17.22 ms - roughly 2x - and `benches/decoder.rs` never exercises multi-frame APNG decoding, so this
path has never been profiled in this workspace. Candidate 007 already fully characterized one
per-frame cost (`fdeflate::Decompressor::new()`'s 2 allocations / ~175 ns per call, invoked from
`ZlibStream::reset`). This candidate asks about everything *else* that runs once per frame:
`ZlibStream::reset` (`src/decoder/zlib.rs:81-84`, `self.started = false; *self.state =
Decompressor::new();`) and `UnfilteringBuffer::start_frame` (`src/decoder/unfiltering_buffer.rs:120-126`,
five field writes plus `self.data_stream.clear()`), called once per `fcTL` from `src/decoder/mod.rs:387`.

## What was tried

Reused `___OPTIMIZATIONS___/harness/encoder_bench`'s existing `decode-apng` mode (built for exactly
this: it was the harness used by candidate 007 to establish the allocation numbers above), no
harness changes needed. Image: synthetic 512x512 RGBA `photo` pattern, 8 frames, encoded once with
`Filter::Adaptive` + `Level(6)` then decoded in a loop via the public `Reader::next_frame` API.

1. Two independent `taskset -c 0` baseline runs (n=200, n=500) to confirm the number is stable
   before trusting it for anything.
2. One unpinned (multi-core) run to rule out a single-core-only effect.
3. `perf record -F 4000 -g` (`perf_event_paranoid=-1`, no privilege issue) over 2000 decode
   iterations (138,594 samples, ~34 s of wall time, so 8 x 2000 = 16,000 individual
   `ZlibStream::reset` + `start_frame` calls are inside the sampled window) to see where decode
   time actually goes and whether the reset paths show up as a measurable line item at all.

| Variant | single-core p50 | multi-core p50 | notes |
|---|---|---|---|
| decode-apng, n=200 | 17.296 ms | - | allocs=38 reallocs=57 alloc=1.41 MiB peak_rss=10.8 MiB |
| decode-apng, n=500 (repeat) | 17.679 ms | - | second independent run, confirms stability |
| decode-apng, n=500, unpinned | - | 17.299 ms | matches the pinned numbers, no multi-core-specific effect |
| apng (encode, for reference) | 8.815 ms | - | same image/frames/filter/compression, from the same harness |

`perf report` (flat, `-g none`, full output saved as `perf_report_top40.txt` /
`perf_report_reset_related.txt` in this folder) over the 138,594-sample profile:

| Symbol | Self % |
|---|---|
| `<fdeflate::decompress::Decompressor>::read` | **90.20%** |
| `png::filter::unfilter` | 3.63% |
| `libc __memmove_avx512_unaligned_erms` | 0.93% |
| `fdeflate::huffman::build_table` | 2.27% |
| `crc32fast::specialized::pclmulqdq::calculate_avx512` | 0.69% |
| `<png::decoder::unfiltering_buffer::UnfilteringBuffer>::unfilter_curr_row_in_place` | 0.11% |
| `<png::decoder::zlib::ZlibStream>::decompress` | 0.04% |
| `<png::decoder::unfiltering_buffer::UnfilteringBuffer>::prev_row` | 0.03% |
| `<png::decoder::unfiltering_buffer::UnfilteringBuffer>::shift_buffer_left` | 0.01% |
| `<png::decoder::zlib::ZlibStream>::reset` | **0.00%** (no samples landed here at all) |
| `start_frame` (inlined) | **0.00%** (no samples landed here at all) |
| `reset_prev_row` (inlined) | **0.00%** (no samples landed here at all) |

## Result

**Time**, over 16,000 real reset/start_frame calls sampled at 4 kHz across a real 8-frame APNG
decode workload: both `ZlibStream::reset` and `UnfilteringBuffer::start_frame` show **0.00% self
time** - not "small", literally zero samples landed in either function or its inlined call sites,
in a profile with 138,594 total samples. This is consistent with candidate 007's own estimate
(~175 ns/reset x 8 frames = ~1.4 us out of a ~17.4 ms 8-frame decode, i.e. ~0.008% of total time) -
below the sampling resolution of a 4 kHz profile by roughly two orders of magnitude, so "not
observed" here is the expected outcome given that number, not a surprise.

The wall-clock gap this candidate set out to explain (8.77 ms encode vs 17.22 ms decode, ~2x) is
**not** explained by per-frame reset overhead at all: 90.20% of decode's own time is inside
`fdeflate::Decompressor::read` (the actual inflate work, run once per IDAT chunk, completely
independent of how many frames the stream is split into) and a further 3.63%/2.27%/0.69% is real
per-byte unfiltering/Huffman-table-build/CRC work. None of that is reset-shaped; it is the
irreducible cost of decompressing and unfiltering the pixel bytes themselves. Encode being faster
than decode here is not evidence of a "frame reset tax" - it reflects that this image's zlib
decompression (fixed cost per byte, always paid via `fdeflate`, however the encoder produced the
stream) happens to take longer than `Level(6)` compression did for this particular corpus file/
pattern, which is a fact about this one image's compressibility, not about the frame-reset code
paths this candidate targeted.

## Does it change behavior/output?

- N/A - measurement/characterization only. No file in `png`'s source was changed, so there is
  correctly no `patch.diff` in this folder (see workspace `README.md`'s ground rules and
  `SKILL.md`'s "patch.diff is mandatory whenever the fix is a change to the tested project" - this
  is the other, equally valid state: a pure investigation with nothing to patch).
- Project test suite: not run - no source file was changed, nothing to regress.

## Why the result looks the way it does

Both suspect functions are trivial by construction: `ZlibStream::reset` is one bool write plus the
`Decompressor::new()` call already fully characterized by candidate 007 (2 allocations, ~175 ns,
not re-measured here), and `UnfilteringBuffer::start_frame` is five `usize`/enum field writes plus
`Vec::clear()` on a `Vec<u8>` - `clear` on a byte vector has no per-element drop glue to run, so it
compiles down to a single length-field store, not a deallocation or a memset. Neither function
allocates, loops over the frame's pixel data, or touches memory outside a handful of struct fields.
There was never a plausible mechanism for either to cost more than a few nanoseconds per call, and
the profile confirms that directly rather than by extrapolation alone.

## What's not done / open questions

- The original 2x encode-vs-decode gap that motivated this candidate is real (confirmed here at
  8.815 ms vs 17.296-17.679 ms on the same image/frame count/harness) but is fully attributable to
  `fdeflate::Decompressor::read`'s own inflate cost (90.20% self time) plus real per-byte unfilter/
  Huffman-table/CRC work (~6.6% combined) - not to anything this candidate's hypothesis targeted.
  Whether that inflate cost itself has headroom (e.g. `fdeflate::huffman::build_table` at 2.27% is
  rebuilt once per compressed block, not once per frame - a much larger, block-count-scaled cost
  than either reset function) is a genuinely different, open lead - filed as a new idea rather than
  chased further in this candidate (see final report's `NewIdeasNoticed`).
- Not measured: whether a non-synthetic (`photo` pattern) real-world image would shift the
  encode/decode ratio - the underlying `fdeflate`/`unfilter` costs are per-byte and pattern-
  dependent on compressibility, so a different image could show a smaller or larger gap without
  changing this candidate's own conclusion (the reset paths are unmeasurable regardless of image).
