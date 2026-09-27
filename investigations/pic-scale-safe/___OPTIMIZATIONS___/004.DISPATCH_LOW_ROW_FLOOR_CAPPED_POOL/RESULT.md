# Hybrid: low row-count floor plus the per-filter capped thread pool

**Ported from `image` repo's `___OPTIMIZATIONS___/036.VENDORED_DISPATCH_LOW_ROW_FLOOR/`, with
candidate `038.VENDORED_DISPATCH_LOW_ROW_FLOOR_SMALL_BENCH_SCENARIO`'s additional below-floor
measurement folded in (038 added no new source change - it only added a smaller benchmark
scenario and re-measured 036's already-fixed patch).** Originally measured in 2026-08 via a
vendored `pic-scale-safe` 0.1.11 copy inside the `image` project. **Not re-measured in this
workspace** - numbers below are copied verbatim. The patch was rebuilt by hand against this repo's
current `HEAD` (`b50e020`), including the `resampling_function` parameter threading (see candidate
003's own note). `git apply --check` passes, `cargo check --features rayon` compiles, `cargo test
--release --features rayon` passes 5/5. Includes candidate 001's `resize_nearest.rs` gate.

Candidate 003 (always-dispatch per-filter capped pool) fixed candidate 002's multi-core
regression, but introduced a small, consistent single-core regression (+4.9% to +13.6%) at this
line's `400x300->32x24` scenario, since every call now pays `pool.install()`'s fixed dispatch cost
even at only 24 output rows. This candidate adds `const MIN_ROWS_FOR_POOL: usize = 8` to each of
the 6 dispatch call sites: below it, take the same serial loop as the `rayon`-off build; at/above
it, use candidate 003's per-filter capped pool unchanged.

## What was tried

Same vendoring/dispatch structure as candidate 003, plus a per-function `if destination_size.height
>= 8 { pool_for_filter(...).install(...) } else { <serial loop> }` gate. Measured at two distinct
destination sizes:

**At `400x300->32x24` (`dst_height=24 >= 8`, above the floor)** - identical to candidate 003 on
both axes for all four filters (every `E -> H` comparison landed inside noise, `-2.3%` to `+0.5%`);
`G -> H` (vs. candidate 002's flat gate) reproduces 003's own pattern near-exactly: multi-core
`-35.9%` to `-41.0%` (5/5), single-core `+4.4%` to `+12.4%` regression (0-1/5) - **the single-core
cost candidate 003 introduced is still present, unchanged, at this scenario** (structurally
expected: `24 >= 8`, so the floor never activates here).

**At `400x300->8x6` (`dst_height=6 < 8`, below the floor, added by the folded-in candidate 038)** -
the floor's mechanism does what it was designed to do:

| Filter | single-core (E->H) | multi-core (E->H) |
|---|---|---|
| Triangle | **-14.6%** (4/5) | **-18.8%** (5/5) |
| CatmullRom | -8.9% (3/5, outside noise but not decisive by pair count) | **-15.2%** (5/5) |
| Gaussian | -9.2% (3/5) | **-20.7%** (5/5) |
| Lanczos3 | **-6.5%** (5/5) | **-13.0%** (5/5) |
| Nearest | no meaningful difference (routes through candidate 001's separate gate, untouched by this dispatch-level floor) | same |

## Result

**Small, real win overall, not a regression anywhere measured.** At the workspace's original
`dst_height=24` scenario, this candidate is statistically indistinguishable from candidate 003
(the floor doesn't activate there) - so it inherits 003's multi-core win over ungated/flat-gated
dispatch, at the cost of 003's own small single-core regression, unchanged. At a genuinely small
output (`dst_height=6`, below the floor), the floor is a decisive-to-mostly-decisive win over
candidate 003's always-dispatch pool on **both** axes for every filter that reaches the gated
dispatch functions - skipping `pool.install()` at a size with no real parallel work to amortize
against helps more on multi-core here, not less, since there's nothing to parallelize productively
at 48 total output pixels.

## Does it change behavior/output?

- **Bit-exact?** Yes, per the original measurement: 40 hashes across 5 filters x 4 scenarios
  (including `dst_height=6`, specifically exercising the new below-floor serial branch, and
  `dst_height=24`/above, exercising the unchanged pool branch) - matched exactly. The folded-in
  correctness check (038) re-confirmed the same at `dst_height=6` across 3 builds (serial-only,
  no-floor, and this floor) with 20 additional hashes, also matched exactly.
- **Is this a breaking change?** No public API change.
- Project test suite, re-verified this session against current `HEAD`: `cargo test --release
  --features rayon` - **5 passed, 0 failed**. `cargo check --features rayon` compiles clean.

## Why the result looks the way it does

There is no floor value that helps at the fixed `dst_height=24` scenario without also giving up
the parallelism that scenario already has enough real work to use productively (candidate 002's
own finding) - a floor `<= 24` (as chosen here, `8`) cannot change behavior at `dst_height=24` at
all, so this candidate inherits candidate 003's exact result there. The floor only pays off for
outputs genuinely smaller than whatever scenario a reader measures against - which is exactly what
the `dst_height=6` data shows.

## What's not done / open questions

- The `MIN_ROWS_FOR_POOL = 8` value was not swept - the originating workspace's own analysis
  showed any value `<= 24` behaves identically to candidate 003 at the established scenario, so
  sweeping within a small range wouldn't change the qualitative conclusion; only a genuinely
  larger sweep against various small `dst_height` values would characterize the floor's own
  optimum.
- Only one below-floor size (`dst_height=6`) was timed; a sweep of destination heights between 1
  and 8 was not done.
- Not independently re-benchmarked against current `HEAD` in this session - only correctness and
  build/test were re-verified; timing numbers are historical and directional only.
- This is the most-refined candidate in this workspace's 001-004 line (folds in 001's `Nearest`
  fix, 002's discovery, 003's regression fix, and its own below-floor validation) - a reader
  wanting a single fix to consider from this line would likely start here rather than with 001-003
  individually, though all four remain separately documented per this skill's "one fix per
  candidate" convention (003 and 004 were tested as genuinely different code, not just different
  parameters of the same patch).
