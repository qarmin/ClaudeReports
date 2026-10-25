# kqueue_evfilt_proc_pid_unwrap

`Event::filter()`'s `EVFILT_PROC` arm (`src/event/kqueue.rs:119`) does
`Pid::from_raw(self.inner.ident as _).unwrap()` when decoding a real `kevent()`-returned
`kevent_t`, which panics if `ident` is ever `0` (`Pid::from_raw(0)` is `None`, since `Pid` is a
`NonZeroI32`). Flagged because `Event::filter()` decodes whatever `kevent_t` a real `kevent()`
syscall returned, not just a user-constructed `Event` - so this depends entirely on the kernel
always echoing back exactly the same nonzero PID that was originally registered via
`EventFilter::Proc`, a kernel-contract defense in the same family as several already-closed
candidates in this workspace (046, 049, 051, 052, 056, 057, 062). This code only compiles under
`cfg(bsd)` (FreeBSD/NetBSD/OpenBSD/DragonFly/Apple); this container has no BSD/macOS hardware or
emulation, so no real `kevent()` syscall can be executed here.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: rustix's own public construction path for an `EVFILT_PROC` registration | `src/event/kqueue.rs` `Event::new`/`EventFilter::Proc`, `src/pid.rs` `Pid::from_raw` | `Event`'s `inner: kevent_t` field is private and `Event::new` is the only public constructor; `EventFilter::Proc { pid: Pid, .. }` requires an actual `Pid` value, and `Pid` is `#[repr(transparent)] struct Pid(NonZeroI32)` - there is no public way to obtain a `Pid` whose raw value is `0` (`Pid::from_raw(0)` returns `None`). So rustix's own typed API cannot be used to register an `EVFILT_PROC` kevent with `ident == 0` in the first place - the only way to get such a `kevent_t` onto the kqueue at all is to bypass rustix and call the raw `kevent()`/`libc::kevent` syscall directly with a hand-built struct. |
| Kernel source review: does a real BSD/Darwin kernel ever recompute/zero the `ident` field of an in-flight `EVFILT_PROC` knote after registration, independent of what rustix registered | Read current upstream kernel source for 3 independently-maintained kqueue implementations: FreeBSD (`sys/kern/kern_event.c`), Apple/XNU (`bsd/kern/kern_event.c`), NetBSD (`sys/kern/kern_event.c`), fetched fresh from each project's GitHub mirror | All three copy the registering kevent's `ident` field into the knote **verbatim** at `EV_ADD` time and return that **same stored value unchanged** on every later report, never recomputing it from anything else: FreeBSD's `kqueue_register` does `kn->kn_kevent = *kev;` (line 1841) and `kqueue_scan`/`kqueue_process` emit `*kevp = kn->kn_kevent;` (lines 1098/1190/2238/2265) as-is; XNU's event-reporting path builds the output kevent as `.ident = kn->kn_id` (line 4382) with no transformation; NetBSD's registration path does `kn->kn_id = kev->ident;` (line 1974) directly from the caller-supplied value. None of the three ever substitute a different/zeroed `ident` for `EVFILT_PROC`-filtered knotes at any point in their lifetime. |
| Manual boundary-value construction via the real, unmodified public API, cross-interpreted under Miri for the real `cfg(bsd)` target `x86_64-apple-darwin` (the real `Event::new`/`Event::filter` logic, no mock, no real `kevent()` syscall involved - this is pure in-memory encode/decode) | `cargo +nightly miri run --target x86_64-apple-darwin --features "event,std,process"`, `examples/kq_proc_ident_check.rs`: builds a real `EventFilter::Proc { pid: Pid::from_raw(1234).unwrap(), .. }` via `Event::new`, decodes it back via the real `Event::filter()`, asserts the round-tripped pid equals `1234`; separately asserts `Pid::from_raw(0) == None`. 5 repeat runs. | **5/5 runs**: round-trip succeeds every time (`ok: round-tripped pid 1234`), and `Pid::from_raw(0)` is confirmed `None` every time - deterministic, no layout/seed dependency, since this is pure computation with no real syscall. Confirms the type-level half of the argument directly in real compiled/interpreted code, not just by reading the signatures. A companion **positive control** (`positive_control_ident_zero_would_panic`, same file) hand-transmutes a `libc::kevent` with `ident = 0, filter = EVFILT_PROC` directly into an `Event` (bypassing `Event::new` on purpose, exactly the way *only* non-rustix code calling the raw syscall directly could) and confirms `Event::filter()` really does panic on that input - i.e. the suspected panic path is real and reachable in principle, it just requires a precondition (`ident == 0` on an `EVFILT_PROC` kevent) that the kernel source review above shows a real kernel never produces from a rustix-driven registration. |
| `cargo build --target x86_64-apple-darwin|x86_64-unknown-freebsd --features "event,std" --lib` | Confirms the investigated code is real, shipped, reachable (not dead code) on both an Apple and a non-Apple BSD target | Both build cleanly. |

See `output.txt` for the full captured output of every run above, plus the exact kernel-source
lines quoted.

## Status

**Not reproducible.** Real `kevent()` execution against a genuine BSD/Darwin kernel was not
possible in this environment (no BSD/macOS hardware or emulation available, no qemu installed, and
disk space was too tight - 11G free - to attempt installing one). Within that constraint, the
question was fully answered analytically instead: (1) rustix's own public API structurally cannot
register an `EVFILT_PROC` event with `ident == 0` (`Pid` is `NonZeroI32`), confirmed by a real
compiled/Miri-interpreted round-trip, 5/5 runs; and (2) three independent, actively-maintained BSD
kernel codebases (FreeBSD, XNU, NetBSD) all copy the registration-time `ident` through verbatim and
never recompute it for `EVFILT_PROC`, so no in-kernel path turns a nonzero registered pid into `0`
later. A positive control confirms the panic is real *if* that precondition were ever violated -
but no mechanism was found, in rustix's own API or in any of the three kernels' source, that could
violate it starting from a rustix-driven registration.

**Fully reproducible**: N/A - Status is Not reproducible, not Confirmed.

## Class

N/A (not reproducible)

## Root cause

Not a presently-triggerable bug via rustix's own public API, for two independent, compounding
reasons:

1. **rustix itself cannot construct the triggering state.** The only public constructor for an
   `Event`, `Event::new`, requires an actual `Pid` for `EventFilter::Proc`, and `Pid` is
   `NonZeroI32`-backed - there is no public rustix API that produces a `Pid` with raw value `0`.
   So a `kevent()` changelist built entirely through rustix's own types can never contain an
   `EVFILT_PROC` registration with `ident == 0` to begin with.
2. **No real kqueue kernel implementation recomputes `ident` for this filter after registration.**
   Checked directly in current upstream source for FreeBSD, Apple/XNU, and NetBSD: the `ident`
   field of an `EVFILT_PROC` knote is set once, verbatim, from the registering `kevent`'s `ident`
   field, and returned unchanged in every subsequent report of that knote - there is no kernel-side
   mechanism (process exit, reparenting to a reaper, pid reuse, etc.) that alters it.

The `.unwrap()` is therefore defended by a real, verified kernel contract (the kernel only ever
echoes back exactly what a well-typed caller registered), not merely an assumption - the same shape
of outcome as this workspace's candidates 046, 049, 051, 052, 056, 057, and 062. The one residual
gap (see "What's not done" below) is the same one several of those candidates already recorded: a
non-rustix caller sharing the same process/kqueue could in principle register a raw `ident == 0`
`EVFILT_PROC` kevent directly via `libc::kevent`, bypassing rustix's typed API entirely, and
`Event::filter()` would then panic decoding it - but that is a different participant entirely, not
something `Event::filter()` decoding rustix's own registrations needs to defend against, and no
real kernel source reviewed here rejects or re-zeroes such a registration either (FreeBSD's
`pfind(0)`/XNU's `proc_find(0)` behavior for a literal `ident = 0` registration attempt was not
independently chased down, since it is already outside what rustix's own API can produce).

## Introduced by

N/A - not a presently-triggerable bug, so there is nothing to attribute a fix to. For reference
only: the `Pid::from_raw(...).unwrap()` pattern at this exact call site was introduced by 0f1032a1
"Implement the kqueue API (#488)" (2023-03-02); eb68ba30 "Reconsider guarantees of Pid and refactor
(#657)" (2023-06-16) later made `Pid::from_raw` safe (removing an `unsafe` wrapper around the same
call) without changing the `.unwrap()` itself.

## The fix

No fix needed/applied - Status is Not reproducible, and no source file in the main project's
working tree was modified this session (all investigation happened in an isolated worktree using
only a new, non-shipped `examples/` file that was never merged back).

- Project test suite: N/A - no code change was made, so there is no before/after to compare. (The
  project's own test suite also does not build/run this `cfg(bsd)`-only module at all in this
  Linux container, so a `cargo test --workspace` run here would not have exercised this code
  either way.)

## What's not done / open questions

- Not verified against a real FreeBSD/NetBSD/OpenBSD/DragonFly/Apple kernel's actual `kevent()`
  behavior end-to-end (no such hardware/emulation available in this environment - the same
  category of gap this workspace's candidates 035/038/039/057/058 already recorded for other
  BSD/macOS-only code paths). The analysis above resolves the question via upstream kernel source
  for 3 of the 5 `cfg(bsd)` kernel families (FreeBSD, XNU, NetBSD); OpenBSD's and DragonFlyBSD's own
  `kern_event.c` were not separately checked, for the same reason named in several prior
  candidates' "what's not done" sections - diminishing returns once 3 independently-maintained
  kqueue implementations all showed the identical verbatim-copy-through behavior and no signal of
  divergence appeared.
- `EventFilter::Signal`'s adjacent decode arm in the same function
  (`Signal::from_raw_unchecked(self.inner.ident as _)`, `src/event/kqueue.rs:124`) was noticed as a
  structurally similar sibling while reading this file, but candidate 057 already flagged this
  exact call site as an unchased sibling in its own "what's not done" section - not re-investigated
  here to avoid duplicating that note.
- The scenario of a non-rustix participant in the same process registering a raw `ident == 0`
  `EVFILT_PROC` kevent directly via `libc::kevent` (bypassing rustix's typed API) was identified as
  the one theoretical way to reach the panic, but not chased further (would require confirming
  whether `pfind(0)`/`proc_find(0)` even succeeds for FreeBSD's/XNU's pid-0 kernel process, which
  is a question about a different, non-rustix-driven threat model than what this candidate was
  scoped to).
