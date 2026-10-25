# getxattr_zero_len_buffer_bogus_length

`getxattr`/`lgetxattr`/`fgetxattr`/`listxattr`/`llistxattr`/`flistxattr` (`src/fs/xattr.rs`) all
call `Buf::assume_init(len)` where `len` is the raw syscall return value, with no clamp against the
buffer's own actual length. On Linux, passing a **zero-length buffer** to `getxattr`/`listxattr` is
the documented, standard idiom for "just tell me the size" (see `xattr(7)`: "if size is specified
as zero, these calls return the current size of the named extended attribute" - the value is not
written at all in that case). So a real, ordinary caller passing an empty buffer gets back a
`Buffer::Output` that lies about how many bytes were "initialized" - up to and including a full
`usize` claiming thousands of bytes were written into a genuinely empty (possibly zero-capacity,
even non-existent-backing) buffer. Found while sweeping for other `assume_init(min(len, ...))`-
shaped call sites per candidate 001's backlog note, and confirmed to be a distinct, previously
unconfirmed bug (not the same as 001/005, which are both about `net::recv*`, not `fs::*xattr`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/fs/xattr.rs`, `src/buffer.rs`'s `Buffer` trait impls for `&mut [T]` / `&mut [MaybeUninit<T>]` | Confirmed none of the 6 get/list xattr functions clamp the syscall-reported `len` against the buffer's own actual length before calling `assume_init(len)`; confirmed `&mut [T]`'s `assume_init` is a bare pass-through (`len` returned as-is, no bounds check at all), while `&mut [MaybeUninit<T>]`'s calls `split_at_mut(len)`, which panics if `len` exceeds the buffer's real length |
| Real repro, real file, real syscalls (`setxattr` then `getxattr`/`listxattr` with a genuine `&mut [u8; 0]` buffer against a real xattr set on a real tmpfs/overlayfs file) | `cargo run --release`, 6 cases (0-len buffer against 1/64/1024-byte attributes, a normal-sized-buffer regression check, the `MaybeUninit` variant, and `listxattr`) | 5/6 cases show the bug: `getxattr` returns the *real* attribute size (1, 64, or 1024) for a 0-length buffer instead of 0; `listxattr` does the same (returns 10 for a 0-length buffer); the `MaybeUninit` variant panics (`mid > len` in `buffer.rs:176`) instead of returning `(&[], &[])`. The one non-bug case (adequately-sized buffer) behaves correctly, confirming this is specific to the zero-length-buffer path, not a general defect. |
| Repeat runs | Same build, same repro, 5 consecutive runs against the pristine tree | 5/5 identical: `case1` returns `64` every run |

## Status

**Confirmed**. 5/5 runs, release build, real filesystem (this container's overlayfs-backed tmp
directory, which supports `user.*` xattrs). Deterministic - no layout/seed dependency, this is a
direct consequence of the zero-size-query Linux xattr convention, not a race or timing issue.

**Fully reproducible**: Yes. The repro calls the crate's real public `setxattr`/`getxattr`/
`listxattr` entry points against a real file with real xattrs on the real filesystem - no
hand-constructed struct/buffer state standing in for what a "second participant" would produce.
This is a single-participant finding (one process calling its own crate's public API against a
real external resource, the filesystem) - there's no "other side" to fake, per `CONFIRMATION.md`
section 11's carve-out for exactly this shape.

## Class

`logic` / API-contract violation with a downstream memory-safety implication: the function
signature and docs promise `Buffer::Output` (a `usize` for the `&mut [u8]` case) means "the number
of elements initialized," but the returned value can exceed the buffer's real capacity - any caller
that trusts this contract to justify unsafe indexing/slicing of their own buffer (a completely
reasonable thing to do, since the contract is the whole point of the `Buffer` abstraction) would be
handed a length wider than what they can safely read. Also demonstrated a panic (`&mut
[MaybeUninit<u8>]` case: `mid > len` in `buffer.rs:176`) - an unexpected process abort from what
looks like ordinary safe-Rust usage of a documented API, i.e. a DoS-shaped consequence of the same
root cause.

## Root cause

`getxattr`/`lgetxattr`/`fgetxattr`/`listxattr`/`llistxattr`/`flistxattr` (`src/fs/xattr.rs`) all
follow the same shape:

```rust
let len = unsafe { backend::fs::syscalls::getxattr(path, name, value.parts_mut())? };
unsafe { Ok(value.assume_init(len)) }
```

`len` here is exactly what the kernel returned from the real `getxattr`(2)/`listxattr`(2) syscall.
Per `xattr(7)`: "if size is specified as zero, these calls return the current size of the named
extended attribute (or the current size of the list of extended attribute names) which can be used
to estimate the size of a buffer... **The value is not written**." Since `value.parts_mut()`
forwards the buffer's *own* length as the syscall's `size` argument, passing an empty buffer
(`&mut [u8; 0]`, `Vec::new()`, an empty `&mut [MaybeUninit<u8>]`, etc.) genuinely triggers this
mode - not a hypothetical, but the officially documented way to query an attribute's size before
allocating. The syscall then reports the *real* (nonzero) attribute size while writing 0 bytes, and
`assume_init(len)` is called with that unclamped `len` - which for the `&mut [T]` `Buffer` impl
(`src/buffer.rs`) is a bare `len` pass-through with **no bounds check whatsoever**:

```rust
unsafe fn assume_init(self, len: usize) -> Self::Output {
    len  // returned verbatim - no check against self.len()
}
```

For the `&mut [MaybeUninit<T>]` impl, the same unclamped `len` is passed to `self.split_at_mut(len)`,
which panics if `len` exceeds the slice's real length - a crash rather than a silent lie, but still
a direct consequence of the same missing clamp.

This is the same underlying bug *class* as candidate 001 (`recv`/`recvfrom`: kernel reports a
length it didn't actually write, and the wrapper certifies it as valid without checking), just
triggered through a different Linux convention (the xattr zero-size-query idiom) rather than a
`RecvFlags::TRUNC`-style flag, and in a different subsystem entirely (`fs::xattr`, not `net`).

## Introduced by

45ee8d25 "Introduce a new `Buffer` trait. (#1290)" (2025-02-26) - same commit that introduced
candidate 001's bug; this commit added the `Buffer` abstraction and rewrote all six xattr functions
to use it without adding the length clamp `recv`/`recvfrom` also lacked at the time.

## The fix

Clamp the syscall-reported `len` to the buffer's own real length before calling `assume_init`, in
all six functions - the exact same `min(reported_len, buf_len)` pattern candidate 001 used for
`recv`/`recvfrom`:

```rust
let (_, buf_len) = value.parts_mut();
let len = unsafe { backend::fs::syscalls::getxattr(path, name, value.parts_mut())? };
let len = core::cmp::min(len, buf_len);
unsafe { Ok(value.assume_init(len)) }
```

(same shape applied to `lgetxattr`, `fgetxattr`, `listxattr`, `llistxattr`, `flistxattr`.)

Verified against 6 cases spanning the bug class (0-length buffer against 1-byte, 64-byte, and
1024-byte attributes; a normal adequately-sized buffer as a regression check that real reads still
work and return correct data; the `MaybeUninit` variant; and `listxattr`) - all 6 pass after the
fix, and the regression case (`case4`) confirms ordinary non-zero-length usage is completely
unaffected (still returns the real 32-byte value, byte-for-byte correct).

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc"`): identical
  before and after - 51+14+70+16 passed across the suite's test binaries, 1 pre-existing failure
  both before and after (`ioctl::test_int_setter`, fails because `/dev/net/tun` doesn't exist in
  this container - unrelated to this patch, documented in `___BUGHUNT___/README.md` and
  `projects/rustix.md`).

## What's not done / open questions

- Not checked: whether any *other* subsystem in the crate has the same "syscall's `size=0` means
  query-only, don't clamp the report" convention (xattr is a well-known example of this Linux
  idiom, but it's not unique to xattr in general POSIX APIs - e.g. some `ioctl`s follow a similar
  pattern). Worth a follow-up sweep specifically for "0-sized-buffer query" semantics elsewhere,
  distinct from the `RecvFlags::TRUNC`-shaped sweep candidate 001 already prompted.
- The Windows-only `net/wsa.rs` xattr-adjacent code and any BSD/Apple-specific xattr variants
  (`XATTR_NOFOLLOW` etc., not present on Linux) were not audited - this session only has a Linux
  environment to test against.
- No test was added to the project's own `tests/` in this session (per the "always revert" rule -
  a real test addition is left to whoever applies `patch.diff` for real).
