# gcc_mul_hardcoded_signed_nonnative

`src/int.rs:210-212`: `gcc_mul` calls `multiplicative_operation(BinaryOp::Mult, "mul", true, a, b)`
with a literal `signed = true`, unconditionally, for both `i128 * i128` and `u128 * u128`. In the
non-native (array-based, `supports_128bit_integers = false`) branch of `multiplicative_operation`
(`src/int.rs:262-281`), this `signed` bool is string-formatted directly into an extern function
name (`__multi3` when `true`, `__umulti3` when `false`) that is called to perform the
multiplication. This looked suspicious by analogy to the same-file division/remainder path
(`division_operation`, which does thread a real signed/unsigned distinction into `__divti3` vs.
`__udivti3`) and to the already-confirmed atomic `fetch_max`/`fetch_min` sign bug (candidate 004)
and `type_uint_from_ty` sign bug (candidate 006) in this same codebase - both instances of a GCC-
level signedness decision not matching the Rust-level operand type. A backlog note already argued
this was likely fine (multiplication's low-N truncated result bits are sign-independent in two's
complement, and libgcc only ships `__multi3`, never `__umulti3`) - this candidate verifies that
claim rather than taking it on faith.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static code reading of `src/int.rs` (`gcc_mul`/`multiplicative_operation`), `src/context.rs` (`is_native_int_type`), and the system's installed `libgcc.a` (`nm /usr/lib/gcc/x86_64-linux-gnu/15/libgcc.a \| grep -i multi3`) | - | `libgcc.a` on this system exports `__multi3` but **no** `__umulti3` symbol at all. Mathematically, for two N-bit two's-complement operands, the low N bits of their product are identical whether the operands are interpreted as signed or unsigned (truncated multiplication mod 2^N doesn't depend on sign interpretation - this is the same reason a CPU's own integer multiply instruction only needs one opcode for the low half of the result, e.g. x86's `imul`/`mul` differ only in how they fill the *high* half). This means `__umulti3` is unnecessary by construction, and hardcoding `signed = true` (always emitting `__multi3`) is not just harmless but the only option that would ever link. |
| Manual boundary-value construction: a real Rust program (`repro/mul_boundary.rs`) exercising `i128::MIN.wrapping_mul(2)`, `i128::MIN.wrapping_mul(-1)`, `(-7i128).wrapping_mul(6)`, `u128::MAX.wrapping_mul(3)`, `u128::MAX.wrapping_mul(u128::MAX)`, `(1u128<<127).wrapping_mul(2)`, two MSB-set mixed-magnitude u128 products, `i128::MAX.wrapping_mul(i128::MAX)`, `i128::MIN.wrapping_mul(i128::MIN)`, and `u128::MAX.wrapping_mul(1u128<<127)` (all through `std::hint::black_box` to defeat constant folding), compiled and run three ways: (1) plain `rustc`/LLVM as ground truth, (2) this backend as-is on this x86_64 host (`supports_128bit_integers = true`, native GCC `__int128`, so `gcc_mul` takes the *native* branch, not the one under test), (3) this backend with `CodegenCx::new`'s `supports_128bit_integers` parameter forced to `false` (`repro/force_non_native.patch`, a 2-line diagnostic patch to `src/context.rs`, **not a fix** - applied and reverted transiently, same technique candidate 003 used for the analogous division/remainder question) to force every i128/u128 value onto the array-based representation and genuinely exercise `multiplicative_operation`'s `dyncast_array` branch on this same host, since no GCC target actually available in this environment lacks native 128-bit int support | Release build, `./y.sh rustc`, real captured stdout, see `output.txt` and `repro/run.sh` | All three runs produced bit-for-bit identical output across every boundary value, including the MSB-set u128 cases that would be most likely to expose a signed/unsigned truncation difference if one existed. `objdump -d` on the forced-non-native binary (step 3 of `repro/run.sh`) confirms 10 real calls to `__multi3` (the exact count of multiplications in the test program) and confirms no `__umulti3` symbol/reference exists anywhere in the binary - this is a real run of the exact code path the candidate targets, not a simulation. |
| (soft cap of 3 not needed - the first two approaches already converged on the same answer with no partial/ambiguous signal, so a third tool was not pursued) | - | - |

## Status

**Not reproducible.** Covered: `i128::MIN` (most negative value, MSB set) multiplied by both a
small positive and by -1, a small negative-times-positive product, `u128::MAX` multiplied by a
small value and by itself (both operands with every bit set), a power-of-two-times-2 case (tests
the exact boundary where the true product's bit 128 would need to be truncated away), two
MSB-set-in-both-operands u128 products with different magnitudes in each half, and the two
i128 extremes (`MAX*MAX`, `MIN*MIN`). All matched plain `rustc`/LLVM exactly, in both the
native-128-bit-int GCC path and the forced array-based (non-native) path. No sanitizer/fuzzing was
used - this is a codegen-correctness question (does the array-based path's fixed-signedness
dispatch produce the right bits), not a memory-safety one; boundary-value construction against a
differential (rustc/LLVM) oracle, per `HEURISTICS.md`'s "logic bugs" guidance, is what applies
here and is what was used - the same tool and the same reasoning that ruled out the sibling
division/remainder suspect in candidate 003.

**Fully reproducible**: N/A (nothing reproduced - this field only applies to `Confirmed` findings).

## Class

N/A (Not reproducible).

## Root cause

Not a bug: unlike division/remainder (where signed vs. unsigned division genuinely produce
different results, so `division_operation` correctly threads a real signed/unsigned distinction
into `__divti3`/`__udivti3`), N-bit-truncated multiplication is mathematically identical for
signed and unsigned operand interpretation - the low N bits of a product depend only on the
operands' bit patterns, not on how those bits are interpreted. `gcc_mul`'s `signed = true` literal
therefore always selects `__multi3`, and this is the *only* correct choice available: `libgcc`
(confirmed via `nm` on this system's installed `libgcc.a`) never ships an `__umulti3` symbol at
all, because none is needed - the same fact independently confirmed by disassembling this
candidate's forced-non-native repro binary and finding zero `__umulti3` references anywhere. Had
`gcc_mul` instead threaded a real per-call-site `signed` bool through to `u128 * u128` (mirroring
what `division_operation` does), the `u128` case would try to call a symbol that doesn't exist and
simply fail to link - so the current hardcoding isn't an oversight that happens to work, it's the
only option that works at all.

## Introduced by

N/A - not reproducible, no buggy commit to identify.

## The fix

No fix - nothing to fix. No `patch.diff` in this folder.

- Project test suite: N/A, no fix was applied. The one diagnostic patch used during the
  investigation (`repro/force_non_native.patch`) was applied to an isolated git worktree
  (`.worktrees/candidate-021`, not the main project tree) only transiently to force the code path
  under test, then reverted (`git checkout -- src/context.rs`) before this candidate's
  investigation ended; `git status` in that worktree showed a clean tree afterward, and the main
  project's own working tree was never touched by this candidate at all.

## What's not done / open questions

- This only checked `*` (the subject of the candidate). The sibling additive path
  (`gcc_add`/`gcc_sub` -> `additive_operation`'s `dyncast_array` branch, which derives `signed`
  from `a_type.is_compatible_with(self.i128_type)` rather than a caller-supplied literal, and
  *does* need a real signed/unsigned distinction since `__rust_i128_add`/`__rust_u128_add` are
  genuinely different operations at the overflow-detection level even though plain wraparound
  addition itself is also sign-independent at the bit level) was read but not independently
  boundary-tested here - already flagged as an open question in candidate 003's own "What's not
  done" section, not newly discovered by this candidate.
- Not tested here (same caveat already on record from candidate 003): 128-bit shifts and
  comparisons in the array-based path, which are a genuinely different mechanism (type-inspection
  via `a_type.is_signed(self)`, not a fixed literal) and so aren't covered by this candidate's
  reasoning either.
- As with candidate 003, the forced-non-native path was exercised on this x86_64 host by patching
  `CodegenCx::new` to ignore the real `supports_128bit_integers` query rather than by testing on an
  actual GCC target that lacks native 128-bit integers (none was available in this environment) -
  the generated array-based codegen itself should be identical either way (the same `int.rs` code
  paths run regardless of *why* `supports_128bit_integers` is `false`), but this wasn't
  cross-checked against a genuinely different target triple/GCC build.
