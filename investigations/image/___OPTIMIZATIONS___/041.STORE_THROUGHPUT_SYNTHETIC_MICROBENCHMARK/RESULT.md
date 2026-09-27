<!--
Measurement-only investigation - no patch to the tested project's own source. See "Does it
change behavior/output?" below for the explicit N/A statement this workspace requires.
-->

# Synthetic store-throughput microbenchmark: does output-write throughput limit the BMP palette-decode branch-removal win?

Answers `IDEAS.md`'s top-of-backlog entry (hypothesis (b), left open by candidate 037): whether
output-write/store throughput becomes a limiting factor once enough bounds-check branches are
removed from the BMP palette decode path. `cachegrind` (candidate 037) could not test this - it
has no execution-port/IPC/store-buffer-contention model, only instruction and cache-hit/miss
counts. `perf` remains uninstallable in this container (no `perf` binary, `sudo`/`apt` blocked by
"no new privileges" - confirmed again across candidates 025/037). This candidate takes approach
(1) from `IDEAS.md`'s two suggested angles: a tight, controlled synthetic microbenchmark that
only writes N pixels/iteration (no palette lookup, no bounds check at all) at densities 1/2/4/8,
matching the BMP decoder's 8/4/2/1-bit outer-loop shapes, to measure an achievable *ceiling* for
pure store throughput at each density and compare it against the real decode's own measured
per-pixel cost (candidate 037's numbers) - this doesn't require `perf` at all, just wall-clock
timing of a purpose-built loop.

This is **not** a change to `image`'s own source - no code path in the real crate was touched,
patched, or built differently. It's a standalone measurement program (`bench/`, no dependency on
the `image` crate) whose only relationship to the tested project is measuring an upper bound the
real BMP decode loop's store cost can be compared against.

## What was tried

Base commit referenced (context only, not built/patched): `44ce9226c541dd2b11897b9ef07156a28871cdb7`.

Built a standalone Rust binary, `bench/src/main.rs` (no external dependencies, not part of the
`image` crate), with two `#[inline(never)]` fill functions:

- `fill_natural` - plain `ptr::write::<u32>`, LLVM is free to optimize/vectorize however it likes
  (an upper bound on achievable store throughput on this machine).
- `fill_volatile` - `ptr::write_volatile::<u32>`, forces one genuine, unmergeable 32-bit store
  instruction per pixel, closer to what the real BMP loop's data-dependent (palette-indexed)
  writes actually compile to (a runtime-data-dependent memory access defeats auto-vectorization
  the same way `volatile` does, just for a different reason).

Each fill function takes a runtime `density` parameter (1/2/4/8): the outer loop runs
`total_pixels / density` iterations, each iteration doing `density` consecutive 4-byte (u32,
RGBA-pixel-sized) stores before the loop counter/branch fires again - directly modeling how many
pixels the real BMP decoder writes per outer-loop iteration at each bit depth (8-bit: 1 input
byte -> 1 pixel/iteration; 4-bit: 2; 2-bit: 4; 1-bit: 8), with **no** palette lookup and **no**
bounds check anywhere in the loop, per `IDEAS.md`'s exact request. Two buffer-size regimes:
`small` (153,560 pixels, matching `Core_8_Bit.bmp`/`Core_4_Bit.bmp`'s 435x353 scale from
candidates 008/011/025/037, fits comfortably in L2/L3) and `large` (4032x3024 = 12,192,768
pixels ~= 48.8 MB, this workspace's usual "realistic photo" scale, exceeds typical L3). All 16
(density x variant x size) configs measured in true round-robin (every config once per round, 30
rounds, ~10% warmup discarded) per this workspace's own documented noise lesson (`IDEAS.md`,
candidate 009's session - back-to-back repeats of one variant can get caught inside a
machine-wide slow/fast "mode" that looks like a per-variant effect but isn't).

Ran the binary 4 times total (2x `taskset -c 3`, "single-core"; 2x unpinned, "multi-core" - this
program is single-threaded, so "multi-core" here means default/unpinned OS scheduling, not
thread-pool parallelism) to check repeatability. Raw output saved in `evidence/single_run1.txt`,
`evidence/single_run2.txt`, `evidence/unpinned_run1.txt`, `evidence/unpinned_run2.txt`.
`uptime` was 0.9-2.6 (well under the 24-core budget) for all four runs.

**`min_ns` across the 4 runs (most stable metric - `p50`/`mean` show this machine's
already-documented "mode lottery" noise, e.g. `small/d=1/volatile`'s `min` was consistent
[113996, 85121, 85752, 85141] across runs while `p50` swung 89530-114708; picking the best `min`
per config is the standard way to read this kind of throughput ceiling)**, converted to ns/pixel
and GB/s:

| size | density | variant | best min (ns) | ns/pixel | GB/s |
|---|---|---|---|---|---|
| small | 1 | natural | 91,032 | 0.5928 | 6.75 |
| small | 1 | volatile | 85,121 | 0.5543 | 7.22 |
| small | 2 | natural | 56,777 | 0.3697 | 10.82 |
| small | 2 | volatile | 56,627 | 0.3688 | 10.85 |
| small | 4 | natural | 42,610 | 0.2775 | 14.42 |
| small | 4 | volatile | 35,367 | 0.2303 | 17.37 |
| small | 8 | natural | 14,177 | 0.0923 | 43.33 |
| small | 8 | volatile | 28,324 | 0.1844 | 21.69 |
| large | 1 | natural | 7,223,808 | 0.5925 | 6.75 |
| large | 1 | volatile | 6,750,471 | 0.5536 | 7.22 |
| large | 2 | natural | 4,510,707 | 0.3699 | 10.81 |
| large | 2 | volatile | 4,513,442 | 0.3702 | 10.81 |
| large | 4 | natural | 3,948,521 | 0.3238 | 12.35 |
| large | 4 | volatile | 2,824,802 | 0.2317 | 17.27 |
| large | 8 | natural | 1,382,058 | 0.1134 | 35.29 |
| large | 8 | volatile | 2,277,345 | 0.1868 | 21.42 |

`objdump -d -C -M intel` (`evidence/objdump_fill_natural.txt`, `evidence/objdump_fill_volatile.txt`)
confirms the mechanism behind `natural`'s sharp jump at density 8: LLVM auto-vectorizes
`fill_natural`'s inner loop into 128-bit (`xmm`, 4x u32) `movdqu` stores whenever the runtime
`density` argument is a multiple of 8 (`rcx = rdx & !0x7`, `cmp rdx,0x8` gate) - at densities
1/2/4 it falls through to a scalar `mov DWORD PTR` tail loop instead, matching the much smaller
GB/s figures measured for those densities. `fill_volatile` never emits an `xmm`/`movdqu`
instruction at any density (confirmed by grepping the full disassembly) - it's always scalar
`mov DWORD PTR [...],eax`-style stores, unrolled by 4 for general loop efficiency but never
merged/widened, exactly as `volatile` is meant to guarantee.

## Result

**`small` and `large` produce essentially identical ns/pixel at every density/variant** (e.g.
density=1/natural: 0.5928 vs 0.5925 ns/px; density=4/volatile: 0.2303 vs 0.2317 ns/px) - this
loop is not memory-bandwidth-limited even at 48.8 MB (well outside L2/L3 on any consumer CPU).
What actually varies is per-store/per-outer-iteration overhead being amortized differently as
`density` grows, not DRAM bandwidth.

**Comparing this ceiling against the real BMP decode's own measured per-pixel cost (candidate
037's multi-core patched wall-clock numbers, same machine/session type, converted to ns/pixel:
8-bit 1.0357, 4-bit 1.0844, 2-bit 0.8698, 1-bit 0.8337)**, matching each bit depth to its
corresponding outer-loop density:

| Bit depth | Density | Real decode ns/px (candidate 037, whole decode incl. header/lookup/store) | Pure-store floor ns/px (natural / volatile) | Pure-store share of real time |
|---|---|---|---|---|
| 8-bit | 1 | 1.0357 | 0.593 / 0.554 | 57% / 53% |
| 4-bit | 2 | 1.0844 | 0.370 / 0.370 | 34% / 34% |
| 2-bit | 4 | 0.8698 | 0.278 / 0.230 | 32% / 26% |
| 1-bit | 8 | 0.8337 | 0.092 / 0.184 | 11% / 22% |

Pure output-store cost (with **zero** lookup, **zero** bounds check - the absolute floor) already
accounts for **roughly half of the real 8-bit decode's total per-pixel time** (53-57%), but a
**much smaller fraction (11-26%) of the real 1-bit decode's total per-pixel time**. This is a
genuinely new, quantitative data point candidate 037's cachegrind pass could not produce (it only
showed cache-hit-rate parity, not a store-cost ceiling comparison). It's real evidence *for*
hypothesis (b) at low density/high store-share bit depths (8-bit, and to a lesser extent 4-bit):
there is a hard, unavoidable store cost that already consumes a majority-to-large-minority share
of the total budget, leaving comparatively little room for *any* per-pixel fix (branch removal
included) to produce a large relative win - consistent with candidate 037's small, fairly
uniform 5-9% observed wall-clock improvement rather than something proportional to the "checks
eliminated per iteration" that first motivated this whole investigation line. It does **not**
cleanly explain why 1-bit's observed win (candidate 037: -4.2% to -8.5% multi-core) is roughly
the *same order of magnitude* as 8-bit's despite 1-bit having far more non-store headroom (74-89%
of its time is *not* pure store) - if store-throughput saturation were the *whole* story, 1-bit
would be expected to show a proportionally *larger* win than 8-bit (more non-store budget for a
fixed-instruction-count savings to eat into), not a similar one. So this result **partially, not
fully, supports hypothesis (b)**: it's real, quantified evidence that store cost is a
non-negligible, bit-depth-dependent share of the total budget (ruling out "stores are always a
tiny fraction, irrelevant" as a blanket dismissal), but it doesn't by itself explain the full
observed cross-depth win pattern - real hardware execution-port/IPC counters (`perf`, still
unavailable here) would still be needed to fully resolve this.

## Does it change behavior/output?

**Measurement-only investigation - N/A, no patch applied to the tested project's own source.**
`git status` in the main project's working tree was never touched by this candidate (verified
before and after: only `___OPTIMIZATIONS___/`/`___OTHER___/` untracked, no modified files) - there
is correctly **no `patch.diff`** in this folder; see `SKILL.md`'s "`patch.diff` is mandatory
whenever the fix is a change to the tested project" section for why a measurement-only candidate
like this one is one of the two valid states with no patch.

- **Bit-exact?** N/A - no source change.
- **Is this a breaking change?** N/A - no source change.
- Project test suite (`cargo test --release`, unmodified `main` tree, run once since there is no
  before/after to compare): **550 passed / 1 failed**, the same single pre-existing failure every
  prior BMP-decoder-related candidate in this workspace has recorded
  (`resizing_with_catmul` in `tests/regression.rs` - a pre-existing, unrelated CatmullRom-filter
  regression test failure, not caused by this or any prior candidate). Matches candidates
  008/011/016/019/020/025/037's own recorded counts (candidate 037 reported 551/1 with
  candidate 020's extra `pal2_large.bmp` test fixture copied in for that session only; this
  session's tree doesn't have that extra fixture, hence 550 vs 551 - same single failure either
  way). Full log: `evidence/cargo_test.log`.

## Why the result looks the way it does

See "Result" above. In short: this microbenchmark gives a genuine, wall-clock-measured *ceiling*
for pure store cost (no lookup, no branch) at each of the BMP decoder's four outer-loop
densities, and that ceiling is a bit-depth-dependent fraction of the real decode's total
per-pixel time (roughly half at 8-bit, a small fraction at 1-bit) - because a fixed per-outer-
iteration loop overhead (counter increment, back-edge branch) is amortized over more stores as
density grows, so the *effective* per-pixel store cost drops sharply from density 1 to density 8,
while the corresponding real decode's *non-store* work (palette lookup, bit-field unpacking to
extract multiple sub-pixel indices from one input byte) presumably grows in the opposite
direction (1-bit needs 8x more bit-extraction work per input byte than 8-bit does) - the two
effects partially offset, which is a plausible structural reason the real, whole-decode ns/pixel
numbers (0.83-1.08 across all four depths, candidate 037) stay in a much *tighter* band than
either the pure-store ns/pixel numbers (0.09-0.59) or a naive "checks removed per iteration"
count (1/2/4/8) would suggest on their own.

## What's not done / open questions

- This still doesn't have real hardware execution-port/IPC/store-buffer-occupancy counters -
  `perf` remains uninstallable in this container. The store-share-of-total-time table above is a
  wall-clock-only proxy; it's suggestive, not a hardware-counter-confirmed attribution of *why*
  the observed cross-depth win pattern (roughly uniform 5-9%, not proportional to store share)
  looks the way it does.
- The "real decode ns/pixel" column reuses candidate 037's own numbers (same repo/commit, but a
  different session) rather than being re-measured in the same session as this microbenchmark -
  the two are comparable (both multi-core, unpinned, this workspace's usual paired-comparison
  discipline) but weren't run back-to-back on the same machine-state.
- A version of this microbenchmark that also includes a cheap array-index lookup (still no bounds
  check, e.g. a fixed small palette array indexed by a monotonic counter rather than a literal
  value) was not built - it would show how much *additional* time the lookup itself (as opposed
  to the store) costs at each density, closing the gap between "pure store ceiling" and "real
  whole-decode time" one component at a time rather than leaving everything non-store lumped
  together as one residual.
- Only one machine/session was used; per this workspace's own documented noise characteristics
  (`IDEAS.md`, several rows), a from-scratch re-run on a different day/machine-state would
  strengthen confidence that the store-share percentages above aren't an artifact of this
  particular session's cache/frequency state.
