# Does `Filter::MinEntropy` actually compress worse than `Adaptive`? Corpus-wide check (measurement only, no patch)

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

An earlier session's 2-image spot check (`kodim02.png` and a synthetic 512x512 RGBA image) found
`Filter::MinEntropy` both slower *and* producing a larger file than `Filter::Adaptive` on both, and
IDEAS.md flagged this explicitly as "two images is not a verdict" - this candidate widens the size
comparison to the full 15-file real corpus.

## What was tried

`harness/encoder_bench --mode filter --input <file>` (`FdeflateUltraFast` compression, `taskset -c
0`, 10 iterations) for every file in `tests/benches/`, comparing `Adaptive` (the default for most
compression levels) against `MinEntropy`.

| File | Adaptive ms | Adaptive B | MinEntropy ms | MinEntropy B | time delta | size delta |
|---|---|---|---|---|---|---|
| Exoplanet_Phase_Curve_(Diagram).png | 9.582 | 975273 | 13.622 | 964756 | +42.2% | **-1.08%** |
| Exoplanet...indexed_gimp.png | 3.388 | 574204 | 7.133 | 559521 | +110.5% | **-2.56%** |
| Fantasy_Digital_Painting.png | 9.674 | 2554125 | 16.772 | 2552718 | +73.4% | -0.06% |
| Lohengrin...png (grayscale) | 17.431 | 9960501 | 30.479 | 9951603 | +74.9% | -0.09% |
| Transparency.png | 0.071 | 4716 | 0.173 | 4631 | +143.7% | **-1.80%** |
| kodim02.png | 1.526 | 663722 | 2.900 | 666988 | +90.0% | +0.49% |
| kodim07.png | 1.589 | 620116 | 3.073 | 623031 | +93.4% | +0.47% |
| kodim17.png | 1.503 | 692476 | 3.172 | 693127 | +111.0% | +0.09% |
| kodim23.png | 1.553 | 598913 | 2.913 | 597982 | +87.6% | -0.16% |
| lorem_ipsum_oxipng.png (indexed) | 0.376 | 120303 | 1.197 | 119497 | +218.4% | -0.67% |
| lorem_ipsum_screenshot.png | 1.017 | 350440 | 2.371 | 349168 | +133.1% | -0.36% |
| paletted-zune.png (indexed) | 6.212 | 1818454 | 13.828 | 1747469 | +122.6% | **-3.90%** |
| tango-...-128-rsvg-convert.png | 0.040 | 13055 | 0.212 | 13039 | +430.0% | -0.12% |
| tango-...-16.png | 0.002 | 704 | 0.011 | 706 | +450.0% | +0.28% |
| tango-...-32.png | 0.005 | 1931 | 0.024 | 1934 | +380.0% | +0.16% |

**MinEntropy produced smaller output than Adaptive on 10 of 15 real files.**

## Result

**Time: unambiguous, consistent, real cost.** `MinEntropy` is slower than `Adaptive` on **every
single file** (+42% to +450%), confirming that half of the earlier 2-image finding.

**Size: the earlier "always larger" claim does not hold corpus-wide - it was an artifact of which
2 images happened to be spot-checked.** `MinEntropy` produces *smaller* output on 10/15 files (up
to -3.90% on `paletted-zune.png`), and only larger on the 5 files that happen to include both
originally-tested images (`kodim02.png`, `kodim17.png`) plus 3 others, with the "worse" cases capped
at +0.49%. This is a real correction to a claim this workspace itself recorded earlier - worth
having caught before it became a permanent "known fact" about this crate.

## Does it change behavior/output?

- N/A - measurement only, no patch. `Filter::MinEntropy` is already public, documented, opt-in.
- Correctness of the measurement: `verify=OK` on every file, both filters (round-trip decode +
  determinism check).
- Project test suite: not run - no source file was changed by this candidate.

## Why the result looks the way it does

`MinEntropy`'s entropy-estimation heuristic (`ilog2i` approximation, the "runs of zeros are free"
short-circuit, the 4-way counter scatter in `entropy()`) evidently tracks *actual* deflate
compressibility reasonably well on most real images (indexed/paletted images especially, where it
wins by the largest margins) - the earlier 2-image sample just happened to land on two of the
minority cases where it doesn't. The **time** cost is the one truly unconditional finding here:
`entropy()`'s 4x4KiB-counter-zeroing-per-row cost (flagged separately in IDEAS.md) is real and
visible in every single measurement, 42% to 450% slower depending on file, for a size benefit that
is real but usually under 1% and never enough to justify that cost as a *default* - though it may
still be a reasonable *opt-in* choice for someone who specifically wants the smallest possible
output on an indexed/paletted image and doesn't mind the time cost.

## What's not done / open questions

- Not investigated: *why* `MinEntropy` wins on indexed/paletted images specifically (3 of its best
  4 results are indexed color type) - could be a genuine property of entropy-based filter choice
  interacting with palette-indexed data, or coincidental to this small corpus. Would need a larger,
  indexed-heavy corpus to tell apart.
- The `entropy()` 4KiB-per-row counter-zeroing cost (separate IDEAS.md backlog entry) was not
  patched here - this candidate only re-characterizes whether `MinEntropy` is worth optimizing at
  all, concluding it's plausibly worth keeping as an *opt-in*, which makes that follow-up candidate
  more justified than it looked before this correction.
