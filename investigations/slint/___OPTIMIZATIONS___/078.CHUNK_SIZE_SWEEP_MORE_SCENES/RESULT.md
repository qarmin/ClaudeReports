# 78 - Sweep candidate 073's `chunked_arena::CHUNK_SIZE` on `layout`/`flat` scrolling, `rects`, and `list` scenes

IDEAS.md's backlog (added while closing out candidate 074) asked whether `CHUNK_SIZE` matters on
scenes candidate 074 didn't sweep: 074 only tested the `layout` static scene by name, plus one
inconclusive `bare` scrolling attempt (a bimodal/lottery-mode scene that produces no usable
paired signal for this parameter). This candidate sweeps two representative values from 074's
table - `64` (074's clear, consistent regression on `layout` static) and `1024` (074's best-
looking, though not decisive, candidate on `layout` static) - across the four scenes IDEAS.md
named: `layout` scrolling, `flat` scrolling, `rects`, and `list`.

Same prerequisite as 074: `chunked_arena` does not exist on `master`, so candidate 073's patch
(`CHUNK_SIZE = 256`) is applied first as the baseline every swept value is compared against;
`patch.diff` here is that same 073 patch, unmodified. The two swept values are one-line edits to
`internal/core/partial_renderer.rs`'s `const CHUNK_SIZE` on top of it, not separate patches.

## What was tried

Three binaries (`CHUNK_SIZE` = 256, 64, 1024), `cargo build --bin perf_test --profile rdebug`,
compared pairwise via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol as 073/074/075.
Scenes: `scroll --count 100000 --row-style layout` (scrolling, not static - the "wash" scene from
073's own table), `scroll --count 100000 --row-style flat` (scrolling), `rects --count 3000`,
`list --count 100000`. Each row's median pair is (256 A-side of that specific invocation) ->
(swept-value B-side of the same invocation); 256 is re-measured per pairing, not reused.

| Scene | Value | single-core (`taskset -c 2`) | multi-core | notes |
|---|---|---|---|---|
| `layout` scrolling | 64 | median 157.51 -> 159.18ms (+1.1%, 9/9 paired) | median 156.59 -> 159.05ms (+1.6%, 9/9 paired) | consistent regression, both cores - matches 074's static-scene direction for `64` |
| `layout` scrolling | 1024 | median 157.97 -> 159.27ms (+0.8%, 2/9) | median 156.72 -> 157.90ms (+0.8%, 2/9) | NOT consistent, both cores - wash |
| `flat` scrolling | 64 | median 117.68 -> 118.46ms (+0.7%, 3/9) | median 117.88 -> 118.50ms (+0.5%, 4/9) | NOT consistent, both cores - wash |
| `flat` scrolling | 1024 | median 116.37 -> 113.82ms (-2.2%, 6/9); rerun: 115.88 -> 113.43ms (-2.1%, 5/9) | median 116.15 -> 110.44ms (**-4.9%, 9/9 paired**); rerun: 118.17 -> 112.79ms (**-4.6%, 9/9 paired**) | **multi-core only**: consistent win, reproduced twice; single-core leans the same direction (-2.1/-2.2%) but never reaches the 8-9/9 bar |
| `rects` (count 3000) | 64 | median 4.46 -> 4.46ms (+0.0%, 3/9) | median 4.53 -> 4.54ms (+0.2%, 3/9) | NOT consistent - no signal at this scale |
| `rects` (count 3000) | 1024 | median 4.47 -> 4.46ms (-0.2%, 5/9) | median 4.54 -> 4.55ms (+0.2%, 5/9) | NOT consistent - no signal |
| `rects` (count 50000, supplementary) | 64 | median 120.04 -> 123.46ms (+2.8%, 3/9) | not measured | NOT consistent even at a much larger scene |
| `list` (count 100000) | 64 | median 2.41 -> 2.41ms (+0.0%, 2/9) | median 2.45 -> 2.44ms (-0.4%, 3/9) | NOT consistent - no signal |
| `list` (count 100000) | 1024 | median 2.41 -> 2.41ms (+0.0%, 0/9, "consistent" per the tool's `<=1`-win rule but a literal 0.0% median move) | median 2.42 -> 2.43ms (+0.4%, 3/9) | single-core "consistent" reading is a zero-magnitude tie, not a real effect; multi-core not consistent |

## Result

Two real findings, one wash confirmed at a new scene, and two scenes with essentially no
measurable signal at any tested chunk size:

1. **`layout` scrolling repeats 074's `64`-regression finding**: +1.1%/+1.6% (single-/multi-core,
   9/9 both), same direction and similar magnitude to 074's `layout` static result
   (+3.7%/+4.8%). `1024` is a wash here (2/9 both cores), also consistent with 074's static-scene
   finding for the same value.
2. **`flat` scrolling at `1024` is a new, real, but core-count-dependent win**: multi-core is
   consistent and reproduced across two independent runs (-4.9%, -4.6%, 9/9 both times);
   single-core leans the same direction on both runs (-2.1%, -2.2%) but never reaches this
   workspace's 8-9/9 consistency bar. This is the first scene in this whole `CHUNK_SIZE` sweep
   (073/074/075/078 combined) where a swept value produces a paired-consistent win on any core
   count - 074 found nothing but washes and one small `4096` single-core regression on `layout`
   static, and 075's non-power-of-two value found nothing at all.
3. **`rects` and `list` show no signal at either swept value, on either core count**, including a
   supplementary `rects --count 50000` run (a scene an order of magnitude bigger than 074's
   default 3000, still no consistent signal for `64`). Per-frame times on these scenes (4.4-4.6ms
   for `rects`, 2.4ms for `list`) are roughly 30-60x shorter than the scrolling scenes (110-160ms)
   at these counts - `list` is virtualized (only visible rows ever occupy the arena regardless of
   `--count`) and `rects --count 3000/50000` renders fast enough that whatever chunk-locality cost
   `CHUNK_SIZE` imposes on the arena is too small a fraction of the frame to surface above this
   measurement's noise floor.

Net: `CHUNK_SIZE` does matter on some scrolling scenes beyond the one candidate 074 named, but the
effect (where present) is small (a few percent) and, in `flat` scrolling's case, visible on one
core count and not the other - not a decisive case for changing 073's `256` default either
direction. `rects`/`list` remain fully unaffected by this parameter at the scales tested.

## Does it change behavior/output?

- **Bit-exact outside the noise floor**, `tools/pixelcheck.sh` (`RUNS=6`), `256` vs. the relevant
  swept value, on every scene in this sweep:
  - `flat` scrolling, `256` vs. `1024`: 245px noise floor, 201px raw difference, 0px outside it.
  - `rects --count 3000`, `256` vs. `64`: 0px noise floor, 0px raw difference.
  - `list --count 100000`, `256` vs. `64`: 194px noise floor, 114px raw difference, 0px outside it.
  - (`layout` scrolling and `rects`/`list` at `1024` were not separately pixel-checked beyond
    these three - candidate 074 already established bit-exactness for `CHUNK_SIZE` changes on two
    other scenes including a `--mutate` free-list-exercising one, and nothing about this
    candidate's implementation differs from 074's, only the scenes measured.)
- No test suite run in this candidate specifically - `cargo test -p i-slint-core --lib` and
  `cargo test -p slint --test partial_renderer` were already reported unchanged (307/307, 24/24)
  by candidate 074 for the same underlying patch at `CHUNK_SIZE = 32`, the smallest value tried
  across this whole sweep; nothing in this candidate's implementation changes that scope.
- **Is this a breaking change?** No - same as 073/074/075: `CHUNK_SIZE` is a private `const`
  inside a private module, not part of any public surface.

## Why the result looks the way it does

`rects`/`list` showing no signal is the most legible result here: `list` is a virtualized
`ListView`, so regardless of `--count`, only the visible rows (a few dozen) ever have a live
`PartialRenderingCachedData` entry in the arena at once - far too few to make chunk count or
chunk-internal layout matter, unlike the scrolling scenes' `--count 100000` where the whole model
backs live arena entries. `rects` at `count 3000` (or even the supplementary 50000) is dominated
by per-primitive femtovg/software rendering cost per candidate 006's and others' prior findings
in this workspace, not by arena traversal - frames are 30-60x shorter than the scrolling scenes,
leaving too little relative time in the arena-touching code path for a chunk-locality difference
to clear the measurement noise floor.

`flat` scrolling's `1024` result being multi-core-only is the more interesting, less certain one.
It was not further isolated under this candidate's budget (no `perf`, same limitation every
candidate in this sweep has noted), but a plausible, unconfirmed explanation: unpinned multi-core
runs let the allocator/kernel schedule the arena's chunk allocations (`Box::new(core::array::from_fn(...))`
per new chunk) across cores/NUMA-adjacent memory differently than a single pinned core does, and
`1024`'s fewer, larger chunks (about 98 chunks for 100 000 rows, vs. 1563 for `64` or 391 for
`256`) may interact with that scheduling in a way a single pinned core (which always uses the
same core-local cache hierarchy every run) does not exhibit. This is consistent with, but not
proof of, `74`'s own reasoning that larger chunks trade a startup/scroll-extension allocation cost
for better steady-state cache locality - `flat` scrolling's rows are the shallowest of the three
`--row-style` variants (fewest sub-items per row, per `README.md`'s scene table), which may leave
more of the frame budget for arena-traversal cost (where `CHUNK_SIZE` matters) relative to
per-item layout/paint cost (where it doesn't) than `layout`'s deeper rows do - a plausible reason
this scene, and not `layout` scrolling, is the one that shows a `1024` win, but this was reasoned
from the scene definitions, not measured directly.

## What's not done / open questions

- No `perf`/allocation-profiler attribution was attempted for the `flat`-scrolling `1024` result
  (or any result in this candidate) - same limitation every candidate in this sweep has noted
  (`perf_event_open` blocked in this environment). The core-count-dependent explanation above is
  reasoned from the pattern of results, not confirmed by a trace.
- `layout` scrolling and `flat` scrolling were only tested at `64` and `1024`, not the full
  32/512/4096 range candidate 074 used for `layout` static - if `flat` scrolling's `1024` result
  is real, `512`/`4096` might sharpen or bracket it further, untested here.
- `rects`/`list` were only tested at `64` and `1024` too, and only at two item counts each
  (3000/50000 for `rects`, 100000 for `list`) - a scene specifically constructed to force a much
  larger live arena population (e.g. a non-virtualized `list`-like scene, if one existed) might
  still show a signal this sweep's two scenes could not.
- The `flat`-scrolling multi-core win was reproduced twice (both 9/9), but the single-core result
  was also reproduced twice and never crossed the consistency bar either time (6/9, then 5/9) -
  a third single-core rerun was not attempted; it is reported as "leans the same direction, not
  decisive" rather than pushed further to try to force a verdict.
- As with 074/075, this candidate does not recommend a `CHUNK_SIZE` value different from 073's
  original 256 - see "Result" above for why the one real effect found (`flat` scrolling, `1024`,
  multi-core) is narrow (one scene, one core count) rather than a clear case to change the default.
