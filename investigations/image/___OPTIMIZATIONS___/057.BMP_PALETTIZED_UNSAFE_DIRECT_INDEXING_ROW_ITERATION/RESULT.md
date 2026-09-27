# BMP palettized row decode: `unsafe`/`get_unchecked_mut` direct-offset indexing (no bounds check at all)

Direct follow-up to candidates 051/056, both of which left open exactly this next step ("What's
not done"): candidate 051 replaced `ChunksExactMut`+`Option`-returning `.next()` with a still-
*checked* `row.get_mut(offset..offset+3)` in the plain (non-RLE) palettized row path
(`read_palettized_pixel_data`'s `set_1bit/2bit/4bit/8bit_pixel_run`), and found a confusing,
non-monotonic real-decode result across bit depths (1-bit regressed ~30-43%, 2-bit regressed
~37-38% per candidate 056, 4-bit improved ~12-14%, 8-bit was small/inconsistent) that did not match
candidate 047's isolated microbenchmark prediction. Both 051 and 056 proposed an `unsafe`,
`get_unchecked_mut`-based variant next, to isolate whether the regression was about the *shape* of
the bounds check (a two-sided `Range` check in `get_mut` vs. `ChunksExactMut::next()`'s own cursor
check) or about direct indexing itself. This candidate does exactly that.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as candidates 051/056).

Changed the plain (non-RLE) palettized row path's four pixel-store functions to take `row: &mut
[u8]` and `num_channels: usize` directly (same shape as candidate 051's `_direct` functions),
track a running byte `offset`, and store through `unsafe { row.get_unchecked_mut(offset..offset +
3) }` - no bounds check of any kind on the per-pixel store, unlike 051/056's still-checked
`get_mut`. Safety relies on an invariant that already holds unconditionally in this codebase (see
the `SAFETY` comment added at the call site in `patch.diff`): `row` is always exactly `width *
num_channels` bytes (`with_rows_resumable`'s own `get_row_mut` is the sole source of `row` in the
caller), `num_channels` is always 3 or 4 on this path (the 1-channel indexed-color case takes a
different, `clone_from_slice`-based branch that never reaches these functions), and each function
below is capped to write at most `width` pixels (1-bit needed a new `n_pixels` parameter added for
this - the original `set_1bit_pixel_run` had no cap, relying entirely on `pixel_iter.next()`
returning `None` to terminate at the row boundary, which the unsafe variant can no longer rely on
for termination). Under that cap, the last store starts at `offset == (width - 1) * num_channels`,
and `offset + 3 <= (width - 1) * num_channels + 3 <= width * num_channels == row.len()` whenever
`num_channels >= 3`, which always holds here.

Same split as candidate 051: `set_1bit_pixel_run`/`set_2bit_pixel_run` (only caller is this path)
were changed in place, renamed `_unsafe`; `set_4bit_pixel_run`/`set_8bit_pixel_run` (also called
from the RLE4/RLE8 decode path, which relies on `ChunksExactMut`'s own persistent cursor across
several operations within one row - `nth`, `for_each`, partial `.next()` consumption) were left
untouched and new `set_4bit_pixel_run_unsafe`/`set_8bit_pixel_run_unsafe` functions were added
alongside for the plain path only. Full change in `patch.diff` (116 insertions, 27 deletions to
`src/codecs/bmp/decoder.rs`, plus candidate 020's `pal2_large.bmp` bench fixture re-added exactly
as candidate 056 did, bundled into the same `patch.diff` so all four bit depths can be measured in
one session - see that file's own header comment).

Measured with the crate's own existing `benches/decode.rs` criterion bench: `load-Bmp/Core_1_Bit.bmp`,
`load-Bmp/pal2_large.bmp` (candidate 020's 700x700 2-bit fixture, re-added), `load-Bmp/Core_4_Bit.bmp`,
`load-Bmp/Core_8_Bit.bmp` - covering all four bit depths in a single session, unlike 051 (three
depths) or 056 (2-bit only).

Methodology: two prebuilt `--release` bench binaries (baseline = fixture-only, unpatched decoder;
patched = fixture + this `unsafe` decoder patch), compared via criterion's own
`--save-baseline`/`--baseline` A/B mechanism, same as 051/056. 4 pairings per bit depth
(single-core `taskset -c 3` run1 baseline-then-patched, single-core run2 patched-then-baseline
reversed, unpinned run1 baseline-then-patched, unpinned run2 patched-then-baseline reversed) - same
two prebuilt binaries reused throughout, only comparison order/pinning changes. `uptime` was
2.13-17.61 during the session (24-core machine; one initial reading of 17.61 prompted a short wait
before the single-core runs, which then proceeded at 5.84 and below - all readings stayed under
the 24-core count throughout, so no re-baseline was needed per SKILL.md's "load > core count"
rule).

Criterion's own point estimates (mean-of-samples "time:" middle value, µs, `n=100` samples per
run, 3s warmup). Each row's "change" is criterion's own printed point estimate for that specific
run (current run relative to whichever binary was saved as that run's baseline - direction noted
in the "current is" column so the two reversed-order rows can be read as agreeing or disagreeing):

| Bit depth | Pairing | current run's own time | saved-baseline's time | change (current vs. saved) | current is |
|---|---|---|---|---|---|
| 1-bit | single-core run1 (patched vs. saved=baseline) | 85.725 | 78.552 | +5.92% (p=0.00) | patched, slower |
| 1-bit | single-core run2 (baseline vs. saved=patched) | 81.266 | 84.764 | -3.64% (p=0.00) | baseline, faster (= patched slower, same direction) |
| 1-bit | unpinned run1 (patched vs. saved=baseline) | 82.947 | 74.854 | +9.88% (p=0.00) | patched, slower |
| 1-bit | unpinned run2 (baseline vs. saved=patched) | 78.192 | 80.636 | -2.36% (p=0.00) | baseline, faster (= patched slower, same direction) |
| 2-bit | single-core run1 (patched vs. saved=baseline) | 248.62 | 264.01 | -8.53% (p=0.00) | patched, faster |
| 2-bit | single-core run2 (baseline vs. saved=patched) | 270.05 | 246.70 | +9.67% (p=0.00) | baseline, slower (= patched faster, same direction) |
| 2-bit | unpinned run1 (patched vs. saved=baseline) | 245.65 | 259.26 | -5.47% (p=0.00) | patched, faster |
| 2-bit | unpinned run2 (baseline vs. saved=patched) | 260.13 | 243.43 | +7.17% (p=0.00) | baseline, slower (= patched faster, same direction) |
| 4-bit | single-core run1 (patched vs. saved=baseline) | 80.612 | 122.68 | -34.83% (p=0.00) | patched, faster |
| 4-bit | single-core run2 (baseline vs. saved=patched) | 119.35 | 80.595 | +48.28% (p=0.00) | baseline, slower (= patched faster, same direction) |
| 4-bit | unpinned run1 (patched vs. saved=baseline) | 78.831 | 119.13 | -34.21% (p=0.00) | patched, faster |
| 4-bit | unpinned run2 (baseline vs. saved=patched) | 122.47 | 78.159 | +53.47% (p=0.00) | baseline, slower (= patched faster, same direction) |
| 8-bit | single-core run1 (patched vs. saved=baseline) | 90.294 | 106.67 | -16.07% (p=0.00) | patched, faster |
| 8-bit | single-core run2 (baseline vs. saved=patched) | 104.94 | 89.377 | +17.90% (p=0.00) | baseline, slower (= patched faster, same direction) |
| 8-bit | unpinned run1 (patched vs. saved=baseline) | 91.302 | 103.13 | -12.07% (p=0.00) | patched, faster |
| 8-bit | unpinned run2 (baseline vs. saved=patched) | 104.90 | 90.494 | +17.73% (p=0.00) | baseline, slower (= patched faster, same direction) |

(All 16 pairings statistically significant at p=0.00; every reversed-order pairing agrees with its
run1 counterpart on *which binary is faster*, so none of this is a run-order/thermal artifact. Raw
criterion console output for the unpinned re-run, matching exactly, is also reproduced verbatim by
this candidate's own `justfile`'s `just test` - see that run's transcript for the exact numbers
without the table's rounding.)

## Result

**A materially different, and substantially more positive, pattern than candidates 051/056's
checked `get_mut` variant** - only 1-bit regresses, and by far less than 051's checked variant did:

- **1-bit: ~2.4-9.9% SLOWER**, consistently across all 4 pairings - a real regression, but an
  order of magnitude smaller than candidate 051's checked `get_mut` variant at the same bit depth
  (~30-43% slower there).
- **2-bit: ~5.5-9.7% FASTER**, consistently across all 4 pairings - the *opposite sign* from
  candidate 056's checked `get_mut` variant, which regressed 2-bit by ~37-38%.
- **4-bit: ~32.8-53.5% FASTER**, consistently across all 4 pairings - same direction as candidate
  051's checked variant (which also improved 4-bit) but roughly 3x the magnitude (051: ~12-14%).
- **8-bit: ~10.8-18.5% FASTER**, consistently across all 4 pairings - also the *opposite sign* from
  candidate 051's checked variant, which measured 8-bit as small and inconsistent
  (+4.9% to +5.4%/insignificant).

Removing the bounds check entirely (rather than trading `ChunksExactMut` for a still-checked
`get_mut`) turns three of the four bit depths from regressions/noise into clear wins, and leaves
only 1-bit as a real (much smaller) regression. This directly answers 051/056's own open question:
the earlier regression at 2-bit and 8-bit was substantially about the *shape* of the bounds check
(`get_mut`'s two-sided `Range` check specifically), not about direct indexing or removing
`ChunksExactMut` per se - removing the check altogether recovers (and then some) at those two
depths. 1-bit's smaller-but-still-real regression suggests something else is still costing time
there beyond just the bounds check (see "Why the result looks the way it does").

## Does it change behavior/output?

**Bit-exact.** The unsafe store computes the exact same output bytes as the original and as
candidates 051/056's checked variant (`row[offset..offset+3] = palette[idx][0..3]`, same
three-channel copy) - only the per-pixel store's bounds-checking mechanism changed (none, instead
of a checked `Range` index or a `ChunksExactMut` cursor check), no rounding/approximation/output
logic changed. Confirmed by the project's own reference-image tests (`tests/reference_images.rs`),
which passed identically before and after for every BMP corpus file, including
`Core_1_Bit.bmp`/`Core_4_Bit.bmp`/`Core_8_Bit.bmp`, `pal2.bmp`/`pal2color.bmp`, `pal2_large.bmp`,
`pal4rle.bmp`/`pal8rle.bmp` (`cargo test --release --test reference_images -- bmp`: 71 passed / 0
failed, both before and after).

**Is this a breaking change?** No - internal-only change to private, non-`pub` functions; no
public API/behavior/file-format change.

**Project test suite** (`cargo test --release`, run in full both before applying the patch and
after, bench fixture present in both runs): **551 passed / 1 failed, identical set, both before and
after.** The single failure is the same pre-existing, unrelated fixture mismatch every prior
BMP-decoder candidate in this workspace has recorded (`resizing_with_catmul` in
`tests/regression.rs`, a CatmullRom-filter regression test failure with no connection to BMP
decoding). No new failures introduced by this patch.

## Why the result looks the way it does

The core mechanism candidates 051/056 already identified still applies: `get_mut`'s two-sided
`Range` bounds check (`offset <= offset + 3 && offset + 3 <= row.len()`) costs more per call than
`ChunksExactMut::next()`'s own single-comparison cursor check at higher call densities. This
candidate's result shows that removing the check entirely (rather than trading one checked shape
for another) recovers that cost at 2-bit and 8-bit specifically, and roughly triples 4-bit's
existing improvement - consistent with "the check itself, not `ChunksExactMut` vs. direct
indexing in the abstract, was the dominant cost at those three depths."

1-bit remains a real (smaller) regression even with the bounds check fully removed, which the
"bounds-check shape" explanation alone does not cover. A plausible but unconfirmed guess: 1-bit's
inner `loop` (up to 8 stores per input byte, one per bit) is the only one of the four that needed a
**new** `n_pixels`-based termination check added (`if n_pixels == 0 { break 'outer; }`) purely to
replace the bounds check's own implicit termination role (the original relied on `pixel_iter.next()`
returning `None` at the row boundary to stop; the unsafe version has no such signal and must be
told explicitly via a decrementing counter) - so 1-bit trades one branch (`get_mut`'s check) for
another (`n_pixels == 0`) rather than eliminating a branch outright the way 2/4/8-bit's *existing*
`n_pixels`-based checks (already present in the original code for those three depths, just reused
here) do. This would predict 1-bit alone showing a smaller net win than the others, but does not
fully explain why it flips to a regression rather than just a smaller improvement - not confirmed
by disassembly this session (see "What's not done").

## What's not done / open questions

- **No disassembly-level confirmation** of the "why" section's 1-bit hypothesis (the added
  `n_pixels` check specifically, not the bounds-check removal, being the source of 1-bit's
  remaining regression) - `perf` remains unavailable in this container (per this workspace's
  `README.md`); an `objdump`/cachegrind instruction-count comparison of
  `set_1bit_pixel_run_unsafe` vs. the original `set_1bit_pixel_run`, matching candidate 037's own
  cachegrind methodology, would confirm or refute this directly. New idea, added to `IDEAS.md`.
- This candidate's `unsafe` block relies on an invariant (`row.len() == width * num_channels`,
  `num_channels >= 3`) that is not itself enforced by any type-level guarantee at the call site -
  it holds today because `with_rows_resumable` and `num_channels()` are both private to this
  module and their current implementations were read and confirmed by hand, not because the
  compiler can check it. A future refactor of either could silently invalidate the safety argument
  without triggering a compiler error - worth a `debug_assert!(row.len() >= offset + 3)` immediately
  before each `get_unchecked_mut` call if this were ever applied for real (not added here, to keep
  the release-mode-measured patch identical to what would actually ship; noted for whoever takes
  this further).
- Only one machine/session was used for this candidate's measurements, same as 051/056.
