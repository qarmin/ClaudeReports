# sockaddr_unix_addr_rs_bsd_illumos_len_underflow

`src/backend/libc/net/addr.rs`'s `path_bytes()` (line 177), `bytes()` (224-225), `PartialEq::eq`
(241-242), `Ord::cmp` (261-263) and `Hash::hash` (271-272) all compute
`self.len() - offsetof_sun_path()` with no clamp. Candidate 020 cleared the analogous concern for
`read_sockaddr_unix`'s own internal slicing on Linux, but explicitly flagged it never tested a
non-Linux `libc`-backend target, since `read_sockaddr.rs:204-208`'s own comment says FreeBSD/illumos
"sometimes set the length to longer than the length of the NUL-terminated string" - i.e. may not
enforce `addrlen <= sizeof(sockaddr_un)` the way Linux does. This candidate checks whether that
BSD/illumos quirk can propagate into `self.len()` on a `SocketAddrUnix` built from a real kernel
round-trip, underflowing in these `addr.rs` methods specifically (a different call path/file than
020 checked).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: traced every call site that produces a `SocketAddrUnix` from kernel data | `grep -rn "SocketAddrUnix::new\|new_abstract_name\|new_unnamed"` across both backends; read `src/backend/libc/net/read_sockaddr.rs:180-234` and `src/backend/libc/net/addr.rs:30-105` | Found the field `self.len()`/`addr_len()` that `path_bytes()`/`bytes()`/`eq`/`cmp`/`hash` all read is never the raw kernel `addrlen`. Every place that builds a `SocketAddrUnix` from OS data (`read_sockaddr_unix`, used by `bind`+`getsockname`, `connect`+`acceptfrom`, `getpeername`, and the public `TryFrom<SocketAddrAny>`) routes the decoded bytes back through one of the three constructors (`SocketAddrUnix::new`, `::new_abstract_name`, `::new_unnamed`), each of which recomputes its own `len`/`sun_len` field as `offsetof_sun_path() + bytes_actually_stored.len()` (or exactly `offsetof_sun_path()` for empty/unnamed) and independently bounds-checks against `unix.sun_path.len()` (returning `Errno::NAMETOOLONG` if exceeded) before ever constructing the struct. The BSD/illumos quirk (`decode.sun_path`'s length claim exceeding the true NUL-terminated string) is handled *before* this point, inside `read_sockaddr_unix` itself, by searching for the actual NUL (`decode.sun_path.iter().position(\|b\| *b == 0)`) and trimming to that - the raw, possibly-inflated kernel `addrlen` is used only as a slicing bound on `decode.sun_path` inside `read_sockaddr_unix` (already guarded by that function's own `assert!(len >= offsetof_sun_path)` and NUL search), never stored directly as the resulting object's own `len` |
| Cross-compile type-check of the exact `cfg(bsd)`/`cfg(solarish)` code paths (`rustup target add x86_64-unknown-freebsd`; `x86_64-unknown-illumos` was already installed) | `cargo check --target x86_64-unknown-illumos --no-default-features --features="std,net,use-libc,all-apis"` and the same for `--target x86_64-unknown-freebsd`, against this worktree at base commit 287214b8 | Both targets type-check cleanly (`Finished dev profile ... 0.0Xs`, no errors) - confirms `_new()`/`new_abstract_name()`'s invariant-establishing arithmetic (`(offsetof_sun_path() + bytes.len()).try_into().unwrap()`, stored in `unix.sun_len` under `cfg(bsd)` instead of a separate `len` field) is the *same* logic under those cfgs, not something BSD/illumos-specific that could behave differently - only the storage location of the computed length differs, not how it's computed or bounds-checked |
| Boundary-value construction + a real kernel round-trip, executed on Linux with the `libc` backend force-selected (`RUSTFLAGS=--cfg=rustix_use_libc`, same backend-forcing technique candidate 020 used) - standalone binary at `investigation/bincheck_main.rs` | `cargo run` (debug, overflow-checked arithmetic) and `cargo run --release`, both profiles; inputs: empty path, Linux abstract-name at length 0 and the max 107, filesystem path at the max-with-NUL length (107) and the exact-fit-no-NUL length (108, since `bytes.len() > sun_path.len()` is `108 > 108 == false`), plus a real `socket`+`bind`+`getsockname` round-trip on an actual `AF_UNIX` path socket, each followed by calling `path_bytes()`, `Hash::hash`, `PartialEq::eq`, and `Ord::cmp` - the exact methods under suspicion | `ALL BOUNDARY/ROUND-TRIP CHECKS PASSED, NO PANIC, NO UNDERFLOW` in both debug and release profiles - no subtraction underflow (which the debug profile's default overflow-checked arithmetic would have caught as a panic, not a silent wraparound) on any boundary value tried, including the real kernel-round-tripped address |

## Status

**Not reproducible.** Covered: full code-reading trace of every `SocketAddrUnix`-from-kernel-data
call site (both backends); a cross-compile type-check of the literal `cfg(bsd)`/`cfg(solarish)`
branches of the relevant constructors against `x86_64-unknown-freebsd` and `x86_64-unknown-illumos`;
and an executable boundary-value + real-kernel-round-trip check (debug and release profiles) against
the `libc` backend on this container's Linux kernel. No underflow signal appeared in any of the
three.

**Fully reproducible**: N/A (no reproduction to report).

## Root cause

Not applicable - no bug confirmed. The premise in the candidate description conflates two different
"lengths": the raw, potentially-untrustworthy `addrlen` the kernel reports on the wrapping
`SocketAddrAny`/`sockaddr_un` (which is what `read_sockaddr.rs:204-208`'s BSD/illumos comment is
about, and which is used only transiently inside `read_sockaddr_unix` to slice `decode.sun_path`),
and `self.len()` as read by `addr.rs`'s `path_bytes()`/`bytes()`/`eq`/`cmp`/`hash` on the final
constructed `SocketAddrUnix` object (which is a *different*, self-computed value - `addr_len()`
returns either a dedicated `len: c::socklen_t` field on non-BSD targets, or `unix.sun_len` on BSD,
and both are set exclusively inside `_new()`/`new_abstract_name()`/`new_unnamed()` from the length of
the bytes those functions actually write into `sun_path`, never copied verbatim from a kernel-
supplied `addrlen`). Because `read_sockaddr_unix` always reconstructs a `SocketAddrUnix` through one
of those three constructors - including on the BSD/illumos branch, which explicitly trims
`decode.sun_path` to the position of its first NUL byte before calling `SocketAddrUnix::new` on the
trimmed slice - the resulting `self.len()` is structurally guaranteed to be at least
`offsetof_sun_path()` (and at most `offsetof_sun_path() + sun_path.len()`) regardless of how
unreliable the kernel's own raw `addrlen` was. There is no code path, on any target this crate
supports, where a `SocketAddrUnix`'s own `len`/`sun_len` field is ever set directly from an untrusted
external value without going through this bounds-checked reconstruction.

## Introduced by

N/A - no bug confirmed, nothing to bisect.

## The fix

N/A - no bug found, nothing to fix. No `patch.diff` in this folder.

- Project test suite: not run as part of this candidate (no fix to validate; see "Candidate loop"
  step 7 - mandatory before/after test-suite runs apply only when a fix was applied and needs
  verifying). N/A.

## What's not done / open questions

- No genuine FreeBSD or illumos kernel/VM was available in this container to actually exercise a
  real `bind`+`getsockname` round-trip on those OSes themselves (the same limitation 020 noted) -
  this candidate instead closes the gap analytically: by showing the suspect field is never
  populated from raw kernel data on *any* target (the reconstruction-through-constructors pattern
  is OS-independent Rust code, not conditioned on which kernel is underneath), a real BSD/illumos
  kernel actually misbehaving the way the code comment describes can't reach `self.len()` in a bad
  state in the first place. A future session with real BSD/illumos hardware/VM access could still
  repeat 020's raw C-level kernel-boundary probe there for extra confidence, but it would be
  confirming the same structural guarantee, not a different one.
- Did not separately fuzz `SocketAddrUnix::new`/`new_abstract_name` with property-based random
  lengths beyond the explicit boundary values tried (0, 1, max-1, max, max-with-no-NUL) - the
  invariant proven (every constructor either returns `Err(NAMETOOLONG)` or a `len` in
  `[offsetof_sun_path(), offsetof_sun_path() + sun_path.len()]`) is a simple enough arithmetic
  argument from reading `_new()`/`new_abstract_name()`'s bodies that a wider fuzz sweep looked
  unlikely to add signal; flagged here rather than silently skipped.
