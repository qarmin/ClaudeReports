# errno_decode_range_trust_not_reproducible

`src/backend/linux_raw/io/errno.rs`'s `try_decode_raw_fd`/`try_decode_void`/`try_decode_error` use
`debug_assert!(raw.is_in_range(-4095..0))` (checked only in debug builds) immediately followed by
an **unconditional** `core::intrinsics::assume(raw.is_in_range(-4095..0))` (present in every build
that has `core_intrinsics`, not gated by `debug_assertions`) once the fast-path check
(`is_negative()`/`is_nonzero()`) already indicated an error. Flagged by candidate 002 as a
structurally similar "trust the range" pattern to its own confirmed bug, but with input sourced
from the kernel's own real syscall return rather than FFI/user data - suspected lower risk, worth a
quick check.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading: every architecture's raw-syscall asm wrapper (`src/backend/linux_raw/arch/*.rs`) that produces the `RetReg<Num>` value these functions decode | All 13 supported architectures (`aarch64`, `arm`/`thumb`, `mips`/`mips32r6`/`mips64`/`mips64r6`, `powerpc`/`powerpc64`, `riscv64`, `s390x`, `x86`/`x86_64`) | 4 MIPS variants (`mips.rs`, `mips32r6.rs`, `mips64.rs`, `mips64r6.rs`) explicitly `wrapping_neg()` the raw MIPS `$a3`-flagged positive-errno convention into the standard "negative value in `-4095..0`" encoding *before* returning from the asm wrapper - confirmed by reading the actual `asm!` blocks (MIPS signals errors via the `$a3` register, not a negative return value, per the file's own doc comment: "mipsel Linux indicates success or failure using `$a3` rather than by returning a negative error code as most other architectures do"). All other architectures rely on the kernel's own native negative-errno-on-error convention directly, with no translation needed. |
| Call-site audit (`grep -rn "try_decode_error\|try_decode_void\|try_decode_raw_fd" src/backend/linux_raw/`) | Whole `linux_raw` backend | All three functions are only ever called (via `conv.rs`) on `RetReg<Num>` values produced by the `syscall!`/`syscall_readonly!` macros - i.e. a real raw syscall's real return register(s), already arch-normalized as above. No call site feeds a non-syscall value (e.g. the separate vDSO C-function-pointer call path in `vdso_wrappers.rs` uses a completely different decoding mechanism - `MaybeUninit::assume_init`, not this errno-range path - so it never reaches these functions at all). |

## Status

**Not reproducible**. The `-4095..0` invariant these `debug_assert!`/`assume()` pairs rely on holds
by construction on every architecture `linux_raw` supports: either the kernel's native calling
convention already guarantees it (x86/x86_64/arm/aarch64/riscv64/s390x/powerpc/powerpc64), or the
arch-specific wrapper explicitly translates a different native convention (MIPS's `$a3`-flag
scheme) into the same encoding before the value ever reaches `errno.rs`'s generic decoding
functions. There is no call site where a value violating the invariant could reach `try_decode_
raw_fd`/`try_decode_void`/`try_decode_error`.

## Root cause

N/A - not a bug. The suspected risk (a syscall-sourced value violating the range invariant the
`assume()` depends on, causing real UB in a release build with no `debug_assert!` to catch it
first) requires a code path that hands one of these functions a `RetReg<Num>` that didn't come from
a genuine, correctly-decoded raw syscall. No such path exists: every call site is downstream of the
`syscall!`/`syscall_readonly!` macros, and every architecture's asm wrapper already normalizes to
the same encoding. This differs from candidate 002's confirmed bug in exactly the way the backlog
note predicted: candidate 002's `Errno::from_raw_os_error` is public API accepting arbitrary
`c_int` from **any caller** (FFI, user code, no invariant enforced at the type level), while these
three functions are `pub(in crate::backend)`, `unsafe fn`s with a documented safety precondition
("must only be used with syscalls which return X on success") that every actual call site in this
codebase genuinely satisfies.

## Introduced by

N/A - not applicable, no bug confirmed.

## The fix

N/A - no bug found, nothing to fix.

## What's not done / open questions

- This only audited `linux_raw`'s own architecture wrappers as they exist in this checkout
  (287214b8). If a future architecture is added to `linux_raw/arch/`, its wrapper would need the
  same check (native negative-errno convention, or explicit translation like MIPS) to preserve this
  invariant - not something this candidate can check preemptively.
- Not re-verified under an actual non-x86_64 architecture (this container is x86_64 only) - the
  conclusion rests on reading the `asm!` blocks and trusting they match each architecture's
  documented Linux syscall ABI, not on running the code under qemu-user for each arch. Given the
  invariant is about a static ABI fact (how each architecture's kernel signals syscall errors) which
  doesn't depend on runtime data, code reading is a reasonably strong confirmation here, but a
  qemu-user cross-arch run (see `COMMANDS.md`'s aarch64/qemu recipe, extendable to mips) was not
  performed this session.
