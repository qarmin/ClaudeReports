# FIT_TO_BEZPATH_OUTPUT_ALLOC

`fit_to_bezpath` returns a fresh `BezPath` per call and lets it grow by doubling, which makes the
`fit` scene the highest allocation-*count* scene of the whole harness: 1173 allocations / 318 536
bytes per iteration over 500 `fit_to_bezpath` calls (2.346 allocations per call). The candidate
adds a `fit_to_bezpath_into(source, accuracy, &mut BezPath)` variant - mirroring `offset_cubic`'s
existing `&mut BezPath` convention - so a caller fitting many curves in a loop reuses one buffer,
and switches kurbo's own only in-crate caller (`simplify::SimplifyState::flush`) over to it.

## Attribution first (the backlog's guess was wrong)

The backlog guessed "a `Vec` per subdivision attempt". It is not that:

- the `fit` scene calls **only** `fit_to_bezpath` (harness `main.rs`, scene `"fit"`); the bare
  `Vec::new()` at `fit.rs:565` is in `fit_to_bezpath_opt`, which only the separate `fit-opt` scene
  reaches;
- `CurveDist::from_curve` uses `ArrayVec<_, 20>` and does not heap-allocate;
- `CurveFitSample::intersect` collects into `ArrayVec<f64, 3>`, also not heap.

Proven by construction, not by reading: with the harness's `fit` scene switched to
`fit_to_bezpath_into` + one reused output path, `allocs/iter` goes **1173.0 -> 0.0** and
`bytes/iter` **318 536 -> 8**, with an identical `CHECK`. Every single allocation in the `fit`
scene was the output `BezPath`'s growth; nothing else in `fit.rs` allocates at all on this path.

The per-call growth pattern also matches exactly. `size_of::<PathEl>() == 56`, so `Vec`'s first
allocation is capacity 4, then 8, then 16. Solving the observed 2.346 allocations/call and 637
bytes/call against that ladder gives ~15% of calls emitting <=4 elements, ~35% emitting 5-8 and
~50% emitting 9-16 - and the `with_capacity(16)` variant below lands at 1.022 allocations/call,
i.e. ~98% of calls do fit in 16 elements, confirming the distribution.

## What was tried

`rustc 1.98.0` / `cargo 1.98.0`. Harness: `___OPTIMIZATIONS___/harness/kurbo_bench`, built in an
isolated git worktree of `ca27349` whose `kurbo` dependency points at that worktree (verified:
`Compiling kurbo` present in the build, and `sha256 bin_baseline != sha256 bin_patched`).

### `fit` scene (500 `fit_to_bezpath` calls per iteration)

| Variant | single-core p50 | multi-core p50 | allocs/iter | bytes/iter | peak RSS |
|---|---|---|---|---|---|
| baseline (`BezPath::new()`) | 15.507 ms | 15.640 ms | 1173.0 | 318 536 | 3.37-3.47 MB |
| patch, library only (harness still calls `fit_to_bezpath`) | 15.573 ms (+0.4%, 5/9 - NOT consistent) | 15.691 ms (+0.3%, 3/9 - NOT consistent) | 1173.0 | 318 536 | unchanged |
| patch + caller reuses one `BezPath` via `fit_to_bezpath_into` | 15.471 ms (-1.2%, 8/9 - consistent) | 15.682 ms (-0.1%, 6/9 - NOT consistent) | **0.0** | **8** | 3.37-3.48 MB |
| side-experiment: `BezPath::with_capacity(8)`, no API change | 15.664 ms | not run | 727.0 (-38%) | 330 632 (+3.8%) | unchanged |
| side-experiment: `BezPath::with_capacity(16)`, no API change | 15.463 ms | not run | 511.0 (-56%) | 457 864 (+43.7%) | unchanged |

### `simplify` scene (kurbo's own in-crate `fit_to_bezpath` caller, harness unmodified)

| Variant | single-core p50 | multi-core p50 | allocs/iter | bytes/iter | peak RSS |
|---|---|---|---|---|---|
| baseline | 0.2200 ms | 0.2196 ms | 39.0 | 769 312 | 4.23-4.38 MB |
| patch (`SimplifyState` reuses one fit buffer) | 0.2163 ms (**-1.5%, 9/9 - consistent**) | 0.2160 ms (**-1.9%, 9/9 - consistent**) | **25.0 (-35.9%)** | 766 176 (-0.4%) | 4.29-4.44 MB |

Wall clock via `___OPTIMIZATIONS___/tools/paired.sh`, `RUNS=9`, alternating runs, `TASKSET=2` for
single-core and unpinned for multi-core. Baseline noise floor (two repeat runs each, same
binary): `fit` 15.571/15.596 ms single-core and 15.551/15.606 ms multi-core; `simplify`
0.2213/0.2217 ms single-core and 0.2190/0.2254 ms multi-core. Load average 3.2-3.7 throughout
(5 sibling candidates running concurrently under a shared `flock`).

### Reproduction re-run from a fresh clone (`just test`), under much heavier load

`just test` was run end-to-end afterwards (fresh `git clone` at `base_commit`, patch applied,
three binaries built) while the host load average had climbed from ~3.2 to ~17 (sibling
candidates). The **allocation counts reproduced exactly** - `simplify` 39.0 -> 25.0, `fit`
1173.0 -> ~0 (`0.1` as printed, i.e. a single allocation for the whole 20-iteration run) - and
both `CHECK`s were again identical. Every wall-clock comparison, however, came back
`NOT consistent` at that load, and the untouched baseline itself drifted from 15.5 ms to 18.0 ms
on `fit` and 0.220 ms to 0.275 ms on `simplify`. A `RUNS=15` re-check of `simplify` at load ~17
gave -2.1% / 12 of 15 single-core and -1.4% / 11 of 15 multi-core: same direction and magnitude
as the quiet-host 9/9 runs, but below `paired.sh`'s threshold. This is exactly why the headline of
this candidate is the allocation count (deterministic, hardware- and load-independent) and the
callgrind `Ir` delta, not the timing.

### Callgrind (deterministic, `valgrind --tool=callgrind`, whole-process `Ir`)

| comparison | baseline Ir | patched Ir | delta |
|---|---|---|---|
| `fit 5`, library-only patch | 1 140 067 746 | 1 140 092 131 | +0.002% |
| `fit 5`, patch + reusing caller | 1 140 067 746 | 1 137 814 775 | **-0.198%** |
| `simplify 100` | 336 434 138 | 332 076 355 | **-1.295%** |

## Result

Dimensions measured: **allocation count, allocated bytes, peak RSS, and wall clock.**

- **Allocation count is where the effect is.** For a loop caller that adopts
  `fit_to_bezpath_into`, `fit`'s allocations per iteration go from 1173 to **0** and its requested
  bytes from 318 536 to 8 - i.e. all of that scene's allocation traffic was the returned
  `BezPath`, and it is fully removable. For kurbo's own in-crate caller (`simplify_bezpath`,
  Subdivide opt level) with no user change at all, `simplify`'s allocations go 39 -> 25
  (**-35.9%**). These counts are deterministic and repeated identically across runs.
- **Peak RSS: no measurable change** in either scene. The removed allocations are small and
  short-lived (each fitted path is freed at the end of its own iteration), so `VmHWM` stays inside
  its own run-to-run noise band (`fit` 3.37-3.48 MB in both builds, `simplify` 4.23-4.44 MB).
  Allocation *count*, not footprint, is the honest headline here.
- **Wall clock, `simplify`: a real small win** - -1.5% single-core and -1.9% multi-core, 9/9
  paired both times on a quiet host (load ~3.2), against a ~0.2-3% noise floor, and corroborated
  by a -1.30% callgrind `Ir` delta which cannot be noise. A later re-check at load ~17 kept the
  same direction and magnitude (-2.1% 12/15, -1.4% 11/15) but no longer crossed `paired.sh`'s
  consistency threshold; the `Ir` delta is what makes this one solid.
- **Wall clock, `fit`: marginal at best.** -1.2% single-core (8/9, consistent) but NOT consistent
  multi-core (-0.1%, 6/9), against a -0.198% `Ir` delta. Leaning on `Ir` and the allocation count
  rather than the -1.2%: the single-core figure is only ~5x the run-to-run noise floor while five
  sibling candidates were building, and the multi-core run does not reproduce it. The scene is
  dominated by libm work in `fit_to_cubic` (see candidate 016), so ~0.2% of instructions is the
  right order of magnitude for removing 2.3 malloc/realloc calls out of a ~326 k-instruction
  per-call budget (1 140 067 746 `Ir` / 3500 calls: 5 timed + 2 warmup iterations x 500 sources).
- The library-only patch measured against an **unchanged** caller is correctly a **no-op** on
  `fit` (+0.002% `Ir`, both wall-clock comparisons NOT consistent) - the new API only pays off
  once a caller uses it. That row is here as the control, not as a result.
- The two `with_capacity(K)` side-experiments are **mixed**, not wins: they cut allocation count
  without an API change (-38% at K=8, -56% at K=16) but increase requested bytes (+3.8% and
  +43.7%), because a fixed reservation over-allocates for the ~15% of calls that emit <=4
  elements. Neither shows a wall-clock effect. `fit_to_bezpath_into` strictly dominates both.

## Does it change behavior/output?

- **Bit-exact?** **yes**, in every configuration measured. `CHECK` is identical for `fit`
  (`0x2a09907696ae11d0`) across baseline, library-only patch, reusing-caller patch and both
  `with_capacity` variants, and identical for `simplify` (`0x0b8da460d331dc7e`) across baseline and
  patch. This is expected and not luck: `fit_to_bezpath_into` starts with `result.truncate(0)`
  (mirroring `offset_cubic`), so `fit_to_bezpath_rec`'s `path.is_empty()` test that decides whether
  to emit the leading `move_to` sees exactly the same state as before.
- **Is this a breaking change?** No. `fit_to_bezpath`'s signature, semantics and output are
  untouched. `fit_to_bezpath_into` is a **new** public item (re-exported from `lib.rs` alongside
  `fit_to_bezpath`), which is additive; `simplify.rs`'s change is entirely internal to the private
  `SimplifyState`.
- This folder **has** a `patch.diff` - it is a change to kurbo's own source (`kurbo/src/fit.rs`,
  `kurbo/src/lib.rs`, `kurbo/src/simplify.rs`), +34/-9 lines, verified with `git apply --check`
  against `ca27349`. The harness edit used to produce the "reusing caller" row is a *harness*
  change (one scene body switched to the new API) and is deliberately **not** in `patch.diff`; it
  is reproduced by the `bench-reuse` recipe in this folder's `justfile` instead.
- Project test suite, `cargo test --workspace --all-features`:
  **before = 252 passed / 0 failed** (187 + 10 + 55 + 0 across 4 `test result` lines),
  **after = 252 passed / 0 failed** (187 + 10 + 55 + 0, same 4 lines). No new failures, no
  `^error` lines in either run.
- `cargo build -p kurbo --no-default-features --features libm` (the `no_std` path) builds clean.
  `cargo clippy --workspace --all-features --all-targets` emits exactly one warning, the
  pre-existing `needless_late_init` at `kurbo/src/common.rs:389`, untouched by this patch.

## Why the result looks the way it does

`fit_to_bezpath`'s cost is overwhelmingly floating-point: `fit_to_cubic` does three `atan2`s plus
a quartic solve per attempt, ~326 k instructions per `fit_to_bezpath` call in this scene. Two or three
`malloc`/`realloc` calls are a few hundred instructions against that, so removing them cannot move
wall clock much - which is exactly what the -0.198% `Ir` says. The allocation *count* is still the
largest of any scene in the harness, and it is 100% removable, which matters to an allocator-bound
or fragmentation-sensitive embedder even though it barely shows in a timing.

`simplify` moves more (-1.3% `Ir`, -1.5/-1.9% wall) for a different reason: its per-flush fitted
paths are much larger than a single offset segment's, so each flush was paying several reallocs
with real `memcpy` behind them, and there are only a handful of flushes per iteration to amortize
over (14 of the scene's 39 allocations per iteration disappear, which bounds the flush count and
the per-flush realloc ladder). Reusing one buffer means the second and later flushes hit an
already-grown allocation and never realloc at all. Its `bytes/iter` barely drops (-0.4%) because the dominant byte consumer in that
scene is `SimplifyBezPath::new`'s `Vec<SimplifyCubic>` (a flattened path's worth of cubics), which
this patch does not touch.

`with_capacity(K)` cannot win outright because the output length is genuinely unknown before the
fit runs (it depends on how deep the recursion goes), so any fixed `K` trades bytes for count.
The measured element-count distribution (~15% <=4, ~35% 5-8, ~50% 9-16) leaves no `K` that is
better on both axes.

## What's not done / open questions

- **No kurbo-internal stroker caller was converted, because there isn't one.** `stroke.rs`,
  `expand.rs` and `offset.rs` never call `fit_to_bezpath` at `ca27349` (grep confirms) - kurbo's
  stroke expansion goes through `offset_cubic`, which already writes into a `&mut BezPath`. The
  "stroker calls this once per offset segment" framing in the backlog describes *downstream*
  stroking built on `fit_to_bezpath` (e.g. a client's own `ParamCurveFit`), not kurbo's own. So the
  `fit`-scene 1173 -> 0 result is what a **downstream** caller gains by adopting the new API; the
  only in-crate beneficiary is `simplify_bezpath`.
- `SimplifyOptLevel::Optimize` still allocates a fresh path per flush, since `fit_to_bezpath_opt`
  has no `_into` twin. Adding one is a separate, larger change (its inner loop appends and
  truncates the path at several points) and was not attempted here.
- Whether `fit_to_bezpath_into` is the API shape kurbo's maintainers want is a design question,
  not a measurement one - `offset_cubic`'s `(.., result: &mut BezPath)` + `truncate(0)` convention
  was copied deliberately, but an `append`-rather-than-replace variant, or a
  `ParamCurveFit::fit_into` method (the doc comment on `fit_to_bezpath` floats making it a trait
  method), are equally defensible.
- The `fit-opt`, `stroke`, `offset` and `dash` scenes were not re-measured: none of them reach the
  changed code (`fit_to_bezpath` has no caller in `offset.rs`/`stroke.rs`/`expand.rs`, and
  `fit-opt` goes through the untouched `fit_to_bezpath_opt`).
