# simd_saturating_sub_min_negation_overflow

`simd_saturating_sub`'s signed branch (`src/intrinsic/simd.rs`, `generic_simd_intrinsic`'s
`(signed=true, is_add=false)` arm) implements `core::simd`'s signed saturating subtraction by
negating the right-hand operand and reusing the (separately correct) saturating-add
overflow-detection algorithm on `lhs + (-rhs)`. The code's own FIXME already flagged this as
unproven ("this uses the same algorithm from saturating add, but add the negative of the right
operand. Find a proper subtraction algorithm."). This candidate checks whether that FIXME's doubt
is founded, in the spirit of this workspace's "actively look for more instances of [the
GCC-declared-sign] family" note - it turned out to be a distinct but related bug (an overflow in
the negation used to reduce subtraction to addition, not a sign-mismatch), directly paralleling an
already-open `IDEAS.md` lead about the same "negate then saturating-add" shape in the x86 `_mm_subs_epi8`/`_mm_subs_epi16`
builtin path, but confirmed here in the target-independent `core::simd` intrinsic lowering instead.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning from the algorithm: hand-computed `saturating_sub(127i8, -128i8)` by tracing the exact GCC-level operations the code emits (`rhs = -(-128) = -128` due to two's-complement negation overflow, `sum = 127 + (-128) = -1` raw, overflow-check on that raw `sum` finds no overflow since -1 is representable) | Pen-and-paper, no build | Predicted a wrong result of `-1` where saturation to `127` (MAX) is correct - a distinct bug class from this workspace's usual "wrong GCC-declared sign" family: here the *value* fed into an otherwise-correct algorithm is corrupted by the negation of a MIN operand overflowing |
| Real end-to-end confirmation: minimal standalone crate using `core::simd`'s portable-SIMD `i8x4`/`i16x4`/`u8x4::saturating_sub`/`saturating_add`, built via `CHANNEL=release ./y.sh cargo build` against this checkout's real sysroot/backend, run natively | Release backend build (`./y.sh build --release`), debug repro binary, 3 runs | Exact predicted mismatch reproduced live: `i8x4::saturating_sub` on `[127, 0, -128, -1]` minus `[-128, -128, -128, 1]` returned `[-1, -128, -128, -2]` instead of the correct `[127, 127, 0, -2]` - 3/3 identical runs |
| Differential confirmation against real rustc/LLVM: same source compiled with the host's normal `rustc` (no `-Zcodegen-backend`) | Host nightly rustc, LLVM backend, 1 run | Printed/asserted the correct saturated values (`[127, 127, 0, -2]`) with no panic - confirms the GCC backend's output is the anomaly, not an error in the expected values |
| Final `repro/simd_saturating_sub.rs` compiled+run via the project's actual entry point (`./y.sh rustc`, matching this workspace's own convention from candidate 018), covering the MIN-operand case, a non-MIN control case, `saturating_add` as a sanity control, a wider (`i16`) element width, and unsigned `saturating_sub` as a control | Release backend build, 3 runs before the fix, 3 runs after | Before: 3/3 identical panics (exit 101) on the MIN-operand assertion, all other assertions passing. After: 3/3 identical clean exits (exit 0, all assertions pass) |

## Status

**Confirmed.** Reproduces 3/3 runs, deterministic, real release-mode build of the actual backend
(`./y.sh build --sysroot --release` then `CHANNEL=release ./y.sh rustc
repro/simd_saturating_sub.rs`), independently cross-checked against a real LLVM-backed rustc build
of the identical source producing the correct result.

**Fully reproducible**: Yes. `repro/simd_saturating_sub.rs` is a plain, public-API-only
(`core::simd`, nightly `portable_simd`) Rust source file compiled through the project's actual,
unmodified end-to-end entry point (`rustc -Zcodegen-backend=<real librustc_codegen_gcc.so>`,
wrapped by the project's own `./y.sh rustc`) - the real affected compiler backend reacting to a
real Rust program it is handed. This is a single-participant finding (a compiler compiling a
source file it's handed), so `CONFIRMATION.md` section 11's two-participant bar doesn't apply.

## Class

logic (a well-typed, safe `Simd<i8, N>::saturating_sub` call silently produces a wrong, non-
saturated numeric result whenever the right-hand operand is the element type's minimum value - no
crash, no compile-time diagnostic).

## Root cause

`generic_simd_intrinsic`'s signed `simd_saturating_sub` arm (`src/intrinsic/simd.rs`, the
`(true, false)` match arm) computes `lhs.saturating_sub(rhs)` by first negating `rhs`
(`UnaryOp::Minus`) and then reusing the signed `simd_saturating_add` arm's own overflow-detection
algorithm (a well-known, independently-correct two's-complement addition-overflow bit trick) on
`lhs + (-rhs)`.

This reduction is only mathematically valid when `-rhs` can be represented exactly. When `rhs`
equals the element type's minimum value (e.g. `i8::MIN = -128`), its true mathematical negation
(`128`) is not representable in the type, so GCC's `UnaryOp::Minus` performs a two's-complement
negation that silently wraps back to the same minimum value (`-(-128) == -128` in 8-bit two's
complement) instead of raising or signaling anything. The subsequent `sum = lhs + rhs` therefore
computes `lhs + rhs` (the *original*, un-negated `rhs`) rather than the intended `lhs - rhs` - a
different, wrong value - and the borrowed overflow-detection algorithm faithfully reports whether
*that* wrong addition overflowed, not whether the real subtraction did. For `lhs=127, rhs=-128`:
the wrong addition `127 + (-128) = -1` doesn't overflow (both `-1` and its inputs are ordinary
representable `i8` values), so the algorithm confidently returns the wrong, non-saturated `-1`
instead of the correct saturated `127`.

This is a distinct mechanism from this workspace's dominant "GCC-declared-signedness-vs-Rust-
semantic-signedness" family (candidates 004/006/018/etc.) - the GCC-level types here are already
correctly signed throughout; the bug is an unhandled overflow in the *negation* used to reduce one
operation to another, the same failure shape as the also-open (not yet independently fixed)
`IDEAS.md` lead for the x86-specific `_mm_subs_epi8`/`_mm_subs_epi16` SSE2 builtin path - this
candidate confirms the identical failure mode also exists, independently, in this project's own
target-agnostic `core::simd` Rust-level lowering, not only in whatever the bundled libgccjit's
x86 builtin does internally.

## Introduced by

d725cfb "Merge commit '08a6d6e16b5efe217123e780398969946266268f' into sync-cg_gcc-2023-03-04"
(2023-03-05) - the earliest commit in this repository's own tracked history containing this exact
"negate then reuse the saturating-add algorithm" shape and its own FIXME comment (a subtree-sync
merge from the upstream `rust-lang/rust` `cg_gcc` tree, so the code predates this merge in the
upstream tree's own history, but this is the point it entered this repository).

## The fix

Replaced the negate-then-add reduction with a direct, subtraction-specific overflow-detection
algorithm that never negates either operand: `diff = lhs - rhs` (two's-complement subtraction is
always well-defined/wraps, regardless of operand values - unlike negation of MIN), and adjusted the
overflow condition to subtraction's own rule (overflows when the operands have *different* signs
and the result's sign disagrees with `lhs`'s - the mirror image of addition's "same sign" rule,
so `xor1 & xor2` rather than `~xor1 & xor2`). The saturate-direction selection (`mask_min`, derived
from the sign of the raw result) is otherwise unchanged, just keyed off `diff` instead of the
former (corrupted) `sum`.

```
src/intrinsic/simd.rs
             (true, false) => {
                 let arg_type = lhs.get_type().unqualified();
-                // FIXME(antoyo): this uses the same algorithm from saturating add, but add the
-                // negative of the right operand. Find a proper subtraction algorithm.
-                let rhs = bx.context.new_unary_op(None, UnaryOp::Minus, arg_type, rhs);
-
-                // FIXME(antoyo): convert lhs and rhs to unsigned.
-                let sum = lhs + rhs;
+                let diff = lhs - rhs;
                 ...
                 let xor1 = lhs ^ rhs;
-                let xor2 = lhs ^ sum;
-                let and = !xor1 & xor2;
+                let xor2 = lhs ^ diff;
+                let and = xor1 & xor2;
                 let mask = and >> width;
                 ...
-                let shift2 = sum >> width;
+                let shift2 = diff >> width;
                 let mask_min = shift1 ^ shift2;
-                let and1 = !mask & sum;
+                let and1 = !mask & diff;
                 let and2 = mask & mask_min;
                 and1 + and2
             }
```

(See `patch.diff` for the exact diff including comments.)

Verified against `repro/simd_saturating_sub.rs`, which covers the MIN-operand case (`i8`, the
originally-observed failure), a non-MIN control case, `saturating_add` as a sanity control (its own
algorithm is unaffected by this bug and unchanged by the fix), a wider `i16` element width, and
unsigned `saturating_sub` as a control (a separate, unaffected code path). Before the fix: exit 101
(assertion failure on the MIN-operand case, `left: [-1, -128, -128, -2], right: [127, 127, 0,
-2]`), reproduced identically 3/3 runs. After the fix: exit 0, all assertions pass, 3/3 runs. See
`output.txt` for the actual captured output.

- Project test suite (`CHANNEL=release ./y.sh test --release --mini-tests` and `--cargo-tests`,
  which runs `tests/lang_tests.rs`'s `lang_tester`-driven `tests/run/*.rs` suite in both debug and
  release profiles, plus the mini-tests smoke test and cargo unit tests): **identical before and
  after the fix** - mini-tests smoke test passes both times; cargo-tests: 9 passed/0 failed (unit
  tests), 41 passed/0 failed/1 ignored (debug lang_tests), 41 passed/0 failed/1 ignored (release
  lang_tests), both before and after - no regressions, and (expectedly, per candidate 018's same
  observation) the existing suite doesn't happen to cover this specific SIMD intrinsic path at all,
  which is exactly why the bug went unnoticed since 2023 despite its own FIXME comment flagging the
  algorithm as unverified. `--test-libcore` was also run once against the fixed build: 578
  passed/0 failed.

## What's not done / open questions

- The sibling `IDEAS.md` lead (x86 `_mm_subs_epi8`/`_mm_subs_epi16` via
  `__builtin_ia32_psubsb128`/`__builtin_ia32_psubsw128`) was **not** independently re-checked this
  session - that path dispatches straight to a GCC/libgccjit builtin (`src/intrinsic/llvm.rs`),
  not through this `simd_saturating_sub` Rust-level lowering at all, so this candidate's fix does
  not touch it; whether that builtin has an analogous internal bug (this workspace's prior
  differential-testing session's own hypothesis, not yet verified by reading libgccjit's own
  source) remains open and should stay in `IDEAS.md` as its own, separate candidate.
- The mirror-image concern noted in the original FIXME ("convert lhs and rhs to unsigned") was not
  otherwise investigated - the fix in this candidate doesn't need it (subtraction of two signed
  values via GCC's native `-` is well-defined two's-complement wraparound regardless of the
  operands' declared sign, the same reasoning candidate 023 used to rule out an equivalent concern
  for `gcc_shl`), but a future session revisiting this code for unrelated reasons should not assume
  the FIXME was fully resolved by this patch, only the negation-overflow half of it.
- Only `i8`/`i16` element widths and 4-lane vectors were exercised; `i32`/`i64`/`i128` element
  widths were not separately checked, though the algorithm is width-generic (parameterized by
  `elem_width`) and the fix touches no width-specific logic, so a width-specific regression is
  considered unlikely but was not explicitly tested.
