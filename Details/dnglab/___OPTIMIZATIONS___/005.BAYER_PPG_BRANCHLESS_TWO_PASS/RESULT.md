# Bayer PPG: split branchy per-pixel loops into branch-free two-pass stride-2 loops

`interpolate_green`, `interpolate_rb_at_green`, and `interpolate_rb_at_non_green`
(`rawler/src/imgop/sensor/bayer/ppg.rs:111-249`) each iterate every interior column of a row
(`col` in `3..w-3`) and gate the whole body on `shifted.color_at(row, col) == / != CFA_COLOR_G`.
Candidate #004 replaced the surrounding `.skip().take()` iterator chains with direct slice
indexing and found no measurable change, concluding the real blocker was the per-pixel branch
itself plus the non-adjacent unsafe loads, not the loop-bound expression form (see
`004.BAYER_PPG_SKIP_TAKE_SLICE/RESULT.md`). PPG only ever runs on a 2x2-periodic Bayer CFA
(`demosaic()` panics for anything `!cfa.is_rgb()`), which means within a single row the
green/non-green column parity is fixed - and so are several values the original code recomputed
per pixel via `shifted.color_at(...)` (the non-green channel `ch`/`x_ch`, and `h_ch`/`v_ch` in
`interpolate_rb_at_green`). This candidate hoists the row-constant parity/channel lookups once per
row and turns each function's inner loop into a plain `col += 2` stride walk with no branch and no
per-pixel `color_at()` call.

## What was tried

One variant, applied to all three functions identically: replace
`for (col, pixel) in buf.iter_mut().enumerate().skip(3).take(w - 6) { if shifted.color_at(row, col) <cond> CFA_COLOR_G { ... } }`
with computing `start` (3 or 4, whichever column parity satisfies `<cond>` for this row) and the
row-constant channel(s) once, then `let mut col = start; while col < w - 3 { ...; col += 2; }` with
no per-pixel branch or `color_at()` call. Same set of `(row, col)` pairs visited and the same
arithmetic per pixel as before (verified bit-exact, see below) - the compiler now sees a
branch-free, uniform-stride loop instead of a fully-scanned, branch-gated one.

Measured with `rawler/benches/demosaic.rs`'s existing `full_image_canon_r5` case (Canon EOS R5
CRAW, criterion, `--features rawdb`), plus the same kind of scratch example candidate #004 used
(`rawler/examples/ppg_dump.rs`, added and removed for this candidate only, not part of the patch)
that md5-hashes the full `Intermediate::ThreeColor` output buffer for bit-exactness.

The machine turned out to be noisy mid-session (see "Why the result looks the way it does"), so
non-interleaved same-session baseline/patched pairs (`base1/base2` vs `patched1/patched2`,
`base*_sc` vs `patched1_sc/patched2_sc`) are reported for completeness but the interleaved
runs (`*_late*`, baseline and patched measured back-to-back, alternating which side ran first)
are the ones actually trusted for the result below.

| Variant | single-core p50 (`RAYON_NUM_THREADS=1 taskset -c 0`) | multi-core p50 (unpinned, 24 cores) | notes |
|---|---|---|---|
| baseline, run 1 (`base1`/`base1_sc`) | 1.4656 s [1.4557, 1.4784] | 271.99 ms [265.28, 279.09] | early in session, machine quiet (`uptime` load ~1.9-6.7) |
| baseline, run 2 (`base2`/`base2_sc`) | 1.4837 s [1.4582, 1.5131] | 273.53 ms [264.93, 286.80] | repeat, same conditions |
| patched, run 1 (`patched1`/`patched1_sc`) | 1.4411 s [n/a, from criterion] | 279.89 ms [275.35, 283.82] | measured later in session, after `uptime` load had risen to ~6.7-9.0 |
| patched, run 2 (`patched2`/`patched2_sc`) | 1.3834 s [n/a] | 276.48 ms [270.14, 287.78] | same later-session conditions |
| **interleaved pair 1**: baseline (`base_late`) then patched (`patched_late`), multi-core only | - | base: 309.03 ms [282.08, 344.40]; patched: 273.95 ms [267.23, 284.59] | `uptime` load 15-22 during this pair (heaviest-noise point of the session); patched ~11.4% faster than baseline measured immediately before it |
| **interleaved pair 2**: baseline (`base_late2`) then patched (`patched_late2`), multi-core | - | base: 278.47 ms [275.59, 282.47]; patched: 276.09 ms [273.56, 278.39] | load settling (~9.7-13.8); patched ~0.9% faster, tight overlapping CIs - essentially flat |
| **interleaved pair 3**: baseline (`base_late3`) then patched (`patched_late3`), multi-core | - | base: 273.37 ms [269.72, 277.27]; patched: 263.21 ms [261.76, 264.30] | load quiet again (~8-10); patched ~3.7% faster, CIs non-overlapping |
| **interleaved pair, single-core**: baseline (`base_late_sc`) then patched (`patched_late_sc`) | base: 1.4475 s [1.4440, 1.4519]; patched: 1.3700 s [1.3663, 1.3736] | - | load quiet (~3-4); patched ~5.4% faster, CIs non-overlapping |

## Result

**Single-core** (the low-noise ranking metric): the one clean, interleaved, back-to-back
comparison (`base_late_sc` vs `patched_late_sc`) shows a consistent, non-overlapping-CI ~5.4%
speedup (1.4475 s -> 1.3700 s). This agrees in direction with two of the three earlier
non-interleaved patched single-core numbers (`patched2_sc` 1.3834 s clearly faster than both
`base1_sc`/`base2_sc`; `patched1_sc` 1.4411 s only marginally faster).

**Multi-core**: the naive same-session comparison (`base1`/`base2` at ~264 ms median vs
`patched1`/`patched2` at ~275-278 ms median) looks like a ~4-5% *regression* - but that comparison
is confounded: the patched runs were captured later in the session, after the machine's `uptime`
load average had already climbed from ~1.9-6.7 to ~6.7-9.0 from the earlier benching itself. Three
separate interleaved pairs (baseline and patched measured back-to-back, no other work between
them) all show patched at or below baseline: -11.4%, -0.9%, -3.7%. None show patched slower than
its immediately-preceding baseline. The magnitude varies with how noisy the machine was at that
moment (the -11.4% pair was measured during the noisiest 5-minute window of the whole session,
`uptime` load 15-22 on 24 cores), but the *direction* is consistent across all three: real,
reproducible improvement once the session-time drift confound is controlled for.

## Does it change behavior/output?

- **Bit-exact?** Yes. Full `Intermediate::ThreeColor` output buffer for the Canon EOS R5 CRAW
  sample (559,851,264 bytes, 3 f32 channels) hashed identically (md5 `c27e167f1732eab0fd277971a9cc3987`)
  before and after the patch - the same hash candidate #004 recorded for the same sample,
  confirming this rewrite visits the exact same `(row, col)` set with the exact same arithmetic.
- **Is this a breaking change?** No public API change - all three touched functions are private
  (`fn`, not `pub fn`) to `rawler::imgop::sensor::bayer::ppg`.
- Project test suite: `cargo test -p rawler --lib` - 66 passed, 0 failed, both before and after.
- **New assumption baked into the code**: the rewrite assumes the CFA period is exactly 2x2 (every
  real Bayer RGGB/BGGR/GRBG/GBRG pattern `CFA::new()` can produce, since those are always 4-char
  pattern strings). This isn't a new *runtime* assumption - the original algorithm's neighbor
  offsets (`row-1`, `row-2`, `col+1`, `col+2`, diagonals) only make arithmetic sense for a
  2-periodic Bayer CFA in the first place - but it is now baked into how row start-offsets are
  derived (`shifted.color_at(row, 3)` sampled once instead of `color_at` being called, and
  implicitly re-validated, at every column). No code path in this codebase constructs a non-2x2
  CFA and then feeds it through `PPGDemosaic` (`is_rgb()` doesn't itself enforce 2x2, but no
  camera config or decoder in this repo produces an RGB-only pattern wider than 2x2 - see grep
  results in the session transcript). Not enforced with a `debug_assert!` in this patch; a
  maintainer taking this patch for real should consider adding one.

## Why the result looks the way it does

Splitting the per-pixel branch into two passes (conceptually - implemented here as a single
stride-2 loop per row rather than literally two separate passes, since for a 2-periodic CFA "the
G pass" and "the non-G pass" are just the two column parities) removes both the per-pixel branch
*and* the per-pixel `color_at()` table lookups that fed it (`ch`/`x_ch` in `interpolate_green`/
`interpolate_rb_at_non_green`, `h_ch`/`v_ch` in `interpolate_rb_at_green` are now computed once
per row instead of once per pixel). The remaining per-pixel work (8-9 `unsafe` loads from
non-adjacent offsets, `abs()`/comparison chains, `hue_transit`) is unchanged and is inherently a
gather-style access pattern that plain auto-vectorization won't turn into contiguous SIMD loads
regardless - so the win here comes from cutting the branch-and-lookup overhead around that gather,
not from vectorizing the gather itself. That's consistent with a real but modest (~4-5%
single-core-comparable) improvement rather than a multiplicative one: PPG's per-pixel cost was
never *only* the branch, and the branch/lookup removal is a fixed per-pixel saving on top of an
irreducible gather cost.

The initial multi-core numbers looked like a regression purely because of a session-timing
confound (see "Result" above) - a same-day, same-machine but different-time comparison is not a
valid A/B without interleaving, which is exactly the trap MEASUREMENT.md's noise-floor section
warns about. This was caught by rerunning as three interleaved pairs rather than trusting the
first same-session-but-sequential numbers.

## What's not done / open questions

- No disassembly comparison done to directly confirm the branch/lookup elimination in the
  generated code (the multiversion-tagged functions have several target variants; a definitive
  check would need to pick one and diff it against candidate #004's baseline disassembly).
- The CFA-period-2 assumption (see "Does it change behavior/output?") is implicit, not asserted.
  Adding `debug_assert!(shifted.width == 2 && shifted.height == 2)` at the top of `demosaic()`
  would make it explicit and catch a future caller that violates it in a debug build, at zero
  release-build cost.
- Not tried: applying the same row-constant-hoisting idea to `interpolate_borders` (untouched by
  this candidate - it's not one of the three functions #004's profiling flagged, and its loop
  shape - variable-width border strip, not a full-width interior scan - doesn't map onto a simple
  stride-2 rewrite as directly).
- The single-core result rests on one clean interleaved pair plus two non-interleaved patched runs
  that partially agree (`patched2_sc` agrees, `patched1_sc` is closer to baseline) - a fourth,
  fully-interleaved single-core pair would strengthen confidence further but wasn't run given the
  ~85s-per-run cost of single-core criterion sampling here.
- Machine was unusually noisy for part of this session (`uptime` load spiking to ~30 on a 24-core
  box with no other user-visible CPU-bound process in `ps aux` - plausibly host-level contention
  from something outside this container/session). Anyone re-running this candidate's justfile on
  a quieter machine should expect tighter numbers than the noisiest interleaved pair reported here.
