# rlimit_from_libc_unwrap_panic

`src/backend/libc/process/syscalls.rs:319-331` (`rlimit_from_libc`, used by `getrlimit`/`prlimit`)
converts the C `c::rlim_t` read from the kernel into the crate's `u64`-based `Rlimit::current`/
`maximum` via `.try_into().unwrap()`, special-casing only `RLIM_INFINITY` beforehand. Its sibling
`rlimit_to_libc` (same file, opposite direction) has explicit `.map_err(|_e| io::Errno::INVAL)?`
instead. The asymmetry looked suspicious on any libc target where `c::rlim_t` is a *signed* type,
since a negative, non-`RLIM_INFINITY` value returned by the kernel would make `try_into::<u64>()`
fail and panic instead of erroring.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `rlim_t` type definition for every platform in the vendored `libc` crate (0.2.189, both installed toolchains) that isn't excluded by this function's own `#[cfg(not(any(espidf, fuchsia, horizon, redox, vita, wasi)))]` guard | `grep -rn "pub type rlim_t" src/` across `libc-0.2.189`'s whole `src/` tree (every `unix/*/mod.rs`, `fuchsia/mod.rs`, `solid/mod.rs`, `vxworks/mod.rs`) | Every Linux-family type (`gnu` b32/b64, `musl`, `uclibc`, `android`, `l4re`), Apple, NetBSD, Solaris/illumos, Haiku, AIX, Hurd, Cygwin, Redox (excluded anyway) is unsigned (`u64`/`c_ulong`/`c_ulonglong`/`uintptr_t`). Only two are genuinely signed: `unix/bsd/freebsdlike/mod.rs` (`rlim_t = i64`, shared by FreeBSD and DragonFlyBSD, both of which are real rustix CI targets per `Cargo.toml`'s `"x86_64-unknown-dragonfly"` entry and `build.rs`'s `freebsdlike` cfg) and `unix/nuttx/mod.rs` (`rlim_t = i64`, but NuttX is not a rustix target at all - no `nuttx` reference anywhere in this crate's own source). So FreeBSD/DragonFlyBSD is the one real, in-scope platform family where this pattern could matter. |
| Cross-compile check: `cargo check --target x86_64-unknown-freebsd --no-default-features --features std,use-libc` | rustc 1.x, `x86_64-unknown-freebsd` target installed via rustup | Compiles cleanly - confirms `rlimit_from_libc`/`rlimit_to_libc` really are built with `c::rlim_t = i64` for a real, CI-exercised rustix target, not a purely hypothetical cfg combination. |
| External verification against the real kernel source (no FreeBSD VM/hardware available in this environment; fetched FreeBSD's actual `sys/kern/kern_resource.c` directly via `curl` from `cgit.freebsd.org`, not reconstructed from memory) | `kern_proc_setrlimit()` - the single internal function backing `setrlimit(2)`/`sys_setrlimit`/`prlimit(2)`'s write path for the calling/target process's own limits | Lines 682-688 of the fetched file: `/* Preserve historical bugs by treating negative limits as unsigned. */ if (limp->rlim_cur < 0) limp->rlim_cur = RLIM_INFINITY; if (limp->rlim_max < 0) limp->rlim_max = RLIM_INFINITY;` - **any** negative `rlim_cur`/`rlim_max` passed to `setrlimit`/`prlimit` is deliberately normalized to `RLIM_INFINITY` *before* being stored in the process's limit structure, and default/inherited limits are hard-coded non-negative kernel constants. So a genuine `getrlimit`/`prlimit` call on FreeBSD can never read back a negative value that isn't `RLIM_INFINITY` (which `rlimit_from_libc` already special-cases) - the only real-world input path that could trigger the suspected panic is closed off by the kernel itself. |
| Manual boundary-value construction (sanity check of the mechanism only, not of reachability) | `rustc -O` one-off snippet: `let rlim_cur: i64 = -5; let current: u64 = rlim_cur.try_into().unwrap();` | Panics as expected (`TryFromIntError(NegOverflow)`, exit code 101) - confirms the `.unwrap()` pattern *would* panic given such an input, but this is just confirming ordinary `TryInto` behavior, not evidence the kernel ever supplies one. |

## Status

**Not reproducible.** Of the two platforms (within rustix's own supported-target list) where
`c::rlim_t` is signed, only FreeBSD/DragonFlyBSD are real rustix targets, and FreeBSD's own kernel
source demonstrates the precondition needed to trigger the panic (a negative, non-`RLIM_INFINITY`
`rlim_cur`/`rlim_max` surviving into a `getrlimit`/`prlimit` return value) cannot occur through any
genuine kernel code path - `kern_proc_setrlimit` normalizes negative inputs to `RLIM_INFINITY`
before storage, and default limits are non-negative constants. DragonFlyBSD's own kernel source
was not independently checked this session (`gitweb.dragonflybsd.org` returned an anti-bot/Anubis
challenge page instead of the file), but DragonFly forked from FreeBSD in 2003 and this exact
"preserve historical bugs" normalization predates that fork by a wide margin in FreeBSD's history,
so it is very likely shared - this is an inference, not independently confirmed, and is flagged as
such below.

## Root cause

N/A - not a bug reachable via any genuine kernel-produced input on the one real in-scope platform
family (FreeBSD/DragonFlyBSD) where `rlim_t` is signed. The code asymmetry against `rlimit_to_libc`
is real (one direction has explicit error handling, the other doesn't), but the kernel's own
contract on the read side (`getrlimit`/`prlimit`) guarantees the value is always either
`RLIM_INFINITY` (already handled) or a genuine non-negative limit - never an arbitrary negative
`rlim_t` that would make `try_into::<u64>()` fail.

## Introduced by

1df450c8 "Implement `setrlimit` and `prlimit`. (#191)" (2022-01-25) - this commit refactored the
pre-existing `getrlimit`-only conversion (which used `result.rlim_cur.try_into().ok()`, silently
producing `None` on failure, never panicking) into today's `rlimit_from_libc`/`rlimit_to_libc` pair,
introducing the `.unwrap()` in `rlimit_from_libc` alongside the new `rlimit_to_libc`'s explicit
`.map_err(|_| io::Error::INVAL)?`. This is the commit that created the asymmetry the candidate
flagged - not confirmed to ever actually panic in practice (see Status above), so this is recorded
as the origin of the asymmetric *code shape*, not of a reachable bug.

## The fix

No fix applied. Not reproducible as a real bug this session, and the asymmetry - while real and
worth tidying up defensively (mirroring `rlimit_to_libc`'s explicit `io::Errno`-returning pattern
instead of `.unwrap()`, for robustness against a future libc/kernel combination where the invariant
doesn't hold, since `getrlimit`/`prlimit` are safe-ish `pub` API that shouldn't panic on a
corner-case regardless of today's kernel behavior) - is a defensive-coding suggestion, not a patch
for a demonstrated defect. Left for a maintainer to decide whether it's worth hardening anyway.

- Project test suite: not run - no code change was made (see "Not reproducible" above and "the one
  rule that matters most" in `SKILL.md`: a fix is only attempted/verified for a confirmed bug).

## What's not done / open questions

- DragonFlyBSD's own `kern_resource.c` was not independently fetched/verified this session (host
  returned an Anubis anti-bot challenge for `gitweb.dragonflybsd.org`); the "very likely shares the
  same normalization" conclusion above is an inference from shared FreeBSD ancestry, not a direct
  read of DragonFly's current source.
- No FreeBSD/DragonFlyBSD hardware or VM was available in this environment to actually execute
  `getrlimit`/`prlimit`/`setrlimit` natively and observe real kernel behavior; the conclusion rests
  entirely on reading FreeBSD's authoritative kernel source (fetched live from `cgit.freebsd.org`
  this session, not from memory/training data) rather than an executed end-to-end repro. Given the
  finding is explicitly about a kernel invariant (a static fact about what `setrlimit`'s kernel-side
  validation allows to be stored), source-level confirmation is a reasonably strong substitute for
  a live run here - but it is not the same as watching a real syscall round-trip.
- If a future libc/platform combination introduces a *new* signed `rlim_t` (or rustix adds NuttX
  support, where `rlim_t` is also `i64` per the vendored `libc` crate), this same question would
  need re-checking against that platform's own kernel/validation behavior - the "not reproducible"
  conclusion here is specific to FreeBSD/DragonFlyBSD as they exist today, not a blanket clearance
  of the code pattern for every possible future signed-`rlim_t` target.
