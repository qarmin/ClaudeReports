# 80 - Sweep candidate 073's `chunked_arena::CHUNK_SIZE` at `512`/`4096` on `flat`/`layout` scrolling scenes

IDEAS.md's backlog (added while closing out candidate 078) noted that 078 only tested `64`/`1024`
(074's two most-informative values from the `layout` *static* sweep) on the newly-covered `flat`/
`layout` *scrolling* scenes, and never checked `512`/`4096` - the two values 074 found least
interesting on `layout` static - even though `flat` scrolling's response to `1024` (a real,
multi-core-only win) already diverged from `layout` static's (a wash). This candidate fills that
gap: `512` and `4096` on both scrolling scenes, both core counts.

Same prerequisite as 074/078: `chunked_arena` does not exist on `master`, so candidate 073's patch
(`CHUNK_SIZE = 256`) is applied first as the baseline every swept value is compared against;
`patch.diff` here is that same 073 patch, unmodified. `512` and `4096` are one-line edits to
`internal/core/partial_renderer.rs`'s `const CHUNK_SIZE` on top of it, not separate patches.

## What was tried

Three binaries (`CHUNK_SIZE` = 256, 512, 4096), `cargo build --bin perf_test --profile rdebug`,
compared pairwise via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol as 073/074/075/078.
Scenes: `scroll --count 100000 --row-style layout` and `scroll --count 100000 --row-style flat`
(both scrolling, not static). Each row's median pair is (256 A-side of that specific invocation)
-> (swept-value B-side of the same invocation); 256 is re-measured per pairing, not reused.
Paired-consistent results were rerun once to check reproducibility, same discipline as 078's
`flat`/`1024` finding.

| Scene | Value | single-core (`taskset -c 2`) | multi-core | notes |
|---|---|---|---|---|
| `layout` scrolling | 512 | median 157.81 -> 158.89ms (+0.7%, 1/9) | median 156.78 -> 156.82ms (+0.0%, 3/9) | NOT consistent, both cores - wash |
| `layout` scrolling | 4096 | median 157.49 -> 152.42ms (**-3.2%, 9/9**); rerun: 157.09 -> 152.31ms (-3.0%, 7/9) | median 156.38 -> 152.01ms (**-2.8%, 8/9**); rerun: 155.81 -> 152.00ms (**-2.4%, 9/9**) | consistent on 3 of 4 runs, same direction/magnitude on the 4th (7/9) - a real win |
| `flat` scrolling | 512 | median 120.82 -> 116.62ms (-3.5%, 6/9) | median 117.01 -> 113.81ms (-2.7%, 6/9) | NOT consistent, both cores - wash (medians lean the same direction as 4096 but never reach 8-9/9) |
| `flat` scrolling | 4096 | median 118.63 -> 115.14ms (-2.9%, 6/9); rerun: 118.10 -> 112.21ms (**-5.0%, 8/9**) | median 118.44 -> 112.86ms (**-4.7%, 9/9**); rerun: 119.46 -> 114.02ms (**-4.6%, 8/9**) | consistent on 3 of 4 runs, same direction/magnitude on the 4th (6/9) - a real win, both cores |

## Result

Unlike 078's `1024` (a win on `flat` scrolling only, and only on multi-core), `4096` is a real win
on **both** scrolling scenes and, across the two reruns each, mostly on **both** core counts:

1. **`layout` scrolling at `4096`**: -3.2%/-2.8% first run (9/9, 8/9), -3.0%/-2.4% rerun (7/9,
   9/9). Three of the four single+multi/run1+run2 combinations cross this workspace's 8-9/9
   consistency bar; the fourth (single-core rerun, 7/9) still moves the same direction by the same
   magnitude - not a decisive miss, just short of the strict threshold. This *reverses* 074's
   finding for `layout` **static** at `4096`, which found a small single-core regression there -
   scrolling and static clearly respond differently to this constant, matching the pattern 078
   already established for `1024`.
2. **`flat` scrolling at `4096`**: -2.9%/-4.7% first run (6/9, 9/9), -5.0%/-4.6% rerun (8/9, 8/9).
   Again three of four cross the bar; the first single-core run (6/9) is the outlier, but its
   median still moved -2.9% in the same direction. This is a larger, more solid version of 078's
   `1024` result on the same scene (078: multi-core only, -4.9%/-4.6%, single-core never above
   6/9) - `4096` gets what `1024` was already leaning toward, on single-core too.
3. **`512` is a wash on both scenes, both core counts** - medians lean in the same direction as
   `4096` (faster) on `flat` scrolling (-3.5%/-2.7%) but never reach paired consistency (6/9 both
   times), and essentially flat on `layout` scrolling (+0.7%/+0.0%, 1/9 and 3/9). This matches
   074's finding that `512` was the least interesting value on `layout` static too - the
   uninteresting value stayed uninteresting when moved to scrolling scenes.

Net: of the four `CHUNK_SIZE` values now swept on scrolling scenes across 078 and this candidate
(`64`, `512`, `1024`, `4096`), `4096` is the one with the broadest, most reproducible signal -
real on both `flat` and `layout` scrolling, mostly on both core counts, and larger in magnitude
than `1024`'s narrower multi-core-only win on `flat` alone. It is also the value 074 flagged as a
*regression* on `layout` static, underscoring that a single `CHUNK_SIZE` default cannot be optimal
for both static and scrolling scenes given current measurements.

## Does it change behavior/output?

- **Bit-exact outside the noise floor**, `tools/pixelcheck.sh` (`RUNS=6`), `256` vs. each swept
  value, on both scenes:
  - `flat` scrolling, `256` vs. `4096`: 247px noise floor, 226px raw difference, 0px outside it.
  - `layout` scrolling, `256` vs. `4096`: 194px noise floor, 105px raw difference, 0px outside it.
  - `flat` scrolling, `256` vs. `512`: 244px noise floor, 213px raw difference, 0px outside it.
  - `layout` scrolling, `256` vs. `512`: 194px noise floor, 176px raw difference, 0px outside it.
- **Test suite run in this candidate specifically** (not just cited from an earlier candidate):
  `cargo test -p i-slint-core --lib` and `cargo test -p slint --test partial_renderer`, run twice
  each - once on plain `master` (no `chunked_arena` patch at all) and once with candidate 073's
  patch applied at `CHUNK_SIZE = 4096` (the value this candidate's finding centers on):
  - Before (plain `master`): `i-slint-core --lib` 307 passed, 0 failed; `slint --test
    partial_renderer` 24 passed, 0 failed.
  - After (patch applied, `CHUNK_SIZE = 4096`): `i-slint-core --lib` 307 passed, 0 failed; `slint
    --test partial_renderer` 24 passed, 0 failed. No new failures, no failure-count change.
- **Is this a breaking change?** No - same as 073/074/075/078: `CHUNK_SIZE` is a private `const`
  inside a private module, not part of any public surface.

## Why the result looks the way it does

Not independently re-derived here beyond what 078 already reasoned (no `perf`/allocation-profiler
attribution was attempted in this candidate either, same `perf_event_open` limitation). The
broader pattern across 074/078/this candidate: `layout` and `flat` scrolling both have a **live
arena population that scales with `--count`** (100 000 rows, all backing a real
`PartialRenderingCachedData` entry, unlike `list`'s virtualized handful) - unlike `layout` static,
which never scrolls and so never continuously churns the free list the way a moving viewport does
(new rows entering the arena, old ones freed, repeatedly). A larger `CHUNK_SIZE` means fewer,
bigger chunks and fewer chunk-boundary crossings per scroll step; if the free-list churn from
continuous scrolling is what makes chunk locality matter at all, that would explain why `4096`
helps scrolling scenes specifically while regressing (074) or doing nothing (this candidate did
not retest static scenes) on a static one. This remains reasoned from the pattern of results, not
measured directly - the same caveat 078 recorded for its own explanation of `1024`'s multi-core
split.

## What's not done / open questions

- No `perf`/allocation-profiler attribution was attempted for any result in this candidate -
  same limitation every candidate in this `CHUNK_SIZE` sweep has noted (`perf_event_open` blocked
  in this environment).
- `4096` was not tested against `1024` directly (only each against the `256` baseline) - whether
  `4096` is actually better than `1024`, or the two are statistically indistinguishable from each
  other and both just better than `256`/`512`/`64`, is not established by these numbers.
- Values above `4096`, and non-power-of-two values in that range (075 established `256` and
  power-of-two-ness don't matter for the compiled division/modulo, so this is purely about
  chunk-locality effects, not instruction-selection ones), remain untested - if `4096`'s scrolling
  win is real and monotonic in chunk size, a larger value might do even better, or the effect might
  plateau or reverse; neither is known.
- `rects`/`list` were not retested at `512`/`4096` - 078 already found no signal at any tested
  value on those two scenes, and this candidate's scope (per IDEAS.md) was `flat`/`layout`
  scrolling specifically.
- As with 073/074/075/078, this candidate does not recommend a `CHUNK_SIZE` value different from
  073's original `256` - see "Result" above for why: `4096`'s scrolling win now looks like the
  most reproducible finding across the whole sweep, but 074 found it *regresses* `layout` static,
  so a single global default still cannot be optimal for every scene shape tested so far. Whether
  the scrolling win outweighs the static regression in a real application is a product decision,
  not a measurement this workspace makes.
