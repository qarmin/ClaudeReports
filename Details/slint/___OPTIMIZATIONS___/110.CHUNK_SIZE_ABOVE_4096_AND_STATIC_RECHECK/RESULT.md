# 110 - Sweep `chunked_arena::CHUNK_SIZE` above 4096, and re-verify 074's `layout` static regression at 4096

IDEAS.md's backlog (added while closing out candidate 080) asked two things: (1) whether
candidate 080's broadest confirmed scrolling win (`CHUNK_SIZE = 4096`, on both `flat` and
`layout` scrolling) keeps climbing at larger values (`8192`, `16384`) or plateaus/reverses, and
(2) whether candidate 074's `layout` **static**-scene regression at `4096` (a small,
single-core-only, single-pass finding: +0.5%, 8/9 paired) still reproduces under 080's tighter
two-reruns-per-core-count protocol.

Same prerequisite as 073/074/075/078/080: `chunked_arena` does not exist on `master`, so
candidate 073's patch (`CHUNK_SIZE = 256`) is applied first as the baseline scaffold; `patch.diff`
here is that same 073 patch, unmodified. `8192`/`16384` are one-line edits to
`internal/core/partial_renderer.rs`'s `const CHUNK_SIZE` on top of it, not separate patches.

## What was tried

Four binaries (`CHUNK_SIZE` = 256, 4096, 8192, 16384), `cargo build --bin perf_test --profile
rdebug`, compared pairwise via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol as
073/074/075/078/080.

**Methodology note:** the scrolling comparisons were first run with `--scroll-step 1` by mistake
(not matching 080's methodology, which used the tool's default `--scroll-step` of 200px for its
"scrolling" scenes). That first pass is discarded entirely - not reported below - and every
scrolling comparison was re-run with the correct default scroll step before being recorded here.

Scrolling scenes: `scroll --count 100000 --row-style layout` and `scroll --count 100000
--row-style flat` (default `--scroll-step`, i.e. 200px/frame, matching 080). Static scene (the
074 recheck): `scroll --count 100000 --row-style layout --scroll-step 0`, matching 074 exactly.

Each row's median pair is (A-side of that specific invocation) -> (B-side of the same
invocation); the reference side is re-measured per pairing, not reused across pairings.

### Part 1: `4096` vs `8192` / `16384` on scrolling scenes

| Scene | Comparison | single-core (`taskset -c 2`) | multi-core | notes |
|---|---|---|---|---|
| `layout` scrolling | 4096 vs 8192 | run1: median 161.07 -> 154.34ms (**-4.2%, 9/9**); rerun: median 162.17 -> 159.70ms (-1.5%, 6/9) | median 155.71 -> 159.66ms (+2.5%, 2/9) | single-core disagrees with itself between the two runs (9/9 then 6/9) - not reproducible; multi-core is a wash |
| `layout` scrolling | 4096 vs 16384 | median 161.85 -> 160.32ms (-0.9%, 7/9) | median 164.24 -> 162.94ms (-0.8%, 5/9) | NOT consistent either core count - small same-direction lean single-core, wash multi-core |
| `flat` scrolling | 4096 vs 8192 | median 117.14 -> 116.86ms (-0.2%, 5/9) | median 113.26 -> 112.67ms (-0.5%, 5/9) | wash, both cores |
| `flat` scrolling | 4096 vs 16384 | median 116.46 -> 119.05ms (+2.2%, 4/9) | median 111.67 -> 111.34ms (-0.3%, 5/9) | wash, both cores |

### Part 2: re-verify 074's `layout` static regression at `4096` (256 vs 4096, `--scroll-step 0`)

| Core count | Run | Result |
|---|---|---|
| single-core | 1 | median 166.55 -> 149.94ms (**-10.0%, 9/9 consistent**) |
| single-core | 2 (rerun) | median 152.13 -> 148.25ms (**-2.6%, 9/9 consistent**) |
| multi-core | 1 | median 152.39 -> 147.21ms (**-3.4%, 9/9 consistent**) |
| multi-core | 2 (rerun) | median 171.87 -> 167.88ms (-2.3%, 7/9, same direction, short of the bar) |

`uptime` load average during this part of the session: 4.79-6.60 (well under the 24-core count,
so no automatic re-baseline trigger per this workspace's rule, but noticeably higher than a quiet
host).

## Result

**Part 1 (values above 4096):** no consistent win over `4096` at either `8192` or `16384`, on
either scrolling scene, at either core count, once the scroll-step methodology matches 080's. The
one exception - `layout` scrolling single-core at `8192`, 9/9 on its first run - did not reproduce
on rerun (6/9, smaller magnitude), so per this workspace's disagreement-handling convention
(candidate 083) it is reported as unresolved/not real rather than a win. **The effect plateaus at
`4096`; it does not keep climbing.**

**Part 2 (the static-scene recheck):** this candidate's remeasurement does **not** reproduce
074's regression. Instead, under this session's protocol (two runs per core count), `4096` shows
a **consistent win** over `256` on `layout` static: single-core consistent both runs (-10.0% then
-2.6%, both 9/9), multi-core consistent on the first run (-3.4%, 9/9) and leaning the same
direction but short of the bar on the second (-2.3%, 7/9). This is the opposite direction and a
much larger magnitude than 074's original finding (a +0.5% single-core-only regression, 8/9, and
a multi-core wash). Both the sign and the magnitude disagree with 074; this candidate does not
resolve which measurement better reflects the code's actual behavior, only that the two sessions'
results disagree sharply on the same code and the same scene.

## Does it change behavior/output?

- **Bit-exact.** `tools/pixelcheck.sh` (`RUNS=6`), on the corrected (default scroll-step) scenes:
  - `flat` scrolling, `4096` vs `8192`: 0px noise floor, 0px raw difference.
  - `layout` scrolling, `4096` vs `8192`: 0px noise floor, 0px raw difference.
  - `flat` scrolling, `4096` vs `16384`: 0px noise floor, 0px raw difference.
  - `layout` scrolling, `4096` vs `16384`: 0px noise floor, 0px raw difference.
  - `layout` static, `256` vs `4096` (`--scroll-step 0`): 0px noise floor, 0px raw difference.
  All five comparisons were literally pixel-identical (not merely "outside the noise floor"),
  unlike 080's comparisons at the same scenes which had a nonzero (but fully noise-floor-covered)
  raw difference - likely due to `pixelcheck.sh`'s short default `FRAMES=6` landing on an earlier,
  less-scrolled frame than 080's measurement happened to compare.
- **Test suite, run in this candidate specifically**, `cargo test -p i-slint-core --lib` and
  `cargo test -p slint --test partial_renderer`, run twice each - once on plain `master` (no
  `chunked_arena` patch) and once with candidate 073's patch applied at `CHUNK_SIZE = 4096` (the
  value both parts of this candidate center on):
  - Before (plain `master`): `i-slint-core --lib` 307 passed, 0 failed; `slint --test
    partial_renderer` 24 passed, 0 failed.
  - After (patch applied, `CHUNK_SIZE = 4096`): `i-slint-core --lib` 307 passed, 0 failed; `slint
    --test partial_renderer` 24 passed, 0 failed. No new failures, no failure-count change.
- **Is this a breaking change?** No - `CHUNK_SIZE` is a private `const` inside a private module,
  not part of any public surface, same as every prior candidate in this sweep.

## Why the result looks the way it does

**Part 1** is consistent with the reasoning 073/074/080 already gave: `4096` needs only ~25
chunks for a 100 000-row scene, already few enough that further halving the chunk count (to ~13
at `8192`, ~7 at `16384`) has little further locality benefit for the outer `Vec`, while each
chunk's own `Box<[Slot<T>; N]>` allocation keeps growing (proportionally larger first-touch cost
after `grow()`). If there is a real curve here, `4096` already sits at or past its knee - larger
chunks buy no further access-locality win and the per-chunk allocation cost keeps growing, a
wash at best.

**Part 2's reversal is not explained by anything measured in this candidate.** Two honest
candidates: (a) session-to-session host noise - `taskset -c N` pinning is known (candidate 087,
IDEAS.md row 31) not to isolate a core from all other schedulable work, and this session's load
average (4.8-6.6) was higher than typical for this workspace, which could inflate the `256`
baseline's variance enough to flip which side "wins" a paired comparison; the `256` A-side's
raw single-core run1 numbers (151.83-174.96ms, a ~23ms spread) are markedly noisier than the
`4096` B-side's (147.21-152.10ms, a ~5ms spread) in that same run, consistent with intermittent
contention landing disproportionately on whichever binary runs first in a pair. (b) A genuine
change in which code path dominates this scene between 074's original session and now - nothing
in the source changed (same `base_commit`, `cdb6c43f7`, in both), so this would have to be a
purely environmental/scheduling effect, not a code difference. This candidate cannot distinguish
(a) from (b); see "What's not done" below.

## What's not done / open questions

- **The Part 2 reversal is unresolved.** Neither 074's original regression nor this candidate's
  win can be preferred over the other from the evidence gathered so far - a third, independent
  session (ideally on a quiet host, `uptime` load close to 0, and/or with `isolcpus`/cgroup
  isolation per IDEAS.md row 31) is needed to settle which one reflects the code's real behavior,
  if either does cleanly.
- Part 1's `layout` scrolling single-core `8192` result (9/9 then 6/9) was treated as
  unresolved/not real per the disagreement-handling convention from candidate 083, rather than
  investigated further (e.g. a third rerun) - budget for this candidate was spent covering both
  parts of IDEAS.md's original ask instead.
- No `perf`/allocation-profiler attribution was attempted for any result here, same limitation
  every candidate in this `CHUNK_SIZE` sweep has noted (`perf_event_open` blocked in this
  environment).
- Non-power-of-two values above 4096, and values between 4096 and 8192, remain untested - 075
  already established power-of-two-ness doesn't matter for the compiled indexing, so this would
  only matter if the true optimum sits strictly between the values already tried.
- As with every prior candidate in this sweep, this candidate does not recommend a `CHUNK_SIZE`
  value different from 073's original 256 - the reader decides what to do with the fact that
  `4096`'s scrolling win (established by 080) does not extend further, and that its static-scene
  effect is now measured in two contradictory directions across two sessions.
