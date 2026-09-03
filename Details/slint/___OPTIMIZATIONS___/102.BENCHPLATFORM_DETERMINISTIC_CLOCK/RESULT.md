# 102 - Give `BenchPlatform` a deterministic virtual clock instead of the real system clock

This is a **harness-correctness candidate, not a performance candidate**: it implements the design
decision the top-of-backlog `IDEAS.md` entry (added while building candidate 099) explicitly left
open. Candidate 099 root-caused a residual 4-14px run-to-run pixel difference in `--bench --mode
text --type-text` (rows 796-797) to `perf_test`'s `BenchPlatform` never overriding
`Platform::duration_since_start`, so Slint's animation clock - and therefore any `animate` binding,
here the Fluent `std-widgets` `ScrollBar` thumb's `animate width, height { duration: 150ms; }` -
samples real elapsed wall-clock time between frames instead of a value derived from the frame
count the way scrolling/mutation/resize already are. 099 only proved the mechanism with a
throwaway diagnostic (reverted, not kept); this candidate makes the fix permanent in
`perf_test/src/bench.rs` (untracked in the main repo, like every other `perf_test` harness fix in
this workspace - see candidate 097's precedent).

## The fix

`BenchPlatform` gets an `Rc<Cell<Duration>>` (`virtual_now`), returned by `duration_since_start`.
The frame loop in `run()` advances it by a fixed synthetic 16ms (a nominal 60fps tick) right before
each `slint::platform::update_timers_and_animations()` call, instead of the default
`Platform::duration_since_start` implementation reading `Instant::now()`. This is the exact
mechanism candidate 099's own diagnostic used to prove the theory, now made permanent instead of
thrown away, and scoped only to `--bench` mode (`BenchPlatform` is not used by `perf_test --frames
N`'s real-backend/real-clock path).

No design decision about `--animate` was needed in the end: see "Does it change behavior/output?"
below - `--animate`'s own `--mode rects` scene turns out to have a *second*, unrelated source of
run-to-run non-determinism that this fix does not touch, so its existing "not pixel-comparable"
documentation stays accurate as-is, no new warning code required.

## What was tried

Only one variant - the design space here is "pick the value approach", not "pick the faster
implementation", so there's no losing-variant table. Timing was measured to confirm this
harness-only change (a `Cell` get/set replacing a real-clock read) has no effect on frame cost, not
to find a performance win:

| Variant | single-core p50 (`--mode text --count 10000 --type-text --scroll-step 0`) | multi-core p50 | notes |
|---|---|---|---|
| baseline (real clock, `Instant::now()` via default `Platform::duration_since_start`) | 190.66 ms | 191.57 ms | `tools/paired.sh`, `RUNS=9`, `FRAMES=20` |
| virtual clock (this candidate) | 190.56 ms (-0.1%) | 191.31 ms (-0.1%) | 6/9 and 5/9 pairs faster - **not consistent, a wash**, as expected for a `Cell` access replacing a syscall-backed clock read |

## Result

No measurable performance delta (expected - `taskset -c 3` single-core and unpinned multi-core
both land inside `tools/paired.sh`'s noise floor, 6/9 and 5/9 pairs, "NOT consistent" per its own
verdict line). The actual result of this candidate is a **correctness fix for the harness's
determinism guarantee**, verified directly:

Reproducing candidate 099's exact scenario (`--bench --mode text --count 10000 --type-text
--scroll-step 0 --frames 150`, with candidate 092's patch applied to product source so the
per-frame cost is fast enough to expose the issue - see 099's own `RESULT.md` for why it's
invisible on plain `master`), 20 repeated runs of the identical binary/flags, comparing all pixels
of every run against run 1:

| Tree | Self-differing px across 20 runs | Rows |
|---|---|---|
| unpatched harness (real clock) | 16 | 796-797 |
| this candidate's harness fix (virtual clock) | **0** | - |

## Does it change behavior/output?

- **Bit-exact?** Yes, for the specific mechanism this candidate targets (`animate`-*property*
  transitions sampled by `--bench` mode, demonstrated on the `ScrollBar` thumb case above) - 20/20
  runs identical with the fix, vs. 16 differing px without it, on the same tree/scenario.
- **However, `--animate`'s own scenes are not made pixel-comparable by this fix**, and this
  candidate's own testing found why not just by assumption but by measurement: `--animate`'s
  `animation-tick()` reads the exact same clock (`crate::animations::Instant::now(ctx)` calls
  `ctx.platform().duration_since_start()`, the same value `AnimationDriver::update_animations`
  advances from), so making that clock deterministic does give `animation-tick()` deterministic
  *values* - two runs at `--frames 1` or `--frames 2` are pixel-identical (0 differing px). But
  `--mode rects --animate --count 2000` at `--frames 5/10/20/30` still differs run to run (12204,
  1986, 1168, 12908 differing px respectively - non-monotonic with frame count, so not simple
  accumulation of a small per-frame drift). This is a **second, unidentified non-determinism
  source** specific to that scene, not touched or explained by this candidate - `--animate` keeps
  its existing `___OPTIMIZATIONS___/README.md` "not pixel-comparable" label, now for a different
  (unknown) reason than before. Not chased further here; added to `IDEAS.md`.
- **Is this a breaking change?** No public API changed (the fix is entirely inside `perf_test`'s
  own untracked harness code, not Slint itself). No product behavior changed - a real (non-`--bench`)
  application's `animate` bindings and `animation-tick()` are untouched; only `BenchPlatform`, which
  only `perf_test --bench` uses, got a different clock source.
- Project test suite (`i-slint-core`, unaffected since no product source changed by the version of
  this fix left in the tree - see "Reproduce" for how the ScrollBar-jitter table above was produced
  with candidate 092's patch temporarily applied and then fully reverted):
  `cargo test -p i-slint-core --all-features --lib`: **319 passed, 0 failed**, both before (pristine
  `master`, harness unfixed) and after (pristine `master`, harness fixed) - identical, as expected
  for a harness-only change.

## Why the result looks the way it does

`BenchPlatform` was built to make scrolling/mutation/resize deterministic (all driven directly from
the frame index), but never overrode `Platform::duration_since_start`, so anything reading Slint's
own animation clock inherited the default real-system-clock implementation. Swapping a `Cell` read
for a syscall-backed clock read is performance-neutral by construction (both are O(1), and a
`Cell<Duration>` get/set is if anything cheaper than `Instant::now()`), which is exactly what the
paired timing comparison shows - a wash, not a regression or a win, because this was never a
performance change to begin with.

## What's not done / open questions

- **`--mode rects --animate`'s second non-determinism source is unidentified.** Found here as a
  side effect of checking whether this fix would also make `--animate` pixel-comparable (it
  doesn't) - added to `IDEAS.md` rather than chased down in this candidate, which is scoped to the
  `animate`-property-transition mechanism 099 root-caused, not every source of run-to-run jitter in
  every scene.
- **Not checked on `list`/`scroll` modes' own `ScrollBar`** (same Fluent component, same
  `IDEAS.md` entry candidate 099 already flagged as open) - only the `text` mode scenario 099
  originally found was re-verified here.
- **The synthetic 16ms-per-frame value is an arbitrary nominal-60fps choice**, not swept or
  justified beyond "matches candidate 099's own diagnostic and is a plausible real frame interval."
  A different fixed value would change *how far* into a 150ms transition a given frame count lands,
  but not whether the result is deterministic - not expected to matter for the determinism goal,
  unconfirmed whether any existing candidate's saved pixel-comparison numbers assumed a specific
  transition-progress point that this specific value coincidentally produces.
- This candidate's own bench.rs change is kept permanently in the live `perf_test/` tree (matching
  candidate 097's precedent for harness-only fixes - see that candidate's `RESULT.md`), unlike
  every product-source patch in this workspace which is always reverted. `git status` in the main
  project therefore shows `perf_test/src/bench.rs` changed relative to how this session found it,
  which is intentional, not a violation of "always leave the tree clean" (that rule is about
  tracked product source, which is fully reverted - see `patch.diff`, empty).

## Reproduce

```sh
# 1. Reproduce candidate 099's exact residual (needs 092's patch for the per-frame cost to be fast
#    enough to expose the issue - see 099's RESULT.md):
git apply ___OPTIMIZATIONS___/092.RETAINED_LINE_BREAKING_ACROSS_RESHAPE/patch.diff
# temporarily revert perf_test/src/bench.rs's clock fix to see the unpatched-harness baseline:
patch -R -p1 -d perf_test < ___OPTIMIZATIONS___/102.BENCHPLATFORM_DETERMINISTIC_CLOCK/benchplatform_clock_fix.diff
cd perf_test && cargo build --bin perf_test --profile rdebug --features software
for r in $(seq 1 20); do
  ./../target/rdebug/perf_test --bench --mode text --count 10000 --type-text --scroll-step 0 \
      --frames 150 --snapshot /tmp/r$r.ppm >/dev/null
done
# compare all 20 .ppm pairwise for differing pixels - 16px at rows 796-797 in this session.

# 2. Re-apply the fix and confirm it's gone:
cd .. && patch -p1 -d perf_test < ___OPTIMIZATIONS___/102.BENCHPLATFORM_DETERMINISTIC_CLOCK/benchplatform_clock_fix.diff
cd perf_test && cargo build --bin perf_test --profile rdebug --features software
# repeat the same 20-run loop - 0 differing pixels.

# 3. Revert 092's product-source patch before finishing (harness fix stays applied):
cd .. && git checkout -- internal/core/item_rendering.rs internal/core/textlayout/sharedparley.rs \
  internal/core/textlayout/sharedparley/cache.rs internal/core/textlayout/sharedparley/layout.rs \
  internal/core/textlayout/sharedparley/shaping.rs internal/core/textlayout/sharedparley/tests.rs
```
