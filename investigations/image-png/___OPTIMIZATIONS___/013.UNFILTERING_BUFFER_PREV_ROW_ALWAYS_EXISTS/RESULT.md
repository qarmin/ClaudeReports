# UnfilteringBuffer: pre-reserve prev row so it always exists (issue #661)

Implements anforowicz's suggestion in [issue #661](https://github.com/image-rs/image-png/issues/661)
(originally posted on PR #640): rather than `PrevRow::None` as a sentinel that every consumer of
`UnfilteringBuffer` must special-case for the first row of a frame/Adam7 pass, always keep a real,
zero-filled "previous row" buffer around, so `prev_row()` always returns actual data and the
`None` branch disappears from the hot per-row path. Toolchain: `rustc 1.98.0`, `cargo 1.98.0`,
AMD Ryzen 9 7900 (24 threads), Linux.

## What was tried

The literal patch (`patch.diff`): `UnfilteringBuffer::reset_prev_row` now takes the upcoming row's
length and always sets `prev_row = PrevRow::Scratch(<zero-filled Vec of that length>)` instead of
`PrevRow::None`, reusing whichever backing allocation is already available. `start_frame` was
changed the same way (it previously set `PrevRow::None` directly) because `Reader::next_frame`'s
non-interlaced bulk path calls `next_interlaced_row_impl` directly, bypassing `read_row`'s own
`reset_prev_row` call entirely for row 0 - an early version of this patch only patched the
`read_row` call site and consequently measured **no** effect at all on `next_frame`, because it
never touched the code path that API actually uses for row 0 of a non-interlaced image. `PrevRow`
still has three variants (`None`/`InPlace`/`Scratch`) rather than being collapsed to two, since
`None` is still the struct's initial value before any frame starts; per issue #661 itself this is
purely a code-simplification/robustness motivation (removing a case every future maintainer of
this file has to reason about), not a performance one - see "Why the result looks the way it does".

No project benchmark reaches `UnfilteringBuffer` at a controlled row-width/height/filter
combination, so a standalone bench (`bench/`, `png = { path = "../clone", features =
["benchmarks"] }`) was built with three modes: `single` (full `next_frame` decode of a synthetic
image at given width/height/filter, repeated, hash-verified), `multi` (N threads looping the same
decode for an aggregate throughput number), and `row_only` (calls `png::benchable_apis::unfilter`
directly with `previous = &[]` vs a real zero-filled slice of the same length, isolating exactly
the mechanism issue #661's own maintainer thread discussed, with no decode/decompression plumbing
in the loop).

**`row_only` microbenchmark** (200,000 iters/cell, `taskset -c 0`, row_bytes=4096, ns/call):

| filter | bpp | `previous=&[]` (baseline path) p50 | real zero-filled `previous` (patched path) p50 | ratio |
|---|---|---|---|---|
| Paeth | 1 | 811 ns | 5360 ns | 6.6x |
| Paeth | 2 | 751 ns | 4779 ns | 6.4x |
| Paeth | 3 | 651 ns | 3346-3367 ns | 5.1-5.2x |
| Paeth | 4 | 220-250 ns | 2224-2234 ns | ~9-10x |
| Paeth | 6 | 281 ns | 1924-1934 ns | ~6.9x |
| Paeth | 8 | 120-130 ns | 1103-1112 ns | ~8.6-9.2x |
| Up | any bpp | 20 ns | 90 ns | 4.5x (tiny absolute cost either way) |
| Avg | 1-8 | 331-1533 ns | 471-3045 ns | ~1.6-2.0x |
| Sub | 1-8 | unchanged (Sub never reads `previous.is_empty()`) | unchanged | 1.0x |

**End-to-end `single` mode**, synthetic RGBA8 image, `BENCH_WIDTH=8192` (row_bytes=32768),
`Filter::Paeth` forced for the whole image, `taskset -c 0`, single-core, p50 of 200 iters (10%
warmup discarded), paired alternating baseline/patched runs (`RUNS` column = how many of N
alternating pairs baseline was faster):

| height | baseline p50 | patched p50 | delta | paired consistency |
|---|---|---|---|---|
| 2 | 132-176 us (run-to-run) | 148-186 us | **baseline faster 9/9** | RUNS=9 |
| 4 | 267-334 us | 338-426 us | **baseline faster 9/9** | RUNS=9 |
| 8 | 531-633 us | 633-640 us | +2-20% (noisier) | not separately paired |
| 16 | 1075-1154 us | 1100-1141 us | **baseline faster 8/9** | RUNS=9 |
| 32 | 2151-2179 us | 2179-2208 us | +1.3-1.4% | single comparison |
| 128 | 8669-8779 us | 8735-8820 us | +0.5-0.9% | single comparison |
| 1024 | 70906-71747 us | 71234-71826 us | +0.05-0.5% (noise floor) | single comparison |

Also checked, same forced-filter/width/height=4 methodology (7 alternating pairs each): `Up`
baseline faster 7/7 (221-224us vs 278-282us, +26%); `Avg` baseline faster 6/7 (253-319us vs
312-388us, +23%); `Sub` baseline faster 7/7 (254-257us vs 310-313us, +22%) - **Sub is unaffected by
the filter-specific mechanism above**, so this residual ~+22% at H=4/W=8192 is a *second*,
filter-independent mechanism: `reset_prev_row`/`start_frame` now `Vec::resize(row_len, 0)` a full
32768-byte row once per frame (a real memset), where the baseline's `PrevRow::None` path pays
nothing at that point - this cost is incurred once per decoded frame regardless of filter choice.

**Real-world corpus files** (actual `Adaptive`-selected filters, real content, `taskset -c 0`,
3000 iters, min/p50 in ns): `tango-icon-address-book-new-16.png` (16x16, real row width) baseline
p50=5110ns vs patched p50=5189ns (+1.5%); `tango-icon-address-book-new-32.png` (32x32) baseline
p50=9048ns vs patched p50=9088ns (+0.4%); `kodim02.png` (768x512) baseline p50=2,030,556ns vs
patched p50=2,032,109ns (+0.08%) - all within noise. These have short-to-moderate height but
*narrow* rows, so the absolute per-row cost difference (which scales with row width, per the
`row_only` table above) is too small to see above the timer's own noise floor.

**Multi-core** (`multi` mode, unpinned, 12 threads x 3s, `BENCH_WIDTH=8192 height=4 Paeth`,
decodes/sec, 3 repeated trials): baseline 40961/40019/41136 vs patched 38611/38692/38683 - baseline
faster in 3/3 trials, ~-5.7% to -6.6% aggregate throughput.

## Result

**Time.** A real, reproducible regression exists, but it is conditional on image shape, not
universal: for a **wide-row, short-height** image (row_bytes=32768, i.e. width=8192 RGBA8; height
2-16) using `Filter::Paeth`, `Up`, or `Avg` for row 0, single-core wall time is measurably worse
(paired 9/9, 9/9, 8/9 baseline-faster at heights 2/4/16 respectively), by roughly +10% to +32% of
total single-decode time at height=2-4, shrinking to +1-2% by height=16-32 and to noise-floor by
height>=128. Multi-core throughput at the worst-case shape measured -5.7% to -6.6%, consistent
across 3 trials. On real corpus files (narrow rows, even when short: 16x16, 32x32) and on tall
images regardless of width, the effect is within measurement noise (<2%). No peak-RSS/allocation
measurement was done (the change does not add or remove any allocation on the already-established
hot path - `reset_prev_row`/`stash_scratch_buf_for_reuse` reuse the same buffer either way - the
mechanism here is CPU time, not memory).

## Does it change behavior/output?

- **Bit-exact?** Yes. Every `single`/`row_only`/real-corpus comparison above asserted the decoded
  hash was identical between baseline and patched builds (and across iterations); this is expected
  since `unfilter()`'s existing `previous.is_empty()` special case (Sub-for-Paeth,
  NoFilter-for-Up) is a pure micro-optimization of the *same* arithmetic when the true previous row
  is all zero - not a different rounding rule.
- **Is this a breaking change?** No public API changes. `UnfilteringBuffer` and its methods are
  all `pub(crate)`.
- Project test suite: **107 passed, 1 ignored, 0 failed**, both before and after
  (`cargo test --release`: 91+1+4+3+8 = 107 across the lib/doctests/`tests/bugfixes.rs`/
  `tests/check_testimages.rs`/`tests/partial_decode.rs`), identical counts both builds.

## Why the result looks the way it does

This matches, with real numbers, exactly the tradeoff the maintainer (fintelia) raised in the
issue #661 discussion before this candidate was ever run: "Special-casing the first row is meant
as a performance optimization" (Paeth-with-zero-previous is mathematically identical to Sub, and
Up-with-zero-previous to NoFilter, so the special case skips reading/writing a same-sized "previous"
array for no different output) - and separately flagged the exact same zero-fill cost this
candidate also found ("we don't have to fill an entire row of bytes with zeros"). The `row_only`
microbenchmark isolates why Paeth is hit hardest (6-10x): the full Paeth predictor computes three
candidate differences and picks a min per byte, which both does more work per byte *and* is far
less friendly to auto-vectorization than Sub's straight-line carry chain, even though the extra
work is provably adding zero. `Up`'s 4.5x ratio is on a tiny absolute base (20ns vs 90ns for a
whole 4096-byte row) because `NoFilter` is a literal no-op while `Up`-with-real-zero still walks
and writes the row. `Avg`'s ~1.6-2x is more moderate because both branches of the existing code
already iterate the full row (the "empty" branch just divides a compile-time-known-zero `prev[i]`
by 2 instead of reading it). `Sub` is untouched because its `unfilter()` arm never inspects
`previous.is_empty()` at all - the ~+22% still seen for `Sub` at H=4/W=8192 is not from filter
substitution but from the added `Vec::resize(row_len, 0)` memset in `reset_prev_row`/`start_frame`,
paid once per frame regardless of which filter row 0 uses. Both mechanisms shrink relative to total
decode time as height grows (the affected row is always exactly 1 out of N, or 1 per Adam7 pass 8x
per frame if interlaced), which is why the corpus/tall-image numbers land in the noise floor.

## What's not done / open questions

- Not measured: the specific combination of *interlaced* (Adam7) images, where the "affected row"
  count is 7 per frame (once per pass) instead of 1 - the `read_row`/`reset_prev_row` call site is
  exercised correctly there (verified via the `next_interlaced_row_impl` code-path trace during
  development, see patch's `start_frame` doc comment), but no timing was collected specifically for
  that shape.
- Not measured: peak RSS / allocation count deltas - reasoned to be unchanged (no new allocations
  on the steady-state path) but not instrumented to confirm.
- The maintainer thread's own open question - "is unconditionally pre-allocating okay for really
  big `rowlen`s" (their corpus's 10 biggest widths are 2607-4501px, i.e. up to ~36KB rows at RGBA8)
  - is exactly what this candidate's worst-case measurement (row_bytes=32768) targets, and the
  answer found here is: yes, it costs something real and reproducible at that scale, concentrated
  in short images.
- This candidate does not by itself decide whether issue #661's code-simplification goal is worth
  this measured cost for the affected image shapes - that tradeoff (maintainability vs. a
  conditional regression) is exactly what the issue thread was already weighing, and remains a
  human/maintainer judgment call per this skill's own ground rules.
