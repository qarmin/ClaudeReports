# futex_val2_int_to_ptr_cast_miri_ub

The `libc`-backend futex `timeout`-argument-smuggling trick in `futex_val2` builds a fake
`*const Timespec` via a bare integer-to-pointer `as` cast, which Miri's strict-provenance model
rejects as UB. Sourced from GitHub issue #1627.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/backend/libc/thread/syscalls.rs::futex_val2`, the real `futex(2)` ABI's `val2`-in-`timeout`-slot reuse for `FUTEX_REQUEUE`/`FUTEX_CMP_REQUEUE`/`FUTEX_WAKE_OP` | Confirmed the cast is a real ABI-mandated need (not a mistake in intent), but built in a Miri-hostile way |
| Man-page cross-check | live `man7.org/linux/man-pages/man2/futex.2.html` vs. the Debian bookworm/trixie shipped `futex(2)`/`FUTEX_REQUEUE(2const)` man pages | Confirmed the reporter's "the exact quoted sentence is gone from the linked page" claim, but the *technical fact itself* (kernel casts timeout to unsigned long then uint32_t for this argument slot) still holds and is documented elsewhere - the link went stale, the ABI behavior didn't change |
| `cargo +nightly miri run` under `-Zmiri-strict-provenance` | `RUSTFLAGS="--cfg rustix_no_linux_raw"` (forces libc backend; linux_raw's inline-asm syscalls aren't Miri-executable at all), `--features "std,thread,use-libc"` | Hard error pinpointing exactly the suspected line |
| `cargo +nightly miri run`, default (permissive) Miri | same config | Warning at the same line, then a separate pre-existing/unrelated Miri tooling limitation on the real `libc::syscall()` variadic-argument type-strictness (out of scope, present identically before/after) |
| `cargo test --workspace --features "std,all-apis,use-libc"` before/after fix | | No regressions; one pre-existing unrelated sandbox failure (`ioctl::test_int_setter`, needs `/dev/net/tun`) present both before and after |
| Real-syscall smoke test on real Linux, fix applied | direct calls to `futex::requeue`/`futex::wake_op` with nonzero `val2` | Succeeds normally - the non-Miri code path is untouched by the fix |

## Status

**Confirmed**. Deterministic and reproduced on every run under both `-Zmiri-strict-provenance`
(hard error) and default Miri (permissive-provenance warning) - this is a compile-time-shape bug,
not flaky in any way.

**Fully reproducible**: Yes. The repro calls the real public API (`rustix::thread::futex::requeue`)
which internally reaches the exact suspect line through its real call path - no internal
scaffolding bypassed.

## Class

`UB` - a strict-provenance violation as flagged by Miri. Not exploitable as a real memory-safety
bug on any real hardware today (the fabricated pointer is never dereferenced, only used to carry
an integer payload through an FFI argument slot the kernel reinterprets as an integer) - but it is
undefined behavior under Rust's abstract machine and blocks running this code path under Miri at
all, which matters for any downstream crate that wants to verify itself with Miri while depending
on rustix's `libc` backend.

## Root cause

`src/backend/libc/thread/syscalls.rs:478` (current HEAD, `futex_val2`):

```rust
let timeout = val2 as usize as *const Timespec;
```

The real Linux `futex(2)` syscall reuses its 4th argument slot: for ops like `FUTEX_REQUEUE`,
`FUTEX_CMP_REQUEUE`, and `FUTEX_WAKE_OP`, the kernel does not treat it as a `struct timespec *` at
all - it reinterprets the raw register/argument bits as a 32-bit integer, `val2`. Since the FFI-level
`syscall!` macro in `src/weak.rs` passes `*const T` arguments through unchanged to preserve
provenance for genuine pointers, `futex_val2` has to fabricate a `*const Timespec` whose *bit
pattern* equals `val2` purely so it can occupy that argument slot - this pointer is never
dereferenced. That's a legitimate, ABI-mandated need, but the way it's built (`as usize as *const
Timespec`) is exactly the bare integer-to-pointer cast that Rust's strict-provenance model (and
Miri's `-Zmiri-strict-provenance` checker) forbids, because it fabricates a pointer with no real
provenance.

The in-code comment's cited justification (a specific sentence quoted from
`man7.org/linux/man-pages/man2/futex.2.html`) is genuinely no longer present on that live page -
the man-pages project split `futex(2)` into `futex(2)` plus per-operation `FUTEX_*(2const)` pages,
and the "kernel casts timeout to unsigned long, then uint32_t" wording still exists verbatim in the
currently-shipping Debian bookworm/trixie mirror of the man page, describing exactly
`FUTEX_CMP_REQUEUE`/`FUTEX_WAKE_OP`/`FUTEX_REQUEUE`'s reuse of `val2`. So the reporter is right that
the *specific link target* went stale, but the underlying ABI behavior itself hasn't changed - this
is a real, still-current kernel ABI quirk, correctly worked around in principle, just built with a
Miri-hostile cast.

PR #1632 ("Change the futex `timeout` argument to `u32`") is about a different argument (the real
`timeout` pointer type in the public API) and does not touch or fix this cast - confirmed by
inspecting current `futex_val2`'s `val2: u32` parameter and this specific line; unrelated/
non-overlapping with this candidate.

## Introduced by

b0d3681c "Futex Rework (#1098)" (Daniel Schemmel, 2024-08-23) introduced the cast; the explanatory
comment + man-page link was added immediately after by e2014c52d (Dan Gohman, 2024-08-27).

## The fix

See `patch.diff`: gate the cast on `#[cfg(miri)]`, using `core::ptr::without_provenance::<Timespec>(val2 as usize)`
(a strict-provenance-safe way to fabricate a non-dereferenced pointer from an integer) only under
Miri, keeping the original `as usize as *const Timespec` cast for real (non-Miri) builds. This is
not done unconditionally because `core::ptr::without_provenance` stabilized in Rust 1.84, newer
than this crate's stated MSRV (`rust-version = "1.65"` in `Cargo.toml`) - Miri always runs on a
recent nightly toolchain, well past 1.84, so gating it behind `#[cfg(miri)]` is safe without
raising the crate's real MSRV.

Verified against more than the exact line by construction: the fix doesn't special-case a specific
`val2` value, it changes how *any* `val2` is turned into the smuggled pointer, and the real
(non-Miri) code path is byte-for-byte unchanged, confirmed via a real-syscall smoke test with a
nonzero `val2` on real Linux hardware succeeding normally after the patch.

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc"` - 151 passed / 1
  failed before, identical 151 passed / 1 failed after (the one failure, `ioctl::test_int_setter`,
  is a pre-existing sandbox limitation needing `/dev/net/tun`, unrelated to this change, confirmed
  present identically with and without the patch). The 5 real futex tests
  (`test_lock_unlock_pi`, `test_wait_wake`, `test_wait_timeout`, `test_waitv_wake`,
  `test_waitv_timeout`) all pass with the fix.

## What's not done / open questions

- A separate, pre-existing Miri tooling limitation remains after this fix: Miri's variadic-argument
  type-strictness on `libc::syscall()` itself flags `SYS_futex`'s `uaddr` argument
  (`*const AtomicU32` vs. libc's declared `*mut u32` prototype) as "incorrect c-variadic argument
  type." This is unrelated to `timeout`/`val2` and was not investigated or fixed as part of this
  candidate - it means the real futex syscall path still can't be run to completion under Miri even
  after this fix, only the specific strict-provenance violation this candidate targeted is gone.
  Worth a dedicated future candidate if fully Miri-clean futex execution is desired.
- Only `src/backend/libc/thread/syscalls.rs` and the immediate futex code were swept for
  integer-to-pointer/provenance casts this session - the broader crate (other backends, other
  syscall wrappers stuffing auxiliary values into pointer-typed FFI slots) was not checked.
