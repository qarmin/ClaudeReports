# Resolving candidate 023's unexplained single-core gap vs. candidate 018

Candidate 023 (fused min/max fold + rayon threshold-guard + slice signature for
`Lut::new_rescale_and_normalize`) measured -23.2% single-core (no-rayon build) on `mono_above.dcm` -
larger than candidate 018's own standalone fold-fusion-only measurement of -15.4% on the *same*
fixture. Both patches were measured in different, non-adjacent sessions, so the ~8-point gap was
never isolated: was it the `&[S]` slice signature (replacing the generic `IntoIterator`/`Clone`/
`NumCast` bound) removing extra iterator-adapter overhead beyond the fold fusion, or something
else? `IDEAS.md` flagged this as open, suggesting "a variant that fuses the fold but keeps the old
generic-iterator signature" to separate the two effects.

## What was tried

That exact variant already exists and needed no new code: **candidate 018's own `patch.diff` IS
the fusion-only, old-generic-iterator-signature variant** `IDEAS.md` asked for - it was never
re-measured together with candidate 023's full patch in the same controlled session. Built three
binaries from the same base tree (`--no-default-features`, single-core, matching candidate 023's
own methodology): baseline (no patch), candidate 018 alone (fusion only, generic `IntoIterator`
signature unchanged), and candidate 023's full patch (fusion + slice signature + rayon guard, only
the guard's own `if` branch is reachable in this no-rayon build - same code path as the fusion
alone). All three compared via `tools/paired.sh` (`RUNS=9`, `taskset -c 0`) on the same fixture
(`mono_above.dcm`, 1,048,576 samples) candidate 023 originally used, in one session.

| Comparison | round 1 (`taskset -c 0`) | round 2 (re-check, `taskset -c 0`) | round 3 (fresh `just test` clone, unpinned) |
|---|---|---|---|
| baseline vs. candidate 018 alone | -22.9%, 9/9 consistent | -23.0%, 9/9 consistent | -25.1%, 9/9 consistent |
| baseline vs. candidate 023 full | -22.9%, 9/9 consistent | -22.6%, 9/9 consistent | -20.1%, 9/9 consistent |
| candidate 018 alone vs. candidate 023 full (direct) | -0.1%, 6/9 - **NOT consistent** | (not re-run, round 1 already decisive) | +2.7%, 2/9 - **NOT consistent**, confirms round 1 |

## Result

**The gap does not reproduce - it was a cross-session measurement artifact, not a real effect of
the slice-signature change.** Measured together, apples-to-apples, across three independent rounds
(including a fresh-clone `just test` run, unpinned): candidate 018 alone (-22.9%/-23.0%/-25.1%)
and candidate 023's full patch (-22.9%/-22.6%/-20.1%) are statistically indistinguishable round to
round, and a **direct head-to-head comparison between the two patched binaries shows no consistent
difference at all, twice** (-0.1% 6/9, then +2.7% 2/9 on the fresh-clone re-check - fails the
consistency bar both times, in opposite directions, exactly what noise looks like). The originally
recorded numbers (018: -15.4%, 023: -23.2%) were each real measurements in their own session, but
comparing them to each other across sessions was never a valid apples-to-apples comparison -
different box states/binaries built at different times are exactly the kind of confound
MEASUREMENT.md's isolating-confounds discipline warns about, and this is a concrete case of it
actually causing a phantom ~8-point "gap" that a same-session, controlled comparison shows isn't
there.

This resolves the `IDEAS.md` item's original hypothesis (slice signature sheds extra generic-
iterator overhead) as **not supported by the data** - the slice signature change appears to have
essentially zero standalone single-core effect on this fixture; the entire single-core win comes
from the fold fusion alone.

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum=0x96db96ecb3a17025` identical across baseline, candidate 018
  alone, and candidate 023's full patch.
- **Is this a breaking change?** No - measurement-only investigation, no new source change (both
  patches are candidates 018/023's own, applied unchanged, one at a time). Copies of both are saved
  here as `patch_018.diff`/`patch_023.diff` (identical to `018.LUT_NORMALIZE_SINGLE_PASS_MINMAX/patch.diff`
  and `023.LUT_NORMALIZE_RAYON_GUARD/patch.diff` respectively) so this folder is self-contained; each
  applies clean against this workspace's `base_commit` on its own (`git apply --check`, verified).
- Project test suite: `cargo test --release -p dicom-pixeldata` for both patches individually:
  **39 + 1 + 1 + 10 = 51 passed / 0 failed** each, matching candidates 018/023's own recorded
  counts.

## Why the result looks the way it does

Not a new mechanism - this candidate's contribution is methodological: it shows that two numbers
measured in different sessions, even against nominally "the same fixture," should not be
subtracted to infer a mechanism without re-measuring both together first. The likely source of the
original gap is ordinary box-state/session-to-session variance (background load, thermal state,
or a slightly different binary layout from an intervening build) rather than anything about the
`&[S]` vs. generic-`IntoIterator` signature specifically.

## What's not done / open questions

- This closes the `IDEAS.md` item asking to root-cause the gap - the answer is "the gap isn't
  real," which resolves the question without needing further investigation.
- **General lesson worth carrying into future sessions on this project** (also written into
  `projects/dicom-rs.md`): when comparing two candidates' historical numbers to infer a
  third effect (e.g. "does X contribute beyond Y"), re-measure both together in one session before
  trusting the arithmetic difference between two separately-recorded percentages - this workspace
  already has one other case of a "borderline result that keeps landing differently across
  measurement environments" (candidate 033's 100MB async-pdatawriter finding), and this is a
  second, related trap: a real, consistent single-session result can still not survive being
  compared against a different session's real, consistent result.
