# flock_transmute_not_reproducible

`src/process/types.rs::Flock::from_raw_unchecked` does `transmute::<i16, FlockType>(raw_fl.l_type)`
and `transmute::<i16, FlockOffsetType>(raw_fl.l_whence)` (i32 variants on hurd/x86) on the result of
a real `fcntl(F_GETLK)` syscall. Both `FlockType` and `FlockOffsetType` are fieldless, exhaustively-
enumerated `#[repr(i16)]` enums with exactly 3 valid discriminants each - transmuting an
out-of-range integer into either is immediate, real Undefined Behavior (not just a logic bug), so
this looked like a strong candidate: found via discovery (grep for `transmute` across the crate,
per `HEURISTICS.md`'s "`unsafe` blocks: transmute" pattern) after the session's backlog ran dry.
Only `l_type` is defensively checked before the transmute (`curr_lock.l_type == F_UNLCK` is handled
separately, without a transmute, before falling into the `from_raw_unchecked` branch); `l_whence`
is transmuted with **no check at all**.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/process/types.rs` (`Flock::from_raw_unchecked`, `FlockType`, `FlockOffsetType` definitions), both call sites (`src/backend/{libc,linux_raw}/process/syscalls.rs::fcntl_getlk`) | Confirmed both backends call a real `fcntl(F_GETLK)`/`fcntl64(F_GETLK64)` syscall, check `l_type == F_UNLCK` first (handled without transmute, returns `None`), then transmute the *remaining* `l_type` value (necessarily `F_RDLCK` or `F_WRLCK` if the kernel behaves) and the **completely unchecked** `l_whence` into `FlockOffsetType`. Per Linux kernel source reasoning (`fs/locks.c`'s POSIX-lock-to-flock conversion path always writes `l_whence = SEEK_SET` on F_GETLK, since internal lock ranges are tracked as absolute offsets), `l_whence` should always come back as `SEEK_SET` (0) - a valid discriminant - on genuine Linux, but this is a kernel-behavior assumption, not something rustix's own code enforces or checks. |
| Real repro: two real, separate OS processes (`fork()`, child holds a genuine `F_SETLK` POSIX record lock via real libc `fcntl()`, parent calls the crate's real public `fcntl_getlk` after synchronizing over a pipe) | `cargo run --release`, default (`linux_raw`) backend, 1 run with a real cross-process lock conflict | Kernel reported the conflicting lock correctly: `Flock { start: 0, length: 0, pid: Some(Pid(child)), typ: WriteLock, offset_type: Set }` - `l_whence` came back as `SEEK_SET` exactly as kernel-source reasoning predicted, and `l_type` as `F_WRLCK`. No transmute of an out-of-range value observed; both values landed on valid, expected discriminants. |
| Same real repro, re-run under the `libc` backend (`RUSTFLAGS="--cfg rustix_no_linux_raw"`, `use-libc` feature) | `cargo run --release`, 1 run with a real cross-process lock conflict | Identical result: `Flock { start: 0, length: 0, pid: Some(Pid(child)), typ: WriteLock, offset_type: Set }` - same kernel, same real `fcntl(F_GETLK)` syscall reached through libc's wrapper instead of rustix's own inline asm, same valid discriminants observed. |

## Status

**Not reproducible**. The transmute's safety invariant (both `l_type` and `l_whence` are always one
of their type's valid discriminants) is not checked by rustix's own code, but holds in practice
because the real Linux kernel's `fcntl(F_GETLK)` implementation only ever writes back the specific
constant values these enums were designed to represent (`F_RDLCK`/`F_WRLCK` for `l_type` after the
`F_UNLCK` case is already excluded; always `SEEK_SET` for `l_whence`). Confirmed empirically with a
genuine two-process lock conflict, not just by reasoning about kernel source - `l_whence` really
does come back as `SEEK_SET` for a real conflicting lock, matching the kernel-source-level
prediction.

## Root cause

N/A - not a bug as of this checkout, on this kernel. Same general shape as candidate 011's "trust
the syscall return" finding: the invariant these unchecked transmutes depend on is a kernel ABI
guarantee, not user/attacker-controlled data, so there's no path for an adversarial value to reach
`from_raw_unchecked`. Structurally riskier than 011 in one respect worth flagging even though it
didn't reproduce: this uses `transmute` directly into an enum type, which is real, non-recoverable
UB the instant the invariant is ever violated (no `debug_assert!` at all to catch a violation even
in a debug build, unlike 011's `errno.rs` functions which at least assert in debug builds before
the release-only `assume()`).

## Introduced by

N/A - not applicable, no bug confirmed. (For reference, `Flock`/`FlockType`/`FlockOffsetType` and
this transmute pattern were both present at the time `fcntl_getlk` was added; not separately
bisected since there's no bug to date.)

## The fix

N/A - no bug found, nothing to fix. If a defensive posture were wanted despite the kernel guarantee
holding, `l_whence` could be validated with a `match`/`TryFrom` instead of a bare `transmute` (at
the cost of a small amount of code / a theoretical `Err` case that should be unreachable in
practice) - not implemented here since there's no confirmed defect motivating a patch this session,
per `SKILL.md`'s guidance not to force a fix onto a suspect that didn't pan out.

## What's not done / open questions

- Not verified on the `hurd`/`x86` variant (`transmute::<i32, ...>`), which needs different
  hardware/a different kernel entirely - out of reach in this container.
- This is a "kernel invariant, not a rustix bug" result on Linux specifically; a future session with
  access to a non-Linux POSIX kernel `libc` backend target (e.g. a BSD) could in principle check
  whether that kernel's own F_GETLK implementation makes the same `l_whence = SEEK_SET` guarantee -
  not checked here.
