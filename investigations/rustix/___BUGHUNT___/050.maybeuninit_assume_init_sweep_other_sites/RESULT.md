# maybeuninit_assume_init_sweep_other_sites

Follow-up to candidate 039 (`fstat`/`stat`/`lstat` reading uninitialized `st_lspare`/`st_qspare`
reserved fields on Apple targets, confirmed via Miri and fixed with `MaybeUninit::zeroed()`): a
systematic sweep of every other `MaybeUninit::<T>::uninit()` -> `assume_init()` call site named in
this candidate's brief, using the same Miri cross-interpretation technique, to check whether the
same bug class (a struct with a reserved/padding field the real syscall doesn't guarantee to
write) recurs anywhere else in the `libc` backend.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of all 8 named call-site groups to classify which already use `zeroed()`, which touch a struct with no plausible reserved field, and which are worth testing | Read `src/backend/libc/{system,process,event,net,termios}/*.rs`, `src/net/wsa.rs`, `src/termios/types.rs` | 5 of the 8 named `sockopt.rs` lines (1053, 1180, 1215, 1259, 1276) already use `MaybeUninit::zeroed()` (not `uninit()`) or a `MaybeUninit<[MaybeUninit<u8>; N]>` pattern that's trivially safe regardless of what the kernel writes - not a bug, already safe. `rlimit_from_libc`'s `c::rlimit` is a plain 2-integer struct with no reserved field on any known platform. `event/syscalls.rs:404` is actually `port_get` (Solaris/illumos `port_event`), not BSD `kevent` as the brief's line-number gloss suggested; the real BSD `kevent`/`Event` path lives in `src/event/kqueue.rs` via a `Buffer<Event>::assume_init(len)` pattern, not a whole-struct `uninit()`. `src/termios/types.rs` turned up a genuine, previously-unnoticed structural lead: a `pub(crate) __reserved: [ffi::c_uint; 3]` field in rustix's own `Termios`, present only `#[cfg(target_os = "nto")]` (QNX), mirroring an identical private `__reserved: [c_uint; 3]` field in `libc::unix::nto::termios` - never explicitly initialized anywhere in rustix's own code (`grep -rn __reserved src/` shows no write to it, unlike the sibling `futex_waitv::__reserved` which IS explicitly zeroed in its constructor) |
| Miri cross-interpretation (same technique as candidate 039): run rustix's real public API for each still-plausible site, `--target x86_64-apple-darwin` / native `x86_64-unknown-linux-gnu` / `--target x86_64-pc-nto-qnx710` / `--target x86_64-unknown-illumos`, `MIRIFLAGS="-Zmiri-disable-isolation"` | debug build, 1 run per site/target combination (uname repeated 4x total for repeatability) | `rustix::system::uname()` **does** produce a real Miri UB diagnostic ("`.sysname[5]`, encountered uninitialized memory") - reproduces 4/4 runs, both on native Linux and cross-interpreted for apple-darwin. `rustix::net::socketpair()` runs **cleanly, no UB**, on both native Linux and apple-darwin. `rustix::termios::tcgetattr()` (QNX), `rustix::event::kqueue::kqueue()` (apple), and `rustix::event::port::create()` (illumos) all fail with "unsupported operation: can't call foreign function ... on OS ..." - Miri has no shim at all for these 3 functions on their respective target OSes, the same kind of tooling gap candidate 039 hit for `fstatat` on apple, not an answer either way |
| Independent verification of the one real UB signal found (`uname`/`sysinfo`) against the REAL syscall, not Miri's shim: a poison-byte test (`std::ptr::write_bytes(ptr, 0xAA, 1)` before the call, then check every byte of the result for leftover `0xAA`) run as a plain (non-Miri) native binary | release-independent plain `cargo run`, real `x86_64-unknown-linux-gnu` host kernel, 3 repeat runs for `uname()`/`sysinfo()`/`getrlimit()` each | **No poison byte survives** for any of the 3 real syscalls, every run - the real kernel/glibc path fully initializes all 390 bytes of `utsname`, all 112 bytes of `sysinfo`, and all 16 bytes of `rlimit`, consistent with how these syscalls are actually implemented (the kernel's `sys_newuname`/`do_sysinfo` copy a single fully-populated fixed-size kernel struct back to userspace in one `copy_to_user`, not a field-by-field partial write) |

## Status

**Not reproducible.** The sweep found one real, repeatable Miri UB signal (`uname`, reproduces
4/4 runs including cross-interpreted for apple-darwin and natively on Linux), but a direct
poison-byte test against the real syscall proved it does not reflect real OS behavior - it is a
gap in Miri's own `uname`/`sysinfo` shim (which apparently writes only a short placeholder string
into `sysname` without zero-filling the rest of the 65-/256-byte buffer first), not a bug in
rustix. This is the inverse of candidate 039: there, Miri's apple shim for `fstat` was
independently corroborated by a real-hardware bug report (upstream issue #1668); here, the
corresponding real-syscall check shows the opposite - the real OS always fully initializes the
struct, so `assume_init()` is sound in practice at this call site, Miri's finding notwithstanding.

The structurally most promising remaining lead - the QNX-only `Termios::__reserved` field, which
really is a `pub(crate)` field rustix's own code never writes before the non-Linux
`MaybeUninit::<Termios>::uninit()` -> `tcgetattr()` -> `assume_init()` path - could not be tested
at all: Miri has no foreign-function shim for `tcgetattr` on OS "nto" in this Miri version, and no
real QNX hardware/emulator is available in this environment. The same "no shim, no hardware"
outcome applies to BSD `kevent`/`kqueue()` (apple) and Solaris/illumos `port_get`/`port_create()`.
These are tooling-coverage gaps, not evidence the suspect sites are safe - see "What's not done"
below.

**Fully reproducible**: **N/A** (no Confirmed finding this session - the one real, repeatable
signal found was independently shown not to reflect actual OS behavior, see above).

## Class

N/A (Not reproducible)

## Root cause (why each suspect turned out safe, untestable, or already fixed)

- **`uname`/`sysinfo`** (`src/backend/libc/system/syscalls.rs:26-46`): the real Linux `uname(2)`/
  `sysinfo(2)` syscalls always return a fully-populated fixed-size kernel struct in one
  `copy_to_user` (verified empirically via the poison-byte test, consistent with how
  `sys_newuname`/`do_sysinfo` are implemented in the kernel: both operate on a single
  already-fully-initialized in-kernel struct, never a partial/field-by-field fill), so
  `assume_init()` is sound on real hardware. Miri's own shim for these two functions does not
  model this faithfully (it appears to leave most of the destination buffer genuinely
  uninitialized rather than zero-filling it the way the real kernel does) - a Miri limitation,
  not a rustix bug.
- **`rlimit_from_libc`** (`src/backend/process/syscalls.rs:276,306`): `c::rlimit` is `{rlim_cur,
  rlim_max}`, two same-sized, same-aligned integers with no reserved/padding field on any
  supported platform; the real `getrlimit(2)` call fully initializes both (poison-byte-verified).
  No Miri shim exists to cross-check further, but there is no structural hook for this bug class
  here at all.
- **`socketpair`** (`src/backend/libc/net/syscalls.rs:431`): `[OwnedFd; 2]` cast to `*mut c_int`
  for the raw write - 2 plain integers, no reserved field, Miri *does* have a working shim here
  and it runs clean (no UB) on both native Linux and cross-interpreted apple-darwin.
- **XDP `sockopt.rs` sites** (1053, 1180, 1215, 1259, 1276): already safe by construction. 1053
  (`tcp_congestion`) uses `MaybeUninit<[MaybeUninit<u8>; 16]>`, whose outer `assume_init()` is
  trivially sound regardless of what the kernel wrote (the inner `MaybeUninit<u8>`s have no
  validity requirement); 1180/1215/1259/1276 (`xdp_mmap_offsets`/`xdp_statistics`) already use
  `MaybeUninit::<T>::zeroed()`, not `uninit()`, with an explicit code comment ("All members ...
  are u64 and thus are correctly initialized by zeroed()") - this bug class was already fixed or
  never present at these specific lines; the candidate brief's line numbers pointed at
  already-safe code.
- **`kevent`'s `Event`** (named as `event/syscalls.rs:404`, which is actually Solaris/illumos
  `port_get`'s `MaybeUninit::<c::port_event>::uninit()`; the real BSD `kevent` path is
  `src/event/kqueue.rs`'s `Buffer<Event>::assume_init(len)`): neither could be exercised - Miri
  has no shim for `kqueue()` on OS "macos" nor for `port_create()` on OS "illumos" in this Miri
  version (both fail with "unsupported operation: can't call foreign function ... on OS ...").
  Not ruled in or out.
- **`termios/syscalls.rs:56,108,367`**: line 56 (`TCGETS2` ioctl path, Linux-only) already copies
  only the known fields it understands into rustix's own `Termios`, not a whole-struct
  `assume_init()` of the raw kernel type. Line 367 (`tcgetwinsize`/`Winsize`) is a plain
  4x`u16` struct with no reserved field on any platform, filled in full by `TIOCGWINSZ` on every
  known OS - not independently Miri/poison-tested this session, ruled out by code reading only.
  Line ~108 (the generic non-Linux `MaybeUninit::<Termios>::uninit()` -> `c::tcgetattr()` ->
  `assume_init()` path) is the one genuinely unresolved, structurally compelling lead: on
  `target_os = "nto"` only, rustix's own `Termios` carries a `pub(crate) __reserved: [ffi::c_uint;
  3]` field that mirrors `libc::unix::nto::termios`'s own private `__reserved` field exactly, and
  nothing in rustix's own code ever writes to it - but Miri has no `tcgetattr` shim for OS "nto",
  and no real QNX hardware/emulator was available to check what QNX's real `tcgetattr(3)` actually
  does with it. Not ruled in or out - see "What's not done" below.
- **`wsa.rs:25`** (Windows `WSADATA`): not tested at all - this environment has no Windows host,
  and a Miri cross-interpretation attempt was not made for a `*-pc-windows-*` target (Miri's
  Windows shim coverage for networking APIs specifically was judged unlikely to include
  `WSAStartup`, based on the same shim-sparsity pattern seen for `kqueue`/`port_create`/
  `tcgetattr` above, but this was not independently verified this session). `WSADATA`'s
  `iMaxSockets`/`iMaxUdpDg` fields are documented by Microsoft as deprecated/unused as of Winsock
  2.2, which is at least superficially the same "reserved field" shape as this bug class, but
  that alone isn't evidence either way without checking what `WSAStartup` actually writes there.

## Introduced by

N/A (Not reproducible; the one real UB signal found was ruled out as a tool artifact, not a bug,
so there is no introducing commit to identify).

## The fix

No fix - nothing in this sweep was confirmed as a real bug. No source changes were made to the
main project in this session.

- Project test suite: not run (no code change to validate).

## What's not done / open questions

- **The QNX `Termios::__reserved` field is the one lead this session could not close either way**
  - it has the exact right shape (an explicit reserved field in both the vendored `libc` struct
    and rustix's own mirrored struct, never written by rustix, relied upon entirely from the real
    syscall) but needs either real QNX hardware/an emulator, or a newer Miri version with an
    `nto` `tcgetattr` shim, to actually test. Recommend re-attempting with a different
    Miri version or toolchain before writing this off, and/or filing it upstream as a
    documentation/code question (does rustix's own CI ever build+run for QNX? If not, this is an
    untested target in general).
- **BSD `kevent`/`Event`'s own struct fields were not inspected for platform-specific reserved
  fields** the way `Termios` was for QNX - `src/event/kqueue.rs`'s `Buffer<Event>::assume_init(len)`
  pattern only needed to be ruled out as "no Miri shim available", not actually checked field-by-
  field against `libc::kevent`'s definition on every BSD variant (FreeBSD/NetBSD/OpenBSD/DragonFly
  each have their own `struct kevent`/`kevent64` layout, not all identical to apple's).
- **Solaris/illumos `port_event`'s own struct was not inspected for reserved fields** either, for
  the same reason (no Miri shim, no hardware) - the candidate brief's suspicion about this call
  site remains neither confirmed nor ruled out.
- **`wsa.rs`'s `WSADATA`** needs either real Windows or a documented, verified answer for what
  `WSAStartup` actually writes into the deprecated `iMaxSockets`/`iMaxUdpDg` fields - not attempted
  this session beyond noting the fields are documented-deprecated.
- **Whether Miri has a newer/updated shim set that covers any of `tcgetattr`(nto)/`kqueue`(apple)/
  `port_create`(illumos) in a different Miri version** was not checked - this session used
  whatever Miri ships with this container's `nightly` toolchain component; a different nightly
  might have broader shim coverage.
- This session's main actionable finding for a future session: **Miri's own `uname`/`sysinfo`
  shims are measurably less complete than real OS behavior** (confirmed by direct poison-byte
  comparison) - any *future* candidate that gets a Miri UB signal from `uname`/`sysinfo`
  specifically should be treated with the same skepticism applied here, not assumed to mirror
  real hardware the way candidate 039's `fstat` finding did.
