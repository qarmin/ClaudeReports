# zero_len_buffer_query_convention_sweep

Follow-up to candidate 010: that candidate confirmed `fs::xattr`'s 6 get/list functions mishandle
a 0-length buffer because Linux's xattr syscalls follow a "size=0 means query-only, report the
real size without writing" convention that the wrapper doesn't clamp for. Backlog suspected other
subsystems in the crate might follow the same 0-length-buffer-as-query-mode convention with the
same missing-clamp bug.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Enumerate every public function generic over `Buf: Buffer<...>` (`grep -rn "Buf: Buffer<" src/`) | Whole crate | Exactly 16 functions total: `getrandom`, `read`/`pread`, `getxattr`/`lgetxattr`/`fgetxattr`/`listxattr`/`llistxattr`/`flistxattr` (already fixed by candidate 010), `epoll::wait`, `port::getn`, `kqueue::kevent`/`kevent_timespec`, `readlinkat_raw`, `recv`/`recvfrom` (already fixed by candidate 001) - this is the complete set of buffer-accepting entry points in the crate, since `Buffer` is a sealed trait implementable only by the crate's own blessed types. |
| Per-function real-syscall-semantics check (man pages + kernel source reasoning) for each of the 8 not-yet-covered functions | `getrandom(2)`, `read(2)`/`pread64(2)`, `epoll_wait(2)`, Solaris/illumos `port_getn`, BSD `kevent(2)`, `readlinkat(2)` | None follow a "0 means query, report unclamped true size" convention: `getrandom`/`read`/`pread` are all hard-capped by the kernel to never write or report more than the requested buffer length (a basic Linux I/O invariant, not just usual behavior); `epoll_wait`/`port_getn`/`kevent`'s returned count is inherently bounded by the caller-supplied max-events array length (the kernel physically cannot report more events than slots given); `readlinkat` always returns `min(true_target_length, buflen)` per kernel source reasoning (`fs/namei.c`'s `readlink_copy` uses `min(len, buflen)`) - unlike xattr, there is no "0 means give me the real size" mode. |
| Real repro: `rustix::fs::readlinkat_raw` called with a genuine 0-length `&mut [u8]` buffer against a real symlink on the real filesystem | `cargo run --release`, one real call | Returns a real `Errno` (`EINVAL`, "Invalid argument") - a clean error, not a bogus success or a fabricated length. Empirically confirms the kernel-source reasoning above: `readlinkat`'s 0-length-buffer behavior is nothing like xattr's query convention. |

## Status

**Not reproducible**. The "0-length buffer triggers a real, larger, unclamped size report"
convention confirmed in candidate 010 is specific to the xattr syscall family (`getxattr`/
`listxattr` and their `l`/`f` variants) - no other `Buffer`-consuming function in the crate shares
it. This is a complete sweep: `Buffer` is sealed, so every possible call site was enumerated (16
total), not just a sample.

## Root cause

N/A - no new bug found. The xattr family's zero-size-query convention is a POSIX/Linux-specific
idiom for that syscall family in particular (letting a caller determine an attribute's size before
allocating a properly-sized buffer) - it isn't a general "buffer size 0" convention shared across
Linux I/O syscalls. The other candidate functions either hard-cap their report at the buffer's own
length (I/O reads, `getrandom`) or are bounded by an array-length argument that mechanically cannot
be exceeded (event-polling APIs), so there was never a code path for the same class of bug to hide
in.

## Introduced by

N/A - not applicable, no new bug confirmed.

## The fix

N/A - no new bug found, nothing to fix.

## What's not done / open questions

- This sweep only covered functions generic over the `Buffer` trait. A handful of other rustix
  functions take a raw `&mut [u8]`/similar directly (not through `Buffer`) and separately decode a
  syscall-reported length - these weren't in scope here (candidate 010 and this sweep were both
  specifically about the `Buffer::assume_init` abstraction's own clamp, not every length-reporting
  syscall in the crate generally).
- None beyond the general caveat that this sweep is scoped to the `Buffer`-trait abstraction (see
  the note above about non-`Buffer` length-reporting call sites being out of scope).
