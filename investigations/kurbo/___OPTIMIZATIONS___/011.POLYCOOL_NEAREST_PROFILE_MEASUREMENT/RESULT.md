# 011. Where `ParamCurveNearest` actually spends its time - a measurement, not a patch

**There is no `patch.diff` in this folder, by design** - this is a pure measurement/investigation
candidate (SKILL.md's "patch.diff is mandatory" rule, second valid state). Nothing was applied to
kurbo's or polycool's source. See "Does it change behavior/output?" below.

`CubicBez::nearest` is open GitHub issue #437's subject ("(reasonably) efficient
`ParamCurveNearest` on `BezPath`") and looked like an obvious optimization target. It isn't a kurbo
problem at all: **most of its cost is inside `polycool`**, the polynomial root-finder that kurbo
delegates to. This candidate establishes that split with numbers, and rules out the cheap fixes, so
a future session doesn't spend a candidate rediscovering it.

`polycool` is a **member of this same cargo workspace**, not an external dependency - so SKILL.md's
"vendoring a suspect dependency" procedure does not apply; it is directly patchable in place.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was measured

`kurbo_bench nearest` - `ParamCurveNearest::nearest` over 2 000 random cubics at accuracy 1e-6.
Scene cost: 0.432 ms median, **0 allocations per iteration** (8 bytes total - i.e. nothing on the
heap at all).

`perf record -F 999 -g`, `taskset -c 2`, self time:

| symbol | self % |
|---|---|
| `<polycool::poly::Poly<6>>::roots_between` | **54.7%** |
| `polycool::yuksel::find_root::<Poly<6>::roots_between_with_buffer::{closure#0}, {closure#1}>` | 24.4% |
| `<polycool::poly::Poly<4>>::first_root` | 10.9% |
| `<CubicBez as ParamCurveNearest>::nearest` | 9.9% |

Callgrind (deterministic, `nearest 10`, top attributed lines of 44,967,698 total `Ir`):

| source | `Ir` | share |
|---|---|---|
| `polycool/src/yuksel.rs` in `find_root` (Poly<6>) | 8,724,888 | 19.4% |
| `polycool/src/yuksel.rs` in `Poly<6>::roots_between` | 7,380,612 | 16.4% |
| `polycool/src/yuksel.rs` in `Poly<4>::first_root` | 4,550,412 | 10.1% |
| `polycool/src/poly.rs` in `Poly<6>::roots_between` | 3,536,760 | 7.9% |
| `core/ops/arith.rs` in `Poly<6>::roots_between` | 3,385,596 | 7.5% |
| `kurbo/src/vec2.rs` in `CubicBez::nearest` | 2,566,944 | 5.7% |

**About 90% of this scene is polynomial root-finding; roughly 10% is kurbo's own code.**

## Result

**N/A - characterization only, no patch was applied or compared against a baseline.** The finding
is the attribution above: `ParamCurveNearest` is a `polycool` workload, and optimizing
`kurbo/src/cubicbez.rs` would be working on 10% of the problem.

## Does it change behavior/output?

- **N/A - measured/quantified only, no patch applied to real source.** Nothing was modified;
  the working tree was never dirtied for this candidate.
- Project test suite: not re-run for this candidate, because no change was made - the baseline for
  this session is recorded in every other candidate here (`cargo test --workspace --all-features` =
  252 passed / 0 failed, 4 `test result` lines, 0 `^error` lines).

## Why the result looks the way it does - and what was ruled out

`polycool` is already carefully optimized, and its authors left the evidence in comments. Three
specific cheap fixes were considered and **rejected on inspection**, each for a concrete reason:

1. **"Use a faster polynomial evaluation."** `Poly::eval` is already textbook Horner
   (`acc = acc * x + c` over reversed coefficients), with a comment explaining that `f64::mul_add`
   was deliberately *not* used because it is slow on architectures without a fused-multiply-add
   instruction. Nothing to take here.
2. **"Evaluate the polynomial and its derivative jointly."** `yuksel.rs` carries exactly this note:
   *"There's potentially a performance gain to be had by evaluating the derivative and the
   polynomial jointly, so that subexpressions can be shared. At least for the cubic case, the
   compiler is smart enough to do that for us."* This is the shape of lead that paid off three
   times elsewhere in this session, so it was checked properly - and **it does not apply here**.
   The joint Horner-with-derivative trick wins when the derivative is *not* already available, at
   `n` multiply-adds instead of `2n`. But `roots_between_with_buffer` computes `deriv` once as a
   real `Poly<N-1>` and closes over it, so `find_root` evaluates two independent, already-reduced
   Horner chains - `n` plus `n-1` multiply-adds, i.e. *fewer* than the joint form, and independent
   chains schedule well. There is no redundancy to remove.
3. **"Reduce allocation."** There is none: `roots_between` uses stack `ArrayVec`s throughout, and
   the harness confirms **0 heap allocations per iteration** for the whole scene.

Two further non-redundancies worth recording, since they look like opportunities and are not:
`roots_between_with_buffer` already passes the bracket endpoint values (`last_val`, `val`) into
`find_root` rather than making it re-evaluate them; and the recursion reuses the caller's two
buffers by swapping `out`/`scratch` at each level rather than allocating per level.

What remains is genuinely algorithmic: the method finds all critical points of a degree-5
polynomial by full recursive descent (Poly<6> -> Poly<5> -> ... -> Poly<2>) before bracketing, for
every one of the 2 000 curves. A cheaper approach would have to change *that*, not the arithmetic
inside it.

## What's not done / open questions

- **The algorithmic angle is untouched and is the only remaining lever**: whether
  `CubicBez::nearest` needs the full root set of the degree-5 distance-derivative polynomial, or
  whether a cheaper bracketing/pruning step could avoid the full recursive critical-point descent
  for most inputs. That is a research-shaped change, not a patch-shaped one.
- Issue #437 asks specifically for an efficient `ParamCurveNearest` on a whole `BezPath`. That is a
  different (and easier) problem than making a single segment's `nearest` faster: a bounding-box
  prefilter over `segments()` would skip most segments entirely, and none of the cost measured here
  would need to change. That remains in `IDEAS.md` and is probably the better use of a future
  candidate than attacking `polycool`.
- `polycool` has no `___OPTIMIZATIONS___` of its own and does not need one - it is a workspace
  member of this repo, so any future candidate against it belongs in this workspace.
