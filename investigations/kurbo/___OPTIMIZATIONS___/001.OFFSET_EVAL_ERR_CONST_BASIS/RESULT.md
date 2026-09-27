# 001. `CubicOffset::eval_err` - precompute the Bernstein basis at its fixed sample points

`offset::CubicOffset::eval_err` is the single dominant symbol in kurbo's two heaviest real-world
entry points: 62.7% of self time in the `offset` bench scene and 55.0% in `stroke` (stroke
expansion is essentially "call `offset_cubic` a lot"). It samples the candidate approximation at
`N_LSE = 8` parameter values `ta = (i + 1) * (1/9)` - values that never change from call to call -
yet re-evaluates the cubic and its derivative through the general `ParamCurve::eval` /
`ParamCurveDeriv::deriv` path every single time, recomputing the basis polynomials from scratch at
each of the 8 samples.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)` (recorded
because this candidate's evidence includes callgrind instruction counts, not only wall clock).

## What was tried

Two `const fn`-computed weight tables, following the idiom `offset.rs` already uses for
`A_WEIGHTS`/`B_WEIGHTS` (`mk_a_weights`, same file):

- `POS_WEIGHTS: [[f64; 4]; 8]` - the cubic Bernstein basis at the 8 sample points.
- `TAN_WEIGHTS: [[f64; 4]; 8]` - the *derivative's* basis, expressed over the cubic's own four
  control points with `deriv()`'s factor of 3 folded in, so the derivative curve `qa` is never
  constructed at all.

`eval_err` then hoists `c_approx`'s four control points out of the loop and computes each sample
as a four-term weighted sum. The sample parameter is written with the identical expression the
original used (`(i + 1) as f64 * (1.0 / (N_LSE + 1) as f64)`, not `(i + 1) / 9`), so the sample
*locations* are unchanged.

Command: `taskset -c 2 kurbo_bench <scene> 20`, compared with
`___OPTIMIZATIONS___/tools/paired.sh` (alternating A/B invocations, `RUNS=9`), per MEASUREMENT.md
section 2b-2c - the host's load average was drifting between ~1 and ~5 during the session.

| Variant | single-core p50 (`taskset -c 2`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline, `offset` | 12.120 ms | 12.118 ms | |
| patched, `offset` | 11.040 ms (**-8.9%**) | 11.029 ms (**-9.0%**) | paired 9/9 both modes, verdict `consistent` |
| baseline, `stroke` | 11.606 ms | 11.642 ms | |
| patched, `stroke` | 10.524 ms (**-9.3%**) | 10.490 ms (**-9.9%**) | paired 9/9 both modes, verdict `consistent` |
| baseline, `dash` | 7.326 ms | not run | single spot check |
| patched, `dash` | 7.025 ms (-4.1%) | not run | single spot check, not paired - quoted as indicative only |

Deterministic instruction counts (`valgrind --tool=callgrind`, `kurbo_bench offset 3`; these are
exact, no noise floor applies):

| | baseline | patched | delta |
|---|---|---|---|
| whole program `Ir` | 743,180,863 | 716,623,862 | **-3.57%** |
| `Ir` attributed to `eval_err` (summed across its inlined source files) | 431,123,040 | 404,568,360 | **-6.16%** |

Added later in the same session, while measuring candidate 005: this candidate's `dash`
instruction count is **462,407,847 -> 455,659,159 (-1.46%)**. Recorded here because candidate 005
(the bit-exact alternative to this one) *regresses* `dash` by +1.11%, and the comparison only means
something with both numbers - see `005.EVAL_ERR_BIT_EXACT_SCALAR_WEIGHTS/RESULT.md`. This does not
change anything above; it adds a third scene on which this candidate wins.

Memory, `offset` / `stroke` scenes: allocations per iteration unchanged (4.0 / 25.1), bytes
requested per iteration unchanged (1 800 / 1 361 928), peak RSS unchanged within noise
(3 396 -> 3 568 kB, 4 832 -> 4 836 kB). This candidate is a pure arithmetic change; it neither
helps nor hurts memory, and the numbers confirm it.

## Result

A real, decisive wall-clock win of **-8.9% to -9.9%** on the two heaviest scenes, in both
single-core and multi-core modes, with 9 of 9 paired runs going the same way in all four
comparisons (`paired.sh` verdict `consistent` each time). The measured dimension is primarily
**time**; allocation count and peak RSS were also measured and are unchanged.

Note the wall-clock win (-9%) is notably larger than the whole-program instruction-count win
(-3.6%). That gap is consistent with the change also shortening a serial dependency chain: the
original computes `mt = 1 - t`, then `mt*mt`, then `mt*mt*mt` as a chain before any control point
can be touched, whereas the constant weights let all four multiply-accumulates issue immediately.
`Ir` cannot see that; it is exactly the kind of win MEASUREMENT.md warns instruction counting
alone will under-report.

## Does it change behavior/output?

- **Bit-exact?** **No.** The `CHECK` checksum differs on the `offset`, `stroke` and `dash` scenes.
- What exactly changes: the weighted-sum form `w0*p0 + w1*p1 + w2*p2 + w3*p3` rounds differently
  from the original's nested form `p0*mt³ + (p1*3mt² + (p2*3mt + p3*t)*t)*t`. Mathematically
  identical, different floating-point rounding. Quantified by dumping every output coordinate from
  both builds at full `f64` precision (`kurbo_bench --dump offset` / `--dump stroke`, added to the
  harness for this candidate) and diffing:

  | scene | output values | differing | max absolute difference | max relative difference |
  |---|---|---|---|---|
  | `offset` (2 000 cubics, d=5, tol=1e-3) | 121 558 | 35 764 (29.4%) | 3.95e-12 (on a coordinate of 38.22) | 1.71e-13 |
  | `stroke` (100x8-cubic path, width 4, tol=1e-4) | 113 178 | 37 468 (33.1%) | 8.29e-11 (on a coordinate of 708.14) | 3.00e-12 |

  **The element counts are identical** in both scenes (121 558 vs 121 558, 113 178 vs 113 178),
  which means the subdivision decisions - how many curves the offset is split into, and where -
  were unchanged for every one of the 2 000 test cubics. The perturbation is ~8 to 12 orders of
  magnitude below the tolerances these calls were made with (1e-3 / 1e-4), so it cannot move a
  result across the tolerance check except in a case already sitting exactly on the boundary.
- **Is this a breaking change?** Not to any documented behavior or public API: no signature
  changes, no new public items, `offset_cubic`'s contract (approximate the parallel curve within
  `tolerance`) is unaffected and still met with several orders of magnitude of margin. It *is*
  observable to anyone byte-comparing rendered geometry against stored golden output.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, `grep -cE '^error'` = 0 both
  times (checked per MEASUREMENT.md section 17, since a suite that fails to compile emits no
  result line at all). No new failures, and none of kurbo's own offset regression tests (the
  "combinations of parameters that have caused problems in the past" test in `offset.rs`) are
  sensitive to the difference.
- `cargo build -p kurbo --no-default-features --features libm` (the `no_std` path): succeeds, 0
  errors.
- `cargo clippy --workspace --all-features`: 1 warning, `unneeded late initialization` in
  `kurbo/src/common.rs:389` - **pre-existing**, in a file this patch does not touch. No new
  warnings, and nothing from the workspace's strict lint set.

## Why the result looks the way it does

The 8 sample parameters are compile-time constants and always were; only the four control points
vary per call. Every call was therefore paying for basis-polynomial evaluation that could have
been a table lookup, 8 times over, up to 3 times per `offset_rec` invocation (once for the initial
arc approximation plus up to `N_REFINE = 2` least-squares refinements). Folding `deriv()`'s factor
of 3 into `TAN_WEIGHTS` removes a second cost the profile did not show separately: `qa =
c_approx.deriv()` built a whole `QuadBez` (3 vector subtractions, 3 scalar multiplications) on
every call purely to be sampled at those same fixed points.

The file already contained the exact same optimization for a neighbouring loop -
`A_WEIGHTS`/`B_WEIGHTS` are `const fn`-computed Bézier weights at these very same evenly
subdivided t values, used by `refine_least_squares` - so this is filling in a gap in a pattern the
module's own author had already established, not introducing a new technique.

## What's not done / open questions

- **A bit-exact variant was not measured.** Precomputing only the scalar sub-expressions
  (`mt³`, `3mt²`, `3mt`, `t`) while keeping the original's nested evaluation order would preserve
  the output bit-for-bit and still remove ~6 flops per sample, at a smaller speedup than the ~9%
  measured here. If the maintainers would rather not perturb output at all, that variant is the
  obvious fallback and is worth its own measurement.
- Only the `offset`, `stroke` and `dash` scenes were measured. `fit` also runs offset-shaped code
  through a different entry point and was not re-measured.
- The neighbouring backlog items in `IDEAS.md` (hoisting `rec.cusp0.signum()` out of the same
  loop; hoisting the `rec`-invariant endpoint evaluations out of `CubicOffset::apply`) were
  deliberately **not** bundled into this patch, so that this measurement is of one change only.
  They stack with this one and remain untested.
- No golden/snapshot files exist in this project to regenerate.
