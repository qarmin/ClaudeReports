# readlink_embedded_nul_cstring_panic

`fs::readlink`'s success path (`src/fs/abs.rs`, `_readlink`) does `CString::new(buffer).unwrap()`
where `buffer` is the exact raw bytes the `readlink(2)` syscall wrote. Flagged as a candidate
because a normal `symlink(2)`-created target can never contain an embedded NUL (creation takes a
NUL-terminated C string), but `readlink(2)`'s *return* isn't guaranteed to respect that for every
filesystem - a FUSE daemon's own `readlink` callback, or a corrupted/crafted on-disk symlink
inode, can hand the kernel arbitrary bytes to copy back verbatim. Same bug shape as the
macOS-only sibling at `src/backend/libc/fs/syscalls.rs:2205-2211` (`getpath`'s `unwrap()` on a
NUL search), not independently investigated this session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `src/fs/abs.rs` `_readlink`, both backends' `readlink` syscall wrappers (`src/backend/linux_raw/fs/syscalls.rs:965`, `src/backend/libc/fs/syscalls.rs:287`) | Confirmed: neither backend validates or filters the bytes the syscall writes before they reach `CString::new(...).unwrap()`; the raw `nread`/buffer flow straight through |
| Real-FUSE-daemon feasibility check (the genuine second participant for this bug class) | `ls /dev/fuse`, `sudo -n true`, `ls /dev/loop*`, `unshare -rm` in this sandboxed container | Infeasible: no `/dev/fuse` device node, no root/`CAP_SYS_ADMIN` (`sudo` blocked by `no_new_privs`), no loop devices (rules out mounting a corrupted on-disk image too), unprivileged user namespaces blocked outright - see `output.txt` for exact commands/output |
| Manual boundary-value construction via the real public API, syscall boundary mocked for one magic path only (`mock.patch`), real `_readlink`/`CString::new(...).unwrap()` logic unmodified | Debug build, `cargo run --example` against the worktree, then `just test` from a completely fresh clone | Reproduced: panics with `NulError` at `src/fs/abs.rs:133:44`, 5/5 runs; also reproduced with the embedded NUL at the first byte and at the last byte before the terminator (2 more boundary variants, both panicked identically) |

## Status

**Confirmed**, reproduces 5/5 runs (debug build, deterministic - no layout/seed dependency; the
panic is driven entirely by the crafted buffer content, not by memory layout), plus 2/2 additional
boundary-value variants in the same class (NUL at offset 0, NUL at the last byte).

**Fully reproducible**: **No**, justified. The real second participant for this bug class is
either (a) a real FUSE daemon implementing its own `readlink` callback, or (b) a mounted
filesystem image whose on-disk symlink target was corrupted/crafted to contain an embedded NUL
(bypassing `symlink(2)`'s NUL-terminated-string constraint entirely). Both were attempted and
found genuinely infeasible in this sandboxed container, not merely inconvenient:
- `/dev/fuse` does not exist on this host at all (`ls /dev/fuse` -> "No such file or directory") -
  there is no FUSE communication channel to open regardless of privilege.
- No root and no `CAP_SYS_ADMIN` (`sudo -n true` fails: "the 'no new privileges' flag is set,
  which prevents sudo from running as root").
- No loop devices present (`/dev/loop*` -> "No such file or directory"), which rules out building
  and mounting a corrupted raw filesystem image as an alternative to FUSE.
- Unprivileged user-namespace creation (which can sometimes grant a mount namespace without real
  root) is itself blocked outright (`unshare -rm ...` -> "unshare failed: Operation not
  permitted").

This is the narrow, explicitly-justified exception in `CONFIRMATION.md` section 11 ("needs
hardware... or similar" - here, a device node and kernel privilege this container structurally
does not expose). Given that, `repro/` mocks only the single lowest-level syscall wrapper
(`backend::fs::syscalls::readlink` in the `linux_raw` backend) for one magic path, in a
`mock.patch` applied to a freshly cloned pristine checkout before building - every other line
exercised is the real, unmodified source: the real public `rustix::fs::readlink` entry point, the
real private `_readlink` loop, the real `assert!`/`resize`/`CString::new(...).unwrap()`. Every
other path through that same syscall wrapper still takes the real, unmodified `__NR_readlinkat`
syscall - only the one magic path is simulated.

## Class

crash

## Root cause

`_readlink` (`src/fs/abs.rs:118-139`) calls `backend::fs::syscalls::readlink(path, &mut buffer)`,
which on Linux is a thin wrapper directly around the `readlinkat`/`readlink` syscall (both
backends: `src/backend/linux_raw/fs/syscalls.rs:965`, `src/backend/libc/fs/syscalls.rs:287`).
Neither wrapper does anything but forward `buf`/`buf.len()` to the kernel and return the raw byte
count the kernel reports. Back in `_readlink`, once `nread < buffer.len()` (the common case - the
symlink target fit in the current buffer), the code does:

```rust
buffer.resize(nread, 0_u8);
return Ok(CString::new(buffer).unwrap());
```

`CString::new` returns `Err(NulError)` if its input contains any byte equal to `0` anywhere
except the position it would add itself as the terminator. The code assumes the kernel's returned
bytes can never contain an embedded `0`, which holds for the overwhelmingly common case (a
regular filesystem's `symlink(2)`-created target, itself created from a NUL-terminated C string)
but is not actually guaranteed by the `readlink(2)` kernel interface for every filesystem driver:
a FUSE filesystem's userspace daemon fully controls the bytes returned for its own `readlink`
callback (nothing in the FUSE kernel module/protocol NUL-filters or validates them before copying
them back to the caller), and a filesystem image whose on-disk symlink inode was corrupted or
hand-crafted directly (bypassing the `symlink(2)` syscall's own NUL-terminated-string input
constraint) can make an otherwise-ordinary filesystem driver report the same kind of bytes. Either
way, `CString::new(buffer)` then returns `Err`, and `.unwrap()` panics the calling thread.

## Introduced by

00edd161 "Add non-at filesystem functions." (2023-05-24) - this commit introduced `_readlink`
with the `CString::new(buffer).unwrap()` line exactly as it reads today; confirmed via `git log -S
"CString::new(buffer).unwrap()" -- src/fs/abs.rs`, which returns only this one commit.

## The fix

Changed the one line from `Ok(CString::new(buffer).unwrap())` to
`CString::new(buffer).map_err(|_cstr_err| io::Errno::ILSEQ)`, returning a clean `io::Result`
error instead of panicking - mirroring the exact same `map_err(|_r| io::Errno::ILSEQ)` convention
already used for an analogous kernel-controlled-bytes-to-`CString` conversion in
`src/thread/prctl.rs:84` (`prctl(PR_GET_NAME)`), so this isn't a novel error-handling choice for
the crate. Verified against 3 inputs in the same bug class (NUL in the middle, NUL as the first
byte, NUL as the last byte before the terminator) - all 3 go from panicking to a clean
`Err(Errno::ILSEQ)`, 3/3 runs each. See `patch.diff` (+6/-1 lines against `src/fs/abs.rs`).

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc"` - 151 passed, 1
  failed before the fix; 151 passed, 1 failed after the fix (identical). The one failure
  (`ioctl::test_int_setter`) is pre-existing and unrelated - this sandboxed container has no
  `/dev/net/tun` (see `___BUGHUNT___/README.md`'s toolchain table), confirmed present and
  identical both before and after this patch.

## What's not done / open questions

- The macOS-only sibling this candidate was explicitly compared against
  (`src/backend/libc/fs/syscalls.rs:2205-2211`, `getpath`'s own `unwrap()` on a NUL search) was
  **not** independently investigated this session - same bug shape, different platform/code path,
  worth its own candidate.
- No genuine end-to-end repro (real FUSE daemon, or a mounted corrupted on-disk image) was built,
  for the reasons stated above under "Fully reproducible". A future session with access to
  `/dev/fuse` (or root/`CAP_SYS_ADMIN` for mounting a crafted image) should be able to build one
  directly on top of this candidate's root-cause analysis - the triggering condition (an embedded
  NUL in the bytes `readlink(2)` returns) is precisely characterized here.
- Not checked whether any other `CString::new(...).unwrap()` site in the crate reads
  syscall-returned bytes with the same unchecked assumption (a crate-wide grep for
  `CString::new(` sites without `.map_err(` was not performed this session beyond the one
  already-known macOS sibling above) - worth a sweep candidate.
