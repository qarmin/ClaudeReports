# timespec_to_duration_tv_nsec_sign_extension

`src/timespec.rs`, `impl TryFrom<Timespec> for Duration` (lines 243-249): `Duration::new(ts.tv_sec.try_into()?, ts.tv_nsec as _)`
casts the `i64`/`c_long` `tv_nsec` field straight into `u32` with no range check, even though the
function is a `TryFrom` and its sibling `to_sec_usec` (lines ~210-218) explicitly validates/carries
instead of blindly casting. `Timespec`'s own doc comment says `tv_nsec` "Must be less than
1_000_000_000", but nothing enforces `0 <= tv_nsec` here. This was candidate 101 in the batch that
also produced candidate 048 (`fix_negative_nsecs` assert off-by-one) and candidate 097
(`checked_sub` premature `tv_sec` overflow) - neither of those investigated this specific
`TryFrom<Timespec> for Duration` conversion path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction against the real public API (`rustix::event::Timespec` -> `std::time::Duration::try_from`), release build, path dependency on this worktree | `tv_nsec` in `{-1, -1_000_000_000, 1_000_000_000, 1_999_999_999, i64::MIN}`, `tv_sec = 5` (or `0` for the `i64::MIN` case) | **Reproduced** on first attempt - every out-of-range value was silently accepted as `Ok(Duration)` with a nonsensical value instead of `Err`; no further escalation needed (a pure, deterministic, single-participant arithmetic conversion - the same bug class as candidate 097, which also stopped at the first tool) |

## Status

**Confirmed**

Reproduces 3/3 runs by hand (and every run of the standalone `repro/`, which was built and run
multiple times while iterating on the fix) - deterministic, pure `TryFrom` conversion with no I/O,
randomness, or layout dependence, so no flakiness is possible or was observed.

**Fully reproducible**: **Yes**. `repro/` calls rustix's real, public `rustix::event::Timespec`
struct (both fields `pub`) and the real public `Duration::try_from`/`TryFrom<Timespec>` impl
directly - the genuine entry point a real caller would use. There is no second participant to fake
here: like candidate 097, this is a single-participant pure arithmetic conversion operating only on
caller-supplied struct fields, not a parser/decoder reacting to another participant's wire input -
CONFIRMATION.md section 11's "real second participant" requirement does not apply.

## Class

logic

## Root cause

`TryFrom<Timespec> for Duration` only validates `tv_sec` (via `.try_into()?`, which rejects a
negative `i64`/`c_long` since `Duration::new`'s first argument is `u64`). It does **not** validate
`tv_nsec` at all before casting it to `u32` with `as _`. Two distinct failure modes follow from the
same missing check:

1. **Negative `tv_nsec`**: `as` casting a negative `i64` to `u32` reinterprets the two's-complement
   bit pattern, e.g. `-1i64 as u32 == 0xFFFFFFFF == 4_294_967_295`. `Duration::new(secs, nanos)`
   (`core::time`) does *not* reject `nanos >= 1_000_000_000` - it normalizes it by carrying
   `nanos / 1_000_000_000` into `secs` and keeping `nanos % 1_000_000_000`. So
   `Timespec { tv_sec: 5, tv_nsec: -1 }` silently becomes `Duration::new(5 + 4, 294_967_295)` =
   `9.294967295s` - more than 4 seconds and nearly 0.3s off from any value that input could
   plausibly represent, with no error ever surfaced.
2. **`tv_nsec >= 1_000_000_000` (still non-negative, within u32 range)**: the same missing
   validation lets an over-large-but-positive `tv_nsec` (e.g. `1_999_999_999`, almost double the
   documented bound) pass through `as u32` unchanged, and `Duration::new`'s own carry-normalization
   silently "fixes" it into a plausible-looking but wrong `Duration` (`6.999999999s` instead of a
   rejection).

Both are reachable from any caller-constructed `Timespec` (`tv_sec`/`tv_nsec` are both `pub`), and
also from any `Timespec` obtained by copying raw bytes into the struct (e.g. from a corrupted file,
a non-kernel source, or hand-rolled FFI) - the type's own `#[repr(C)]`/`Default`/`Copy` derives are
clearly meant to support exactly that kind of construction.

Note: `i64::MIN as u32` happens to produce `0u32` (because `i64::MIN`'s low 32 bits are all zero),
so that specific extreme case converts to `Ok(Duration::new(secs, 0))` rather than a visibly huge
value - still wrong (an `i64::MIN` `tv_nsec` is nonsensical and should be rejected, not silently
treated as "zero nanoseconds"), just a less dramatic-looking instance of the same missing-validation
root cause.

## Introduced by

b710cf2fb80758bd81a881f21ec67bee61387866 "Add addition, subtraction and conversion to/from
`core::time::Duration` for Timespec (#1311)" (2025-02-06) - this commit added both
`TryFrom<Timespec> for Duration` and `TryFrom<Duration> for Timespec` to `src/timespec.rs` in one
patch; the unchecked `tv_nsec as _` cast has been present, unchanged, since this commit introduced
the impl (confirmed via `git log -S`/`git log -p` on the exact line).

## The fix

Validate `tv_nsec` against the same documented invariant the struct's own doc comment states
(`0 <= tv_nsec < 1_000_000_000`) before casting, returning an error instead of silently
misinterpreting an out-of-range value:

```rust
impl TryFrom<Timespec> for Duration {
    type Error = TryFromIntError;

    fn try_from(ts: Timespec) -> Result<Self, Self::Error> {
        let secs = ts.tv_sec.try_into()?;
        // `tv_nsec` is documented to always hold `0..1_000_000_000`; reject anything outside
        // that range instead of silently misinterpreting it via a lossy `as` cast (a negative
        // or overlong `tv_nsec` would otherwise produce a nonsensical `Duration`).
        if !(0..1_000_000_000).contains(&ts.tv_nsec) {
            // There's no public constructor for `TryFromIntError`, so produce a genuine one via
            // an int conversion that's guaranteed to fail.
            return Err(u8::try_from(u32::MAX).unwrap_err());
        }
        Ok(Self::new(secs, ts.tv_nsec as u32))
    }
}
```

The public `Error = TryFromIntError` associated type is kept unchanged (so this is not a breaking
API change) by deliberately triggering a real, guaranteed-to-fail `u8::try_from(u32::MAX)` to
produce a genuine `TryFromIntError` value in the rejection path, since `TryFromIntError` has no
public constructor in `core`.

Verified against more than just the one triggering input (CONFIRMATION.md section 7): re-ran the
same boundary-value repro against the patched tree - all five previously-wrong cases
(`tv_nsec` = -1, -1_000_000_000, 1_000_000_000, 1_999_999_999, `i64::MIN`) now return `Err`, and the
two valid in-range cases (`tv_nsec` = 0, 999_999_999) still convert to the identical correct
`Duration` as before the fix (`5s`, `5.999999999s`) - confirming the fix only tightens the
rejection path and does not regress valid input.

- Project test suite (`cargo test --features=all-apis --release --workspace --no-fail-fast`,
  21 test binaries): **413 passed, 1 failed before and after** (identical before/after - the one
  failure, `ioctl::test_int_setter`, is a pre-existing environment issue unrelated to this change:
  it needs a real ioctl-capable device file this sandboxed container doesn't have, confirmed by
  reading its panic message - `Os { code: 2, kind: NotFound, message: "No such file or directory" }`
  - and reproduces identically on an unpatched tree).

## What's not done / open questions

- The sibling `impl TryFrom<Duration> for Timespec` (lines 251-260) was read and does not share
  this bug: `Duration::subsec_nanos()` is itself guaranteed by `core::time::Duration` to always be
  `< 1_000_000_000`, so there's no out-of-range input possible on that path.
- `Timespec::as_c_int_millis` (lines 192-205) also reads `tv_nsec` without validating it's
  non-negative (`i64::from(self.tv_nsec) + 999_999`, then divides) - a negative `tv_nsec` there
  would produce a wrong millisecond count rather than panicking or erroring, same missing-invariant
  root cause as this candidate, but a different call site with different fallout (silently wrong
  `c_int` milliseconds passed into a real `poll`/`epoll` timeout, not a `Duration`). Not
  investigated this session - worth a separate candidate.
- No longer fuzzing campaign was run for this specific function since manual boundary-value
  construction already found and confirmed the bug on the first attempt (per `SKILL.md`'s tool
  selection guidance, arithmetic/cast bugs on documented-but-unenforced invariants are exactly the
  case where hand-constructed boundary values are the right first tool, not a fuzz harness).
