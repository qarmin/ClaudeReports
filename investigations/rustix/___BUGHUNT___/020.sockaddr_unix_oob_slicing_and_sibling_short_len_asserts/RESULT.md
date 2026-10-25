# sockaddr_unix_oob_slicing_and_sibling_short_len_asserts

`read_sockaddr_unix` in both `src/backend/linux_raw/net/read_sockaddr.rs:96-124` and
`src/backend/libc/net/read_sockaddr.rs:180-230` indexes/slices the fixed `[c_char; 108]` `sun_path`
array using `addr.addr_len()` (an untrusted-looking `u32` that `SocketAddrAny::new`/`read` only
bound-check against `size_of::<SocketAddrStorage>()`, ~128 bytes, not against `sizeof(sockaddr_un)`,
110 bytes) with no further clamp. If a `SocketAddrAny` tagged `AF_UNIX` ever carried a length
between 111 and 128, `&decode.sun_path[1..len - offsetof_sun_path]` (abstract-name branch) or
`decode.sun_path[len - 1 - offsetof_sun_path]` (filesystem-path branch) would index past the
108-byte array and panic. The sibling `assert!(addr.addr_len() as usize >= size_of::<T>())` guards
in the same files (`read_sockaddr_v4`/`_v6`/`_xdp`/`_netlink`) were flagged as the mirror-image
concern: a hard panic if the OS ever reported a *shorter* `addr_len` than the target struct.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Raw C-level kernel-boundary probe, bypassing rustix entirely (`investigation/kernel_boundary_probe.c`) | Real `AF_UNIX` `bind()`/`getsockname()` via plain libc on this container's kernel (`7.0.0-34-generic`): (1) abstract name, `addrlen=110` (max `unix_mkname` allows) with trailing garbage bytes after the name - kernel stores and reports back exactly `110`, never more; (2) `addrlen=128` (`sizeof(sockaddr_storage)`) passed to `bind()` directly - kernel's own `unix_mkname` rejects it outright with `EINVAL`; (3) filesystem path with trailing garbage after the NUL and `addrlen=110` - kernel re-clamps the stored/reported length down to `strlen(path)+1+offsetof_sun_path` (33 in this run), discarding the garbage | Real kernel enforces `addrlen <= sizeof(struct sockaddr_un)` (110) for `AF_UNIX` at `bind()` time, and recomputes a filesystem path's length from its real `strlen` rather than trusting a caller-supplied `addrlen` - confirms no genuine kernel-written `AF_UNIX` sockaddr can exceed 110 bytes on this kernel |
| Real end-to-end boundary sweep through rustix's own public safe API (`bind`/`getsockname`), both backends (`investigation/sockaddr_sweep_main.rs`) | `cargo run --release --bin sockaddr_sweep`, `rustix` as a path dependency (`features = ["std","net","all-apis","use-libc"]`), default (`linux_raw`) backend and forced `libc` backend (`RUSTFLAGS="--cfg rustix_no_linux_raw"`); abstract-name lengths swept `0..=108` (108 correctly rejected at construction with `NAMETOOLONG`), filesystem-path lengths swept at `{50,100,106,107}` (boundary values `0`,`1` couldn't be hit exactly given this environment's `$TMPDIR` length, noted and skipped rather than faked) | `112 oks / 0 panics` on **both** backends - every real bound address round-tripped through `read_sockaddr_unix` via the actual `getsockname()` syscall path without panicking, across the full legitimate length range |
| Real two-participant test: server thread + client thread, real `connect()`/`listen()`/`acceptfrom()`/`getpeername()` over a real `AF_UNIX` socket, both ends bound to the maximal legitimate abstract name (107 bytes) (`investigation/bin_accept.rs`) | `cargo run --release --bin bin_accept`, default backend, 1 run | `acceptfrom`'s own peer address and a separate `getpeername()` call both reported `addr_len = 110` (the real maximum) and decoded cleanly via `read_sockaddr_unix` with no panic - exercises the `accept`/`getpeername` call sites specifically, not just `bind`/`getsockname` |

## Status

**Not reproducible.** Every current call site in this crate that produces a `SocketAddrAny` tagged
`AF_UNIX` (`bind`+`getsockname`, `connect`+`acceptfrom`, `getpeername`, on both the `linux_raw` and
`libc` backends) goes through a real syscall, and the real Linux kernel itself enforces
`addrlen <= sizeof(struct sockaddr_un)` (110 bytes) for `AF_UNIX` at the point an address is stored
(`unix_mkname` in `net/unix/af_unix.c`: rejects `bind()` outright with `-EINVAL` if the caller's
`addrlen` exceeds `sizeof(struct sockaddr_un)`, and independently re-clamps a filesystem path's
*reported* length to its real `strlen` regardless of what the caller originally passed) - so no
genuinely kernel-written `AF_UNIX` address can ever carry a length outside `[2, 110]` in the first
place. `len - offsetof_sun_path` is therefore bounded to `[0, 108]`, which is exactly in range for
indexing a 108-byte `sun_path` array (slicing `1..108` and indexing `107` are both valid). The
suspected OOB requires a `SocketAddrAny` whose family byte and length are mutually inconsistent in a
way no real syscall on this kernel produces; the only way to construct one is to call the `unsafe
fn SocketAddrAny::new`/`read` directly with input that already violates their own documented safety
precondition ("storage must contain a valid socket address") - at that point the caller has already
committed undefined behavior by their own contract violation, not something this finding can lay at
the panic site's feet as a new defect.

**Fully reproducible**: N/A (no reproduction to report - see above).

## Root cause

Not applicable - no bug confirmed on this kernel/crate version. The suspicion (`SocketAddrAny::new`/
`read` only bound-checking against `size_of::<SocketAddrStorage>()`, not against the smaller
`sizeof(sockaddr_un)`) is real as a matter of code reading, but it's inert in practice because
`SocketAddrAny` is a single generic container shared by every address family in this crate -
`sockaddr_in6`/`sockaddr_xdp`/an unrecognized family can legitimately need up to the full
`sockaddr_storage` size, so a universal cap at `sockaddr_un`'s size would be wrong for those; the
actual invariant that matters (an `AF_UNIX`-tagged address's length never exceeds `sockaddr_un`'s
size) is instead enforced one layer down, by the kernel itself at `bind()` time, and every call site
in this crate that reads one back goes through a real syscall rather than caller-supplied raw bytes.

The "mirror image" sibling concern (`assert!(addr.addr_len() as usize >= size_of::<T>())` in
`read_sockaddr_v4`/`_v6`/`_xdp`/`_netlink` panicking on a too-short `addr_len`) was reasoned about
but not separately empirically tested with its own dedicated harness this session: every internal
call site that produces these addresses (`src/backend/{linux_raw,libc}/net/syscalls.rs`) always
supplies a full `size_of::<SocketAddrStorage>()`-sized destination buffer to the syscall
(`SocketAddrBuf::new()`), and Linux's own `getsockname`/`getpeername`/`accept`/`recvfrom` semantics
guarantee the kernel reports the address's *true* length (never a value shorter than the structure
actually written) regardless of how large a buffer was supplied - truncation (when it happens at
all) only occurs when the supplied buffer is smaller than the true address, which never happens
here since the buffer is always maximal. This is the same "kernel ABI guarantee, not
user/attacker-controlled data" shape candidates 011 and 015 already found and documented for sibling
spots in this exact file/area.

## Introduced by

N/A - no bug confirmed, nothing to bisect.

## The fix

N/A - no bug found, nothing to fix. No `patch.diff` in this folder.

## What's not done / open questions

- Not tested on a non-Linux `libc` backend target (FreeBSD/illumos) - the libc backend's own code
  comment at `src/backend/libc/net/read_sockaddr.rs:204-208` explicitly says "FreeBSD and illumos
  sometimes set the length to longer than the length of the NUL-terminated string," i.e. those
  kernels may genuinely NOT enforce the same `addrlen <= sizeof(sockaddr_un)` invariant this
  session confirmed for Linux - this candidate's "Not reproducible" conclusion is Linux-specific
  and should not be read as clearing those platforms. No BSD/Solaris environment was available in
  this container to check (same limitation noted by every other platform-specific item in
  `projects/rustix.md`'s risk map).
- The sibling short-`addr_len` assert concern (`read_sockaddr_v4`/`_v6`/`_xdp`/`_netlink`) was
  reasoned through but not given its own dedicated empirical harness this session (reasoning: every
  internal call site supplies a maximal destination buffer, and Linux always reports the true
  address length regardless of buffer size) - a future session wanting a direct empirical check
  could still write one.
- Both `SocketAddrAny::new` and `SocketAddrAny::read` are `unsafe fn` whose own documented
  precondition ("storage must contain a valid socket address") already excludes the
  family/length-mismatched input this candidate worried about; nothing in this crate's current safe
  surface can construct or pass through such a mismatched value. Worth noting for a maintainer
  anyway: the slicing in `read_sockaddr_unix` still has no internal defensive bound check (unlike,
  say, a `get(..)` that would turn a future violation into a clean panic location rather than
  relying entirely on the caller-side safety contract) - not changed here since there's no
  confirmed defect motivating a patch.
