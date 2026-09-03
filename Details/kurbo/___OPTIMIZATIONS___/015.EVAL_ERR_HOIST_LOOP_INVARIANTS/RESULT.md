# 015. `CubicOffset::eval_err` - hoist the two loop-invariants out of the `0..N_LSE` loop

`CubicOffset::eval_err`'s `for i in 0..N_LSE` loop is the hottest loop in the library (62.7% of
self time in the `offset` scene, 55.0% in `stroke`, measured 2026-09-02). Two values inside its
body do not depend on `i`:

- `kurbo/src/offset.rs:380` - `let cusp = rec.cusp0.signum();`. `rec` is a `&OffsetRec` that never
  changes across iterations, so this recomputes the same `signum` eight times. A `signum` in a
  loop body is also an obstacle to reasoning about (or getting) auto-vectorization, so removing it
  is interesting beyond its own cost.
- `kurbo/src/offset.rs:371` - `let ta = (i + 1) as f64 * (1.0 / (N_LSE + 1) as f64);` recomputes a
  compile-time-constant reciprocal per iteration.

The question this candidate answers is whether LLVM already handles both, since a null result here
is what makes the surrounding vectorization question tractable.

## What was tried

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`,
`valgrind-3.26.0`.

`patch.diff` (+3/-2 lines) lifts `let cusp = rec.cusp0.signum();` above the loop and names the
reciprocal `const DTA: f64 = 1.0 / (N_LSE + 1) as f64;`, leaving `let ta = (i + 1) as f64 * DTA;`
in the body. Nothing else changes; the arithmetic and its order are untouched.

### Wall clock (`tools/paired.sh`, `RUNS=9`; harness `kurbo_bench <scene> 20`)

Host load average was 2.8-4.4 during the measurement window, with 5 sibling candidates building
and benchmarking concurrently behind a shared `flock`. The baseline's own repeat-run noise floor
was measured first, with no code change at all: `offset` 12.543 -> 12.463 ms single-core (0.6%),
`stroke` 11.796 -> 12.175 ms (3.2%), `dash` 7.532 -> 7.597 ms (0.9%). So ~3% is the resolvable
floor here, well above the effect this candidate could plausibly have.

| Variant | scene | single-core p50 (`taskset -c 2`) | multi-core p50 | paired verdict |
|---|---|---|---|---|
| baseline | `offset` | 12.458 ms | 12.492 ms | - |
| patched | `offset` | 12.458 ms (+0.0%) | 12.480 ms (-0.1%) | 4/9 and 3/9 - `NOT consistent` |
| baseline | `stroke` | 12.018 ms | 11.977 ms | - |
| patched | `stroke` | 11.976 ms (-0.4%) | 12.002 ms (+0.2%) | 5/9 and 3/9 - `NOT consistent` |
| baseline | `dash` | 7.532 ms | 7.576 ms | - |
| patched | `dash` | 7.579 ms (+0.6%) | 7.574 ms (-0.0%) | 5/9 and 5/9 - `NOT consistent` |

All six paired comparisons came back `NOT consistent`, i.e. the medians are explicitly not
evidence in either direction. `RUNS` was *not* raised to 15, because the disassembly comparison
below already settles the question deterministically - re-rolling the wall clock could only have
crossed a threshold by chance.

Allocation counts and peak RSS are unchanged by construction (this patch allocates nothing):
`offset` 0.0 allocs/iter both builds; `stroke` 25.1 allocs/iter, 1 361 928 bytes/iter both builds;
`dash` 25.1 allocs/iter, 1 839 608 bytes/iter both builds.

### Deterministic instruction counts (`valgrind --tool=callgrind`, `kurbo_bench <scene> 3`)

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `offset` | 743,181,171 | 743,181,267 | +96 (+0.000013%) |
| `stroke` | 742,515,007 | 742,515,001 | -6 (-0.0000008%) |
| `dash` | 462,407,661 | 462,407,333 | -328 (-0.00007%) |

Those deltas are three to four orders of magnitude smaller than any real effect measured in this
workspace so far (compare candidate 003's -3.02% / +1.44%). They are startup/environment jitter,
not a signal - which the disassembly explains.

### Disassembly comparison - this is the actual evidence

```
for b in bin_baseline bin_patched; do
  objdump -d --no-show-raw-insn ./$b \
    | sed 's/^ *[0-9a-f]*://; s/llvm\.[0-9]*/llvm.X/g' > ./$b.n.asm
done
diff ./bin_baseline.n.asm ./bin_patched.n.asm
```

The two binaries' entire `.text` differs by **exactly 4 lines**, and they are the same two
instructions appearing in two different places:

```
> 	unpcklpd %xmm10,%xmm10
> 	movapd %xmm10,0x210(%rsp)      # moved ~50 instructions earlier
< 	unpcklpd %xmm10,%xmm10
< 	movapd %xmm10,0x210(%rsp)
```

Both positions are inside the same straight-line prologue of the inlined `offset_rec` (the block
that broadcasts scalars into `xmm` lanes and spills them, ending at the `xor %eax,%eax` /
`nopl (%rax)` that starts the loop). The loop bodies are byte-identical, the instruction multiset
is identical, and nothing crosses a loop boundary. Before normalizing, the only other differences
are the `llvm.<hash>` suffixes of two anonymous constant symbols, which change because the source
file's content hash changed.

## Result

**No change, on any dimension.** Wall clock: six paired comparisons, all `NOT consistent`, all
medians within +0.6%/-0.4% of a baseline whose own repeat-run drift was up to 3.2%. Instruction
count: within 0.00007% on all three scenes. Allocation count and peak RSS: identical.

The reason is not that the effect is small - it is that **there is no effect**: rustc 1.98.0 /
LLVM already hoists both invariants out of the loop in the unpatched build, so the patched and
unpatched binaries contain the same machine code. The dimensions measured are time, instruction
count, allocation count and peak RSS; all four are null.

The `dash` scene was measured alongside `offset`/`stroke` per this workspace's standing rule
(candidates 003/005/012 improved the latter two while regressing `dash`). It is null here too,
which is the expected consequence of identical codegen rather than a separate finding.

## Does it change behavior/output?

- **Bit-exact?** **Yes.** `CHECK` compared across **all 25 harness scenes**: 25 identical,
  0 differing. This is guaranteed rather than lucky - hoisting a pure `signum` of an unchanging
  value and naming a constant cannot perturb a float result, and the multiply `(i + 1) as f64 * DTA`
  is textually the same operation on the same two values.
- **Is this a breaking change?** No. `eval_err` is a private method; no public API, documented
  behavior or output format changes.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after** (187 + 10 + 55 + 0), 4 `test result` lines both times,
  `grep -cE '^error'` = 0 both times.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors (the patch
  touches float math, so the `no_std` path was checked).
- `cargo clippy --workspace --all-features`: 1 warning, the pre-existing `unneeded late
  initialization` in `kurbo/src/common.rs`. No new warnings; the `const` in function scope does
  not trip `missing_docs`, and no cast was added.

## Why the result looks the way it does

`f64::signum` is `if self.is_nan() { NAN } else { 1.0_f64.copysign(self) }` - pure, side-effect
free, no memory access beyond the one load of `rec.cusp0`. That load is hoistable because `rec:
&OffsetRec` carries `noalias`/`readonly` and the only stores in the loop go through
`ts: &mut [f64; N_LSE]` and two local arrays, none of which LLVM has to assume alias `*rec`. So
LICM lifts the load and the `copysign` out unconditionally. The reciprocal is even simpler: `N_LSE`
is a `const usize`, so `1.0 / (N_LSE + 1) as f64` is folded to the literal `0.111...` at the IR
level regardless of where it is written.

This makes the patch a *readability* change with a proof attached (the 4-line disassembly diff),
not a performance change. Its practical value is negative-result value: it removes "the loop still
recomputes `signum` eight times" as a candidate explanation for why this loop does not vectorize.
The obstacle to vectorizing `eval_err` is elsewhere - the body is a chain of dependent scalar
`Point`/`Vec2` evaluations with a division (`tana.dot(pa - p) / tana.dot(...)`), a clamp against
`rec.t0`/`rec.t1`, a data-dependent `is_finite` fixup, and a `max` reduction, all per lane, with
`ts[i]` read-modify-written.

## What's not done / open questions

- **Auto-vectorizing the loop itself was not attempted.** With `signum` now known to be a
  non-issue, the remaining blockers are listed above; restructuring the loop into separate
  vectorizable passes (compute all eight `ta`/`pa`/`tana`, then all eight Newton steps, then the
  error reduction) would need scratch arrays and is a different, much larger candidate. Note
  candidate 009's warning: a rewrite that splits x and y into separate scalar passes can destroy
  the SIMD pairing LLVM already gets from `Vec2`'s two-lane layout, and lose wall clock while
  winning `Ir`.
- The patch is offered only if maintainers want the invariant made explicit in the source. It is
  measurement-neutral, so there is no performance argument for taking it - and no risk argument
  against it either, the codegen being provably identical.
- Only the `rustc 1.98.0` / LLVM in use here was checked. An older toolchain at kurbo's MSRV
  (1.85) might not hoist, which would make the patch a (small) win there; that was not measured.
- Whether `refine_least_squares`' own `for i in 0..N_LSE` loop (`offset.rs:428`) has
  loop-invariants LLVM misses was not examined - it reads only `err.unorms[i]`/`err.err_vecs[i]`
  and the `A_WEIGHTS`/`B_WEIGHTS` constant tables, so it looks unlikely.
