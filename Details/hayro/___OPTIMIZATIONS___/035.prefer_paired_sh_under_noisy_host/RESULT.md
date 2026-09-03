# prefer_paired_sh_under_noisy_host

`rustc 1.91.1`, `cargo 1.91.1` (workspace toolchain, unchanged this session).

Measurement-methodology candidate, not a new source-code fix: this re-runs candidate 029's exact
`Tf`-resolution-cache patch (byte-identical `patch.diff`, same as candidates 007/012/018/019/029)
against the same real PDF (`font_truetype_7.pdf` page 6, 218 `Tf` calls/page reusing 7 fonts) to
directly contrast the naive "build once, run twice per variant" methodology used by candidates
007/012/018 against `../tools/paired.sh`'s alternating-invocation win-count, on a host this
session's own `uptime` showed to be actively fluctuating (not just elevated) at measurement time -
the exact condition candidate 019 flagged as an open backlog item ("generally prefer
`../tools/paired.sh` over naive twice-run comparisons when `uptime` looks unstable"). The
deliverable is the formalized guidance added to `MEASUREMENT.md` (new section 2c) and
`COMMANDS.md`, backed by the fresh empirical contrast recorded below - not a new performance
finding about the `Tf` cache itself (candidate 029 already characterized that).

## Host noise, confirmed live during this session

`uptime` sampled repeatedly (this is a parallel-batch perf-optimize session; sibling candidate
worktrees `030`/`032` were building/benchmarking concurrently, sharing the same 24-core host):

```
09:01:19  load average: 3.62, 4.95, 6.01
09:01:31  load average: 3.29, 4.84, 5.96
09:01:41  load average: 2.94, 4.71, 5.91
09:01:51  load average: 2.71, 4.60, 5.86
09:02:01  load average: 8.31, 5.71, 6.21     <- +5.6 in 10s
09:02:11  load average: 11.54, 6.49, 6.46
...
09:04:04  load average: 12.94, 9.64, 7.67
09:04:28  load average: 20.18, 11.42, 8.30   <- +7.7 in 8s
09:04:36  load average: 22.73, 12.09, 8.53
```

This is exactly the "fluctuating over consecutive checks, not just elevated at one point" pattern
this candidate was asked to formalize a response to - a single `uptime` snapshot taken before
starting a naive measurement would have looked "merely elevated" (12.94) and not obviously
disqualifying, but the swings within the following seconds (up to 22.73) say otherwise.

## What was tried

Same bench and same patch as candidate 029 (`bench/`, `patch.diff` - both copied verbatim, not
re-derived). Built `baseline_bin` (pristine) and `patched_bin` (candidate 029's patch applied)
once each, then compared them two ways, back to back, on the noisy host above:

| Method | single-core (`taskset -c 2`) | multi-core (unpinned) | notes |
|---|---|---|---|
| naive, round 1 (2 runs/variant, 300 iters) | baseline p50 avg 1.5605ms, patched p50 avg 1.5202ms -> **-2.6%** | baseline p50 avg 1.4809ms, patched p50 avg 1.4710ms -> **-0.7%** | looks like a clean small win, no way to tell if it's real from 2 runs alone |
| `paired.sh` (`RUNS=15`, 100 iters/run) | median 1.624 -> 1.523ms (**-6.2%**), **13/15** wins | median 1.442 -> 1.437ms (**-0.4%**), **11/15** wins | both **"NOT consistent"** per the tool's own 14/15 (or 1/15) threshold - correctly reports "I don't know" instead of a confident number |
| naive, round 2 (2 runs/variant, 300 iters, ~2 min later, host now louder: load 15.5) | baseline p50 avg 2.2819ms, patched p50 avg 2.6721ms -> **+17.1%** (patched *slower*) | not re-run | **sign-flipped** from round 1, on the identical pair of binaries |
| `paired.sh` re-run via `just test` in a fresh scratch clone (separate moment, host quieter: load ~9.6) | median 1.453 -> 1.429ms (**-1.6%**), **14/15** wins, **"consistent"** | not re-run | matches candidate 029's own characterization (-1.4% to -1.8%) once the host was quiet enough for `paired.sh`'s own threshold to clear |

All five runs above used the identical `baseline_bin`/`patched_bin` pair (md5-verified different
from each other, same pixmap hash from both) - every number in this table is genuinely about the
same two binaries, only the host's own noise level and the comparison method changed.

## Result

**The naive twice-run-per-variant method produced a directionally wrong answer under this
session's noisy host**: round 1 said "-2.6% (win)", round 2 - on the *same binaries*, ~2 minutes
later, with load average having climbed from ~13 to ~15.5 and still rising - said "+17.1% (patched
is slower)". Neither naive round carries any internal signal that it might be wrong; both print a
single confident-looking percentage.

`paired.sh` never produced a wrong-signed answer. When the host was actively swinging (13-23 load
average within seconds), it printed **"NOT consistent"** for both pin modes (13/15 and 11/15,
short of its own 14/15 threshold) - correctly declining to assert a direction rather than printing
a misleading number. When re-run later at a quieter moment (load ~9.6, still fluctuating but less
sharply), it cleared its threshold (14/15, "consistent") and reported -1.6%, which matches
candidate 029's original, less-noisy-host characterization (-1.4% to -1.8%) far more closely than
either naive round did.

This confirms candidate 019's finding was not a one-off: on a second, independently-noisy host
session, the same failure mode (naive comparisons landing in contradictory clusters) reproduced,
and `paired.sh`'s alternating design caught it the same way. The formalized guidance (see below)
generalizes both sessions' evidence into a decision rule: check `uptime` more than once,
consecutively, before choosing a measurement method, not just once at the start.

## What changed in the skill files

Added to `~/.claude/skills/perf-optimize/MEASUREMENT.md`: a new section **"2c. Check `uptime`
more than once before trusting a naive comparison"**, cross-referenced from section 2b, stating
the decision rule (sample `uptime` 3-4 times a few seconds apart before measuring; if the load
average is climbing/falling/swinging rather than roughly flat, use `tools/paired.sh` instead of a
naive twice-run-per-variant comparison, and re-check `uptime` again if a `paired.sh` run itself
comes back "NOT consistent" - that is itself a signal to look at `uptime` again rather than
retrying blindly) with this candidate's and candidate 019's numbers as the two supporting
data points.

Added a short cross-reference note to `~/.claude/skills/perf-optimize/COMMANDS.md`'s existing
`paired.sh`/noise-related entries pointing at the new `MEASUREMENT.md` section.

Neither edit touches `hayro`'s own source or this project's `___OPTIMIZATIONS___/SUMMARY.md`/
`IDEAS.md` (batch-level transcription is a separate, later step per this session's instructions).

## Does it change behavior/output?

- **Bit-exact?** Yes - `patched_bin` produced the identical FNV-1a pixmap hash
  (`0xf4543a40af003f8a`) as `baseline_bin` across every run in every table row above (naive and
  paired, both pin modes, both sessions).
- No new source change was authored for this candidate; `patch.diff` is byte-identical to
  candidates 007/012/018/019/029's own `Tf`-resolution-cache fix, included here (per SKILL.md's
  "patch.diff is mandatory whenever the fix is a change to the tested project's own source") only
  because it's the vehicle used to demonstrate the measurement-methodology point - not because
  this candidate found or is proposing a new fix.
- **Is this a breaking change?** No - same internal-only patch as prior candidates; this
  candidate's own contribution (the `MEASUREMENT.md`/`COMMANDS.md` edits) is documentation, not a
  code change.
- Project test suite: `cargo test --release -p hayro-tests --test tests -- render`, run before
  (pristine) and after (patch applied), in the isolated worktree. Both runs: **274 passed; 1252
  failed** (181 filtered out) - every failure is `Os { code: 2, kind: NotFound }` against
  `hayro-tests/pdfs/downloads/corpus/*.pdf`, a gitignored local-only asset directory that does not
  currently exist anywhere on this machine (confirmed absent from both the main working tree and
  this worktree) - the same pre-existing environmental limitation every prior candidate in this
  workspace has noted, not a patch effect. Diffed the full failing-test-name and passing-test-name
  sets between the before/after runs: **identical** in both directions (`diff` reported no
  differences).

## Why the result looks the way it does

Two independent mechanisms compound on this host during a parallel-batch session:

1. **Real host contention from sibling candidates.** `git worktree list` confirmed candidates
   `030`/`032` had live worktrees during this measurement window - their own `cargo
   build`/benchmark steps (wrapped in the same shared `flock` this candidate used) periodically
   saturate CPU even though the *lock* serializes this candidate's own heavy steps against
   theirs; other host activity outside the lock (compilation of dependencies not yet cached,
   background OS work) still shows up as swinging `uptime`.
2. **A single naive comparison has no way to detect when it landed in a bad window.** Two runs
   of 300 iterations each take well under a second combined - short enough that either run (or
   both) can land entirely inside or outside a noise spike, and nothing in the naive method's own
   output distinguishes "clean signal" from "unlucky timing". `paired.sh`'s alternating A/B/A/B
   design instead interleaves both binaries through the *same* sequence of noise spikes, so a
   spike affects both sides similarly rather than skewing whichever variant happened to be
   sampled during it - and its win-count threshold turns "I don't have enough signal" into an
   explicit, printed verdict instead of a silently overconfident number.

## What's not done / open questions

- The exact `uptime`-sampling cadence/threshold now written into `MEASUREMENT.md` (3-4 samples,
  a few seconds apart, "climbing/falling/swinging vs. roughly flat") is a judgment call derived
  from two sessions' worth of observed swings (candidate 019's and this one) - not a statistically
  derived threshold. A future session with more data points could tighten it (e.g. a specific
  load-average delta over N seconds) if the same ambiguity keeps coming up.
- This candidate did not attempt to isolate *which* sibling candidate's build caused which
  specific spike - `git worktree list` only established that concurrent worktrees existed, not a
  moment-by-moment attribution. Not needed for the guidance being formalized (the guidance reacts
  to `uptime` directly, regardless of cause).
- No new performance finding about the `Tf`-resolution-cache fix itself - candidate 029's
  characterization (-1.4% to -1.8%, small win, right at this bench's noise floor at 218 calls)
  stands unchanged; this candidate's quieter-moment `paired.sh` re-run (-1.6%, 14/15) is
  consistent with it, not a revision of it.
