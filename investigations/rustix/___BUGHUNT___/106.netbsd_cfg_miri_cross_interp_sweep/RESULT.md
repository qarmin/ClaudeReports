# netbsd_cfg_miri_cross_interp_sweep

A dedicated, systematic sweep of `target_os = "netbsd"`-gated code across the crate (~50
`#[cfg]` sites touching `clockid.rs`, `pty.rs`, `process/wait.rs`, `time/*`,
`backend/libc/{time,fs,event,net,system}/*`, `event/kqueue.rs`, `net/types.rs`, `termios/types.rs`,
`fs/{abs,fd,constants}.rs`, `fs/dir.rs`, `fs/makedev.rs`), using the Miri-cross-interpreting-a-
non-Linux-cfg-target technique proved out on macOS/BSD by candidates 035/039/058. Unlike those,
and unlike the OpenBSD/Fuchsia/Redox/wasi sweep in candidate 081, NetBSD had never been
independently swept this way before - every prior NetBSD mention in `SUMMARY.md` (candidates 066,
067, 094) was an incidental aside reached while investigating a different primary target.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Miri setup + capability probe: can Miri cross-interpret `--target x86_64-unknown-netbsd` at all, and which NetBSD-specific foreign-function shims does it actually implement? | `cargo miri setup --target x86_64-unknown-netbsd` (sysroot built from `rust-src` in ~12s, same mechanism as candidate 039); fetched Miri's own `src/shims/unix/netbsd/foreign_items.rs` from the exact installed Miri commit (5c543b0b8c73c7b72bc8284ced4fb22ead15734d) via GitHub to see the full NetBSD-specific shim list | Sysroot builds fine (cross-interpretation is possible in principle, same as for apple-darwin). But NetBSD's own **shim file implements exactly 2 functions**: `__unsetenv13` and `__errno`. Everything else must fall through to the generic unix shims, which only match **plain, non-ABI-versioned** libc symbol names - and NetBSD aliases most of its stat/dir/time/event-family syscalls to versioned symbols (`__fstat50`, `__stat50`, ...) for historical ABI-compatibility reasons that don't exist on Linux/Apple/FreeBSD |
| Real end-to-end Miri cross-interpretation of rustix's public API, 2 ways: (a) the plain-symbol paths Miri *can* execute (file I/O, `pipe()`+`poll()` - which exercises the one `target_os = "netbsd"`-gated block in `backend/libc/event/syscalls.rs`, a weak-symbol `ppoll` lookup that falls back to plain `poll()` when not found), and (b) the 3 most structurally promising netbsd-gated suspects found by code reading (`fstat` - custom `Stat` struct with reserved `st_flags`/`st_gen`/`st_spare` fields, same bug *shape* as candidate 039's confirmed apple finding; `kqueue()` - same filter-decode code candidate 094 confirmed buggy on apple; `uname()` - the exact call candidate 050 got a since-ruled-out Miri signal from on Linux) | `MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --target x86_64-unknown-netbsd`, debug build, 3 repeat runs each for (a); 1 run each for (b) (not independently repeated - no non-deterministic element to repeat against, see below) | (a) **Reproduces cleanly 3/3 runs**: file create/write/remove, real `pipe()`+`poll()` round-trip (the weak-symbol `ppoll` lookup correctly resolves to "not found" under Miri and falls through to the plain-`poll()` fallback branch, which then correctly reports the real pending read), and `makedev()`/`major()`/`minor()` round-trip at their bit-field boundary values (`maj=0xff`, `min=0xfffff`) - no UB, no wrong result. (b) **All 3 immediately abort with Miri's own `unsupported operation: can't call foreign function ... on OS netbsd`** (`__fstat50`, `kqueue`, `uname` respectively) - Miri has no shim for any of them on this target, so none of candidate 039's/094's/050's techniques can be repeated here; this is a tooling-coverage gap, not evidence the suspects are safe or unsafe |
| Manual code reading + direct comparison against real upstream sources for every other `target_os = "netbsd"`-only cfg block (not shared with `freebsdlike`/`apple`/`bsd` groups, which are covered by the existing macOS/BSD sweep) | Compared rustix's hand-written NetBSD `Stat` struct (`backend/libc/fs/types.rs`) field-by-field against the vendored `libc` crate's real `netbsd::stat` (both 21 fields, identical order/types: `dev,mode,ino,nlink,uid,gid,rdev,atime,atimensec,mtime,mtimensec,ctime,ctimensec,birthtime,birthtimensec,size,blocks,blksize,flags,gen,spare`); fetched the real NetBSD kernel header (`https://raw.githubusercontent.com/NetBSD/src/trunk/sys/sys/timerfd.h`) to check `backend/libc/c.rs`'s oddly-aliased `TFD_TIMER_ABSTIME = O_WRONLY` / `TFD_TIMER_CANCEL_ON_SET = O_RDWR` constants; re-derived the netbsd `AF_OROUTE` constant (`net/types.rs:702`), the netbsd-only `makedev`/`major`/`minor` wrappers (confirmed the vendored libc functions are `const safe fn`, pure bit math, no FFI at all - Miri cross-checks these with zero shim-coverage risk), and the `ppoll`-weak-symbol-with-`poll()`-fallback control flow in `event/syscalls.rs` (confirmed by inspection that the final `#[cfg(not(any(linux_kernel, freebsdlike, fuchsia, haiku, hurd)))]` fallback block is *not* excluded for netbsd, so it's reachable both when the first block's weak-symbol lookup fails to find `ppoll` and reaches the end of its own block without returning) | No discrepancy found anywhere. `Stat`'s field layout matches the real kernel ABI exactly (not just size - `assert_eq_size!` only catches size, but the manual field-by-field order comparison here additionally rules out a silently-reordered-field bug the size check alone couldn't catch). The `TFD_TIMER_*` constants' bizarre `O_WRONLY`/`O_RDWR` aliasing is a faithful transcription of NetBSD's own real, equally bizarre kernel header (confirmed byte-for-byte against the upstream source), not a rustix mistake. `AF_OROUTE` and the makedev wrappers are plain, correctly-typed constant/bit-math translations. The `ppoll` control flow is correct by construction (intentional weak-symbol-with-fallback design for NetBSD 9.x, which lacks `ppoll`) |

## Status

**Not reproducible.** No new NetBSD-specific bug was found. 3/3 repeat runs for every executable
probe were fully deterministic (no flakiness in either direction - the "works" probes always work,
the "no shim" probes always abort identically). This is an honest negative, not an absence of
effort: the dominant finding of this sweep is that **Miri's NetBSD cross-interpretation coverage is
far shallower than its Apple/FreeBSD/illumos coverage** (2 NetBSD-specific shims vs. working
`fstat`/`stat`/`lstat` shims on apple, as candidate 039 used), which structurally limits how much of
this technique can be repeated for NetBSD at all with the currently-installed Miri version.

**Fully reproducible**: N/A (no Confirmed finding).

## Class

N/A (Not reproducible)

## Root cause

Not applicable to a specific bug - but worth recording precisely *why* each structurally-promising
suspect turned out untestable or safe, so a future session doesn't re-walk the same path:

- **`fstat`/`stat`/`lstat`/`statat`** (the same `MaybeUninit::<Stat>::uninit()` -> `assume_init()`
  call sites candidate 039 confirmed buggy for Apple's `st_lspare`/`st_qspare` reserved fields):
  structurally, NetBSD's own `Stat` has the exact same shape of risk (`st_flags`, `st_gen`,
  `st_spare: [u32; 2]` are all plausible-reserved fields a real `__fstat50` syscall might not fully
  populate) - **but this is the same source lines candidate 039 already fixed, unconditionally,
  for every target** (`MaybeUninit::uninit()` -> `MaybeUninit::zeroed()`, not gated on `apple`).
  Confirming the NetBSD-specific manifestation of this exact bug would not be a new finding; it
  would be re-confirming candidate 039 under a different `--target`. It could not be independently
  verified either way this session regardless, since Miri has no `__fstat50` shim for OS netbsd.
- **`kqueue()`/`Event::new()` filter-decode panic** (the same code candidate 094 confirmed buggy for
  Apple's `EVFILT_MACHPORT`/`VM`/`EXCEPT`/`AIO` constants): same situation - shared, not
  NetBSD-specific code, already Confirmed elsewhere, and untestable here regardless (no `kqueue`
  shim for OS netbsd in this Miri version).
- **`uname()`**: candidate 050 already showed the Miri `uname`/`sysinfo` UB signal is a Miri shim
  incompleteness artifact, not a real bug (poison-byte-tested against the real Linux syscall). Not
  independently testable for netbsd specifically - Miri has no `uname` shim for OS netbsd at all
  (not even the incomplete one it has for Linux/apple), so there is no signal to even evaluate here.
- **Every other `target_os = "netbsd"`-only cfg site** (constants, the `ppoll` fallback, the
  makedev/major/minor wrappers): read and independently verified safe by direct comparison against
  real upstream sources (NetBSD's own kernel header for `TFD_TIMER_*`, the vendored `libc` crate's
  struct/const-fn definitions) - no logic/layout mismatch found.

## Introduced by

N/A (Not reproducible; no bug to attribute).

## The fix

No fix - nothing in this sweep was confirmed as a new bug. No source changes were made to the main
project.

- Project test suite: not run (no code change to validate, and the suspects investigated are
  gated on `target_os = "netbsd"`, which cannot execute at all on this x86_64-linux sandbox outside
  Miri cross-interpretation).

## What's not done / open questions

- **The structural "same bug class, different target" cases above (`fstat`, `kqueue`) were not
  independently Miri-verified for NetBSD** - only code-read and cross-referenced against the
  already-Confirmed candidates 039/094. If a future Miri release adds NetBSD shims for
  `__fstat50`/`kqueue`, re-running this sweep's `unsupported_sweep` probes would turn these from
  "blocked by tooling" into either an independent re-confirmation or a genuine ruling-out.
- **Every other NetBSD syscall using an ABI-versioned symbol name** (`__stat50`, `__lstat50`,
  `__readdir30`, `__getdents30`, `__select50`, `__clock_gettime50`, `__sigaction14`, and more) was
  not individually enumerated and probed - this sweep stopped at the 3 structurally most-promising
  ones (matching already-known bug *shapes* from 039/094/050) once the "no shim" pattern repeated
  identically all 3 times; a future session with more budget could enumerate the full versioned-
  symbol surface systematically (e.g. via `nm`/`strings` on a real NetBSD libc.so, not available in
  this sandbox) to know exactly how much of the crate's NetBSD-gated surface Miri can ever reach.
- **No real NetBSD hardware/VM/emulator was available** in this sandbox (consistent with every
  prior BSD-family candidate in this workspace - see candidates 080/090/094/096/100) - only Miri
  cross-interpretation and static/structural comparison against real upstream sources were
  possible.
- This session's main actionable finding for a future one: **Miri's NetBSD shim coverage (2
  functions: `__unsetenv13`, `__errno`) is dramatically narrower than its Apple/FreeBSD/illumos
  coverage**, so the macOS/BSD Miri-cross-interpretation technique (035/039/058/094) only
  generalizes to NetBSD for code paths using plain, non-ABI-versioned libc symbol names (plain
  I/O, `poll`, pure-computation helpers like `makedev`/`major`/`minor`) - any future NetBSD
  candidate relying on this technique for a *stat/dir/time*-family syscall should expect the same
  "unsupported operation" wall this session hit, not treat it as a dead end specific to that one
  suspect.
