# Transcode::transcode (decode_inline, 8-bit path): consume decoded pixels instead of cloning them

Third finding from the same "is a large CIOD's pixel data cloned where a consume would do"
sweep as candidates 019/020 (`object`/`ul`), this time in `pixeldata/src/transcode.rs` - the
`Transcode` trait's implementation for converting a `FileDicomObject` from one transfer syntax to
another.

`decode_inline` (called when transcoding *from* an encapsulated/compressed transfer syntax *to* a
native/uncompressed one - e.g. JPEG Baseline -> Explicit VR Little Endian) decodes the pixel data
via `obj.decode_pixel_data()`, which returns an owned `DecodedPixelData<'a>` (its `data` field is
a `Cow<'a, [u8]>`, and **every construction site in `pixeldata/src/lib.rs` builds it as
`Cow::Owned`** - confirmed by reading all 5 places `DecodedPixelData` is constructed, none ever
use `Cow::Borrowed`). The 8-bit branch then did:

```rust
let pixels = decoded_pixeldata.data().to_vec();
```

`decoded_pixeldata` is a local variable, never used again after this line (confirmed by reading
the rest of the function) - it is dropped immediately after. `.data()` borrows `&self.data`
(`&[u8]`), so `.to_vec()` clones the *entire decoded pixel buffer* just to throw away the
original a moment later - exactly the "clone instead of consume, invisible in peak RSS because
the redundant copy is freed right after" pattern this sweep was asked to look for. For
`color3d_jpeg_baseline.dcm` (480x640x3, 120 frames, decodes to ~105 MB), that's a ~105 MB `memcpy`
on every transcode call, for nothing.

## What was tried

1. **`pixeldata/src/lib.rs`**: added `DecodedPixelData::into_data(self) -> Vec<u8>`, a consuming
   accessor: `self.data.into_owned()`. For the (always, in practice) `Cow::Owned` case this is a
   plain move, not a clone; it only copies if the data were genuinely borrowed (structurally
   still possible via the `'a` lifetime parameter, just never exercised by any constructor in this
   crate today).
2. **`pixeldata/src/transcode.rs`**: `decode_inline`'s 8-bit arm now calls
   `decoded_pixeldata.into_data()` instead of `decoded_pixeldata.data().to_vec()`. The 16-bit arm
   (`data_ow()`) is untouched - it performs an actual byte-pair-to-`u16` type conversion, not a
   plain ownership transfer, so there's no equivalent "just move it" shortcut available there.

## Correctness verification

- **`cargo test -p dicom-pixeldata --release --features image,native`: 71 passed / 0 failed**
  (58 lib + 1+1+11 doctests), both before and after.
- **`cargo test --workspace` (default features): 537 passed / 0 failed, both before and after.**
- **New harness mode `transcode-to-native`**: `Transcode::transcode` from JPEG Baseline to
  Explicit VR Little Endian on `color3d_jpeg_baseline.dcm`, output written via `write_all` and
  hashed - **checksum identical between baseline and patched builds** (`0xbf9b774ccdcfb473`),
  confirming the transcoded object's bytes are unaffected.
- `cargo clippy -p dicom-pixeldata --release --features image,native`: clean, both before and
  after.

## What was tried (performance) and Result

Benchmark: new `transcode-to-native <file> <iters>` harness mode - re-parses the object fresh
each iteration (transcode mutates in place; a second call to the same target TS is a no-op),
times `obj.transcode(&target_ts)`, then writes the result out to compute a correctness checksum
(the write itself is outside the timed region). Fixture: `color3d_jpeg_baseline.dcm` (JPEG
Baseline, 480x640, 3 samples/pixel, 120 frames - decodes to ~105 MB of native pixel data).

| Variant | single-core p50 (`taskset -c 0`) | multi-core p50 |
|---|---|---|
| baseline (`.data().to_vec()`) | 240.6 ms | 223.9 ms / 231.5 ms (2 rounds) |
| patched (`.into_data()`) | 191.3 ms | 175.8 ms / 177.2 ms (2 rounds) |

**-20.5% single-core, -21.5%/-23.5% multi-core across two independent rounds, 9/9 paired wins in
all three rounds, fully consistent.** `uptime` load average was 1.5-2.4 throughout (real but
moderate background load; the fully consistent 9/9 verdicts across three independent
configurations leave no ambiguity).

## Does it change behavior/output?

- **Bit-exact?** Yes - see "Correctness verification" above.
- **Is this a breaking change?** No. `into_data` is a new, additive `pub` method on
  `DecodedPixelData`. No existing method signature changed.
- Project test suite: **537 passed / 0 failed, both before and after** (`cargo test --workspace`);
  **71 passed / 0 failed, both before and after**, specifically for `dicom-pixeldata`.

## Why the result looks the way it does

Unlike candidate 020's PDataWriter fix (where the eliminated copy was one of *two* unavoidable
copies, and the *other* one - the destination sink's own write - dominated), here the eliminated
`.to_vec()` was a **pure, unconditional waste**: nothing downstream needed a second copy of the
pixel buffer, and the *original* buffer was discarded a line later regardless. The whole cost of
that `memcpy` (proportional to decoded frame data size, ~105 MB here) simply disappears. The
remaining ~175-240ms is dominated by actual JPEG decoding work (120 frames) and the subsequent
`write_all` pass, which this fix does not touch - the ~20-23% reduction is entirely attributable
to removing the one redundant full-buffer copy, consistent with its size relative to the total
per-call cost.

## Peak memory (measured directly, result differs from time - reported honestly)

Using the same `/proc/self/status` `VmHWM` technique as candidate 019 (new
`transcode-to-native-peak-mem` mode, one-shot transcode + write, no loop): baseline
341,224 / 342,936 / 343,304 KB vs. patched 341,576 / 342,640 / 342,568 KB (3 runs each) -
**no consistent difference, well within run-to-run noise (~0.2-0.5%).** This is the opposite
finding from candidate 019 (where the analogous clone-removal produced a large, clean peak-RSS
drop) - **the redundant `.to_vec()` clone removed here does not set this process's peak RSS**,
because decoding a 120-frame JPEG image itself already allocates enough (per-frame decode
buffers, the concatenated ~105 MB output buffer, `image`/`jpeg-decoder` internals) to establish a
peak at or above what the extra clone would have added on top. The wall-clock win is real and
large; the peak-RSS win is, on this fixture, not present. Both facts are reported because neither
implies the other - a wasted `memcpy` is wasted CPU time regardless of whether it happens to be
the tallest allocation in the process at the time.

## What's not done / open questions

- **The 16-bit arm (`data_ow()`) was not touched.** It performs a real transformation (bytes ->
  `u16` samples), not a pure ownership transfer, so `into_data()` doesn't directly apply there;
  whether `data_ow()` itself could avoid an intermediate allocation (e.g. by writing directly into
  the `PrimitiveValue::U16` buffer instead of an intermediate `Vec<u16>`) was not investigated in
  this pass - a distinct, smaller-scoped question, noted in `IDEAS.md`.
- **Other `Transcode` code paths were not audited as closely.** `decode_and_encode` (the
  decode-then-re-encode path, `(true, true)` case not covered by the JPEG-recompression
  fast-path) and the JPEG-recompression fast path itself (`writer.encode(&*self, options.clone(),
  ...)`) were skimmed but not measured - `options.clone()` there is a small struct
  (`EncodeOptions`), not flagged as a concern, but the fragment/offset-table handling in
  `decode_and_encode` was not specifically checked for the same class of redundant clone.
- Not tried on a fixture with fewer/larger frames (e.g. one huge frame instead of 120 smaller
  ones) - the peak-RSS result above may be specific to this fixture's per-frame JPEG decode
  overhead already dominating; a single-frame, very-large native-after-decode fixture might show
  a different peak-RSS relationship. Not measured due to no single-frame 8-bit encapsulated
  fixture of comparable decoded size being available in this session's testdata.
