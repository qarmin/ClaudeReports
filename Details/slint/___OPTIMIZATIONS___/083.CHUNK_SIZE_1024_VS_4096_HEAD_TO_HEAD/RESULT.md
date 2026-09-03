# 83 - Directly pit candidate 073's `chunked_arena::CHUNK_SIZE = 1024` against `4096` on `flat`/`layout` scrolling

IDEAS.md's backlog (added while closing out candidate 080) noted that 078's `1024` and 080's
`4096` were each only ever benchmarked against the `256` baseline, never against each other -
078 found `1024` a multi-core-only win on `flat` scrolling (-4.9%/-4.6%, wash on `layout`), 080
found `4096` a broader win on both scrolling scenes and mostly both core counts (`flat`:
-4.6/-5.0%, `layout`: -2.4/-3.2%), but it was unestablished whether `4096` is actually better than
`1024` or the two are statistically indistinguishable (both simply beating `256`). This candidate
compares them directly, head-to-head, with no `256` binary involved at all.

Same prerequisite as 073/074/075/078/080: `chunked_arena` does not exist on `master`, so
candidate 073's patch (`CHUNK_SIZE = 256`) is applied first; `patch.diff` here is that same 073
patch, unmodified. `1024` and `4096` are one-line edits to
`internal/core/partial_renderer.rs`'s `const CHUNK_SIZE` on top of it, not separate patches -
`patch.diff` alone reproduces neither swept value by itself, see the `apply` step in `justfile`.

## What was tried

Two binaries (`CHUNK_SIZE` = 1024, 4096) built directly (patch.diff applied then the constant
edited to each value in turn), `cargo build --bin perf_test --profile rdebug`, compared pairwise
via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol as 073/074/075/078/080. Scenes:
`scroll --count 100000 --row-style flat` and `scroll --count 100000 --row-style layout` (both
scrolling). Each row is (1024 A-side) -> (4096 B-side) of the same invocation; per the
established protocol, each scene/core-count combination was run twice to check reproducibility.

| Scene | core count | run 1 | run 2 | notes |
|---|---|---|---|---|
| `flat` scrolling | single-core (`taskset -c 2`) | median 118.66 -> 121.54ms (+2.4%, B faster 2/9, i.e. **1024 faster 7/9**) | median 114.88 -> 117.58ms (+2.4%, B faster 1/9, i.e. **1024 faster 8/9**) | consistent both runs, identical magnitude: **1024 beats 4096** |
| `flat` scrolling | multi-core | median 116.76 -> 116.70ms (-0.1%, B faster 4/9) | median 116.50 -> 115.58ms (-0.8%, B faster 5/9) | NOT consistent either run - wash |
| `layout` scrolling | single-core (`taskset -c 2`) | median 158.68 -> 158.46ms (-0.1%, B faster 6/9) | median 157.95 -> 154.04ms (-2.5%, B faster 9/9) | inconsistent between the two runs themselves (wash, then consistent) - mixed |
| `layout` scrolling | multi-core | median 156.48 -> 155.11ms (-0.9%, B faster 6/9) | median 156.47 -> 153.16ms (-2.1%, B faster 7/9) | NOT consistent either run, but both lean the same direction (4096 faster) |

## Result

The two values are **not** statistically indistinguishable copies of each other beating `256` by
coincidence - they diverge in opposite directions depending on scene:

1. **`flat` scrolling, single-core: `1024` is the reproducibly faster value.** Both runs show
   `1024` winning by the same +2.4% margin (7/9 and 8/9 pairs), which is close to this workspace's
   8-9/9 consistency bar and identical in magnitude both times - the most internally consistent
   result in this candidate. This is notable because 078 found `1024`'s `flat`-scrolling win over
   `256` was multi-core-only (single-core never reached its consistency bar there); here, compared
   directly against `4096` rather than `256`, `1024` shows a clean single-core advantage.
2. **`flat` scrolling, multi-core: a wash** (-0.1%, -0.8%, neither consistent) - despite 078 and 080
   each independently finding a multi-core win for their respective value over `256` on this same
   scene (078: `1024` -4.9%/-4.6%; 080: `4096` -4.7%/-4.6%), the two wins are close enough to each
   other that head-to-head neither beats the other on multi-core.
3. **`layout` scrolling is genuinely noisy at both core counts** - each core count's own two runs
   disagree with each other (single-core: wash then 9/9 consistent at -2.5%; multi-core: 6/9 then
   7/9, both leaning `4096` but neither crossing the bar). Unlike `flat` scrolling's clean,
   reproducible split, `layout` scrolling does not produce a decisive answer either direction
   within this candidate's two-runs-per-combination budget.

Net: this establishes that `1024` and `4096` are **not** equivalent - `flat` scrolling shows a
real, reproducible single-core preference for `1024` that neither 078 nor 080 could see by
comparing each only to `256` - but does not produce a single global answer, since `layout`
scrolling's own two runs don't agree with each other and `flat` scrolling's multi-core result is a
wash either way.

## Does it change behavior/output?

- **Bit-exact outside the noise floor**, `tools/pixelcheck.sh` (`RUNS=6`), `1024` vs. `4096`
  directly:
  - `flat` scrolling: 246px noise floor, 207px raw difference, 0px outside it.
  - `layout` scrolling: 193px noise floor, 112px raw difference, 0px outside it.
- **Test suite run in this candidate specifically**, both before applying anything (plain
  `master`, no `chunked_arena` patch at all) and after (candidate 073's patch applied,
  `CHUNK_SIZE = 4096` - the same underlying code path exercised at `CHUNK_SIZE = 1024` too, since
  the constant only changes chunk sizing, not the logic any test exercises):
  - Before (plain `master`): `cargo test -p i-slint-core --lib` -> 307 passed, 0 failed;
    `cargo test -p slint --test partial_renderer` -> 24 passed, 0 failed. `grep -c '^error'` on
    both logs: 0.
  - After (patch applied, `CHUNK_SIZE = 4096`): `cargo test -p i-slint-core --lib` -> 307 passed,
    0 failed; `cargo test -p slint --test partial_renderer` -> 24 passed, 0 failed. `grep -c
    '^error'` on both logs: 0. No new failures, no failure-count change.
- **Is this a breaking change?** No - same as 073/074/075/078/080: `CHUNK_SIZE` is a private
  `const` inside a private module, not part of any public surface.

## Why the result looks the way it does

Not independently re-derived beyond what 078/080 already reasoned (no `perf`/allocation-profiler
attribution was attempted here either, same `perf_event_open` limitation noted by every candidate
in this sweep). The one new data point this candidate adds: `flat` scrolling's single-core
preference for `1024` over `4096` only became visible in a *direct* comparison - both 078 (`1024`
vs `256`) and 080 (`4096` vs `256`) individually found their value's single-core signal on `flat`
scrolling too weak to cross the consistency bar (078: `1024` single-core leaned -2.1/-2.2% but
never above 6/9; 080: `4096` single-core was -2.9%/-5.0% at 6/9 and 8/9). Comparing the two
swept values against each other rather than each against a third, slower baseline apparently
cancels out enough shared noise/overhead to surface a difference between them that neither
solo comparison could resolve - consistent with this workspace's general finding (`MEASUREMENT.md`
section 2b) that this machine's per-process performance-mode noise dominates small effects unless
paired directly against the exact thing being compared.

## What's not done / open questions

- `layout` scrolling's disagreement between its own two runs (both core counts) was not resolved
  with a third rerun - per the task's specified protocol (two reruns per core count), this
  candidate stops at two and reports the result as genuinely mixed rather than forcing a verdict.
- No `perf`/allocation-profiler attribution was attempted for any result here - same limitation
  every candidate in this `CHUNK_SIZE` sweep has noted (`perf_event_open` blocked in this
  environment). Why `flat` scrolling's single-core split is reproducible while `layout`
  scrolling's is not remains reasoned from the pattern of results, not measured directly.
- `rects`/`list` were not retested here - 078 already found no signal at any tested `CHUNK_SIZE`
  value on those two scenes, and this candidate's scope (per IDEAS.md) was `flat`/`layout`
  scrolling specifically, matching 078/080's own scoping.
- As with every candidate in this sweep, this does not recommend a `CHUNK_SIZE` value different
  from 073's original `256` default - see "Result" above for why: the one clear finding here
  (`1024` beats `4096` on `flat` scrolling's single-core path) is narrower than either 078's or
  080's own claims about beating `256`, and `layout` scrolling remains unresolved between the two
  values.
