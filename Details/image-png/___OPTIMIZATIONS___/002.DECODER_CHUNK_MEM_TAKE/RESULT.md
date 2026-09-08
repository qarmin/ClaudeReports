# Move chunk bytes out instead of cloning them for PLTE/sBIT/tRNS/eXIf/bKGD

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

`self.current_chunk.raw_bytes` holds one chunk's fully-assembled bytes (`parse_chunk` runs only
after all of a chunk's bytes have arrived). The per-chunk-type parsers for PLTE, sBIT, tRNS, eXIf
and bKGD (`src/decoder/stream.rs:1325, 1357, 1403, 1755, 2033`) each did
`self.current_chunk.raw_bytes.clone()` to get an owned copy for `Info` - a full second copy of
data the decoder is about to discard anyway, since `raw_bytes` is unconditionally `.clear()`-ed
before the next chunk's bytes start arriving. `eXIf` in particular can carry a real-world EXIF
blob hundreds of KB to low-single-digit MB in size.

## What was tried

**First attempt failed a correctness check and is recorded here because the failure mode itself
is the most important finding in this candidate.** The obvious fix, `std::mem::take(&mut
self.current_chunk.raw_bytes)`, passed a first correctness spot-check (an empty eXIf chunk) but
then **hung indefinitely** decoding a PNG with a 1024-byte eXIf chunk. Root cause: `mem::take`
replaces the Vec with `Vec::new()`, which has **zero capacity**, not just zero length - and the
chunk-reading state machine's buffer-growth step (`ReadChunkData` in `next_state`,
`src/decoder/stream.rs:802-823`) computes how much to grow the buffer by as
`raw_bytes.capacity().min(self.limits.bytes)`. Every chunk in the file up to that point had kept
`raw_bytes.capacity() >= CHUNK_BUFFER_SIZE (128)` (chunk construction reserves 128 bytes up front,
`.clear()` never shrinks capacity), so this invariant silently held in the unpatched crate and the
growth-loop bug it exposes is unreachable there - `mem::take` broke that invariant by handing back
a genuinely empty, zero-capacity Vec, and the *next* chunk read `0.min(limit) = 0` bytes to grow by
every step forever: zero bytes consumed, zero progress, same state forever. **This is not a
pre-existing bug in the shipped crate** (the invariant always holds without this patch) - it only
manifested because of this patch's own first attempt, and is recorded here as a correctness lesson,
not filed to `___BUGHUNT___` (nothing in current `HEAD` is affected).

The fix used instead: `std::mem::replace(&mut self.current_chunk.raw_bytes,
Vec::with_capacity(CHUNK_BUFFER_SIZE))` - same move-out-the-data effect, but the buffer left behind
for the next chunk has the same non-zero starting capacity a freshly-constructed `ChunkState` would
have, preserving the invariant the growth loop depends on.

Measured with a standalone bench (`bench/`, this candidate's own - no existing harness reaches
metadata-chunk decoding): builds a real 1x1 grayscale PNG via the crate's own encoder (for a valid
tRNS chunk) and splices in a hand-built eXIf chunk of a given size before IDAT, then decodes it
repeatedly, verifying `exif_metadata`/`trns` come back byte-correct every iteration.

| exif chunk size | baseline p50 (n=270, `taskset -c 0`) | patched p50 | allocs (avg) | reallocs (avg) | bytes/decode (avg) |
|---|---|---|---|---|---|
| 0 B | 0.0005ms | 0.0005ms | 6.0 -> 7.0 | 0.0 -> 0.0 | 19.37 -> 19.62 KiB |
| 1024 B | 0.0006ms | 0.0006ms | 7.0 -> 7.0 | 3.0 -> 3.0 | 21.24 -> 20.49 KiB |
| 65536 B | 0.0032ms -> 0.0024ms | | 7.0 -> 7.0 | 9.0 -> 9.0 | 147.24 -> 83.49 KiB |
| 1,048,576 B | 0.5849ms | 0.0280ms | 7.0 -> 7.0 | 13.0 -> 13.0 | 2067.24 -> 1043.49 KiB |

Paired/alternating comparison (`tools/paired.sh`, `RUNS=15`, `taskset -c 0`, `--iters 300`; the
tool reads the *last* printed `RESULT` line, which is the 1,048,576 B config):

```
A (baseline): median 0.586ms
B (patched):  median 0.028ms
-95.2%, B faster in 15 of 15 pairs - verdict: consistent
```

## Result

**Time: real, large, consistently-reproduced win for chunks in the hundreds-of-KB-to-MB range**
(only eXIf realistically reaches this size among the 5 patched chunk types - PLTE/sBIT/tRNS/bKGD
are all bounded to at most a few hundred bytes by the PNG spec itself). -95.2% median latency for a
1 MiB eXIf chunk, 15/15 paired runs consistent. For small chunks (0-1024 B) the difference is at or
below this machine's measurement floor - not a meaningful win or loss either way.

**Allocation count:** unchanged (still one allocation per parsed chunk) - the fix trades a large
`clone()` for a move, but a small `Vec::with_capacity(CHUNK_BUFFER_SIZE)` still has to be allocated
to leave a valid buffer behind for the next chunk, so allocation *count* doesn't improve, only the
*bytes copied* does (confirmed via the `bytes_avg_KiB` column above, and directly explains the time
result: no more O(n) `memcpy` of the chunk payload).

**Allocated bytes:** roughly halved for chunks large enough for the difference between "clone the
existing buffer" and "128 fresh bytes plus a move" to matter (65536 B: 147 -> 83 KiB; 1 MiB: 2067 ->
1043 KiB - almost exactly the removed clone's worth). For a 0 B chunk it's marginally *larger*
(19.37 -> 19.62 KiB, +0.25 KiB) since a 128-byte buffer is now allocated even when there was nothing
to save a copy of - negligible in absolute terms.

## Does it change behavior/output?

- **Bit-exact?** yes - the standalone bench asserts the decoded `exif_metadata`/`trns` bytes match
  exactly, every iteration, for every chunk size tested, both before and after.
- **Is this a breaking change?** No - `Info`'s public fields and the decoder's public API are
  unchanged; only how an internal scratch buffer is recycled changed.
- Project test suite (`cargo test --release`): **107 passed, 1 ignored, 0 failed**, identical
  before and after.

## Why the result looks the way it does

`Vec::clone()` for N bytes is one allocation plus one `memcpy` of N bytes; `mem::replace` with a
small fresh buffer is one allocation of 128 bytes (not N) plus a pointer move - the removed
`memcpy` is exactly where the time goes for large chunks, and exactly why nothing changes for tiny
ones (a 128-byte alloc either way, memcpy of <1KB is already below the noise floor here). Only
eXIf realistically benefits in absolute terms because it's the only one of the five chunk types
without a spec-imposed small size cap.

## What's not done / open questions

- iCCP (also named in the original backlog entry) turned out **not** to need this fix - it never
  clones `raw_bytes` in the current source; it borrows `&self.current_chunk.raw_bytes[..]` directly
  (`src/decoder/stream.rs:1777`, inside `parse_iccp_raw`, which passes it straight into the zlib
  inflater without an intermediate owned copy). No change was made there.
- `parse_captured` (`src/decoder/stream.rs:~1815`, feeds `Info::captured_chunks` for chunk types
  the caller opted into capturing via `DecodeOptions`) still uses `.clone()` and was **deliberately
  left unpatched**: it runs *before* the type-specific parser in `parse_chunk`'s dispatch, so for a
  chunk type that is both "captured" and one of PLTE/sBIT/tRNS/eXIf/bKGD, the specific parser still
  needs `raw_bytes` intact afterward - taking/replacing it in `parse_captured` would starve that
  second read. A correct fix would need to special-case "this chunk type has no further specific
  parser," which is more branching than this bounded candidate's scope; noted as a follow-up idea
  in IDEAS.md rather than attempted here.
- Not measured: whether the growth-loop invariant this candidate depends on (`raw_bytes.capacity()
  never reaching 0`) has any other place in the codebase that could violate it independently of this
  patch. Not found in this session's reading, but worth keeping in mind for any future change to
  `ChunkState`/`raw_bytes` handling.
