# 012. Making candidate 003's hoist lazy - the follow-up that closes that line of work

Candidate 003 hoisted `CubicOffset::apply`'s subdivision-only inputs (four curve evaluations and
two square roots) out of the least-squares refinement loop. It came out **Mixed**: -3.02% / -3.11%
instruction count on `offset` / `stroke`, but **+1.44% on `dash`**. The diagnosis there was that
`dash` produces many short sub-segments whose offsets converge on the first try, so the refinement
loop never runs - the hoisted values are then paid for (kept live across `eval_err` and
`refine_least_squares`) with nothing to amortize them over.

003's own "what's not done" section proposed the obvious fix and `IDEAS.md` carried it: **build the
hoisted values lazily, only once the refinement loop is actually about to run a second `apply`.**
That should keep the `offset`/`stroke` win and leave the `dash` path untouched.

It does not. This candidate measures that and closes the line.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

`apply` was split into `apply_base` (the `rec`-only part) and `apply_with_base`, with `apply`
itself rewritten as `self.apply_with_base(rec, self.apply_base(rec), a, b)` so the *first*,
unconditional call behaves exactly as before. The refinement loop then builds the base on demand:

```rust
let mut base: Option<ApplyBase> = None;
for _ in 0..N_REFINE {
    if err.err_squared <= self.tolerance * self.tolerance { break; }
    let (a2, b2) = self.refine_least_squares(rec, a, b, &err);
    let base = *base.get_or_insert_with(|| self.apply_base(rec));
    let c_approx2 = self.apply_with_base(rec, base, a2, b2);
    ...
}
```

A subdivision that converges immediately never touches `base`, so in principle the `dash` path pays
nothing.

### Deterministic instruction counts (`valgrind --tool=callgrind`, `<scene> 3`)

| scene | baseline | **012 (lazy)** | candidate 003 (eager), for comparison |
|---|---|---|---|
| `offset` | 743,181,232 | 735,201,326 (**-1.07%**) | **-3.02%** |
| `stroke` | 742,515,186 | 734,258,053 (**-1.11%**) | **-3.11%** |
| `dash` | 462,407,584 | 471,181,858 (**+1.90%**) | +1.44% |

### Wall clock (`paired.sh`, `RUNS=15`, `taskset -c 2`)

| scene | single-core |
|---|---|
| `offset` (20 iters) | 12.127 -> 12.292 ms (**+1.4%**), B faster in only **1 of 15** pairs - `consistent` **regression** |
| `dash` (20 iters) | 7.508 -> 7.777 ms (+3.6%), 4/15 - `NOT consistent`, but leaning regression |

Reproduced from a fresh clone by this folder's own `just test`, which pins the `dash` side down
more firmly than the in-tree run did: `offset` 12.273 -> 12.305 ms (+0.3%, 3/9, `NOT consistent`)
and `dash` 7.343 -> 7.661 ms (**+4.3%, B faster in 0 of 9 pairs**, `consistent`). So the `dash`
regression is robust across both measurement contexts, while the `offset` side is somewhere between
neutral and slightly negative depending on the run.

## Result

**A small regression, and strictly worse than candidate 003 on every scene measured.** The lazy
form wins *less* where 003 won (-1.07% vs -3.02% `Ir` on `offset`) and regresses *more* where 003
regressed (+1.90% vs +1.44% `Ir` on `dash`). Wall clock is worse still: `offset` is a
**consistent regression** (+1.4%, 1 of 15 paired wins) even though its instruction count improved -
the fifth instance in this workspace of `Ir` and wall clock disagreeing.

Taken with 003, this closes the line: hoisting `apply`'s subdivision-only inputs is not worth doing
in either the eager or the lazy form. Dimension measured: instruction count and time. Allocations
unchanged.

## Does it change behavior/output?

- **Bit-exact?** **Yes.** `CHECK` compared across all 25 harness scenes: 25 identical, 0 differing.
  The arithmetic and its ordering are unchanged; only *when* the values are computed differs.
- **Is this a breaking change?** No. `ApplyBase`, `apply_base` and `apply_with_base` are private.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, 0 lines matching `^error`.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors.
- `cargo clippy --workspace --all-features`: only the pre-existing `unneeded late initialization`
  warning in `common.rs`. No new warnings.

## Why the result looks the way it does

The lazy form was supposed to be free on the no-refinement path and identical to 003 on the
refinement path. It is neither, for two compounding reasons:

1. **Splitting `apply` changed the unconditional first call.** Every subdivision - including every
   short `dash` segment - now goes through `apply_base` building an `ApplyBase` struct that is
   immediately consumed by `apply_with_base`, rather than one flat function computing and using the
   values in place. Even with `#[inline]` on both halves, that is a different shape for LLVM to
   optimize, and `dash` got measurably worse (+1.90% vs 003's +1.44%) despite this variant being
   the one designed to protect it.
2. **`Option<ApplyBase>` + `get_or_insert_with` is not free.** The `Option` occupies a stack slot
   live across the loop, the closure captures `self` and `rec`, and the discriminant check runs on
   every refinement iteration - eating most of the saving on the path where the hoist was supposed
   to pay off (`offset` dropped from -3.02% to -1.07%).

The underlying lesson matches candidate 003's: `apply` is fully inlined into `offset_rec`, so there
is no call to eliminate and no expensive struct return to delete - the only thing hoisting can buy
is skipping four `eval`s, and that is smaller than the register-pressure and code-shape cost it
introduces. This is the same reason candidate 007's `#[inline]` on the large `subsegment` lost while
006's on tiny functions won: in this codebase, restructuring around a *small* amount of recomputed
arithmetic reliably costs more than the arithmetic itself.

## What's not done / open questions

- **Nothing further is warranted on this specific idea.** Both the eager (003) and lazy (012) forms
  have now been measured and both are net negative; `IDEAS.md`'s entry for the conditional variant
  has been retired accordingly. A future session should not re-derive either.
- The `find_subdivision_point` source note (`// Note: do we want to track p0 & p3 in rec, to avoid
  repeated eval?`) describes the *same* shape of redundancy in a different method and remains
  untested - but on the evidence of 003 and 012 its expected value is low. Kept in `IDEAS.md` with
  that caveat attached.
