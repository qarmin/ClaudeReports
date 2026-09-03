# BMP palettized row decode: replace `ChunksExactMut`+`Option`-returning `.next()` with direct offset indexing

Follow-up to candidate 047, which found essentially all of the "row/iterator machinery" cost in
the 037/041/042/046/047 line comes from the per-pixel `ChunksExactMut::next()`/`Option` match, not
the once-per-scanline outer closure - and named "a candidate that changes the real decoder's
iteration style (e.g. replacing `ChunksExactMut`+`.next()` with direct indexing)" as the natural
next step, since the closure itself is apparently not where the cost is. This candidate does
exactly that: a real patch to `src/codecs/bmp/decoder.rs`'s plain (non-RLE) palettized row path.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`.

Changed `read_palettized_pixel_data`'s per-row closure (`set_1bit_pixel_run`/`set_2bit_pixel_run`/
`set_4bit_pixel_run`/`set_8bit_pixel_run`) from taking `pixel_iter: &mut ChunksExactMut<u8>` and
calling `pixel_iter.next()` (`Option`-returning, one per pixel) to taking `row: &mut [u8]` and
`num_channels: usize` directly, tracking a running byte `offset` and writing through
`row.get_mut(offset..offset + 3)` (still bounds-checked - `get_mut` returns `Option`, matching the
original's own early-exit-on-overrun behavior for malformed/short rows - but as a single range
index against the row slice's own length, not a separate `ChunksExactMut` iterator object with its
own internal chunk-count bookkeeping), then `offset += num_channels` per pixel.

New functions (`set_8bit_pixel_run_direct`/`set_4bit_pixel_run_direct`) were added **alongside**
the originals rather than replacing them in place, because `set_8bit_pixel_run`/
`set_4bit_pixel_run` are also called from the RLE4/RLE8 decode path (`read_rle_data`, around line
2140-2340), which relies on the `ChunksExactMut` iterator's own persistent cursor across several
different operations within one row (`nth` to skip to a resume position, `for_each` to zero-fill
the row tail, partial `.next()` consumption inside nested control-byte handling) - not just a
plain per-pixel `.next()` in a tight loop, so it isn't a fair like-for-like swap and was left
untouched. `set_1bit_pixel_run`/`set_2bit_pixel_run` have no other caller, so those were changed
in place (renamed to `_direct` for naming consistency with the 4/8-bit pair) with no duplication
needed. Full change in `patch.diff` (107 insertions, 29 deletions - net growth from the two
duplicated 4/8-bit functions).

Measured with the crate's own existing `benches/decode.rs` criterion bench (`load-Bmp/Core_1_Bit.bmp`,
`load-Bmp/Core_4_Bit.bmp`, `load-Bmp/Core_8_Bit.bmp` - the same 435x353-scale corpus files used as
the "small" scale by candidates 037/041/042/046/047, exercising exactly the palettized-row path
this patch touches). No 2-bit case exists in the default `BENCH_DEFS` list (candidate 020 added
one, `pal2_large.bmp`, but that candidate's changes are reverted like every other candidate's -
2-bit is untested by this candidate, noted under "What's not done").

Methodology: two prebuilt binaries (baseline = unpatched `HEAD`, patched = this patch applied),
compared via criterion's own `--save-baseline`/`--baseline` A/B mechanism rather than relying on a
single binary's repeated runs (this workspace's own README notes single-binary repeated
`cargo bench` was untrustworthy on this machine). Each of 4 pairings (single-core `taskset -c 3`
run1 baseline-then-patched, single-core run2 patched-then-baseline reversed, unpinned run1
baseline-then-patched, unpinned run2 patched-then-baseline reversed) rebuilds neither binary -
same two prebuilt binaries reused throughout, only the comparison order/pinning changes, so any
consistent number across all 4 orderings can't be a build-order or binary-identity artifact.
`uptime` was 0.71-12.73 during the session (one single-core run1 saw an elevated 12.73 load
average from an unrelated background process - repeated as unpinned run1 afterward at 0.71 to
confirm the number wasn't a load artifact; same magnitude both times).

Criterion's own point estimates (mean-of-samples "time:" middle value, µs, `n=100` samples per
run, 3s warmup):

| Bit depth | Pairing | baseline (unpatched) | patched (`_direct`) | change |
|---|---|---|---|---|
| 1-bit | single-core run1 | 73.271 | 104.31 | +42.40% (p=0.00) |
| 1-bit | single-core run2 (reversed) | 104.26 | 73.12 | -29.90% (p=0.00, same direction) |
| 1-bit | unpinned run1 | 71.727 | 102.34 | +42.65% (p=0.00) |
| 1-bit | unpinned run2 (reversed) | 104.36 | 72.23 | -30.40% (p=0.00, same direction) |
| 4-bit | single-core run1 | 114.83 | 101.23 | -11.85% (p=0.00) |
| 4-bit | single-core run2 (reversed) | 101.28 | 115.82 | +14.35% (p=0.00, same direction) |
| 4-bit | unpinned run1 | 112.84 | 99.720 | -11.85% (p=0.00) |
| 4-bit | unpinned run2 (reversed) | 100.48 | 113.05 | +12.28% (p=0.00, same direction) |
| 8-bit | single-core run1 | 102.02 | 107.07 | +4.92% (p=0.00) |
| 8-bit | single-core run2 (reversed) | 107.09 | 100.49 | -4.90% (p=0.00, same direction) |
| 8-bit | unpinned run1 | 99.563 | 105.54 | +5.42% (p=0.00) |
| 8-bit | unpinned run2 (reversed) | 105.55 | 103.60 | -0.57% (p=0.59, **not** significant) |

("same direction" = the reversed-order pairing's sign agrees that the same binary is faster,
confirming the effect isn't a run-order/thermal artifact - baseline is consistently faster at
1-bit, patched is consistently faster at 4-bit, both regardless of which binary ran first.)

## Result

**Mixed, and by a large margin at 1-bit specifically**, not the uniform improvement the
"replace the iterator with direct indexing" hypothesis predicted:

- **1-bit (density 8, most `.next()`/index calls per input byte): patched is ~30-43% SLOWER**,
  consistently across all 4 pairings (both pinning modes, both run orders) - the single largest
  effect of any bit depth measured, and a regression, not a win.
- **4-bit (density 2): patched is ~12-14% FASTER**, consistently across all 4 pairings - a real,
  reproducible improvement.
- **8-bit (density 1, one call per byte): small and inconsistent** - +4.9% to +5.4% regressed in
  3 of 4 pairings, but statistically insignificant (p=0.59) in the 4th (unpinned run2) - at best a
  small regression, plausibly within noise at this scale.
- 2-bit was not measured (no bench case in the default corpus - see "What's not done").

This does **not** confirm candidate 047's implicit framing (that removing the `ChunksExactMut`/
`Option` iterator machinery should recover time proportional to its isolated synthetic cost at
every density). The direction and magnitude vary by bit depth in a way the synthetic microbenchmark
line (037/041/042/046/047) did not predict - 047's own isolated "iterator machinery" cost was
*largest* at 1-bit (+15.9% of real decode time) and *smallest* at 4-bit (+3.8%), which is the
opposite ranking from this candidate's real-decode result (4-bit improves most, 1-bit regresses
most).

## Does it change behavior/output?

**Bit-exact.** The direct-indexing store computes the exact same output bytes as the original
(`row[offset..offset+3] = palette[idx][0..3]`, same three-channel copy, same early-return-on-overrun
semantics via `Option`-returning `get_mut` in place of `Option`-returning `.next()`) - no rounding,
approximation, or output-affecting logic changed, only how the destination byte range is computed
and bounds-checked. Confirmed by the project's own reference-image tests (`tests/reference_images.rs`
hashes/compares every BMP corpus file, including `Core_1_Bit.bmp`/`Core_4_Bit.bmp`/`Core_8_Bit.bmp`,
`pal2.bmp`/`pal2color.bmp`, `pal4rle.bmp`/`pal8rle.bmp`), which passed identically before and after.

**Is this a breaking change?** No - internal-only change, no public API/behavior/file-format
change.

**Project test suite** (`cargo test --release`, run in full both before applying the patch and
after): **550 passed / 1 failed, identical set, both before and after.** The single failure is the
same pre-existing, unrelated fixture mismatch every prior BMP-decoder candidate in this workspace
has recorded (`resizing_with_catmul` in `tests/regression.rs`, a CatmullRom-filter regression test
failure with no connection to BMP decoding). No new failures introduced by this patch.

## Why the result looks the way it does

The 1-bit path calls the per-pixel store up to 8 times per input byte (once per bit, in a tight
inner `loop`), by far the most of any density tested - so any extra per-call cost from
`row.get_mut(offset..offset+3)` (a two-sided `Range` index, which needs to verify both
`offset <= offset + 3` and `offset + 3 <= row.len()`) versus `ChunksExactMut::next()` (which only
needs to check its own internal remaining-chunk count before advancing a fixed-stride pointer) gets
multiplied by far more calls than at any other density - consistent with 1-bit showing the largest
regression of any bit depth. This mirrors the *direction* candidate 047's isolated microbenchmark
predicted (iterator-machinery cost scales with density) but not the *sign*: 047 predicted removing
that machinery should help, proportionally more at higher density; here it consistently hurts at
the highest density (1-bit) and consistently helps at a lower one (4-bit, density 2). Candidate
047's own synthetic loop never modeled `get_mut`'s two-sided range check specifically - it measured
`ChunksExactMut`+`Option` there as *slower* than a raw pointer store, but never measured
`get_mut(offset..offset+3)` as a third option, so nothing in 037/041/042/046/047 predicted a
same-density comparison between `ChunksExactMut` and range-indexed `get_mut` specifically (the
comparison this candidate's real patch actually makes) - that comparison was never isolated in a
microbenchmark before this candidate, real or synthetic.

Why 4-bit specifically improves is not confirmed by disassembly in this session (see "What's not
done") - a plausible but unconfirmed guess is that 4-bit's `set_pixel!` macro already branches
(`if n_pixels == 0 { break; }`) per sub-pixel regardless of store style, so the marginal cost added
by `get_mut`'s extra range check is smaller relative to what LLVM already emits around that
existing branch, while whatever `ChunksExactMut::next()`-specific overhead this replaces (Option
construction/matching, `#[repr]` internal cursor increment logic) happened to cost more than the
range-indexing replacement at this specific density - not verified against actual codegen.

## What's not done / open questions

- **2-bit was not measured** - no bench case exists in the default `benches/decode.rs` `BENCH_DEFS`
  list for this workspace's tree (candidate 020 added `pal2_large.bmp` but that patch, like every
  other candidate's, is reverted). Given how much the effect varies between 1-bit/4-bit/8-bit, 2-bit
  could plausibly land anywhere in the range already seen - this is a real gap, not a rounding
  error, added to `IDEAS.md`.
- **No disassembly-level confirmation** of the "why" section's hypothesis (a two-sided `Range`
  bounds check in `get_mut` costing more per-call than `ChunksExactMut::next()`'s internal cursor
  check, especially at 1-bit) - inferred from the measured shape (density correlates with
  regression size for 1-bit/8-bit) but not from reading generated assembly side by side, and does
  not explain why 4-bit improves rather than also regressing by a smaller amount. `perf` remains
  unavailable in this container (see this workspace's `README.md`); an `objdump`/cachegrind
  instruction-count comparison of `set_1bit_pixel_run_direct` vs. the original
  `set_1bit_pixel_run`, matching candidate 037's own cachegrind methodology, would confirm or
  refute this directly and was not attempted this session (time-boxed).
- An `unsafe`, `get_unchecked_mut`-based direct-indexing variant (skipping the bounds check
  entirely rather than trading `ChunksExactMut` for a still-checked `get_mut`) was not tried - it
  would isolate whether the regression is specifically about the *shape* of the bounds check
  (range vs. cursor) rather than direct indexing itself, but introduces `unsafe` into the real
  decoder, a different risk profile from every other candidate in this workspace so far. New idea,
  added to `IDEAS.md`.
- Only one machine/session was used for this candidate's measurements.
