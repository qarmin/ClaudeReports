# timeval_rounding_sibling_sweep

The repo's own recent history (commit 0cea1880 "fix(bug): ns->us overflow handling (#1655)",
2026-09-16 - two days before this session, already on `main`/HEAD at 287214b8) fixed a real bug:
several `Duration`/`Timespec` -> `timeval` (sec + usec) conversions computed
`tv_usec = (tv_nsec + 999) / 1000` without checking whether the rounded-up result could reach
`1_000_000` (a full second, e.g. when `tv_nsec` is near its max valid value `999_999_999`), which
produces a `timeval` with an out-of-range `tv_usec` field instead of carrying the overflow into
`tv_sec`. The fix centralized this into a new `Timespec::to_sec_usec` helper (`src/timespec.rs`)
that correctly carries the overflow, and updated the known call sites. Per `SKILL.md`'s "the same
underlying bug can live in more than one place" - this candidate swept for any sibling call site
the fix might have missed, since a bug class the project's own maintainers just had to fix once is
exactly the kind of thing likely to recur elsewhere in the same codebase.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `git show 0cea1880` - read the full fix to understand exactly which files/functions it touched and the shape of the bug | Whole commit diff | Confirmed the fix touched `src/backend/libc/event/{syscalls,windows_syscalls}.rs`, `src/backend/{libc,linux_raw}/net/sockopt.rs`, and added `Timespec::to_sec_usec`/`to_timeval` to `src/timespec.rs`. All now delegate to the shared, carry-correct helper. |
| `grep -rn "+ 999\|+999\|/ 1000\b" src/ --include=*.rs` - find every remaining hand-rolled "round up division by 1000/1000000" site, fixed or not | Whole crate | 3 matches: `timespec.rs:202` (`as_c_int_millis`, ms conversion), `timespec.rs:212` (`to_sec_usec` itself, the now-fixed shared helper), `backend/libc/net/sockopt.rs:271` (a `#[cfg(windows)]`-only, Windows `SO_RCVTIMEO`/`SO_SNDTIMEO` millisecond `u32` conversion). |
| Per-site structural analysis: does this conversion produce a **two-field struct** (a `sec`+`subsec` pair with a strict range requirement on the subsec field, the shape that actually needs a carry) or a **single scalar** value (where rounding up into the "next unit" is already correct behavior with no second field to violate)? | `as_c_int_millis` (→ single `c_int` millis scalar), the Windows millis conversion (→ single `u32` millis scalar) | Both are scalar millisecond conversions, not sec+subsec structs - rounding `999_999_999`ns up to `1000`ms and adding it to `secs*1000` is already the mathematically correct total millisecond count, with no separate field whose valid range (`< 1_000_000` for `tv_usec`) could be violated. Neither has the bug class 0cea1880 fixed. |
| `grep -rn "tv_usec\s*[:=]" src/ --include=*.rs` - every remaining `tv_usec` field assignment anywhere in the crate | Whole crate | Every non-literal-`0` assignment goes through the now-fixed `to_sec_usec`/`to_timeval` helpers (`usec as _` immediately following a `to_sec_usec()`/`to_timeval()` call); the only other assignments are literal `tv_usec: 0` (no rounding involved at all). No leftover hand-rolled, uncarried `(nsec + 999) / 1000` computation feeding a `tv_usec` field remains anywhere in the crate. |

## Status

**Not reproducible**. Commit 0cea1880's fix is complete and consistent: every `tv_usec`-producing
code path in the crate now goes through the carry-correct `Timespec::to_sec_usec`/`to_timeval`
helpers, and the two remaining hand-rolled `(x + N-1) / N`-shaped rounding computations elsewhere in
the crate are both scalar millisecond conversions (no separate subsec field to overflow), not
instances of the same bug class.

## Root cause

N/A - no bug found. This was a targeted verification sweep prompted by the project's own recent fix
commit, not a fresh suspicion from code reading; the result (fix is complete, no sibling left
behind) is itself the useful finding for this workspace's record.

## Introduced by

N/A - not applicable, no bug confirmed.

## The fix

N/A - no bug found, nothing to fix.

## What's not done / open questions

- Not audited: whether any *other* duration/timespec-shaped conversion in the crate (not
  necessarily producing a `timeval`'s `tv_usec` specifically - e.g. a nanosecond-to-some-other-unit
  conversion with its own two-field carry requirement, if one exists elsewhere for a different
  target struct) has an analogous, differently-shaped carry bug. This sweep was scoped specifically
  to the `(nsec + 999) / 1000`-into-`tv_usec` pattern the fix commit addressed, not every possible
  time-unit conversion in the crate.
