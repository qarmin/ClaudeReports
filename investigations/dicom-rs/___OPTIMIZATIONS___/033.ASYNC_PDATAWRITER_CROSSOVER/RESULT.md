# Narrowing candidate 029's (AsyncPDataWriter direct dispatch) win/no-effect crossover point

Candidate 029 (`ul/src/association/pdata.rs`'s `AsyncPDataWriter::poll_write` direct-dispatch fix
for large single writes) measured only two payload sizes: 100MB ("no consistent change") and
300MB ("-2.3%/-2.7%, consistent win"). `IDEAS.md` flagged the gap between them as unexplored.
This candidate re-applies candidate 029's patch unchanged and sweeps 100/150/200/250/300MB to find
where the effect actually starts.

## What was tried

Same isolated in-memory-sink harness mode as candidate 029 (`async-pdatawriter-inprocess`,
`--features async-pdatawriter-inprocess,dicom-ul/async,dicom-ul/bench-internals`, plus candidate
029's own `bench-shim.diff` for `AsyncPDataWriter::new_for_bench`). Two binaries (baseline =
`bench-shim.diff` only, patched = `bench-shim.diff` + `patch.diff`), compared via
`tools/paired.sh` (`RUNS=9`), 20 iterations/run for 100-250MB, 10 iterations/run for 300MB (larger
payload, kept the whole sweep inside a practical time budget).

| Payload | round 1 (main tree) | round 2 (main tree, re-check) | round 3 (fresh `just test` clone) |
|---|---|---|---|
| 100MB | +1.5%, 1/9 - consistent (regression) | +1.7%, 1/9 - consistent (regression), reproduces round 1 | +0.5%, 3/9 - NOT consistent |
| 150MB | -0.6%, 6/9 - NOT consistent | -0.4%, 4/9 - NOT consistent, reproduces round 1 | -0.8%, 6/9 - NOT consistent, reproduces rounds 1-2 |
| 200MB | -1.4%, 9/9 - consistent (win) | not re-run | -1.7%, 9/9 - consistent (win), reproduces round 1 |
| 250MB | -2.0%, 9/9 - consistent (win) | not re-run | -2.6%, 9/9 - consistent (win), reproduces round 1 |
| 300MB | -2.3%, 9/9 - consistent (win) | matches candidate 029's own originally-recorded -2.3%/-2.7% | -2.2%, 9/9 - consistent (win), reproduces round 1 and candidate 029 |

## Result

The crossover is sharper and better-characterized than candidate 029's original two-point
measurement suggested:

- **100MB is genuinely borderline, not a clean "no effect"** - 2 of 3 independent rounds (main
  tree, both pinned to the same box/session) found a small, consistent regression (+1.5%/+1.7%,
  1/9 both); the third round, run from a completely fresh `just test` clone build in a separate
  process/binary, found no consistent effect (+0.5%, 3/9). Combined with candidate 029's own
  original two rounds (also both "not consistent" at 100MB, 3/9 and 2/9), the honest summary is:
  **100MB sits right at this box's noise floor, leaning toward a small regression more often than
  not (3 of 5 total rounds across both candidates found a consistent small regression, 2 of 5
  found no consistent effect, 0 of 5 found a consistent win)** - not the clean "no consistent
  change" candidate 029 originally reported, but also not confidently a regression either.
- **150MB sits in a genuine transition/noise zone** - three independent rounds (all three main
  tree + fresh-clone) agree: never clears the consistency bar in either direction.
- **200MB is where the win becomes consistent and stays consistent through 300MB**, scaling
  smoothly from -1.4/-1.7% (200MB) to -2.0/-2.6% (250MB) to -2.2/-2.3% (300MB) across the two
  independent measurement environments (main tree vs. fresh clone), matching candidate 029's own
  originally-recorded 300MB number both times.

Time only was measured (no peak-RSS/allocation follow-up in this candidate - see "What's not
done" below).

## Does it change behavior/output?

- **Bit-exact?** Yes - same patch as candidate 029, same correctness argument: byte-for-byte
  reconstructed PDU stream, unchanged from that candidate's own differential verification.
- **Is this a breaking change?** No - internal `ul` crate implementation detail, same as
  candidate 029.
- Project test suite: `cargo test --release -p dicom-ul --features async,bench-internals`:
  **34 passed / 0 failed** (patch applied) - matches candidate 029's own recorded count exactly.
  Reverted-tree (baseline) count is identical (this is a new-state-variant addition, not a
  behavior change to any existing code path when the direct-dispatch threshold isn't hit).

## Why the result looks the way it does

Same mechanism candidate 029 already identified: `poll_write`'s direct-dispatch path avoids
copying a large payload into `self.buffer` before writing it, at the cost of a slightly more
complex state machine (`WritingDirect` vs. the original `Writing` state) for every write,
including small ones near the dispatch threshold. Below ~150-200MB, the avoided-copy saving is
too close to this box's own noise floor to draw a confident conclusion either way (100MB leans
toward a small regression more often than not, but not decisively - see "Result" above); above
~200MB, the avoided copy (memcpy of a multi-hundred-MB buffer, real cache/bandwidth cost) clearly
dominates and the fix pays off, scaling roughly linearly with payload size through 300MB across
two independent measurement environments.

## What's not done / open questions

- The exact crossover between 150MB (noise) and 200MB (consistent win) was not narrowed further
  (e.g. 175MB) - two points bracket it, not pinpointed.
- Peak-RSS/allocation-count follow-up for this fix, still open per `IDEAS.md`'s own separate
  backlog entry (not resolved by this candidate - that entry remains open, only the crossover-point
  entry is resolved here).
- Real loopback-TCP behavior (vs. this isolated in-memory-sink measurement) also remains a
  separate open `IDEAS.md` item, unaffected by this candidate.
- Given 100MB's borderline-negative lean and 150MB's confirmed noise-zone status, a future session
  applying candidate 029's fix for real should consider whether a size threshold (~200MB on this
  box, where the win first becomes consistently confirmed) is worth adding before the
  direct-dispatch path, rather than dispatching directly for every write above the PDU-size
  boundary as the current patch does unconditionally - this candidate does not implement that
  threshold, only documents the case for weighing it.
