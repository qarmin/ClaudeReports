# 016. `fit_to_cubic` - three `atan2` + three `sin_cos` replaced by cross/dot products

`fit.rs:401-407` was the largest remaining libm cost anywhere in the kurbo profile: `atan2` is
17.6% of the `fit` scene's self time (`__ieee754_atan2`, plus a further 3.2% in `__atan2`) and
`__sincos` another 8.0%. Per `fit_to_cubic` call it did

```rust
let th = d.atan2();
let th0 = mod_2pi(start.tangent.atan2() - th);
let th1 = mod_2pi(th - end.tangent.atan2());
...
let aff = Affine::translate(start.p.to_vec2()) * Affine::rotate(th) * Affine::scale(chord);
for (cand, d0, d1) in cubic_fit(th0, th1, unit_area, mx) { ... }
```

Reading the consumers first (as the candidate brief asked) showed that **none of the three angles
is ever used as an angle**:

- `th0` and `th1` reach exactly one place, `cubic_fit`, whose first two lines are
  `th0.sin_cos()` / `th1.sin_cos()`. They are never compared, never added to anything.
- `th` reaches only `Affine::rotate(th)`, which is `sin_cos` again.

So all three arctangents and all three sine/cosine pairs cancel, and this turned out to be a
local rewrite after all rather than the algorithmic change the brief anticipated.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

`th0` and `th1` are *differences* of angles, so for the chord vector `d` and a tangent `u`:

```
sin(th0) = d.cross(u) / (|d| |u|)      cos(th0) = d.dot(u) / (|d| |u|)
sin(th1) = u.cross(d) / (|d| |u|)      cos(th1) = d.dot(u) / (|d| |u|)
```

The `(cross, dot)` pair already has magnitude `|d| |u|`, so one `sqrt().recip()` normalizes both
components at once - no separate tangent normalization and no chord length needed:

```rust
let (s0, c0) = unit_sin_cos(d.cross(tan0), d.dot(tan0));
let (s1, c1) = unit_sin_cos(tan1.cross(d), d.dot(tan1));
```

and `cubic_fit` takes the four scalars instead of two angles (its own source already asked for
this: `// Note: maybe we want to take unit vectors instead of angle?`).

`mod_2pi` disappears: `sin`/`cos` are 2*pi-periodic, so reducing the argument only ever mattered
for the accuracy of the `sin_cos` call that no longer happens.

Separately, `Affine::rotate(th) * Affine::scale(chord)` is the matrix
`[chord*cos(th), chord*sin(th), -chord*sin(th), chord*cos(th), 0, 0]`, i.e. the linear map that
sends `(1, 0)` to `d` - so it is just `Affine::new([d.x, d.y, -d.y, d.x, 0.0, 0.0])`, with no
trigonometry and no `chord2.sqrt()` at all.

A zero tangent has no direction. `Vec2::atan2` reported angle 0 for it (`atan2(0.0, 0.0) == 0.0`),
which is the `+x` direction, so the patch substitutes `Vec2::new(1.0, 0.0)` explicitly rather
than letting the normalization produce `NaN`; that keeps the degenerate-cusp path behaving as
before. (`sample_pt_deriv`'s own doc says "In curves with cusps, the derivative can go to zero",
and `SimplifyBezPath::sample_pt_tangent` hands the raw derivative straight through.)

Net per call: **3 `atan2` + 3 `sin_cos` + 1 `sqrt` -> 2 `sqrt` + 2 `recip`.**

### Wall clock (`tools/paired.sh`, `RUNS=9`, load average ~3.9)

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) |
|---|---|---|
| baseline, `fit` (20 iters) | 15.683 ms (repeats 15.74 / 15.83) | 15.754 ms (repeats 15.56 / 15.80) |
| patched, `fit` | 14.280 ms (**-8.9%**), paired 9/9 `consistent` | 14.628 ms (**-7.1%**), paired 9/9 `consistent` |
| baseline, `fit-opt` (5 iters) | 92.129 ms (repeats 91.77 / 92.42) | 91.423 ms (repeats 90.71 / 91.53) |
| patched, `fit-opt` | 83.515 ms (**-9.4%**), paired 9/9 `consistent` | 85.091 ms (**-6.9%**), paired 9/9 `consistent` |
| baseline, `simplify` (500 iters) | 0.2213 ms (repeats 0.2200 / 0.2202) | 0.2220 ms (repeats 0.2199 / 0.2202) |
| patched, `simplify` | 0.2185 ms (**-1.3%**), paired 8/9 `consistent` | 0.2188 ms (**-1.5%**), paired 9/9 `consistent` |

Baseline was measured twice before patching (the "repeats" column above): the run-to-run drift was
0.5-1.5% on `fit`, 0.7-0.9% on `fit-opt` and 0.1% on `simplify`, so the `fit`/`fit-opt` effects are
5-10x the noise floor and `simplify`'s is ~10x its own.

**Attribution, and a second measurement window.** A partial variant that keeps
`Affine::rotate(d.atan2()) * Affine::scale(chord)` and rewrites only `th0`/`th1` was built and
measured too. By then the host load had risen to ~15 and the untouched baseline had drifted
15.68 -> 18.38 ms, so this block is internally comparable but not comparable to the table above:

| Variant (`fit 20`, single-core, load ~15) | p50 | vs baseline in the same window |
|---|---|---|
| baseline | 18.383 ms | - |
| only `th0`/`th1` rewritten, `Affine::rotate` kept | 17.552 ms | **-4.2%**, paired 8/9 `consistent` |
| full patch | 17.211 ms | **-6.4%**, paired 9/9 `consistent` |
| full patch vs the partial variant, head to head | 17.239 vs 17.539 ms | **-1.7%**, paired 9/9 `consistent` |

So roughly two thirds of the win comes from the two tangent angles and one third from the
`Affine::rotate` + `Affine::scale` removal; both parts are individually real.

### Deterministic instruction counts (`valgrind --tool=callgrind`)

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `fit 3` | 814,473,062 | 782,402,225 | **-3.94%** |
| `fit-opt 1` | 3,033,204,993 | 2,877,009,591 | **-5.15%** |
| `simplify 20` | 69,309,853 | 68,375,722 | **-1.35%** |

Both measurements agree on direction and rank the three scenes the same way. Per the project
notes, `Ir` under-reports this class of win (a libm call is one `call` instruction plus a
dependency stall), and it does so again here: -3.94% `Ir` against -7.1 to -8.9% wall clock on
`fit`. **The number leaned on is wall clock**, because all six primary comparisons were paired
8-9/9 `consistent` against a measured 0.1-1.5% noise floor; `Ir` is quoted as the load-immune
corroboration that the direction is not a scheduling artifact.

### Memory

Unchanged, as expected - the patch allocates nothing and adds no state.
`fit`: 1173.0 allocs/iter and 318 536 bytes/iter both before and after.
`simplify`: 39.0 allocs/iter and 769 312 bytes/iter both before and after.

## Result

A win of **-7.1% to -8.9% wall clock on `fit`**, **-6.9% to -9.4% on `fit-opt`** and **-1.3% to
-1.5% on `simplify`**, paired 8-9 of 9 in every comparison, corroborated by deterministic
instruction counts of -3.94% / -5.15% / -1.35%. Under a heavily loaded host (load ~15) the `fit`
win compresses to -6.4%, still 9/9. The dimensions measured are time and instruction count;
allocation count and bytes/iter were checked and are identical.

`simplify` gains far less than `fit` because `simplify_bezpath` spends most of its time in
`SimplifyBezPath`'s analytic `moment_integrals` and in flattening bookkeeping, with `fit_to_cubic`
a smaller slice.

## Does it change behavior/output?

- **This candidate has a real `patch.diff`** against the tested project's own source
  (`kurbo/src/fit.rs`, +37/-15). The additional `harness_fit_dump.diff` in this folder is *not*
  the fix - it only adds four untimed `--dump` targets to the workspace harness so the numbers
  below could be measured at all.
- **Bit-exact?** **No.** `CHECK` differs on exactly 3 of the 25 harness scenes - `fit`, `fit-opt`,
  `simplify` - and is bit-identical on the other 22 (`flatten`, `flatten-fine`, `flatten-small`,
  `mixed-flatten`, `stroke`, `stroke-miter`, `dash`, `offset`, `arclen`, `arclen-coarse`, `area`,
  `bbox`, `affine`, `svg-parse`, `svg-write`, `nearest`, `intersect`, `segments`, `solve-cubic`,
  `solve-quartic`, `deriv`, `subdivide`). That matches the call graph: inside kurbo, the only
  consumer of `fit.rs` is `simplify.rs`; `stroke`/`offset`/`dash` do **not** go through
  `fit_to_cubic` (they use `offset.rs`'s own subdivision), which is why they are untouched here
  even though the brief asked for them to be measured.
- **What changes, quantified.**

  | scene | output values | element counts | differing values | max abs difference | requested accuracy |
  |---|---|---|---|---|---|
  | `fit` (500 offset-curve sources) | 23 380 vs 23 380 | **identical for all 500 paths** | 12 377 (52.9%) | 2.68e-4 | 1e-3 |
  | `simplify` (flattened 50x8-cubic path) | 10 920 vs 10 920 | **identical, 5432 elements** | 9 (0.08%) | 4.55 | 1.0 |

  Both scenes keep the **structure** of the output: the same number of path elements per path, so
  the coordinate-wise diffs above are correctly aligned (unlike candidate 004's `stroke` case).
  The distribution matters more than the maximum:

  - `fit`: p50 diff 7.1e-15, p90 5.3e-13, p99 1.4e-11, p99.9 1.9e-10. Only **2 of 23 380**
    coordinates differ by more than 1e-6, and they are the x and y of a *single* control point of
    a single cubic (2.68e-4 and 1.39e-4), out of ~3 900 emitted elements. Everything else is
    last-ulp noise.
  - `simplify`: 9 differing values out of 10 920. Six are last-ulp to 6e-6; three belong to one
    `CurveTo`. In the baseline that segment is
    `p1 = (496.3716, 339.4095), p2 = p3 = (500.9191, 336.6567)`; in the patched build it is fully
    degenerate, `p1 = p2 = p3 = (500.9191, 336.6567)`, i.e. a straight line. The two emitted
    curves for that one segment deviate from each other by up to **1.29 units** (sampled symmetric
    Hausdorff distance) on a segment whose chord is 5.60 units, with the requested accuracy 1.0 -
    consistent with the "both within 1.0 of the source, hence within 2.0 of each other" bound.
    One other cubic's control point moved by 9.5e-3.

  Mechanism in both cases: `cubic_fit` builds a quartic whose roots are candidate control-point
  distances, and `fit_to_cubic` keeps whichever candidate has the smallest measured error. Near a
  degenerate segment two candidates have almost equal error, so a last-ulp change in `s0`/`c0`
  flips which one wins - and the two winners can be geometrically different curves even though
  both pass the same `err2 < accuracy^2` test. The baseline's choice is equally arbitrary; nothing
  in the algorithm prefers it.
- **Accuracy contract.** `fit_to_cubic` only returns a candidate after `CurveDist::eval_dist`
  measures its error and `err2 < acc2` holds, and `fit_to_bezpath` subdivides until that succeeds.
  Both builds therefore emit only segments within the requested accuracy of the source; that is
  by construction, not by measurement here.
- **Is this a breaking change?** No public API changes and no documented behavior changes
  (`cubic_fit` and `mod_2pi` are private; `fit_to_cubic`'s signature is untouched). It **is**
  observable to anyone byte-comparing stored output of `fit_to_bezpath`, `fit_to_bezpath_opt` or
  `simplify_bezpath` - and, per the `simplify` row above, not only as a last-ulp perturbation:
  one near-degenerate segment in that scene became a straight line instead of a small S-curve.
  That is the thing a maintainer has to weigh. It is also *more* accurate as an input: the old
  code formed `th0` by subtracting two `atan2` results, which loses precision by cancellation when
  the two angles are close, whereas `cross`/`dot` give the difference's sine and cosine directly.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after** (187 + 10 + 55 + 0, 4 `test result` lines both times, 1 ignored
  both times). `grep -cE '^error'` = 0 both times. Set difference of failing tests: **empty**
  (no test failed in either run). This includes `fit.rs`'s and `simplify.rs`'s own tests.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors, 0 warnings.
  The patch removes `atan2`/`sin_cos`/`round` uses from this file and adds only `sqrt` and
  `recip`, all already routed through `FloatFuncs` on that path.
- `cargo clippy --workspace --all-features`: 1 warning, the pre-existing
  `unneeded late initialization` in `kurbo/src/common.rs`. No new warnings. (`clippy::float_cmp`
  is allow-by-default and not in the Linebender lint set, so the `== Vec2::ZERO` comparison - an
  exact-zero test, which is what it is meant for - does not trip anything.)

## Why the result looks the way it does

`atan2` is one of the more expensive libm functions (argument reduction plus a rational
polynomial), and `sin_cos` is another; six such calls per `fit_to_cubic` were being made to
transport information that the input vectors already carried. Two `sqrt` instructions (~15-20
cycles, pipelined) plus two reciprocals replace all of them. The `fit` scene is dominated by
`fit_to_cubic` (it is `fit_to_bezpath`'s inner loop), so removing ~25% of that scene's libm self
time lands as a ~8% whole-scene win.

`fit-opt` gains slightly more than `fit` in instruction count (-5.15% vs -3.94%) because
`fit_to_bezpath_opt` calls `fit_to_cubic` many more times per output segment while searching for
optimal subdivision points - the per-call saving is amortized over less other work.

The `Affine` part is a third of the win on its own, which is more than it looks: it removes not
just a `sin_cos` but also the `chord2.sqrt()` that only existed to feed `Affine::scale`, and it
lets `aff` be built from values already in registers.

## What's not done / open questions

- **The one visibly different `simplify` segment is characterized but not adjudicated.** Both
  emitted curves satisfy the algorithm's own accuracy test; which is "better" was not measured
  (e.g. by flattening both and measuring deviation from the true source curve). A maintainer who
  cares about output stability across kurbo versions needs to look at that row specifically.
- `fit-opt`'s output difference was **not** characterized coordinate-wise - only its `CHECK` is
  known to differ. It uses the same `fit_to_cubic`, so the mechanism is identical, but the
  `--dump` targets added here cover `fit` and `simplify` only.
- The `libm`-feature build was checked for compilation but its **performance** was not measured,
  even though the benefit there should be larger than under `std` (six `libm`-crate calls removed
  from the hot path rather than six platform-libm ones).
- The same "`atan2(cross, dot)` followed by trig that undoes it" pattern is still untouched at
  `offset.rs:501`, `stroke.rs:492` and `expand.rs:324` (flagged by candidate 004, still open).
- Stacks with candidates 001/002/004/006/009; no combination was measured together.
- `unit_sin_cos` will return `NaN` if `|d| * |tangent|` underflows to a value whose square is 0
  (i.e. below ~1e-162), where the old `atan2` path was well defined. This is far outside any
  plausible geometry - `chord2 > accuracy^2` is already required to reach this code - but it is a
  strictly-narrower domain than before and was not exercised by a test.
