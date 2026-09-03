# unpack_overlay_bits: table-driven byte unpack instead of one bit at a time

Fresh discovery from `HEURISTICS.md`'s "bitstream/entropy reader doing 1 byte or 1 bit at a time"
pattern, found while sweeping `pixeldata/src/overlay.rs` (overlay plane decoding - not previously
touched by any candidate this session, unlike the LUT/colorspace/pixel-data-writer code that's
already been gone over extensively).

`unpack_overlay_bits` (`pixeldata/src/overlay.rs`) expands a DICOM overlay plane's packed 1-bit
bitmap (`Overlay Data`, `60xx,3000`) into one byte per pixel (0 or 1), needed before an overlay
can be rendered or queried. The original implementation:

```rust
fn unpack_overlay_bits(packed: &[u8], num_pixels: usize) -> Vec<u8> {
    (0..num_pixels)
        .map(|i| (packed[i >> 3] >> (i & 0x7)) & 1)
        .collect()
}
```

processes exactly one output pixel per loop iteration: a bounds-checked index into `packed`
(`packed[i >> 3]`), a shift, and a mask - for every single bit, even though 8 output bytes always
come from the same one input byte.

## What was tried

Added a 256-entry lookup table (`UNPACK_BITS_TABLE: [[u8; 8]; 256]`, built at compile time via a
`const` block) mapping each possible packed byte directly to its 8 unpacked output bytes (LSB
first, matching the original's bit order exactly). The function now iterates over `packed` a
**byte** at a time (plain slice iteration, no per-element bounds-checked indexing) and does one
`extend_from_slice` of 8 bytes per input byte, handling only the final partial byte (if
`num_pixels` isn't a multiple of 8) as a short slice from the same table.

## Correctness verification

- **`cargo test -p dicom-pixeldata --release --features image,native`: 71 passed / 0 failed**
  (58 lib + doctests), both before and after - including `unpack_bits_is_lsb_first` (an explicit
  bit-order assertion) and `decode_overlays_from_mr_with_overlays` (a real DICOM file with actual
  overlay data), plus every other overlay test (multi-frame, embedded/legacy, error cases).
- **`cargo test --workspace` (default features): 537 passed / 0 failed, both before and after.**
- **New harness mode `overlay-decode <rows> <cols> <iters>`**: builds a synthetic object with one
  4096x4096 overlay plane (~16.8M pixels) and calls the public `PixelDecoder::decode_overlays()`
  entry point - **checksum of the unpacked output identical between baseline and patched builds**
  (`0x1085a213bc222325`).
- `cargo clippy -p dicom-pixeldata --release --features image,native`: clean, both before and
  after.

## What was tried (performance) and Result

Benchmark: `overlay-decode 4096 4096 20` (~16.8M-pixel synthetic overlay plane, packed as
~2.1MB, unpacked to ~16.8MB) - isolates `decode_overlays()`, which for a standard (non-legacy)
overlay plane is almost entirely `unpack_overlay_bits`.

| Variant | single-core p50 (`taskset -c 0`) | multi-core p50 |
|---|---|---|
| baseline (bit-at-a-time) | 8.813 ms | 8.676 ms / 8.689 ms (2 rounds) |
| patched (table-driven) | 1.009 ms | 1.066 ms / 1.025 ms (2 rounds) |

**-88.5% single-core, -87.7%/-88.2% multi-core across two independent rounds, 9/9 paired wins in
all three rounds, fully consistent.** `uptime` load average was 1.2-1.5 throughout. This is the
largest relative win found this session - consistent with the heuristic's own "10-20% if genuinely
unoptimized" estimate being a floor, not a ceiling, when the original code was doing meaningfully
more redundant work per byte than a typical case (a bounds-checked array index *and* a shift *and*
a mask, 8 separate times, for what the table-driven version does once).

## Does it change behavior/output?

- **Bit-exact?** Yes - see "Correctness verification" above (checksum-identical output on a real
  synthetic overlay, plus every existing overlay test passing unmodified, including the explicit
  bit-order test).
- **Is this a breaking change?** No. `unpack_overlay_bits` and the new `UNPACK_BITS_TABLE` are
  both private (non-`pub`) items in `pixeldata/src/overlay.rs`. No public API changed.
- Project test suite: **537 passed / 0 failed, both before and after** (`cargo test --workspace`);
  **71 passed / 0 failed, both before and after**, specifically for `dicom-pixeldata`.

## Why the result looks the way it does

The original loop did real, unavoidable-looking work eight times over for what is structurally a
single input byte: recompute `i >> 3` (always the same value for 8 consecutive `i`), bounds-check
`packed[i >> 3]` (the same index, checked 8 times), shift, and mask. The table-driven version
collapses all of that into one table lookup plus a fixed 8-byte copy, and LLVM can trivially
auto-vectorize/unroll `extend_from_slice` of a compile-time-sized `[u8; 8]`. The win is large
because *every* step of the original inner loop was redundant relative to the byte-level
granularity of the actual data, not just one part of it.

## What's not done / open questions

- Not tried: an even more aggressive SIMD/`u64`-at-a-time unpack (e.g. PDEP/pext-style bit
  expansion, or processing 8 packed bytes = 64 pixels per SIMD lane). Given the table-driven
  version already gets within striking distance of a pure `memcpy`-bound rate for this access
  pattern (8x data expansion, mostly bound by the *output* write volume rather than compute), a
  further SIMD rewrite was not attempted - diminishing returns expected, not measured.
- Not tried: applying the same "many tiny redundant per-bit computations" fix to
  `extract_embedded_overlay_bits` (the legacy/embedded overlay path, `pixeldata/src/overlay.rs`)
  - it already uses `chunks_exact` (no per-element bounds-checked indexing), so it doesn't have the
  same anti-pattern; not benchmarked separately since it looked structurally fine on inspection.
- Real-world significance depends on overlay plane size and how often `decode_overlays` is called
  per object - overlays are a comparatively niche DICOM feature (mostly legacy ultrasound/nuclear
  medicine annotations), so the *absolute* time saved per typical (much smaller than 4096x4096)
  overlay plane will be proportionally smaller than this benchmark's synthetic worst case, though
  the *relative* win (-88%) should hold at any size given the mechanism is purely about
  per-iteration overhead, not a fixed cost that amortizes away.
