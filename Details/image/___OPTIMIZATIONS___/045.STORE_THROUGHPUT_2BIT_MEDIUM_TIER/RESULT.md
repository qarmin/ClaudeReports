<!--
Measurement-only investigation - no patch to the tested project's own source. See "Does it
change behavior/output?" below for the explicit N/A statement this workspace requires.
-->

# Size-matched `medium` tier for the 2-bit row of the store-throughput comparison table

Answers `IDEAS.md`'s entry left by candidate 043: candidates 041/042/043's synthetic
store-throughput comparison table reused the `small` size tier (153,560px, matching the
8/4/1-bit sample files' actual size) for every bit depth's row, including 2-bit, even though the
real 2-bit corpus file (`pal2_large.bmp`, candidate 020) is 490,000px - over 3x larger. Candidate
041 already found `small` (153,560px) and `large` (12,192,768px) produce essentially identical
ns/pixel at every density, but that check never included a buffer exactly at 490,000px. This
candidate adds a size-matched `medium` (490,000px = 700x700, exactly `pal2_large.bmp`'s scale) tier
to candidate 041's own `bench/` harness (no new mechanism - one more buffer size in the existing
round-robin sweep) and re-runs it, to confirm scale-invariance also holds at exactly the size that
matters for the 2-bit row.

This is **not** a change to `image`'s own source - no code path in the real crate was touched,
patched, or built differently, the same as candidates 041/042. It's an extension to the existing
standalone measurement program (`bench/`, no dependency on the `image` crate).

## What was tried

Base commit referenced (context only, not built/patched): `44ce9226c541dd2b11897b9ef07156a28871cdb7`
(unchanged from candidates 020/025/037/041/042/043 - `HEAD` had not moved since those sessions).

Copied candidate 041's `bench/src/main.rs` verbatim and added a third size regime, `medium`
(`700 * 700 = 490,000` pixels, exactly `pal2_large.bmp`'s dimensions from candidate 020), alongside
the existing `small` (153,560px) and `large` (12,192,768px) tiers - same `fill_natural`/
`fill_volatile` loops, same 4 densities (1/2/4/8) x 2 variants, same 30-round true-round-robin
design, same `#[inline(never)]`/`black_box` discipline. This makes 24 (density x variant x size)
configs instead of candidate 041's 16. A persistent `medium_buf` is allocated once and reused
across rounds, the same as the existing `small_buf`/`large_buf`.

Built once (`cargo build --release`) and ran 4 times (2x `taskset -c 3` "single-core", 2x
unpinned "multi-core" - this program is single-threaded, so "multi-core" here means default OS
scheduling, not thread-pool parallelism), matching candidate 041's own repeat count. `uptime` was
11.3-11.8 throughout all four runs (this session's machine had a persistently higher background
load than candidates 037/041/043's sessions, which saw 0.1-2.6 - still comfortably under this
24-core machine's budget, so no re-baseline was needed per MEASUREMENT.md's "load > core count"
threshold, but it means this session's *absolute* ns/pixel numbers run noticeably higher across
every tier, including `small`, than candidates 037/041/043's - see "Why the result looks the way
it does" below). Raw output in `evidence/single_run{1,2}.txt`, `evidence/unpinned_run{1,2}.txt`.

**Best `min_ns` across the 4 runs (this workspace's standard way to read this kind of throughput
ceiling - see candidate 041), converted to ns/pixel, for all three size tiers at every density:**

| Density | small ns/px (natural/volatile) | medium ns/px (natural/volatile) | large ns/px (natural/volatile) | medium vs small delta |
|---|---|---|---|---|
| 1 | 1.4648 / 1.1618 | 1.4657 / 1.1650 | 1.4715 / 1.1688 | +0.06% / +0.28% |
| 2 | 0.8847 / 0.7609 | 0.8859 / 0.7628 | 0.8912 / 0.7677 | +0.14% / +0.25% |
| **4 (2-bit)** | **0.6286 / 0.4514** | **0.6295 / 0.4527** | 0.6359 / 0.4557 | **+0.14% / +0.29%** |
| 8 | 0.1951 / 0.3620 | 0.1964 / 0.3620 | 0.1981 / 0.3638 | +0.67% / 0.00% |

(`small`/`large` columns computed from this session's own 4 runs, the same methodology candidate
041 used - not copy-pasted from candidate 041's original session, since this session's absolute
levels differ per the `uptime` note above; recomputing them in-session keeps the `small`-vs-
`medium`-vs-`large` comparison internally consistent.)

## Result

**`medium` (490,000px, the real 2-bit corpus file's exact size) sits within 0.06-0.67% of `small`
(153,560px) at every density/variant, and within 0.4-1.0% of `large` (12,192,768px)** - well
inside this session's own run-to-run noise (the 4 individual runs for any single config already
spread by roughly that much, see raw evidence files). At the 2-bit-relevant density (4), `medium`
vs `small` differs by only +0.14% (natural) / +0.29% (volatile). This directly confirms, at the
exact size that matters, what candidate 041's `small`-vs-`large` check already suggested at two
much-further-apart sizes: **this synthetic store-throughput ceiling is scale-invariant** across
the whole 153,560px-12,192,768px range, including at 490,000px specifically - the microbenchmark
is not memory-bandwidth-limited at any of these sizes on this machine, so the exact buffer size
used for the 2-bit row of candidates 041/042/043's store-share comparison table does not
materially change that table's conclusions. Closes the last open item in the 041->043
store-throughput investigation chain.

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
  prior BMP-decoder-related candidate in this workspace has recorded (`resizing_with_catmul` in
  `tests/regression.rs` - a pre-existing, unrelated CatmullRom-filter regression test failure, not
  caused by this or any prior candidate). Matches candidate 041's own count (550/1 - this session's
  tree, like 041's, doesn't have candidate 020/037/043's extra `pal2_large.bmp` fixture copied in).
  Full log: `evidence/cargo_test.log`.

## Why the result looks the way it does

The synthetic loop's working set at every tested size (600KB `small`, 1.9MB `medium`, 48.8MB
`large`) is small enough, and the store pattern regular enough (sequential, unit-stride 4-byte
writes), that none of these sizes saturates this machine's memory hierarchy differently from the
others - the per-store cost is dominated by instruction-level overhead (loop counter/branch,
store-buffer drain rate), not by cache/DRAM bandwidth, at any of these three scales. This matches
candidate 041's own explanation for its original `small`-vs-`large` result and extends cleanly to
the intermediate `medium` size. This session's persistently elevated background load (`uptime`
11.3-11.8 vs. candidates 037/041/043's 0.1-2.6) shifted every tier's *absolute* ns/pixel numbers
upward by roughly 2x compared to those earlier sessions (e.g. this session's `small`/density=4
natural is 0.6286 ns/px vs. candidate 041's original 0.2775 ns/px) - but since all three size
tiers were measured back-to-back in the same session under the same load, the *relative*
`small`-vs-`medium`-vs-`large` comparison this candidate cares about is unaffected by that shift
(the same reasoning candidate 043 used to argue same-session comparisons survive cross-session
absolute-level drift).

## What's not done / open questions

- This candidate does not re-measure the real `pal2_large.bmp` decode's own wall-clock cost in
  this session (that was candidate 043's job, already done and matching within noise) - it only
  confirms the synthetic store-throughput *ceiling* itself doesn't move at the size-matched
  buffer. Recomputing candidates 041/042/043's full store-share-of-real-decode-time table with
  `medium` in place of `small` for the 2-bit row was not done, since this result shows it would
  change that table's 2-bit percentages by well under 1 percentage point either way - not a
  material update.
- This session's noticeably higher background load (`uptime` ~11-12 throughout, vs. ~0.1-2.6 in
  candidates 037/041/043's sessions) is itself a new data point: it did not change the
  scale-invariance conclusion, but it's a reminder that this workspace's absolute ns/pixel numbers
  are not directly comparable across sessions without re-baselining - only same-session
  comparisons (as used throughout this candidate) are safe to read at face value.
- `perf` remains uninstallable in this container (no `perf` binary, `sudo`/`apt` blocked by "no
  new privileges"), so this remains a wall-clock-only comparison, same limitation as candidates
  041/042/043.
