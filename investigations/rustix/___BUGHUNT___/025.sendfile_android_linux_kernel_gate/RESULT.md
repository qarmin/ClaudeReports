# sendfile_android_linux_kernel_gate

`src/fs/sendfile.rs`'s module declaration (`src/fs/mod.rs`), its public re-export (`src/fs/mod.rs`),
and the libc backend's `sendfile()` implementation (`src/backend/libc/fs/syscalls.rs`) are all
gated on the literal `#[cfg(target_os = "linux")]`, which excludes `target_os = "android"` - even
though Android runs the Linux kernel and bionic libc provides `sendfile64` (the same function the
libc backend already calls for "regular" Linux). Sourced from open PR #1688
(https://github.com/bytecodealliance/rustix/pull/1688), which proposes switching all three sites to
the crate's own `linux_kernel` cfg alias; independently re-derived and verified rather than trusted
blind. The candidate task also asked for a quick sweep for the same mistake (`target_os = "linux"`
used where `linux_kernel` should be) elsewhere in the crate - see "What's not done" below.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `grep` for `sendfile` across `src/`; read `src/fs/mod.rs`, `src/fs/sendfile.rs`, `src/backend/{libc,linux_raw}/fs/syscalls.rs`, `build.rs`'s `linux_kernel` cfg definition | Confirmed the 3 gate sites use the literal `target_os = "linux"` while every sibling Linux-kernel-specific item in the same files already uses `linux_kernel` (`os == "android" \|\| os == "linux"`) |
| Manual boundary-value construction (cross-compile check) | `cargo check --target x86_64-linux-android` against a tiny crate (`repro/`) calling the real public `rustix::fs::sendfile(...)` API, with `default-features = false, features = ["std", "fs", "use-libc"]` (forces the libc backend, which is what Android actually uses - the `linux_raw` backend's own arch gate already excludes `target_os = "android"`) | Reproduced: `error[E0425]: cannot find function \`sendfile\` in module \`rustix::fs\`` with rustc's own "found an item that was configured out" note pointing at both `target_os = "linux"` gates in `src/fs/mod.rs` |
| Same check against the host target (`x86_64-unknown-linux-gnu`) | `cargo check` / `cargo run --release` with the same repro | Compiles and runs fine both before and after the patch - confirms the bug is Android-specific, not a general regression |

No sanitizer/miri/fuzzing was applicable - this is a compile-time `cfg` gating bug with no runtime
code path to instrument, so cross-target type-checking is the right (and sufficient) tool; the soft
3-approach cap was not needed since the 2nd approach already produced a clean, repeatable
reproduction.

## Status

**Confirmed**, reproduces 3/3 runs (deterministic - a `cfg` gate is either present or absent, not
flaky): every `cargo check --target x86_64-linux-android` against the unpatched tree fails
identically with the same `E0425` diagnostic; every run against the patched tree succeeds
identically.

**Fully reproducible**: **Yes**. `repro/` calls the real, public `rustix::fs::sendfile` API exactly
as a genuine caller would - there is no second participant involved (this is a single-process
compile-time gating bug, not a reaction to another participant's input), so CONFIRMATION.md section
11's second-participant bar doesn't apply here.

## Class

other (missing-platform-support / API-availability bug - not a crash, not UB, not a logic error in
any executed code path; the function simply doesn't exist for the affected target).

## Root cause

`src/fs/mod.rs` declares and re-exports the `sendfile` module/function under
`#[cfg(target_os = "linux")]` literally, instead of the crate's own `linux_kernel` cfg alias
(`build.rs`: `if os == "android" || os == "linux" { use_feature("linux_kernel"); }`). The libc
backend's `sendfile()` implementation in `src/backend/libc/fs/syscalls.rs` carries the identical
literal gate. Since `target_os = "linux"` is strictly narrower than `linux_kernel` (it excludes
Android), the entire public `rustix::fs::sendfile` API is compiled out when targeting Android, even
though nothing about the implementation is actually Linux-vs-Android-specific - bionic's `libc`
crate exposes `sendfile64` identically to glibc's, and the libc backend already calls it via the
exact same code path used for "regular" Linux once the gate is fixed.

## Introduced by

730b0d2c "Add a `linux_kernel` cfg, to simplify a common platform set. (#675)" (2023-05-21). This
commit introduced the `linux_kernel` cfg and mechanically converted every sibling
`#[cfg(any(target_os = "android", target_os = "linux"))]` in `src/fs/mod.rs` to
`#[cfg(linux_kernel)]` (`copy_file_range`, `mount`, `openat2`, `raw_dir`, `statx`, `xattr`, etc. -
all converted in the same diff) - but `sendfile`'s two `#[cfg(target_os = "linux")]` lines were
left untouched, appearing only as unchanged context in that commit's own diff. `sendfile` was never
Android-gated to begin with (it was Linux-only since its introduction in 8e8ccea3, "Implement Linux
`sendfile`.", predating the `linux_kernel` cfg entirely) - this reads as a mechanical sweep that
simply missed updating a site whose cfg predicate text (`target_os = "linux"`) didn't match the
exact `any(target_os = "android", target_os = "linux")` pattern the sweep was presumably
search-and-replacing, rather than a deliberate choice to exclude Android.

## The fix

Changed all 3 gate sites from `#[cfg(target_os = "linux")]` to `#[cfg(linux_kernel)]`:
`src/fs/mod.rs`'s `mod sendfile;` declaration, `src/fs/mod.rs`'s `pub use sendfile::sendfile;`
re-export, and `src/backend/libc/fs/syscalls.rs`'s `pub(crate) fn sendfile(...)` definition. This
is the identical, independently-arrived-at fix proposed by PR #1688 (+3/-3 lines total,
`git diff --numstat`).

Verified: `cargo check --target x86_64-linux-android` against `repro/`'s use of
`rustix::fs::sendfile` fails before the patch (`E0425`, see `output.txt`) and succeeds after it;
the native host build (`x86_64-unknown-linux-gnu`) compiles and runs identically before and after,
confirming no regression for the platform that already worked.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc"`, native
  `x86_64-unknown-linux-gnu` host - this sandbox has no Android emulator to run the crate's own
  integration tests under, only to type-check against): **410 passed / 2 failed, identical before
  and after the patch**. Both failures are pre-existing environment artifacts unrelated to this
  change: `io::ioctl::test_int_setter` (`/dev/net/tun` doesn't exist in this container) and
  `termios::ttyname::test_ttyname_ok` (no real tty attached in this sandboxed session) - both
  already documented in `___BUGHUNT___/`'s `projects/rustix.md` project notes as this sandbox's
  known baseline. No test touches `sendfile` directly (there is no `tests/fs/sendfile.rs` in this
  version of the crate), consistent with this being a platform-gating gap rather than a logic bug
  an existing test could have caught.

## What's not done / open questions

- **The candidate's own suggested sweep** ("grep for other APIs still gated on `target_os = "linux"`
  literally instead of `linux_kernel`") was run
  (`grep -rn '#\[cfg(target_os = "linux")\]' src/`) and turned up roughly 80 other call sites
  (`src/net/sockopt.rs`, `src/net/send_recv/msg.rs`, `src/net/types.rs`, `src/process/*`,
  `src/pty.rs`, `src/system.rs`, `src/backend/{libc,linux_raw}/net/*`, `src/backend/libc/process/
  syscalls.rs`, `src/backend/libc/mm/*`, `src/backend/libc/system/syscalls.rs`,
  `src/backend/linux_raw/param/libc_auxv.rs`). None of these were individually checked against
  whether Android's bionic libc/kernel-ABI actually provides the same functionality (many plausibly
  ARE correctly Linux-only - e.g. mainline-only `/proc`-adjacent `sysctl`/`prctl` options or
  `TCP_CONGESTION`/`tcp_info` fields that may genuinely differ on Android/bionic) - distinguishing
  "mistakenly too narrow, same bug as sendfile" from "correctly Linux-only" for each of ~80 sites is
  a full sweep-candidate's own scope (same shape as candidate 009's `flags_all_filtering_sweep`
  follow-up to candidate 007), not something to resolve inside this one. Recorded as a new suspect
  for the backlog rather than investigated further here.
- Not tested: actually running (not just cross-type-checking) the fixed `sendfile` on real Android
  hardware/emulator - no Android emulator/device was available in this sandbox. The fix only
  changes which Rust `cfg` predicate compiles the code in, not the syscall sequence itself (the
  libc backend's `sendfile()` body calling `libc::sendfile64` is identical to the already-working
  Linux path), so this is considered a low-risk gap, but is explicitly not verified end-to-end on
  real Android.
