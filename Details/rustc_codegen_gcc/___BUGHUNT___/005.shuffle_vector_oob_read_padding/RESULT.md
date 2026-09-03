# shuffle_vector_oob_read_padding

`Builder::shuffle_vector`'s mask-padding branch (`src/builder.rs`, `vec_num_units < mask_num_units`)
looked suspicious because it reads `v2` at indices `0..(mask_num_units - vec_num_units)` while `v2`
only actually has `vec_num_units` elements - a candidate hand tied this to GitHub issue #605
("Invalid loads in core::simd::splat for vectors with >= 4 elements", `-fsanitize=undefined` firing
inside portable-simd's `swizzle.rs:90`) as the likely root cause.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading + manual boundary-value reasoning (`src/builder.rs:2162-2199`, `src/intrinsic/simd.rs`'s `simd_splat`/`simd_shuffle` handlers, and the vendored `portable-simd/crates/core_simd/src/swizzle.rs` / `vector.rs` in this project's own built sysroot) | - | Found that `Simd::splat` on the toolchain pinned by this repo (nightly-2026-08-04) no longer goes through `shuffle_vector` at all - it uses the dedicated `simd_splat` intrinsic added to this backend by commit f11abba (2026-01-19), which builds the broadcast vector directly with `vec![scalar; out_len]`. Issue #605 predates that commit by about a year (filed 2025-01-22), so at the time it was filed `Simd::splat` still went through the swizzle-based path this candidate is about. The underlying `shuffle_vector` padding bug is still live today, just no longer reachable through `Simd::splat` specifically - it's reachable through `simd_swizzle!`'s still-unchanged `Swizzle::swizzle`/`concat_swizzle` (`swizzle.rs`), which broadcasts a shorter vector to a longer one the same way splat used to. |
| Standalone repro using `simd_swizzle!` directly (real public portable-simd API, not `Simd::splat`) with a project-added opt-in UBSan instrumentation hook (`repro/ubsan_hook.patch`, gated behind `CG_GCCJIT_UBSAN=1`) forwarding `-fsanitize=undefined` into libgccjit, since this backend has no built-in `-Zsanitizer=` support | Debug build (`CHANNEL=release` backend, `[profile.dev] lto = "off"` repro crate), linked with `gcc`+GCC's own `libubsan` (clang's compiler-rt ubsan archive was missing on this host) | Reproduced exactly issue #605's diagnostic shape: `swizzle.rs:88:13: runtime error: load of address ... with insufficient space for an object of type 'unsigned char'/'unsigned int'`, for a 1-lane->4-lane `u8` broadcast (case B) and a 2-lane->8-lane `u32` broadcast (case C); a 1-lane->2-lane broadcast (case A) produced zero diagnostics, confirming the exact boundary predicted by reading the code (OOB only when `mask_num_units - vec_num_units > vec_num_units`). 3/3 repeat runs: 40 diagnostics every time (case B + case C combined), 0 before case A's own marker. In every case the program's own output was already correct (`ok`, all `assert_eq!` pass) - the OOB-read values are computed but never actually selected by the shuffle mask. |

## Status

**Confirmed** - reproduces 3/3 runs (debug build, backend built in release mode, no fixed seed;
UBSan diagnostic addresses vary with ASLR but the diagnostic text/location and the total count (40)
are identical every run). Also verified the fix removes every diagnostic 3/3 runs with the same
program output.

**Fully reproducible**: Yes - `repro/src/main.rs` calls `std::simd::simd_swizzle!`, portable-simd's
own real public macro, exactly the way any real `#![feature(portable_simd)]` user would call it.
There is no second participant (peer/caller) being impersonated - `CONFIRMATION.md` section 11 does
not apply, this is a single-process, single-participant repro.

## Class

memory-safety

## Root cause

`Builder::shuffle_vector` (`src/builder.rs:2173-2199`) handles a `simd_shuffle`/`simd_swizzle!`
call whose mask is longer than its input vectors (broadcasting a shorter vector to more lanes) by
building a padded "v1" that's `mask_num_units` elements long: the first `vec_num_units` elements
are read from the real `v1`, and the remaining `mask_num_units - vec_num_units` elements are read
from `v2` via `new_vector_access(v2, i)` for `i` in `0..(mask_num_units - vec_num_units)`.

`v2` (whether it's a real second input, for `concat_swizzle`, or `v1` itself passed twice, for a
single-vector `swizzle`) only actually has `vec_num_units` elements. The loop only stays in bounds
when `mask_num_units - vec_num_units <= vec_num_units`, i.e. `mask_num_units <= 2 * vec_num_units`.
Portable-simd's own macro only ever produces mask *values* less than `2 * vec_num_units` (see the
`assert!(index < M)` / `assert!(index < 2 * M)` in `swizzle.rs`'s `swizzle`/`concat_swizzle`), but
that assert bounds the mask *values*, not the mask *length* - a swizzle can broadcast to an
arbitrarily large output length while every mask value still legally points at index 0. Whenever
the mask length exceeds `2 * vec_num_units`, this loop reads `v2` at indices past its last valid
element - a genuine out-of-bounds read of a GCC vector's storage, which is exactly what UBSan
flags in `swizzle.rs`.

The bug is silent (no wrong output, no crash under a plain non-instrumented build) because the
mask value used at every broadcast lane is always `< 2 * vec_num_units` by construction of the
macro's own assert, so the out-of-bounds *values* computed here are always among the padding slots
that `new_rvalue_vector_perm`'s mask never actually selects - they're materialized (the OOB read
genuinely happens at the GCC IR/machine-code level) but the result they'd feed into is discarded.
This matches issue #605's own observation that the bug "does not seem to lead to crashes / incorrect
behaviour".

This is a real, standing bug independent of `Simd::splat`'s current implementation - see "What was
tried" above for why `Simd::splat` itself no longer triggers it on this repo's pinned toolchain.

## Introduced by

faebf739 "Merge commit 'b385428e3ddf330805241e7758e773f933357c4b' into subtree-update_cg_gcc_2024-03-05"
(2024-03-05) - `git blame` on `src/builder.rs:2178-2188` (the exact `v2`-reading loop) attributes it
to this commit, authored by Guillaume Gomez. Like candidate 001's finding, this is a subtree-sync
merge commit (this project periodically merges from an internal/upstream subtree), so it's the
point this code entered this repo's own tracked history rather than necessarily where it was
originally authored.

## The fix

Applied directly to `src/builder.rs`: the `v2`-reading loop now only calls `new_vector_access` for
indices that are actually within `v2`'s bounds (`i < vec_num_units`); any index beyond that is
padded with a zero rvalue instead. This is sufficient (not just papering over the one triggering
input) because:

- Every slot this loop fills beyond `vec_num_units` was already guaranteed to be discarded before
  this fix (per the root cause above - the shuffle mask's own values, bounded by portable-simd's
  own `assert!(index < 2 * M)`, can never select an index that only exists because of padding
  beyond a real `2 * vec_num_units`-element concatenation), so substituting a zero for those slots
  changes nothing about which value the mask ultimately picks - checked against three differently-
  shaped inputs (case A: 1->2 lanes `u8`, unaffected by this branch since it never reads OOB in the
  first place; case B: 1->4 lanes `u8`; case C: 2->8 lanes `u32`, a different element width and a
  different lane-count ratio), not just the one byte sequence issue #605 itself used, per
  `CONFIRMATION.md` section 7.
- Re-running `repro/` against the patched, UBSan-instrumented backend: 0 diagnostics, 3/3 runs
  (down from 40 diagnostics, 3/3 runs, on the pristine backend) - `output.txt` has both full runs.
  Program output (`ok`, every `assert_eq!` passes) is byte-for-byte identical before and after.

- Project test suite (`./y.sh test --test-libcore`, coretests): 578 passed; 0 failed both before and
  after the patch (coretests does not itself exercise `core::simd`, so this is a general regression
  check on the patched codegen path rather than a targeted one). `./y.sh test --mini-tests` also
  passed identically before and after (`mini_core_hello_world` builds and runs with the expected
  output). The project's own `extended_sysroot_tests` (`regex`/`rand` crates, part of the
  unqualified `./y.sh test --release`) could not be used for this comparison in this session - it
  hit a linker/hash-mismatch failure caused by another concurrent bug-hunt worktree mutating the
  same shared `build/regex`/`build/build_sysroot` directories (via the `build/` symlink both
  worktrees pointed at) at the same time, unrelated to this patch; not retried once concurrency
  ended, given time budget.

## What's not done / open questions

- Not checked whether `simd_gather`/`simd_scatter` (which the code comment at
  `src/builder.rs:2124-2125` notes also calls into `shuffle_vector`) can construct a mask/vector
  length combination that hits this same branch - only `simd_swizzle!`'s two macro cases (`swizzle`,
  `concat_swizzle`) were exercised this session.
- Not checked against a wider fuzz/property sweep of `(vec_num_units, mask_num_units)` pairs beyond
  the three hand-picked boundary cases in `repro/` - the three cases were chosen specifically to
  bracket the `mask_num_units <= 2 * vec_num_units` boundary this root cause identifies, not to
  exhaustively cover every ratio.
- The `extended_sysroot_tests` gap noted above (regex/rand crates) was not independently re-run
  after the concurrent worktree's activity ended; a future session re-checking this candidate could
  confirm the fix against that broader suite too.
