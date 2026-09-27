# 005. `eval_err` - a bit-exact variant of candidate 001, and why it is the worse option

Candidate 001 precomputes the Bernstein basis at `eval_err`'s eight fixed sample points and gets
~9% on `offset`/`stroke`, but it perturbs output (up to 3.95e-12 on a coordinate) because a
four-term weighted sum rounds differently from `ParamCurve::eval`'s nested form. This candidate
exists to answer the obvious follow-up question a maintainer would ask: **what does the safe,
bit-exact version cost, and is it good enough to prefer?**

The answer turns out to be no, on every axis - which is worth knowing precisely, because "just do
the bit-exact one instead" is the intuitive response to 001 and it is wrong here.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

Instead of collapsing the evaluation into a weighted sum, precompute only the *scalar
sub-expressions* that `CubicBez::eval` and `QuadBez::eval` build from `t`, and combine them with
the control points in exactly the original order:

```rust
struct EvalWeights { t: f64, c0: f64, c1: f64, c2: f64, q0: f64, q1: f64 }
// c0 = mt^3, c1 = 3mt^2, c2 = 3mt   (as CubicBez::eval uses them)
// q0 = mt^2, q1 = 2mt              (as QuadBez::eval uses them)
```

```rust
let pa = (c_approx.p0.to_vec2() * w.c0
    + (c_approx.p1.to_vec2() * w.c1
        + (c_approx.p2.to_vec2() * w.c2 + c_approx.p3.to_vec2() * w.t) * w.t) * w.t).to_point();
let tana = qa.p0.to_vec2() * w.q0 + (qa.p1.to_vec2() * w.q1 + qa.p2.to_vec2() * w.t) * w.t;
```

This removes only the ~8 flops per sample that computed `mt`, `mt*mt`, `mt*mt*mt`, `3mt^2`, `3mt`,
`2mt`. Unlike 001, the derivative curve `qa = c_approx.deriv()` still has to be built, because the
weights are applied to *its* control points rather than folded onto the cubic's.

### Deterministic instruction counts (`valgrind --tool=callgrind`, `kurbo_bench <scene> 3`)

| scene | baseline | **005 (bit-exact)** | **001 (not bit-exact)** |
|---|---|---|---|
| `offset` | 743,181,009 | 724,826,832 (**-2.47%**) | 716,623,862 (**-3.57%**) |
| `stroke` | 742,515,414 | 724,336,976 (**-2.45%**) | not measured |
| `dash` | 462,407,847 | 467,558,145 (**+1.11%**) | 455,659,159 (**-1.46%**) |

### Wall clock (`paired.sh`, `RUNS=9`)

| scene | single-core (`taskset -c 2`) | multi-core (unpinned) |
|---|---|---|
| `offset` | 14.284 -> 13.446 ms (-5.9%), paired 8/9 `consistent` | 14.745 -> 12.918 ms (-12.4%), paired 9/9 `consistent` |
| `stroke` | 11.605 -> 10.503 ms (-9.5%), paired 7/9 - `NOT consistent` | 14.339 -> 12.952 ms (-9.7%), paired 8/9 `consistent` |

**These wall-clock numbers should not be used.** The baselines within this single batch ranged from
11.6 to 14.7 ms for the same untouched binary, and the implied deltas (-5.9% to -12.4%) are far
wider than any plausible effect from removing ~8 flops per sample - larger, in fact, than candidate
001's win from removing strictly more work. The host was heavily loaded during this window. The
instruction counts above are the evidence for this candidate; the wall-clock rows are recorded only
so the noise is visible rather than hidden.

## Result

The bit-exact variant is **Mixed, and strictly dominated by candidate 001**: it delivers a smaller
instruction-count win on `offset` (-2.47% vs 001's -3.57%) and turns `dash` from a -1.46%
improvement under 001 into a **+1.11% regression**. There is no scene on which it is the better
choice.

The practical conclusion for a reader deciding what to do with candidate 001: the bit-exactness is
not available "for free at a slightly smaller win". Taking the safe variant costs about a third of
the `offset`/`stroke` gain *and* gives up the `dash` gain entirely. If output stability is
non-negotiable this is what it costs; if the 3.95e-12 perturbation in 001 is acceptable (it is 9
orders of magnitude below the tolerance the caller asked for, and leaves the path structure
identical), 001 is the better patch on every measured axis.

Dimension measured: instruction count (primary) and wall clock (unusable this session).
Allocations unchanged.

## Does it change behavior/output?

- **Bit-exact?** **Yes** - which was the entire point. `CHECK` checksums compared across all 25
  harness scenes: **25 identical, 0 differing**. The sample parameter uses the identical expression
  `(i + 1) as f64 * (1.0 / (N_LSE + 1) as f64)` and the control points are combined in the same
  nested order `p0*c0 + (p1*c1 + (p2*c2 + p3*t)*t)*t` that `CubicBez::eval` uses.
- **Is this a breaking change?** No. `EvalWeights` and `EVAL_WEIGHTS` are private; no public API or
  output change of any kind.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, `grep -cE '^error'` = 0 both
  times.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors.
- `cargo clippy --workspace --all-features`: only the pre-existing `unneeded late initialization`
  warning in `kurbo/src/common.rs`. No new warnings.

## Why the result looks the way it does

Two mechanisms explain the gap against 001:

1. **`qa` survives.** Preserving `QuadBez::eval`'s exact operation order forces the weights to be
   applied to the *derivative's* control points, so `c_approx.deriv()` must still be constructed
   every call (3 vector subtractions and 3 scalar multiplications). 001 folds `deriv`'s factor of 3
   into its tangent weight table and applies it to the cubic's own control points, so the
   derivative curve never exists. That is a real saving 005 cannot have without changing rounding.
2. **More constant data per iteration.** 005 loads a 6-field, 48-byte `EvalWeights` per sample;
   001 loads two 4-element weight rows and does strictly fewer arithmetic operations with them.

The `dash` regression has the same shape as candidate 003's: dashing produces many short
sub-segments whose offsets converge without the least-squares refinement loop running, so the
per-call setup (the constant-table loads, and `qa` still being built) is paid without the
per-sample arithmetic saving being amortized over repeated `eval_err` calls. That the same
workload-dependent split shows up here and in 003, from two unrelated changes, is a property of the
`dash` scene worth remembering rather than a coincidence.

## What's not done / open questions

- 001's `stroke` and `dash` instruction counts were measured retroactively here to make the
  comparison fair (`dash` -1.46%); 001's own `RESULT.md` predates that number. 001's `stroke` `Ir`
  was not measured.
- Whether an intermediate variant exists - folding `deriv`'s factor of 3 in (001's main structural
  saving) while keeping the *cubic* evaluation in nested form - was not tried. It would be
  bit-exact for `pa` but not for `tana`, so it is a third point on the accuracy/speed curve rather
  than a strict improvement on either. Added to `IDEAS.md`.
- The wall-clock side needs redoing on a quiet host before any of the timing figures here are
  quotable.
