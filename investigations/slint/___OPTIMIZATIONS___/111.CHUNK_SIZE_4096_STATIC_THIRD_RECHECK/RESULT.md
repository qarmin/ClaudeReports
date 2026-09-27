# 111 - Third, independent recheck of `chunked_arena::CHUNK_SIZE=4096` on `layout` static

IDEAS.md's backlog asked for a third, independent session to resolve the contradiction between
candidate 074 (a small +0.5% single-core regression, single pass, 8/9 paired) and candidate 110
(a consistent -10.0%/-2.6% single-core win, two runs) for `CHUNK_SIZE = 4096` vs `256` on the
`layout` static scene (`scroll --count 100000 --row-style layout --scroll-step 0`) - same
`base_commit` (`cdb6c43f7`) as both prior candidates, same scene/invocation. This session's
`uptime` load average was 2.96-6.07 across the run (host has 24 cores; no `isolcpus`/cgroup
core isolation was available - this container has no write access to `/sys/fs/cgroup`, root:root
755, and no `isolcpus` on the visible kernel cmdline - so `taskset -c N` is the same isolation
mechanism 074/110 used, not the "ideally isolcpus/cgroup" upgrade IDEAS.md asked for).

`chunked_arena` does not exist on `master`; `patch.diff` here is candidate 073's chunked_arena
patch with `CHUNK_SIZE` set directly to `4096` (self-contained, unlike 074/080/110's copies of
073's unmodified `256` patch plus a described one-line edit).

## What was tried

Two binaries (`CHUNK_SIZE = 256` and `4096`), `cargo build --bin perf_test --profile rdebug`,
compared pairwise via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol as 073/074/080/110.
Scene: `scroll --count 100000 --row-style layout --scroll-step 0` (074's exact scene). Three
single-core and three multi-core runs (one more than 074's single pass or 110's two, specifically
because the first single-core run and the first two multi-core runs each landed short of or
crossed the 8-9/9 "consistent" bar in different directions, and a tie-breaker was needed).

Each row's median pair is (`256` A-side) -> (`4096` B-side); the `256` side is re-measured per
pairing, not reused across pairings.

| Core count | Run | `uptime` load avg at start | Result |
|---|---|---|---|
| single-core (`taskset -c 2`) | 1 | 5.01, 4.81, 4.77 | median 158.50 -> 153.78ms (-3.0%, 7/9) - NOT consistent |
| single-core (`taskset -c 2`) | 2 | 3.83, 4.01, 4.43 | median 155.65 -> 152.19ms (**-2.2%, 8/9 consistent**) |
| single-core (`taskset -c 2`) | 3 | 2.96, 3.59, 4.21 | median 155.27 -> 153.23ms (**-1.3%, 8/9 consistent**) |
| multi-core (unpinned) | 1 | 3.32, 3.51, 4.10 | median 154.58 -> 151.03ms (-2.3%, 7/9) - NOT consistent |
| multi-core (unpinned) | 2 | 3.76, 3.60, 4.06 | median 154.08 -> 155.47ms (+0.9%, 4/9) - NOT consistent |
| multi-core (unpinned) | 3 | 6.07, 4.35, 4.27 | median 160.55 -> 192.48ms (**+19.9%, 1/9 "consistent" by the script's win-count rule**) - B-side shows extreme outliers (188-195ms vs. its own other two runs' 154-162ms) coinciding with the highest `uptime` load average seen in this session; treated as contamination, not a real result (see below) |

## Result

**Single-core: a small, same-direction lean across all three runs (`4096` faster: -3.0%, -2.2%,
-1.3%), two of three crossing this workspace's 8-9/9 "consistent" bar.** This is closer to
candidate 074's original magnitude (a fraction of a percent to a few percent) than to candidate
110's (-10.0%/-2.6%) - no run here came close to 110's largest figure - but it agrees with 110 on
*direction* (`4096` faster, not slower) and disagrees with 074's original direction (074 found
`4096` slightly *slower*, +0.5%, 8/9). Across three independent runs in one quiet-ish session,
the single-core signal leans consistently toward `4096` being faster on this scene by roughly
1-3%, not the near-zero-or-negative effect 074 originally reported, and not the double-digit
effect 110's first run reported.

**Multi-core: still not resolved, and this session adds a clear illustration of why.** Two runs
landed on opposite sides of even (-2.3% and +0.9%, both short of the consistency bar); the third
run produced a large, "consistent-by-the-script's-count-rule" result in the *opposite* direction
from the single-core lean (+19.9%, `256` faster) that lines up with a visible `uptime` load spike
(6.07, the highest seen this session) and a bimodal-looking outlier pattern (5 of 9 B-side runs
at ~190-195ms, 4 of 9 at the same ~150-162ms as every other run in this table) inside a single
paired run - the same specific contamination the "not consistent" verdict is designed to allow
`paired.sh`'s alternation to catch across two binaries, except here it also produces a false
"consistent" reading because the outliers all clustered on one side by chance. This is treated as
noise, not a real regression, for that reason - not folded into "multi-core leans toward 256."

**Net for the original 074-vs-110 contradiction:** this candidate does not reproduce either
prior session's exact magnitude. It sides with 110 on direction (single-core: `4096` faster) but
at a magnitude far below 110's own headline figure and much more consistent than 074's single,
opposite-direction pass. Multi-core remains genuinely unresolved across all three sessions now
(074: wash; 110: leaning-but-short-of-bar; 111: two washes plus one contamination-driven false
reading) - three sessions in a row have failed to produce a clean multi-core verdict either way.

## Does it change behavior/output?

- **Bit-exact.** `tools/pixelcheck.sh` (`RUNS=6`), `256` vs. `4096` on `scroll --row-style layout
  --scroll-step 0`: 0 px noise floor, 0 px raw difference, pixel-identical.
- **Test suite, run in this candidate:**
  - Before (plain `master`, no `chunked_arena` patch): `cargo test -p i-slint-core --lib` - 307
    passed, 0 failed; `cargo test -p slint --test partial_renderer` - 24 passed, 0 failed.
  - After (patch applied, `CHUNK_SIZE = 4096`): `cargo test -p i-slint-core --lib` - 307 passed,
    0 failed; `cargo test -p slint --test partial_renderer` - 24 passed, 0 failed. No new
    failures, no failure-count change.
- **Is this a breaking change?** No - `CHUNK_SIZE` is a private `const` inside a private module,
  not part of any public surface, same as every prior candidate in this sweep.

## Why the result looks the way it does

This session's own multi-core run 3 is itself the clearest evidence for the leading hypothesis
already named in 110's `RESULT.md`: `taskset -c N` pinning does not isolate a core from other
schedulable work on this host/container (candidate 087's finding, IDEAS.md row 31), and this
container has no available mechanism (`isolcpus`, or a writable cgroup cpuset) to do better -
`/sys/fs/cgroup` is `root:root 755` and this session runs as an unprivileged user, and the
visible kernel cmdline has no `isolcpus`. Session-to-session (and even run-to-run within one
session) host contention is the most parsimonious explanation for why three sessions measuring
the exact same code, scene, and invocation keep landing on different single-core magnitudes and
an unresolved multi-core picture: `uptime`'s load average moved measurably within this single
session (2.96 to 6.07) and the one run with the highest load average is also the one run whose
result looks least trustworthy (a bimodal outlier split landing entirely on one side of the
alternation by chance). Nothing in this candidate points to a code-level explanation - the same
`base_commit` was used in all three sessions.

## What's not done / open questions

- **Multi-core is still unresolved after three sessions.** A fourth session is unlikely to
  settle it without genuine core isolation (`isolcpus` at boot, or root access to configure a
  cgroup cpuset excluding other host processes from the pinned core) - this environment cannot
  provide either without an explicit, out-of-band decision to escalate privileges or reboot the
  host, which this skill's standing rules say not to do unilaterally.
- **Single-core now has a 1-3% lean toward `4096`** across three runs in one session, more
  consistent than either prior session's own single run/two runs, but still short of the kind of
  margin (074's +0.5% and 110's up-to-10%) that would make a confident recommendation either way
  - this workspace does not issue recommendations regardless (see SKILL.md's "one rule that
  matters most"), but a reader weighing this data should note the *effect size* itself does not
  triangulate to a single number across the three sessions, only the *sign* does (weakly, 2 of 3
  sessions agree `4096` is faster single-core; 074 disagreed).
- No `perf`/allocation-profiler attribution was attempted, same limitation every candidate in
  this sweep has noted (`perf_event_open` blocked in this environment).
