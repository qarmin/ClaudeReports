# 105 - Trying to reproduce `--mode rects --animate`'s "second unidentified" non-determinism (candidate 102)

This is an **investigation candidate, not a performance candidate**: `patch.diff` is empty, no
product source was touched. It follows up the top-of-backlog `IDEAS.md` entry left open by
candidate 102: after making `BenchPlatform`'s clock deterministic (fixing the real-clock-driven
`ScrollBar` transition jitter candidate 099 diagnosed), 102 additionally reported that `--mode
rects --animate --count 2000` was still non-deterministic run-to-run at `--frames 5/10/20/30`
(12204/1986/1168/12908 differing px respectively, non-monotonic with frame count), while being
bit-identical at `--frames 1`/`2` - described as "a second, unidentified non-determinism source"
never chased down.

## What was tried

No source-code change - this candidate is a reproduction attempt. Built `perf_test` fresh
(`cargo build --profile rdebug --bin perf_test --features software`, default features, which
already include `software`) against the current tree (unchanged since 102 - `internal/core/` etc.
carry no uncommitted candidate patch; `perf_test/` itself is untracked but its `bench.rs`
virtual-clock fix from 102 is the permanent, already-applied state), then ran the exact scenario
102 named plus variations, comparing every run's full `--snapshot` `.ppm` byte-for-byte (not just
the sub-820-row region `tools/pixelcheck.sh` normally restricts to - the full file, status line
included, since 102's own reported diff counts were on the same full-snapshot comparison):

| Scenario | Runs compared | Result |
|---|---|---|
| `--mode rects --animate --count 2000 --frames 10` (initial recheck) | 4 pairwise vs. run 1 | 0/4 differing |
| same, extended | 10 pairwise vs. run 1 | 0/10 differing |
| `--frames 5/10/20/30` (102's exact named frame counts), 3 runs each | 3 pairwise per frame count | 0 differing at every frame count |
| `--frames 20`, larger sample | 20 runs, all-pairs via SHA-256 | all 20 hashes identical |
| `--frames 1..50` (every value), 2 runs each | 2 pairwise per frame count | identical at every one of 16 sampled frame counts (1,2,3,4,5,6,7,8,9,10,15,20,25,30,40,50) |
| `--frames 30`, largest reported diff (12908px) | 30 runs, all-pairs via SHA-256 | all 30 hashes identical |
| `--frames 30` under injected background CPU load (20 busy-loop processes, `uptime` load 5.5-21.5 during the runs, vs. baseline ~3.5) | 20 + 20 runs (`--count 2000` and `--count 5000`) | all hashes identical (also identical to the no-load baseline hash) |
| `--animate --animate-color --count 2000 --frames 30` | 5 runs | all identical |
| `--animate --gradient-kind radial --count 2000 --frames 30` | 5 runs | all identical |
| `--animate --count 2000 --frames 30 --scroll-step 5` | 5 runs | all identical |

Total: **~140 runs across 9 scenario variants, 0 differing bytes found in any comparison**,
including the exact frame counts (5, 10, 20, 30) and count (2000) candidate 102 reported as
non-deterministic, under both quiet (`uptime` ~3.5) and artificially loaded (`uptime` up to ~21.5)
conditions.

## Result

**Could not reproduce candidate 102's reported non-determinism.** Every run of `--mode rects
--animate --count 2000` at every frame count tried (1 through 50, densely around and including
102's named 5/10/20/30) produced byte-identical `.ppm` output across repeated invocations of the
same binary, both under normal load and under artificially injected background CPU contention.
This directly contradicts 102's own numbers (12204/1986/1168/12908 differing px at those same
frame counts) - not a smaller/marginal residual, a complete absence of any difference in this
session's ~140 runs.

## Does it change behavior/output?

- **Bit-exact?** N/A to the product - no tracked Slint source was changed (`patch.diff` empty).
  This candidate only re-ran `perf_test --bench`'s existing headless snapshot path, unmodified.
- **Is this a breaking change?** No - nothing was changed.
- Project test suite: not run for this candidate - a pure investigation with no patch, which
  `~/.claude/skills/perf-optimize/SKILL.md`'s "Correctness" section explicitly exempts from the
  mandatory before/after test-suite run ("no exceptions other than a pure investigation with no
  patch").

## Why the result looks the way it does

Two explanations were considered, neither confirmed further given the fix already reproduces as
absent:

1. **Candidate 102's own finding was a stale-binary or one-off measurement artifact** (SKILL.md's
   pitfall #1, "Stale binary" - confirm a rebuild actually happened after editing source). 102's
   `RESULT.md` documents this specific check only as a side note ("found here as a side effect of
   checking whether this fix would also make `--animate` pixel-comparable") with no saved
   reproduction command for *this* part (unlike the `ScrollBar`-jitter finding in the same
   candidate, which does have a full worked repro script) - consistent with it having been a
   quick, not-fully-controlled check at the time, possibly against a binary built before the
   virtual-clock fix was actually in place, or against a build with a stray uncommitted edit from
   an adjacent investigation.
2. **An environment-specific, non-reproducible artifact of that session** (a corrupted snapshot
   write, a filesystem race in `--snapshot`'s own file I/O, or similar) - possible but unfalsifiable
   at this remove; nothing in the current `bench.rs`/`animations.rs`/`rect-mode.slint` code path
   for this scenario reads any real-time clock or other unseeded source of randomness (checked:
   `grep -n "Instant::now\|SystemTime::now"` in `perf_test/src/bench.rs` only finds the harness's
   own fill/frame-timing measurements, unrelated to pixel content; `internal/core/animations.rs`'s
   `update_animations`/`animation_tick` path is a pure function of the `Instant` passed in, which
   itself comes from `BenchPlatform`'s `Cell<Duration>`, deterministically advanced by a fixed 16ms
   per frame).

Candidate 104's own, independently-found "second, unidentified, core-scheduling-correlated source"
in `scroll` mode's growing-content `ScrollBar` (a few px near one corner, disappearing under
`taskset` pinning) was explicitly compared against 102's `rects+animate` finding in `IDEAS.md` as a
"possibly the same mechanism" lead. Given 104's effect is small (single-digit-to-low-teens px in
one screen corner, present in some fraction of unpinned runs) and this candidate's `rects+animate`
recheck found **zero** difference in ~140 runs including under injected load (which, per 104's own
theory, should have made a core-scheduling-correlated effect *more* likely to show, not less), the
two are unlikely to share a root cause even if 102's original 12000+-px-scale finding had been
real - 104's mechanism, whatever it is, does not explain a difference of that magnitude on this
scene as tested here.

## What's not done / open questions

- **Candidate 102's session/environment is not available to compare against** - it's possible a
  detail of that specific run (a background process, a filesystem state, a slightly different
  `bench.rs` at that exact moment mid-edit) mattered and can't be reconstructed now. If it
  resurfaces, capturing the failing `.ppm` pair immediately (rather than just the differing-pixel
  count) would let a future session diff the actual pixels instead of re-guessing scenarios.
- **The two related backlog follow-ups queued under this idea** ("does `rects+animate`'s residual
  share 104's `scroll`-mode root cause", "does `taskset` alone fix it") are now moot for the
  `rects+animate` side specifically, since there is currently no residual to explain or fix - left
  in `IDEAS.md` reworded to reflect that, in case a future session's environment reproduces 102's
  original finding after all.
- `README.md`'s `--animate` "not pixel-comparable" label is **not removed** by this candidate: a
  single session's inability to reproduce a previously-reported non-determinism is not proof it
  can never occur (e.g. if it really is core-scheduling/timing-sensitive in some way not captured
  by the load-injection method tried here). Downgrading that warning would need a positive
  determinism proof (e.g. hundreds of runs across genuinely different machines/kernels), not
  a negative "didn't see it this time" result.

## Reproduce

```sh
cd perf_test && cargo build --profile rdebug --bin perf_test --features software
mkdir -p /tmp/105_check
for r in $(seq 1 20); do
  ./../target/rdebug/perf_test --bench --mode rects --animate --count 2000 --frames 30 \
      --snapshot /tmp/105_check/r$r.ppm >/dev/null
done
sha256sum /tmp/105_check/r*.ppm | awk '{print $1}' | sort -u | wc -l   # 1 in this session
```
