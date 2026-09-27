# 004. `CubicOffset::draw_arc` - replace `atan2` + `sin` + `cos` with half-angle identities

`draw_arc` produces the initial arc approximation for every subdivision of every offset curve, and
it is where stroking and offsetting spend their libm time (`__ieee754_atan2` 2.6-2.8% and
`__sincos` 1.5% of self time in the `stroke`/`offset` scenes). It computed:

```rust
let th = rec.utan1.cross(rec.utan0).atan2(rec.utan1.dot(rec.utan0));
let a = (2. / 3.) / (1.0 + (0.5 * th).cos()) * 2.0 * (0.5 * th).sin();
```

The angle `th` is never used for anything except `cos(th/2)` and `sin(th/2)`, so the arctangent
and the trigonometry that undoes it cancel. The source already said so:
`// possible optimization: this can probably be done with vectors rather than arctangent`.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

Both tangents are unit length, so their cross product *is* `sin(th)` and their dot product *is*
`cos(th)` - no angle needs to be recovered. The half-angle identities then give the two values
directly:

```rust
let sin_th = rec.utan1.cross(rec.utan0);
let cos_th = rec.utan1.dot(rec.utan0);
let half_cos = (0.5 * (1.0 + cos_th)).max(0.0).sqrt();
let half_sin = (0.5 * (1.0 - cos_th)).max(0.0).sqrt().copysign(sin_th);
let a = (2. / 3.) / (1.0 + half_cos) * 2.0 * half_sin;
```

`th` lies in `(-pi, pi]`, so `th/2` lies in `(-pi/2, pi/2]` and `cos(th/2)` is never negative -
which is what makes the positive square root the correct branch. `sin(th/2)` takes its sign from
`sin(th)`, recovered with `copysign` so that a negative zero behaves as it did before. The
`.max(0.0)` clamps guard against rounding leaving `cos_th` a hair outside `[-1, 1]` even for
unit-length inputs.

Three libm calls (`atan2`, `sin`, `cos`) become two square roots.

### Deterministic instruction counts (`valgrind --tool=callgrind`, `kurbo_bench <scene> 3`)

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `offset` | 743,181,009 | 700,821,039 | **-5.70%** |
| `stroke` | 742,515,414 | 701,266,754 | **-5.56%** |
| `dash` | 462,408,075 | 447,846,369 | **-3.15%** |

### Wall clock (`paired.sh`, `RUNS=9`)

| Variant | single-core p50 (`taskset -c 2`) | multi-core p50 (unpinned) |
|---|---|---|
| baseline, `offset` (20 iters) | 12.248 ms | 12.178 ms |
| patched, `offset` | 11.358 ms (**-7.3%**), paired 8/9 `consistent` | 11.262 ms (**-7.5%**), paired 9/9 `consistent` |
| baseline, `stroke` (20 iters) | 11.617 ms | 11.631 ms |
| patched, `stroke` | 10.857 ms (**-6.5%**), paired 9/9 `consistent` | 10.889 ms (**-6.4%**), paired 9/9 `consistent` |
| baseline, `dash` (20 iters) | 7.386 ms | 7.604 ms |
| patched, `dash` | 7.111 ms (**-3.7%**), paired 8/9 `consistent` | 7.294 ms (-4.1%), paired 7/9 - `NOT consistent` |

The one `NOT consistent` row (`dash` multi-core) is backed by the deterministic -3.15% instruction
count, so the direction is not in doubt there either; the host was at load average ~15 during this
window and the wall-clock threshold simply wasn't cleared.

Unlike candidate 003, **every scene improves here** - including `dash`, which 003 regressed.

Memory: unchanged. Allocation counts and bytes per iteration are identical; this patch allocates
nothing and adds no state.

## Result

A real win of **-6.4% to -7.5% wall clock** on `offset` and `stroke` and **-3.7%** on `dash`, with
8-9 of 9 paired runs agreeing in five of the six comparisons, corroborated by deterministic
instruction counts of **-5.70% / -5.56% / -3.15%** on the three scenes. The dimension measured is
time and instruction count; allocations and peak RSS were checked and are unchanged.

A secondary, unmeasured benefit worth noting for a `no_std` reader: this removes three `libm`
calls from the hot path entirely. Under the `libm` feature those go through the `libm` crate
rather than the platform's, so the saving is likely larger there than the numbers above (which
were taken with the default `std` feature) - not measured, so not claimed.

## Does it change behavior/output?

- **Bit-exact?** **No.** `CHECK` differs on exactly 4 of the 25 harness scenes - `offset`,
  `stroke`, `stroke-miter`, `dash` - and is identical on the other 21, confirming the change is
  scoped to the offset/stroke path and touches nothing else.
- **What changes, quantified.** Two different pictures, and they need separating:

  | scene | output values | structure | differing | max absolute difference | max relative |
  |---|---|---|---|---|---|
  | `offset` (2 000 independent cubics, d=5, tol=1e-3) | 121 558 vs 121 558 | **identical** | 14 083 (11.6%) | **3.13e-12** (on a coordinate of 38.22) | 1.93e-13 |
  | `stroke` (100x8-cubic path, width 4, tol=1e-4) | 113 178 vs 113 088 | **differs** | n/a | n/a | n/a |

  For `offset` the path structure is identical on all 2 000 test curves and the coordinates move
  by at most 3.13e-12 - 9 orders of magnitude below the 1e-3 tolerance requested.

  For `stroke` the patched build emits **90 fewer coordinate values, i.e. about 15 fewer curve
  elements out of roughly 18 860 (~0.08%)**. A coordinate-by-coordinate diff of the two dumps is
  therefore **misaligned past the first structural divergence and its numbers are meaningless** -
  they are deliberately not quoted here. What happened is that the last-ulp change in `a` moved
  `err_squared` across the `tolerance²` check for a handful of segments, so those converged
  without one further subdivision. Both builds only ever emit a segment once its measured error is
  within tolerance (or `MAX_DEPTH` is hit), so both outputs satisfy `offset_cubic`'s documented
  contract; they are each within `tolerance` of the same true offset curve, hence within
  `2 * tolerance` of each other. Emitting slightly fewer segments for the same guarantee is not a
  quality loss, but it **is** a visible structural change to anyone byte-comparing stored path
  output.
- **Is this a breaking change?** No public API or documented behavior changes; `draw_arc` is
  private. It is observable to golden-file comparisons of stroked geometry, and - unlike the
  `offset` case - not merely as a last-ulp perturbation but as a different number of path
  elements. That is the thing a maintainer needs to weigh.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, `grep -cE '^error'` = 0 both
  times. This includes `offset.rs`'s own "combinations of parameters that have caused problems in
  the past" regression test.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors. (`copysign`,
  `max` and `sqrt` are all already available on this path; `offset.rs` uses `copysign` and `sqrt`
  elsewhere already.)
- `cargo clippy --workspace --all-features`: only the pre-existing `unneeded late initialization`
  warning in `kurbo/src/common.rs`. No new warnings.

## Why the result looks the way it does

`atan2` is one of the more expensive libm functions (argument reduction plus a polynomial), and
`sin`/`cos` of the resulting half-angle add a second and third such call - all to recover
information that was already sitting in the inputs. The tangents are unit vectors by construction
(`offset_cubic` normalizes them, and `subdivide` passes normalized ones down), so `cross` and
`dot` are exactly `sin(th)` and `cos(th)` with no extra work. Two `sqrt` instructions (~15-20
cycles each, and pipelined) replace roughly 100+ cycles of libm.

The effect is proportionally smaller on `dash` (-3.15% vs -5.70%) because dashing spends a larger
share of its time in the dash iterator and the stroker's own bookkeeping, so `draw_arc` is a
smaller slice of the whole.

## What's not done / open questions

- **The `stroke` structural difference is characterized qualitatively, not with a geometric
  distance.** A proper quantification would flatten both stroke outputs to fine polylines and
  measure the maximum deviation between them; that was not built in this session. The argument
  given above (both outputs satisfy the same tolerance contract, so they are within `2 *
  tolerance` of each other) is sound but is a bound, not a measurement.
- The same `atan2(cross, dot)` pattern appears at three other sites that were **not** touched, one
  fix per candidate: `offset.rs:501`, `stroke.rs:492` (`let angle = cross.atan2(dot)`) and
  `expand.rs:324` (`let angle = crossn.atan2(dotn).abs()`). Whether each is followed by trig that
  cancels the same way needs checking individually - added to `IDEAS.md`.
- `fit.rs:401-407` uses three `atan2` calls per `fit_to_cubic` and is 17.6% of the `fit` scene -
  by far the largest remaining `atan2` cost - but those angles feed `mod_2pi` and the fitting math
  itself rather than cancelling immediately, so removing them is an algorithmic change, not a
  local rewrite. Left in `IDEAS.md` as a separate, larger item.
- The `libm`-feature build was checked for compilation but its performance was not measured, even
  though the benefit is probably larger there.
- Stacks with candidates 001 and 002; the combination was not measured together.
