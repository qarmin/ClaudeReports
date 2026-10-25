# kernelsigset_unchecked_array_index

`src/kernel_sigset.rs:61-87` - `KernelSigSet::insert`/`remove`/`contains` compute `raw =
sig.as_raw().wrapping_sub(1) as usize` and index `self.0.sig[raw / sigs_per_elt]` with no bounds
check before indexing the fixed-size `kernel_sigset_t` array (1 `c_ulong` element on every arch
this container targets, x86_64). Flagged as a candidate because index math derived from an
externally-influenceable value with no visible range check is exactly `HEURISTICS.md`'s "index/
slice arithmetic near a boundary" pattern, and because a `cfg(linux_raw)` test
(`test_assumptions`) only asserts the per-arch sizing assumption holds for named/real-time signal
constants - it never actually exercises `insert`/`remove`/`contains` with an out-of-range value.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: trace every call site of `KernelSigSet::insert`/`remove`/`contains` in the crate | `grep -rn` across `src/` | Every internal call site (the `Debug` impl's `1..=_NSIG` loop, and the test module's `sigs()`/`libc_sigs()` helpers) only ever constructs `Signal` values already known to be in range (named POSIX signals, or `libc::SIGRTMIN()..SIGRTMAX()`/`KERNEL_SIGRTMIN..KERNEL_SIGRTMAX`, both within the array's bit range by `test_assumptions`' own check). No internal caller currently feeds an out-of-range raw value into these functions. |
| Code reading: trace how an out-of-range `Signal` could ever be constructed | `src/signal.rs` | The only public, *safe* constructors are `Signal::from_named_raw`/`from_named_raw_nonzero`, which return `None` for anything in the `SIGRTMIN..=SIGRTMAX` reserved range and have no path to an arbitrary out-of-range `i32` at all. The only way to get a `Signal` whose raw value is outside the valid 1..=64 range (1..=128 on MIPS) is through the already-`unsafe` `Signal::from_raw_unchecked`/`from_raw_nonzero_unchecked`, whose own doc comments already require "a valid and non-zero signal number" as a precondition - i.e. producing the triggering input itself already requires violating a documented `unsafe` contract. |
| Manual boundary-value construction, debug-assertions build (`cargo run`, dev profile; release profile re-checked too since Rust array/slice indexing bounds-checks unconditionally, not just under `debug-assertions`), `--features "std,io_uring"` | `KernelSigSet::empty().insert(Signal::from_raw_unchecked(64))` (in-range: last valid bit) succeeds; `.insert(Signal::from_raw_unchecked(100_000))` and `.insert(Signal::from_raw_unchecked(-5))` | Both out-of-range calls panic: `index out of bounds: the len is 1 but the index is 1562` (for 100000) and `index out of bounds: the len is 1 but the index is 288230376151711743` (for -5, via `wrapping_sub`'s `i32` underflow then `as usize` sign-extension) |
| Repeat-run check | Same debug/release build, 5 consecutive runs of the standalone repro (see `repro/`) | 5/5 runs: identical panic location (`kernel_sigset.rs:65:9` before the fix) and identical messages every time for both inputs - fully deterministic (a pure function of the hand-picked `raw` input, no heap-layout/ASLR/seed dependency anywhere in this code path) |

A debug build plus hand-picked boundary values was sufficient on the first real attempt (a clear,
deterministic signal immediately) - no need to escalate to miri (this code path is pure
computation with no real syscall, so miri would work, but the plain debug/release build already
gave a clean, repeatable answer) or a fuzz harness (the triggering condition is a single obvious
boundary, not something needing broader input-space coverage), per the soft cap in `SKILL.md`.

## Status

**Confirmed.** Reproduces 5/5 runs, both debug and release profiles (Rust's own array-indexing
bounds check is unconditional, not gated on `debug-assertions`), fully deterministic.

**Important reachability caveat, stated explicitly (this is the actual subject of this
investigation, not an afterthought):** the panic is only reachable by first violating the
documented safety precondition of `Signal::from_raw_unchecked`/`from_raw_nonzero_unchecked` (an
already-`unsafe` call). No code currently in this crate does that when feeding a value into
`KernelSigSet`. This is not a bug reachable from 100% safe Rust, and not an independently
exploitable path on its own in the current codebase - it is a defense-in-depth gap: *if* some
other `unsafe` call site (existing or added later) ever violated its own stated precondition and
produced an out-of-range `Signal`, passing it to `KernelSigSet::insert`/`remove`/`contains` would
convert that bug into an opaque "index out of bounds" panic instead of either a clear diagnostic or
(preferably) being caught closer to its actual source.

**Fully reproducible**: **Yes**. The repro calls the real public API
(`rustix::io_uring::KernelSigSet::insert`, re-exported from the private `kernel_sigset` module)
directly, with a `Signal` produced via the crate's own real, public
`Signal::from_raw_unchecked` - the exact same function any real (mis)user would have to go through
to produce an out-of-range `Signal` in the first place. There is no second participant anywhere in
this finding's trigger path (no peer, no syscall, no network/file input) to fake - see
`CONFIRMATION.md` section 11's own clarification that a single-process repro is complete whenever
the input is a single artifact with no "other side" being impersonated; here the "artifact" is the
out-of-range `Signal` value itself, produced by the real unsafe constructor, not by hand-forging
internal struct state.

## Class

crash (a documented-safety-contract violation elsewhere would manifest here as an opaque,
non-descriptive Rust array-indexing panic rather than UB - confirmed non-UB because `self.0.sig`
is a plain fixed-size array/slice index, which the Rust compiler bounds-checks unconditionally in
every build profile, not just under `debug-assertions`)

## Root cause

`insert`/`remove`/`contains` (`src/kernel_sigset.rs:61-87`) all compute `raw = (sig.as_raw()
.wrapping_sub(1)) as usize` and then index `self.0.sig[raw / sigs_per_elt]` directly, trusting
that `sig.as_raw()` is always within `1..=_NSIG` (1..=64 on every arch this container targets,
1..=128 on MIPS) - the same assumption `Signal::from_raw_unchecked`'s own `# Safety` doc comment
states as a precondition on its caller, but never actually checked at the point of use inside
`KernelSigSet`. For `sig.as_raw() == 0` this is impossible (`Signal` wraps a `NonZeroI32`), but for
any other out-of-range value - either a large positive raw signal number or a negative one -
`wrapping_sub(1) as usize` produces an index far outside `self.0.sig`'s actual length (1 element
on x86_64), and plain array indexing panics with Rust's generic, non-descriptive "index out of
bounds" message instead of a clear assertion naming the actual problem (the out-of-range signal
number).

## Introduced by

28036de6 "Make `io_uring_enter` more ergonomic. (#1345)" (2025-02-22) - the commit that introduced
`KernelSigSet` itself, with this exact unchecked index math in `insert`/`remove`/`contains` from
day one (`git log -S"wrapping_sub(1)" -- src/kernel_sigset.rs` returns only this one commit; `git
show 28036de6 -- src/kernel_sigset.rs` shows the three functions already present with identical
code to what exists at `HEAD`). Every later commit touching the file (per `git log --follow`) only
moved/renamed code around it, none changed this logic.

## The fix

Added an explicit `assert!(raw < self.0.sig.len() * sigs_per_elt, "signal number out of range:
{}", sig.as_raw())` immediately before the indexing expression in all three of `insert`/`remove`/
`contains`, matching the existing precedent already set elsewhere in this same workspace
(`___BUGHUNT___/041.cpu_set_bounds_panic_undocumented`, where the `linux_raw` backend's
`CPU_SET`/`CPU_CLR`/`CPU_ISSET` got the identical treatment: a clear, explicit assertion in place
of a bare Rust indexing panic). This is deliberately *not* a behavior change for any currently
in-range input, and deliberately keeps the out-of-range case as a panic (not a `Result`) - the
input is already a documented-unsafe-precondition violation by the time it reaches here, so a
panic remains the correct "fail loudly, don't silently corrupt the set" response; the only change
is replacing an opaque, generic message with one that actually names the problem (`signal number
out of range: <raw>` instead of `index out of bounds: the len is 1 but the index is 1562`).

Checked against more than the one originally-tried input, per `CONFIRMATION.md` section 7: both a
too-large positive raw value (100000) and a negative one (-5) - the two distinct ways
`wrapping_sub(1) as usize` can produce an out-of-range index - were verified to produce the new,
clear message after the fix, not just the first one found.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`):
  **before fix**: 411 passed, 1 failed (`ioctl::test_int_setter` - pre-existing, unrelated; fails
  identically with or without this patch because `/dev/net/tun` isn't present in this sandbox, see
  the skill's own `projects/rustix.md` notes), 6 ignored. **After fix**: identical - 411 passed, 1
  failed (same `ioctl::test_int_setter`), 6 ignored. No new failures introduced, no
  previously-failing test newly passes (expected - this patch only affects already-out-of-range
  input, which no existing test exercises).

## What's not done / open questions

- The fix only adds a defense-in-depth check; it does not (and structurally cannot, from inside
  `KernelSigSet`) prevent an `unsafe` caller from violating `Signal::from_raw_unchecked`'s own
  precondition in the first place - that responsibility stays with whatever `unsafe` code
  constructs the `Signal`.
- No current internal call site in this crate actually violates that precondition when the result
  reaches `KernelSigSet` (verified above), so this fix has no observable effect on any
  currently-shipped code path - it only helps if/when a future caller (internal or external,
  since `KernelSigSet`/`Signal::from_raw_unchecked` are both public API) gets this wrong.
- Not swept this session: whether any other `unsafe`-constructed value elsewhere in the crate
  (e.g. `Pid::from_raw_unchecked`, `Flock::from_raw_unchecked`) has a similar sibling consumer
  with the same "trusts the precondition, no defense-in-depth check" shape - out of scope for a
  single candidate, noted as a general pattern worth a future sweep.
