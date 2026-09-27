# Optimization results

**Impact** (derived from the Result column, for scanning at a glance - not a recommendation):
**Strong win** (large, confirmed improvement) - **Small win** (real but modest improvement) -
**No change** (null result / within noise) - **Mixed** (improves one thing, regresses another) -
**Small/Strong regression** (would make things worse) - **N/A** (pure measurement/characterization).

| # | Impact | Candidate | Result | Changes behavior? | Link |
|---|---|---|---|---|---|
| 003 | Small win | PNG DEFLATE compression level (Fast to Fastest) | -20.7% isolated vs. Fast (own fresh baseline pair); -88.5% isolated vs. current Balanced default | no, bit-exact (lossless) | [003.png_compression_level_fastest/RESULT.md](003.png_compression_level_fastest/RESULT.md) |
| 002 | Strong win | PNG DEFLATE compression level (Balanced to Fast) | -87.8% isolated, -48.6% end-to-end (real img.pdf) | no, bit-exact (lossless) | [002.png_compression_level/RESULT.md](002.png_compression_level/RESULT.md) |
| 001 | Small win | Pixmap::take_unpremultiplied (float division to integer LUT) | -25.4% isolated; within noise end-to-end for hayro's 3 test PDFs | yes - see RESULT.md (up to 1 LSB on ~0.5% of component,alpha pairs) | [001.take_unpremultiplied_lut/RESULT.md](001.take_unpremultiplied_lut/RESULT.md) |

See ../../hayro/___OPTIMIZATIONS___/SUMMARY.md for candidates from the hayro side of this audit
that wire these fixes in, including the combined end-to-end measurement.
