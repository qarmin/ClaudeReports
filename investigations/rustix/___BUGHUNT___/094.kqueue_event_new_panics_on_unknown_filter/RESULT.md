# kqueue_event_new_panics_on_unknown_filter

`src/event/kqueue.rs`'s `Event::filter()` decodes any real `kevent()`-returned filter type it
doesn't have typed support for into the public, `#[non_exhaustive]` `EventFilter::Unknown`
variant - reachable via the safe public API simply by calling `.filter()` on a real `kevent()`
result whose filter is one of the many real BSD/Apple kqueue filters rustix never enumerates
(e.g. Apple's `EVFILT_MACHPORT`/`EVFILT_VM`/`EVFILT_EXCEPT`/`EVFILT_AIO`, or `EVFILT_USER` on
NetBSD/OpenBSD where rustix's `User` variant is cfg-gated to `apple`/`freebsdlike` only). `Event::
new()` (the only public constructor) however does `EventFilter::Unknown => panic!("unknown
filter")`, so a generic "decode an event, then re-register/re-submit it" pipeline that round-trips
a decoded `Unknown` back through the constructor aborts the process instead of returning a
recoverable error - a state-machine invariant violated by a value the crate's own public decode API
can hand back to it. Flagged while reading this same file during candidates 067/068's own
investigation of this module.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: which real filter constants exist in the `libc` crate (the actual kernel-defined values a real `kevent()` syscall can report) vs. which ones `Event::filter()`'s `match` actually handles | `src/event/kqueue.rs` `Event::filter()`/`Event::new()`; `libc` 0.2.189's `src/unix/bsd/apple/mod.rs` and `src/unix/bsd/freebsdlike/freebsd/mod.rs` | `Event::filter()` only handles `EVFILT_READ`/`WRITE`/`EMPTY`(FreeBSD only)/`VNODE`/`PROC`/`SIGNAL`/`TIMER`/`USER`(`apple`/`freebsdlike` only). Real, actively-used Apple filters it does NOT handle: `EVFILT_AIO` (-3), `EVFILT_MACHPORT` (-8, used by CoreFoundation/XPC for Mach-port readiness), `EVFILT_VM` (-12), `EVFILT_EXCEPT` (-15). `build.rs` confirms `freebsdlike` = FreeBSD/DragonFly only and excludes NetBSD/OpenBSD (`netbsdlike`), so `EVFILT_USER` - a real, kernel-supported filter on NetBSD (since 6.0) and OpenBSD (since 5.3) - also falls outside rustix's typed `User` variant on those two OSes even though the kernel genuinely implements it. Any of these, returned by a real `kevent()` call (registered via a raw `libc::kevent` call directly, since rustix has no typed constructor for them), decodes through rustix's own public `Event::filter()` to `EventFilter::Unknown`. |
| Environment check: can this be exercised with a real `kevent()` syscall against real Apple/BSD hardware or emulation | `df -h` (11G free / 916G, 99% used), `which qemu-system-x86_64` (absent, only `qemu-img`), network reachable (`curl` to freebsd.org succeeded) | No real or emulated BSD/Apple kernel available in this sandbox - same constraint already recorded for this exact file by candidates 067/068. Installing/building one was judged impractical in the time budget for one candidate (same conclusion those two reached). |
| Miri cross-interpretation of the real, unmodified `Event::filter()`/`Event::new()` logic for the real `x86_64-apple-darwin` target, feeding in a `kevent_t` transmuted (via `Event`'s own `#[repr(transparent)]` layout) to carry each of the four real Apple filter constants above, standing in for what a real `kevent()` syscall writes into the caller's eventlist buffer (the kernel is the only thing simulated - not a second rustix-side participant) | `cargo +nightly miri run --target x86_64-apple-darwin --features "event,std,process,fs"`, `repro/src/main.rs` (then as `examples/kq_unknown_filter_check.rs` during live investigation) | 3/3 runs: all four filters (`EVFILT_AIO`/`EVFILT_MACHPORT`/`EVFILT_VM`/`EVFILT_EXCEPT`) decode to `EventFilter::Unknown` every time, and feeding that `Unknown` back into `Event::new()` panics with `"unknown filter"` every time (`src/event/kqueue.rs:68:37`), deterministic, no layout/seed dependency (pure computation, no real syscall involved). See `output.txt`. |
| `cargo +nightly check --target x86_64-apple-darwin --features "event,std,process,fs" --lib`/`--examples` (real cross-compile type-check, not Miri) | Confirms the investigated code is real, shipped, reachable on a real Apple target (not dead code), and that the shipped `examples/kq.rs` still compiles against the patched signature | Both succeed cleanly (linking itself is skipped - no Apple SDK/linker available in this sandbox, `cargo check` only). |

## Status

**Confirmed.** Reproduces 3/3 runs under Miri cross-interpretation (`--target x86_64-apple-darwin`,
nightly), deterministic every time (pure computation, no real syscall, no layout/seed dependency) -
see `output.txt`.

**Fully reproducible**: **No**, with an explicit justification per `CONFIRMATION.md` section 11's
stated exception. The finding is about how `Event::new()` (a `rustix`-side consumer) reacts to a
value `Event::filter()` (also `rustix`-side) handed back to it after decoding real kernel-reported
data - the "other participant" here is the kernel itself via a real `kevent()` syscall, not a
second rustix-side caller, so this is the single-external-artifact case section 11 says is fine to
repro directly (analogous to a malformed file handed to a parser). The one piece this environment
genuinely cannot do is *execute* that real syscall: no BSD/Apple hardware or emulation is available
in this sandbox (confirmed via `df -h`/`qemu-system-x86_64` absence, same constraint candidates
067/068 hit on this identical file), and setting one up was judged impractical for a single
candidate's time budget. Unlike 067/068 (which concluded `Not reproducible` because their own
precondition - `ident == 0` echoed back from a rustix-driven registration - was shown to be
*unreachable* via any real kernel's own source), this candidate's precondition (a real,
non-rustix-typed kqueue filter appearing on a kqueue a rustix caller is also polling) is
straightforwardly reachable on a real kernel - `EVFILT_MACHPORT` in particular is routinely
registered by Apple system frameworks (CoreFoundation run loops, XPC) on file descriptors a mixed
Rust/Objective-C or Rust/C process could plausibly also be polling via rustix. What was substituted
instead - Miri cross-interpreting the real, unmodified source for the real target, fed a `kevent_t`
carrying a real, named kernel filter constant (not an arbitrary sentinel) - demonstrates the panic
is real and reachable on real kernel-producible input; it does not demonstrate it via an actual
`kevent()` syscall round trip, which is the part hardware-unavailability blocks.

## Class

logic (process abort / panic on a value the crate's own public decode API can legitimately produce
and hand back to its own constructor; not memory-unsafe - `panic!` unwinds/aborts cleanly, no UB).

## Root cause

`EventFilter` is `#[non_exhaustive]` and has an `Unknown` variant specifically so `Event::filter()`
can represent "the kernel reported a filter type we don't have typed support for" instead of
panicking at decode time. But the matching encode side, `Event::new()`, treats reaching
`EventFilter::Unknown` as a programmer error worth a hard panic rather than a possible runtime
state - even though `Unknown` is not something a caller constructs on purpose, it's exactly what a
decode-then-reconstruct pipeline (log an event generically, transform it, re-arm/re-submit it) gets
handed whenever the underlying kqueue carries a filter type outside rustix's enumerated set. The
set of filter types rustix enumerates is a strict subset of what real kqueue-kernel implementations
support (new filters have been added to Apple/FreeBSD/NetBSD/OpenBSD over the years - `EVFILT_
MACHPORT`/`VM`/`EXCEPT`/`AIO` on Apple, `EVFILT_USER` on NetBSD/OpenBSD outside rustix's `apple`/
`freebsdlike` gate), so this gap isn't theoretical - it grows as rustix's typed surface lags behind
each kernel's own filter list, and the resulting value is always representable (`Unknown` exists
precisely to hold it) up until the moment something tries to turn it back into a registration.

## Introduced by

0f1032a1 "Implement the kqueue API (#488)" (2023-03-02) - the very first commit implementing the
kqueue API already included both `EventFilter::Unknown`'s decode fallback (`_ =>
EventFilter::Unknown`) and `Event::new()`'s `EventFilter::Unknown => panic!("unknown filter")` arm,
with the `Unknown` variant's own doc comment explicitly noting "Passing this into `Event::new()`
will result in a panic" - this was a deliberate, documented design choice from the start, not a
later regression.

## The fix

Changed `Event::new()`'s signature from `fn new(...) -> Event` to `fn new(...) -> io::Result
<Event>`, returning `Err(io::Errno::INVAL)` for `EventFilter::Unknown` instead of panicking (same
shape as every other fallible rustix constructor). This is a breaking API change (same category as
several already-applied fixes in this workspace, e.g. candidates 006/019/064/065) - the only
in-tree call site, `examples/kq.rs`, was updated to propagate the new `Result` with `?` at each of
its 4 `Event::new(...)` call sites (its own `fn main() -> std::io::Result<()>` already has a
working `From<Errno> for std::io::Error` impl available, so `?` converts cleanly). No test in the
workspace references `kqueue`/`Event::new` at all (`grep -rln kqueue tests/` found nothing), so no
test changes were needed.

Verified: `repro/` (via its `fixed` Cargo feature selecting the post-patch, `Result`-returning code
path) re-run under the same Miri cross-interpretation against the patched tree returns
`Err(Errno::INVAL)` - no panic - for all four tested real kernel filter values
(`EVFILT_AIO`/`MACHPORT`/`VM`/`EXCEPT`), confirming the fix holds across more than just one
triggering filter value, not only the one first tried (`CONFIRMATION.md` section 7). `cargo
+nightly check --target x86_64-apple-darwin --features "event,std,process,fs" --lib --examples`
also confirms the whole crate (including the now-updated `examples/kq.rs`) still type-checks
cleanly for a real Apple target.

- Project test suite: 411 passed / 1 failed / 5 ignored, identical before and after the patch
  (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`, 21 test binaries).
  The 1 failure (`ioctl::test_int_setter`, `NotFound` opening `/dev/net/tun`) is this sandbox's own
  pre-existing, environment-caused failure (see `___BUGHUNT___/README.md` and the skill's own
  `projects/rustix.md`), present identically in both runs - this module is `cfg(bsd)`-only, so
  none of its code is even compiled on this x86_64-linux-gnu host either way, before or after.

## What's not done / open questions

- Not verified against a real Apple/FreeBSD/NetBSD/OpenBSD/DragonFly kernel's actual `kevent()`
  behavior end-to-end - no such hardware/emulation available in this environment (`df -h`: 11G
  free of 916G; no `qemu-system-x86_64`; same gap already recorded by candidates 035/038/039/057/
  058/067/068 for other BSD/macOS-only code in this crate). A future session with access to real
  or emulated Apple/BSD hardware could register one of the exotic filters above via a raw
  `libc::kevent` call, wait on it through rustix's own `kevent_timespec`, and confirm the full
  syscall-to-panic chain directly rather than via Miri's simulated "kernel wrote this" step.
  Candidates 067/068 record the same gap and the same reasoning for why it wasn't chased further
  this session (disk space, no qemu, diminishing returns after 067/068 already established the
  same constraint on this identical file).
- Sibling filter-encoding paths that might share a similar "crate enumerates a strict subset of
  what the kernel supports" shape were not audited this session - this candidate only checked
  Apple's filter list and the FreeBSD/NetBSD/OpenBSD `EVFILT_USER` gating; FreeBSD's own
  `EVFILT_PROCDESC`/`EVFILT_SENDFILE`/`EVFILT_EMFILE` and NetBSD/OpenBSD/DragonFly's full filter
  lists were not individually enumerated against rustix's `match` arms.
- A longer fuzzing campaign isn't a good fit here (the input space is a small, enumerable set of
  kernel filter constants per OS, not an adversarial byte stream) - manual boundary-value/kernel-
  source-list cross-referencing, as done here, is the right-sized tool for this bug class; no
  further escalation via `fuzz-bug-repro` is warranted.
