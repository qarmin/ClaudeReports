# dir_read_zero_reclen_stall

`Dir::read` (`src/backend/linux_raw/fs/dir.rs:176-181`) reads a `getdents64` record's `d_reclen`
field via an unaligned `u16` load and only checks `assert!(self.buf.len() - pos >= d_reclen as
usize)` - it never checks `d_reclen > 0`. A `d_reclen` of 0 passes that assert trivially
(`self.buf.len() - pos >= 0` is always true) and leaves `self.pos += 0`, so the next call to
`read()`/`.next()` re-parses the exact same bytes - forever, with `any_errors` never set. Flagged
as a candidate because this is the kind of "untrusted size drives index/arithmetic in a loop with
no `> 0` check" pattern `HEURISTICS.md` calls out, in code that already treats the kernel's VFS as
trusted (correctly, for a real kernel) but has no defense if that trust is misplaced (a buggy or
malicious filesystem backend - FUSE, 9p, a corrupted on-disk image surfaced through some other
backend).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value reasoning (code reading) | Traced `d_reclen=0` through `read()`: assert passes, `self.pos` unchanged, top-of-function `if self.any_errors { return None }` never trips since `any_errors` is never set on this path. | Confirmed the mechanism purely by reading - matches the candidate description exactly. |
| Real-FUSE end-to-end repro (the only way to make a genuine kernel `getdents64` call actually return `d_reclen=0`, since the real VFS never does) | Checked `/dev/fuse` (absent), `CAP_SYS_ADMIN` (`capsh --print` lists it explicitly excluded from the bounding set), `sudo`/`mknod`/`modprobe fuse` (all blocked - "no new privileges" flag set, no root), `strace`/`apt-get install fuse3`/`strace` (no package-manager access, no network to crates.io either for a `ptrace`-based alternative) | **Infeasible in this sandboxed container** - no path found to make a real backend return a malformed `getdents64` record. See "Fully reproducible" below. |
| In-crate `#[cfg(test)]` test driving the real `Dir::read_from`/`Iterator::next` public entry point, reaching into `Dir`'s private fields only to corrupt the 2 `d_reclen` bytes of a record whose rest of the bytes came from one real `getdents64` syscall (same technique the file's own pre-existing `dir_iterator_handles_io_errors` test already uses for a different private-field manipulation) | debug build, `--features=all-apis`, `cargo test --lib -- backend::fs::dir::tests::dir_read_zero_reclen_stalls_forever --exact`, 5 repeat runs | **Reproduces 5/5 runs**, bit-for-bit identical panic message every time: `Dir::read` returns the exact same `DirEntry` for all 50,000 iterations attempted, `self.pos` never advances, `any_errors` never set. |

## Status

**Confirmed**

Reproduces 5/5 runs, debug build, `--features=all-apis`, deterministic (bit-for-bit identical
panic message each run - no flakiness, no dependency on heap layout/ASLR/hashmap order).

**Fully reproducible**: **No** - justified.

The bug is specifically about how `Dir::read` (one participant) reacts to a malformed
`getdents64` record produced by the kernel/filesystem-backend syscall layer (the other
participant, per CONFIRMATION.md section 11's framing) - and the real Linux VFS never emits
`d_reclen == 0`, so demonstrating this for real requires an actually-misbehaving backend (a
broken FUSE/9p driver, or a corrupted on-disk image surfaced through one). This candidate's own
description already flags that caveat. Building a real one in this session's sandboxed container
was attempted and found genuinely infeasible, not merely inconvenient:
- No `/dev/fuse` device node exists.
- No `CAP_SYS_ADMIN` - `capsh --print`'s "Current IAB" line explicitly lists `cap_sys_admin`
  among the capabilities excluded from the bounding set.
- `sudo` (for `mknod`/`modprobe fuse`) is blocked outright: "The 'no new privileges' flag is set,
  which prevents sudo from running as root."
- No package-manager access to install `fuse3`/`strace` tooling (`apt-get install` fails with
  "Could not open lock file ... Permission denied", not root).
- No network to `crates.io` either (`curl -sI https://crates.io` → HTTP 403), which would have
  been needed to pull in a `ptrace`-based syscall-return fault injector as a fallback way to
  corrupt a real `getdents64` reply in-flight without a real FUSE mount (`libc`/`nix` crates were
  available from the local registry cache, but writing and debugging a ptrace harness from
  scratch was judged a worse use of a single candidate's budget than the approach actually taken,
  once the FUSE route was ruled out).

Per CONFIRMATION.md section 11's explicit exception ("a finding that's clearly real from code
reading alone but where building a genuine end-to-end repro is impractical or infeasible in the
time available... needs hardware... may still be written up"): this is written up as Confirmed
using the closest practical substitute instead - `repro/repro_test.patch` drives the real,
unmodified `Dir::read_from`/`Iterator::next` public entry point against a `buf` that *was*
populated by one real `getdents64` syscall on a real directory, and only overwrites the 2 bytes of
the one `d_reclen` field a buggy backend would have gotten wrong (everything else in the buffer -
`d_ino`, `d_off`, `d_type`, `d_name`, every other record - is exactly what the real kernel
returned). This reuses the exact same "reach inside `Dir`'s private fields for a specific
fault-injection scenario" technique the file's own pre-existing `dir_iterator_handles_io_errors`
test already applies (there: swap `dir.fd` for a plain file via `dup2` to force a real
`getdents64` error) - it is not a wholesale hand-authored buffer standing in for the entire
syscall result, only a single targeted field.

## Class

logic (unbounded-loop / denial-of-service via a missing invariant check on untrusted-but-normally-
trusted kernel data)

## Root cause

`Dir::read`'s iteration-advance logic is `self.pos += d_reclen as usize` with no lower bound on
`d_reclen`. The surrounding `assert!(self.buf.len() - pos >= d_reclen as usize)` only guards
against reading past the end of the buffer (an upper bound); it says nothing about `d_reclen`
being large enough to make progress. Combined with `Dir::read`'s own early-exit check (`if
self.any_errors { return None; }`, which is never set on this path) and `Iterator::next` just
delegating straight to `read`, a `d_reclen` of 0 is a fixed point: every subsequent call
re-derives the identical `pos`, re-parses the identical bytes, and yields the identical
`DirEntry`, with nothing in the loop ever able to break out of it on its own. The kernel's own VFS
(`fs/readdir.c`) always emits a `d_reclen` that accounts for at least the fixed-size header plus a
NUL-padded name, so this is unreachable from any correctly-behaving backend - but nothing in this
function defends against a backend that doesn't uphold that contract.

## Introduced by

17e7e3a3 "Port to linux-headers-sys" (2021-06-08) - the first commit found (via `git log -S
reclen`/`git log -S d_reclen` across the whole history) to implement `linux_raw`'s own
`linux_dirent64` field-offset parsing with an unaligned reclen read and an upper-bound-only
`assert!(self.buf.len() - pos >= reclen)`, already lacking a `reclen > 0` check at that point. The
file has been reorganized/renamed several times since (`src/imp/linux_raw/fs/dir.rs` ->
`src/backend/linux_raw/fs/dir.rs` across commits like 1d0dfeac, "Reorganize the codebase to handle
linux_raw and libc consistently") but the missing check was carried through unchanged every time;
no earlier commit in the repository's history touches a `reclen`/`d_reclen` variable at all
(checked with `git log -S reclen --all` and `git log -S d_reclen --all`, both bottom out at
17e7e3a3).

## The fix

Add an explicit `d_reclen == 0` check right after the unaligned load, before it's used to advance
`self.pos`: set `any_errors = true` (matching every other error path in this function) and return
`Some(Err(io::Errno::BADMSG))` - `EBADMSG` ("bad message") is the existing error code this crate
already uses elsewhere for "the data doesn't have the structure it's supposed to", and is already
imported/available via `io::Errno` with no new dependency. This makes a zero `d_reclen` behave
exactly like every other "the directory stream is corrupted" case `Dir::read` already handles
(e.g. the pre-existing `dir_iterator_handles_io_errors` test, which forces a real `getdents64`
error a different way): the iterator now stops cleanly with an `Err` on the first bad record
instead of spinning forever. +9/-0 lines, `src/backend/linux_raw/fs/dir.rs` only.

This is the only concretely-constructible boundary value in this bug's class - `d_reclen` either
is 0 (the exact fixed point this candidate is about) or is not, there is no adjacent "near 0"
value that produces a distinct variant of the same stall (a `d_reclen` of 1, for example, *does*
advance `pos`, just by a wrong, misaligned amount - a different bug, out of this candidate's
scope and not something the fix above needs to handle). Per CONFIRMATION.md section 7, the fix was
re-verified against this exact value in multiple contexts: with the corrupted record positioned at
different buffer offsets across repeat runs (the "next" entry after an already-consumed one varies
by real directory-entry ordering between runs/files involved), and it held in every one of the 5
repeat runs.

Before -> after, re-running `repro/`'s test (`cargo test --features=all-apis --lib --
backend::fs::dir::tests::dir_read_zero_reclen_stalls_forever --exact`):
- Before (pristine): panics every time with `REPRODUCED: Dir::read returned the same unchanged
  entry for 50000 straight iterations (d_reclen=0 stall)` - 5/5 runs.
- After (patch.diff applied): `ok` every time (the test's early-return-on-`Err` branch fires
  immediately on the very first `.next()` call after the corruption) - 5/5 runs.

Project test suite (`cargo test --features=all-apis --workspace --no-fail-fast`), with the repro
test included in both counts for the "after" run (it's part of the same file):
- Before: **413 passed, 1 failed, 5 ignored**.
- After: **414 passed, 1 failed, 5 ignored** (the +1 passed is the new repro test itself; no other
  change).
- The one failure in both runs, `ioctl::test_int_setter` ("No such file or directory"), is a
  pre-existing, unrelated environmental failure - this container has no `/dev/net/tun`, which that
  test depends on (already documented in this workspace's own `README.md` from an earlier
  candidate this session). No new failures introduced by the fix.

## What's not done / open questions

- `Dir::read`'s unrelated, pre-existing `assert!(self.buf.len() - pos >= d_reclen as usize)`
  (upper-bound check: a `d_reclen` *larger* than the remaining buffer) still panics rather than
  returning a clean `Err` - out of scope for this candidate (a crash, not a stall) but worth a
  separate look since it's the same untrusted-field-no-validation code path one line below the
  fix here.
- The `libc`-backend equivalent directory-iteration code (if any similarly hand-parses `dirent`
  records rather than delegating to `libc::readdir`) was not checked for the same class of issue -
  this candidate only covers `src/backend/linux_raw/fs/dir.rs`.
- A longer, dedicated fuzzing campaign (via `fuzz-bug-repro`, run separately) feeding arbitrary
  byte buffers directly into a test-only `Dir`-construction helper could surface further
  `getdents64`-field-trust issues in this same function (e.g. `d_off`/`d_type`/`d_ino` are loaded
  but never range-checked either, though nothing downstream currently indexes with them the way
  `d_reclen` is used to advance `pos`).
