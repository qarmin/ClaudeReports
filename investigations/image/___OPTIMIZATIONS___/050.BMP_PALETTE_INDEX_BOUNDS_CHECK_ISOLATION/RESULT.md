<!--
Measurement-only investigation - no patch to the tested project's own source. See "Does it
change behavior/output?" below for the explicit N/A statement this workspace requires.
-->

# Palette-index bounds-check isolation: what does `palette[idx as usize]`'s real, unsized-slice bounds check cost, on top of candidate 047's row/iterator floor?

Follow-up to candidate 047, closing the exact gap it named in its own "What's not done": *"The
bounds check itself (the actual branch/panic-path overhead from `palette[idx as usize]` where
`palette: &[[u8; 3]]` is an unsized slice, per candidates 008/011/025's own findings) is still
never isolated in any microbenchmark in this line (037/041/042/046/047) - all are deliberately
built branch-free via `get_unchecked`/fixed-size arrays."* This is the last per-pixel-loop
hypothesis left in the 037/041/042/046/047 investigation line before the residual would have to be
attributed to something structurally different (e.g. I/O/buffer-fill overhead in `read_scanline`,
also flagged in `IDEAS.md`).

This is **not** a change to `image`'s own source - no code path in the real crate was touched,
patched, or built differently. It's a standalone measurement program (`bench/`, no dependency on
the `image` crate), extending candidate 047's harness with one new variant rather than starting a
new one, per this skill's "Bench harnesses: grow a few, don't multiply".

## What was tried

Base commit referenced (context only, not built/patched): `44ce9226c541dd2b11897b9ef07156a28871cdb7`
(same tree as candidates 037/041/042/046/047 - unmodified, `git status` confirmed clean before and
after this candidate).

Extended candidate 047's `bench/src/main.rs` (kept every prior function - `fill_store_natural`
through `fill_unpack_volatile_specialized_rowclosure` - unchanged, re-measured this session rather
than reused from a prior session, per `MEASUREMENT.md`'s paired-comparison discipline) with one new
function:

- `fill_unpack_volatile_specialized_rowclosure_boundschecked::<DENSITY>` - byte-for-byte identical
  to `fill_unpack_volatile_specialized_rowclosure` (same store via `ChunksExactMut<u32>` +
  `Option`-returning `.next()`, same per-row closure through `with_rows_like`, same unpack
  arithmetic), except the palette lookup: `palette.get_unchecked(idx as usize)` into a
  `&[u32; 256]` **fixed-size array** becomes plain `palette[idx as usize]` into a `&[u32]`
  **unsized slice** - a real, checked index that panics on out-of-range, matching the real
  decoder's `palette[*idx as usize]` into `palette: &[[u8; 3]]` in
  `src/codecs/bmp/decoder.rs`'s `set_8bit_pixel_run`/`set_4bit_pixel_run`/`set_2bit_pixel_run`/
  `set_1bit_pixel_run`. The function is `#[inline(never)]` and takes `palette: &[u32]` as a
  parameter (not a fixed-size array), so the compiler cannot statically know the slice's length at
  the point the bounds check is emitted - the same situation the real decoder is in, since its own
  palette length comes from the BMP file's color table at runtime, not a compile-time constant.
  This isolates only the bounds check itself: store/iterator-machinery/row-closure/unpack
  arithmetic are all held fixed at candidate 047's own `_rowclosure` shape.

Correctness of the new variant is verified on every run (not just assumed): extended candidate
047's `verify_iterator_variants_correctness()` (still run unconditionally at binary startup) with a
third comparison against the same from-scratch reference extraction, for all 4 densities, on the
same 3-row / 32-bytes-per-row test buffer as the `_chunksmut`/`_rowclosure` checks. All four run
logs (`evidence/{single,unpinned}_run{1,2}.txt`) contain both `verify_unpack_correctness: OK` and
`verify_iterator_variants_correctness: OK` lines (the latter's message updated to name all three
variants it now covers).

Same methodology as candidates 041/042/046/047: 2 buffer-size regimes (`small` = 153,560 pixels
matching `Core_8_Bit.bmp`/`Core_4_Bit.bmp`'s 435x353 scale; `large` = 4032x3024 = 12,192,768
pixels), all 44 configs (4 densities x 11 variants) measured in true round-robin, 30 rounds, ~10%
warmup discarded, `min_ns` across 4 runs (2x `taskset -c 3`, 2x unpinned) as the stable metric.
`uptime` was 0.21/1.92/2.37 (1/5/15-min load) for all runs, well under the 24-core budget. Raw
output: `evidence/single_run{1,2}.txt`, `evidence/unpinned_run{1,2}.txt`.

`objdump -d -C` (`evidence/objdump_full.txt`) confirms the new variant's codegen actually contains
a real bounds check: `core::panicking::panic_bounds_check` appears as a distinct called symbol
(`0x1b13b`), reached from the `with_rows_like::<...rowclosure_boundschecked<N>::{closure#0}>`
functions for `N`=1,2,4 (confirmed via a backwards `objdump` scan from the `panic_bounds_check`
symbol to the nearest preceding function labels) - `fill_unpack_volatile_specialized_rowclosure`'s
own closures (the fixed-array/`get_unchecked` floor) have no such call. `nm --print-size`
(`evidence/nm_sizes.txt`) shows every `_rowclosure_boundschecked::<N>` closure is larger than its
`_rowclosure::<N>` counterpart: `N`=1: 0x1ac (428B) vs 0x16f (367B), `N`=2: 0x1f8 (504B) vs 0x1b3
(435B), `N`=4: 0x267 (615B) vs 0x22e (558B), `N`=8: 0x392 (914B) vs 0x31d (797B) - a consistent
+55-117 byte growth from the added bounds-check instructions, tracking the added `panic_bounds_check`
call plus its comparison/branch.

Best-min ns/pixel per config, `small` scale (matching candidate 037's real-decode corpus scale) and
`large` scale (this workspace's usual 4032x3024 "realistic photo" scale) - full per-run numbers in
`evidence/`:

| density | scale | `_rowclosure` floor (047) | `_rowclosure_boundschecked` (this candidate) | delta | delta as % of floor |
|---|---|---|---|---|---|
| 1 | small | 0.2872 | 0.3340 | +0.0468 | +16.3% |
| 1 | large | 0.3088 | 0.3531 | +0.0442 | +14.3% |
| 2 | small | 0.2922 | 0.3465 | +0.0543 | +18.6% |
| 2 | large | 0.2959 | 0.3475 | +0.0516 | +17.5% |
| 4 | small | 0.2900 | 0.3350 | +0.0450 | +15.5% |
| 4 | large | 0.2896 | 0.3343 | +0.0447 | +15.4% |
| 8 | small | 0.3317 | 0.3606 | +0.0289 | +8.7% |
| 8 | large | 0.3324 | 0.3618 | +0.0294 | +8.9% |

`small`/`large` figures agree closely at every density (within 0.2-1.4 percentage points of each
other), consistent with candidate 047's own observation that this line's numbers are not
scale-sensitive.

## Result

**A real, measurable, paired-comparison cost: the bounds check itself (checked-slice indexing vs.
`get_unchecked` into a fixed-size array, everything else held identical) adds +0.029 to +0.054
ns/pixel, i.e. +8.7% to +18.6% on top of candidate 047's own `_rowclosure` floor, largest at
density 2 (4-bit) and smallest at density 8 (1-bit) - not monotonic with bit depth, similar in
shape to candidate 047's own iterator-machinery finding (also non-monotonic, also largest away from
the extremes).**

Translated to "share of real BMP decode time" using candidate 037's own real-decode ns/px numbers
(1.0357 / 1.0844 / 0.8698 / 0.8337 ns/px at 8/4/2/1-bit respectively) and this candidate's own
`small`-scale delta:

| Bit depth | Density | Bounds-check delta (ns/px) | As % of real decode time |
|---|---|---|---|
| 8-bit | 1 | +0.0468 | +4.5% |
| 4-bit | 2 | +0.0543 | +5.0% |
| 2-bit | 4 | +0.0450 | +5.2% |
| 1-bit | 8 | +0.0289 | +3.5% |

Adding this to candidate 047's own running total (store+lookup+unpack+iterator-machinery = 27.5%-
41.1% of real decode time at 8/4/2/1-bit) gives an **additive** combined estimate of
**32.0%-46.3%** of real decode time now accounted for across store+lookup+unpack+iterator-
machinery+bounds-check, leaving a residual of **53.7%-68.0%** still unexplained at every bit depth
- narrower than candidate 047's own 58.9%-72.5% residual, but still a majority at every bit depth
measured. (8-bit: 37.8%+4.5%=42.3%, residual 57.7%. 4-bit: 27.5%+5.0%=32.5%, residual 67.5%.
2-bit: 34.3%+5.2%=39.5%, residual 60.5%. 1-bit: 41.1%+3.5%=44.6%, residual 55.4%.)

**This additive combination should be read with one explicit caveat**: it sums candidate 047's own
floor-explained percentage (measured in a *different* session) with this candidate's own delta
percentage (measured *this* session), the same convention every candidate in this line
(041/042/046/047) has used to build up a running total. This session's own re-measurement of the
*unchanged* `_rowclosure` floor function (density 1, `small` scale: 0.2872 ns/px this session vs.
0.3910 ns/px recorded in candidate 047's own `RESULT.md`) differs by about 26% - a larger
session-to-session drift than at the other three densities (which differ by roughly 2-13%). Because
the bounds-check *delta* used above is a same-session, paired comparison against this session's own
`_rowclosure` re-measurement, the delta itself is not affected by this drift, but the additive
combination with 047's own explained-percentage figure inherits whatever uncertainty that drift
implies for cross-session comparisons in this line generally (see "What's not done" below).

## Does it change behavior/output?

**Measurement-only investigation - N/A, no patch applied to the tested project's own source.**
`git status` in the main project's working tree was confirmed clean (only `___OPTIMIZATIONS___/`/
`___OTHER___/` untracked) both before and after this candidate - there is correctly **no
`patch.diff`** in this folder; see `SKILL.md`'s "`patch.diff` is mandatory whenever the fix is a
change to the tested project" section for why a measurement-only candidate like this one is one of
the two valid states with no patch.

- **Bit-exact?** N/A - no source change. The synthetic bench's own new-variant correctness is
  verified per-pixel against a from-scratch reference on every run (see above); all four runs
  printed both `verify_unpack_correctness: OK` and `verify_iterator_variants_correctness: OK`.
- **Is this a breaking change?** N/A - no source change.
- Project test suite (`cargo test --release`, unmodified `main` tree at the same commit as
  candidates 041/042/046/047, run once since there is no before/after to compare - per `SKILL.md`'s
  explicit exception for a pure investigation with no patch, this run was extra rigor, not a strict
  requirement): **550 passed / 1 failed**, the same single pre-existing failure every prior
  BMP-decoder-related candidate in this workspace has recorded (`resizing_with_catmul` in
  `tests/regression.rs` - a pre-existing, unrelated CatmullRom-filter regression test failure).
  Matches candidates 041/042/046/047's own 550/1 exactly (same tree, no fixture drift). Full log:
  `evidence/cargo_test.log`.

## Why the result looks the way it does

A checked slice index (`palette[idx]`) compiles to a comparison (`idx < palette.len()`), a
conditional branch to `core::panicking::panic_bounds_check` on failure, and then the same load the
`get_unchecked` version does on success - confirmed present in the new variant's disassembly and
absent from the floor's. In principle a well-predicted branch on modern hardware (the panic path
is never taken here - every extracted index is provably within `[0, 2^density)` and the palette is
256 entries) should cost very little; the measured +8.7% to +18.6% is larger than "just a
predicted branch" would suggest, and the density-2 (4-bit) case showing the largest relative cost
(not the extremes, density 1 or density 8) doesn't fit a single simple mechanism any more cleanly
than candidate 047's iterator-machinery finding did. Two plausible contributors, neither confirmed
by counter-level profiling (no `perf` in this container, per the same limitation as every other
candidate in this line):

1. **Code size / icache pressure.** The `nm --print-size` comparison shows a consistent 55-117
   byte growth per closure from the added bounds-check machinery - not free, though likely a small
   fraction of the measured delta at these iteration counts.
2. **Compiler backend effects from the panic call itself.** A function containing a call to a
   `#[cold]`-attributed panic path can see different register allocation/instruction scheduling
   decisions around that call site than a function with no such call, independent of whether the
   call is ever taken at runtime - not confirmed here by a full instruction-level diff of the two
   closures' hot loop bodies (only the presence/absence and size delta were checked, per the
   objdump summary above), left as an open question, same as candidate 047 left its own density-1
   codegen anomaly unresolved.

Combined with candidates 041/042/046/047, this closes the last per-pixel-loop hypothesis explicitly
named in this investigation line's own "What's not done" lists: store, lookup, unpack arithmetic,
row/iterator machinery, and now the bounds check itself have all been isolated with fair,
const-generic-specialized comparisons. Together they explain an estimated 32.0%-46.3% of real BMP
decode time (with the cross-session caveat above) - still leaving a majority of real decode time
unaccounted for at every bit depth. The only remaining named hypothesis in `IDEAS.md` for this line
is I/O/buffer-fill overhead upstream of the per-pixel loop (`read_scanline`'s own file reads),
which is structurally different from anything measured in this line so far (not a per-pixel-loop
cost at all).

## What's not done / open questions

- The additive "combined % of real decode time explained" figure reuses candidate 047's own
  floor-explained percentage from a *different* session, and this session's own re-measurement of
  the unchanged `_rowclosure` floor differs from 047's original number by up to ~26% at density 1
  (see "Result" above) - a full re-run of the *entire* 037/041/042/046/047/050 stack in one single
  session (rather than accumulating cross-session deltas) would remove this specific source of
  uncertainty, not attempted here as it would mean re-doing every prior candidate's own
  measurement, not just this one's new variant.
- No instruction-level, side-by-side read of `_rowclosure::<N>` vs. `_rowclosure_boundschecked::<N>`'s
  full hot-loop disassembly was done (only presence/absence of `panic_bounds_check` and overall
  symbol-size deltas) - would be needed to confirm which of the two "why" hypotheses above (icache
  pressure vs. codegen-scheduling effects around the panic call) actually explains the measured
  gap, or whether it's something else entirely.
- With store+lookup+unpack+iterator-machinery+bounds-check now all isolated and still only
  explaining an estimated 32.0%-46.3% of real decode time, the sole remaining un-isolated
  hypothesis for the residual is I/O/buffer-fill overhead upstream of the per-pixel loop
  (`read_scanline`'s own file reads in `src/codecs/bmp/decoder.rs`, called once per row) - already
  flagged in `IDEAS.md` from candidate 047's own session, not attempted here.
- Real hardware execution-port/IPC counters (`perf`) remain unavailable in this container (no
  `perf` binary, `sudo`/`apt` blocked) - the mechanistic explanations above are plausible and
  consistent with the measured shape, but not counter-confirmed.
- Only one machine/session was used for this candidate's own new measurements.
