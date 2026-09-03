<!--
Measurement-only investigation - no patch to the tested project's own source. See "Does it
change behavior/output?" below for the explicit N/A statement this workspace requires.
-->

# Lookup-inclusive synthetic store-throughput microbenchmark: does the palette lookup itself explain the BMP decode's non-store residual?

Follow-up to candidate 041, answering the "What's not done" item left open there and the
top-of-backlog `IDEAS.md` entry it produced: candidate 041's synthetic microbenchmark isolated
only the store cost of the BMP palette-decode per-pixel loop (no palette lookup, no bounds
check), leaving palette lookup + bit-field unpacking + the bounds check itself lumped together as
one residual against the real whole-decode time (candidate 037's measured numbers). This
candidate adds a cheap, still branch-free array-index lookup on top of the same store, at the
same four outer-loop densities (1/2/4/8, matching the BMP decoder's 8/4/2/1-bit outer-loop
shapes), to isolate the lookup's own added cost from the store's - the specific experiment
`IDEAS.md`'s entry asked for.

This is **not** a change to `image`'s own source - no code path in the real crate was touched,
patched, or built differently. It's a standalone measurement program (`bench/`, no dependency on
the `image` crate), extending candidate 041's harness with two new fill functions rather than
starting a new one, per this skill's "Bench harnesses: grow a few, don't multiply".

## What was tried

Base commit referenced (context only, not built/patched): `44ce9226c541dd2b11897b9ef07156a28871cdb7`
(same working tree state as candidate 041 - unmodified, `git status` confirmed clean before and
after this candidate).

Extended candidate 041's `bench/src/main.rs` (kept `fill_store_natural`/`fill_store_volatile` -
functionally identical to 041's `fill_natural`/`fill_volatile`, re-measured in this same session
rather than reusing 041's numbers across sessions, per `MEASUREMENT.md`'s paired-comparison
discipline) with two new `#[inline(never)]` functions:

- `fill_lookup_natural` / `fill_lookup_volatile` - same store (plain `ptr::write` / `ptr::write_volatile`
  respectively), but the value written is read from `palette[idx as usize]` first: `palette` is a
  fixed `[u32; 256]` array (non-trivial values, `wrapping_mul`/xor so LLVM can't fold it to a
  constant), `idx` a wrapping `u8` counter incremented once per pixel. `idx` never needs
  range-clamping (`u8::wrapping_add` already stays in `0..256`), so `palette.get_unchecked(idx as
  usize)` is used purely to force zero bounds-check-branch codegen, not because the index could
  otherwise go out of range - matching `IDEAS.md`'s "still branch-free" requirement exactly.

Same methodology as candidate 041: 2 buffer-size regimes (`small` = 153,560 pixels, matching
`Core_8_Bit.bmp`/`Core_4_Bit.bmp`'s 435x353 scale - the same scale candidate 037's real-decode
numbers below were measured on; `large` = 4032x3024 = 12,192,768 pixels, ~48.8 MB, this
workspace's usual "realistic photo" scale), all 16 configs (4 densities x 4 variants, now
`store_natural`/`store_volatile`/`lookup_natural`/`lookup_volatile`) measured in true round-robin,
30 rounds, ~10% warmup discarded, `min_ns` across 4 runs (2x `taskset -c 3`, 2x unpinned) as the
stable metric, same reasoning as 041. `uptime` was 0.07-0.68 for all four runs (well under the
24-core budget). Raw output: `evidence/single_run{1,2}.txt`, `evidence/unpinned_run{1,2}.txt`,
parsed best-min table in `evidence/parsed_best_min.json`.

`objdump -d -C -M intel` confirms the codegen mechanism (`evidence/objdump_fill_*.txt`):

- `fill_lookup_natural`/`fill_lookup_volatile` contain **no** panic/index/bounds-check call at
  any density (grepped for `panic`/`index` across both disassemblies - none found), confirming
  the lookup is genuinely branch-free as intended.
- `fill_store_volatile` and `fill_lookup_volatile` both auto-unroll their main loop by 4 whenever
  `density >= 4` (identical `and edx,0x3` / `cmp rdx,0x4` gating structure in both), and both fall
  back to an identical-shape 1-at-a-time scalar tail loop below that - i.e. the `volatile` variant
  pair is a fair, structurally-matched comparison at every density.
- `fill_store_natural` only auto-vectorizes (128-bit `movdqu`, `paddd`-based increment) when
  `density` is a multiple of 8, matching candidate 041's finding - but `fill_lookup_natural`
  **also** auto-vectorizes at density 8 (via a *different* mechanism: since consecutive `idx`
  values index consecutive `palette` slots, LLVM emits `movups` loads of 4 contiguous palette
  entries plus a runtime guard, `cmp rax,0x100`/`setae`, that falls back to scalar whenever the
  8-wide window would wrap past the 256-entry palette) **and additionally gets a 4-wide unrolled
  scalar path at density 4 that `fill_store_natural`'s tail loop does not get** (visible in
  `evidence/objdump_fill_lookup_natural.txt` lines ~44-79 vs `evidence/objdump_fill_store_natural.txt`'s
  1-at-a-time tail loop lines ~53-59). This is a genuine, unexpected finding on its own: adding
  the lookup did not just add cost at density 4 - it incidentally triggered *better* LLVM codegen
  for that density than the pure-store baseline gets, an artifact of the two hand-written loops
  being optimized independently by the compiler, not a property of "lookup cost" in general. This
  confound is exactly why the `volatile` variant pair (which does NOT show this asymmetry - both
  get the same 4-wide unrolling) is used as the primary comparison below, with `natural`
  reported alongside for transparency rather than treated as equally trustworthy.

Best-min ns/pixel per config (`small` scale, matching candidate 037's real-decode corpus scale):

| density | store_natural | store_volatile | lookup_natural | lookup_volatile |
|---|---|---|---|---|
| 1 | 0.5894 | 0.5519 | 0.6134 | 0.5521 |
| 2 | 0.3680 | 0.3680 | 0.4600 | 0.4329 |
| 4 | 0.2760 | 0.2300 | 0.2761 | 0.2304 |
| 8 | 0.0923 | 0.1841 | 0.0925 | 0.2073 |

(`large` scale numbers are consistent with `small` at every density/variant, same as candidate
041 found - not memory-bandwidth-limited; full table in `evidence/parsed_best_min.json`.)

## Result

**The lookup's own added cost (`lookup_volatile` minus `store_volatile`, the structurally-fair
comparison) is small and does not grow monotonically with bit depth/density:**

| Bit depth | Density | Real decode ns/px (candidate 037) | Store floor (volatile, this session) | Store+lookup floor (volatile) | Lookup's own added cost | Lookup as % of real decode time |
|---|---|---|---|---|---|---|
| 8-bit | 1 | 1.0357 | 0.5519 (53.3%) | 0.5521 (53.3%) | +0.0002 | +0.0% |
| 4-bit | 2 | 1.0844 | 0.3680 (33.9%) | 0.4329 (39.9%) | +0.0649 | +6.0% |
| 2-bit | 4 | 0.8698 | 0.2300 (26.4%) | 0.2304 (26.5%) | +0.0004 | +0.0% |
| 1-bit | 8 | 0.8337 | 0.1841 (22.1%) | 0.2073 (24.9%) | +0.0232 | +2.8% |

(`large`-scale figures tell the same story: lookup adds +0.1%/+3.6%/+1.2%/+3.5% respectively -
same small-and-non-monotonic shape, not a clean function of density.)

Even **including** the lookup, `store+lookup` together still explain only 53%, 40%, 27%, and 25%
of the real decode's per-pixel time at 8/4/2/1-bit respectively - almost unchanged from
store-alone's 53%/34%/26%/22% (candidate 041's original finding, re-confirmed this session). The
lookup itself contributes at most a few percentage points (0-6%) at any density, and its
contribution does **not** grow disproportionately at higher bit depths/higher densities - if
anything the largest relative addition (6.0%/3.6%) is at density 2 (4-bit), not density 8 (1-bit),
which is the opposite of "grows with density" and inconsistent with a clean per-store-instruction
cost story. This is the same-session, structurally-matched (`volatile`-vs-`volatile`) answer to
`IDEAS.md`'s exact question.

## Does it change behavior/output?

**Measurement-only investigation - N/A, no patch applied to the tested project's own source.**
`git status` in the main project's working tree was confirmed clean (only `___OPTIMIZATIONS___/`/
`___OTHER___/` untracked) both before and after this candidate - there is correctly **no
`patch.diff`** in this folder; see `SKILL.md`'s "`patch.diff` is mandatory whenever the fix is a
change to the tested project" section for why a measurement-only candidate like this one is one of
the two valid states with no patch.

- **Bit-exact?** N/A - no source change.
- **Is this a breaking change?** N/A - no source change.
- Project test suite (`cargo test --release`, unmodified `main` tree at the same commit as
  candidate 041, run once since there is no before/after to compare - per `SKILL.md`'s explicit
  exception for a pure investigation with no patch, this run was extra rigor, not a strict
  requirement): **550 passed / 1 failed**, the same single pre-existing failure every prior
  BMP-decoder-related candidate in this workspace has recorded (`resizing_with_catmul` in
  `tests/regression.rs` - a pre-existing, unrelated CatmullRom-filter regression test failure).
  Matches candidate 041's own 550/1 exactly (same tree, no fixture drift this session). Full log:
  `evidence/cargo_test.log`.

## Why the result looks the way it does

The `volatile`-pair comparison (the fair one, since both functions get identical 4-wide-unrolling
codegen at every density) shows the lookup itself - a single `get_unchecked` load from a 1 KiB,
L1-resident array - costs at most a few percent of real decode time, and that cost doesn't track
density/bit-depth in any clean way (largest at density 2, near-zero at density 1 and 4, small
again at density 8). A single extra load with no data dependency on the previous iteration's
value is cheap for an out-of-order CPU to hide behind existing store/loop-branch latency at most
densities; where it doesn't fully hide (density 2, and to a lesser extent density 8) is plausibly
about how much spare instruction-level-parallelism headroom that density's own loop shape already
has, not about the *density* itself.

The practical implication for `IDEAS.md`'s original question: **the palette lookup is not the
missing piece.** Store-alone (candidate 041) explained 53%/34%/26%/22% of real per-pixel decode
time at 8/4/2/1-bit; adding the lookup only moves those numbers to 53%/40%/27%/25% - a large
residual (47%/60%/73%/75%) remains completely unaccounted for at every bit depth, and that
residual actually **grows** as bit depth drops, the opposite of what "lookup cost dominates the
low-bit-depth residual" would predict. That residual must therefore be dominated by something
neither this candidate nor 041 modeled: the bit-field unpacking arithmetic that extracts multiple
sub-pixel indices from one input byte (which does grow with bit depth: 1-bit needs 8x more
per-input-byte extraction work than 8-bit), the bounds-check branch itself (deliberately excluded
from both microbenchmarks so far, per `IDEAS.md`'s "still branch-free" framing), and/or per-
input-byte outer-loop overhead shapes this pixel-count-driven synthetic loop doesn't reproduce
exactly. This sharpens candidate 041's "loop overhead amortization vs. bit-extraction work"
explanation by ruling out one specific candidate (the lookup itself) as the dominant unexplained
component, rather than replacing it outright.

## What's not done / open questions

- The bit-field unpacking itself (extracting N sub-pixel indices from one input byte via
  shift/mask, the actual majority-share suspect per the reasoning above) still has no isolated
  microbenchmark - a natural next follow-up: add a third variant per density that does the
  shift/mask extraction (still no real bounds check) on top of store, to see how much of the
  remaining 47-75% residual that alone accounts for.
- The bounds check itself (the actual branch/panic-path overhead, as opposed to just being
  branch-free code) is still never isolated in either this or candidate 041's microbenchmark -
  both are deliberately built to have *zero* bounds-check branches, so neither can speak to what
  the branch itself would cost if present (only to what removing it saves, indirectly, via
  candidate 037's real before/after wall-clock delta).
- Real hardware execution-port/IPC/store-buffer-occupancy counters (`perf`) remain unavailable in
  this container (no `perf` binary, `sudo`/`apt` blocked) - the "why doesn't the lookup's added
  cost track density monotonically" explanation above (ILP headroom varying by density) is
  plausible but not counter-confirmed.
- As with candidate 041, the "real decode ns/pixel" column reuses candidate 037's own numbers
  (same repo/commit, different session) rather than being re-measured in the same session as this
  microbenchmark - comparable methodology (multi-core, unpinned) but not run back-to-back on the
  same machine-state.
- Only one machine/session was used for this candidate's own new measurements; a from-scratch
  re-run on a different day/machine-state would strengthen confidence in the non-monotonic shape
  of the lookup's added cost, since it's a small enough effect (single-digit percent) that this
  session's own noise floor is a real consideration.
