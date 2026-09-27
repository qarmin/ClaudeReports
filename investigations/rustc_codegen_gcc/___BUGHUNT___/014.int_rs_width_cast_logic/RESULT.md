# int_rs_width_cast_logic

`src/int.rs`'s width-changing integer-cast logic - `CodegenCx::gcc_int_cast` (the shared cast
primitive, line ~914) and its callers `Builder::intcast`/`trunc`/`zext`/`sext` (`src/builder.rs`),
plus the sibling width-sensitive operations in the same file (`gcc_lshr`/`gcc_shl`'s
native/non-native shift decomposition, `gcc_icmp`'s non-native comparison, `gcc_bswap`'s
recursive halving, and `additive_operation`'s array-path sign detection via
`a_type.is_compatible_with(self.i128_type)`) - explicitly flagged in this workspace's project
knowledge file as "not yet explored at all" this session, separate from the division/remainder
sign-cast logic already investigated in candidates 002/003. This looked suspicious because it's
the same file, and the same general "does GCC's declared type sign/width actually match Rust
semantics" bug family that produced confirmed bugs elsewhere in this project (004, 006), applied
specifically to the native<->non-native (128-bit-as-2x64-array) boundary that `gcc_int_cast`
straddles - candidate 003's own "what's not done" section explicitly named shifts/comparisons in
that array-based path as untested.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static code reading of `gcc_int_cast` (all four branches: native<->native, non-native->native truncation, native->non-native widening, non-native<->non-native bitcast), `Builder::intcast`/`trunc`/`zext`/`sext`, `SignType::to_signed`/`to_unsigned`/`is_i128`/`is_u128` (`src/common.rs`), `gcc_lshr`/`gcc_shl`'s array-based decomposition, `gcc_icmp`'s non-native comparison, and how `i128_type`/`u128_type` are constructed as distinct `array<i64,2>`/`array<u64,2>` GCC types when `supports_128bit_integers = false` (`src/context.rs:212-227`) | - | Traced through the widening branch's sign-extension logic (`is_negative = value < zero`, evaluated using the *source* value's actual GCC-level sign, then used to fill the high array word) by hand for every native source width (i8/u8/i32/u32/i64/u64/bool) crossed with both target signs, and confirmed it matches Rust's `as`-cast semantics (extension sign follows the *source* type's signedness, independent of the destination's declared sign) - found no case where the logic diverges. Noted `is_i128`/`is_u128` use direct type-object equality (`unqualified() == cx.i128_type.unqualified()`) rather than `is_compatible_with`, which - given `i128_type`/`u128_type` are distinct array types (different element type: `i64` vs `u64`) - correctly discriminates the array representation's sign, consistent with candidate 012's finding that `is_compatible_with` itself is sound. No divergence found by inspection alone; moved to differential testing to check empirically rather than rely solely on manual reasoning about a 4-branch dispatch function. |
| Differential boundary-value testing: a real Rust program (`repro/width_cast_boundary.rs`, ~50 `black_box`-guarded expressions) exercising widening casts (every native int width and `bool` to `u128`/`i128`, both directions, including negative/all-ones/`MIN` boundary values), narrowing casts (`i128`/`u128` MIN/MAX/mixed-word boundary values down to every native width), same-width `i128<->u128` reinterpret casts (including the exact sign-bit-only value), 128-bit shifts at every structurally significant boundary (0, 1, 31, 32, 63, 64, 65, 96, 100, 127) for both `<<`/`>>`, signed and unsigned, `i128`/`u128` comparisons, `wrapping_add`/`wrapping_sub` (the array-path sign-detection sibling to 002/003's div/rem), `swap_bytes`/`reverse_bits` (`gcc_bswap`'s recursive halving), narrow-then-widen round-trips, and `wrapping_neg`/`!` (`gcc_neg`/`gcc_not`'s array path) - compiled and run three ways: (1) plain `rustc`/LLVM as ground truth, (2) this backend as-is on this x86_64 host (`supports_128bit_integers = true`, native GCC `__int128`), (3) this backend with `CodegenCx::new`'s `supports_128bit_integers` parameter forced to `false` (`repro/force_non_native.patch`, the same 2-line diagnostic patch candidate 003 used, **not a fix** - reverted immediately after use) to force every i128/u128 value through the array-based representation and genuinely exercise every branch of `gcc_int_cast`/`gcc_lshr`/`gcc_shl`/`gcc_icmp`/`gcc_bswap` under test | Release build, `CHANNEL=release ./y.sh rustc`, real captured stdout - see `output.txt` and `repro/run.sh` | All three configurations produced bit-for-bit identical output (132 lines) for every expression. `objdump -d` on the forced-non-native binary confirms the array path was genuinely compiled and executed (e.g. `main`'s `{:?}`-formatting of the array-based `u128` values calls libgcc's `__udivti3`, only reachable when 128-bit division goes through the array-path's extern-function dispatch rather than a native GCC divide instruction) - this is a real run of the code path under test, not a simulation. |
| Repeat under a debug build of the backend itself: rebuilt `rustc_codegen_gcc` without `CHANNEL=release` (this project's `Cargo.toml` still applies `debug-assertions = true` to the backend crate even in that mode, since only `opt-level` is overridden for it), re-ran the same forced-non-native test program through it | `./y.sh build` (dev profile) + `./y.sh rustc`, same test file | Compiled and ran without triggering any of `int.rs`'s several `debug_assert!` invariant checks (e.g. `debug_assert!(a_type.dyncast_array().is_some())` in `additive_operation`/`multiplicative_operation`), and produced output identical to both the release-backend run and the plain-rustc/LLVM reference. |

## Status

**Not reproducible.** Covered: every native integer width (`i8`/`u8`/`i16`.../`i64`/`u64`) plus
`bool` widened to `u128`/`i128` at boundary values (`-1`, all-ones, `MIN`, positive), `i128`/`u128`
narrowed to every native width at boundary values (`MIN`, `MAX`, a mixed-high/low-word pattern),
same-width `i128<->u128` reinterpret casts including the exact sign-bit-only value, 128-bit shifts
at every boundary amount from 0 to 127 (both directions, signed and unsigned), 128-bit comparisons,
`wrapping_add`/`wrapping_sub`, `swap_bytes`/`reverse_bits`, round-trip narrow-then-widen casts, and
`wrapping_neg`/bitwise-not - all run for real against the array-based (non-native 128-bit
representation) code path in both a release and a debug build of the backend, matching plain
rustc/LLVM bit-for-bit in every configuration (3/3 configurations, 132/132 lines matching, no
`debug_assert!` firing).

**Fully reproducible**: N/A (nothing reproduced - this field only applies to `Confirmed` findings).

## Class

N/A (Not reproducible).

## Root cause

Not a bug: `gcc_int_cast`'s native<->non-native widening branch derives its sign-extension
decision (`is_negative = value < zero`, compared using the source value's own GCC-level type) from
exactly the same signal Rust's own `as`-cast semantics use - the *source* type's signedness,
independent of the destination's declared sign - and `Builder::intcast` (the actual `IntToInt`
cast entry point from `rustc_codegen_ssa`) always forces `value`'s GCC type to match the caller-
supplied `is_signed` flag (which is itself the *source* scalar's real Rust-level signedness, per
`rustc_codegen_ssa::mir::rvalue::cast_immediate`'s `(Int(_, is_signed), Int(..))` match arm)
*before* calling `gcc_int_cast`, so the two-step dispatch is sign-correct by construction. The
narrowing branch (non-native array -> native) only ever reads the low array element regardless of
sign, which is correct because truncation never depends on sign in two's-complement. The
non-native<->non-native branch is a bitcast, correct because `i128`/`u128`'s array representations
share the same bit width and layout, differing only in interpretation. `SignType::is_i128`/`is_u128`
(used throughout to route sign-sensitive decisions like `additive_operation`'s dispatch to
`__rust_i128_add` vs `__rust_u128_add`) correctly discriminate the array representation because
`i128_type`/`u128_type` are constructed as genuinely distinct GCC array types (`array<i64,2>` vs
`array<u64,2>`, different element type) and compared by direct type-object equality, not by the
looser `is_compatible_with`. `gcc_lshr`/`gcc_shl`'s array-based bit-shift decomposition (splitting
a 128-bit shift into two 64-bit half-shifts plus a sign/zero fill) was independently checked by
hand against the standard double-width shift algorithm and found to implement it correctly for all
three sub-cases (shift amount 0, 1-63, 64-127).

## Introduced by

N/A - not reproducible, no buggy commit to identify.

## The fix

No fix - nothing to fix. No `patch.diff` in this folder.

- Project test suite: N/A, no fix was applied. The one diagnostic patch used during the
  investigation (`repro/force_non_native.patch`, identical to candidate 003's) was applied to the
  main tree only transiently to force the array-based code path under test, then reverted
  (`git checkout -- src/context.rs`) before this candidate's own investigation ended; the backend
  was rebuilt afterward and `git status` shows the main tree clean except for this workspace's own
  `.gitignore` entry.

## What's not done / open questions

- This checked `gcc_int_cast`/`intcast`/`trunc`/`zext`/`sext`/`gcc_lshr`/`gcc_shl`/`gcc_icmp`/
  `gcc_bswap`/`gcc_neg`/`gcc_not` and `additive_operation`'s sign dispatch. It did **not**
  independently re-verify `multiplicative_operation`'s array-path sign dispatch beyond what
  candidate 003 already covered for `/`/`%` specifically (both use the same
  `a_type != b_type -> gcc_int_cast(b, a_type)` line, already argued safe in 003's root-cause
  section since `/`/`%`/`+`/`-`/`*` on primitive ints never actually reach that line with mismatched
  types in practice).
- `Builder::sext`'s own FIXME ("check that it indeed sign extend the value") was already
  independently investigated and ruled not-live in candidate 007 (`sext_missing_extend_int_fix`) -
  not re-litigated here.
- `Builder::zext`'s own FIXME ("this does not zero-extend", `src/builder.rs:1885`) was read but
  **not independently tested this session** - it calls the same `gcc_int_cast` verified here, and
  this candidate's own differential test does exercise zero-extending casts (e.g. `u8 as i128`,
  `bool as u128`) through that exact code path without finding a divergence, but a dedicated test
  isolating `zext`'s own call sites (as opposed to `intcast`'s, which pre-corrects the source
  type's sign before delegating to `gcc_int_cast`) was not built. Worth a narrow follow-up if a
  `zext`-specific call site with a genuinely mismatched source sign can be identified (this
  project's own knowledge file notes an upstream PR #552 addressed a related `zext` concern,
  without touching `sext`).
- Only exercised on this x86_64 host's bundled libgccjit (which has native `__int128` support);
  the non-native/array-based path was forced via the same diagnostic patch candidate 003 used
  rather than tested on a genuine GCC target lacking native 128-bit integers (none was available
  in this environment) - same caveat 003 already recorded.
- No `debug`-build sysroot/guest-side test was run (only the *backend's own* debug build was
  tried) - `debug-assertions`-enabled guest code paths (e.g. checked shifts panicking on an
  out-of-range shift amount before ever reaching codegen) weren't part of what this candidate was
  investigating, since the guest program's own overflow/shift-amount checks are inserted by
  `rustc_codegen_ssa` before this backend ever sees the operation.
