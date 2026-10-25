# rt_sigtimedwait_siginfo_union_assume_init

`src/backend/linux_raw/runtime/syscalls.rs`'s `kernel_sigwaitinfo`/`kernel_sigtimedwait`
(lines ~232-335) issue the raw `__NR_rt_sigtimedwait`/`__NR_rt_sigtimedwait_time64` syscalls into a
`MaybeUninit::<Siginfo>::uninit()` and then unconditionally call `.assume_init()` on the full
128-byte `siginfo_t`, whose `_sifields` is a C union (`_kill`/`_timer`/`_rt`/`_sigchld`/
`_sigfault`/`_sigpoll`/`_sigsys`) plus a top-level `_si_pad: [c_int; 32]` padding arm - the kernel
is only obligated to fill in the one union variant relevant to the delivered signal's `si_code`,
so the suspicion (per candidate 039's confirmed, structurally similar bug in the `libc` backend's
`fstat`/`stat`/`lstat`) was that `assume_init()` certifies genuinely uninitialized stack/leftover
memory as a valid `Siginfo` for every byte outside that one variant.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end repro: call rustix's actual public (if name-mangled/`doc(hidden)`) entry point `rustix::runtime_448b8ad740e2a26f::kernel_sigtimedwait` after explicitly poisoning nearby stack memory (`0xAA` fill in a sibling stack frame) just before the call, triggered via a real realtime signal queued to the process with `sigqueue()` (`SI_QUEUE`/`_rt` union arm: `_pid`/`_uid`/`_sigval`) | Debug build (5 runs) + release build (5 runs), real `x86_64-unknown-linux-gnu` host, no emulation | **0/10 runs** showed any `0xAA` poison byte anywhere in the materialized 128-byte `Siginfo` (bytes 32-127, well beyond the `_rt` variant's own content, were `0x00` in every run) |
| Deterministic direct-syscall test, independent of stack-reuse luck: heap-allocate (`Vec<u8>`) the 128-byte `siginfo_t` buffer, explicitly fill every byte with `0xAA`, then issue the real `rt_sigtimedwait` syscall directly via inline `asm!` (same syscall number/argument shape as rustix's own wrapper) against that buffer, triggered by the same real `sigqueue()`-delivered realtime signal | Release build, 5 runs | **0/5 runs** retained any `0xAA` byte anywhere in the 128-byte buffer - the real kernel syscall overwrote the entire buffer, not just the `_rt` variant's ~20 bytes |
| `git log -S "kernel_sigtimedwait"` | whole history | Only 2 hits (`ddca5cc1`, `a92c89fe`), both plumbing/compat refactors of the existing call shape, not the original introduction of the `assume_init()` pattern (predates these, not separately tracked down since no bug was confirmed to need it) |

## Status

**Not reproducible**. On this environment's kernel (`7.0.0-34-generic`, real hardware, no
container/VM emulation beyond the usual sandbox), the real `rt_sigtimedwait` syscall unconditionally
overwrites **every** byte of the 128-byte `siginfo_t` destination - including the ~96 bytes that lie
entirely outside the `_rt` union variant actually relevant to the `SI_QUEUE`-coded signal used in
both tests - leaving zero poison bytes across 15 total runs (10 via the real rustix API end-to-end,
5 via a fully deterministic direct-syscall test with an explicitly pre-poisoned heap buffer). This
matches known Linux kernel history: `clear_siginfo()` (kernel commit series around 2018, part of
Eric W. Biederman's siginfo-hardening audit) ensures every in-kernel `struct kernel_siginfo` used
for signal delivery is zeroed before any union variant is populated, and `copy_siginfo_to_user()`
copies/clears the complete user-visible `siginfo_t`, not just the relevant variant's bytes - the
exact kernel-side fix for the uninitialized-stack-disclosure bug class this candidate suspected.

## Root cause

Not a bug in this codebase as written, given current kernel behavior: `assume_init()`'s safety
contract only requires every byte to have *some* defined value (not that the value is semantically
meaningful for the union variant in use) - reading an inactive union variant's field is a
documented POSIX `siginfo_t` API-misuse concern (callers must consult `si_code`/`si_signo` first),
not a memory-safety violation, *as long as the kernel actually wrote a defined byte everywhere*,
which it does on this (and, per the `clear_siginfo()` history, any currently-supported) Linux
kernel. This differs from candidate 039's confirmed `libc`-backend `fstat` bug in an important way:
that bug was confirmed by directly observing Miri's own apple-target libc shim leave specific
reserved fields (`st_lspare`/`st_qspare`) uninitialized - a real, directly-observed gap. Here, the
real kernel syscall was directly tested twice, independently, and both showed full initialization;
the theoretical union/padding concern this candidate raised does not have a kernel-side gap to
expose it in practice today. There remains a latent defensive-programming concern - the code's
soundness relies on an implicit "the kernel always fully clears `siginfo_t`" assumption that isn't
locally documented or asserted anywhere in this crate (unlike `clear_siginfo()`, there is no public
POSIX/Linux ABI guarantee cited in rustix's own comments) - but this is not a reproducible bug on
any kernel actually available to test against this session.

## Introduced by

N/A - no bug confirmed, so no introducing commit was tracked down (the 2-hit `git log -S` search
above only found later refactors of the existing shape, not worth a deeper bisect for a
non-reproducing suspect per `SKILL.md`'s guidance not to spend the investigation budget on this for
a negative result).

## The fix

N/A - no bug found, nothing to fix. The main project's working tree was never modified; nothing
needed reverting.

- Project test suite: not affected (no change made). Baseline re-confirmed this session in the
  isolated worktree: `cargo test --workspace` (default features) -> 22 passed / 0 failed / 1 ignored
  + 7 doc-tests, matching `projects/rustix.md`'s recorded baseline.

## What's not done / open questions

- Only tested on this session's one real kernel (`7.0.0-34-generic`, x86_64). Older kernels that
  predate the `clear_siginfo()` hardening (pre-~4.19-ish; exact version not independently verified
  this session, only recalled from kernel history) could plausibly still exhibit the suspected
  uninitialized-copy gap - not independently tested here (would need an old-kernel VM/qemu image,
  out of scope for this session's tooling).
- Only the `_rt` (`SI_QUEUE`, via `sigqueue()`) union variant was exercised. Other variants
  (`_sigfault` in particular, which the original candidate description called out by name) were not
  independently triggered (would require delivering a real hardware fault signal through
  `sigwaitinfo`, which is unusual/unsafe to construct deliberately) - given the kernel zeroes the
  *entire* 128-byte structure regardless of which variant applies (confirmed by the direct-syscall
  test observing all 128 bytes, not just the `_rt` region, turn to zero), this is not expected to
  differ by variant, but it was not independently confirmed for `_sigfault` specifically.
- Miri is inapplicable to this code path (consistent with `projects/rustix.md`'s existing note):
  `linux_raw`'s syscalls go through inline `asm!`, which Miri cannot interpret at all, regardless
  of `--target` (unlike candidate 039's `libc`-backend library-call case, there is no Miri libc
  shim standing in for a raw `asm!` `syscall` instruction) - not attempted as a 3rd tool for this
  reason, per `HEURISTICS.md`'s own tool-selection guidance.
- The latent "no local documentation of the kernel-side `clear_siginfo()` guarantee this code
  implicitly relies on" observation above might be worth a small code-comment-only follow-up
  (explaining why `assume_init()` here is sound) but isn't a bug to fix - noted for
  `projects/rustix.md`, not filed as its own candidate.
