# tcgetpgrp_pid_zero_unsound_bsd_macos

Follow-up to candidate 028's backlog note (upstream issue #1678): `src/backend/libc/termios/
syscalls.rs`'s `tcgetpgrp` (lines 166-179) only rejects a kernel-returned `pid == 0` under
`#[cfg(linux_kernel)]`. On any `not(linux_kernel)` target in the `libc` backend (macOS, FreeBSD,
NetBSD, OpenBSD, DragonflyBSD, etc.) that guard doesn't exist at all, so the function falls
straight through to `Pid::from_raw_unchecked(pid)` with `pid == 0` - `Pid` wraps `NonZeroI32`, and
`from_raw_unchecked`'s own documented safety contract requires the caller to guarantee `raw` is
non-zero. 028 flagged this as "a strong candidate for CONFIRMATION.md section 11's hardware
exception if picked up standalone" but never ran it to its own verdict; this candidate does.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code read: compare `tcgetpgrp`'s cfg gating against `Pid`'s own safety contract (`src/pid.rs`) | - | Confirmed `Pid(NonZeroI32)`, `from_raw_unchecked` has `debug_assert!(raw > 0)` plus an unconditional safety requirement that `raw != 0`; confirmed the `#[cfg(linux_kernel)]` on the `if pid == 0` check is the *only* thing gating the zero-rejection, not present in any other form for non-Linux targets |
| Real second participant: open a real pty master via `posix_openpt`/`grantpt`/`unlockpt` (no slave ever opened/attached, so no session/foreground process group has ever been assigned) and call the real libc `tcgetpgrp(2)` on it, directly, 3 times | host x86_64 Linux, debug build | **Genuine kernel-produced 0, 3/3 runs** - this is the real OS behavior rustix's own source comment already attributes to Linux ("it appears tcgetpgrp can succeed and set the pid to 0 if we pass it a pseudo-terminal device fd"); BSD/macOS share the same BSD-derived pty/tty-subsystem lineage, which is the premise behind issue #1678 |
| Feed that genuine kernel-produced `0` into the real public `rustix::process::Pid::from_raw_unchecked` directly (mirroring, line for line, the exact unguarded branch the real macOS/BSD-compiled function executes - the `#[cfg(linux_kernel)]` check is simply never run on those targets), debug build, 3 repeat runs | host x86_64 Linux, debug build, `debug_assertions` on | **Panics 3/3 runs**, identical message each time: `assertion failed: raw > 0` from `from_raw_unchecked`'s own `debug_assert!`, at `src/pid.rs:61` |
| Miri, isolating just the final construction step (`Pid::from_raw_unchecked(0)`, the same value the pty step genuinely produced) to get a platform-independent confirmation that this isn't merely a documented-but-harmless contract violation | `cargo +nightly miri run --release` (debug assertions off, to see past the earlier panic into what a release build would actually do) | **Real, deterministic Undefined Behavior**: `error: Undefined Behavior: entering unreachable code` at `NonZeroI32::new_unchecked(raw)` (`src/pid.rs:62`) - std's own precondition check fires under Miri regardless of release/debug profile |
| Static cross-target check: does the real affected code actually compile for a real non-Linux target, before and after the fix | `cargo check --target x86_64-apple-darwin --features "std,all-apis,use-libc"` (prebuilt rust-std available via `rustup target add`, no SDK/linker needed for a `cargo check`) | Compiles cleanly both before and after `patch.diff` - confirms the vulnerable branch is genuinely reachable on the real affected target and that the fix is a clean drop-in |
| Attempt real end-to-end execution on actual macOS/BSD hardware or an emulator | checked for qemu-system/Darling/bhyve/FreeBSD or macOS VM images | **None available in this environment** - no macOS/BSD hardware, no qemu-system-x86_64-based macOS/BSD emulation set up, no Darling. Real end-to-end execution of the exact compiled-for-macOS/BSD binary genuinely isn't possible here (same conclusion candidate 028 reached for sparc64 specifically, for the same underlying reason: hardware/toolchain unavailability, verified directly rather than assumed) |

## Status

**Confirmed**. Reproduces 3/3 runs (debug build, genuine kernel-produced `pid == 0` from a real
pty, host x86_64 Linux) with an identical panic each time; independently confirmed as real,
deterministic Undefined Behavior under Miri (not dependent on debug build or release build - std's
own precondition check fires either way once debug assertions are off and the invalid value is
actually constructed).

**Fully reproducible**: **No**, for the same category of reason candidate 028 recorded for its
sparc64 finding (CONFIRMATION.md section 11's hardware/toolchain exception, not the two-participant
exception - this *is* a two-participant finding (the kernel is the second participant producing
`pid == 0`), but the part that's impractical isn't fabricating that participant's input: it's
executing the real macOS/BSD-*compiled* function body at all. This session verified, not assumed,
that:
- a real second participant (the Linux kernel's own tty subsystem, via a genuinely unattached pty
  master) really does produce `pid == 0` from a real `tcgetpgrp(2)` call, 3/3 times - not
  hand-authored bytes standing in for what a kernel would return;
- that genuine value was then fed, unmodified, into the real public `rustix::process::
  Pid::from_raw_unchecked` - the exact constructor the vulnerable function calls, with the
  `#[cfg(linux_kernel)]` guard simply never present on the real affected targets (not re-derived
  logic, the guard's absence is read directly from source);
- the real affected function's surrounding code (both before and after `patch.diff`) was confirmed
  to actually compile for a real non-Linux target (`x86_64-apple-darwin`, `rustup target add`
  prebuilt std) - so this isn't merely a theoretical cfg interaction;
- but actually *executing* the resulting macOS/BSD binary isn't possible here: no macOS/BSD
  hardware, no qemu-system-based macOS/BSD emulation, no Darling in this environment (checked, not
  assumed). `rust-lld` and the available cross-toolchains in this container target Linux triples
  only.

The gap between this and a `Yes` is narrow but real: the repro runs the real public constructor
with a real kernel-produced value, but doesn't execute the literal macOS/BSD-compiled `tcgetpgrp`
function (which can't be executed anywhere in this environment) - it mirrors that function's body
verbatim instead, exactly as 028's sparc64 repro mirrored `tcgetattr`/`tcsetattr`'s body rather
than running a sparc64-compiled binary under `qemu-sparc64` (which also failed there, for an
analogous toolchain reason).

## Class

UB

## Root cause

`Pid` (`src/pid.rs`) is `#[repr(transparent)] struct Pid(NonZeroI32)` specifically so that
`Option<Pid>` has the same size/layout as a raw `pid_t` (the niche optimization the crate's own
`test_sizes` unit test asserts). `Pid::from_raw_unchecked` is `unsafe fn` precisely because it
skips the zero-check that `Pid::from_raw` performs, documented as requiring "the caller must
guarantee raw is non-zero." `tcgetpgrp` (`src/backend/libc/termios/syscalls.rs:166-179`) calls the
real `libc::tcgetpgrp(2)`, which this session independently verified can genuinely return 0 on an
unattached pty master fd (the exact scenario rustix's own source comment already attributes to
Linux). The function rejects that value with `OPNOTSUPP` - but only `#[cfg(linux_kernel)]`;
on every other `libc`-backend target (macOS, the BSDs), the rejection simply isn't compiled in at
all, so `pid == 0` flows straight into `Pid::from_raw_unchecked(pid)`, violating its documented
precondition. In a debug build this is caught by `from_raw_unchecked`'s own `debug_assert!(raw >
0)` (a panic, not silent). In a release build (where `debug_assert!` is a no-op), this constructs
a `NonZeroI32` whose value is actually zero - confirmed by Miri to be real, immediate Undefined
Behavior (`entering unreachable code` at the point std's own precondition check fires), independent
of whatever the caller does with the resulting `Pid` afterward: the compiler is entitled to assume
every `NonZeroI32`/`Pid` value is never zero (that's the entire reason the niche optimization
exists), so this single construction alone is sufficient to miscompile anything downstream that
relies on that invariant - not merely a logic error contingent on later use.

## Introduced by

bc4fa30d "Fix `tcgetpgrp` and `tcsetpgrp` on Linux. (#910)" (2023-11-06) - this commit introduced
both the `if pid == 0 { return Err(io::Errno::OPNOTSUPP) }` check and its `#[cfg(linux_kernel)]`
gate in their current form (verified via `git blame -L 165,180` on
`src/backend/libc/termios/syscalls.rs`; the commit's own title makes clear the fix was scoped to
Linux specifically, which is exactly how the gate ended up Linux-only - not a later regression,
the guard was never generalized to other platforms in the two years since).

## The fix

Remove the `#[cfg(linux_kernel)]` attribute from the `if pid == 0` check so it runs on every
platform `tcgetpgrp` is compiled for, not just Linux - `Pid::from_raw_unchecked`'s precondition
(`raw != 0`) is unconditional, so the check that establishes it must be unconditional too. This is
the same fix upstream issue #1678 requests and the same one noted as "known" in this candidate's
own description.

```diff
-        #[cfg(linux_kernel)]
+        // `Pid` is documented to never hold 0 (it wraps a `NonZeroI32`), so
+        // this check must run on every platform, not just Linux: the same
+        // kernel/tty-subsystem behavior is possible on macOS/BSD too (see
+        // <https://github.com/bytecodealliance/rustix/issues/1678>), and
+        // `Pid::from_raw_unchecked(0)` is unsound regardless of platform.
         if pid == 0 {
             return Err(io::Errno::OPNOTSUPP);
         }
```

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: re-ran the
exact same real-pty-driven repro 3 times against the patched logic (`after.rs`), not just once -
all 3 cleanly return `Err(OPNOTSUPP)` with no `Pid` ever constructed from 0. Also re-confirmed the
patched source still compiles cleanly for the real affected target
(`x86_64-apple-darwin`, see output.txt section 4) - the fix doesn't just move the problem to a
different platform-specific branch.

- Build-verified (patched tree, libc backend): `cargo build --features "use-libc,all-apis,std"` on
  host x86_64 - succeeds (this host is `linux_kernel`, so the removed `#[cfg]` doesn't change what
  compiles here; the meaningful compile check is section 4 of output.txt, against the real
  non-Linux target).
- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64): **411 passed, 1 failed, identical both before and after** - the one failure
  (`ioctl::test_int_setter`, `NotFound` opening `/dev/net/tun`) is the same pre-existing,
  environment-caused failure candidate 028 documented; no new failures introduced by the fix.

## What's not done / open questions

- Not independently confirmed on real macOS/BSD hardware - see "Fully reproducible" above for
  exactly what was and wasn't verified instead, and why.
- `tcsetpgrp` (the sibling function, `src/backend/libc/termios/syscalls.rs:183-185`) takes a `Pid`
  as an *argument* rather than constructing one from a kernel return value, so it isn't exposed to
  this same construction-site bug - not re-checked further here since the asymmetry is structural
  (caller-supplied `Pid`, not kernel-returned raw `i32`), not an oversight.
- Didn't search the rest of the `libc` backend for other `Pid::from_raw_unchecked`/
  `Gid`/`Uid`-style `NonZero`-wrapped construction sites that might have the same
  Linux-only-cfg-guard pattern (e.g. in `src/backend/libc/process/syscalls.rs`,
  `src/backend/libc/thread/syscalls.rs`) - worth a follow-up sweep, not done as part of this
  one-bug-per-candidate session.
