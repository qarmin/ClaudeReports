# fix_negative_nsecs_hard_assert_timespec

`src/timespec.rs`'s `fix_negative_nsecs`/`adjust` (`#[cfg(apple)]`-only, reached directly from raw
`stat.st_atime_nsec`/`st_mtime_nsec`/`st_ctime_nsec` in `src/backend/libc/fs/syscalls.rs:2688-2694`
and a raw `clock_gettime`-family result in `src/backend/libc/time/syscalls.rs:522`) uses real
(non-`debug_assert!`) `assert!`s to validate the macOS negative-nanoseconds quirk before adjusting
it - the candidate description asked whether a worse-than-one-second quirk, or `secs` already at
`time_t::MIN`, could abort the process instead of degrading gracefully. Investigating the
mechanism in detail (rather than just the two extreme-boundary asserts named in the description)
turned up a third, much more directly reachable bug: `adjust()`'s `assert!(*secs < 0)` is a strict
off-by-one against the very upstream writeup this workaround is based on (linked from the
function's own doc comment, rust-lang/rust#108277, comment by joshtriplett), which defines the bug
for `seconds <= 0` - `secs == 0` included.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Fetch and read the full upstream writeup (rust-lang/rust#108277 comments, via `gh`/GitHub API) that `fix_negative_nsecs`'s doc comment links to, to pin down the quirk's real documented magnitude/boundaries rather than guessing | `curl` against the GitHub REST API, read joshtriplett's 2023-10-31 comment in full | The upstream algorithm is explicitly defined as: "if we get a Timespec value with seconds **less than or equal to zero**, and nanoseconds between -999_999_999 and -1 (inclusive)" - `secs == 0` is explicitly in scope, and the described mechanism (buggy_secs = correct_secs + 1, buggy_nsecs = correct_nsecs - 1_000_000_000) never produces `nsecs < -999_999_999`, so the `nsecs >= -1_000_000_000` assert's own boundary looked safe by construction from this same mechanism |
| Manual boundary-value construction of `fix_negative_nsecs(0, -100_000_000)` - the documented buggy encoding of a real macOS timestamp ~0.1s before the Unix epoch (correct form: `secs: -1, nsecs: 900_000_000`) - as a `#[cfg(apple)]`-gated unit test added directly to `src/timespec.rs`'s own `mod tests`, built+run via `cargo +nightly miri test --target x86_64-apple-darwin` (Miri cross-interprets MIR, so the real `#[cfg(apple)]` code compiles and runs without macOS hardware or a cross-linker - same technique as this workspace's candidate 039) | debug build, `MIRIFLAGS="-Zmiri-disable-isolation"`, 1 initial run + 3 repeats, in an isolated git worktree | **Reproduces 4/4 runs**, deterministic, identical panic every time: `panicked at src/timespec.rs:383:9: assertion failed: *secs < 0` |
| Checked whether the other two asserts in the same function (`nsecs >= -1_000_000_000`, `*secs > c::time_t::MIN`) are independently reachable, to avoid treating only one of the three named-in-the-description failure modes | Code reading against the documented bug mechanism above (not an independent Miri/fuzz run - the mechanism itself rules these out) | Both remain theoretical, not demonstrated reachable: `nsecs < -1_000_000_000` would require the buggy conversion to be off by more than exactly one second, which the documented mechanism (a simple off-by-one-second XNU internal conversion) never produces; `secs == time_t::MIN` would require a file timestamp within one second of `i64::MIN` nanoseconds-since-epoch (~292 billion years before 1970), not a real filesystem timestamp. Recorded as checked-but-not-independently-confirmed rather than silently ignored (`SKILL.md` pitfall #7) |
| Applied a bounded fix (`assert!(*secs < 0)` -> `assert!(*secs <= 0)`) and re-ran the same test under Miri, 1 initial + 3 repeats | Same build/target/flags | Assert no longer fires; function now returns the correct normalized form (`secs: -1, nsecs: 900_000_000`), matching the upstream writeup's own worked example exactly - 4/4 runs, no flakiness |

## Status

**Confirmed**. Reproduces 4/4 runs (debug build, Miri cross-interpreting `--target
x86_64-apple-darwin`, `-Zmiri-disable-isolation`), fully deterministic, matching the exact assert
message and source line every run.

**Fully reproducible**: **No**, for a specific, concrete reason (`CONFIRMATION.md` section 11's
narrow hardware exception, not an unjustified shortcut). This is a single-process,
single-participant finding - the "second participant" conceptually responsible for producing the
negative-nanoseconds value is the real macOS/XNU kernel, not a peer sending adversarial bytes over
a channel this repro could instead drive for real (same category as this workspace's own
candidates 004/039/035, which also needed Apple-specific kernel behavior). Unlike candidate 039
though, this specific quirk is a genuine XNU internal-conversion bug, not something Miri's own
generic libc shim (or any accessible emulation in this environment) reproduces by itself when
driven through the real public `rustix::fs::fstat`/`stat`/`clock_gettime` entry points - Miri's
shim passes real host-kernel values through on this Linux sandbox, and the real Linux host kernel
has no such bug, so driving the public API end-to-end here would just observe correct output, not
the quirk. No real Apple hardware, macOS CI, or XNU-accurate emulator is available in this
environment to produce a genuinely-buggy `stat()`/`clock_gettime()` result. The repro therefore
drives the real, unmodified, `pub(crate)` `fix_negative_nsecs` function directly (not a
reimplementation - the exact source function, compiled for the real `x86_64-apple-darwin` target)
with the exact `(secs, nsecs)` pair the upstream writeup documents as the real XNU output for this
case, which is the closest honest substitute available for "the single external artifact this bug
is about" (analogous to CONFIRMATION.md section 11's own "a malformed file handed to a parser's
real public entry point" carve-out, except the entry point itself - a real buggy macOS stat() call
- cannot be produced in this sandbox at all).

## Class

crash

## Root cause

`adjust()` (inside `fix_negative_nsecs`, `src/timespec.rs`) is the cold-path fixup applied when a
raw `(secs, nsecs)` pair decoded from a real `stat`/`clock_gettime`-family syscall result has
`nsecs < 0`. Its own doc comment and inline logic are based on a specific upstream explanation
(rust-lang/rust#108277, comment by joshtriplett, 2023-10-31) of exactly when and how macOS
produces this: "if we get a Timespec value with seconds less than or equal to zero, and
nanoseconds between -999_999_999 and -1 (inclusive), we can add 1_000_000_000 to the nanoseconds
and subtract 1 from the seconds." That is, the documented precondition is `secs <= 0`. `adjust()`'s
own guard, `assert!(*secs < 0)`, is strictly narrower than this by exactly one value (`secs == 0`
is excluded) - an off-by-one against the very algorithm it's implementing. `secs == 0, nsecs < 0`
is not a contrived edge case: it is precisely the buggy macOS encoding of any real file/clock
timestamp strictly between the Unix epoch and one second before it (e.g. a file touched a few
hundred milliseconds before 1970-01-01 00:00:00 UTC) - the correct normalized form would be `secs:
-1, nsecs: <900_000_000-ish>`, and the documented macOS bug represents it instead as `secs: 0,
nsecs: <negative>` (one second "off" in the other direction from the `-59`/`-900_000_000` example
the function's own existing test already covers). Because the `assert!` is a real, non-debug
assertion, it aborts the whole process (not a recoverable `Err`) the moment a real macOS `stat`/
`fstat`/`lstat`/`statat`/`clock_gettime`-family call ever returns a timestamp in exactly this
window - which, per the upstream bug report, is a genuine, observed real-world XNU behavior, not a
hypothetical.

## Introduced by

912db929 "Work around a negative timestamps bug on macOS. (#1305)" (2025-02-01) - this commit
introduced `fix_negative_nsecs`/`adjust` in its original form, including the exact `assert!(*secs
< 0)` line, verified via `git log -p --follow -- src/timespec.rs` (the only historical hit for
this exact assert text; it has never been touched since).

## The fix

Changed `adjust()`'s `assert!(*secs < 0)` to `assert!(*secs <= 0)` in `src/timespec.rs`, matching
the upstream writeup's own documented precondition exactly. This is sufficient for the
demonstrated case: with the relaxed guard, `fix_negative_nsecs(0, -100_000_000)` now returns
`(-1, 900_000_000)`, the correct normalized `Timespec` representation, instead of aborting.

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: re-derived the
upstream algorithm's own worked example (`secs: -59, nsecs: -900_000_000` -> `secs: -60, nsecs:
100_000_000`, the function's pre-existing `test_negative_timestamps` test) still passes unchanged
after the fix (`*secs <= 0` is strictly weaker than `*secs < 0`, so every input the old assert
already accepted is still accepted); the two other asserts in the same function
(`nsecs >= -1_000_000_000`, `*secs > c::time_t::MIN`) were left unchanged - see "What was tried"
above for why they were checked and found not independently reachable, so changing them had no
demonstrated bug to fix and risked masking a genuinely-impossible state instead of a real one.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64-unknown-linux-gnu, where `#[cfg(apple)]` code including this fix doesn't even
  compile): **411 passed, 1 failed, identical before and after** - the one failure
  (`ioctl::test_int_setter`) is a pre-existing, environment-caused gap (`/dev/net/tun` doesn't
  exist in this sandbox), documented in `projects/rustix.md`, unrelated to this change.
- Fix-specific verification instead ran via Miri cross-interpretation for `x86_64-apple-darwin`
  (the only way to exercise `#[cfg(apple)]` code at all from this host) - see "What was tried".

## What's not done / open questions

- The two other asserts in `adjust()` (`nsecs >= -1_000_000_000`, `*secs > c::time_t::MIN`) were
  investigated (see "What was tried") and found not independently reachable from the documented
  bug mechanism, but this is a code-reading conclusion, not an empirically forced crash the way
  the `secs <= 0` case was - a maintainer who has observed macOS behavior beyond what the linked
  upstream writeup describes may know of a real path to either. Converting them from a hard abort
  into graceful degradation (e.g. clamping, or changing the public call sites to return
  `io::Result` instead of an infallible tuple) would be a larger, non-bounded API change, not
  attempted this session.
- Not independently confirmed on real Apple hardware - see "Fully reproducible: No" above for the
  specific, verified reason. A maintainer with access to real macOS hardware (or CI running on
  one) could re-run this candidate's `repro/` test (or a real `stat()`/`fstat()` call on a file
  touched a few hundred ms before the Unix epoch, if such a timestamp can still be set on current
  macOS/APFS) against the patched binary for a fully end-to-end confirmation.
- The sibling call sites that feed into this same function (`fix_negative_stat_nsecs` in
  `src/backend/libc/fs/syscalls.rs`, `fix_negative_timespec_nsecs` in
  `src/backend/libc/time/syscalls.rs`) were read and confirmed to call `fix_negative_nsecs`
  unconditionally with no additional guard of their own - the fix in the one shared function
  covers all of them, nothing project-specific to each call site was found that would need a
  separate patch.
