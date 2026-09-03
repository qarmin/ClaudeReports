# 008. `Vec2::normalize` reciprocal-multiply - a null result, because the premise was false

`Vec2::normalize` reads `self / self.hypot()`. On a code read that looks like **two divisions**
(one per component), and division is several times more expensive than multiplication on x86.
`normalize` is called once per sample inside `CubicOffset::eval_err` - the hottest loop in the
library - plus at 17 other sites across the crate, 8 of them in `offset.rs`/`stroke.rs`. Replacing
it with one division and two multiplications looked like a cheap, one-line win.

It isn't, and the reason is worth writing down so nobody re-derives it: **the transformation was
already there.**

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

```rust
-        self / self.hypot()
+        self * self.hypot().recip()
```

## Results

### Deterministic instruction counts (`valgrind --tool=callgrind`, `<scene> 3`)

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `offset` | 743,181,009 | 743,178,743 | -0.0003% |
| `stroke` | 742,515,414 | 742,512,858 | -0.0003% |
| `dash` | 462,407,847 | 462,405,416 | -0.0005% |

Deltas of ~2 500 instructions out of ~740 million - i.e. nothing, attributable to harness startup.

### Cycle counts (`perf stat -e cycles`, `taskset -c 2`, 5 alternating runs each)

Instruction counting is structurally blind to this candidate - a `divsd` retires as one
instruction whatever its ~14-cycle latency - so cycles were measured directly:

| scene | outcome |
|---|---|
| `offset` | patched lower in 5/5 pairs, but by 0.02%-1.3% (medians ~1 413M -> ~1 405M, ~-0.5%) |
| `stroke` | patched lower in 2/5 pairs - no signal at all |

### Wall clock (`paired.sh`, `RUNS=15`)

| scene | single-core (`taskset -c 2`) | multi-core (unpinned) |
|---|---|---|
| `offset` | 14.658 -> 14.683 ms (+0.2%), 4/15 - `NOT consistent` | 12.366 -> 12.358 ms (-0.1%), 8/15 - `NOT consistent` |
| `stroke` | 14.274 -> 14.277 ms (+0.0%), 9/15 - `NOT consistent` | 11.818 -> 11.827 ms (+0.1%), 6/15 - `NOT consistent` |

All four `NOT consistent`, all four medians within ±0.2%.

## Result

**No change**, on every dimension measured: instruction count (-0.0003%), cycles (no signal on
`stroke`, at most ~0.5% on `offset` and not separable from noise), and wall clock (±0.2%, 4-9 of 15
paired wins - i.e. a coin flip). Allocations unchanged.

## Does it change behavior/output?

- **Bit-exact?** **Yes** - and this is the diagnostic that explains the whole candidate. `CHECK`
  was identical across all 25 harness scenes. A genuine `a / b` -> `a * (1/b)` rewrite changes
  rounding and *cannot* be bit-exact; getting 25/25 identical was the signal that the two forms
  were already the same computation.
- **Is this a breaking change?** No - and in fact no change whatsoever.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, 0 lines matching `^error`.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors.

## Why the result looks the way it does

`Vec2`'s own `Div<f64>` implementation already does the reciprocal multiply, and says so
(`kurbo/src/vec2.rs:474`):

```rust
impl Div<f64> for Vec2 {
    /// Note: division by a scalar is implemented by multiplying by the reciprocal.
    ///
    /// This is more efficient but has different roundoff behavior than division.
    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: f64) -> Vec2 {
        self * other.recip()
    }
}
```

So `self / self.hypot()` **was already** `self * self.hypot().recip()`, exactly. The patch renamed
an operation without changing it, which is why the instruction counts, the cycle counts, the wall
clock and the checksums all agree that nothing happened.

This is the second time in this session that a `vec2.rs` "obvious libm/arithmetic win" turned out
to be already done. The first was `Vec2::hypot`, which an earlier version of this workspace's
`IDEAS.md` listed as "calls the slow `f64::hypot`" - it does not, it is `hypot2().sqrt()`, again
with a comment saying so. **`vec2.rs` has already been hand-optimized and its comments record it.
Read the operator impls, not just the method bodies, before proposing arithmetic changes there.**

## What's not done / open questions

- The genuinely open question this candidate does *not* answer: whether `normalize`'s `sqrt` +
  `recip` (a division) could become a single reciprocal-square-root sequence
  (`hypot2().sqrt().recip()` -> an `rsqrt`-style approximation plus a Newton step). That would be a
  real change in both cost and accuracy, unlike this one. Not attempted - kurbo forbids `unsafe`
  and has no intrinsics access, so it would depend on LLVM choosing to emit `vrsqrtss`, which it
  will not do for `f64` without fast-math. Effectively closed on this toolchain.
- No follow-up work is warranted on this specific line.
