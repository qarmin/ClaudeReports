# prctl_systemic_int_to_ptr_cast_sweep

`src/thread/prctl.rs` and `src/process/prctl.rs` contain dozens of call sites of the shape
`value as usize as *mut _` (and the bare already-`usize` form `value as *mut _`), fabricating a
`*mut c_void` whose bit pattern carries an integer payload (a sub-operation code, a capability/
feature number, a pid, a length, a bitflags value) through `prctl(2)`'s `arg2`/`arg3` FFI slots -
never dereferenced by the kernel for any of these ops. Same shape as already-confirmed candidate
004 (`futex_val2_int_to_ptr_cast_miri_ub`, a strict-provenance violation under
`cargo +nightly miri run -Zmiri-strict-provenance`), which only checked
`src/backend/libc/thread/syscalls.rs::futex_val2`. These prctl.rs sites are a distinct, much larger
set of call sites, reached through fully safe public rustix APIs (`rustix::process::
set_dumpable_behavior`, `rustix::thread::remove_capability_from_bounding_set`,
`rustix::thread::configure_capability_in_ambient_set`, etc.) through the libc backend's real
`libc::prctl`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | Grepped both files for `as \*mut _` / `as \*const _`; manually classified each of the 71 matching lines as either a genuine integer-to-pointer fabrication (sub-operation codes, capability numbers, pids, lengths, bitflags, bools-as-usize) or a real pointer cast (`.as_ptr()`, `NonNull::as_ptr()`, `.cast()`) that needed no fix | Found 61 genuine integer-fabrication call sites across both files (process/prctl.rs: 30, thread/prctl.rs: 31), confirming the candidate's "dozens of call sites" description |
| `cargo +nightly miri run --bin <site>` under `-Zmiri-strict-provenance` | `RUSTFLAGS="--cfg rustix_no_linux_raw"` (forces libc backend; linux_raw's inline-asm syscalls aren't Miri-executable at all), `--features "std,thread,process,use-libc"`, 3 distinct real public-API call sites exercised in separate runs (Miri aborts the whole process on its first unsupported-operation error, so each site needs its own run): `process::set_dumpable_behavior` (process/prctl.rs:141), `thread::remove_capability_from_bounding_set` (thread/prctl.rs:497), `thread::configure_capability_in_ambient_set` (thread/prctl.rs:757, the bare-`usize`-without-intermediate-`as usize` textual shape) | Hard error at each site's own line, identical message to candidate 004's finding |
| `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast` before/after fix | | 411 passed / 1 failed, identical before and after (the 1 failure, `ioctl::test_int_setter`, needs `/dev/net/tun`, pre-existing and unrelated) |
| Real-syscall smoke test on real Linux, fix applied, both backends (default `linux_raw` and forced `libc` via `RUSTFLAGS="--cfg rustix_no_linux_raw"`) | `set_dumpable_behavior`, `capability_is_in_bounding_set`, `capability_is_in_ambient_set`, `current_timer_slack`/`set_current_timer_slack`, `core_scheduling_cookie` | Identical observed output both backends, both before and after - the non-Miri code path is untouched by the fix |

Per the soft cap of 3 escalating approaches: this reached a clear, deterministic signal on the
first tool (Miri), as expected going in (the task was explicitly to confirm the same already-known
bug class at more sites, not to discover a new class) - the remaining two approaches are the
verification steps (real tests, real smoke test), not further bug-class digging.

## Status

**Confirmed**. Deterministic and reproduced on every run (3/3 distinct call sites tried, both
textual shapes of the cast) under `-Zmiri-strict-provenance` - this is a compile-time-shape bug,
not flaky in any way, identical in kind to candidate 004.

**Fully reproducible**: Yes. Each of the 3 repro binaries calls a real public API
(`rustix::process::set_dumpable_behavior`, `rustix::thread::remove_capability_from_bounding_set`,
`rustix::thread::configure_capability_in_ambient_set`) which internally reaches the exact suspect
line through its real call path - no internal scaffolding bypassed.

## Class

`UB` - a strict-provenance violation as flagged by Miri, identical in nature to candidate 004. Not
exploitable as a real memory-safety bug on any real hardware today (the fabricated pointer is never
dereferenced, only used to carry an integer payload through an FFI argument slot the kernel
reinterprets as an integer or ignores) - but it is undefined behavior under Rust's abstract machine
and blocks running any of these code paths under Miri at all.

## Root cause

Same mechanism as candidate 004, just at many more call sites. `prctl(2)`'s C prototype takes
`unsigned long` arguments for `arg2`/`arg3`/etc., but rustix's libc-backend FFI binding
(`src/backend/libc/prctl/syscalls.rs::prctl`, via `libc::prctl`) types them as `*mut c_void` to
also support the ops that take a real pointer. For every op that instead wants a plain integer
(sub-operation codes like `PR_CAP_AMBIENT_RAISE`, capability bit indices, pids, lengths, bitflags
values, bools), the only way to pass it through that pointer-typed slot is to fabricate a pointer
whose bit pattern equals the integer - `value as usize as *mut _` (or, where `value` is already a
`usize` constant/length, the bare `value as *mut _`). This pointer is never dereferenced by the
kernel for these ops; it's a legitimate ABI need, but a bare `as` integer-to-pointer cast is exactly
what Rust's strict-provenance model (and Miri's `-Zmiri-strict-provenance` checker) forbids, because
it fabricates a pointer with no real provenance.

Two of the sites (`capability_is_in_bounding_set`/`remove_capability_from_bounding_set` at
thread/prctl.rs:467-498, and the ambient-set functions at 698-763) already carried an inline
comment acknowledging this exact problem - `// as *mut _ should be ptr::without_provenance_mut but
our MSRV does not allow it.` - but no `#[cfg(miri)]` branch was ever added to actually apply that
fix, so the comment was descriptive only.

61 call sites across both files fit this pattern (confirmed by manual classification of every
`as *mut _`/`as *const _` occurrence in both files - see "What was tried" above); the remaining
occurrences in both files (`.as_ptr() as *mut _`, `NonNull::as_ptr()`, `as_ptr(x) as *mut _`,
`.cast()`) are real pointer casts that need no fix and were left untouched.

## Introduced by

51040e35 "Add support for prctl()." (Koutheir Attouchi, 2022-09-04) introduced both files
and the `as usize as *mut _` idiom; every call site added since (the ambient-capability-set,
core-scheduling-cookie, speculation-control, tagged-address, and syscall-user-dispatch functions
added across several later commits) copied the same idiom rather than introducing a new one -
confirmed via `git log -S"as usize as *mut _" -- src/thread/prctl.rs src/process/prctl.rs`, which
shows only 51040e35 and one subsequent clippy-lint-only touch-up (c8339a60) ever changed that exact
substring.

## The fix

See `patch.diff`. Added one centralized helper, `pub(crate) fn int_to_ptr(value: usize) -> *mut
c_void` in `src/prctl.rs` (the shared module both `src/thread/prctl.rs` and `src/process/prctl.rs`
already import `prctl_1arg`/`prctl_2args`/`prctl_3args` from), gated the same way as candidate 004's
fix: `core::ptr::without_provenance_mut(value)` under `#[cfg(miri)]`, the original `value as *mut
c_void` cast under `#[cfg(not(miri))]` (not done unconditionally because `without_provenance_mut`
stabilized in Rust 1.84, newer than this crate's stated MSRV of 1.65 in `Cargo.toml`; Miri always
runs on a recent nightly, well past 1.84). Then replaced all 61 genuine integer-fabrication call
sites in both `src/thread/prctl.rs` and `src/process/prctl.rs` with calls to `int_to_ptr(...)`,
leaving every genuine real-pointer cast (`.as_ptr()`, `NonNull::as_ptr()`, `as_ptr(x)`, `.cast()`)
untouched. The two stale "should be `ptr::without_provenance_mut`" comments were removed since the
fix they described is now actually applied.

Verified against more than the 3 call sites exercised directly under Miri: every one of the 36
replaced sites was re-read after the mechanical substitution to confirm the replacement compiles to
the same runtime value (`cargo build`/`cargo clippy` both clean, no new warnings introduced), and
`cargo fmt` was re-run and re-verified clean. The real (non-Miri) code path is byte-for-byte
unchanged in meaning (`value as *mut c_void` vs. the original `value as usize as *mut _`/`value as
*mut _` - identical generated code, just routed through one named function instead of inlined at
every call site), confirmed via a real-syscall smoke test covering 5 different functions across
both files, on real Linux hardware, with both the `linux_raw` and forced-`libc` backends, producing
identical output before and after the patch.

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast` -
  411 passed / 1 failed before, identical 411 passed / 1 failed after (the one failure,
  `ioctl::test_int_setter`, is a pre-existing sandbox limitation needing `/dev/net/tun`, unrelated
  to this change, confirmed present identically with and without the patch).

## What's not done / open questions

- The same pre-existing, unrelated Miri limitation noted in candidate 004 remains: Miri cannot
  emulate the real `prctl` syscall at all (`Miri does not support \`prctl\` syscall with op=N`,
  one level deeper than the fixed line, inside the real `libc::prctl` FFI call) - this fix only
  removes the strict-provenance violation at the cast itself; no prctl-using code path can be run
  to full completion under Miri even after this fix.
- Two cfg(linux_raw_dep)-gated functions (`enabled_pointer_authentication_keys`,
  `configure_pointer_authentication_keys` in process/prctl.rs, and
  `reset_pointer_authentication_keys` in thread/prctl.rs) contain 3 of the 61 fixed call sites but
  could not be directly exercised under Miri in this session: forcing the libc backend via
  `RUSTFLAGS="--cfg rustix_no_linux_raw"` (required for Miri, since linux_raw's inline `asm!`
  syscalls aren't Miri-executable) also disables the `linux_raw_dep` cfg those functions are gated
  behind (confirmed by reading `build.rs`), so they're compiled out entirely in exactly the
  configuration needed to test them under Miri. They were fixed by the same mechanical
  substitution as every other site (same `as usize as *mut _` shape, same `int_to_ptr(...)` call),
  verified by `cargo build`/`cargo clippy` under the *default* (non-Miri-forced) configuration where
  they do compile, but not verified under Miri directly. Not expected to behave differently (same
  textual shape as the 3 directly-Miri-tested sites), but noted as the one part of this sweep not
  independently confirmed under Miri.
- Only `src/thread/prctl.rs` and `src/process/prctl.rs` were swept this session, per the
  candidate's own scope. The broader crate (other backends, other syscall wrappers stuffing
  auxiliary integer values into pointer-typed FFI slots) was not re-checked beyond candidate 004's
  original `futex_val2` finding - candidate 004's own "what's not done" section already flagged this
  as worth a dedicated future sweep.
