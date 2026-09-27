<!--
Measurement-only investigation - no patch to the tested project's own source. See "Does it
change behavior/output?" below for the explicit N/A statement this workspace requires.
-->

# Bit-unpack-inclusive synthetic store+lookup microbenchmark: does the bit-field extraction arithmetic explain the BMP decode's residual?

Follow-up to candidate 042, answering the exact "What's not done" item it left open and the
top-of-backlog `IDEAS.md` entry it produced: candidate 042 found the palette lookup itself
explains only a small (0-6%), non-monotonic slice of real BMP decode time, leaving a residual
(47-75% of real time, growing at lower bit depths) still fully unexplained. This candidate adds
the bit-field unpacking arithmetic itself - extracting N sub-pixel palette indices from one input
byte via shift/mask, still no real bounds check - on top of the same store+lookup loop, at the
same four densities (1/2/4/8, matching the BMP decoder's 8/4/2/1-bit outer-loop shapes), to
isolate the extraction's own added cost.

This is **not** a change to `image`'s own source - no code path in the real crate was touched,
patched, or built differently. It's a standalone measurement program (`bench/`, no dependency on
the `image` crate), extending candidate 042's harness with new fill functions rather than starting
a new one, per this skill's "Bench harnesses: grow a few, don't multiply".

## What was tried

Base commit referenced (context only, not built/patched): `44ce9226c541dd2b11897b9ef07156a28871cdb7`
(same tree as candidates 041/042 - unmodified, `git status` confirmed clean before and after this
candidate).

Extended candidate 042's `bench/src/main.rs` (kept `fill_store_natural`/`fill_store_volatile`/
`fill_lookup_natural`/`fill_lookup_volatile` unchanged, re-measured this session rather than reused
from a prior session, per `MEASUREMENT.md`'s paired-comparison discipline) with new functions:

- `fill_unpack_natural` / `fill_unpack_volatile` - same store+lookup as candidate 042's
  `fill_lookup_*`, but the palette index for each sub-pixel is now extracted from a shared
  input-byte buffer via `extract_index(byte, density, d)`, using **exactly** the shift/mask
  patterns the real decoder uses per bit depth (`src/codecs/bmp/decoder.rs`
  `set_8bit_pixel_run`/`set_4bit_pixel_run`/`set_2bit_pixel_run`/`set_1bit_pixel_run` - density 1:
  the whole byte is the index; density 2: `byte >> 4` / `byte & 0xf`; density 4: `(byte >> (6 -
  2*d)) & 0x3`; density 8: `(byte >> (7 - d)) & 0x1`, functionally identical to the real decoder's
  `(idx & bit) != 0` bit-test form). `density` is a **runtime** parameter here, same as candidate
  042's functions.
- `fill_lookup_volatile_specialized<const DENSITY: usize>` / `fill_unpack_volatile_specialized<const
  DENSITY: usize>` - added *after* an initial run showed a large, suspicious discrepancy (see
  below): the same lookup-only and unpack loops as above, but with density as a **compile-time
  const generic** instead of a runtime parameter, matching how the real decoder actually dispatches
  (one dedicated function per bit depth, chosen once for the whole row - never re-checked per
  byte). This pair is the structurally-fair, apples-to-apples comparison used for the primary
  result below; the runtime-density pair is kept and reported too, as a documented methodological
  cautionary finding (see "Why the result looks the way it does").

**Correctness of the extraction itself is verified on every run**, not just assumed: a
`verify_unpack_correctness()` function runs unconditionally at binary startup, comparing every
output pixel (not just a checksum) of `fill_unpack_natural`/`fill_unpack_volatile`/
`fill_unpack_volatile_specialized` against a from-scratch reference extraction, for all 4
densities, on a buffer size (97 input bytes) deliberately not a multiple of any density to catch
tail bugs. All four run logs (`evidence/{single,unpinned}_run{1,2}.txt`) contain the printed
`verify_unpack_correctness: OK` line confirming this passed every time.

Same methodology as candidates 041/042: 2 buffer-size regimes (`small` = 153,560 pixels matching
`Core_8_Bit.bmp`/`Core_4_Bit.bmp`'s 435x353 scale - the scale candidate 037's real-decode numbers
below were measured on; `large` = 4032x3024 = 12,192,768 pixels), all configs measured in true
round-robin, 30 rounds, ~10% warmup discarded, `min_ns` across 4 runs (2x `taskset -c 3`, 2x
unpinned) as the stable metric. `uptime` was 9.7-14.3 for all runs (well under the 24-core budget).
Raw output: `evidence/single_run{1,2}.txt`, `evidence/unpinned_run{1,2}.txt`, parsed best-min table
in `evidence/parsed_best_min.json`.

`objdump -d -C -M intel` (`evidence/objdump_full.txt`) confirms: `fill_lookup_volatile_specialized`
and `fill_unpack_volatile_specialized` contain **no** panic/index call anywhere (grepped `panic`/
`index` - none found), i.e. fully branch-free extraction and lookup as intended. The runtime-density
`fill_unpack_natural`/`fill_unpack_volatile` **do** each retain one `core::panicking::panic` call
(the unreachable `_ => unreachable!()` arm of `extract_index`'s match on a non-const `density`,
which LLVM cannot prove unreachable since it isn't compile-time known) - never executed given how
`main()` calls these functions (only 1/2/4/8), but its presence is one concrete, visible piece of
the extra codegen complexity the runtime-density version carries that the specialized version does
not.

Best-min ns/pixel per config (`small` scale, matching candidate 037's real-decode corpus scale):

| density | store_volatile | lookup_volatile | lookup_volatile_specialized | unpack_volatile (runtime density) | unpack_volatile_specialized |
|---|---|---|---|---|---|
| 1 | 0.5898 | 0.5903 | 0.1977 | 0.9248 | 0.2339 |
| 2 | 0.4912 | 0.4319 | 0.1967 | 0.6907 | 0.2635 |
| 4 | 0.2458 | 0.2457 | 0.1435 | 0.6404 | 0.2511 |
| 8 | 0.1966 | 0.2212 | 0.1447 | 0.5161 | 0.2690 |

(`large`-scale figures tell a broadly consistent story - full table in
`evidence/parsed_best_min.json`; see "Why the result looks the way it does" for the one place they
diverge in magnitude.)

## Result

**Isolated fairly (both const-generic-specialized, so neither number carries a dispatch-overhead
confound), the bit-unpack arithmetic's own added cost over lookup alone is small in absolute terms
but - unlike candidate 042's lookup-only finding - grows monotonically with bit depth:**

| Bit depth | Density | Real decode ns/px (candidate 037) | Lookup floor, specialized | Store+lookup+unpack floor, specialized | Unpack's own added cost | Unpack as % of real decode time |
|---|---|---|---|---|---|---|
| 8-bit | 1 | 1.0357 | 0.1977 (19.1%) | 0.2339 (22.6%) | +0.0362 | +3.5% |
| 4-bit | 2 | 1.0844 | 0.1967 (18.1%) | 0.2635 (24.3%) | +0.0668 | +6.2% |
| 2-bit | 4 | 0.8698 | 0.1435 (16.5%) | 0.2511 (28.9%) | +0.1076 | +12.4% |
| 1-bit | 8 | 0.8337 | 0.1447 (17.4%) | 0.2690 (32.3%) | +0.1243 | +14.9% |

Even with unpacking included, `store+lookup+unpack` together explain only 22.6% / 24.3% / 28.9% /
32.3% of real decode time at 8/4/2/1-bit respectively - a residual of 77.4% / 75.7% / 71.1% / 67.7%
remains unaccounted for at every bit depth. The residual **shrinks** slightly as bit depth drops
(the opposite of candidate 042's headline finding about the *combined* store+lookup floor, but
consistent with it once decomposed: unpack's own contribution is the one component in this whole
037/041/042/046 line whose share actually grows with bit depth, exactly matching the hypothesis
`IDEAS.md`'s entry asked to test).

**A second, cautionary finding:** the naive version of this same unpack loop - `density` as a
runtime parameter rather than a compile-time constant, the way one might casually write this
microbenchmark, but *not* how the real decoder is structured (which uses one dedicated function
per bit depth, never re-deciding the shift/mask pattern per byte) - measures dramatically higher:
`unpack_volatile`'s 0.5161-0.9248 ns/px is 62-89% of real decode time on its own, which would
naively look like "unpacking basically explains the whole gap." `objdump` shows why: LLVM cannot
prove `density` is always 1/2/4/8 when it's a runtime `usize`, so it keeps a genuine (if
predictable, once-per-byte, never-actually-taken) dispatch branch and a residual `unreachable!()`
panic call that the const-generic specialized version's monomorphization eliminates entirely. This
gap (0.92 vs 0.23 ns/px at density 1, a 4x difference for logically identical extraction/lookup/store
work) is a bigger effect than the phenomenon being measured, and would have been reported as "the
gap is closed" without the specialized-variant cross-check.

## Does it change behavior/output?

**Measurement-only investigation - N/A, no patch applied to the tested project's own source.**
`git status` in the main project's working tree was confirmed clean (only `___OPTIMIZATIONS___/`/
`___OTHER___/` untracked) both before and after this candidate - there is correctly **no
`patch.diff`** in this folder; see `SKILL.md`'s "`patch.diff` is mandatory whenever the fix is a
change to the tested project" section for why a measurement-only candidate like this one is one of
the two valid states with no patch.

- **Bit-exact?** N/A - no source change. The synthetic bench's own extraction correctness is
  verified per-pixel against a from-scratch reference on every run (see above); all four runs
  printed `verify_unpack_correctness: OK`.
- **Is this a breaking change?** N/A - no source change.
- Project test suite (`cargo test --release`, unmodified `main` tree at the same commit as
  candidates 041/042, run once since there is no before/after to compare - per `SKILL.md`'s
  explicit exception for a pure investigation with no patch, this run was extra rigor, not a strict
  requirement): **550 passed / 1 failed**, the same single pre-existing failure every prior
  BMP-decoder-related candidate in this workspace has recorded (`resizing_with_catmul` in
  `tests/regression.rs` - a pre-existing, unrelated CatmullRom-filter regression test failure).
  Matches candidates 041/042's own 550/1 exactly (same tree, no fixture drift). Full log:
  `evidence/cargo_test.log`.

## Why the result looks the way it does

Once dispatch overhead is controlled for (both lookup-only and unpack variants given the same
const-generic specialization treatment, so the comparison attributes only the added byte-load +
shift/mask work, not a difference in how well each gets compiled), the extraction arithmetic costs
a small, real, monotonically-growing amount: roughly a third of a nanosecond added at 1-bit depth
vs. essentially nothing at 8-bit depth (where no shift/mask is even needed - the byte *is* the
index). This tracks a simple mechanistic story: extracting 8 sub-pixels from one byte at 1-bit
depth requires 8 separate (fully unrolled, compile-time-constant-shift) extract-lookup-store
sequences per input byte, and while the byte *load* itself is amortized across all 8 pixels, the
extraction and lookup work is not - it's paid once per sub-pixel regardless of bit depth. That is
exactly the "N times more per-input-byte extraction work at low bit depths" mechanism the original
`IDEAS.md` entry hypothesized, now measured directly and confound-checked rather than inferred.

The magnitude is nonetheless too small to close the gap: even at 1-bit depth, where unpacking adds
its largest share (+14.9% of real decode time), 67.7% of real decode time remains unexplained by
store + lookup + unpack combined. Candidate 042 already ruled out the lookup itself as the missing
piece; this candidate rules out bit-unpacking arithmetic as the (sole) missing piece too, in the
same direct, isolated way. What remains unmodeled by this whole 037/041/042/046 line: the bounds
check itself (deliberately excluded from every microbenchmark in this line so far, per `IDEAS.md`'s
own "still branch-free" framing - candidate 037's real before/after wall-clock delta is the only
evidence available on this so far, indirect at best), the row/iterator machinery around
`set_Nbit_pixel_run` (`ChunksExactMut`, the outer per-row closure, `pixel_iter.next()`'s
`Option` handling - none of which this pixel-count-driven synthetic loop reproduces), and I/O /
buffer-fill overhead upstream of the per-pixel loop entirely.

The cautionary finding (naive runtime-density dispatch inflating the apparent cost 2.2-4x) is a
useful addition to this workspace's own methodology record: a microbenchmark's own accidental
lack of specialization can produce a number that looks like it answers the question being asked
while actually measuring something else (harness dispatch overhead, not real per-pixel work) -
worth checking whenever a synthetic microbenchmark's numbers approach the real thing's magnitude
suspiciously closely, per `MEASUREMENT.md`'s general "isolate confounds" discipline (a new,
project-independent instance of it worth keeping in mind, not itself added to `MEASUREMENT.md`
since it's a restatement of principles already there rather than a new one).

## What's not done / open questions

- The bounds check itself (the actual branch/panic-path overhead) is still never isolated in any
  microbenchmark in this line (037/041/042/046) - all are deliberately built branch-free. The only
  evidence on its cost remains candidate 037's real before/after wall-clock delta, indirect.
- The row/iterator overhead around the real `set_Nbit_pixel_run` functions (`ChunksExactMut`,
  `Option`-returning `pixel_iter.next()`, the outer row closure) has no isolated microbenchmark
  either - a plausible next specific component to try, following this same pattern.
- As with candidates 041/042, the "real decode ns/pixel" column reuses candidate 037's own numbers
  (same repo/commit, different session) rather than being re-measured in the same session as this
  microbenchmark.
- Real hardware execution-port/IPC counters (`perf`) remain unavailable in this container (no
  `perf` binary, `sudo`/`apt` blocked) - the mechanistic explanation above (per-sub-pixel
  extraction cost not amortized by the byte load) is plausible and consistent with the measured
  monotonic trend, but not counter-confirmed.
- Only one machine/session was used for this candidate's own new measurements; the `large`-scale
  numbers show the same qualitative monotonic trend but diverge somewhat in magnitude from `small`
  scale at density 8 (large: +0.0829 ns/px vs small: +0.1243 ns/px for unpack's own added cost) -
  a from-scratch re-run on a different day/machine-state would help confirm whether that's real
  scale-dependence or this session's own noise floor.
