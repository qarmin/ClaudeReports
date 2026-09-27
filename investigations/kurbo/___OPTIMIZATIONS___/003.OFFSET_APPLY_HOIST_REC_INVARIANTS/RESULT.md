# 003. `CubicOffset::apply` - hoist the subdivision-only inputs out of the refinement loop

`CubicOffset::apply` turns a candidate `(a, b)` parameter pair into a cubic approximation. It is
8.5-9.3% of self time in the `stroke`/`offset` bench scenes. `offset_rec` calls it once for the
initial arc approximation and then again for each least-squares refinement step (up to
`N_REFINE = 2` more), always with the same `rec`.

Four of the values `apply` computes depend only on `rec`, not on `a`/`b`:

- `p0 = c.eval(t0) + d * utan0.turn_90()`
- `p3 = c.eval(t1) + d * utan1.turn_90()`
- `s * q.eval(t0).length()` (the `a`-independent part of `l0`)
- `s * q.eval(t1).length()` (the `b`-independent part of `l1`)

That is four curve evaluations plus two square roots, repeated for every candidate `(a, b)`. The
source even carries a note at the top of the function - `// wondering if p0 and p3 should be in
rec` - so the redundancy was known.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

An `ApplyBase` struct holding those four values, computed once per `offset_rec` by a new
`apply_base` method (placed *after* the cusp early-return, so the cusp path doesn't pay for it),
and passed to each `apply` call. The arithmetic is unchanged and in the same order - `l0` was
already `(s * len) + a * d`, so splitting it at the `+` preserves the result bit-for-bit.

Two variants were built, because a struct passed by reference across a call boundary can cost more
than it saves:

- **A**: `ApplyBase` passed by `&`, no inline hints.
- **B**: `ApplyBase` `Copy`, passed by value, `#[inline]` on both `apply` and `apply_base`.

**The two variants are codegen-equivalent** - their instruction counts differ by 158 out of 720
million on `offset` (0.00002%) and 48 out of 469 million on `dash`. LLVM was already scalarizing
the struct in variant A, so the passing convention is not what drives the result below. `patch.diff`
contains variant B.

### Deterministic instruction counts (`valgrind --tool=callgrind`, `kurbo_bench <scene> 3`)

These are exact and repeat to the last digit; no noise floor applies. They are the primary
evidence for this candidate, because the host's load average climbed from ~1-3 to ~9.5-10 during
this candidate's measurement window and wall clock could no longer resolve a ~3% effect.

| scene | baseline `Ir` | variant A `Ir` | delta | variant B `Ir` |
|---|---|---|---|---|
| `offset` | 743,181,009 | 720,707,489 | **-3.02%** | 720,707,647 |
| `stroke` | 742,515,414 | 719,445,419 | **-3.11%** | 719,445,300 |
| `dash` | 462,408,075 | 469,079,027 | **+1.44%** | 469,078,979 |

### Wall clock (`paired.sh`, `RUNS=9`, taken while the host was at load ~10)

| scene | single-core (`taskset -c 2`) | multi-core (unpinned) |
|---|---|---|
| `offset` | 14.338 -> 14.303 ms (-0.2%), paired **7/9** - `NOT consistent` | 14.617 -> 14.487 ms (-0.9%), paired 6/9 - `NOT consistent` |
| `stroke` | 13.909 -> 13.773 ms (-1.0%), paired 6/9 - `NOT consistent` | 14.137 -> 14.043 ms (-0.7%), paired 6/9 - `NOT consistent` |
| `dash` | 8.654 -> 8.962 ms (**+3.6%**), paired **1/9** - `consistent` | 8.866 -> 9.276 ms (**+4.6%**), paired 1/9 - `consistent` |

The `offset`/`stroke` rows are explicitly *not* evidence - `paired.sh` said so itself. Note the
baseline for `offset` also drifted from 12.1 ms (candidate 001's window) to 14.3 ms here with no
code change, which is exactly the host-noise scenario MEASUREMENT.md section 2c describes. The
`dash` regression, by contrast, cleared the threshold in both modes *and* is corroborated by the
deterministic instruction count, so it is the one wall-clock row worth believing.

## Result

**Mixed, and the mixture is real rather than measurement noise**: a ~3% instruction-count
reduction on `offset` and `stroke`, against a ~1.4% instruction-count *increase* on `dash`. Both
directions are confirmed by callgrind, which has no noise floor. The dimension measured is
instruction count primarily and wall clock secondarily; allocation counts are unchanged (this
patch allocates nothing).

The wall-clock magnitude of the `offset`/`stroke` win could not be established in this session -
the host was too loud at the time to resolve 3%, and `paired.sh` correctly refused to call it.
Re-measuring those two scenes on a quiet host is the obvious follow-up.

## Does it change behavior/output?

- **Bit-exact?** **Yes.** `CHECK` checksums compared across **all 25 harness scenes**: 25
  identical, 0 differing. The split of `l0 = s * len + a * d` into `l0_base = s * len` followed by
  `l0_base + a * d` preserves the original operation order exactly, and `p0`/`p3` are the same
  expressions merely evaluated once instead of up to three times.
- **Is this a breaking change?** No. `ApplyBase`, `apply` and `apply_base` are all private; no
  public API or documented behavior changes.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, `grep -cE '^error'` = 0 both
  times.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors.
- `cargo clippy --workspace --all-features`: only the pre-existing `unneeded late initialization`
  warning in `kurbo/src/common.rs`. No new warnings.

## Why the result looks the way it does

`apply` is fully inlined into `offset_rec` in both builds - callgrind shows no standalone `apply`
symbol in either, and the cost lands on `offset_rec` directly. So this is not a function-call
question; it is a register-pressure question.

In the `dash` scene, `offset_rec`'s *own* attributed instruction count goes **up**: 13,301,515 ->
14,719,095 for the recursive frame and 8,787,785 -> 10,636,560 for the outer one. The most likely
mechanism: `base` must stay live across the intervening `eval_err` and `refine_least_squares`
calls, which are themselves register-hungry (`eval_err` alone keeps two 8-element `Vec2` arrays).
The original code recomputed `p0`/`p3`/`l0`/`l1` *after* those calls and therefore needed nothing
live across them. Hoisting trades four curve evaluations for six extra live `f64`s, and in a
workload where the refinement loop usually doesn't run, that trade is a loss.

That last clause is the key to the split: dashing a path produces many short sub-segments, whose
offsets converge on the first try, so `apply` is called **once** and there is nothing to amortize
the hoist over - only the added pressure. The `offset` scene's 2 000 random cubics at `d = 5`
need refinement far more often, so the same hoist pays for itself there.

This is a genuine case of the same change being right for one workload and wrong for another, not
a measurement artifact - which is why it is recorded as Mixed rather than being tuned until one
number looked good.

## What's not done / open questions

- **A conditional variant was not tried**: compute `base` lazily only when the refinement loop is
  actually about to run a second `apply` (i.e. hoist after the first error evaluation shows the
  tolerance was missed). That would keep the `offset`/`stroke` win and avoid the `dash` cost, at
  the price of a more tangled `offset_rec`. This is the obvious next step and is left in
  `IDEAS.md`.
- The `offset`/`stroke` wall-clock magnitude needs re-measuring on a quiet host; only the
  instruction-count win is established for them.
- `find_subdivision_point` carries its own note - `// Note: do we want to track p0 & p3 in rec, to
  avoid repeated eval?` - pointing at the same redundancy in a different method. Not touched here
  (one fix per candidate); left in `IDEAS.md`.
- Whether the `dash` regression also appears with other dash patterns / segment lengths was not
  explored; only the harness's single 4-entry pattern was measured.
