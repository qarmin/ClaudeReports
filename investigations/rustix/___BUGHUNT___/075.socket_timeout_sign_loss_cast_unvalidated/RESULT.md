# socket_timeout_sign_loss_cast_unvalidated

`src/backend/libc/net/sockopt.rs:298-301`, inside `socket_timeout()`:

```rust
Ok(Some(
    Duration::from_secs(timeout.tv_sec as u64)
        + Duration::from_micros(timeout.tv_usec as u64),
))
```

casts the kernel-returned signed `timeval.tv_sec`/`tv_usec` fields straight to `u64` with no range
check, unlike the setter path (`set_socket_timeout`), which since commit 0cea1880 routes through
`Timespec::to_sec_usec()` with explicit clamping. The same unchecked-cast shape also exists in the
`linux_raw` backend's sibling, `duration_from_linux_sock_timeval`/`duration_from_linux_old_timeval`
in `src/backend/linux_raw/net/sockopt.rs:262-277`. The suspicion: if the kernel (or an emulation
layer/libc shim) ever returned a negative `tv_sec`, this would silently produce a bogus
multi-century `Duration` instead of an error.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction, real kernel, raw C: a standalone C program calls the genuine `setsockopt(SO_RCVTIMEO)`/`getsockopt(SO_RCVTIMEO)` syscalls directly (no rustix involved) with hand-picked boundary `timeval` values to see what the real kernel actually stores/returns | Ubuntu 26.04 LTS, Linux 7.0.0-34-generic (x86_64), glibc; `tv_sec` tried: `-5`, `-1`, `i64::MAX` (`9223372036854775807`), a large-but-plausible value (`100000000000`, ~3170 years), `0`; `tv_usec` tried: `-100`, `100`, `0`, `999999` | `tv_sec = -5`/`-1`: `setsockopt` returns success (`ret=0`), but `getsockopt` reads back `tv_sec=0, tv_usec=0` - the kernel's own `sock_set_timeout()` (net/core/sock.c) detects `tv.tv_sec < 0`, silently clamps the *stored* timeout to 0 (block forever) instead of persisting the negative value, and logs a rate-limited kernel warning. `tv_sec = i64::MAX`: also silently clamped to `0,0` (exceeds `MAX_SCHEDULE_TIMEOUT / HZ`). `tv_sec = 100000000000`: round-trips verbatim (within the kernel's accepted range). `tv_usec < 0` alone: outright rejected by `setsockopt` with `EDOM` (errno 33), nothing stored. In every case, `getsockopt` never returned a negative `tv_sec`. |
| Real end-to-end repro through rustix's actual public API, both backends: a standalone Rust program (local path-dependency on this worktree's `rustix`) creates a real `TcpListener`, uses a raw `libc::setsockopt()` call (the real kernel, not rustix's own validated setter) to attempt to plant the same boundary `tv_sec` values directly into the kernel's socket state, then calls rustix's real public `rustix::net::sockopt::socket_timeout()` (the actual affected entry point) to read it back | `cargo run`, default (`linux_raw`) backend and, separately, rebuilt with the `use-libc` feature to exercise `src/backend/libc/net/sockopt.rs` specifically (the file named in this candidate); features `net,time` (`time` needed only to route around the already-tracked, unrelated `timespec` module-gating bug from candidate 024) | Identical results on both backends: `tv_sec=-5`/`-1`/`i64::MAX` -> `socket_timeout()` returns `Ok(None)` (correct - because the kernel itself never actually stored a negative/overflowing value for rustix's getter to read back; `getsockopt` always yielded `0,0` for all three). `tv_sec=100000000000` -> `Ok(Some(100000000000s))`, a correct, large-but-not-wraparound `Duration`. `tv_sec=0` -> `Ok(None)` as expected. No wrong value, panic, or wraparound was observed in any case, on either backend. |
| Checked whether a debug-assertions build would add any protection here | Code reading only, no build needed | N/A as a distinct tool: `as` casts in Rust (including `i64 as u64`) never panic in any build profile, debug or release - there is no debug-assertion equivalent for a sign-loss/narrowing `as` cast the way there is for arithmetic overflow, so this bug class (if it were reachable) would not be caught by a debug-assertions build at all; noted for the record, not counted as a 3rd empirical approach since it produced no new data, only a reason the "obvious" tool doesn't apply here. |

## Status

**Not reproducible.** On this platform (Linux x86_64, kernel 7.0.0-34-generic, glibc), the real
kernel's own `sock_set_timeout()` unconditionally clamps a negative or out-of-range `tv_sec` to a
stored timeout of `0` (block forever) at `setsockopt()` time, and `getsockopt(SO_RCVTIMEO)`/
`SO_SNDTIMEO` only ever returns a `timeval` reconstructed from that non-negative stored value. There
is no code path through the genuine kernel SO_RCVTIMEO/SO_SNDTIMEO get/set mechanism, on this
platform, that causes `getsockopt` to hand back a negative `tv_sec` for `socket_timeout()`'s cast to
observe - tested directly with boundary values (`-5`, `-1`, `i64::MAX`) via both a raw C program and
rustix's real public API on both backends (`linux_raw` and `libc`), all consistent.

**Fully reproducible**: N/A - no reproduction exists to classify; this is a clean negative, not a
flaky or layout-dependent one. The negative result itself was produced via the real kernel path end
to end (no hand-authored/faked input), satisfying `CONFIRMATION.md` section 11's bar for what counts
as a genuine attempt.

## Class

N/A - no bug found.

## Root cause

N/A - no bug found. The unchecked `as u64` cast on `tv_sec`/`tv_usec` is real and is genuinely
asymmetric with the now-fixed setter path (so it remains a latent defense-in-depth gap: the code
itself still assumes "the kernel never returns a negative value" without stating or checking that
assumption), but this session could not construct any input - via the real Linux kernel, on this
platform - that causes the kernel to actually hand back a negative `tv_sec` through
`SO_RCVTIMEO`/`SO_SNDTIMEO`. The `tv_sec < 0` guard this candidate's description speculated might be
missing turns out to already be enforced one layer down, inside the kernel's own `sock_set_timeout`,
on the *set* side - so the *get* side never has a negative value to mishandle in the first place, on
this platform/kernel version.

## Introduced by

N/A - not applicable, no bug confirmed.

## The fix

N/A - no bug found, nothing to fix. (No fix was attempted or applied to the working tree; nothing
to revert.)

## What's not done / open questions

- Only tested on Linux x86_64, this specific kernel version (7.0.0-34-generic) and glibc. The
  candidate's own stated threat model - "a libc shim, emulation layer, or 32-bit time_t quirk" -
  specifically calls out non-Linux or 32-bit-`time_t` platforms (e.g. 32-bit targets near the 2038
  rollover, BSD/Apple kernels reached through the same `src/backend/libc/net/sockopt.rs` code path,
  WASI, or a non-Linux libc with different `SO_RCVTIMEO`/`SO_SNDTIMEO` validation behavior than
  Linux's `sock_set_timeout`) - none of those were reachable to test in this environment. The code's
  own assumption ("the kernel always returns a non-negative `tv_sec`/`tv_usec`") is still unverified
  for those platforms, just empirically confirmed true for the one tested here.
- The same unchecked-cast *shape* also exists in `src/backend/linux_raw/net/sockopt.rs` (noted
  above) - not a new candidate, since it was exercised by the same real end-to-end repro used here
  (the default backend) with the same negative result, but worth noting as the sibling location if a
  future session on a different platform ever does find a reachable negative value.
- No standalone `repro/` folder or `patch.diff` was created for this candidate since the result is
  `Not reproducible` (both are only mandatory for `Confirmed` findings per `SKILL.md`'s candidate
  loop). The ad hoc C program and Rust scratch crate used for the sweep were not preserved beyond
  this session (`/tmp/test_negtv2.c`, `/tmp/bughunt075_check/`), consistent with candidate
  017's precedent for the same `Not reproducible` + "ad hoc scratch program, not preserved" shape.
