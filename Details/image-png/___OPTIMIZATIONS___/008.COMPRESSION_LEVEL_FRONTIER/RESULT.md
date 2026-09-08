# Where does the default compression level (`Level(6)`) sit on the size/time frontier? (measurement only, no patch)

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

The crate's default is `Compression::Balanced` -> `DeflateCompression::Level(6)` (`flate2`'s zlib
binding). A single-image spot check in an earlier session (`kodim02.png`: Level3 26.8ms/645KB,
Level6 69.0ms/631KB, Level9 97.5ms/629KB) suggested Level6 costs 2.6x Level3 for only 2.2% smaller
output - this candidate widens that to the **whole real corpus** (all 15 files in `tests/benches/`)
to check whether that single image was representative, per IDEAS.md's own instruction not to trust
two images as a verdict.

## What was tried

`harness/encoder_bench --mode compress --input <file>` for every file in `tests/benches/`
(`taskset -c 0`, 8 iterations each - single-shot per file, not paired; see "What's not done" below
for why that's an acceptable tradeoff here). Extracted `Level1`/`Level3`/`Level6`/`Level9` p50 time
and output size for each.

| File | L1 ms | L1 B | L3 ms | L3 B | L6 ms (default) | L6 B | L9 ms | L9 B | L6/L3 time | L3->L6 size saved | L6->L9 size saved |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Exoplanet_Phase_Curve_(Diagram).png | 17.41 | 980843 | 53.95 | 811520 | 89.20 | 792296 | 225.87 | 792272 | 1.65x | 2.37% | 0.00% |
| Exoplanet...indexed_gimp.png | 9.43 | 632606 | 26.24 | 541706 | 50.28 | 527715 | 161.31 | 526547 | 1.92x | 2.58% | 0.22% |
| Fantasy_Digital_Painting.png | 34.22 | 2910907 | 97.25 | 2486547 | 384.19 | 2392735 | 1208.77 | 2331936 | 3.95x | 3.77% | 2.54% |
| Lohengrin...png (grayscale) | 49.89 | 8734622 | 253.27 | 8787459 | 442.15 | 8740948 | 858.48 | 8721780 | 1.75x | 0.53% | 0.22% |
| Transparency.png | 0.25 | 5600 | 0.65 | 3465 | 0.73 | 3288 | 1.46 | 3359 | 1.13x | 5.11% | **-2.16%** |
| kodim02.png | 6.94 | 681335 | 22.62 | 645471 | 68.99 | 631220 | 97.34 | 628941 | 3.05x | 2.21% | 0.36% |
| kodim07.png | 7.11 | 656830 | 21.66 | 603593 | 76.04 | 584509 | 130.93 | 579026 | 3.51x | 3.16% | 0.94% |
| kodim17.png | 7.22 | 690501 | 22.61 | 639086 | 68.67 | 625639 | 103.59 | 622746 | 3.04x | 2.10% | 0.46% |
| kodim23.png | 7.03 | 635526 | 21.97 | 595409 | 81.89 | 578737 | 132.91 | 575007 | 3.73x | 2.80% | 0.64% |
| lorem_ipsum_oxipng.png (indexed) | 1.08 | 71636 | 2.20 | 58213 | 4.37 | 56365 | 13.53 | 56049 | 1.98x | 3.17% | 0.56% |
| lorem_ipsum_screenshot.png | 2.14 | 157137 | 5.92 | 126623 | 11.46 | 125313 | 36.95 | 124377 | 1.94x | 1.03% | 0.75% |
| paletted-zune.png (indexed) | 19.17 | 1813760 | 55.96 | 1635049 | 140.09 | 1594063 | 380.18 | 1591189 | 2.50x | 2.51% | 0.18% |
| tango-...-128-rsvg-convert.png | 0.24 | 14898 | 0.51 | 12887 | 1.04 | 12581 | 2.16 | 12442 | 2.05x | 2.37% | 1.10% |
| tango-...-16.png | 0.08 | 708 | 0.09 | 699 | 0.10 | 688 | 0.10 | 688 | 1.08x | 1.57% | 0.00% |
| tango-...-32.png | 0.09 | 2058 | 0.12 | 1974 | 0.14 | 1922 | 0.24 | 1912 | 1.24x | 2.63% | 0.52% |
| **TOTAL (summed)** | | | **585.0ms** | **16949701B** | **1419.3ms** | **16668019B** | **3353.8ms** | **16568271B** | **2.43x** | **1.66%** | **0.60%** |

## Result

**Corpus-wide, not just one image: Level6 (the default) costs 2.43x Level3's total time across all
15 real files, for 1.66% smaller total output.** Level9 costs a further 2.36x on top of Level6 for
only another 0.60%. The per-file `L6/L3 time` ratio ranges 1.08x-3.95x and the size saving 0.53%-
5.11% (excluding the one negative case below) - noisy at the individual-file level but the
aggregate confirms the single-image spot check from the earlier session was representative, not an
outlier.

**One genuine non-monotonicity worth flagging on its own:** `Transparency.png` compresses **worse**
at Level9 than at Level6 (3359B vs 3288B, +2.16%) - zlib's higher-level match-finding heuristics
are not strictly monotonic in output size for every input, particularly small/near-trivial ones.
Not investigated further (this is a `flate2`/zlib property, not something in this crate's control),
but worth knowing if anyone assumes "higher level always compresses at least as well."

## Does it change behavior/output?

- N/A - measurement only, no patch. `Compression`/`DeflateCompression` are already public,
  documented, user-selectable settings; nothing about this candidate changes the default or any
  behavior.
- Project test suite: not run - no source file was changed by this candidate.

## Why the result looks the way it does

zlib's compression levels trade CPU for match-finding thoroughness with steeply diminishing
returns past the middle of the range - this is a well-known general property of DEFLATE
implementations, not specific to this crate, and the corpus-wide numbers here are a direct,
reproducible confirmation of it for this crate's specific default choice.

## What's not done / open questions

- Single-shot per file (not paired/alternating) - given this candidate's own conclusion is about a
  2-4x order-of-magnitude difference between levels, not a few-percent effect, the paired-run
  discipline that mattered so much for candidates 001/004 (subtle, noise-floor-sized deltas) is not
  needed here; the size deltas are exact (deterministic compression output) regardless.
- This is a measurement, not a recommendation - whether the crate's default should change is a
  product decision involving factors this candidate didn't measure (what typical callers actually
  need, backward-compatibility expectations for existing output size guarantees, etc.). The data
  here is what a maintainer would need to make that call, not a suggestion to make it.
- Only default/common settings (`Filter::Adaptive`, `photo`-pattern via the real corpus) were
  exercised - `Filter::Fastest`/`NoFilter` combined with different compression levels was not swept,
  since filter choice already has its own dedicated backlog measurements.
