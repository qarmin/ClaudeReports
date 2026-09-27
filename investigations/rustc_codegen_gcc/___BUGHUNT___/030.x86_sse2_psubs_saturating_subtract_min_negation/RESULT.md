# x86_sse2_psubs_saturating_subtract_min_negation

Investigates a lead recorded in `IDEAS.md` from an external ~1700-test x86 intrinsic differential
suite (hodow), which found that `std::arch::x86_64::_mm_subs_epi8`/`_mm_subs_epi16` return wrong
values when an operand is `i8::MIN`/`i16::MIN`, and hypothesized this was a **separate** bug from
candidate 025 (`simd_saturating_sub_min_negation_overflow`), because these two intrinsics were
believed to dispatch straight through `src/intrinsic/llvm.rs`'s `get_target_builtin_function` to
libgccjit's own `__builtin_ia32_psubsb128`/`__builtin_ia32_psubsw128` builtins (`old_archs.rs`'s
`"sse2.psubs.b" => "__builtin_ia32_psubsb128"` mapping), a code path candidate 025's fix (to
`src/intrinsic/simd.rs`'s `simd_saturating_sub` lowering) does not touch.

**This turned out to be a false lead as stated**: the wrong output is real and reproduces
reliably, but it is not a separate bug - it is the exact same `simd_saturating_sub` defect
candidate 025 already found and fixed, reached via a different Rust API surface. See "Root cause"
below.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: grepped `src/intrinsic/llvm.rs` and `old_archs.rs`/`archs.rs` for `psubs` special-casing in `adjust_intrinsic_arguments`/`ignore_arg_cast` | Static read, no build | Confirmed no rustc_codegen_gcc-side argument handling exists for `"sse2.psubs.b"`/`"sse2.psubs.w"` - if reached, the call goes straight to `context.get_target_builtin_function(...)`, opaque to this crate |
| Minimal standalone C reproduction of `__builtin_ia32_psubsb128`/`__builtin_ia32_psubsw128` directly, compiled with the **system** GCC (15.2.0, Ubuntu), at `-O0`/`-O1`/`-O2`/`-O3`, with the exact operand pattern from the failing hodow tests (`[127,0,-128,-1]` minus `[-128,-128,-128,1]`) | `gcc -O{0,1,2,3} -msse2 test2.c` | Produced the **correct** saturated result (`[127,127,0,-2]`) at every optimization level - no evidence the builtin itself is broken in a current GCC |
| Standalone JIT harness (own `Cargo.toml` crate, real `gccjit` 6.1.0 crate, `dlopen`-loading the actual bundled `libgccjit.so` badf78d0 / GCC 13.3.0 that this project's `./y.sh build --sysroot` downloads) calling `Context::get_target_builtin_function("__builtin_ia32_psubsb128")` directly, mirroring `llvm.rs`'s own call pattern | Release build, `LIBGCCJIT_PATH`/`LD_LIBRARY_PATH` pointed at the project's own downloaded `libgccjit.so` | Dead end, not further pursued: `get_target_builtin_function` returned `NULL` with libgccjit's own diagnostic `"Cannot find target builtin __builtin_ia32_psubsb128"`, even after adding the same `-mavx`/`-masm=intel` command-line options `src/gcc_util.rs`'s `new_context` adds in production. Not resolved (likely some additional per-context setup step this minimal harness is missing) - abandoned once the real end-to-end repro below made it unnecessary, and moot once code tracing (next row) showed this code path isn't reached by the actual failing intrinsics anyway |
| **Real end-to-end repro** (`./y.sh rustc`, the project's own actual compiler entry point) of `_mm_subs_epi8`/`_mm_subs_epi16` with `black_box`-wrapped `i8::MIN`/`i16::MIN` operands, in a dedicated git worktree (`/tmp/bughunt_worktrees/030`, cloned from this repo, own downloaded libgccjit/sysroot reused via symlink per this workspace's `README.md`) | Release backend + sysroot build, 3 runs before fix, 3 runs after | Reproduced the wrong output 3/3 identical runs before any fix (see below) - and, critically, source-tracing this reproduction's actual compiled code path (next row) redirected the whole investigation |
| Source tracing: grepped the pinned toolchain's own bundled stdarch (`build/build_sysroot/sysroot_src/library/stdarch/crates/core_arch/src/x86/sse2.rs`) for how `_mm_subs_epi8`/`_mm_subs_epi16` are actually implemented, and grepped the whole `stdarch` tree for any remaining reference to `"llvm.x86.sse2.psubs"` | Static read of the sysroot source actually in use | **`_mm_subs_epi8`/`_mm_subs_epi16` are implemented as `simd_saturating_sub(a.as_i8x16(), b.as_i8x16())`/`as_i16x8()` - the portable, target-independent Rust intrinsic, not a raw `llvm.x86.sse2.psubs.b`/`.w` call.** A repo-wide grep of `stdarch` for `"sse2.psubs"`/`"x86.sse2.psubs"` found zero matches anywhere - the `old_archs.rs` dispatch entry that maps `"sse2.psubs.b"` to `"__builtin_ia32_psubsb128"` is unreachable dead code for these two intrinsics on the pinned nightly-2026-08-04 toolchain (this doesn't rule out some other, still-`llvm.x86.*`-calling code path reaching it - not investigated) |
| Confirmation that this is the *same* bug as candidate 025: applied candidate 025's own `patch.diff` (`src/intrinsic/simd.rs`'s signed `simd_saturating_sub` arm) to this candidate's own worktree, rebuilt, reran the exact same `_mm_subs_epi8`/`_mm_subs_epi16` repro | Release rebuild (~2s, backend-only), 3 runs | Repro's assertions **all pass** (exit 0) - candidate 025's fix, aimed at `core::simd::Simd::saturating_sub`, independently fixes `_mm_subs_epi8`/`_mm_subs_epi16` too, since they compile to the exact same `simd_saturating_sub` code in `src/intrinsic/simd.rs` |
| Differential confirmation against real rustc/LLVM (host nightly rustc, no `-Zcodegen-backend`) on the identical repro source | 1 run | Printed `all assertions passed`, exit 0 - confirms the expected values are correct and only the unpatched GCC backend disagrees |

## Status

**Confirmed** - the wrong end-to-end output for `_mm_subs_epi8`/`_mm_subs_epi16` with a
`i8::MIN`/`i16::MIN` operand is real and reproduces reliably (3/3 identical runs, real release
build of the actual backend via `./y.sh rustc`). However, root-causing it disproves the
**specific, distinct** mechanism this candidate set out to check (a separate libgccjit-builtin
bug reached via `src/intrinsic/llvm.rs`'s `get_target_builtin_function`) - see "Root cause".

**Fully reproducible**: Yes. `repro/x86_subs_min.rs` is a plain, public-API-only
(`std::arch::x86_64`) Rust source file compiled through the project's actual, unmodified
end-to-end entry point (`rustc -Zcodegen-backend=<real librustc_codegen_gcc.so>`, wrapped by
`./y.sh rustc`) - a single-participant finding (a compiler compiling a source file it's handed),
so `CONFIRMATION.md` section 11's two-participant bar doesn't apply.

## Class

logic (a well-typed, safe `_mm_subs_epi8`/`_mm_subs_epi16` call silently produces a wrong,
non-saturated numeric result whenever an operand is the element type's minimum value - no crash,
no compile-time diagnostic).

## Root cause

**This is not a new/separate bug - it is candidate 025's already-recorded defect, reached through
a different Rust API.** On the toolchain this project currently pins (`nightly-2026-08-04`),
`std::arch::x86_64::_mm_subs_epi8`/`_mm_subs_epi16` (in the bundled stdarch's
`core_arch/src/x86/sse2.rs`) are implemented as:

```rust
pub const fn _mm_subs_epi8(a: __m128i, b: __m128i) -> __m128i {
    unsafe { transmute(simd_saturating_sub(a.as_i8x16(), b.as_i8x16())) }
}
```

i.e. they call the portable `simd_saturating_sub` Rust intrinsic directly, not a raw
`llvm.x86.sse2.psubs.b`/`.w` call. That portable intrinsic is lowered by
`src/intrinsic/simd.rs`'s `generic_simd_intrinsic`, whose signed `(true, false)` arm (subtraction)
is exactly the bug candidate 025 already found, root-caused, and fixed: it computes
`lhs.saturating_sub(rhs)` by negating `rhs` and reusing the saturating-add overflow-detection
algorithm on `lhs + (-rhs)`, and negating `rhs` overflows (wraps back to the same minimum value)
whenever `rhs` is the element type's minimum value, corrupting the sum the overflow check is based
on.

The mechanism this candidate was created to check - `_mm_subs_epi8`/`_mm_subs_epi16` dispatching
straight to libgccjit's own `__builtin_ia32_psubsb128`/`__builtin_ia32_psubsw128` target builtins
via `src/intrinsic/llvm.rs`'s `get_target_builtin_function`, with the corruption happening inside
libgccjit's own (opaque, not part of this crate) implementation of that builtin - **does not
apply**: a repo-wide grep of the pinned toolchain's own bundled `stdarch` source tree for
`"sse2.psubs"`/`"x86.sse2.psubs"` found no remaining reference anywhere. The `old_archs.rs`
dispatch table entry mapping `"sse2.psubs.b"`/`"sse2.psubs.w"` to
`"__builtin_ia32_psubsb128"`/`"__builtin_ia32_psubsw128"` is unreachable dead code for these two
intrinsics on this toolchain (it may still be reachable through some other still-`llvm.x86.*`-
calling code path this session did not check - see "What's not done" below). Two independent
pieces of evidence corroborate that libgccjit's own builtin is *not* the culprit here:

1. A minimal standalone C reproduction calling `__builtin_ia32_psubsb128`/`__builtin_ia32_psubsw128`
   directly, compiled with a current system GCC (15.2.0), produces the **correct** saturated
   result at every optimization level for the exact operand pattern that triggers the wrong
   output through this backend.
2. Applying candidate 025's own `patch.diff` (touching only `src/intrinsic/simd.rs`, nothing in
   `llvm.rs`/`old_archs.rs`) to this candidate's own reproduction **fixes it completely** - if the
   corruption were happening inside libgccjit's own builtin (a code path candidate 025's patch
   never touches), this fix would have had no effect on `_mm_subs_epi8`/`_mm_subs_epi16`.

## Introduced by

Same commit as candidate 025 (same underlying source defect): d725cfb "Merge commit
'08a6d6e16b5efe217123e780398969946266268f' into sync-cg_gcc-2023-03-04" (2023-03-05) - the
earliest commit in this repository's own tracked history containing the "negate then reuse the
saturating-add algorithm" shape and its own FIXME comment (a subtree-sync merge from the upstream
`rust-lang/rust` `cg_gcc` tree, so the code predates this merge in the upstream tree's own
history, but this is the point it entered this repository).

## The fix

Identical to candidate 025's fix (`___BUGHUNT___/025.simd_saturating_sub_min_negation_overflow/patch.diff`),
included again here (`patch.diff`) purely so this folder is self-contained - it is **not** a new,
independently-derived patch. It replaces the negate-then-add reduction in `src/intrinsic/simd.rs`'s
signed `simd_saturating_sub` arm with a direct subtraction-based overflow-detection algorithm that
never negates either operand (`diff = lhs - rhs`, with the overflow condition adjusted to
subtraction's own rule).

Verified against `repro/x86_subs_min.rs` (covering `_mm_subs_epi8` with an `i8::MIN` operand,
`_mm_subs_epi16` with an `i16::MIN` operand, and `_mm_subs_epu8` as an unsigned control) in a
dedicated worktree (`/tmp/bughunt_worktrees/030`, not the main project tree):

- Before fix: exit 101, `assertion left == right failed: _mm_subs_epi8 mismatch (operand includes
  i8::MIN)`, `left: [-1, -128, -128, -2]`, `right: [127, 127, 0, -2]` - reproduced identically
  3/3 runs.
- After fix: exit 0, `all assertions passed` - reproduced identically 3/3 runs.
- Differential check: the identical source compiled with the host's real nightly rustc (LLVM
  backend, no `-Zcodegen-backend`) prints `all assertions passed` / exit 0, confirming the
  expected values are correct.

See `output.txt` for the actual captured output of all of the above.

Project test suite (`CHANNEL=release ./y.sh test --release --mini-tests --cargo-tests`, run in
the same dedicated worktree, **before and after** applying the patch): **identical before and
after** - cargo unit tests: 0 passed/0 failed (no unit tests in this package); mini-tests smoke
test: 9 passed/0 failed; debug lang_tests: 41 passed/0 failed/1 ignored; release lang_tests: 41
passed/0 failed/1 ignored - both before and after, no regressions. As with candidate 025, the
existing suite doesn't happen to cover this SIMD intrinsic path at all (the specific
`_mm_subs_epi8`/`_mm_subs_epi16`-with-MIN-operand repro used for this candidate was kept in a
separate `repro/` file compiled directly via `./y.sh rustc`, deliberately **not** added to
`tests/run/`, so it does not itself become part of the "before/after identical" count above -
matching candidate 025's own convention).

## What's not done / open questions

- Whether the `old_archs.rs` `"sse2.psubs.b"`/`"sse2.psubs.w"` → `__builtin_ia32_psubsb128`/
  `__builtin_ia32_psubsw128` dispatch table entries are reachable through *any* currently-live
  code path (not just the two `_mm_subs_epi8`/`_mm_subs_epi16` intrinsics checked here) was not
  exhaustively verified - only a grep of the pinned toolchain's bundled `stdarch` source tree for
  the literal LLVM intrinsic name was done. If some other caller does still reach this exact
  builtin dispatch, whether libgccjit's own implementation is actually correct for it remains
  supported only by the system-GCC 15.2 C reproduction in this candidate (a different, newer GCC
  major version than the bundled libgccjit 13.3.0) - not tested against the exact bundled
  libgccjit.so itself, since the standalone JIT harness built for that purpose hit an unresolved
  `"Cannot find target builtin"` setup issue (see "What was tried") before this line of
  investigation was abandoned as moot.
- The standalone JIT harness's `get_target_builtin_function` setup issue itself was not
  root-caused (kept as project-agnostic tooling knowledge, not a candidate finding) - a future
  session wanting to directly test other `get_target_builtin_function`-dispatched x86 builtins
  against the exact bundled libgccjit.so via a minimal `gccjit`-crate harness (rather than the
  full `./y.sh rustc` pipeline) will need to resolve why the builtin lookup fails even after
  adding the same `-mavx`/`-masm=intel` command-line options `src/gcc_util.rs` adds in production.
