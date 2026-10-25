# ioctl_fionread_sign_extension_u64

`src/io/ioctl.rs:92-97`, `ioctl_fionread` does `ioctl::ioctl(fd, ctl).map(|n| n as u64)` on a raw
`c_int` whose own doc comment admits it "may contain a wrapped value" once the real count no
longer fits in 32 bits. `n as u64` sign-extends a negative wrapped `i32` into a value near
`u64::MAX` instead of preserving the documented 32-bit wraparound (`n as u32 as u64`). This looked
suspicious as an arithmetic/cast bug (two integer widths, a negative-if-signed boundary) with a
doc comment that itself names the hazard but the code doesn't actually handle it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction, independent C program (`ioctl(2)` directly, bypassing rustix) | `gcc -O2`, a sparse regular file truncated to 3,000,000,000 bytes (apparent size > `i32::MAX`), `FIONREAD` via raw `ioctl()` | Confirmed the kernel's real behavior: writes back raw `c_int = -1294967296`, whose bit pattern reinterpreted as `u32` correctly recovers `3000000000` - exactly the "wrapped value" rustix's own doc comment describes |
| Standalone Rust repro against rustix's real public API (`rustix::io::ioctl_fionread`), release build, real fd from a real sparse file on the real filesystem | `cargo run --release`, same 3,000,000,000-byte sparse file, 5 repeat runs | Reproduced identically on all 5/5 runs: `ioctl_fionread()` returned `18446744072414584320` (near `u64::MAX`) instead of the documented-correct wrapped value `3000000000` |
| Same repro, two more boundary values in the same class (`2_147_483_648` = `i32::MAX`+1, the smallest value that wraps; `4_294_967_294` = `u32::MAX`-1) | `cargo run --release` | Both reproduced the bug identically before the fix, and both produced the correct wrapped value after the fix - confirms the fix isn't narrow to the one originally-found size |

Only one approach was needed (manual boundary-value construction) since the mechanism is pure
integer-width arithmetic with no memory-safety/concurrency angle - miri/ASAN/fuzzing would add
nothing here per `HEURISTICS.md`'s own guidance for this bug class.

## Status

**Confirmed**. Reproduces 5/5 runs, release build, real kernel/real syscall/real public API, no
flakiness - this is a deterministic bit-level truncation bug, not a layout/seed-dependent one.

**Fully reproducible**: **Yes**. The repro calls rustix's actual public `ioctl_fionread` entry
point on a real file descriptor. The "second participant" producing the wrapped `c_int` here is
the real Linux kernel's own `FIONREAD` handling for regular files (`(int)(i_size - f_pos)`,
verified independently against a real `ioctl(2)` call in a standalone C program, not assumed) -
nothing about the kernel's behavior was hand-constructed or faked; the only artificial part is
using `ftruncate` to create a sparse file with an apparent size that exceeds `i32::MAX`, which is
the cheapest real way to get the kernel to genuinely produce a wrapped value (actually queuing
>=2 GiB of real unread data in a pipe/socket buffer to trigger the same kernel code path for a
character/socket fd is impractical in a session's time/memory budget, but is the same underlying
kernel mechanism the doc comment is itself warning about - the regular-file path exercises the
identical `(int)`-truncation hazard through the same `ioctl_fionread` code, not a different one).

## Class

logic

## Root cause

`ioctl_fionread` (`src/io/ioctl.rs`) uses `ioctl::Getter::<{ c::FIONREAD }, c::c_int>` to receive
the kernel's raw `c_int` reply, then does `ioctl::ioctl(fd, ctl).map(|n| n as u64)`. When the real
count the kernel would report doesn't fit in a (signed) 32-bit `c_int`, the kernel truncates it to
32 bits when writing the reply (verified directly: a 3,000,000,000-byte apparent file size yields
a raw int of `-1294967296`, whose `u32` reinterpretation is `3000000000` - exactly the wrapped
value). Rust's `as u64` cast from `i32` to `u64` **sign-extends** a negative `i32`, producing a
value near `u64::MAX` (`18446744072414584320`) that bears no numeric relationship to either the
real count or the documented 32-bit-wrapped fallback. The fix is to go through `u32` first
(`n as u32 as u64`), which zero-extends instead and reproduces exactly the "wrapped value" the
function's own doc comment already promises callers.

## Introduced by

241da5395178c4236f8321d485d24b0c6e9f62ce "Set up a unified system for handling ioctl calls (#788)"
(2023-08-25). This commit merged the libc and linux_raw backends' separate `ioctl_fionread`
implementations into the current single `ioctl::Getter`-based code path with the plain
`.map(|n| n as u64)` cast - and in doing so **regressed** a fix the libc backend had carried since
1bf083188e3ae8378f98c88d16d68642f6755f69 "Don't use `try_into()` for conditions which can never
occur." (2021-06-16), which explicitly changed the libc backend to
`nread.assume_init() as libc::c_uint as u64` with the comment "The best we can do is convert it
back to a `u64` without sign-extending it back first" - i.e. the exact fix this candidate
re-applies. The linux_raw backend, however, never received that 2021 fix: its own `ioctl_fionread`
has used a bare `result.assume_init() as u64` since its earliest appearance in
17e7e3a3 "Port to linux-headers-sys" (2021-06-14) - so on linux_raw this exact bug has existed
essentially since rustix's inception, and the 2023 unification commit is what carried it over to
also cover the libc backend (undoing 1bf08318's fix for that backend) and is what produced the
exact line of code present at HEAD today.

## The fix

Changed `ioctl::ioctl(fd, ctl).map(|n| n as u64)` to `ioctl::ioctl(fd, ctl).map(|n| n as u32 as u64)`
in `src/io/ioctl.rs`, with a one-line comment explaining why (`patch.diff`, +5/-1 lines). This is
the same fix the libc backend already had in 2021 before being regressed, re-applied at the now-
unified call site so both backends get it again.

Verified: re-ran `repro/` against the patched tree - no longer reproduces (prints "did not
reproduce", exit 0) for the original 3,000,000,000-byte case and for two more boundary values in
the same class (`2_147_483_648`, `4_294_967_294`) - see `output.txt`.

- Project test suite (`cargo test`, default features, in the working tree): 22 passed, 0 failed,
  1 ignored (pre-existing, unrelated `#[ignore]` in `tests/backends.rs`) before the fix; 22 passed,
  0 failed, 1 ignored after the fix - identical.

## What's not done / open questions

- Only the regular-file path was used to make the kernel genuinely produce a wrapped `c_int`
  (via `ftruncate` to an oversized sparse file). The same hazard the doc comment describes also
  applies to a pipe/socket/character device with >=2 GiB of real pending data, which exercises the
  same `ioctl_fionread` code path but wasn't separately reproduced - doing so would require
  actually queuing that much real unread data in a kernel buffer, which is impractical in this
  session's time/memory budget and would not exercise any different code in rustix (the kernel's
  truncate-to-`c_int` behavior and rustix's subsequent cast are identical regardless of which fd
  type produced the oversized count).
- No other `ioctl::Getter<_, c::c_int>` call site in this file (or elsewhere in the crate) widens
  its result past `c_int`'s own width the way `ioctl_fionread` does - a quick check of the other
  functions in `src/io/ioctl.rs`/`src/fs/ioctl.rs`/`src/termios/ioctl.rs`/`src/process/ioctl.rs`
  shows they either return `bool`/`()`/narrower types or don't claim a "wrapped value" is possible
  in their own doc comments, so this specific sign-extension pattern appears isolated to
  `ioctl_fionread`. Not exhaustively re-audited beyond a grep for `as u64` adjacent to a `Getter`
  call in this session.
