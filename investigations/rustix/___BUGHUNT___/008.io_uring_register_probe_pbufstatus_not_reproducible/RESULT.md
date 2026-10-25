# io_uring_register_probe_pbufstatus_not_reproducible

Follow-up to candidate 006 (`io_uring_register`'s `arg: *const c_void` should be `*mut c_void` -
confirmed for `IoringRegisterOp::RegisterRingFds`). Two other opcodes were flagged during that
investigation as plausibly sharing the exact same "kernel writes through the `arg` pointer" shape:
`IoringRegisterOp::RegisterProbe` (`IORING_REGISTER_PROBE`) and `IoringRegisterOp::RegisterPbufStatus`
(`IORING_REGISTER_PBUF_STATUS`). This candidate investigated both independently.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Struct layout verification | `linux-raw-sys` 0.12.1 and this container's `/usr/include/linux/io_uring.h` cross-checked against rustix's own `src/io_uring/mod.rs` definitions | Layouts match exactly; `io_uring_buf_status.head` is explicitly commented `/* output */` in the kernel header |
| Real `IORING_REGISTER_PROBE` call, both via rustix's `io_uring_register_with` and a raw `syscall(SYS_io_uring_register, ...)` | Real `io_uring_setup`, poisoned (`0xFF`-filled) `io_uring_probe` + trailing `ops[]` buffer sized for `nr_args` in `{0,1,8,16,32,64,128,200,256,300}`, each tried more than 3 times | `-EINVAL` on every single attempt, independent of `nr_args` - inconsistent with a real, wired-up `io_probe()` implementation, which should behave differently across that range |
| Real `IORING_REGISTER_PBUF_RING` setup (prerequisite for `PBUF_STATUS`), then real `IORING_REGISTER_PBUF_STATUS` call | Real `io_uring_setup` + real buffer-ring registration (bgid 42, 8 entries, user-mmap'd memory - succeeded, `ret=0`), then `PBUF_STATUS` with `buf_group=42` and poisoned `head=0xFFFFFFFF`, `nr_args` in `{0,1,2}`, more than 3 times | `-EINVAL` every time, indistinguishable from calling a deliberately-bogus opcode (100) on the same fd |
| Kernel/header investigation | `/usr/include/linux/io_uring.h` and `/usr/include/linux/io_uring/query.h` in this container | Found `IORING_REGISTER_QUERY = 35` with a `struct io_uring_query_opcode` that duplicates exactly what `PROBE` used to report, in a header file dated "Copyright (C) 2026" |

## Status

**Not reproducible in this environment.** Both opcodes return `-EINVAL` unconditionally on this
container's kernel (`uname -r` = `7.0.0-31-generic`), regardless of otherwise-valid arguments,
consistent with this specific kernel build having removed/superseded both
`IORING_REGISTER_PROBE` and `IORING_REGISTER_PBUF_STATUS` in favor of a newer `IORING_REGISTER_QUERY`
mechanism - the real syscall never reaches any code path that would write through `arg` for either
opcode on this kernel, so no empirical write-through could be observed here. This is a
kernel-environment limitation, not a refutation of the underlying suspicion.

**Fully reproducible**: N/A (not confirmed).

## Class

N/A (not confirmed this session).

## Root cause

Not confirmed empirically in this environment, but per the kernel header comments examined
(`ops`/`ops_len` in `io_uring_probe` are meant to be filled by `io_probe()`; `head` in
`io_uring_buf_status` is explicitly marked `/* output */`), both opcodes almost certainly share
candidate 006's exact defect on any kernel where they're actually implemented (mainline 5.6+ for
`PROBE`, 6.8+ for `PBUF_STATUS`) - `io_uring_register`/`io_uring_register_with`'s `arg: *const c_void`
would genuinely be written through by the kernel there too. This container's unusually new/exotic
kernel build happens to reject both opcodes outright before ever touching the buffer, which is why
this session could not get direct empirical confirmation here.

## Introduced by

N/A (not confirmed this session; would be the same as candidate 006's - see that candidate's
"Introduced by" for the shared `*const c_void` signature history).

## The fix

**No new patch is needed for this candidate.** If/when confirmed on a mainline kernel that still
implements these opcodes, the fix is the same one already independently verified for candidate 006:
changing `arg: *const c_void` to `*mut c_void` on the shared `io_uring_register`/
`io_uring_register_with` signatures in `src/io_uring/mod.rs` (and both backends) - not a per-opcode
fix, since all `IoringRegisterOp` variants share the same function signature. This investigation is
recorded as additional suspected scope for candidate 006, not a standalone bug requiring its own
patch.

- Project test suite: N/A - no code change was made or needed for this candidate.

## What's not done / open questions

- Neither opcode's write-through behavior could be empirically confirmed on any kernel in this
  session - a future session with access to a mainline kernel (5.6-6.17 range, before whatever
  superseded these opcodes on this container's kernel) could complete this verification using the
  same repro approach (poison the buffer, call the real opcode, check if it changed).
- This container's kernel appears to have a `IORING_REGISTER_QUERY` (opcode 35) mechanism not yet
  present in the vendored `linux-raw-sys` 0.12.1 struct definitions used by this crate - worth a
  separate, unrelated follow-up note (not a bug in rustix, just an ABI-freshness observation) if a
  future session wants to investigate `io_uring` support for very recent kernels.
- Also noted (not independently confirmed, backlog-only): `tests/io_uring/register.rs`'s existing
  skip logic (treating `Errno::INVAL` from a zeroed `io_uring_buf_reg` as "kernel doesn't support
  `PBUF_RING`") is fragile - an all-zero `io_uring_buf_reg` (in particular `ring_entries=0`)
  legitimately returns `-EINVAL` on a kernel that *does* fully support `PBUF_RING`, since that's
  "invalid ring_entries," not "opcode unsupported." This could cause the existing test to silently
  pass via early-return rather than actually exercising the ring in some environments - not
  confirmed to happen today, flagged for a future look.
