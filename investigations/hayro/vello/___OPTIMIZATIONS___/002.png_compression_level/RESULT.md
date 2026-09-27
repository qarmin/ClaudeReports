# PNG DEFLATE compression level (`Pixmap::into_png`)

## Why this was a target

The pipeline breakdown in
[`../../hayro/___OPTIMIZATIONS___/README.md`](../../hayro/___OPTIMIZATIONS___/README.md) flagged
PNG encoding as a bigger cost than anything else investigated in that audit (26-31% of total time
on 2 of 3 generic test files). Re-measured specifically on the user's own `img.pdf`: **DEFLATE
alone is 59.5% of total render+encode time** for that file - by far the single biggest phase in
the entire pipeline, bigger than parsing, interpreting, decoding, and rasterizing *combined*.

`into_png()` never called `encoder.set_compression(...)`, so it used the `png` crate's default,
`Compression::Balanced` - which resolves to the `flate2`-based DEFLATE implementation at zlib's
default level. The `png` crate itself documents a purpose-built alternative: `Compression::Fast`
uses [`fdeflate`](https://crates.io/crates/fdeflate), "tuned for PNG... significantly outperforms
libpng... while still providing better compression ratio than the fastest modes of other
encoders."

**Note (added when this workspace was ported to the current format):** the user's actual
downstream pipeline encodes to JPEG via a separate crate (`jpeg-encoder`), not through this PNG
path - this finding is real for anyone using vello's built-in PNG output, but was not relevant to
the specific pipeline this audit was ultimately scoped to. See the hayro-side `README.md`.

## What was tried

`patch.diff` - one line: `encoder.set_compression(png::Compression::Fast);` in
`Pixmap::into_png`.

DEFLATE is a lossless compression format by construction - the compression level only trades
encoding speed for output byte size; it cannot change what pixels come out when the PNG is
decoded again. This is fundamentally different from the `take_unpremultiplied` LUT fix or the
(reverted) IDWT rewrite, both of which touch pixel values. Verified this empirically anyway, not
just by trusting the spec (see below).

## Benchmark: `bench/`

Self-contained project (own `Cargo.toml`, real path dependency on `vello_common`), using real
rendered output from the user's own `img.pdf` (595x841, embedded via `include_bytes!` in
`testdata/img_page_rgba.bin`) rather than synthetic noise - PNG compression ratio (and therefore
the relative cost of different DEFLATE backends) is very sensitive to real content patterns,
unlike raw encode/decode speed.

```
cd bench
cargo run --release -- 30
```

| Compression level | single-core p50 encode time | Δ% vs `Balanced` | Output size | Size Δ% |
|---|---|---:|---|---:|
| `Balanced` (previous default) | 12.708 ms | - | 115,769 bytes | - |
| `Fast` (this patch) | 1.554 ms | -87.8% (8.2x faster) | 163,293 bytes | +41.1% |
| `Fastest` | 1.187 ms | -90.7% (10.7x faster) | 210,363 bytes | +81.7% |
| `NoCompression` | 0.801 ms | -93.7% (15.9x faster) | 2,002,639 bytes | +1629.8% |
| `High` | 39.127 ms | +207.9% (3.1x slower) | 116,086 bytes | +0.3% |

`Fast` was chosen over `Fastest`/`NoCompression` as the better speed/size balance - `Fastest` only
adds ~9% more end-to-end speedup (see below) for roughly double the file-size penalty, and
`NoCompression` bloats the file 16x.

## Result

`taskset -c 2`, 50 iterations, temporary timing script (render + `into_png`, same as the real
`render.rs` example's workflow), real `img.pdf`, full render + encode:

| | single-core p50 (full render + PNG encode) | Δ% |
|---|---|---:|
| Baseline (`Balanced`) | 8.40 ms | - |
| `Fast` (this patch) | 4.32 ms | -48.6% |
| `Fastest` (for comparison, not applied) | 3.92 ms | -53.3% |

## Does it change behavior/output?

- **Bit-exact?** yes for decoded pixels (DEFLATE is lossless at every level) - only the encoded
  file's byte size changes, not what it decodes back to. Verified empirically:
  `bench/` decodes every candidate's PNG output and compares pixels against the `Balanced`
  baseline's decode:

  ```
  Fast          : decoded pixels identical to Balanced = true
  Fastest       : decoded pixels identical to Balanced = true
  NoCompression : decoded pixels identical to Balanced = true
  High          : decoded pixels identical to Balanced = true
  ```

- **Is this a breaking change?** no pixel-level change; output PNG file size increases ~41%.
- Project test suite: `stream_jpx_1/3/5/6` (exact pixel diff) all 4 pass, unchanged. The same
  268-test isolation set used for the `take_unpremultiplied` fix: 268/268 pass, 0 regressions, 0
  new passes - exactly as expected for a lossless change.

## Why the result looks the way it does

`fdeflate` is purpose-built for PNG's specific DEFLATE usage pattern and trades some compression
ratio for a large constant-factor speedup; since PNG DEFLATE was the single largest phase of this
pipeline for `img.pdf` (59.5% of total time), a large win on that one phase produces a large
end-to-end win even without touching anything else.

## What's not done / open questions

- Did not additionally apply `Fastest` - `Fast` already captures the vast majority of the win
  (8.2x of the encode phase) with a more moderate file size increase. If file size matters less
  than speed for a given use case, `Fastest` is a one-line change away (see table above for its
  numbers) and is exactly as risk-free (same lossless guarantee).
- Did not investigate exposing this as a user-configurable setting in hayro's own API (e.g. a
  `PngQuality` knob) - this fix just changes vello's fixed default. If different callers want
  different tradeoffs, that's a separate, larger API design question.
