# Is `StreamWriter`'s 4 KiB default buffer size reasonable? Corpus-wide check (measurement only, no patch)

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

Issue #193 asks why `StreamWriter`'s buffer (`DEFAULT_BUFFER_LENGTH = 4 * 1024`,
`src/encoder.rs:529`) differs from stdlib's 8 KiB `BufWriter` recommendation. An earlier session
measured a 1 KiB vs 64 KiB comparison on one image (768x512) and found the small buffer costs
+1.2% output size for ~11% less wall time relative to the buffered (non-streaming) path - but never
included the actual 4 KiB *default* in that comparison, and only checked one file. This candidate
closes both gaps: sweeps `{1, 4, 8, 32, 64} KiB` (`harness/encoder_bench --mode stream` already
supports this) across the full 15-file real corpus.

## What was tried

`harness/encoder_bench --mode stream --input <file>` (`taskset -c 0`, 12 iterations) for every file
in `tests/benches/`.

| File | buf1KiB B | buf4KiB B (default) | buf8KiB B | buf32KiB B | buf64KiB B |
|---|---|---|---|---|---|
| Exoplanet_Phase_Curve_(Diagram).png | 986154 | 977586 | 976158 | 975090 | 974910 |
| Exoplanet...indexed_gimp.png | 580394 | 575354 | 574514 | 573890 | 573782 |
| Fantasy_Digital_Painting.png | 2584036 | 2561584 | 2557840 | 2555032 | 2554564 |
| Lohengrin...png (grayscale) | 10076411 | 9988871 | 9974279 | 9963335 | 9961511 |
| Transparency.png | 4689 | 4653 | 4641 | 4641 | 4641 |
| kodim02.png | 671453 | 665621 | 664649 | 663917 | 663797 |
| kodim07.png | 627316 | 621868 | 620956 | 620272 | 620164 |
| kodim17.png | 700569 | 694485 | 693465 | 692709 | 692577 |
| kodim23.png | 605911 | 600655 | 599779 | 599119 | 599011 |
| lorem_ipsum_oxipng.png (indexed) | 122602 | 121546 | 121366 | 121234 | 121210 |
| lorem_ipsum_screenshot.png | 354413 | 351329 | 350813 | 350429 | 350369 |
| paletted-zune.png (indexed) | 1839621 | 1823649 | 1820985 | 1818993 | 1818657 |
| tango-...-128-rsvg-convert.png | 13180 | 13072 | 13048 | 13036 | 13036 |
| tango-...-16.png | 702 | 702 | 702 | 702 | 702 |
| tango-...-32.png | 1942 | 1930 | 1930 | 1930 | 1930 |
| **TOTAL** | **19169393** | **19002905** | **18975125** | **18954329** | **18950861** |
| **vs 64 KiB** | **+1.15%** | **+0.27%** | **+0.13%** | **+0.02%** | **0.00%** |
| **Total time** | 66.75ms | 66.15ms | 65.96ms | 64.15ms | 63.97ms |

## Result

**The current 4 KiB default is a reasonable choice, not an outlier needing fixing.** Corpus-wide,
it costs only +0.27% output size versus the largest buffer tested (64 KiB) - a smooth, rapidly
diminishing-returns curve (1 KiB: +1.15%, 4 KiB: +0.27%, 8 KiB: +0.13%, 32 KiB: +0.02%). Wall time
is nearly flat across the whole range (63.97-66.75ms total, a ~4% spread with the smaller buffers
consistently slightly slower - more IDAT-chunk-boundary overhead, as expected). This directly
answers issue #193's question with real numbers: 4 KiB isn't obviously wrong, it sits close to the
knee of the curve already; 8 KiB (stdlib's own recommendation) would be a marginal, not dramatic,
improvement (+0.13% instead of +0.27% - roughly half the gap to optimal).

## Does it change behavior/output?

- N/A - measurement only, no patch. `StreamWriter`'s buffer size is already a public,
  user-selectable parameter (`stream_writer_with_size`); only the *default* value is in question,
  and this candidate doesn't change it.
- Correctness of the measurement: `verify=OK` on every file, every buffer size.
- Project test suite: not run - no source file was changed by this candidate.

## Why the result looks the way it does

A larger `StreamWriter` buffer means fewer, larger IDAT chunks, and fewer chunk-boundary flushes
that interrupt the deflate stream mid-compression (each boundary is a small compression
inefficiency, not just chunking overhead) - the curve shape (rapid initial improvement, fast
diminishing returns) is exactly what that mechanism predicts, and it's consistent across every file
in the corpus regardless of size or color type.

## What's not done / open questions

- This is descriptive, not prescriptive - whether to actually change `DEFAULT_BUFFER_LENGTH` from
  4 KiB to 8 KiB (matching stdlib's `BufWriter`) is a maintainer call weighing a ~0.14 percentage
  point average size improvement against any downstream memory-footprint preference for keeping it
  small; this candidate provides the numbers, not the recommendation.
- Did not test buffer sizes below 1 KiB or above 64 KiB - the earlier single-image data point
  suggested the curve is already flat by 64 KiB, and this corpus-wide sweep confirms that (32 KiB
  and 64 KiB differ by only 0.02 percentage points), so extending the range further seemed unlikely
  to add information.
