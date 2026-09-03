# BMP 2-bit palettized row decode: candidate 051's direct-indexing patch, re-measured with candidate 020's bench fixture reinstated

Closes the gap candidate 051 explicitly left open ("What's not done"): its `ChunksExactMut`+
`Option`-returning `.next()` -> direct offset indexing (`row.get_mut(offset..offset+3)`) patch to
`src/codecs/bmp/decoder.rs`'s plain (non-RLE) palettized row path was measured at 1-bit/4-bit/8-bit
only, because the default `benches/decode.rs` `BENCH_DEFS` has no 2-bit case (candidate 020's own
`pal2_large.bmp` bench-corpus addition was reverted at the end of its session, like every
candidate's). This candidate re-adds candidate 020's `pal2_large.bmp` fixture (bench registration
+ testdata, unmodified) and re-runs candidate 051's exact decoder patch (unmodified) against it.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as candidate 051).

Step 1: reintroduced candidate 020's bench fixture only - `benches/decode.rs`'s new
`"pal2_large.bmp"` entry in the BMP `BENCH_DEFS` list, `tests/images/bmp/images/pal2_large.bmp`
(700x700, 2-bit indexed, 4-color palette, diagonal-stripe index pattern) and
`tests/reference/bmp/images/pal2_large.bmp.png` - taken verbatim from candidate 020's own
`patch.diff` (git-applied with `--include` filters to exclude that candidate's own, unrelated,
`&[[u8; 3]; 256]` fixed-array decoder change). Step 2: applied candidate 051's decoder patch
unmodified on top.

Methodology: same as candidate 051 - two prebuilt `--release` `decode` criterion bench binaries
(baseline: bench fixture only, no decoder patch; patched: bench fixture + candidate 051's decoder
patch), compared via criterion's own `--save-baseline`/`--baseline` A/B mechanism, filtered to
`load-Bmp/pal2_large.bmp`. 4 pairings: single-core (`taskset -c 3`) run1 baseline-then-patched,
single-core run2 patched-then-baseline reversed, unpinned run1 baseline-then-patched, unpinned
run2 patched-then-baseline reversed - same two prebuilt binaries reused throughout, only
comparison order/pinning changes. `uptime` was 5.2-6.2 during the session (24-core machine, well
under core count).

Criterion's own point estimates (mean-of-samples "time:" middle value, µs, `n=100` samples per run,
3s warmup):

| Pairing | baseline (no decoder patch) | patched (051's direct indexing) | change |
|---|---|---|---|
| single-core run1 | 255.80 | 349.60 | +36.72% (p=0.00) |
| single-core run2 (reversed) | 351.94 | 255.14 | -27.54% (p=0.00, same direction) |
| unpinned run1 | 253.40 | 351.23 | +38.45% (p=0.00) |
| unpinned run2 (reversed) | 348.14 | 254.69 | -27.10% (p=0.00, same direction) |

("same direction" = the reversed-order pairing's sign agrees baseline is the faster binary,
confirming the effect isn't a run-order/thermal artifact.)

## Result

**Real, consistent regression at 2-bit, ~37-38% slower** (single-core and unpinned agree,
4/4 pairings, both run orders) - closing candidate 051's open question: 2-bit does **not** land in
or near 4-bit's improvement (-12 to -14%), nor in 8-bit's small/inconsistent range (+4.9 to
+5.4%/insignificant). It lands close to 1-bit's own regression (~30-43% slower per candidate 051's
table), i.e. at the high-regression end of the range candidate 051 already measured, not a new
extreme outside it and not the direction the "iterator-machinery removal should help" hypothesis
originally predicted.

## Does it change behavior/output?

**Bit-exact.** `cargo test --release --test reference_images -- pal2_large` passes identically
with and without the decoder patch applied (same golden-PNG comparison candidate 020 already
established this fixture against) - candidate 051's patch computes the same output bytes, only the
per-pixel store/bounds-check mechanism differs, unchanged from candidate 051's own characterization.

**Is this a breaking change?** No - same internal-only change as candidate 051 (private,
non-`pub` functions only); this candidate's own addition (test image + reference PNG + one bench
registration) is a data-only addition, not a behavior change.

**Project test suite** (`cargo test --release`, run in full before and after applying the decoder
patch, bench fixture present in both runs): **551 passed / 1 failed, identical set, both before and
after.** The one failure is the same pre-existing, unrelated `resizing_with_catmul` fixture
mismatch (`tests/regression.rs`, CatmullRom-filter, no connection to BMP decoding) every prior
BMP-decoder candidate in this workspace has recorded. No new failures introduced.

## Why the result looks the way it does

2-bit's per-input-byte call density (`set_2bit_pixel_run`'s macro-driven loop unpacks 4 sub-pixels
per byte, one `get_mut`/`.next()` call each) sits between 1-bit's (8 calls per byte, in a tight
inner `loop`) and 4-bit's (2 calls per byte) - by candidate 051's own "density correlates with
regression size" observation for 1-bit/8-bit, a call density of 4 predicted a regression somewhere
between 1-bit's (~30-43%) and 4-bit's own reversed sign (a ~12-14% *improvement*), not a value
essentially matching 1-bit's own high end. This candidate's result is consistent with a density
correlation for the *regressing* half of the pattern (1-bit and 2-bit both regress, 2-bit closer
to 1-bit's magnitude than to 4-bit's), but does not explain why 4-bit alone flips sign entirely -
the same open question candidate 051 already left unresolved (see its "Why the result looks the
way it does": 4-bit's `set_pixel!` macro branch structure is a plausible but unconfirmed guess).
No disassembly was attempted this session either (`perf` remains unavailable in this container per
this workspace's `README.md`; the `objdump`/cachegrind comparison candidate 051 flagged as
"not done" is a separate, still-open follow-up, not part of this candidate's narrower scope).

## What's not done / open questions

- **No disassembly-level confirmation**, same gap candidate 051 already left open - now applies to
  2-bit's `set_2bit_pixel_run_direct` too, not just 1-bit's. An `objdump`/cachegrind instruction-
  count comparison across all four bit depths at once (matching candidate 037's methodology) would
  settle whether call-density alone explains the regressing half of the pattern, or whether 4-bit's
  sign flip needs a separate explanation - still open, added back to `IDEAS.md` generalized to all
  four depths rather than just 1-bit.
- The `unsafe`/`get_unchecked_mut` variant candidate 051 proposed (to isolate whether the
  regression is about the *shape* of the bounds check specifically, not direct indexing itself)
  was not tried here either - same open item, now with a fourth density's data point (2-bit)
  available to validate against once it exists.
- Only one machine/session was used for this candidate's measurements, same as candidate 051.
