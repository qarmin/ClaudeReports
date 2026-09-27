# corpus_saturation_threshold_asan_hang_characterization

`fuzz/fuzz_targets/*.rs` under `cargo +nightly fuzz run --sanitizer address` - a backlog item from
`IDEAS.md` (added 2026-08-26, noticed while working candidate `019`). Candidate `019` refined the
default-ASAN empty-corpus hang (first seen in `009`, root-caused to an `RLIMIT_NOFILE` close-loop
in `024`) to "any run in which a fresh, previously-unrecorded coverage event occurs", and showed a
1-file seed almost never avoids it, but never pinned down how large/diverse a pre-existing corpus
actually needs to be, per target, before it reliably does. This candidate empirically bisects that
threshold on two representative targets. Superseded in practical importance by `024`'s
`ulimit -Sn`-based fix (which needs no corpus tuning at all), so this is a lower-urgency,
investigative follow-up rather than a fix for an open bug.

This is a tooling/environment finding, not a bug in `lofty`'s own source - no lofty source file
was suspected or touched.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Corpus-size bisection on `mp4file_read_from`, using the 51-file corpus already left over in `fuzz/corpus/mp4file_read_from/` from earlier candidates' `--sanitizer none` sessions (max achievable coverage `cov: 168`): random subsets of size 0/1/3/5/10/20/22/23/24/25/28/30/32/35/51 files, `--sanitizer address -- -max_total_time=15`, wrapped in a 30s wall-clock kill bound, classified by whether the log ends with libFuzzer's own `Done N runs in Ms` line (completed) vs. stuck at a `NEW_FUNC[...]:` line (hung, matching candidates 009/018/019's signature) | 2 trials per size (different random subset composition per trial, `shuf` with independent random sources) | Sizes 0-22: hung every trial (14/14 hangs across sizes 0,1,3,5,10,20,22). Size 23: clean 2/2. Size 24: hung 2/2 - i.e. **not monotonic in file count**: adding one specific file at rank 24 reintroduced a hang that rank-23 didn't have. Sizes 25 (5 different random compositions total, including 3 extra seeds A/B/C beyond the original 2 trials), 28, 30, 32, 35, 51: clean in every trial (14/14 clean). Full log tails and per-size cov numbers captured during the session (not preserved as a file - see `output.txt` for the standalone `repro/` run's own equivalent evidence). |
| Same bisection on `speexfile_read_from`, first using its own 22-file leftover corpus (`cov: 91` max) | sizes 0 through 22 (the whole pool), `--sanitizer address -- -max_total_time=15`, 30s kill bound | **Hung at every size tested, including the full 22-file pool (2/2)** - this specific leftover corpus never reaches saturation no matter how much of it is used. This directly answers a separate `IDEAS.md` backlog item ("confirm whether `speexfile_read_from`'s 1-file-seed-avoids-the-hang result from candidate `019` holds up on repeat") - it does not: candidate `019`'s one successful 1-file observation on this target was very likely a one-off, not a stable property of the target, since even the *entire* corpus that 1 file came from still hangs. |
| Same bisection on `speexfile_read_from`, using a freshly-built, larger, more diverse corpus (127 files, `cov: 182`, assembled from 4 independent 15s `--sanitizer none` bursts rather than one leftover pool) | sizes 5/10/15/20/25/30/33/35/38/60/90/110/120/127, 2 trials each, same ASAN config, with a fixed process-cleanup step (`pkill -9 -f "release/<target> .*<dir>"`) added after every trial (see "process cleanup pitfall" below) | Sizes 5-38 (i.e. up to 30% of the pool): hung in every trial tested (16/16). Size 60 (47%): mixed, 1/2 hung. Sizes 90 (71%), 110 (87%), 120 (94%), 127 (100%, i.e. the full pool, checked twice independently): clean in every trial (8/8). |
| Standalone `repro/run.sh`: builds a real corpus for `mp4file_read_from` and `speexfile_read_from` from scratch (not reusing the leftover `fuzz/corpus/`), runs the real ASAN command against an empty corpus vs. the resulting full corpus | 4x15s `--sanitizer none` bursts per target to build the corpus, then `--sanitizer address -- -max_total_time=15` with a 30s kill bound on both endpoints | Run 3 times total (2 directly against this checkout + 1 via `just test` against a genuinely fresh `git clone`): **3/3 times**, empty corpus hung and the resulting real, freshly-built corpus (23-50 files for `mp4file_read_from`, 115-144 files for `speexfile_read_from` depending on the run's own random mutation exploration) completed cleanly. See `output.txt`. |

A process-cleanup pitfall was hit and fixed mid-session: `timeout -s KILL <n> cargo +nightly fuzz
run ...` kills the `cargo` wrapper process at the deadline, but the actual libFuzzer target binary
`cargo fuzz` spawns as a child survives as an orphan and keeps running/hanging in the background
(confirmed via `ps aux` - several previous trials' target binaries were still pegged at 99.9% CPU
minutes after their own `timeout` had already returned). This didn't corrupt the bisection's
conclusions (the machine has 24 cores, so a handful of orphaned single-threaded hangs don't starve
new trials of CPU - each trial's own log content, ending in either `Done N runs` or a stuck
`NEW_FUNC[...]:` line, was read directly rather than trusted from a since-discovered exit-code
labeling bug in the sweep scripts), but it does mean every trial from the `speexfile_read_from`
fresh-corpus bisection onward explicitly `pkill -9`s the specific target+corpus-dir combination
after every run, and `repro/run.sh` does the same.

## Status

**Confirmed.** A real, per-target, per-corpus-composition coverage-saturation threshold exists and
was pinned down empirically on two targets via direct bisection with a stated reproduction rate at
each data point (see table above); `repro/run.sh` demonstrates the qualitative core of the finding
(empty corpus hangs, a real accumulated corpus doesn't) 3/3 times, including once via a fully
standalone `just test` run against a fresh clone.

**Fully reproducible: Yes.** Single-participant finding (the real `cargo +nightly fuzz run`
command a maintainer would actually run, against the project's own real, already-existing fuzz
target binaries, with a real corpus built by really running `--sanitizer none` bursts, not
hand-constructed) - `CONFIRMATION.md` section 11's caveat about a faked second participant doesn't
apply, same reasoning as candidates 018/019/024.

## Class

`tooling` (environment/sandbox interaction with `cargo-fuzz`'s default ASAN sanitizer build, same
class as candidates 018/019/024 - not a defect in `lofty`'s own source).

## Root cause

Ties directly into candidate `024`'s mechanism: the hang only happens when ASAN's `SymbolizerProcess`
is spawned at all, which only happens on the *first* `NEW_FUNC` coverage event libFuzzer's own
random mutation exploration produces during a run. Whether a given run produces that first event
depends on whether the corpus already collectively covers every function libFuzzer's mutations are
about to stumble into during that specific run - which is a property of the corpus's *composition*
(which functions its files happen to reach), not simply its file *count*. This explains every
non-monotonic result above: `mp4file_read_from`'s 23-clean/24-hung flip (one specific added file
happened to unlock a mutation path into an uncovered function; a different 24th file might not
have), and why the same raw file count needed for saturation differs wildly by target
(`mp4file_read_from` saturates around 45-50% of an already-diverse ~50-file pool;
`speexfile_read_from` needed ~70-90% of an even larger, 127-file pool built the same way, and its
own smaller 22-file leftover pool never saturated at all). There is no cheap way to predict this
threshold for a given target in advance short of already having built (and, implicitly, having
already needed to survive running under ASAN to check) a sufficiently saturating corpus - which is
exactly why candidate `024`'s `ulimit`-based fix is the practically better answer: it removes the
need to reach this threshold at all rather than trying to characterize it target-by-target.

## The fix

No source fix - this is a tooling/environment finding (characterizing an already-identified
sandbox interaction more precisely), not a code defect. No change to `lofty` source was made or
needed.

`___BUGHUNT___/README.md`'s existing ASAN caveat row already recommends `024`'s `ulimit -Sn`
workaround over corpus-based avoidance; this candidate's finding (threshold is real but highly
target- and composition-dependent, with no cheap way to predict it) reinforces that recommendation
rather than changing it, so no further README update was made beyond what `019`/`024` already
recorded.

- Project test suite: not run for this candidate (no `lofty` source files were modified; nothing
  to regress-test - `git status --porcelain=v1` showed only the untracked `___BUGHUNT___/`
  directory, confirmed both before and after this session).

## What's not done / open questions

- Only 2 of the project's 19 fuzz targets were bisected (`mp4file_read_from`,
  `speexfile_read_from`); the practical range observed (saturation somewhere between ~45% and
  ~90%+ of an already-diverse, independently-built corpus) is unlikely to be a tight universal
  bound - a target with a much larger or smaller reachable-function surface could fall well outside
  it.
- The bisections used random subsets (`shuf`) rather than a principled minimal-covering-set search
  (e.g. `cargo fuzz cmin` against a coverage target); a minimized "smallest corpus that still
  saturates" set was not computed, only "this fraction of files happened to be enough/not enough in
  this trial".
- This closes the two open `IDEAS.md` backlog items it was meant to address (the coverage-
  saturation threshold question, and whether `speexfile_read_from`'s one-off 1-file success from
  candidate `019` holds up); both rows have been removed per `SKILL.md`.
