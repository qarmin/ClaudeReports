# statvfs_favail_copied_from_ffree_libc_backend

Candidate 092 set out to continue this workspace's non-Linux-backend Miri cross-interpretation
sweep for NetBSD (never swept before) and go deeper on Solaris/illumos beyond what candidates 033
(termios speed decode) and 057 (FreeBSD procctl, unrelated OS) already covered - per
`projects/rustix.md`'s risk-map note. The NetBSD/Solaris/illumos-specific angle itself turned up
nothing new (see "What was tried"), but reading through the shared `libc`-backend filesystem code
these platforms all go through (`src/backend/libc/fs/syscalls.rs`) surfaced a real, unrelated,
long-standing bug in `libc_statvfs_to_statvfs` - the private conversion function behind
`rustix::fs::statvfs()`/`fstatvfs()`, shared by every `libc`-backend target (not just the three
this candidate was aimed at): `f_favail` is populated from `from.f_ffree` instead of
`from.f_favail`, discarding the real, independently-documented "inodes available to non-privileged
processes" value every single vendored `libc` crate platform module defines as a separate field,
and substituting the "total free inodes" value instead.

## What was tried (the original NetBSD/Solaris/illumos sweep)

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `rustup target add x86_64-unknown-netbsd`/`x86_64-pc-solaris`/`x86_64-unknown-illumos` (all three have prebuilt `rust-std`, confirmed by a successful `rustup target add`) then `cargo check --tests --target <target> --features "std,all-apis,use-libc" --lib` - the same "does the `#[cfg(test)]` self-check even compile for this target" angle candidate 081 used to find the real wasi bug | type-check only (no link), stable toolchain | **Clean negative for all three**: no compile errors on any of the three targets, including the `#[cfg(test)]` `test_layouts` self-check in `src/fs/constants.rs` (the exact module 081's wasi bug lived in) |
| `cargo test --no-run --target x86_64-unknown-netbsd --features "std,all-apis,use-libc" --lib` (goes one step further than `cargo check --tests`, attempting an actual link) | debug build | Compiles cleanly; fails only at the **link** step (`cannot find -lexecinfo`, no NetBSD sysroot/system libraries in this sandbox) - same "type-checks but can't link/run" category candidate 033 already documented for illumos. No compile-time defect found |
| Manual read of NetBSD's hand-rolled `Stat` struct (`src/backend/libc/fs/types.rs:1034`, used because NetBSD's `st_mtime_nsec` is named `st_mtimensec`) cross-checked field-by-field (name, order, type) against the real vendored `libc-0.2.182`/`0.2.189` NetBSD `struct stat` (`src/unix/bsd/netbsdlike/netbsd/mod.rs:190`) - the exact "layout mismatch" bug class that caught candidate 039 on Apple | reading real vendored source, no build | **Clean negative**: all 20 fields match in name, order, and type between rustix's custom struct and the real libc struct (both `#[repr(C)]`) - no mismatch, unlike Apple's case |
| Grep every NetBSD-gated file (26 files matching `target_os = "netbsd"`/`netbsdlike`) for `unwrap()`/`assume_init`/`transmute`/raw pointer casts/panicking assertions near the NetBSD cfg gate | static analysis, no build | No new unchecked-input-from-kernel pattern found. `process::wait.rs`'s `si_status()` is already explicitly disabled for NetBSD with a code comment explaining why ("the libc crate's `si_status()` implementation doesn't appear to match what's in NetBSD's headers") - already handled, not a live bug |
| Grep every solarish-gated file (41 files) for the same patterns, focused on areas not yet covered by candidate 033 (termios) or 056 (sockaddr_unix) - `net/sockopt.rs`, `process/types.rs` (rlimit constants), `pty/syscalls.rs` (`ptsname_r`), `event/syscalls.rs` | static analysis, no build | No new unchecked-input-from-kernel pattern found in these areas specifically |
| `cargo check --tests --target x86_64-unknown-illumos`/`x86_64-pc-solaris` (same self-check compile probe as above) | type-check only | Clean negative for both |

None of the above justified a NetBSD/Solaris/illumos-specific candidate. While reading
`src/backend/libc/fs/syscalls.rs` for the NetBSD/solarish statfs/statvfs cfg gates (both platforms
flow through this same file's `statvfs`/`fstatvfs` functions), the `libc_statvfs_to_statvfs`
conversion function's field list caught attention: every other field follows the pattern
`f_x: from.f_x as u64`, except one line reading `f_favail: from.f_ffree as u64` - a copy-paste
mismatch, not a platform-specific issue at all.

## What was tried (the actual statvfs finding)

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grepped the vendored `libc-0.2.189` crate's `struct statvfs` definition for every platform rustix's `libc` backend supports (solarish, NetBSD, OpenBSD, DragonFly, FreeBSD, Apple, Haiku, AIX, Cygwin, QNX/nto, Redox, GNU/Hurd, every Linux libc flavor - glibc/musl/uclibc/l4re/emscripten/android/nuttx) | reading real vendored source across ~25 platform modules, no build | **Every single one** defines `f_favail` as a field genuinely separate from `f_ffree` (POSIX's documented "file serial numbers available to non-privileged process" vs "total free file serial numbers") - confirming the field rustix discards is real and meaningful everywhere this function runs, not an artifact of one odd platform |
| `git log -S"f_favail: from.f_ffree" -- src/backend/libc/fs/syscalls.rs` | git history | Found the exact introducing commit (see "Introduced by") - the line has read this way, unchanged, since the function was first written; not a later regression |
| Real end-to-end repro: force the `libc` backend on this sandbox's real x86_64 Linux host (`RUSTFLAGS=--cfg=rustix_use_libc`, documented in `README.md`) and call rustix's real, unmodified public `rustix::fs::statvfs(".")`/`fstatvfs(&file)` against the real filesystem, alongside a direct raw `libc::statvfs`/`fstatvfs` call for comparison | debug build, real syscalls | First attempt: `f_ffree == f_favail` on every real filesystem/overlay accessible in this sandbox (ext4-backed Docker overlay) - Linux's own kernel `statfs`/`statfs64` API has no separate "available to unprivileged users" inode concept at all, so glibc's own `statvfs()`/`statvfs64()` wrapper always copies one into the other; the bug's effect is **not observable** from an unmodified real syscall on this host, for the same structural reason it may never be observable on Linux at all (not a sandbox limitation specifically - this is inherent to how Linux's statvfs is implemented in every libc) |
| Escalate: a small LD_PRELOAD shim (`repro/shim.c`) that passes every call straight through to the real libc via `dlsym(RTLD_NEXT, ...)` (not reimplemented) and only overwrites the returned `f_favail` field, simulating a real OS/filesystem that implements POSIX's documented ffree/favail distinction (e.g. inode reservation) | real syscall + real libc, with one field substituted at the libc-call boundary; 3 repeat runs | **Reproduces 3/3 runs, deterministically**, for both `rustix::fs::statvfs()` and `rustix::fs::fstatvfs()`: rustix's output unconditionally equals the real `f_ffree`, never the (deliberately different) real `f_favail` the shimmed call returned. See "Fully reproducible" below for what this does and doesn't demonstrate |
| Debugging why the first LD_PRELOAD attempt (intercepting only the plain `statvfs`/`fstatvfs` symbols) appeared to show no effect on rustix's call specifically, while an identical raw call in the same test binary was correctly intercepted | `nm -D`/reading `src/backend/libc/c.rs` | Root-caused: on this target (`linux_kernel` + `libc` backend), rustix's own `c::statvfs`/`c::fstatvfs` are explicit re-exports of `libc::statvfs64`/`fstatvfs64` (`src/backend/libc/c.rs:136,143`), not the plain `statvfs`/`fstatvfs` symbols - a real caller linking the plain symbols (as the repro's own comparison call does) resolves to a *different* ELF symbol than rustix's internal call. The shim was fixed to intercept both pairs; documented in `repro/statvfs_check.rs`'s own header comment so this doesn't need rediscovering |

## Status

**Confirmed.** Reproduces 3/3 runs, deterministically, for both the `statvfs` and `fstatvfs` entry
points (debug build, real glibc, real filesystem, real syscalls - only the `f_favail` field of the
real result is substituted by the LD_PRELOAD shim before rustix sees it).

**Fully reproducible**: **No**, with a precise reason (CONFIRMATION.md section 11's narrow
exception): the repro's syscall/filesystem path is 100% real and unmodified - the only faked
element is the "second participant"'s specific *data value* for one field, because no filesystem
or OS accessible in this sandbox (and arguably none in common use - see below) ever reports
`f_ffree != f_favail` in practice, so there was no way to observe the bug's effect from a genuinely
unmodified real syscall alone. This is not simply "no illumos/NetBSD hardware available" (the
033/057 category) - it's that *even a real illumos/Solaris/NetBSD/BSD/Haiku machine* would very
plausibly show the same "no visible difference" result, since POSIX's ffree/favail distinction
exists for inode-reservation accounting that, as far as this session could determine, essentially
no mainstream filesystem actually implements (block reservation for root - `f_bavail` vs
`f_bfree` - is extremely common; the inode-count equivalent is not). The LD_PRELOAD shim simulates
a real OS/filesystem that *did* implement this distinction, which is a real, POSIX-documented, non-
adversarial scenario, not a fabricated protocol message - but it is still a constructed input value
standing in for "what a hypothetical divergent kernel would report," not an input this session
could obtain from any genuinely divergent machine. The bug itself is unambiguous from source +
history (next section) independent of this caveat.

## Class

logic

## Root cause

`libc_statvfs_to_statvfs` (`src/backend/libc/fs/syscalls.rs:1631`), the private function behind
the public `rustix::fs::statvfs()`/`fstatvfs()` (used by every `libc`-backend target except wasi -
all BSDs, Apple, solarish, Haiku, AIX, Cygwin, QNX, Redox, GNU/Hurd, and Linux/Android whenever the
libc backend is selected, which is the default on every one of those non-Linux targets and
opt-in on Linux via `--cfg=rustix_use_libc`), builds rustix's public `StatVfs` struct field-by-
field from the real `c::statvfs` the real libc `statvfs()`/`fstatvfs()` call returned. Every field
follows the same `f_x: from.f_x as u64` pattern, with one exception:

```rust
f_ffree: from.f_ffree as u64,
f_favail: from.f_ffree as u64,   // should read `from.f_favail`
```

This unconditionally discards whatever the real OS/libc reported for `f_favail` (POSIX: "number of
file serial numbers available to non-privileged processes," which can legitimately be less than
`f_ffree` due to inode reservation) and substitutes `f_ffree` instead - the exact same mistake for
inodes that would be immediately obvious for blocks (`f_bavail` is correctly read from
`from.f_bavail`, two lines above - the function clearly knows the ffree/favail distinction matters,
just mishandles it for the inode pair specifically).

## Introduced by

c663a107e1a38ce188d5b5ee3c7046c0d1390ea9 "Implement `statvfs` and `fstatvfs`. (#375)" (2022-07-03).
This line has read `f_favail: from.f_ffree as u64,` unchanged since the function was first written
in this commit (confirmed via `git log -S"f_favail: from.f_ffree" -- src/backend/libc/fs/syscalls.rs`,
single hit) - not a later regression introduced by a subsequent refactor. The commit message itself
doesn't mention any deliberate decision to collapse the two fields; every sibling field in the same
struct literal correctly reads its own matching field, making this read as an unintentional
copy-paste slip (`f_ffree` typed twice in a row) rather than a design choice.

## The fix

One-line change: `f_favail: from.f_ffree as u64,` -> `f_favail: from.f_favail as u64,` in
`src/backend/libc/fs/syscalls.rs`. Verified against both affected public entry points
(`rustix::fs::statvfs()` and `rustix::fs::fstatvfs()`, which share this exact function) via the
LD_PRELOAD-shimmed repro, 3/3 runs each, before (bug reproduces) and after (output matches the real
`f_favail`) - see `output.txt` for the real captured transcript of a fresh `just test` run (clone
-> reproduce-before -> apply -> reproduce-after). Also re-ran the no-shim sanity case (normal,
unmodified real syscall) after the fix to confirm no change in the common case where `f_ffree ==
f_favail` - output is bit-identical to the pre-fix no-shim run, as expected for this specific fix.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host `x86_64-unknown-linux-gnu`): **411 passed, 1 failed, identical before and after** - the one
  failure is the pre-existing, environment-caused `ioctl::test_int_setter` (`/dev/net/tun` missing
  in this sandbox, already documented in `projects/rustix.md` and in candidate 081's own RESULT.md),
  not a regression from this patch. No test in the existing suite exercises `statvfs`/`fstatvfs`'s
  `f_favail` field at all, which is consistent with this bug going unnoticed since 2022.

## What's not done / open questions

- Per "Critical pitfalls" #7: the `linux_raw` backend's own, structurally different
  `statfs_to_statvfs` (`src/backend/linux_raw/fs/syscalls.rs:956`, `f_favail: statfs.f_ffree as
  u64`) looks superficially identical but was **not** treated as the same bug and was **not**
  patched here - Linux's native `statfs`/`statfs64` kernel ABI has no `f_favail`-equivalent field at
  all to read from in the first place, so copying `f_ffree` there mirrors exactly what glibc/musl's
  own `statvfs()` implementations do on Linux (confirmed empirically this session: a real C program
  calling the real glibc `statvfs()` on this host also returns `f_ffree == f_favail` always). That
  line is `linux_raw`'s only reasonable option, not a bug - flagging the distinction explicitly so a
  future session doesn't mistake it for the same issue.
- The "no mainstream filesystem actually diverges ffree/favail" claim in "Fully reproducible" above
  is this session's best effort within its time/tooling budget (checked: no CAP_SYS_ADMIN in this
  sandbox to mount a real XFS/UFS/ZFS filesystem with inode quotas and test directly) - not
  exhaustively verified across every filesystem on every supported OS. If a future session gets
  real BSD/illumos/Solaris/Haiku hardware or a privileged container, re-running the no-shim sanity
  case there (same `repro/statvfs_check.rs`, no LD_PRELOAD) against a filesystem with inode
  reservation configured would upgrade "Fully reproducible" from No to Yes.
- The original NetBSD/Solaris/illumos sweep this candidate was aimed at found nothing new (see
  first "What was tried" table) - per `projects/rustix.md`'s risk map, this leaves the "deeper
  Solaris/illumos sweep" and "NetBSD sweep" items essentially exhausted via the cross-compile-check
  technique; a future session wanting to continue this angle would likely need either (a) real
  non-Linux hardware/VMs (not available in this sandbox), or (b) a different technique entirely
  (e.g. a longer fuzz campaign via `fuzz-bug-repro`, run separately).
