# float_to_u128_i128_cast_saturating_zero_bug

Issue #690 reports that a runtime-computed `f32` that evaluates to `+inf` (via a chain of
multiplications, not a literal), cast `as u128`, sometimes prints `0` instead of the correct
saturating value `u128::MAX`. The candidate hypothesis was a bug in `Builder::fptoint_sat`
(`src/builder.rs:1925`) or the raw `gcc_float_to_int_cast` step it calls into, possibly the same
GCC-optimizer-folds-a-black_box-guarded-comparison class of miscompile confirmed for candidate
010 (`u128_as_f32_cast_broken_release_mode`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Exact original repro from issue #690 (runtime-computed `f32` -> `+inf` -> `as u128`) and FractalFir's minimized reduction (separate `#[inline(never)]` `compute()` fn), both compiled through the real `./y.sh rustc -O` cg_gcc pipeline, vs. host rustc/LLVM for comparison | `CHANNEL=release ./y.sh build --release`; release build of the backend; single run each | Both print the correct saturating value `u128::MAX` on this checkout's pinned libgccjit (badf78d09d16e66f4ca07971c51aa6a227558d4f). The issue's reported `0` does not reproduce end-to-end. |
| Differential boundary/seed sweep (`repro/boundary_sweep.rs`), 12 seeds (incl. `NaN`, `+-inf`, and the exact issue-690 magnitudes) through the same `compute()` shape, casting the result to `u128`/`i128`/`u64`/`i64`/`u32`, cg_gcc vs. host rustc/LLVM | Same release build | Byte-for-byte identical output between cg_gcc and host rustc/LLVM across all 12 seeds and all 5 integer types - no difference. |
| Manual boundary-value construction reproducing the underlying GCC VRP bug class directly in C (`repro/gcc_vrp_bug_demo.c`), compiled with a plain system GCC (unrelated to this project's pinned libgccjit fork) | `gcc -O0` vs. `gcc -O2`, GCC 15.2.0 (Ubuntu 15.2.0-16ubuntu1) | The underlying bug class (VRP wrongly folding a runtime inf-derived comparison to a constant at `-O2`) **is** reproducible in a generic, unrelated system GCC - confirms this is a real GCC bug class, just not one that currently manifests through this project's pinned libgccjit fork snapshot. |
| (Resumed session) Reproduced candidate 010's exact triggering code shape - two consecutive `black_box`-guarded casts compared via an inline `assert_eq!` macro argument - applied to this candidate's own bug class instead of `u128 as f32`: (a) two identical `black_box(f32::INFINITY) as u128 == u128::MAX` asserts back to back (`repro/assert_eq_black_box_consecutive_u128.rs`), (b) a finite non-saturating boundary cast (`1e38f32 as i128`) immediately followed by the saturating `black_box(f32::INFINITY) as i128 == i128::MAX` case, mirroring candidate 010's own finite-then-infinite pairing (`repro/assert_eq_black_box_boundary_i128.rs`), (c) a chain of 7 consecutive inline `assert_eq!` saturating casts covering `i128`/`u64`/`i64`/`u32`/`f64`-sourced `u128`/`i128` (`repro/assert_eq_black_box_all_int_types.rs`) | Same release build, `CHANNEL=release ./y.sh rustc -O` | All three print `ok` (all asserts pass, no panic, no wrong value) - candidate 010's specific triggering shape does not reproduce this candidate's bug class either. |

## Status

**Not reproducible.** Covered: (1) the exact issue-690 repro and a minimized reduction through
the real `./y.sh rustc -O` cg_gcc pipeline; (2) a 12-seed differential boundary sweep across
`u128`/`i128`/`u64`/`i64`/`u32` against host rustc/LLVM, byte-for-byte identical; (3) candidate
010's own exact triggering code shape (two consecutive `black_box`-guarded casts compared via an
inline `assert_eq!` argument, including the specific finite-then-infinite pairing that triggered
010) transplanted onto this candidate's float-to-integer saturating-cast bug class, in three
variants, all passing. Four escalating approaches total (one beyond the usual soft cap of 3,
specifically to rule out the candidate-010 code-shape lead before finalizing), all negative, none
flaky across repeated runs.

## Class

N/A - not reproduced.

## Root cause

Not applicable to this backend/checkout as it stands: the underlying GCC optimizer bug class this
issue describes (VRP/range-propagation wrongly folding a runtime-computed comparison against a
value that reached `+inf` at `-O2`) is real and independently reproducible in a plain, unrelated
system GCC 15.2.0 (`repro/gcc_vrp_bug_demo.c`), but is not currently reachable through this
project's pinned libgccjit fork snapshot (`badf78d09d16e66f4ca07971c51aa6a227558d4f`, per
`libgccjit.version`) for any of the code shapes tried, including the one that did reproduce a
related but distinct miscompile in candidate 010 (a GCC extern-call path via
`__floatuntisf`/`is_native_int_type == false`, for `u128 as f32` specifically, not the saturating
`f32 as u128` direction this issue is about). It is plausible the issue-690 report was observed
against a different (older, or differently-configured) libgccjit build than the one currently
pinned in this checkout, or depended on a GCC optimization pass ordering/inlining decision that
this specific pinned snapshot no longer takes for these particular code shapes.

## Introduced by

N/A - not reproduced, so no introducing commit to identify in this repository. (If the underlying
GCC-side bug class is later confirmed to reproduce again against a different libgccjit snapshot,
the relevant "introducing" change would be in the vendored `rust-lang/gcc` fork's own history, not
this repository's, per the same reasoning as candidate 010's own "Introduced by" section.)

## The fix

Not applicable - no reproduction, so no fix to apply or verify.

- Project test suite: N/A - no fix attempted. Sanity check only: `CHANNEL=release ./y.sh test
  --mini-tests` passes on the unmodified tree (confirms the build/toolchain in this worktree is
  healthy, not a before/after comparison).

## What's not done / open questions

- Whether the original issue-690 report reproduces against an older/different libgccjit build
  than the one currently pinned in this checkout was not tested (would require building an older
  GCC snapshot from source, out of scope for this session).
- `fptoint_sat` (`src/builder.rs:1925`) and `gcc_float_to_int_cast` were read but not instrumented
  directly (no GCC-IR dump comparison was done for this candidate's own code path, unlike
  candidate 010 where a disassembly comparison was the approach that actually found the
  miscompile) - if this issue resurfaces on a future libgccjit bump, a `--emit=asm`/GCC-IR-dump
  comparison of the generated saturating-cast sequence (the way candidate 010's investigation
  found its bug) would be the next escalation, not another black-box differential run.
- This candidate's own persistent suspicion (recorded in the task that resumed this session) that
  candidate 010's exact code shape "hadn't been tried exactly here" is now resolved: it has been
  tried, in three variants, and none reproduce this candidate's bug class.
