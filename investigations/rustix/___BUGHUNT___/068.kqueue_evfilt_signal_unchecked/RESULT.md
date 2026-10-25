# kqueue_evfilt_signal_unchecked

`src/event/kqueue.rs:124`'s `Event::filter()`, in its `EVFILT_SIGNAL` decode arm, does
`unsafe { Signal::from_raw_unchecked(self.inner.ident as _) }` on `self.inner.ident` - a raw field
of a `kevent_t` struct produced by a real `kevent(2)` syscall. `Signal::from_raw_unchecked`'s body
(`src/signal.rs:232-234`) is `Self::from_raw_nonzero_unchecked(NonZeroI32::new_unchecked(sig))`,
i.e. unlike candidate 057's `from_raw_nonzero_unchecked` (whose body is just `Self(sig)`, already
checked non-zero by the caller), this one calls `NonZeroI32::new_unchecked` directly on a
still-possibly-zero raw value - the same structural shape as candidate 035's confirmed-UB
`Pid::from_raw_unchecked`. Investigated (per 057's own "what's not done" note flagging this as an
uninvestigated sibling) to determine whether the kernel can ever actually hand back `ident == 0`
for a triggered `EVFILT_SIGNAL` event, which would make this real, reachable Undefined Behavior.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: compare `from_raw_unchecked`'s body against `from_raw_nonzero_unchecked`'s (`src/signal.rs:213-254`), and against candidate 035's confirmed `Pid::from_raw_unchecked` | - | `from_raw_unchecked` calls `NonZeroI32::new_unchecked(sig)` directly on an unchecked raw `i32` - structurally identical to 035's confirmed-UB case, not to 057's safe `Self(sig)` body. The call site's own `SAFETY` comment ("`EventFilter::new` requires a valid `Signal`") only reasons about registration through this crate's own typed API, not about what a real `kevent(2)` result can contain. |
| Miri boundary-value probe: temporary `#[test]` in `src/signal.rs` calling the real `Signal::from_raw_unchecked` with `0` (adversarial) and 8 nonzero control values (`1`, `-1`, `i32::MAX`, `i32::MIN`, `34`/`64` glibc-shaped, `65`/`126` FreeBSD-real-time-signal-shaped) | `RUSTFLAGS="--cfg rustix_no_linux_raw" cargo +nightly miri test --features "std,all-apis,use-libc" --lib candidate_068`, 3 repeat runs | **Deterministic, 3/3 runs**: `from_raw_unchecked(0)` is real Undefined Behavior (`error: Undefined Behavior: entering unreachable code` at `NonZeroI32::new_unchecked`, identical diagnostic shape to candidate 035's `Pid` case). All 8 nonzero control values round-trip cleanly with no diagnostic, every run. See `output.txt` "Step 2". |
| Kernel source research: can a real `kevent(2)` wait call ever return a *triggered* `EVFILT_SIGNAL` event with `ident == 0`, on FreeBSD and macOS/XNU (the real upstream sources, fetched live) | `curl` of `cgit.freebsd.org/src` (`sys/kern/kern_sig.c`, `sys/sys/_sigset.h`) and `github.com/apple-oss-distributions/xnu` (`bsd/kern/kern_sig.c`, `bsd/kern/kern_event.c`) | **No, on both platforms, for a structural reason independent of this call site**: see "Root cause" below. See `output.txt` "Step 3" for the exact source excerpts. |
| Git archaeology: which commit changed the decode side from a safe, checked constructor to the unsafe unchecked one | `git log --follow -- src/event/kqueue.rs`, `git show fc2a014f -- src/event/kqueue.rs` | fc2a014f0eb124710669b57e883166c15d29225f ("Support `SIGRTMIN+n` signals. (#1292)", 2025-01-28) replaced `Signal::from_raw(self.inner.ident as _).unwrap()` (a safe, checked constructor that would have **panicked** cleanly on an invalid value) with the current unsafe unchecked call - as part of moving `Signal::from_raw`'s libc-dependent implementation out to a separate `rustix-libc-wrappers` crate (confirmed: no `from_raw` function exists in current `src/signal.rs` at all, only in that external crate per its own doc comments). |

## Status

**Not reproducible.** The specific `unsafe` primitive under suspicion (`NonZeroI32::new_unchecked`
inside `from_raw_unchecked`) genuinely is capable of real UB when given `0` - confirmed directly
under Miri, deterministically, 3/3 runs - so this is not the same shape as candidate 057's finding
(where the inner primitive itself was proven incapable of UB regardless of input). What rules this
candidate out instead is the *reachability* question: real kernel source review (FreeBSD's
`kern_sig.c`/`_sigset.h`, macOS/XNU's `kern_sig.c`/`kern_event.c`, fetched live from each project's
own upstream source) shows that signal number 0 can never actually be delivered through the
kernel's real signal-posting path on either kqueue-capable OS family checked, which makes it
structurally impossible for a real, triggered `EVFILT_SIGNAL` kevent to ever carry `ident == 0` -
see "Root cause" for the exact mechanism.

**Fully reproducible**: N/A - Status is Not reproducible, not Confirmed.

## Class

N/A (not reproducible)

## Root cause

Not a presently-triggerable bug, for a reason specific to how kqueue's `EVFILT_SIGNAL` filter is
wired into each kernel's real signal-delivery path (checked for FreeBSD and macOS/XNU, the two
kqueue-capable OS families this session could review upstream source for):

1. **Registering a knote with `ident == 0` for `EVFILT_SIGNAL` is not rejected at attach time** on
   either kernel - FreeBSD's `filt_sigattach` and XNU's equivalent attach path perform no validation
   of `kn->kn_id`/`kn_id` at all. So, in principle, a raw `kevent(2)` registration call made outside
   this crate's typed API (which always requires an already-valid, nonzero `Signal` via
   `EventFilter::new`) *could* add such a knote to a shared kqueue descriptor.
2. **But that knote can never become "ready."** Both kernels' `EVFILT_SIGNAL` filter (`filt_signal`)
   only increments its internal counter (`kn_data`), which gates whether `kevent(2)` ever reports the
   event as triggered, when a `NOTE_SIGNAL | sig` hint is posted with `sig` matching the knote's
   `ident`. The one and only place either kernel posts that hint is deep inside its real
   signal-delivery function (FreeBSD's `tdsendsignal`, XNU's `psignal_internal`) - and both kernels
   guarantee, independently of each other and of this crate, that signal number 0 is never posted
   through that function: FreeBSD's `tdsendsignal` unconditionally **panics** (`_SIG_VALID(sig)` is
   `false` for `sig <= 0`) if it's ever called with `sig == 0`; XNU's `kill(2)` syscall - the only
   place `psignal()`/`psignal_internal()` gets called from a signal number supplied by a caller -
   explicitly special-cases `signum == 0` to skip calling `psignal()` at all (`if (uap->signum) {
   psignal(p, uap->signum); }`), which is exactly POSIX's own `kill(pid, 0)` "existence/permission
   test only, no signal sent" semantics, implemented structurally rather than as an incidental
   omission.
3. **Net effect**: even though the specific `unsafe` primitive this call site uses is genuinely
   unsound for `ident == 0` (step 2's Miri check confirms this directly), and even though nothing
   in this crate's own code prevents a knote with that ident from being *registered*, no real
   `kevent(2)` wait call can ever return it as a *triggered* event - the data path that would have
   to deliver `ident == 0` back out through `self.inner.ident` is cut off at its source, on both
   kernels checked, by an invariant (signal 0 is never actually delivered) that predates and is
   entirely independent of this crate.

The call site's own `SAFETY` comment ("`EventFilter::new` requires a valid `Signal`") is still
narrower than the real justification - it reasons only about this crate's own registration API, not
about a raw/cross-mechanism registration of `ident == 0` sharing the same kqueue fd - but the gap
turns out not to be exploitable, for the reason above, not because the comment's own reasoning was
actually sufficient.

## Introduced by

fc2a014f0eb124710669b57e883166c15d29225f "Support `SIGRTMIN+n` signals. (#1292)" (2025-01-28) -
this is the commit that changed the decode side from `Signal::from_raw(self.inner.ident as
_).unwrap()` (a safe, checked constructor - an invalid value would have caused a clean panic, not
UB) to the current `unsafe { Signal::from_raw_unchecked(...) }`, as a side effect of moving
`Signal::from_raw`'s libc-dependent implementation out of this crate into a separate
`rustix-libc-wrappers` crate. For reference: the surrounding `EVFILT_SIGNAL` decode arm itself
predates this by about two years (0da77b01f "Add kqueue signal filter (#582)", 2023-03-30), and
originally used the still-safe `Signal::from_raw(...).unwrap()` form the whole time until
fc2a014f0 - so this specific `unsafe`-and-unchecked construction is the newer of the two, not a
long-standing original design decision.

## The fix

No fix needed/applied - Status is Not reproducible, and no actual gap was found to close. If a
maintainer still wants the call site's own `SAFETY` comment to read more defensively (acknowledging
that the real justification is "no kernel checked here can ever deliver ident == 0 for a triggered
EVFILT_SIGNAL event," not just "this crate's own registration API only ever supplies a valid
Signal"), that would be a documentation-only change, not a code fix - same conclusion candidate 057
reached for its own, differently-shaped sibling finding.

- Project test suite: N/A - no code change was made (the temporary Miri probe test added to
  `src/signal.rs` for Step 2 was reverted before this candidate folder was written; `git status`
  in the worktree is clean).

## What's not done / open questions

- Only FreeBSD and macOS/XNU's upstream kernel source were actually fetched and reviewed this
  session (the two OS families whose real source repositories were readily reachable). NetBSD,
  OpenBSD, and DragonFlyBSD also compile this same `EVFILT_SIGNAL` decode arm (per
  `src/event/kqueue.rs`'s cfg gating, the arm itself is not restricted to any specific BSD) but
  were not independently checked - the same structural argument (POSIX's universal `kill(pid, 0)`
  "no signal sent" semantics, which every POSIX-derived kernel's signal-delivery path has to honor
  one way or another) very likely applies there too, but this was not verified against their actual
  kernel source the way it was for FreeBSD/XNU.
- No real kqueue-capable hardware/emulation was available in this environment (same constraint
  candidates 028/035/057 recorded for their own BSD/macOS-only findings) - this candidate's
  conclusion rests entirely on upstream kernel source review plus a Miri-isolated probe of the
  Rust-side primitive, not on an end-to-end run of a real `kqueue()`/`kevent()` call.
- The sibling `EVFILT_PROC` arm (`src/event/kqueue.rs:117-120`, `Pid::from_raw(self.inner.ident as
  _).unwrap()`, not an `unsafe`-unchecked constructor at all currently) and candidate 067's own
  kqueue `EVFILT_PROC`/`Pid` investigation were not re-examined as part of this one-bug-per-candidate
  session.
