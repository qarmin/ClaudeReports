# 002. `ToQuads` - stop re-deriving the cubic and re-evaluating the shared endpoint per segment

`kurbo::flatten` is the most-called entry point in the library for any renderer. Profiling its
bench scene put `CubicBez::subsegment` at 23.9% of self time (behind only
`QuadBez::estimate_subdiv` at 40.7%), reached exclusively through `CubicBez::to_quads`, which
`flatten` calls for every `CurveTo` element.

`ToQuads::next` called `self.c.subsegment(t0..t1)` for each of the `n` segments. `subsegment`
does two cubic evaluations, builds the cubic's derivative, and does two evaluations of that
derivative. Two of those four evaluations are avoidable every step:

- `self.deriv()` does not depend on `t0`/`t1` at all, so it was rebuilt `n` times identically.
- Consecutive segments share a parameter value - segment `i` ends at `(i+1)/n`, and segment `i+1`
  starts at exactly the same `(i+1)/n`, computed by the same expression - so each segment's end
  point and end derivative are the next segment's start point and start derivative.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)` (recorded
because this candidate's evidence includes callgrind instruction counts).

## What was tried

`ToQuads` gained three fields: `q` (the derivative, built once in `to_quads`), and `p0`/`d0` (the
current segment's start point and start derivative, carried over from the previous iteration).
`next` now evaluates only the *end* point and end derivative, then hands them to the next
iteration. The arithmetic that builds the segment is written out in the same order `subsegment`
used, rather than calling it, so the result is unchanged bit-for-bit.

Per segment this goes from 2 cubic evaluations + 1 derivative construction + 2 derivative
evaluations, to 1 cubic evaluation + 1 derivative evaluation.

Command: `taskset -c 2 kurbo_bench <scene> <iters>`, compared with
`___OPTIMIZATIONS___/tools/paired.sh` (alternating A/B invocations, `RUNS=9`).

| Variant | single-core p50 (`taskset -c 2`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline, `flatten` (100 iters) | 0.789 ms | 0.685 ms | |
| patched, `flatten` | 0.738 ms (**-6.5%**) | 0.664 ms (**-3.1%**) | paired 8/9 both modes, verdict `consistent` |
| baseline, `flatten-fine` (30 iters, tol 0.001) | 5.244 ms | 4.460 ms | |
| patched, `flatten-fine` | 4.970 ms (**-5.2%**) | 4.328 ms (**-2.9%**) | paired 9/9 both modes |
| baseline, `flatten-small` (200 iters, 10x fewer curves) | 0.077 ms | 0.068 ms | second problem size |
| patched, `flatten-small` | 0.073 ms (**-4.9%**) | 0.066 ms (**-2.8%**) | paired 9/9 both modes |
| baseline, `mixed-flatten` (50 iters, line/quad/cubic mix) | 0.676 ms | 0.591 ms | |
| patched, `mixed-flatten` | 0.649 ms (**-4.0%**) | 0.580 ms (**-1.9%**) | paired 9/9 both modes |

Independently reproduced from a fresh clone at the pinned commit via this folder's own
`just test`: `flatten` 0.659 -> 0.642 ms (-2.6%, paired 8/9), `flatten-fine` 4.343 -> 4.271 ms
(-1.6%, paired 9/9), both `consistent`, and `CHECK` identical between baseline and patched on both
scenes. Note the *direction* and the consistency reproduce but the *magnitude* is roughly half
what the in-tree measurement showed - the clone-built binaries were also faster in absolute terms
(0.659 vs 0.789 ms baseline), so this looks like a different machine state (boost clock / cache
pressure) rather than a different result. Treat the -4 to -6.5% single-core figure as the upper
end of the range and -1.6 to -3% as the lower; both are the same sign with the same paired-win
consistency, which is what the comparison is designed to establish.

Deterministic instruction count (`valgrind --tool=callgrind`, `kurbo_bench flatten 5`):

| | baseline | patched | delta |
|---|---|---|---|
| whole program `Ir` | 34,685,378 | 31,055,282 | **-10.47%** |

The proportional win is consistent across a 10x change in problem size (`flatten` vs
`flatten-small`), which is what you would expect from a per-segment saving and rules out a
fixed-overhead artifact (MEASUREMENT.md section 6).

Memory: unchanged. `flatten` requests 2.1 allocations / 1 064 bytes per iteration before and
after - this candidate adds two `f64`s and a `QuadBez` to a stack-allocated iterator and allocates
nothing.

## Result

A real, bit-exact win of **-4.0% to -6.5% single-core** and **-1.9% to -3.1% multi-core** across
all four flattening scenes, with 8-9 of 9 paired runs going the same way in every one of the eight
comparisons (`paired.sh` verdict `consistent` throughout). The measured dimension is **time**;
allocation count was also measured and is unchanged.

The multi-core percentages are consistently about half the single-core ones, and the multi-core
absolute times are lower for both binaries - the unpinned runs boost to a higher clock, so the
same removed work is a smaller share of a shorter wall time. Both are reported rather than
picking the flattering one.

Note the instruction-count win (-10.5%) is *larger* than the wall-clock win here - the opposite of
candidate 001, where wall clock led. That is consistent with the mechanism: this candidate deletes
real instructions (redundant evaluations) rather than shortening a dependency chain, and the
remaining flatten work includes a per-output-point user callback that instruction counting weights
differently from the CPU.

## Does it change behavior/output?

- **Bit-exact?** **Yes.** Verified beyond the affected scenes: the harness's `CHECK` checksum was
  compared across **all 25 scenes** (which cover flatten, stroke, dash, offset, simplify, arclen,
  area, bbox, affine, SVG parse/write, nearest, intersect, segments, fit, the solvers, deriv and
  subdivide) - **25 identical, 0 differing**.
- The cached values are the same values the old code computed: `self.deriv()` is deterministic and
  was already being rebuilt identically each step, and the shared parameter is produced by the
  identical expression `(i + 1) as f64 / n as f64` at both ends of the seam. The first segment's
  start is initialized with `self.eval(0.0)` / `q.eval(0.0)` rather than with `self.p0` directly,
  specifically so that the sign of a negative zero is handled exactly as before.
- **Is this a breaking change?** No. `ToQuads` is a private struct; `to_quads` keeps its signature
  and returns `impl Iterator<Item = (f64, f64, QuadBez)>` as before. No public API change, no
  output change.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, `grep -cE '^error'` = 0 both
  times. This includes `cubicbez.rs`'s own `degenerate_to_quads` test (collinear control points)
  and its `to_quads` accuracy test, both of which exercise this iterator directly.
- `cargo build -p kurbo --no-default-features --features libm` (the `no_std` path): succeeds, 0
  errors.
- `cargo clippy --workspace --all-features`: only the pre-existing `unneeded late initialization`
  warning in `kurbo/src/common.rs`, which this patch does not touch. No new warnings.

## Why the result looks the way it does

`subsegment` is a general-purpose method: given an arbitrary `t` range it has no way to know the
caller will immediately ask for an adjacent range. `ToQuads` walks a contiguous, evenly spaced
partition of `0..1`, so half of what `subsegment` computes for each step was already computed for
the step before. Rebuilding the derivative every iteration is pure loss with no such excuse - it
is loop-invariant by construction.

The win is bounded by the fact that `subsegment` was 23.9% of the scene and roughly half of its
work is removed, which predicts ~12% of the scene; the measured -6.5% single-core is lower than
that, most likely because `QuadBez::estimate_subdiv` (40.7%, untouched) and the per-point callback
dominate what remains, and because the surviving evaluation of the new endpoint keeps some of the
same cache traffic.

## What's not done / open questions

- `CubicBez::to_quads` has exactly **one** non-test caller in the library (`flatten`, at
  `kurbo/src/bezpath.rs:664`), so the benefit is scoped to flattening. Downstream users calling
  `to_quads` directly get it too, since it is public API.
- `QuadBez::estimate_subdiv` is the larger remaining share of the flatten scene (40.7%) and was
  not touched. It is roughly 9-10 square roots per call across `approx_parabola_integral` (which
  does `.sqrt().sqrt()`), `approx_parabola_inv_integral`, `dd.hypot()` and `scale.sqrt()`; those
  look inherent to the approximation formulas rather than redundant, so it is not obviously the
  same kind of easy win. Left in `IDEAS.md`.
- `QuadBez::subsegment` has the same shape of redundancy (`eval(t0)`/`eval(t1)` on adjacent
  ranges) but no equivalent hot iterator was found calling it in a contiguous walk; not
  investigated further.
- This stacks with candidate 001 (different subsystem, different file); the combination was not
  measured together, per this workspace's one-fix-per-measurement rule.
