# rdrand_rdseed_unimplemented_llvm_intrinsic_ice

Compiling ordinary safe-surface uses of `core::arch::x86_64::_rdrand16_step`/`_rdrand32_step`/
`_rdseed16_step`/`_rdseed32_step`/`_rdseed64_step` with `-C target-feature=+rdrand,+rdseed` (the
same feature gate `-C target-cpu=native` turns on for any CPU that supports these instructions)
panics rustc_codegen_gcc with `not implemented: ***** unsupported LLVM intrinsic
llvm.x86.rdrand.16` (or the `.32`/`rdseed.*` siblings) inside `src/intrinsic/archs.rs`'s generated
`map_arch_intrinsic` catch-all table - an ICE, not a normal diagnostic, on code a plain rustc/LLVM
build compiles and runs without any issue. Only `llvm.x86.rdrand.64` had an explicit special case
in `src/intrinsic/llvm.rs`; its five siblings did not.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: grepped `src/intrinsic/llvm.rs`/`archs.rs` for `rdrand`/`rdseed` | Static read, no build | Found exactly one explicit mapping (`"llvm.x86.rdrand.64" => "__builtin_ia32_rdrand64_step"`, plus matching arg/return-value special-casing) and zero entries for `rdrand.16`/`rdrand.32`/`rdseed.16`/`rdseed.32`/`rdseed.64` anywhere in the generated `archs.rs` table - confirming these five fall into the file's blanket `_ => unimplemented!(...)` arm |
| Real end-to-end repro (`rustc -Zcodegen-backend=<real librustc_codegen_gcc.so>`, this project's actual compiler entry point, release build) compiling a small program calling all six `_rdrand*_step`/`_rdseed*_step` functions with `-C target-feature=+rdrand,+rdseed`, in a dedicated git worktree (`___BUGHUNT___/wt/031`, cloned from this repo's own working tree, downloaded libgccjit/sysroot reused via symlink per this workspace's `README.md`) | Release backend + sysroot build, 3 compile attempts before any fix | Compiler panicked identically 3/3 times: `thread 'rustc' panicked at src/intrinsic/archs.rs:9097:26: not implemented: ***** unsupported LLVM intrinsic llvm.x86.rdrand.16`, exit 101 (ICE) |
| Differential check against real rustc/LLVM (host nightly rustc, no `-Z codegen-backend`) on the identical source | 1 compile + run | Compiled and ran cleanly, printed `all rdrand/rdseed steps completed without an ICE`, exit 0 - confirms this is safe, ordinary code a real rustc/LLVM build handles fine, not a case where the input itself is somehow invalid |
| Read the pinned toolchain's own bundled stdarch (`core_arch/src/x86/rdrand.rs`, `core_arch/src/x86_64/rdrand.rs`) to enumerate exactly which `llvm.x86.rdrand*`/`llvm.x86.rdseed*` intrinsic names the public API can actually reach | Static read | Six names total: `llvm.x86.{rdrand,rdseed}.{16,32}` (both in `x86/rdrand.rs`, reachable on any target) and `llvm.x86.{rdrand,rdseed}.64` (`x86_64/rdrand.rs`, x86_64-only) - `rdrand.64` already had an explicit mapping, the other five did not |
| Applied a bounded fix (see below), rebuilt, reran the same repro 3x | Release rebuild (~9-11s, backend-only) | All six `_rdrand*_step`/`_rdseed*_step` calls now compile and run correctly, reporting sane `0`/`1` success flags, matching the LLVM-backend reference run - reproduced identically 3/3 runs |

## Status

**Confirmed** - reproduces deterministically 3/3 runs, real release build of the actual backend,
compiling through the project's real entry point (`rustc -Zcodegen-backend=<real
librustc_codegen_gcc.so>`).

**Fully reproducible**: Yes. `repro/rdrand_rdseed.rs` is a plain, public-API-only
(`std::arch::x86_64`) Rust source file compiled through the project's actual, unmodified
end-to-end entry point - a single-participant finding (a compiler compiling a source file it's
handed), so `CONFIRMATION.md` section 11's two-participant bar doesn't apply.

## Class

crash (a compile-time ICE - `unimplemented!()` panic inside the backend - on well-typed, safe
surface code that a plain rustc/LLVM build accepts and runs correctly; no memory unsafety
involved).

## Root cause

`src/intrinsic/llvm.rs`'s `intrinsic()` function (the `#[cfg(feature = "master")]` variant, which
is the one actually used since `master` is a default Cargo feature) maps LLVM intrinsic names to
libgccjit target-builtin names through a large explicit `match`, falling back to the
auto-generated `map_arch_intrinsic()` table in `src/intrinsic/archs.rs` for anything not
explicitly listed. That generated table's `x86` arm ends in a blanket `_ => unimplemented!("*****
unsupported LLVM intrinsic {full_name}")` for any x86 LLVM intrinsic name it doesn't recognize -
`archs.rs` was mechanically generated from a fixed external list (llvmint) and was never meant to
cover every builtin GCC itself supports.

`llvm.x86.rdrand.64` only works today because commit 5ab4e2b ("Implement llvm.x86.rdrand.64",
2023-09-18) added it as an explicit override, together with matching special-case code in
`adjust_intrinsic_arguments`/`adjust_intrinsic_return_value` (GCC's `__builtin_ia32_rdrand64_step`
returns the random value through an out-parameter and the 32-bit "success" flag as its own return
value, whereas the LLVM/Rust-visible `x86_rdrand64_step` intrinsic returns both packed into a
`(u64, i32)` tuple value - the codegen needs to synthesize a local, take its address, pass it as
the hidden out-parameter, then repack the result into a tuple after the call). That 2023 fix
covered only the one variant it was written for; the other five members of the same intrinsic
family (`rdrand.16`, `rdrand.32`, `rdseed.16`, `rdseed.32`, `rdseed.64`) - which need the exact
same treatment, just with a 16-bit/32-bit/64-bit out-parameter type and (for `rdseed`) a
differently-named GCC builtin (`__builtin_ia32_rdseed_{hi,si,di}_step` instead of
`__builtin_ia32_rdrand{16,32,64}_step` - GCC's own naming is inconsistent between the two
instruction families) - were simply never added, so any program calling them hits `archs.rs`'s
generic catch-all instead.

## Introduced by

68ac3a4 "Generate all listed architectures from llvmint" (2022-03-30) - the commit that first
generated `archs.rs`'s blanket `unimplemented!()` catch-all for every x86 LLVM intrinsic not given
an explicit override, which is what these five intrinsic names have always fallen into since. The
later commit 5ab4e2b "Implement llvm.x86.rdrand.64" (2023-09-18) added an explicit override for
the sibling `rdrand.64` intrinsic but never extended the same treatment to `rdrand.16`/
`rdrand.32`/`rdseed.16`/`rdseed.32`/`rdseed.64` - so the gap for those five has existed,
unchanged, since 68ac3a4.

## The fix

Added the same "take a local variable's address as the hidden out-parameter, then repack the
result into a tuple after the call" special-casing `llvm.x86.rdrand.64` already had, generalized to
all five missing variants, plus the five missing name-mapping table entries in `intrinsic()`
(`src/intrinsic/llvm.rs`):

- `adjust_intrinsic_arguments`: added `"__builtin_ia32_rdrand32_step" | "__builtin_ia32_rdseed_si_step"`
  (declares a `uint_type` local) and `"__builtin_ia32_rdrand16_step" | "__builtin_ia32_rdseed_hi_step"`
  (declares a `ushort_type` local) arms alongside the existing `"__builtin_ia32_rdrand64_step"` arm
  (extended to also cover `"__builtin_ia32_rdseed_di_step"`, which needs the identical
  `ulonglong_type` treatment).
- `adjust_intrinsic_return_value`: extended the existing `"__builtin_ia32_rdrand64_step"` match arm
  to also cover `"__builtin_ia32_rdrand32_step"`, `"__builtin_ia32_rdrand16_step"`,
  `"__builtin_ia32_rdseed_di_step"`, `"__builtin_ia32_rdseed_si_step"`, and
  `"__builtin_ia32_rdseed_hi_step"` - the existing body is already type-agnostic (it reads the
  pointee type off the argument itself), so no new logic was needed there, just widening the match.
- `intrinsic()`'s name-mapping table: added `"llvm.x86.rdrand.32"`, `"llvm.x86.rdrand.16"`,
  `"llvm.x86.rdseed.64"`, `"llvm.x86.rdseed.32"`, `"llvm.x86.rdseed.16"`, mapping to
  `"__builtin_ia32_rdrand32_step"`, `"__builtin_ia32_rdrand16_step"`,
  `"__builtin_ia32_rdseed_di_step"`, `"__builtin_ia32_rdseed_si_step"`,
  `"__builtin_ia32_rdseed_hi_step"` respectively (GCC's own builtin naming: `rdrand{16,32,64}_step`
  but `rdseed_{hi,si,di}_step` - verified against this machine's installed GCC 15
  `x86gprintrin.h`/`rdseedintrin.h` headers).

See `patch.diff` (+25/-2 lines, `src/intrinsic/llvm.rs` only).

Verified against `repro/rdrand_rdseed.rs` (calls all six `_rdrand16_step`/`_rdrand32_step`/
`_rdrand64_step`/`_rdseed16_step`/`_rdseed32_step`/`_rdseed64_step` functions) in the dedicated
worktree used for this investigation, both directly (`rustc -Zcodegen-backend=...`) and through the
project's own `./y.sh rustc` wrapper:

- Before fix: ICE, `not implemented: ***** unsupported LLVM intrinsic llvm.x86.rdrand.16`, exit
  101 - reproduced identically 3/3 runs.
- After fix: compiles cleanly, runs and prints `all rdrand/rdseed steps completed without an ICE`,
  exit 0 - reproduced identically 3/3 runs.
- Differential check: the identical source compiled with the host's real nightly rustc (LLVM
  backend, no `-Z codegen-backend`) also prints the same success message and exits 0.

See `output.txt` for the actual captured output of all of the above.

Project test suite (`CHANNEL=release ./y.sh test --release --mini-tests --test-libcore
--cargo-tests`, run in the same dedicated worktree, **before and after** applying the patch):
identical before and after - libcore tests: 2807 passed/0 failed/2 ignored; cargo-tests: 578
passed/0 failed; two 0/0 subcrate cargo-test runs; mini-tests smoke test: 9 passed/0 failed; debug
lang_tests: 41 passed/0 failed/1 ignored; release lang_tests: 41 passed/0 failed/1 ignored - no
regressions and no new failures either way (the existing suite doesn't happen to exercise
`rdrand`/`rdseed` at all, which is exactly the gap this candidate's `repro/` was written to cover).

## What's not done / open questions

- Only the six `rdrand`/`rdseed` variants that this project's pinned toolchain's own `stdarch` can
  actually reach from safe/public API were checked and fixed. The same class of gap (an x86 LLVM
  intrinsic falling into `archs.rs`'s blanket `unimplemented!()` catch-all with no override in
  `src/intrinsic/llvm.rs`) very likely affects other, unrelated intrinsic families too - this
  candidate deliberately scoped itself to the specific family named in its own task description
  and did not attempt the broader systematic sweep across all of `archs.rs`'s remaining
  `unimplemented!()` arms that the task description's own last sentence suggested might be
  worthwhile; that sweep is recorded as a new backlog idea rather than attempted here.
- Whether GCC's `__builtin_ia32_rdrand{16,32}_step`/`__builtin_ia32_rdseed_{hi,si,di}_step`
  produce bit-for-bit identical *values* to LLVM's lowering (as opposed to merely "compiles and
  returns a plausible 0/1 success flag") was not checked beyond the success-flag sanity assertions
  in `repro/rdrand_rdseed.rs` - true hardware RNG output isn't a meaningful thing to assert exact
  values for either way, this is consistent with how `_rdrand64_step` (already working before this
  fix) is exercised elsewhere in this workspace.
