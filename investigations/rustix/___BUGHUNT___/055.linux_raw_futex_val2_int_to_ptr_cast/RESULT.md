# linux_raw_futex_val2_int_to_ptr_cast

`src/backend/linux_raw/thread/syscalls.rs:222`, in `futex_val2`, has `let timeout = val2 as usize
as *const Timespec;` - textually identical to the confirmed bug in the libc-backend sibling
(candidate 004, `src/backend/libc/thread/syscalls.rs:478`). Candidate 004 explicitly scoped its
Miri run to the libc backend because "linux_raw's inline-asm syscalls aren't Miri-executable at
all," so this linux_raw sibling - the default backend most users of
`rustix::thread::futex::requeue`/`cmp_requeue`/`wake_op` actually run through - had never been
independently checked.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/backend/linux_raw/thread/syscalls.rs::futex_val2`, `src/backend/linux_raw/reg.rs` (`ArgReg`/`ToAsm`), `src/backend/linux_raw/conv.rs` (`From<*const T> for ArgReg`) | Same ABI-mandated `val2`-in-`timeout`-slot trick as candidate 004, built the same Miri-hostile way; confirmed the pointer is only ever used as a bit pattern fed into inline asm register operands, never dereferenced anywhere in the call chain |
| Checked whether candidate 004's "unconfirmable by Miri" claim is actually a hard project constraint, not just an assumption | Read `build.rs` (`"Miri doesn't support inline asm ... so if we're running under miri, use the libc backend"`, `let miri = ...; let libc = ... \|\| miri \|\| ...`) and `Cargo.toml`'s `target.'cfg(...)'.dependencies` sections (hard-code `not(miri)` / `miri` directly into which `linux-raw-sys` feature set, e.g. `errno`/`auxvec`/`elf`, gets pulled in) | Confirmed this is a deliberate, two-layer project-level gate (build-script logic AND Cargo manifest target-cfg dependency selection), not an assumption - under real `cargo +nightly miri run`, the `libc`-backend code at `src/backend/libc/thread/syscalls.rs:478` is hit even with no extra flags, never the linux_raw path |
| Bypassed that gate for investigation only (not the fix): patched `build.rs` (`\|\| miri` -> `\|\| (miri && false)`) and `Cargo.toml`'s three `target.'cfg(...)'` sections (dropped the `not(miri)`/`miri` clauses) in the working tree, then `cargo +nightly miri run` an example calling the real public `rustix::thread::futex::requeue` | Default (permissive) Miri | Compiles and links against the real linux_raw backend; reaches and runs code up to the exact suspect line, emitting the identical-shaped warning (`integer-to-pointer cast`) at `src/backend/linux_raw/thread/syscalls.rs:222:19`, then continues and fails later at the first real `asm!` block (`src/backend/linux_raw/arch/x86_64.rs:253`, `"unsupported operation: inline assembly is not supported"`) - empirically confirming candidate 004's "not Miri-executable at all" claim about the asm itself, while also proving the suspect cast line is reached and executed before that wall is hit |
| Same bypass, `-Zmiri-strict-provenance` | | Hard error, deterministic, 3/3 runs: `"unsupported operation: integer-to-pointer casts and \`ptr::with_exposed_provenance\` are not supported with -Zmiri-strict-provenance"` at `src/backend/linux_raw/thread/syscalls.rs:222:19` - the exact same error shape, at the linux_raw sibling line, that candidate 004 found at the libc line. Re-run independently a third time from a completely fresh `git clone` (not just the working tree) with the same result |
| Fix applied (`#[cfg(miri)] core::ptr::without_provenance::<Timespec>(val2 as usize)`, else original cast), same bypass + `-Zmiri-strict-provenance` re-run | | The error at line 222 is gone; Miri proceeds further into the same call and hits a separate, pre-existing, out-of-scope cast in `src/backend/linux_raw/conv.rs`'s `pass_usize` (used for every plain integer syscall argument across the whole linux_raw backend, not just futex) - confirms the fix is correctly scoped to the targeted line, see "What's not done" |
| Real-syscall smoke test on real Linux, fix applied, normal (non-Miri, non-bypassed) build | direct call to `rustix::thread::futex::requeue` with nonzero `val2` | Succeeds normally (`done`) - the real (non-Miri) linux_raw code path, which the `#[cfg(miri)]` gate leaves untouched, is unaffected |
| `cargo test --workspace --features "std,all-apis" --no-fail-fast` before/after fix | | No regressions: 413 passed / 1 failed before, identical 413 passed / 1 failed after (the one failure, `ioctl::test_int_setter`, is the same pre-existing sandbox limitation candidate 004 noted, unrelated to this change). The 5 real futex tests all pass before and after, though none of them happen to exercise `requeue`/`cmp_requeue`/`wake_op` with nonzero `val2` - covered separately by the real-syscall smoke test above |

## Status

**Confirmed**. Deterministic and reproduced on every run (3/3) under `-Zmiri-strict-provenance`,
both in the working tree and independently from a completely fresh clone - this is a
compile-time-shape bug, not flaky in any way. Confirming it required bypassing the project's own
two-layer "always use libc under Miri" gate (`build.rs` + `Cargo.toml` target-cfg dependency
selection) for investigation purposes only - this bypass is not part of the fix and is not
expected to ever ship; it exists purely to demonstrate that the linux_raw sibling has the identical
latent issue as candidate 004, not merely "probably does, by analogy."

**Fully reproducible**: Yes. The repro calls the real public API (`rustix::thread::futex::requeue`)
which internally reaches the exact suspect line through its real call path - no internal
scaffolding bypassed for the futex code itself. (The build-config bypass is a prerequisite to
reach linux_raw under Miri at all, not a stand-in for the futex call path or for a second
participant - there is no second participant in this finding, CONFIRMATION.md section 11's bar
does not apply here.)

## Class

`UB` - a strict-provenance violation as flagged by Miri, identical in kind to candidate 004. Not
exploitable as a real memory-safety bug on any real hardware today (the fabricated pointer is
never dereferenced, only used to carry an integer payload through an inline-asm register operand
the kernel reinterprets as an integer) - but it is undefined behavior under Rust's abstract
machine, and would block running this code path under Miri the moment the project's own
"always use libc under Miri" routing (or Miri's own inline-asm support) ever changes to let
linux_raw's code run further than it does today.

## Root cause

`src/backend/linux_raw/thread/syscalls.rs:222` (current HEAD, `futex_val2`):

```rust
let timeout = val2 as usize as *const Timespec;
```

Mechanically identical to candidate 004's libc-backend finding: Linux's `futex(2)` reuses its 4th
argument slot for `FUTEX_REQUEUE`/`FUTEX_CMP_REQUEUE`/`FUTEX_WAKE_OP` as a raw 32-bit integer
(`val2`), not a `struct timespec *`. The linux_raw backend passes syscall arguments as
pointer-typed register operands specifically "to preserve provenance" for genuine pointers (see
`src/backend/linux_raw/reg.rs`'s own doc comment), so `futex_val2` has to fabricate a `*const
Timespec` whose bit pattern equals `val2` purely to occupy that argument's type slot before it's
fed into `asm!`'s `in(reg)` operand - this pointer is never dereferenced by Rust or by the kernel's
treatment of the smuggled value. That's the same legitimate, ABI-mandated need as 004, built with
the same Miri-hostile bare `as` cast.

The one thing genuinely different from 004: this specific line was never actually reachable under
Miri at all before this session, because `build.rs` (`let libc = ... || miri || ...`) and three
`target.'cfg(...)'.dependencies` sections in `Cargo.toml` hard-route any Miri compilation straight
to the libc backend, specifically because Miri cannot execute the inline `asm!` blocks the
linux_raw backend's actual syscalls use. That routing is a correct and necessary workaround for
the asm itself - but it also means the integer-to-pointer cast at line 222, which sits *before* any
`asm!` block executes, was never independently exercised or confirmed, only assumed-by-analogy.
This session's bypass (patch the two gates, confirm the cast is hit and flagged, confirm the
backend genuinely dies at the next real `asm!` block immediately afterward with "inline assembly is
not supported") turns that assumption into an empirical fact: the cast is real, reachable, and
Miri-rejected under strict provenance, and the inline-asm wall immediately behind it is also real
and not something a trivial patch here works around.

## Introduced by

b0d3681c "Futex Rework (#1098)" (Daniel Schemmel, 2024-08-23) - the same commit that introduced
candidate 004's libc-backend cast (`git log -S'val2 as usize as *const Timespec' --
src/backend/linux_raw/thread/syscalls.rs` returns only this one commit). The explanatory comment
citing the man page was added immediately after, in the same commit as 004's
(e2014c52 "Miscellaneous documentation and clippy fixes for futex. (#1134)", confirmed via
`git log -S` on the comment text against this file).

## The fix

See `patch.diff`: gate the cast on `#[cfg(miri)]`, using
`core::ptr::without_provenance::<Timespec>(val2 as usize)` (a strict-provenance-safe way to
fabricate a non-dereferenced pointer from an integer) only under Miri, keeping the original
`val2 as usize as *const Timespec` cast for real (non-Miri) builds - identical approach to
candidate 004's fix, applied to the linux_raw-backend sibling line. Not done unconditionally
because `core::ptr::without_provenance` stabilized in Rust 1.84, newer than this crate's stated
MSRV (`rust-version = "1.65"`); Miri always runs on a recent nightly toolchain, well past 1.84, so
gating it behind `#[cfg(miri)]` is safe without raising the crate's real MSRV.

Verified against more than the exact line by construction: the fix doesn't special-case a specific
`val2` value, it changes how *any* `val2` is turned into the smuggled pointer; the real (non-Miri)
code path is byte-for-byte unchanged (confirmed via the real-syscall smoke test with a nonzero
`val2` on real Linux hardware succeeding normally after the patch); and re-running the Miri repro
after the fix shows the targeted error is gone and execution proceeds further into the same call
(to a different, pre-existing, out-of-scope cast - see "What's not done"), not merely relocated
within the same function.

- Project test suite: `cargo test --workspace --features "std,all-apis" --no-fail-fast` - 413
  passed / 1 failed before, identical 413 passed / 1 failed after (the one failure,
  `ioctl::test_int_setter`, is a pre-existing sandbox limitation, unrelated to this change,
  confirmed present identically with and without the patch). The 5 real futex tests
  (`test_lock_unlock_pi`, `test_wait_wake`, `test_wait_timeout`, `test_waitv_wake`,
  `test_waitv_timeout`) all pass with the fix.

## What's not done / open questions

- Reaching this line under Miri at all currently requires the two-layer build-config bypass
  described above (`build.rs` + `Cargo.toml`'s `target.'cfg(...)'` dependency sections); in the
  project's actual shipped build, this code path is never compiled under Miri in the first place,
  so this fix has no effect on any real Miri run today unless that routing changes in the future
  (e.g. if Miri ever grows real inline-asm support for the `syscall` instruction). The fix is
  applied anyway because it's a genuine, zero-cost-when-inactive correction of the same latent UB
  shape as candidate 004, and because this session's own bypass demonstrates the gate can be
  worked around (for investigation, not for production) at essentially no effort - a future
  session or contributor doing the same thing should not re-discover this same cast from scratch.
- Fixing this exposed a separate, broader, out-of-scope issue one frame further into the same call:
  `src/backend/linux_raw/conv.rs`'s `pass_usize` (`raw_arg(t as *mut _)`) does the same
  integer-to-pointer-cast pattern for every plain integer syscall argument across the *entire*
  linux_raw backend (not just futex) - this is architectural (`src/backend/linux_raw/reg.rs`'s
  `ArgReg`/`ToAsm` design deliberately routes all syscall arguments through pointer-typed register
  operands "to preserve provenance" for the genuine-pointer arguments), so a real fix would need to
  touch every integer-argument conversion site in `conv.rs`, not just one line - a much larger,
  dedicated candidate of its own if the project ever wants the linux_raw backend to run cleanly
  under Miri end-to-end (on top of needing Miri to gain real inline-asm support first, which is the
  more fundamental blocker). Not investigated further or fixed this session; flagged as a new
  suspect below.
- The deeper blocker - Miri's complete lack of inline-asm execution support - was independently
  reconfirmed this session (`"unsupported operation: inline assembly is not supported"` at the
  first real `asm!` block reached), consistent with candidate 004's claim, but this was previously
  an assumption cited from 004's own write-up; it is now a directly observed fact for this project's
  specific `asm!` usage.
- Only `src/backend/linux_raw/thread/syscalls.rs`'s `futex_val2` was fixed this session; the
  broader linux_raw backend (other syscall wrappers, `conv.rs`'s integer-conversion helpers noted
  above) was not swept for other individual integer-to-pointer casts beyond what this investigation
  incidentally surfaced.
