# Root-causing candidate 005's second, liver.dcm-specific write-path regression

Follow-up to `005.TEXT_CODEC_BULK_SINGLE_BYTE` (mandatory reading first): this task was framed
around a claimed second, distinct regression - separate from that candidate's own confirmed
`seq_10k.dcm` `write-only` regression - on `pydicom/liver.dcm`'s combined `roundtrip`
(parse-then-write, repeated) mode, reported as ~11-15% slower patched despite a callgrind
cache-sim comparison showing fewer instructions and comparable-or-better cache-miss rates. The
prescribed next step was a `valgrind --tool=massif`/`heaptrack` allocation-size-histogram
comparison, the technique that resolved candidates 005/016/041's other glibc-specific
regressions.

**That regression does not reproduce in this session.** Every measurement taken here - multiple
independent paired-comparison rounds, both run orders, both allocators (glibc default and
mimalloc), single-core and multi-core, both a realistic and a 5x iteration count, and all three
relevant harness modes (`roundtrip`, `write-only`, `parse`) - shows candidate 005's exact,
unmodified patch consistently **faster**, not slower, on `liver.dcm`. No massif/heaptrack
allocation-histogram investigation was performed, since there is no reproduced regression left to
root-cause - see "Why the result looks the way it does" below for what this candidate did instead
to try to explain the discrepancy, and candidate 050's own `RESULT.md` for a closely analogous
prior case (a different candidate's reported `liver.dcm` regression that also failed to
reproduce in a later session).

## What was tried

Built a fresh git worktree at this workspace's own `base_commit`
(`df2b9e4236298efa3fa37a1f360fc4bf284d586a`, confirmed identical to the current repo `HEAD`),
copied in the canonical `___OPTIMIZATIONS___/harness/bench_pipeline` harness, and re-applied
candidate 005's own `patch.diff` unmodified (no new source change this candidate). Built baseline
and patched release binaries under both the default (glibc) allocator and `--features
diag-mimalloc`, then ran `tools/paired.sh` (`RUNS=9`) against `liver.dcm` in every combination
below, plus one direct order-swap (patched as "A", baseline as "B") to rule out a run-order
artifact, and one manual `taskset -c 3` pinned re-check outside the paired script.

| Comparison | mode | iters | allocator | pinning | result |
|---|---|---|---|---|---|
| baseline vs. patched | `roundtrip` | 400 | glibc default | unpinned | -15.1%, 9/9 (patched faster) |
| baseline vs. patched (re-run) | `roundtrip` | 400 | glibc default | unpinned | -16.5%, 9/9 (patched faster) |
| **patched vs. baseline (order swapped)** | `roundtrip` | 400 | glibc default | unpinned | +18.3%, 0/9 for baseline - i.e. patched still faster in all 9 pairs |
| baseline vs. patched | `roundtrip` | 400 | glibc default | `taskset -c 3` | patched faster in 9/9 manual pairs (~62us vs. ~73us) |
| baseline vs. patched | `roundtrip` | 2000 (5x) | glibc default | unpinned | -15.8%, 9/9 (patched faster) |
| baseline vs. patched | `roundtrip` | 400 | mimalloc | unpinned | -13.5%, 9/9 (patched faster) |
| baseline vs. patched | `write-only` | 500 | glibc default | unpinned | -13.4%, 9/9 (patched faster) |
| baseline vs. patched | `parse` | 500 | glibc default | unpinned | -17.1%, 9/9 (patched faster) |

`uptime` load average during these runs ranged 13.8-17.4 (out of 24 cores) - below the
re-baseline threshold, so not re-baselined.

## Result

**No regression reproduces anywhere.** Every one of the 8 comparisons above - two allocators, two
run orders, 1x and 5x iteration counts, pinned and unpinned, and all three modes plausibly meant
by "combined roundtrip (parse-then-write, repeated)" - shows candidate 005's patch consistently
**faster** on `liver.dcm`, by roughly 13-18%, with every individual comparison clearing the
9/9-consistent bar (or its single manual equivalent). This is the opposite direction from the
task's premise (~11-15% *slower* patched), not just "smaller than reported" or "noisy" - there is
no comparison in this session, under any condition tried, where baseline measured faster than
patched on this fixture.

## Does it change behavior/output?

- **Bit-exact?** Yes. `dump` output (`sha256sum`) is identical between baseline and patched builds
  on `liver.dcm`: `4f8fb316b6df067bdf2ef7bc2385fd571ad5be67e171aed3ed902a71293d9d5c` for both.
  `roundtrip`'s own FNV-1a checksum (`0x48361243b8d39d10`) also matches between builds.
- **Is this a breaking change?** No - this candidate made no new source change; `patch.diff` here
  is a byte-for-byte copy of candidate 005's own `patch.diff`, checked to apply clean against the
  current repo `HEAD` (`git apply --check`, verified) as well as this workspace's pinned
  `base_commit` (they are the same commit).
- Project test suite: `cargo test --release -p dicom-encoding`: **40 + 4 = 44 passed / 0 failed**
  for both the baseline and patched builds - identical count, no new failures, matching candidate
  005's own recorded number exactly.

## Why the result looks the way it does

The prescribed massif/heaptrack step was not run, because there was nothing left to root-cause
with it - a regression that doesn't reproduce has no allocation pattern to histogram. What this
candidate did instead, to rule out the most likely reasons a real prior finding might fail to
reproduce:

- **Not an order/measurement-bias artifact.** Swapping which binary plays "A" vs. "B" in the
  paired script reproduces the exact same conclusion (patched faster) with the labels reversed -
  ruling out a systematic first-vs-second-run bias in the measurement technique itself.
- **Not scale-dependent.** Raising `roundtrip`'s iteration count 5x (400 -> 2000) doesn't change
  the direction or shrink the margin, arguing against a slow-building effect (e.g. progressive
  heap fragmentation) that might only show up at a longer sustained run.
- **Not allocator-specific in the way candidates 005/016 were.** Unlike this workspace's own
  glibc-vs-mimalloc/jemalloc regressions, this comparison shows the *same* direction (patched
  faster) under both allocators tested - if there were a glibc-specific pathology here the way
  there was for `seq_10k.dcm`'s `write-only` mode, mimalloc should have at least narrowed or
  reversed the gap, not reproduced it at a similar magnitude (-13.5% vs. -15 to -18% under glibc).
- **Not specific to the `roundtrip` mode as originally framed.** `write-only` and `parse` alone
  (liver.dcm's two component operations) each independently show the same faster-patched result,
  consistent with `roundtrip` simply being the sum of two individually-faster operations, not a
  mode-specific interaction effect.

This leaves the same leading explanation candidate 050 reached for an analogous case (a different
candidate's own reported `liver.dcm` regression that also failed to reproduce a session later):
**a cross-session measurement artifact** - whatever produced the originally-reported ~11-15%
slowdown (background load, thermal/governor state, binary-layout/ASLR interaction at the time of
that original measurement) was not present, or was reversed, on this box during this session.
Nothing about `liver.dcm`'s content or the patch's own mechanism (a bulk single-byte-codec
decode/encode bypassing per-character virtual dispatch, exercised on this file's real `ISO_IR 100`
`SpecificCharacterSet` text) explains a `liver.dcm`-specific regression that a `seq_10k.dcm`-based
allocator pathology (005's own confirmed `write-only` regression) wouldn't already predict more
directly - `liver.dcm` is pixel-data-heavy with comparatively little text content (per candidate
005's own round-1 finding: "no consistent wall-clock change anywhere" on `color3d.dcm`, another
pixel-heavy fixture), which if anything argues *against* a large write-path text-codec effect of
either sign being easy to see here at all, consistent with this session's inability to find one to
root-cause.

## What's not done / open questions

- **The originally-reported liver.dcm regression remains unexplained**, not because a mechanism
  was found and ruled out, but because the effect itself could not be reproduced under any
  condition tried this session - there is no allocation pattern, code path, or timing window left
  to point a massif/heaptrack comparison at.
- If a future session reproduces this regression again (a different box, a different load
  condition, a specific background-process state), the comparisons in this candidate's own table
  above are a ready-made checklist of what's already been ruled out (order bias, scale, both
  allocators, all three individual modes) - start from what's left, not from scratch.
- This candidate's own finding, combined with candidate 050's near-identical experience for a
  different candidate's `liver.dcm` regression, suggests a standing methodological caution worth
  flagging for `projects/dicom-rs.md`: **this project's own regression claims on `liver.dcm`
  specifically have now failed to reproduce twice, across two unrelated candidates (005 and
  027), in a later session on the same box.** Whether `liver.dcm` itself (its small size, its
  pixel-data-dominated shape) makes it unusually sensitive to whatever cross-session confound is
  at play, versus this being coincidence, was not investigated further here.
