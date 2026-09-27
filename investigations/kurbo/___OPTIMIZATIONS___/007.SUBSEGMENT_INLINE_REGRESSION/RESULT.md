# 007. `#[inline]` on `subsegment` - the same change as 006, one function further, and it loses

Candidate 006 got -15% single-core / -27% multi-core on `flatten` from four plain `#[inline]`
attributes on small cross-module functions. `CubicBez::subsegment` is the *other* function the
`flatten` profile showed as a standalone symbol (23.9% of the scene) with no `#[inline]`, reached
through `ToQuads::next` from `bezpath::flatten` in another module - the same shape, same scene.

This candidate exists to check whether 006's result generalizes to it. **It does not.** Recording
that is the point: the obvious reading of 006 is "kurbo is missing `#[inline]`, add it to the hot
cross-module functions", and applied one function further that reading makes things slower.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

`#[inline]` on `<CubicBez as ParamCurve>::subsegment` (plus an explanatory comment) and on
`<QuadBez as ParamCurve>::subsegment`. Four added lines, no logic change. Measured against the
pristine baseline, exactly like every other candidate here - **not** stacked on top of 006 or 002.

### Deterministic instruction counts (`valgrind --tool=callgrind`, `<scene> 5`)

| scene | baseline | **007** | candidate 006, for comparison |
|---|---|---|---|
| `flatten` | 34,685,305 | 35,171,571 (**+1.40%**) | 32,845,315 (-5.31%) |
| `flatten-fine` | 247,056,104 | 249,865,821 (**+1.14%**) | 237,218,119 (-3.98%) |

### Wall clock (`paired.sh`, `RUNS=15`)

| scene | single-core (`taskset -c 2`) | multi-core (unpinned) |
|---|---|---|
| `flatten` | 0.675 -> 0.709 ms (**+5.0%**), B faster in only **2 of 15** pairs | 0.681 -> 0.708 ms (**+3.9%**), B faster in only **1 of 15** - `consistent` |
| `flatten-fine` | 4.453 -> 4.457 ms (+0.1%), 6/15 - `NOT consistent` | 4.435 -> 4.446 ms (+0.3%), 5/15 - `NOT consistent` |

The `flatten` single-core row reads `NOT consistent` only because `paired.sh`'s threshold is
symmetric (it needs `wins <= 1` or `>= 14`); at 2 of 15 wins it is one pair short of being called a
consistent *regression*, and the multi-core row at 1 of 15 clears it outright. Both agree with the
instruction count. `flatten-fine` is genuinely neutral - 5-6 of 15 is noise, and the +1.14% `Ir`
there does not surface in wall clock.

Reproduced from a fresh clone at the pinned commit by this folder's own `just test`, more
decisively than the in-tree run: `flatten` 0.674 -> 0.707 ms (**+5.0%, B faster in 0 of 15
pairs**, `consistent`) and `flatten-fine` 4.415 -> 4.471 ms (**+1.3%, 1 of 15**, `consistent`).
Note this makes `flatten-fine` a small regression too, where the in-tree run had read it as
neutral - the fresh-clone measurement agrees with the +1.14% instruction count, so treat
`flatten-fine` as mildly regressed rather than unaffected.

Binary size: 8,112,680 (baseline) -> 8,092,992 (007) -> 8,090,552 (006). All three are close;
007 does not bloat the binary, so size is not the mechanism.

## Result

A **small regression**: +1.1% to +1.4% instruction count on both flatten scenes, and +3.9% to +5.0%
wall clock on `flatten` (1-2 of 15 paired wins, i.e. the regression is what reproduces, not the
noise). `flatten-fine` is unaffected in wall clock. Dimension measured: time and instruction count.
Bit-exact, so nothing else changed.

## Does it change behavior/output?

- **Bit-exact?** **Yes.** `CHECK` compared across all 25 harness scenes: 25 identical, 0 differing.
  `#[inline]` cannot change floating-point results in Rust.
- **Is this a breaking change?** No - two attributes on private trait-impl methods.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, 0 lines matching `^error`.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors.
- `cargo clippy --workspace --all-features`: only the pre-existing `unneeded late initialization`
  warning in `common.rs`. No new warnings.

## Why the result looks the way it does

The difference between this and candidate 006 is **the size of the function being inlined**, which
is precisely the distinction HEURISTICS.md draws and this pair of candidates now demonstrates
empirically on the same code path:

- 006's functions are tiny. `approx_parabola_integral` is a single expression;
  `estimate_subdiv`'s cost is largely in *returning a 5-field `FlattenParams` struct through
  memory*, which inlining deletes outright.
- `subsegment` is comparatively large: two cubic evaluations, a derivative construction, and two
  quadratic evaluations. Inlining it into `ToQuads::next` - which already carries `#[inline]` and is
  itself inlined into `flatten`'s inner loop - pushes a large body into an already-hot loop,
  increasing register pressure without deleting an expensive struct return (its `CubicBez` return
  value is consumed immediately by the caller either way).

So the transferable rule is not "kurbo is missing `#[inline]`" but "kurbo is missing `#[inline]` on
*small* cross-module functions, particularly ones returning multi-field structs". Being a
standalone symbol in the profile makes a function a *candidate* for the hint, not a beneficiary of
it.

Note also that candidate 002 attacks the same function from the other side - it removes half of
`subsegment`'s work from `ToQuads::next` by not calling it at all - and 002 is a win (-4 to -6.5%).
The right treatment for `subsegment` on this path is fewer calls, not cheaper ones.

## What's not done / open questions

- `#[inline(always)]` was not tried and should not be, on this evidence.
- Only the two `subsegment` implementations were tested. Whether `#[inline]` on `subsegment`
  helps some *other* caller (it is public API, and other code paths call it outside `flatten`) was
  not measured - this candidate is scoped to the flatten path where the profile pointed.
- The combination "006 + 007" was not measured; 007 was measured against the pristine baseline like
  every other candidate. It is possible though unlikely that the two interact differently together
  than separately.
- `flatten-fine`'s +1.14% instruction count not showing up in wall clock is unexplained; the likely
  reading is that at a fine tolerance the inner subdivision loop (`determine_subdiv_t` plus point
  evaluation) dominates and `subsegment`'s share is too small to matter either way.
