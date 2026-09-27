# Cross-CN verification of candidate 014's AVX-512VNNI vertical/column dispatch branch (CN=2/3/4)

Candidate 014 added an `avx512vnni`+`avx512vl`-gated dispatch branch to
`src/factory/plane_u16.rs`'s `default_u16_column_plan` (bit_depth<=12 vertical/column
convolution) and measured a real, repeatable win via `resize_plane`'s `"Pic scale Plane10:
Lanczos 3"` case - but `default_u16_column_plan` is a `const CN: usize` generic shared by
`plane_u16.rs` (CN=1), `cbcr16.rs` (CN=2), `rgb_u16.rs` (CN=3) and `rgba_u16.rs` (CN=4), so the
same dispatch branch already benefits all four the moment the patch is applied - only CN=1 had
actually been measured. `IDEAS.md`'s backlog (row dated 2026-08-25) flagged this as untested and
named the specific bench cases to check. This candidate reuses candidate 014's `patch.diff`
byte-for-byte (confirmed via `diff` against `014.AVX512VNNI_PLANE_U16_VERTICAL_LB/patch.diff` -
identical) and only adds measurements against the CN=2/3/4 bench cases.

**Deviation from the backlog item's named case:** the backlog item named `"Pic scale CbCr16:
Lanczos 3"` for the CN=2 case, but reading `app/benches/resize_cbcr/main.rs` shows that case uses
`bit_depth=16`, not `bit_depth<=12` - and `src/factory/plane_u16.rs`'s `default_u16_column_plan`
only reaches the `avxvnni`/`avx512vnni`-gated LB (low-bit-depth) branch this candidate touches
when `options.bit_depth <= 12` (line 191: `if options.bit_depth > 12 { ...completely different HB
path... } else { ...LB path with the dispatch branch... }`). `"Pic scale CbCr16: Lanczos 3"`
therefore never executes the patched code at all - any paired measurement against it would show
noise only, not a real null result. `"Pic scale CbCr10: Lanczos 3"` (bit_depth=10, same file) does
reach the LB path and is the direct CN=2 analogue of what candidate 014 measured for CN=1, so it
was used instead. `"Pic scale RGB10: Lanczos 3"` and `"Pic scale RGBA10 without alpha: Lanczos 3"`
(both bit_depth=10) were used as named, unchanged.

## What was tried

No new source change - candidate 014's exact patch, applied and measured against three more bench
cases. Paired via `___OPTIMIZATIONS___/tools/paired.sh` (alternating A/B, 9 pairs each, `RUNS=9`
default, run from `app/` so the bench's relative asset path resolves). A = baseline (patched
`app/Cargo.toml` only, so the pre-existing AVX2 `vpmaddwd`+`vpaddd` fallback ships, same as
today), B = patched (candidate 014's new `avx512vnni`+`avx512vl` branch active, EVEX `vpdpwssd`).
Pinned runs used `taskset -c 5` (core 5, not core 3 - candidate 015 found core 3 specifically noisy
for this workspace's benches and switching fixed it immediately for a `resize_plane` case; used
proactively here rather than discovering the same issue again).

| Variant | pinned (`taskset -c 5`) | unpinned | notes |
|---|---|---|---|
| baseline (AVX2 fallback) vs. patched (avx512vnni+avx512vl), **CbCr10 (CN=2)** | run1: 15.329 -> 14.983 ms (-2.3%, B faster 9/9, consistent) | run1: 14.724 -> 14.183 ms (-3.7%, B faster 7/9, NOT consistent); run2: 13.808 -> 13.402 ms (-2.9%, B faster 8/9, consistent) | pinned only run once (already consistent 9/9); unpinned needed a 2nd repeat to firm up |
| baseline vs. patched, **RGB10 (CN=3)** | run1: 13.885 -> 13.642 ms (-1.8%, B faster 7/9, NOT consistent); run2: 13.977 -> 13.422 ms (-4.0%, B faster 9/9, consistent) | run1: 14.028 -> 13.420 ms (-4.3%, B faster 9/9, consistent) | pinned needed a 2nd repeat to firm up; unpinned consistent on the first try |
| baseline vs. patched, **RGBA10 without alpha (CN=4)** | run1: 16.535 -> 15.316 ms (-7.4%, B faster 7/9, NOT consistent); run2: 19.672 -> 19.496 ms (-0.9%, B faster 6/9, NOT consistent, visibly bimodal ~15-20ms/~24-34ms clustering in both A and B) | run1: 17.054 -> 15.938 ms (-6.5%, B faster 9/9, consistent) | pinned run2 landed on the bimodal `resize_rgb`/`resize_rgba` host-noise pattern candidates 011/012 already documented for `taskset`-pinned runs on this box; unpinned consistent on the first try |

Raw pairs (ms), exactly as captured from `paired.sh`'s stdout during this session. Several runs
were re-read from the output file via `tail -5`/`tail -8`, which truncated the `A:` line for most
of them - only the `B:` array survived truncation in those cases (noted per line below); the
median/verdict lines are always complete since they're the tool's own final output line. Also
saved verbatim in `raw_pairs.txt` in this folder.

- CbCr10 pinned run1: `B: 16.562 16.919 16.508 14.991 14.983 14.109 14.51 14.437 13.738` -
  `median 15.329 -> 14.983 (-2.3%) B faster in 9 of 9 pairs` (A line not retained)
- CbCr10 unpinned run1: `B: 14.461 14.21 14.183 20.331 17.481 13.917 13.489 13.384 13.413` -
  `median 14.724 -> 14.183 (-3.7%) B faster in 7 of 9 pairs` (A line not retained)
- CbCr10 unpinned run2: `B: 21.085 13.47 13.433 13.388 13.402 13.393 13.339 17.892 13.396` -
  `median 13.808 -> 13.402 (-2.9%) B faster in 8 of 9 pairs` (A line not retained)
- RGB10 pinned run1: `B: 13.008 13.301 13.307 13.234 14.528 15.507 13.754 13.642 13.727` -
  `median 13.885 -> 13.642 (-1.8%) B faster in 7 of 9 pairs` (A line not retained)
- RGB10 pinned run2: `B: 15.51 13.589 13.373 13.422 13.391 13.358 13.369 26.81 27.959` -
  `median 13.977 -> 13.422 (-4.0%) B faster in 9 of 9 pairs` (A line not retained)
- RGB10 unpinned run1: `B: 13.595 13.42 13.581 13.349 13.411 13.321 13.513 13.59 13.398` -
  `median 14.028 -> 13.420 (-4.3%) B faster in 9 of 9 pairs` (A line not retained)
- RGBA10 pinned run1: `B: 15.225 15.421 19.145 19.524 17.862 15.316 15.308 15.296 15.262` -
  `median 16.535 -> 15.316 (-7.4%) B faster in 7 of 9 pairs` (A line not retained)
- RGBA10 unpinned run1: `B: 17.524 17.63 15.508 16.494 16.285 15.661 15.591 15.938 15.06` -
  `median 17.054 -> 15.938 (-6.5%) B faster in 9 of 9 pairs` (A line not retained)
- RGBA10 pinned run2: `A: 32.187 31.813 19.672 33.764 17.26 16.364 28.248 16.82 16.202` /
  `B: 31.617 30.981 31.487 18.206 24.469 19.496 15.279 15.604 15.403` -
  `median 19.672 -> 19.496 (-0.9%) B faster in 6 of 9 pairs` (only run where both arrays were
  retained - the visible bimodal clustering in both A and B is genuine, not a truncation artifact)

## Result

All 8 individual repeats (across the three CN values, pinned+unpinned) leaned the same direction
as candidate 014's original CN=1 result: patched faster every single time, no repeat showing a
regression. Magnitudes:

- **CbCr10 (CN=2):** -2.3% to -3.7% (pinned consistent 9/9 on first try; unpinned needed a repeat
  to reach 8/9) - smaller than candidate 014's CN=1 median (~-7.5%) but in the same direction and
  more consistent than RGB10/RGBA10 below.
- **RGB10 (CN=3):** -1.8% to -4.3% - smallest magnitude of the three, and the only case where
  *both* the pinned and unpinned first-try runs needed a second repeat (unpinned's first try was
  already 9/9, pinned's first try was not) to land on a clearly consistent (8-9/9) result.
- **RGBA10 without alpha (CN=4):** -6.5% to -7.4% on the two non-bimodal-noise runs (unpinned
  9/9, pinned run1 7/9) - closest in magnitude to candidate 014's CN=1 result - but pinned run2
  landed squarely on this workspace's documented `resize_rgb`/`resize_rgba` bimodal host-noise
  pattern (candidates 011/012), reducing the measured effect to a noise-dominated -0.9%/6-of-9 in
  that one run.

So: the win **generalizes in direction** to all three untested CN values, but **not uniformly in
magnitude** - CbCr10/RGB10 (CN=2/3) show a smaller, less consistently-reaching-the-bar effect
(~-2% to -4%, several repeats landing at 7-8/9 rather than 9/9) than either candidate 014's CN=1
result or this candidate's own CN=4 result (both ~-6% to -9%). This is consistent with the
backlog item's own hypothesis that CN=2/3's chunk-boundary remainder-loop balance (not dividing
evenly into the kernel's 32/16/8/4-wide chunks the way CN=1 and, incidentally, CN=4 do) changes
how much of each output row runs through the patched dot-product path versus the unpatched
scalar/narrower-SIMD remainder tail.

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone correctness-check crate (`bit_exact_check/` in this folder)
  resizes the same `nasa-4928x3279-rgba.png` asset with the three transforms above (CbCr10/CN=2,
  RGB10/CN=3, RGBA10-without-alpha/CN=4), built once against baseline source and once against
  patched source. Hashes matched exactly in both builds:
  - CbCr10: `len=2018016 hash=b2f46c7323e0ca6a`
  - RGB10: `len=3027024 hash=633b7f995986e40f`
  - RGBA10_no_alpha: `len=4036032 hash=d9c2523d1b23e3d0`
- **Is this a breaking change?** No public API change either way (same conclusion as candidate
  014 - this candidate applies the identical patch).
- Project test suite (`cargo test --release`, no avx512 feature - patch's new code is entirely
  behind `#[cfg(feature = "avx512")]` so this run doesn't compile it at all): baseline 66 passed /
  0 failed (lib) + 0 passed / 20 ignored (doctests); patched: identical, 66 passed / 0 failed +
  0 passed / 20 ignored.
- Project test suite (`cargo test --release --features avx512`, exercises the new cfg-gated code
  path if any existing test happens to touch bit_depth<=12 u16 vertical convolution): baseline
  68 passed / 0 failed (lib) + 0 passed / 20 ignored (doctests); patched: identical, 68 passed /
  0 failed + 0 passed / 20 ignored.

## Why the result looks the way it does

Candidate 014's own explanation (this kernel does no shuffle at all, so removing one instruction
from the per-tap accumulate is a proportionally larger share of the loop body than it was for the
shuffle-heavy horizontal kernels) applies identically here since it's the literal same kernel code
- what differs across CN is only how the per-row pixel count (`width * CN` output elements per
row, chunked into 32/16/8/4-wide SIMD groups by `convolve_column_lb_avx_u16_impl512`) lines up with
those chunk boundaries. CN=1 (candidate 014) and CN=4 (this candidate) both showed the larger,
more consistent effect (~-6% to -9%); CN=2/3 (also this candidate) showed a smaller, less
consistent one (~-2% to -4%). A CN=4-pixel row is already a multiple of 4 (and usually of 32 for
this image's 4928px width, since 4928*4 = 19712 = 616*32 exactly), so it likely runs almost
entirely through the widest 32-lane path; CN=2/3 rows (4928*2=9856=308*32 exactly for CN=2, but
4928*3=14784=462*32 exactly for CN=3 too - both also divide evenly by 32 for *this specific image
width*) don't obviously explain the smaller effect by chunk-boundary misalignment alone for this
particular image size. The more likely explanation, not confirmed with `perf stat` here, is
memory-access-pattern differences: CN=2/3's column stride per tap (`src_stride` bytes apart,
`src_stride` scaling with the row's element count) changes cache-line utilization per SIMD load
differently across CN, changing how much of the total per-tap cost the accumulate instruction
(the one thing this patch touches) is actually responsible for.

## What's not done / open questions

- Not confirmed with `perf stat` (instruction-count / cache-miss comparison) which of "chunk
  alignment" vs. "memory-access-pattern/cache-line utilization" actually explains the CN=2/3 vs.
  CN=1/4 magnitude gap - noted as a real open question above rather than settled. This is the
  same open item candidates 001/012/013/014 already left for a future session.
- RGBA10 pinned run2's bimodal host-noise episode was not chased further (e.g. a 3rd repeat, or
  switching to yet another pinned core) since the other 5 of 6 pinned+unpinned runs across the
  three CN values already established the direction and rough magnitude range without it, and
  this exact noise pattern is already a known, previously-documented host quirk (candidates
  011/012) rather than something specific to this candidate.
- Not tested on real `avxvnni` hardware (Alder Lake+/Sierra Forest) - same caveat as every other
  `avx512vnni`-vs-`avxvnni` candidate in this workspace.
