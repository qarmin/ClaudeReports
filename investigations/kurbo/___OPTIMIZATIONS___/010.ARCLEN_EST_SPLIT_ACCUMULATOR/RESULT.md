# 010. `arclen_rec` - split the error-estimate accumulator, and why it doesn't pay off

`cubicbez::arclen_rec` is the largest untouched hot spot in this workspace: **95.2% of the `arclen`
scene** and **99.6% of the `area` scene** (which reaches it through `perimeter`). Dashing uses it
heavily too. Its error estimate is an 8-term Gauss-Legendre sum:

```rust
let est = GAUSS_LEGENDRE_COEFFS_8.iter()
    .map(|&(wi, xi)| { ... dd_norm2 / d_norm2 })
    .sum::<f64>();
```

`.sum()` accumulates into a single variable, so the eight additions form a **serial dependency
chain** - each add waits ~4 cycles for the previous one. Candidates 001 and 006 in this workspace
both produced wall-clock wins several times larger than their instruction-count wins precisely
because they shortened dependency chains, so this looked like the same shape of opportunity.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

Accumulate even- and odd-indexed terms into two independent variables and add them once at the end,
halving the chain from 8 dependent adds to 4 plus one.

### Deterministic instruction counts (`valgrind --tool=callgrind`, `<scene> 20`)

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `arclen` | 131,823,330 | 129,304,661 | **-1.91%** |
| `area` | 59,093,752 | 58,007,542 | **-1.84%** |
| `dash` | 2,031,766,435 | 2,018,334,944 | **-0.66%** |

### Wall clock (`paired.sh`, `RUNS=15`)

| scene | single-core (`taskset -c 2`) | multi-core (unpinned) |
|---|---|---|
| `arclen` (100 iters) | 0.348 -> 0.342 ms (-1.7%), 12/15 - `NOT consistent` | 0.354 -> 0.352 ms (-0.6%), 8/15 - `NOT consistent` |
| `area` (200 iters) | 0.165 -> 0.165 ms (+0.2%), 3/15 - `NOT consistent` | 0.167 -> 0.168 ms (+0.6%), 4/15 - `NOT consistent` |
| `dash` (20 iters) | 7.387 -> 7.507 ms (**+1.6%**), 1/15 - **`consistent`** | 7.740 -> 7.819 ms (+1.0%), 6/15 - `NOT consistent` |

Reproduced from a fresh clone by this folder's own `just test`: `arclen` 0.500 -> 0.503 ms (+0.6%,
8/15) and `dash` 9.258 -> 9.263 ms (+0.1%, 7/15) - both squarely `NOT consistent`, i.e. **neutral**.
Notably the `dash` regression seen in-tree (+1.6%, 1/15) did **not** reproduce here. Taking both
runs together, the honest reading is that this patch has no reliable wall-clock effect in either
direction, and the single consistent-looking regression was borderline rather than robust.

## Result

**Mixed**: instruction count improves consistently and measurably (-0.66% to -1.91% across three
scenes, deterministic), but wall clock does not follow. `arclen` leans positive without clearing
the threshold (12/15), `area` leans very slightly negative (3-4/15), and `dash` is a **consistent
small regression** (+1.6%, 1 of 15 paired wins). Dimension measured: time and instruction count.
Allocations unchanged.

Net: not a win. The instruction-count improvement is real but does not translate.

## Does it change behavior/output?

- **Bit-exact?** **Yes, in practice** - `CHECK` identical on all 25 harness scenes, and a direct
  numeric dump of all 2 000 arclen values (`kurbo_bench --dump arclen`, added for this candidate)
  gives **0 of 2000 differing, max absolute difference 0**.
- Worth being precise about *why*, because reordering a floating-point sum normally is **not**
  bit-exact: `est` is never part of the returned value. It feeds only the three threshold
  comparisons (`est_gauss8_error < accuracy` and friends) that pick which quadrature rule to use.
  The arclength itself comes from `arclen_quadrature_core`, which this patch does not touch. So a
  last-ulp change in `est` is invisible unless it flips a threshold - which it did not do for any
  of the 2 000 test curves, at any of the accuracies exercised. **It is not bit-exact by
  construction**, and a curve sitting exactly on a threshold boundary could in principle select a
  different quadrature rule and return a (still within-tolerance) different length.
- **Is this a breaking change?** No; `arclen_rec` is a private function and the accuracy contract
  is unchanged.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, 0 lines matching `^error`.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors.
- `cargo clippy --workspace --all-features`: only the pre-existing `unneeded late initialization`
  warning in `common.rs`. No new warnings.

## Why the result looks the way it does

The dependency-chain hypothesis was wrong for this loop, and the reason is visible in the loop body:
each term costs a **division** (`dd_norm2 / d_norm2`, ~14 cycles latency, ~4 cycles throughput) plus
two `hypot2` dot products. The additions were never the critical path - they already had ample slack
to overlap with the next iteration's division. Halving a chain that wasn't binding buys nothing,
while the extra accumulator and the even/odd split add register pressure and (before LLVM unrolls
it) a branch.

That the instruction count nonetheless *improved* by ~1.9% is a good reminder in the other
direction: `Ir` moved cleanly and consistently across three scenes, and it still didn't predict
wall clock. This is the third candidate in this workspace where the two disagree (001 and 006:
wall clock >> `Ir`; 009 variants A/B: `Ir` win, wall-clock regression).

The `dash` regression is the third time in this session that a change helping or neutral on
`offset`/`arclen` has regressed `dash` specifically (see also candidates 003 and 005). `dash`
exercises many short segments and shallow recursion, so per-call setup costs are amortized over
much less work than in the other scenes.

## What's not done / open questions

- **The real cost in `arclen_rec` is untouched**: 4 `hypot` (square roots) for `lp_lc` computed
  unconditionally on every recursive call, and 8 divisions in the `est` sum. Neither has an obvious
  bounded fix - `lp_lc` is needed for the very first threshold test, and the divisions are
  independent per term. A genuine win here likely needs a numerical reformulation of the error
  estimate, not a code-shape change. Left in `IDEAS.md`.
- The maintainer's own note in this function - `// It might be faster to do direct multiplies, the
  data dependencies would be shorter`, about `dm`/`dm1`/`dm2` - was **not** tried. Those simplify
  algebraically to `dm1 = 0.5 * (d23 - d01)` and `dm2 = 0.25 * (d01 - 2*d12 + d23)`, saving two
  vector subtractions, and unlike this candidate the result would genuinely change (those values
  *do* feed the returned length). Given this candidate's outcome, a two-subtraction saving in a
  function dominated by 4 square roots and 8 divisions is unlikely to be measurable. Left in
  `IDEAS.md` with that caveat.
- Whether a 4-way split (rather than 2-way) behaves differently was not tried; on this evidence it
  would not.
