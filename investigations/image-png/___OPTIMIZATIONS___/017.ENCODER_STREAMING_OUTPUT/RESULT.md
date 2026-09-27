# ENCODER_STREAMING_OUTPUT

`Writer::write_image_data` (`src/encoder.rs`, buffered path used by every plain
`write_image_data` call - i.e. anything that isn't `StreamWriter`) builds the *entire* compressed
zlib/fdeflate stream into one in-memory `Vec<u8>` (`zlib_encoded`) before splitting it into
IDAT/fdAT chunks and writing those to the real output. `StreamWriter` already avoids this (it
writes each chunk out as soon as its buffer fills), which suggested the buffered path's own peak
memory - roughly "source image + full compressed image" held live at once - could be cut the same
way. rustc 1.98.0, cargo 1.98.0, release build, Ryzen 9 7900 (12C/24T), Linux.

## What was tried

Added a `ChunkStreamWriter` (`src/encoder.rs`) implementing `io::Write`: it buffers up to 64 KiB
and flushes that as one IDAT/fdAT chunk (via the existing `write_chunk`/`write_fdat_chunk`) as
soon as it fills, instead of requiring the whole compressed stream to exist in memory before any
chunk can be written. `DeflateCompression::Level(level)` (flate2's `ZlibEncoder`, which only needs
`Write`) now compresses directly into a `ChunkStreamWriter` wrapping the real output - no
intermediate `zlib_encoded` buffer at all for this path. `DeflateCompression::NoCompression`
(`fdeflate::StoredOnlyCompressor`) and the `FdeflateUltraFast` fallback both still buffer first:
`StoredOnlyCompressor` requires `Write + Seek` to back-patch each stored block's length header,
which a chunk-emitting sink structurally can't provide once a chunk has already been flushed and
CRC'd; `FdeflateUltraFast`'s own fallback-to-stored-on-worse-ratio check needs the final compressed
size before it can decide whether to keep it, which likewise needs the whole buffer up front. Both
of those paths were left buffered, then their already-buffered result is written out through the
same `ChunkStreamWriter` (previously: a hand-rolled `.chunks(MAX_IDAT_CHUNK_LEN)` loop) purely for
code-path uniformity - no memory benefit for those two configurations, see below.

**Why `--mode memory --skip-verify`, not the existing `compress`/`apng` modes.** The harness's
`compress`/`apng`/etc. modes all encode into a `Vec<u8>` (`encode()`) so the result can be
hashed/roundtrip-verified - which means the harness's own sink ends up holding the whole encoded
PNG in memory regardless of whether the library streamed internally, masking exactly the change
being measured here (confirmed empirically: `compress --only Level6 --size 4096` reported
`peak_rss_MiB=175.2` baseline vs `175.1` patched - no visible difference at all). Added a `memory`
mode to the harness (`___OPTIMIZATIONS___/harness/encoder_bench`, canonical copy, plus this
candidate's own `clone/`) that encodes into `std::io::sink()` instead, so peak RSS reflects only
what `png::Writer` itself holds. Its own one-time correctness check (a normal `Vec`-sink encode +
roundtrip) itself permanently raises the process's `VmHWM` before the sink-based loop even starts
(RSS is a monotonic high-water mark for the whole process) - so a new `--skip-verify` flag was
added to skip that in-process check; correctness for the same configurations is instead confirmed
by a separate `compress`-mode process run (below).

Peak RSS (`peak_rss_MiB`, `/proc/self/status` VmHWM, harness's own accounting), single
configuration per process (`--only Level6`), `Level(6)` = the crate's own default
(`Compression::Balanced`):

| image size (RGBA8, `photo` pattern) | baseline (single-core) | patched (single-core) | delta | baseline (multi-core, unpinned) | patched (multi-core, unpinned) |
|---|---|---|---|---|---|
| 512x512 (1 MiB raw) | 4.7 MiB | 4.0 MiB | -14.9% | - | - |
| 4096x4096 (64 MiB raw) | 112.2-112.4 MiB (3 trials) | 66.8-67.1 MiB (3 trials) | -40.1% to -40.5% | 111.6-111.8 MiB | 66.8-67.0 MiB |
| 8192x8192 (256 MiB raw) | 440.4 MiB | 259.0 MiB | -41.2% | - | - |

Other `DeflateCompression` settings, same harness/mode, 4096x4096, single-core (confirms the
NoCompression/FdeflateUltraFast paths are architecturally unchanged, as expected):

| cfg | baseline peak_rss_MiB | patched peak_rss_MiB | delta |
|---|---|---|---|
| NoCompression | 130.5 | 130.5 | none |
| FdeflateUltraFast | 117.7 | 117.8 | none (noise) |
| Level1 | 110.4 | 66.8 | -39.5% |
| Level6 | 112.3-112.4 | 66.8-67.1 | -40.1% to -40.5% |
| Level9 | 112.3 | 66.8 | -40.5% |

Wall-clock (from the same `memory`-mode runs, secondary metric - this candidate is about memory,
not time): flat within noise, e.g. 4096x4096 Level6 single-core p50 1447-1483ms baseline vs
1432-1458ms patched across trials - no consistent direction, as expected for a change that doesn't
touch the compression algorithm itself, only where its output bytes land.

## Result

Real, consistent, and well-explained peak-RSS reduction for the crate's *default* compression
setting (`Level(6)`/`Compression::Balanced`) and every other `Level(N)`: roughly **-40% peak RSS**
at 4096x4096 and 8192x8192 RGBA8 (scales with image size, as expected - the eliminated buffer is
itself proportional to the compressed output size), reproduced across 3 independent trials at
4096x4096 and confirmed on both a pinned single core and unpinned multi-core. At the smaller
512x512 size the win is much smaller in relative terms (-14.9%, 0.7 MiB) since fixed overhead (the
harness process itself, the source `Vec`) dominates at that scale. `NoCompression` and
`FdeflateUltraFast` show no change, matching the architectural explanation above (both still
buffer their full compressed output before writing it out, for reasons intrinsic to their own
compressors, not this patch).

The measured baseline number matches the mechanism almost exactly: at 4096x4096 the raw RGBA
buffer is 64 MiB and the `Level6` compressed output is ~45.4 MiB (`out=47561852B` from a
`Vec`-sink run) - baseline peak (112.3 MiB) is very close to raw (64 MiB) + a `zlib_encoded` buffer
that overshot to somewhere in the 45-48 MiB range via `Vec`'s doubling growth from empty; patched
peak (67.1 MiB) is very close to raw (64 MiB) alone plus the new 64 KiB `ChunkStreamWriter` buffer
(negligible) - i.e. the whole win is exactly the elimination of that one buffer, not a smaller
side effect of something else.

## Does it change behavior/output?

- **Bit-exact?** No, but decodes identically (a "characterized difference", not a correctness
  issue): compressed output is now split into 64 KiB chunks instead of one large chunk for any
  image whose compressed size exceeds 64 KiB (this applies to every `DeflateCompression` setting,
  even the two that don't get the memory win, since they now also flow through
  `ChunkStreamWriter` at the very end). Multiple IDAT/fdAT chunks per image is already
  standard, valid PNG (this is exactly what `StreamWriter` has always produced) and this crate's
  own decoder - and every other conformant PNG decoder - already handles it.
- **Magnitude of the difference:** consistently **+0.017% to +0.018% larger output** (12 bytes of
  chunk framing overhead per 64 KiB boundary crossed), measured identically across
  `NoCompression`/`FdeflateUltraFast`/`Level1`/`Level3`/`Level6`/`Level9` at 4096x4096
  (e.g. `Level6`: 47,561,852 B -> 47,570,552 B) and on a real corpus file
  (`tests/benches/kodim02.png`, `Level6`: 631,220 B -> 631,328 B, +0.017%). Below ~64 KiB
  compressed output (small images, e.g. the 512x512 case above) there is no difference at all -
  output stays exactly one chunk, identical to baseline.
- **Is this a breaking change?** No public API changes. Every encoded file remains a valid PNG/
  APNG that decodes to the identical pixels (verified below); only the internal chunk boundaries
  of the *encoder's own default output* move, the same way they always have for anyone already
  using `StreamWriter` or a small `stream_writer_with_size`.
- **Correctness checks performed:**
  - `compress` mode (harness's own `Vec`-sink + roundtrip verification, hash-checked every
    iteration) at 4096x4096: `verify=OK` for all 6 `DeflateCompression` settings, baseline and
    patched.
  - `apng` mode (fdAT path, sequence-number bookkeeping), 1024x1024, 6 frames: `verify=OK`
    baseline and patched (`out` grew by 4,692 B total across 6 frames, matching the same
    per-64-KiB-boundary overhead).
  - Real corpus file (`tests/benches/kodim02.png`, 768x512 RGB8), `Level6`: `verify=OK` both.
  - Project's own `cargo test --release` (unit + 3 integration test binaries + 8 doctests): **107
    passed, 1 ignored, 0 failed - identical before and after** (91 lib unit + 1 `bugfixes` + 4
    `check_testimages` + 3 `partial_decode` + 8 doctests each run).

## Why the result looks the way it does

The default compression path (`Level(6)`, what almost every caller of the plain `Encoder` API
actually uses) held two full-size buffers alive simultaneously at its peak: the raw pixel data
handed in by the caller (`data: &[u8]`, not owned by the encoder but still resident for the whole
call) and the complete compressed `zlib_encoded: Vec<u8>` being built up via zlib compression
before a single byte of it could be written out. Streaming compression output directly into
64 KiB chunks removes the second buffer entirely - only ever a small, fixed-size window of
compressed bytes is resident at once, mirroring what `StreamWriter` was already doing. The two
paths that could *not* be converted are both blocked by a real API constraint in `fdeflate`, not
an oversight: `StoredOnlyCompressor` needs `Seek` (to fix up each stored block's length after
the fact), and `FdeflateUltraFast`'s worse-than-stored fallback needs to compare the *finished*
compressed size against a threshold before deciding what to keep, which is inherently
post-hoc.

## What's not done / open questions

- `NoCompression` and `FdeflateUltraFast` still buffer their full output before writing it out;
  making those genuinely streaming would need either a different `fdeflate` API (a `Seek`-free
  stored-block writer, or a compressor that reports its own worse-than-stored condition
  incrementally instead of after the fact) - out of scope for a `png`-side patch, a candidate for
  vendoring/upstreaming into `fdeflate` if this matters enough in practice.
- `STREAM_CHUNK_CAP` was fixed at 64 KiB (chosen for a large size/memory tradeoff margin, informed
  by candidate 010's finding that 64 KiB is near the size/overhead sweet spot for `StreamWriter`'s
  own buffer) rather than swept across sizes for this specific candidate; a smaller cap would trade
  more chunk framing overhead for an even smaller constant memory footprint, a larger cap the
  reverse - not measured here.
- Not measured: `NoCompression`'s and `FdeflateUltraFast`'s own *buffered* peak RSS at very large
  image sizes (e.g. multi-hundred-MB images) - likely similar in magnitude to `Level(6)`'s own
  unpatched baseline, unaffected by this patch either way, per the "no change" rows above.
- The harness's new `memory` mode + `--skip-verify` flag were added to the canonical
  `___OPTIMIZATIONS___/harness/encoder_bench/` (not just this candidate's own copy) since the
  underlying masking problem (a `Vec`-sink hiding an internal buffering change) applies to any
  future encoder-side peak-RSS candidate, not just this one.
