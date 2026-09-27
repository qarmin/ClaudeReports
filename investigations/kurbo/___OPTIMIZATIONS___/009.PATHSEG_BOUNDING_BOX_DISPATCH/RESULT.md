# 009. `PathSeg::bounding_box` - dispatch once per segment instead of once per evaluation

Profiling the `bbox` scene put **97.4%** of self time in `<&[PathEl] as Shape>::bounding_box`.
That path runs `Segments::bounding_box`, which calls `ParamCurveExtrema::bounding_box(&seg)` for
each `PathSeg`.

`impl ParamCurveExtrema for PathSeg` overrides only `extrema()`; `bounding_box` falls through to
the trait's default:

```rust
fn bounding_box(&self) -> Rect {
    let mut bbox = Rect::from_points(self.start(), self.end());
    for t in self.extrema() {
        bbox = bbox.union_pt(self.eval(t));
    }
    bbox
}
```

Run against `PathSeg`, every one of `start()`, `end()` and each `eval(t)` inside the loop is a
`PathSeg` method - i.e. **the Line/Quad/Cubic match is re-done on every call**, including once per
extremum inside the loop.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

Three variants. The one in `patch.diff` is the **smallest** of them, and it is also the only one
that is faster - the other two are recorded here because they were measured and lost, and because
the reason they lost is the interesting part.

**Variant C (shipped, 14 lines, `patch.diff`)** - override `bounding_box` on `PathSeg` to match
once and hand off to the concrete curve:

```rust
fn bounding_box(&self) -> Rect {
    match *self {
        PathSeg::Line(line) => ParamCurveExtrema::bounding_box(&line),
        PathSeg::Quad(quad) => ParamCurveExtrema::bounding_box(&quad),
        PathSeg::Cubic(cubic) => ParamCurveExtrema::bounding_box(&cubic),
    }
}
```

(The fully-qualified call is required: `Line`/`QuadBez`/`CubicBez` each also have a
`Shape::bounding_box`, so a plain `line.bounding_box()` is ambiguous - `E0034`.)

**Variant A (rejected)** - variant C *plus* a specialized `bounding_box` on `CubicBez` that solves
the x and y derivative roots separately (no merge, no sort - `extrema()` sorts its combined list,
which a union does not care about) and evaluates only the relevant coordinate at each root, since a
root of the x derivative constrains only x.

**Variant B (rejected)** - variant C plus a `CubicBez` override that skips the merge-and-sort but
keeps the default's `self.eval(t)` + `union_pt` shape. Kept in this folder as
`rejected_variant_B_cubicbez.rs.txt`.

| Variant | `bbox` `Ir` (200 iters) | vs baseline | `bbox` wall clock, single-core `RUNS=15` |
|---|---|---|---|
| baseline | 152,345,493 | - | - |
| **C (shipped)** | **139,419,225** | **-8.49%** | **-9.9%, 14/15 `consistent`** |
| A (per-axis eval, no sort) | 144,078,253 | -5.43% | **+11.8%, 0/15** - consistent *regression* |
| B (no sort, `union_pt` kept) | 144,652,163 | -5.05% | **+50.5%, 0/15** - consistent *regression* |

Confirmation runs for variant C:

| scene | single-core (`taskset -c 2`) | multi-core (unpinned) |
|---|---|---|
| `bbox` (400 iters), run 1 | 0.066 -> 0.059 ms (**-9.9%**), 14/15 `consistent` | - |
| `bbox` (400 iters), run 2 | 0.058 -> 0.053 ms (**-8.6%**), 13/15 - one pair short of the threshold | 0.055 -> 0.052 ms (**-5.8%**), 14/15 `consistent` |
| `segments` (400 iters) | 0.034 -> 0.034 ms (+0.3%), 6/15 - `NOT consistent` (expected: it never calls `bounding_box`) | - |

## Result

A real win of **-5.8% to -9.9%** wall clock on `bbox`, from a 14-line dispatch override, backed by
a deterministic **-8.49%** instruction count. Two of the three confirmation runs cleared
`paired.sh`'s 14/15 threshold and the third missed by one pair, all in the same direction.
Dimension measured: time and instruction count. Allocations unchanged.

Scenes that don't compute bounding boxes are unaffected (`segments` +0.3%, 6/15 - noise).

## Does it change behavior/output?

- **Bit-exact?** **Yes.** `CHECK` compared across all 25 harness scenes: 25 identical, 0 differing.
  Variant C only changes *which* implementation of the same default algorithm runs; the arithmetic
  is identical.
- **Is this a breaking change?** No. `ParamCurveExtrema::bounding_box` already had a default; this
  overrides it for one type with a semantically identical result.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, 0 lines matching `^error`.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors.
- `cargo clippy --workspace --all-features`: only the pre-existing `unneeded late initialization`
  warning in `common.rs`. No new warnings.

## Why the result looks the way it does

The win in variant C is removing an enum match from the inner loop: `Rect::from_points(self.start(),
self.end())` plus one `self.eval(t)` per extremum were each dispatching on the `PathSeg` variant.
Hoisting the match to once per segment lets the whole body specialize to the concrete curve.

**The rejected variants are the more instructive half of this candidate.** Both execute *fewer
instructions* than the baseline (-5.4% and -5.0%) and are *slower in wall clock* (+11.8% and
+50.5%). Two mechanisms, both plausible and neither visible to an instruction counter:

- `Rect::union_pt` computes `x0.min(px), y0.min(py), x1.max(px), y1.max(py)` - the two mins and the
  two maxes each pair into a single packed-double operation. Variant A replaced that with scalar
  `bbox.x0 = bbox.x0.min(x)` in one loop and the y equivalent in another, which cannot pair, and
  serialized the updates through a mutable `Rect`.
- Both variants split one loop over a 4-element `ArrayVec` into two loops over 2-element ones,
  doubling the loop and `ArrayVec` setup/teardown overhead on a body that only ever runs 0-2 times.

Variant B's +50.5% is larger than variant A's and was not chased further; the shape of the cause
(extra loop/`ArrayVec` overhead swamping a body this small) is the same.

**This candidate is this session's clearest demonstration that instruction count is not a proxy for
time in either direction here.** Candidates 001 and 006 had wall-clock wins 3-5x larger than their
`Ir` wins; variants A and B here have `Ir` wins that are wall-clock *regressions*. `Ir` is
trustworthy for confirming a direction when wall clock is too noisy to resolve it (candidate 003),
but it must never be the only measurement.

## What's not done / open questions

- `QuadBez::extrema` has the same shape as the cubic's (it builds a list and does a conditional
  swap to sort two elements) and `QuadBez` has no specialized `bounding_box`. Given variants A/B
  lost on the cubic, a per-axis quad version is unlikely to win and was not tried.
- `arclen_rec` is 95-99% of the `arclen` and `area` scenes and remains completely untouched - it is
  the largest single unexplored hot spot left in this workspace. It also carries its own source
  note (`// It might be faster to do direct multiplies, the data dependencies would be shorter`).
  Added to `IDEAS.md`.
- The `nearest` scene is 61% inside `polycool` (`find_root`, `roots_between`, `first_root`) - a
  *workspace member*, so directly patchable without vendoring. Untouched. Added to `IDEAS.md`.
- `svg-parse` is 70% in `SvgLexer::get_number`. Untouched. Added to `IDEAS.md`.
