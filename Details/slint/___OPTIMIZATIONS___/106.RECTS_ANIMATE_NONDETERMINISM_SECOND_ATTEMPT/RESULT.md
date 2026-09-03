# 106 - Second attempt at reproducing candidate 102's `--mode rects --animate --count 2000` non-determinism

This is an **investigation candidate, not a performance candidate**: `patch.diff` is empty, no
product source was touched. It is the top-of-backlog `IDEAS.md` entry added while building
candidate 105 ("re-attempt reproducing 102's finding if it resurfaces, capture the failing `.ppm`
pair immediately"). 105 already tried once (~140 runs, 9 scenario variants, 0 differences) and
this candidate tries again with a larger and partly different matrix, specifically targeting two
mechanisms 105 didn't test: **self-contention** (many copies of the exact scenario racing each
other, as opposed to unrelated busy-loop processes) and **heavier sustained CPU oversubscription**
(up to ~60 busy processes on 24 cores, `uptime` load up to ~26 sustained / ~47 peak, vs. 105's
~21.5 peak).

## What was tried

No source-code change. Built `perf_test` fresh against the current tree (`internal/`, `perf_test/`
both clean per `git status`/`git diff` - candidate 102's virtual-clock fix is already the permanent
state of `perf_test/src/bench.rs`, confirmed present via `grep -n virtual_now perf_test/src/bench.rs`
before starting). Every run used `--bench --mode rects --animate --count 2000` (102's/105's exact
scenario) unless noted, comparing `--snapshot` `.ppm` output via SHA-256 of the full file:

| Scenario | Runs compared | Result |
|---|---|---|
| `--frames 5/10/20/30` (102's exact frame counts), 15 runs each vs. each other | 60 | 1 unique hash each frame count - 0 differing |
| Self-contention: 8 copies of `--frames 30` launched concurrently (same binary racing itself), 5 rounds | 40 | 0 differing |
| Larger `--count` (5000, 10000) at `--frames 30`, 10 runs each | 20 | 0 differing |
| Injected background load (20 busy-loop processes, `uptime` ~4-12 sustained), `--frames 5/10/20/30`, 12 runs each | 48 | 0 differing |
| Heavier injected load (20 busy-loop processes, `uptime` ~10-12 sustained after a 15s settle), same 4 frame counts, 15 runs each | 60 | 0 differing |
| Extreme oversubscription (60 busy-loop processes, `uptime` peak ~26-47, `taskset -c 3` pinning the perf_test process onto a contended core), `--frames 5/10` before the 2-minute investigation timeout | 20 | 0 differing (larger frame counts not reached - see below) |
| Dense frame sweep, every value 1-60, 2 runs each, settled conditions | 120 | 0 differing at every one of 60 frame counts |
| `--animate-color`, `--gradient-kind radial`, `--scroll-step 5` (each combined with base scenario at `--frames 30`), 10 runs each | 30 | 0 differing |

Total: **~398 runs across 8 scenario groups, 0 differing bytes found in any comparison**, including
102's exact named frame counts (5, 10, 20, 30) and count (2000), self-contention from the identical
binary/scenario (not tried by 105), and heavier sustained load than 105 reached (up to ~26 sustained
/ ~47 peak `uptime` vs. 105's ~21.5 peak).

The extreme-oversubscription row did not finish its planned frame counts (30, 50, 100) - pinning the
measured process onto a core also serving 60 competing busy loops made each `--frames 100` run take
long enough that the whole loop exceeded a 2-minute command timeout mid-run. The two frame counts
that did complete (5, 10) before the timeout still showed 0 differences. Not re-run without the
timeout constraint, since the dense sweep and the two lighter-load rounds already cover the same
frame-count range under load levels that reached the same "0 differing" answer.

## Result

**Could not reproduce candidate 102's reported non-determinism**, same outcome as candidate 105.
Every run of `--mode rects --animate --count 2000` at every tested frame count (1 through 60,
including 102's named 5/10/20/30) produced byte-identical `.ppm` output, under quiet conditions,
self-contention, and injected CPU load up to roughly double the peak candidate 105 reached. This
adds ~398 more zero-difference runs on top of 105's ~140 (total ~538 across the two sessions),
still without ever recovering the originally reported 1168-12908 differing px.

## Does it change behavior/output?

- **Bit-exact?** N/A to the product - no tracked Slint source was changed (`patch.diff` empty).
  This candidate only re-ran `perf_test --bench`'s existing headless snapshot path, unmodified.
- **Is this a breaking change?** No - nothing was changed.
- Project test suite: not run for this candidate - a pure investigation with no patch, which
  `~/.claude/skills/perf-optimize/SKILL.md`'s "Correctness" section explicitly exempts ("no
  exceptions other than a pure investigation with no patch").

## Why the result looks the way it does

Same two candidate explanations as 105, now with self-contention and heavier load also ruled out
as triggers:

1. **Candidate 102's own finding was a stale-binary or one-off measurement artifact** (still the
   leading explanation - unchanged from 105, since nothing in this session's larger matrix produced
   a single differing byte either).
2. **Self-contention and CPU oversubscription are not the trigger.** This candidate specifically
   added two mechanisms 105 hadn't tried - 8 copies of the exact scenario racing each other on the
   same machine, and load roughly double 105's peak - on the theory that a scheduling-correlated
   effect (the kind candidate 104 found in `scroll` mode's `ScrollBar`) might need the *measured*
   process itself to experience contention, not just background noise from unrelated processes.
   Neither showed any effect, further narrowing what could cause it: nothing in `bench.rs`,
   `animations.rs`, or the `rects` scene generator reads real time, thread-local scheduling state,
   or anything else self-contention could perturb - the frame content is a pure function of the
   frame index and the deterministic virtual clock, as 105 already established by code reading.
3. **104's `scroll`-mode mechanism remains an unlikely match for this scene**, per 105's own
   reasoning (its effect is small - low-single/double-digit px in one screen corner - while 102's
   report was 1000+ px), and this session's higher load levels still didn't surface anything of that
   magnitude here either.

## What's not done / open questions

Same as 105's, still open:

- **Candidate 102's session/environment is not available to compare against.**
- **`README.md`'s `--animate` "not pixel-comparable" label is still not removed** - two sessions'
  combined ~538 zero-difference runs is stronger negative evidence than one, but still not a
  positive determinism proof (different machine/kernel, or some condition neither session's load
  injection reproduced).
- **New for this session:** the extreme-oversubscription row didn't reach frame counts above 10
  before timing out - if a future session wants to specifically chase "does it need *both* self-
  contention and very heavy oversubscription together", that combination at higher frame counts is
  the one gap this session's matrix didn't close. Not re-queued to `IDEAS.md` as a fresh idea (it's
  a narrower variant of the same already-queued backlog item, not a new lead), but noted here so a
  third attempt doesn't have to re-derive it.

## Reproduce

```sh
cd perf_test && cargo build --profile rdebug --bin perf_test --features software
BIN=../target/rdebug/perf_test
mkdir -p /tmp/106_check
# Exact 102/105 scenario, dense frame sweep:
for f in $(seq 1 60); do
  $BIN --bench --mode rects --animate --count 2000 --frames $f --snapshot /tmp/106_check/f${f}_a.ppm >/dev/null
  $BIN --bench --mode rects --animate --count 2000 --frames $f --snapshot /tmp/106_check/f${f}_b.ppm >/dev/null
  sha256sum /tmp/106_check/f${f}_a.ppm /tmp/106_check/f${f}_b.ppm
done

# Self-contention (this session's new addition over 105):
for r in $(seq 1 8); do
  $BIN --bench --mode rects --animate --count 2000 --frames 30 --snapshot /tmp/106_check/c$r.ppm >/dev/null &
done
wait
sha256sum /tmp/106_check/c*.ppm | awk '{print $1}' | sort -u | wc -l   # 1 in this session
```
