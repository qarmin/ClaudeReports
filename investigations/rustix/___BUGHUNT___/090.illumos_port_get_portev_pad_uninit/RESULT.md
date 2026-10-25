# illumos_port_get_portev_pad_uninit

Second look at a carried-forward suspect ("bsd_solaris_kqueue_event_assume_init_untested"):
BSD `kqueue()`/`kevent()` and Solaris/illumos `port_get()` both construct their whole real-OS
`Event` struct via `MaybeUninit::<T>::uninit()` -> (real syscall) -> `assume_init()`, trusting the
kernel to have fully initialized every byte, the same general shape already confirmed buggy for
Apple's `fstat` reserved fields (candidate 039). Candidate 050 tried Miri cross-interpretation here
and hit "no foreign-function shim for `kevent()`/`port_create()` on these target OSes" - a tooling
gap, not evidence of safety. This session re-attempted with a newer Miri, and - when that hit the
identical gap - went to the actual upstream kernel source instead.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Miri cross-interpretation of rustix's real public API (`rustix::event::port::create()`/`get()`, `rustix::event::kqueue::kqueue()`), using the newest Miri available in this environment (`miri 0.1.0 (5c543b0b8c 2026-09-29)`, newer than candidate 050's) | `--target x86_64-unknown-illumos` and `--target x86_64-apple-darwin`/`x86_64-unknown-freebsd`/`x86_64-unknown-netbsd`, `MIRIFLAGS="-Zmiri-disable-isolation"` | Identical tooling gap to candidate 050, confirmed on a newer Miri: "unsupported operation: can't call foreign function `port_create` on OS `illumos`" and "... `kqueue` on OS `macos`/`freebsd`/`netbsd`" - Miri still has no shim at all for the syscall that would even create the fd, let alone retrieve an event. See output.txt. |
| Read the real upstream kernel source for both sides of the candidate: illumos-gate's `usr/src/uts/common/fs/portfs/port.c` (`port_get`/`port_getn`/`port_copy_event`) and FreeBSD's `sys/kern/kern_event.c` (`kevent_copyout`/`knote_to_kevent`-equivalent paths), fetched live from each project's own GitHub mirror, plus every BSD/Apple variant's `struct kevent` definition in the vendored `libc` crate (0.2.189) | `curl` against `raw.githubusercontent.com/illumos/illumos-gate` and `raw.githubusercontent.com/freebsd/freebsd-src`, cross-checked against `libc-0.2.189`'s struct definitions for apple/freebsd/netbsd/openbsd/dragonfly/solarish | **illumos/Solaris side: confirmed real bug.** `port_event_t` has an explicit `portev_pad: c_ushort` field; `port_copy_event()` only ever assigns `portev_source`/`portev_object`/`portev_user`/`portev_events` (`portev_pad` is never assigned anywhere in the file), and the backing array is allocated via `kmem_alloc()` (not `kmem_zalloc()`) before being `copyout()`'d whole to userland. **BSD `kqueue()`/`kevent()` side: ruled out.** None of apple/FreeBSD(pre-13)/NetBSD/OpenBSD/DragonFly's plain `struct kevent` has a reserved/padding field; FreeBSD 13+'s added `ext: [u64; 4]` field is always either round-tripped verbatim from the user's own `EV_ADD` registration (which rustix's `Event::new` always zero-initializes via `..unsafe { zeroed() }`) via `*kevp = kn->kn_kevent`, or explicitly `bzero(&kevp->ext, sizeof(kevp->ext))` in the legacy 32-bit-compat copyout path - no code path returns an uninitialized `ext`. See evidence/illumos_gate_port.c_excerpt.txt. |
| Synthetic Miri reproduction, cross-interpreting `--target x86_64-unknown-illumos`: construct a `MaybeUninit::<libc::port_event>::uninit()` (same type rustix's internal `c::port_event` resolves to, via `src/backend/libc/c.rs`'s `pub(crate) use libc::*;`), write exactly the 4 fields the real kernel's `port_copy_event()` is proven to write, leave `portev_pad` untouched, then `assume_init()` - first as a temporary `#[cfg(all(test, solarish))]` unit test added directly to `src/backend/libc/event/syscalls.rs` (reverted before this folder was written), then as the standalone `repro/` below | debug build, 5 total runs (1 initial + 4 repeats) for the buggy (`uninit()`) pattern; 1 run for the patched (`zeroed()`) pattern | Buggy pattern **reproduces 5/5 runs**, deterministic: "Undefined Behavior: constructing invalid value of type `libc::port_event`: at `.portev_pad`, encountered uninitialized memory, but expected an integer", at the exact `event.assume_init()` expression. Patched pattern (`zeroed()`) runs cleanly, no UB, 1/1 (and later re-verified 3/3 when the test was temporarily restored against the patched source). |

## Status

**Confirmed** (the Solaris/illumos `port_event`/`portev_pad` half of this candidate).
**Not reproducible** (the BSD `kqueue()`/`kevent()` half - ruled out by kernel-source reading, not
merely untested; see root cause below for why).

Reproduces 5/5 runs under Miri (debug build, cross-interpreting `--target x86_64-unknown-illumos`,
`MIRIFLAGS="-Zmiri-disable-isolation"`), fully deterministic - this is a pure type/memory-layout
property (an uninitialized `u16` field in a `#[repr(C)]`-style struct), not something that depends
on heap layout or scheduling, so there is no expectation it would ever flip to "doesn't reproduce".

**Fully reproducible**: **No**, with the specific reason CONFIRMATION.md section 11 requires
stated explicitly: no real Solaris/illumos hardware is available in this sandbox, and Miri - even
the newest nightly available here (`miri 0.1.0 (5c543b0b8c 2026-09-29)`, confirmed this session) -
has no foreign-function shim for `port_create()`/`port_get()` on OS "illumos" at all, so rustix's
real, public `rustix::event::port::get()` cannot be driven to completion by any tool available in
this environment (see output.txt for the exact "unsupported operation" diagnostic). This is a
single-participant finding (rustix reacting to what its own OS's kernel writes, not a
peer/caller-driven interaction), so section 11's two-participant bar for a "real second
participant" doesn't strictly apply the way it would for a network protocol - but the practical
effect is the same: the real syscall itself cannot be executed here. Per section 11's explicit
hardware exception, this candidate is still written up as Confirmed, backed by: (1) the real
upstream illumos-gate kernel source (archived in `evidence/`, fetched live this session, not
assumed or guessed at) proving exactly which bytes of `port_event_t` the real kernel leaves
untouched before `copyout()`, and (2) a Miri-verified reproduction of the exact resulting Rust-level
UB, applied to the identical `libc::port_event` type rustix's own code uses internally (not a
lookalike type - see repro/Cargo.toml for why this is the same type, not an approximation).

## Class

UB (also a real OS-level information-disclosure of 2 bytes of kernel heap memory into user space on
every real `port_get()`/`port_getn()` call on real illumos/Solaris - see "Root cause"; rustix itself
does not currently expose a public accessor for `portev_pad`, so this is UB-at-construction-time
plus a latent/dormant information leak, not yet an actively-exploited one through rustix's own API).

## Root cause

**Confirmed (illumos/Solaris, `src/backend/libc/event/syscalls.rs::port_get`):**

```rust
let mut event = MaybeUninit::<c::port_event>::uninit();
unsafe {
    ret(c::port_get(borrowed_fd(port), event.as_mut_ptr(), timeout as _))?;
}
Ok(Event(unsafe { event.assume_init() }))
```

`c::port_event` is `libc::port_event` (via `src/backend/libc/c.rs`'s `pub(crate) use libc::*;`),
defined (libc 0.2.189, `src/unix/solarish/mod.rs`) as:

```rust
pub struct port_event {
    pub portev_events: c_int,
    pub portev_source: c_ushort,
    pub portev_pad: c_ushort,      // <-- never written by the real kernel
    pub portev_object: crate::uintptr_t,
    pub portev_user: *mut c_void,
}
```

The real illumos kernel's `port_getn()` (which `port_get()` is implemented as, with `max = 1` -
see `usr/src/uts/common/fs/portfs/port.c` around its syscall dispatch) allocates its per-call
buffer of `port_event_t` with `kevp = kmem_alloc(eventsz * nmax, KM_NOSLEEP)` (falling back to
`kmem_alloc(..., KM_SLEEP)`, never `kmem_zalloc`) - i.e. **uninitialized kernel heap memory**. It
then fills each slot via `port_copy_event()`, which assigns only:

```c
puevp->portev_source = pkevp->portkev_source;
puevp->portev_object = pkevp->portkev_object;
puevp->portev_user   = pkevp->portkev_user;
puevp->portev_events = pkevp->portkev_events;
```

`portev_pad` is never assigned anywhere in `port.c`. The whole buffer - including the 2
still-uninitialized `portev_pad` bytes per event - is then `copyout()`'d directly to the calling
userland process. So on a real illumos/Solaris kernel, every `port_get()`/`port_getn()` call
returns a `port_event_t` whose `portev_pad` field holds 2 bytes of leftover kernel heap data, not a
defined value. rustix's `event.assume_init()` is a typed read of the whole struct, which is
immediate UB the instant it executes if any byte (here, `portev_pad`) is uninitialized - regardless
of whether rustix's own `Event` type later exposes that specific field to a caller.

**Ruled out (BSD `kqueue()`/`kevent()`, `src/event/kqueue.rs` + `src/backend/libc/event/syscalls.rs::kevent`):**
checked every BSD-family `struct kevent` (apple/FreeBSD/NetBSD/OpenBSD/DragonFly, via the vendored
`libc` crate) for the same shape (an explicit reserved/pad field, or implicit struct padding from
misaligned field ordering) and found none - every field in the "plain" (non-`kevent64`) struct is a
real, meaningful value with no gaps. FreeBSD 13+ added an extra `ext: [u64; 4]` field; reading
`sys/kern/kern_event.c` shows every event handed back to userland via `kevent_copyout()` either
carries a full copy of `kn->kn_kevent` (which itself was populated wholesale from the user's own
original `EV_ADD` registration struct - and rustix's `Event::new()` always zero-fills the whole
`kevent_t` via `..unsafe { zeroed() }` before setting the real fields, so `ext` is never
kernel-uninitialized memory in the first place), or is explicitly `bzero(&kevp->ext,
sizeof(kevp->ext))` in the legacy 32-bit-compat copyout path. This is a structurally different,
and safe, situation from illumos's `port_event` - no evidence of the same bug class was found here.
(Candidates 067/068's *already-confirmed* EVFILT_PROC/EVFILT_SIGNAL-specific field bugs are a
separate, narrower issue from this candidate's "trust the whole struct" question, and are
unaffected by this conclusion either way.)

## Introduced by

26f1d861e16e6372e49cfedab88455120cfa66ad "feat: Expose the Solarish event ports API (#503)"
(2023-03-02) - this commit introduced `port_get`'s `MaybeUninit::<c::port_event>::uninit()` /
`assume_init()` pattern in its original and only form (verified via `git log -S
"MaybeUninit::<c::port_event>::uninit"` across the file's whole history, which resolves to exactly
this one commit).

## The fix

Changed `port_get` (`src/backend/libc/event/syscalls.rs`) from `MaybeUninit::<c::port_event>::uninit()`
to `MaybeUninit::<c::port_event>::zeroed()`. This is sufficient because every field of
`port_event` (a `c_int`, two `c_ushort`s, a `uintptr_t`, and a raw pointer) permits an all-zero bit
pattern, so zeroing is always a valid starting state; the real syscall then overwrites the 4 real
fields it actually produces, and `portev_pad` - which the kernel never touches - is left at a
well-defined `0` instead of indeterminate heap bytes. Verified via the same Miri reproduction
mechanism used to confirm the bug (see "What was tried" and repro/): the patched pattern runs
clean under Miri with no UB, 3/3 repeat runs, no flakiness either way.

**Checked against more than the one triggering byte pattern (CONFIRMATION.md section 7)**: the
fix changes the allocation strategy itself (`zeroed()` instead of `uninit()`), not a check against
one specific observed uninitialized value - it is correct for *any* leftover byte pattern
`portev_pad` might have held, not just whatever Miri happened to leave there.

**`port_getn`/`getn_query` (same file) share the identical root cause but are NOT fixed this
session**: they fill a caller-supplied buffer via the generic `Buffer<Event>` trait
(`events.parts_mut()` then `events.assume_init(nevents)` in `src/event/port.rs::getn`), where the
buffer's own initial-state guarantee is the generic `Buffer` trait's contract (shared with every
other backend that uses it - `recv`, `io_uring`, BSD `kevent`, etc.), not something `port_getn`
itself controls the same direct way `port_get`'s single local `MaybeUninit` does. Fixing this
properly means either changing the `Buffer` trait's own contract or having every caller
pre-zero its buffer before the syscall - a broader, cross-cutting change out of scope for this
session's bounded fix (same conclusion candidate 050 reached for the identical `Buffer<Event>`
sites in both `port_getn` and BSD `kevent`).

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64-unknown-linux-gnu): **411 passed, 1 failed, identical before and after** - the one
  failure (`ioctl::test_int_setter`, `NotFound` opening `/dev/net/tun`) is the same pre-existing,
  environment-caused failure documented in `projects/rustix.md`, unrelated to this fix. No new
  failures introduced. (The `solarish`-only code this fix touches isn't compiled at all on the
  `x86_64-unknown-linux-gnu` host used for this test run - the Miri reproduction above is the only
  thing that actually exercises the changed line in this environment.)

## What's not done / open questions

- **`port_getn`/`getn_query`'s own `Buffer<Event>::assume_init(nevents)` path has the identical
  root cause** (the caller's buffer can likewise have an uninitialized `portev_pad` per slot) but
  is not fixed this session - see "The fix" above. A maintainer fixing this properly would need to
  decide whether to zero the whole buffer before the syscall (safe, cheap, a few extra writes) or
  revisit the `Buffer` trait's contract more broadly.
- **BSD `kqueue()`/`kevent()`'s "ruled out" conclusion was reached by kernel-source reading, not
  independently Miri/hardware-verified** (Miri has no shim for `kqueue()` on any BSD-family OS -
  see output.txt), so it rests on the FreeBSD source specifically plus the `libc` crate's struct
  layouts for the others - not an exhaustive reading of NetBSD/OpenBSD/DragonFly's own kernel
  source (only their public struct definitions were checked, not their kernel-side copyout code).
  If a future session gets real BSD hardware or a Miri shim, re-verifying NetBSD/OpenBSD/DragonFly
  kernel-side behavior directly (not just inferring safety from "no reserved field exists") would
  close this out completely.
- **Oracle Solaris proper (closed-source) was not independently checked** - only illumos-gate
  (the open-source fork) was read. illumos and Solaris share common ancestry and the same
  `port_event_t` ABI/ioctl-based API, so the same bug shape very likely applies, but this wasn't
  confirmed against Solaris's own (non-public) kernel source.
- **`portev_pad` is not currently exposed via any public accessor on `rustix::event::port::Event`**
  (`events()`/`object()`/`userdata()` only) - so today this is "only" construction-time UB, not an
  actively-reachable information leak through rustix's own API. If a future API change ever adds a
  `Debug` derive or a raw-struct accessor to `Event`, the leak becomes externally observable too;
  worth flagging to maintainers as a reason to fix `port_getn` as well, not just `port_get`.
- A longer Miri-shim-availability check (newer Miri version, or the `rust-lang/miri` issue tracker
  for `port_create`/`kqueue` FFI support) was not performed beyond confirming the newest nightly
  available in this sandbox (2026-09-29) still lacks it.
