# timespec_to_methods_self_convention

`Timespec::to_sec_usec` (src/timespec.rs:210) and `Timespec::to_timeval` (src/timespec.rs:223,
plus a third, Windows-only sibling at line 234) take `&self` even though `Timespec` is `Copy` -
clippy's warn-by-default `clippy::wrong_self_convention` lint flags this, since a `to_*`-named
method conventionally consumes `self` by value when the receiver is `Copy`. Investigated to verify
this is a pure API-convention/style inconsistency and not a sign of a masked correctness issue.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo clippy` with no opt-in flag, real crate source, both backends | `--no-default-features --features std,all-apis` (linux_raw backend) and `--features std,all-apis,use-libc` (libc backend), stable 1.98.1 toolchain, this container | `wrong_self_convention` (`#[warn(...)]` on by default, no `-W` needed) fires on line 210 under both backends, and additionally on line 223 once the libc-backend-only `to_timeval` is compiled in. Reproduced 2/2 independent fresh-clone runs (see `output.txt`). Plain `cargo clippy` with the crate's *bare* `default = ["std"]` features does **not** reach either function at all - `to_sec_usec`/`to_timeval` are only called from the `net`/`event` modules, which require the `all-apis` feature (or a narrower equivalent) to be compiled in; without it clippy never analyzes the dead code. |
| Call-site / semantic review | `grep -rn "to_sec_usec\|to_timeval" src/` (6 call sites: `timespec.rs` internal, `backend/libc/net/sockopt.rs`, `backend/linux_raw/net/sockopt.rs` x2, `backend/libc/event/syscalls.rs` x2, `backend/libc/event/windows_syscalls.rs`) | Every call site invokes the method on either an owned `Timespec` value or a `&Timespec` reference via ordinary method-call syntax (`ts.to_sec_usec()`, `timeout.to_timeval()`); Rust's method resolution auto-derefs a `&Timespec` and copies it for a by-value receiver just as readily as it auto-refs an owned value for a `&self` receiver, so changing the receiver kind requires no call-site changes. No call site relies on NOT copying (no mutation through the reference, no identity/aliasing check), consistent with all six sites only ever reading fields to compute a derived value. |
| Boundary-value equivalence check (before/after the fix) | Existing `timespec::tests::test_to_timeval` (`tv_nsec: 999_999_999` and `500_000_000` boundaries) plus the full `fs`/`io` integration suites, run both before and after applying the self-convention fix, same toolchain/build | Identical test outcomes before and after (see "The fix" below) - changing `&self` to `self` is a pure signature change with no effect on the function bodies' arithmetic, confirming the lint is not masking any behavioral difference. |

## Status

**Confirmed**. The lint reproduces deterministically: 2/2 independent fresh-clone runs of
`cargo clippy` against the real, unmodified crate source at the pristine `base_commit` both flag
`src/timespec.rs:210` (and, with the libc backend compiled in, also `:223`) - see `output.txt` for
both runs' full captured output. This is a real, warn-by-default clippy diagnostic, not a
speculative audit note.

Separately, and most relevantly to this candidate's framing: **no masked correctness issue was
found**. `Timespec` is a plain `#[derive(Copy)]` struct of two integers with no `Drop`, no interior
mutability, and no aliasing concerns; `to_sec_usec`/`to_timeval` only read fields to compute a
derived value. Every one of the 6 real call sites works identically regardless of whether the
receiver is taken by reference or by value (see "What was tried" above), and the full test suite
(see below) passes identically before and after switching the signatures. The `&self` vs `self`
choice has zero behavioral consequence here - it is exactly what the candidate description expected
to verify: an API-convention inconsistency, not a correctness bug.

**Fully reproducible**: **Yes**. The repro runs the real, unmodified `cargo clippy` command
against the real crate source in a real checkout (no hand-authored stand-in) - see
`CONFIRMATION.md` section 11's "single external artifact" exception: `to_sec_usec`/`to_timeval` are
`pub(crate)` (never called by any external participant to fake), so the real source file handed to
the real `cargo clippy` invocation *is* the genuine trigger, with nothing else to impersonate.

## Class

`other` (API-convention / lint-only finding, no behavioral/correctness impact - see "Status" above).

## Root cause

`Timespec` was made `Copy` (it is a 16-byte pair of integers, cheaper to pass by value than by
reference), but `to_sec_usec`/`to_timeval`/the Windows-only `to_timeval` sibling were written taking
`&self`, following the general Rust convention for accessor-style methods rather than the
`to_*`-specific convention (which, per Rust API guidelines and clippy's own lint, expects `to_*` on
a `Copy` type to consume `self` by value, reserving `&self` for `as_*`/non-`Copy` `to_*`
conversions). No call site ever needed the borrow - it was simply never revisited since.

## Introduced by

`to_sec_usec` and its original `to_timeval` caller: 9d06f203 "Replace `TimeSpec` with `Timespec`"
(the function existed with this exact `&self` signature at the time the file was introduced/renamed
to its current form - `git log -S 'fn to_sec_usec' -- src/timespec.rs` shows no earlier signature
change). The Windows-only `to_timeval` sibling (line 234) was added later with the same `&self`
convention by copy-paste from the libc one. Given this is a style nit with no functional history to
trace precisely (the signature has been `&self` since the function's introduction and was never
deliberately chosen over `self` for any documented reason), a more precise single commit is not
worth pinning down further than "present since the function was introduced."

## The fix

Changed all three methods' receiver from `&self` to `self` (src/timespec.rs:210, 223, 234 - the
third being the `#[cfg(windows)]` sibling, unreachable from this Linux container but sharing the
exact same issue, fixed for consistency). No call site needed updating: Rust's method-call syntax
auto-derefs-and-copies a `&Timespec` receiver for a by-value method exactly as readily as it
auto-refs an owned value for a `&self` one.

- `cargo clippy --no-default-features --features std,all-apis,use-libc`: the `wrong_self_convention`
  warning on lines 210/223 is gone after the fix (confirmed via `just test`'s `reproduce-after` step
  and independently in-tree); `cargo build`/`cargo clippy` otherwise unchanged (no new warnings).
- Project test suite, `--no-default-features --features std,all-apis,use-libc`, same toolchain,
  run both before (pristine) and after (patched) applying the fix in this worktree:
  - `cargo test --lib timespec`: 3 tests before -> 4 tests after (`test_to_timeval` only runs under
    `cfg(any(libc, wasi, windows))`, which `use-libc` enables) - all passed, identical both times.
  - `cargo test --no-default-features --features std,all-apis,use-libc` (full workspace, all test
    binaries): **70 passed / 0 failed** in the `fs` test binary and **16 passed / 1 failed** in the
    `io` test binary, identical before and after. The one failure, `io::ioctl::test_int_setter`
    (`Os { code: 2, kind: NotFound, ... }`), is a pre-existing environment issue (a missing
    `/dev`-level device node in this sandboxed container) unrelated to `Timespec` or this patch -
    confirmed identical on the pristine tree with no patch applied at all.

## What's not done / open questions

- This is a public-API-adjacent but not actually public change (`to_sec_usec`/`to_timeval` are
  `pub(crate)`), so it carries no semver implications and needs no deprecation period - a
  maintainer can take it as a plain internal cleanup.
- Not independently re-checked whether any *other* `pub(crate)` or `pub` `to_*`-named method on a
  `Copy` type elsewhere in the crate has the same `&self` inconsistency; this candidate was scoped
  to the two (three, counting the Windows sibling) methods named in the original suspect. A
  `cargo clippy --features all-apis,use-libc` sweep for `wrong_self_convention` across the whole
  crate (as done here, just not exhaustively enumerated beyond this file) would catch any sibling
  instance quickly if someone wants to extend this cleanup.
