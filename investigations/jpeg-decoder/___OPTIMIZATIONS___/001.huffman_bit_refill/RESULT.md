# 001.huffman_bit_refill

Investigated `HuffmanDecoder::read_bits`/`peek_bits`/`consume_bits` in `src/huffman.rs`
(lines 81-164), the bit-buffer refill used by every Huffman symbol decode in baseline/progressive
JPEG entropy decoding, per the lead in `IDEAS.md`. Conclusion: **no patch applied** - the
byte-at-a-time refill is a deliberate, load-bearing part of correctness for a generic `Read`
source, not an oversight, and the obvious "read more bytes per call" change is unsafe to make
generically. Details below.

## What was tried

No source change was applied. This is a code-reading + measurement investigation only.

| Variant | single-core p50 (`taskset -c 0`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline, `benches/large_image.jpg`, 30 iters, run 1 | 28.305 ms | 24.597 ms | |
| baseline, `benches/large_image.jpg`, 30 iters, run 2 | 28.615 ms | 24.626 ms | noise floor ~0.3 ms |
| baseline, `tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg`, 30 iters, run 1 | 258.416 ms | 256.529 ms | |
| baseline, same image, run 2 | 256.391 ms | 256.907 ms | noise floor ~2 ms; single-core and multi-core times are within noise of each other |

No patched binary exists (no patch), so `tools/paired.sh` was not run - there is nothing to pair
against.

## Result

No change measured because no change was made. The one measured fact worth recording: on the
biggest/best-signal-to-noise workload (`jpeg_lossless_sel1-rgb.jpg`, 3072x2048), single-core and
multi-core `min`/`p50` are statistically indistinguishable (254-258 ms both ways, within the
~2 ms repeat-run noise floor), while `large_image.jpg` (2268x1512) does show a real multi-core
speedup (~28 ms -> ~24-25 ms). This is consistent with the larger image spending a bigger share
of its decode time in the sequential, unparallelizable Huffman-entropy-decode stage (which
`rayon` cannot spread across cores, since decoding one scan's bitstream is inherently
sequential) relative to the parallelizable IDCT/upsample/color-convert stages - i.e. it is
independent corroborating evidence that the Huffman decode loop is a real hot path on at least
one of the two benchmark images, even though no patch is on offer here. It is not proof by
itself (could also be explained by the lossless image's rayon work-splitting granularity, scan
count, or thread-pool overhead relative to its size) and no further ablation was run to isolate
it further.

## Does it change behavior/output?

N/A - measurement/investigation only, no patch applied to real source. No `patch.diff` in this
folder, intentionally (per `SKILL.md`'s "the only two valid states" - this is the pure
investigation state, not an omission).

- **Bit-exact?** N/A, nothing changed.
- **Is this a breaking change?** N/A.
- Project test suite (`cargo +stable test --release`, HEAD `eb2d7c0f6a2d0298aba7a7f8b9ca1440353e8f8c`,
  unmodified tree): 23 passed (5 lib unit tests + 12 `tests/lib.rs` integration tests + 4 rayon
  integration tests, 1 each + 2 doctests), 0 failed, 0 errors. Not re-run "after" since nothing
  was changed.

## Why the result looks the way it does

Read `src/huffman.rs` closely (the whole file) and its call sites in `src/decoder.rs`
(`decode_block`, `decode_block_successive_approximation`, `refine_non_zeroes`, all generic over
`R: Read`).

1. **The refill is already batched, not per-symbol.** `HuffmanDecoder::decode`/`get_bits` only
   call `read_bits` when the 64-bit `bits` buffer has fewer than 16 (or `count`) bits left
   (`src/huffman.rs:32-34`, `82-84`). `read_bits` itself (`:123-160`) then loops
   `while self.num_bits <= 56`, i.e. it tops the buffer back up to essentially full (56-64 bits,
   7-8 bytes) every time it runs, not one byte per call. So the *check* (`if self.num_bits < 16`)
   runs once per symbol, cheaply (a branch that's false most of the time), but the actual byte
   I/O already happens in bursts of several bytes roughly once every 7-8 bytes of compressed
   stream consumed, not once per symbol as the initial framing of this lead assumed.

2. **Each byte read still goes through `read_u8` -> `Read::read_exact` on a 1-byte buffer**
   (`src/lib.rs:56-60`), called once per byte inside that burst. For the reader type actually used
   by every benchmark and by most real callers of this crate for in-memory decoding
   (`std::io::Cursor<&[u8]>` / `&[u8]`), `read_exact` on a 1-byte buffer is already close to the
   cheapest possible operation (bounds check + 1-byte copy + pointer bump), and because
   `HuffmanDecoder::read_bits`/`decode`/`decode_scan` are ordinary generic functions (not trait
   objects), `R`'s `Read` impl is monomorphized and inlinable at each call site - there is no
   vtable/dynamic-dispatch cost to cut here, unlike the "generic dispatch overhead" framing in the
   original lead suggested. This matches `HEURISTICS.md`'s own caution that some entropy coders
   are already about as good as this gets.

3. **Batching multiple byte reads into one `Read::read`/`read_exact` call is unsafe to do
   generically**, and this is the actual finding worth recording. The byte-stuffing/marker-scan
   loop inside `read_bits` (`:131-153`) reads one byte, and *only* reads a second byte if the
   first was `0xFF`, specifically to distinguish a stuffed `0xFF 0x00` (real data byte) from a
   real marker (`0xFF` followed by a marker code) - the code's own comment explains why:
   "we can't put data back into the reader, so we have to continue reading to identify the
   marker." The moment a real marker is found, `read_bits` stops touching `reader` at all for the
   rest of that call (remaining iterations just zero-fill `self.bits`), which leaves the
   `reader`'s cursor sitting exactly at the first byte *after* the marker - the outer
   `Decoder::decode_scan`/marker-handling code depends on that exact position to keep parsing the
   rest of the file correctly.
   If `read_bits` instead read, say, 4 or 8 bytes at once into a local buffer with a single
   `Read::read`/`read_exact` call (the natural way to cut per-byte call overhead), and a marker
   happened to start partway through that batch, the bytes *after* the marker within the same
   batch would already have been pulled out of `reader` and buffered locally, even though the
   original byte-at-a-time design would have stopped reading right at the marker. For a generic,
   possibly non-seekable `R: Read` (a pipe, a network stream, anything that isn't a slice), those
   extra bytes cannot be "put back" - the reader's position would end up wrong for whatever reads
   `reader` next (the rest of the file's marker/segment parsing), silently corrupting decode of
   any file where a scan's entropy data doesn't end on an exact multiple of the batch size, which
   is effectively "most real files." This isn't a hypothetical edge case; it is exactly the
   scenario the existing code's own comment is defending against.
   A batch size of 1 (i.e. the status quo) is the only batch size with zero over-read risk for an
   arbitrary `Read` source, which is why the code reads one byte at a time.

4. **A safe version of this idea exists only for specific reader types, and is out of scope for
   this candidate.** `Cursor<&[u8]>` and `&[u8]` (already `BufRead` in `std`, with a zero-copy
   `fill_buf`/`consume`) could look ahead into the underlying slice without truly consuming it,
   scan for the marker locally, and then advance the position by exactly the bytes actually used -
   avoiding the over-read problem for these specific types. But `HuffmanDecoder`'s methods are
   generic over `R: Read` only (not `R: BufRead`), and this generic bound is part of the crate's
   public API surface (`Decoder<R: Read>`) - tightening it to `BufRead` would be a breaking API
   change for any caller passing a raw, unbuffered `Read` (e.g. a bare `File`). The alternative,
   detecting the concrete reader type at runtime via `std::any::Any`/`TypeId` downcasting to opt
   into a fast path only for `&[u8]`/`Cursor<&[u8]>`, would work without an API break, but adds
   real complexity (a second, separately-tested code path plus `unsafe`-adjacent
   downcasting) disproportionate to what a single "did this help" measurement should risk
   introducing into a widely-used crate. Not attempted this session; see `IDEAS.md` for a
   narrower write-up of this specific idea for a future session that wants to take on that scope
   deliberately.

## What's not done / open questions

- No ablation was run to directly quantify what fraction of decode time is Huffman/bit-refill
  specifically vs. IDCT/upsampling/color-convert (e.g. via a temporary phase-timer instrumentation
  reverted afterward) - the single-core-vs-multi-core parity on the lossless/1 image is
  suggestive, not a direct measurement of the Huffman stage's share.
- The `Any`/`TypeId`-downcast specialization for `&[u8]`/`Cursor<&[u8]>` described in point 4
  above was not implemented or measured - it's a real, larger candidate for a future session, not
  ruled out, just out of scope here given the correctness risk of doing it hastily.
- `src/decoder.rs` and `src/parser.rs` still have no dedicated read-through pass per
  `projects/jpeg-decoder.md`'s "Next-session leads" - a full profiling/reading pass there might
  turn up a bigger, safer lever than this one.
