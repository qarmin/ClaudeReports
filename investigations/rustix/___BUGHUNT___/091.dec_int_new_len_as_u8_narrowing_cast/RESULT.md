# dec_int_new_len_as_u8_narrowing_cast

`src/path/dec_int.rs:196`'s `DecInt::new` ends with `NonZeroU8::new(len as u8).unwrap()`, a
narrowing `usize -> u8` cast on a loop counter `len` that is only bounded by a `if len > BUF_LEN {
unsafe { unreachable_unchecked() } }` guard inside the digit-writing loop. The candidate was
flagged (self-described as low-confidence) because "a narrowing cast on a computed length, bounded
only by an `unreachable_unchecked` the reviewer has to re-derive is actually unreachable" is a
shape worth a second independent look, even though the surrounding comment already claims it's
safe.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Existing test suite, debug-assertions build | `cargo test --features "std,all-apis,use-libc" dec_int` (dev profile, debug assertions on). Exercises `tests/path/dec_int.rs`'s `test_dec_int`, which already checks every `i16`/`u16` value exhaustively plus `u8::MAX`/`i8::MIN`/`u16::MAX`/`i16::MIN`/`u32::MAX`/`i32::MIN`/`u64::MAX`/`i64::MIN`/`usize::MAX`/`isize::MIN` against `format!("{i}")` | `ok. 1 passed; 0 failed` - no panic, no wrong output, for every digit-count boundary `DecInt::new` can hit across every `Integer` impl |
| Standalone boundary-value repro against the public API | New crate (`/tmp/decint_repro`) depending on this worktree's `rustix` via a path dependency, `cargo run` (debug build), calling `DecInt::new` on `0`/`1`/`-1` and every type's `MIN`/`MAX` for `i8`/`u8`/`i16`/`u16`/`i32`/`u32`/`i64`/`u64`/`isize`/`usize`, asserting the formatted string matches Rust's own `Display` output | All checks passed, see `output.txt` |
| Miri, same exhaustive existing test | `RUSTFLAGS="--cfg rustix_no_linux_raw" cargo +nightly miri test --features "std,fs" --test path -- dec_int` (forces the `libc` backend per this workspace's `README.md`, since `dec_int.rs` itself does no syscalls and doesn't need this, but the test binary links the whole crate) | `test dec_int::test_dec_int ... ok`, finished in 1020.62s, no UB reported (stacked-borrows/out-of-bounds/`MaybeUninit`-misuse checks all clean) |
| Manual arithmetic proof of the `len` bound (boundary-value construction, reasoning rather than a run) | Traced the loop: `len` starts at 1, each iteration adds 1 and writes one digit, so after `k` digit-writing iterations `len = 1 + k`; `is_neg` adds one more. `BUF_LEN = U64_MAX_STR_LEN + 1 = 21` (`"18446744073709551615".len() == 20`, verified via `python3`). The maximum digit count `k` over every `Integer` impl (`i8/i16/i32/i64/u8/u16/u32/u64/isize/usize`, all <= 64-bit) is 20, achieved only by the unsigned `u64::MAX`/`usize::MAX` case (not negative, so no `+1`), giving `len = 21 == BUF_LEN` exactly. The signed `i64::MIN`/`isize::MIN` case has an unsigned magnitude (`9223372036854775808`) with only 19 digits, giving `len = 1 + 19 + 1 = 21` as well - never 22 | Confirms `len` never exceeds `BUF_LEN` (21) for any supported `Integer` impl, so the `len > BUF_LEN` guard and its `unreachable_unchecked()` are provably dead code, and `len as u8` only ever sees values in `[1, 21]` - nowhere near the `u8::MAX` (255) wraparound point, and never 0 |

## Status

**Not reproducible.** Covered: the project's own existing exhaustive/boundary test (every `i16`/
`u16` value plus every type's `MIN`/`MAX`) under both a plain debug-assertions build and Miri's
full UB checker, a second independent standalone repro hitting the same boundary values through
the public API, and a manual arithmetic proof that the maximum achievable `len` (21) is far below
where the `u8` cast could truncate (256) or where `unreachable_unchecked()` could actually be
reached. No tool surfaced even a partial signal (no panic, no wrong formatted output, no Miri UB
diagnostic) across any of these.

## Class

N/A - not a bug.

## Root cause

N/A - not a bug, but the reason the suspected risk doesn't materialize: `BUF_LEN` is a compile-time
constant (`U64_MAX_STR_LEN + 1 = 21`) sized specifically to fit the longest possible formatted
64-bit integer (`i64::MIN`/`u64::MAX`, 20 digits) plus the optional leading `-` and the loop's
initial `len = 1`. Every `Integer` impl this function is generic over (`i8`/`i16`/`i32`/`i64`/`u8`/
`u16`/`u32`/`u64`/`isize`/`usize`, the latter two only for 16/32/64-bit pointer widths, i.e. never
wider than `i64`/`u64`) can therefore never produce more digits than `U64_MAX_STR_LEN`, so `len`
after the loop (and after the optional `is_neg` increment) is bounded above by exactly `BUF_LEN`
(21). The `len > BUF_LEN` check exists purely as a documented invariant / defense-in-depth for a
hypothetical future change (e.g. a wider `Integer` impl) that might violate the assumption - under
the current set of `Integer` impls it is genuinely always false, making `unreachable_unchecked()`
truly unreachable and `len as u8` a lossless cast (max value 21, `u8` holds up to 255).

## Introduced by

N/A - no bug to attribute; the code has been structurally the same (cast guarded by the same
`unreachable_unchecked` pattern) for as long as `DecInt` has existed in this form.

## The fix

N/A - no fix needed; nothing to patch. No changes were made to the main project's working tree
(only the disposable worktree at `___BUGHUNT___/_worktrees/091`, removed after this investigation,
and a throwaway standalone crate at `/tmp/decint_repro`, outside the repo).

- Project test suite: N/A, no fix applied. For reference, the existing suite passes before this
  investigation: `path::dec_int::test_dec_int` → ok (1 passed; 0 failed), run both under a plain
  debug build and under Miri (see "What was tried" above); nothing in the suite was touched.

## What's not done / open questions

- Only the `Integer`-bounded (<= 64-bit magnitude) call sites were checked, since that's the only
  way to reach `DecInt::new` through its public, sealed `Integer` trait - there is no way for a
  caller to invoke it with a wider integer type without the crate itself growing a new `impl
  Integer for ...`, which would be a deliberate, reviewable change to this same file.
- If the crate ever added a 128-bit `Integer` impl (`i128`/`u128`), `BUF_LEN` would need to grow
  accordingly or this exact guard would become live - worth a comment note for future maintainers,
  but out of scope for this candidate since no such impl exists today.
- No sibling narrowing-cast-on-a-computed-length pattern was searched for elsewhere in the crate as
  part of this candidate; `HEURISTICS.md`/`IDEAS.md` may already track that as a separate sweep
  (not re-derived here).
