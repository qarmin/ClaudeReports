# procctl_pdeathsig_unchecked_signal_raw

`src/process/procctl.rs:94-106`'s `parent_process_death_signal()` reads a raw `c_int` via FreeBSD's
`PROC_PDEATHSIG_STATUS` and, when non-zero, does
`unsafe { Signal::from_raw_nonzero_unchecked(non_zero) }`. Its `SAFETY` comment justifies this by
reasoning about rustix's own API surface only ("the only way to get a libc-reserved signal number in
here would be to do something equivalent to `set_parent_process_death_signal`"), not by anything
actually checked at the call site - so a process-wide kernel value set by another thread, an exec'd
image, or non-Rust code calling the raw syscall directly could in principle reach this function with
an arbitrary non-zero value. Investigated to determine whether that gap can actually produce real
Undefined Behavior, since no FreeBSD hardware/emulation is available in this environment to construct
the literal cross-mechanism scenario.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `Signal::from_raw_nonzero_unchecked`'s actual body (`src/signal.rs:255-257`) vs. the analogous, previously-confirmed-UB case (candidate 035, `Pid::from_raw_unchecked`) | - | `from_raw_nonzero_unchecked(sig: NonZeroI32) -> Self { Self(sig) }` performs **no internal unsafe primitive at all** - `sig` is already a `NonZeroI32` by the time it's passed in (constructed via the checked `NonZeroI32::new(raw)` one line above the call site in `procctl.rs`). This is structurally different from candidate 035's `Pid::from_raw_unchecked`, whose body calls `NonZeroI32::new_unchecked(raw)` on a still-possibly-zero raw `i32` - that is where the genuine UB risk lived in that case. Here there is no analogous internal check being skipped: `Signal` is `#[repr(transparent)] struct Signal(NonZeroI32)` with no additional representation invariant beyond "non-zero", which is already guaranteed by the caller's own type. |
| Miri boundary-value probe: construct `Signal` via the real `from_raw_nonzero_unchecked` with 11 deliberately adversarial values (`1`, `-1`, `i32::MAX`, `i32::MIN`, `i32::MIN+1`, `34`/`64` - typical glibc `SIGRTMIN`/`SIGRTMAX` - `65`/`126` - FreeBSD's own `SIGRTMIN`/`SIGRTMAX` - and large positive/negative values), added as a temporary `#[test]` in `src/signal.rs`'s existing `tests` module | `RUSTFLAGS="--cfg rustix_no_linux_raw" cargo +nightly miri test --features "std,all-apis,use-libc" --lib candidate_057`, 3 repeat runs | **No UB for any of the 11 values, 3/3 runs, deterministic** - `test ... ok` every time, no Miri diagnostic of any kind. See `output.txt` section "Step 4"/"Step 4b". |
| Research: does FreeBSD actually have a "libc-reserved" signal number range analogous to the one the safety comment is modeled on (glibc's two real-time signals reserved internally for NPTL thread-cancellation/`setxid`)? Checked the actual `libc` crate (v0.2.189, the version this workspace resolves to) used as rustix's FreeBSD backend | `grep` across `libc-0.2.189`'s FreeBSD/BSD source files for `SIGRTMIN`/`SIGRTMAX` | **FreeBSD is not among the platforms the `libc` crate exposes a real-time-signal range for at all** (only `linux_like`, `solarish`, `hurd`, `aix`, vxworks/rtems/qurt define `SIGRTMIN`/`SIGRTMAX`). FreeBSD's own named signal set (`freebsdlike/mod.rs` + `freebsd/mod.rs`) tops out at `SIGTHR = 32` and `SIGLIBRT = 33` - and both of those are already explicit, safely-accessible named constants in rustix's own `Signal` enum (`Signal::THR`, `Signal::LIBRT`, `#[cfg(target_os = "freebsd")]` in `src/signal.rs`), not a hidden/reserved range. FreeBSD does support POSIX real-time signals at the kernel level (`SIGRTMIN`/`SIGRTMAX`, numerically 65/126 per FreeBSD's own `sys/signal.h`), but nothing in libc reserves part of that range for its own internal use the way glibc's NPTL does - there is no FreeBSD analogue of "a signal number libc has quietly claimed for itself that user code must never touch." |
| Cross-target compile check: does the real, unmodified `parent_process_death_signal`/`Signal` code actually build for the real affected target | `cargo check --target x86_64-unknown-freebsd --features "std,all-apis,use-libc"` (prebuilt `rust-std`, already installed in this environment from a previous candidate) | Compiles cleanly - confirms the investigated code path is genuinely reachable/real on the actual FreeBSD target, not dead code. |

## Status

**Not reproducible.** The central question this candidate posed - "can
`from_raw_nonzero_unchecked`'s documented UB-on-reserved-signal contract actually be violated" - was
answered directly without needing FreeBSD hardware: the function's own body cannot produce real
Undefined Behavior for *any* non-zero `i32` value, confirmed deterministically under Miri across 11
adversarial boundary values (3/3 repeat runs, identical clean result every time). Separately, the
premise that FreeBSD even has a "libc-reserved" signal range for `PROC_PDEATHSIG_STATUS` to
plausibly return was checked against the actual `libc` crate backend and found not to hold - FreeBSD
has no NPTL-style internally-reserved signal numbers at all; every value its own libc defines up to
33 is already a safe, named `Signal` constant in this crate.

**Fully reproducible**: N/A - Status is Not reproducible, not Confirmed.

## Class

N/A (not reproducible)

## Root cause

Not a presently-triggerable bug, for two independent reasons:

1. **The specific `unsafe` operation under suspicion cannot itself produce memory-unsafety.**
   `Signal::from_raw_nonzero_unchecked`'s entire body is `Self(sig)` - it wraps an already-checked
   `NonZeroI32` with no further invariant to violate. This is unlike the structurally similar but
   actually-unsound case this skill confirmed in candidate 035 (`Pid::from_raw_unchecked`), whose
   body calls `NonZeroI32::new_unchecked` on a value that *hadn't* yet been checked non-zero -
   that's where real UB could and did occur. `Signal` carries no analogous risk because the
   zero-check (`NonZeroI32::new(raw)`) already happened one line above the `unsafe` block, using
   the *safe*, checked constructor, before `from_raw_nonzero_unchecked` is ever called.
2. **The hazard the `SAFETY` comment is actually guarding against - misuse of a "libc-reserved"
   signal value after construction (sending/consuming/blocking it, or altering its handler) - is an
   application-level contract about what a caller does with the returned `Signal` later, not about
   the construction in `procctl.rs` itself.** Whether that downstream hazard is even constructible
   on FreeBSD is itself doubtful: FreeBSD's libc (as modeled by the actual `libc` crate dependency)
   doesn't reserve any signal number for its own internal use the way glibc does, so there is no
   known FreeBSD value `PROC_PDEATHSIG_STATUS` could return that would actually BE a "libc-reserved"
   signal in the sense the comment describes.

The `SAFETY` comment's reasoning ("the only way to get a libc-reserved signal number in here...")
is still narrower than it should be in principle - it only reasons about rustix's own API surface,
not about what else could write that piece of process-wide kernel state - but this gap turns out not
to be exploitable into a real bug given points 1 and 2 above.

## Introduced by

N/A - not a presently-triggerable bug, so there's nothing to attribute a fix to. For reference only:
the `unsafe` call in `procctl.rs` in its current form was added by fc2a014f ("Use
`from_raw_nonzero_unchecked`", 2025-01-28); the surrounding function itself dates to 4a97998a ("Add
part of the FreeBSD `procctl` API (#429)", 2022-11-07).

## The fix

No fix needed/applied - Status is Not reproducible. If a maintainer still wants the `SAFETY` comment
to read more defensively (acknowledging that non-rustix code sharing the process could in principle
set this state too, even though doing so can't currently be shown to cause real UB), that would be a
documentation-only change, not a code fix.

- Project test suite: N/A - no code change was made, so before/after is identical. For completeness,
  `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast` was not re-run for this
  candidate since the working tree was never modified beyond the temporary, reverted Miri probe test
  (see "What was tried").

## What's not done / open questions

- Not verified against a real FreeBSD kernel's actual `procctl(PROC_PDEATHSIG_STATUS, ...)` behavior
  (no FreeBSD hardware/emulation available in this environment - same category of gap candidate 035
  recorded for `tcgetpgrp` on BSD/macOS). The analysis above resolves the question analytically
  (the specific unsafe operation can't produce UB for any input) rather than empirically against the
  real syscall, which is why this is `Not reproducible` rather than a `Yes`-reproducible `Confirmed`
  finding either way.
- The sibling call site with an identical pattern and identical `SAFETY` comment,
  `src/process/prctl.rs:47` (Linux's `PR_GET_PDEATHSIG`), was noticed but not independently
  investigated as part of this one-bug-per-candidate session - the same two-part reasoning above
  (no internal unsafe primitive in `from_raw_nonzero_unchecked`; no known "libc-reserved" value
  Linux's own `prctl(PR_GET_PDEATHSIG)` could plausibly return outside rustix's own control) likely
  applies there too, but wasn't separately confirmed.
- `event/kqueue.rs:124`'s `Signal::from_raw_unchecked(self.inner.ident as _)` is a third, similarly-
  shaped call site (a kernel-returned `kevent` `ident` field cast straight into a `Signal`) not
  investigated here either, for the same reason: out of scope for this specific candidate, flagged
  only as a sibling pattern worth a possible future look.
