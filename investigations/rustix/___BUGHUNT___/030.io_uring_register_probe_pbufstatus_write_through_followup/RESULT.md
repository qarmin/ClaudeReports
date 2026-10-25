# io_uring_register_probe_pbufstatus_write_through_followup

Follow-up to candidate 008 (itself a follow-up to candidate 006, the confirmed
`io_uring_register`/`io_uring_register_with` `arg: *const c_void` should be `*mut c_void` bug for
`IoringRegisterOp::RegisterRingFds`). Candidate 008 flagged `IoringRegisterOp::RegisterProbe`
(`IORING_REGISTER_PROBE`) and `IoringRegisterOp::RegisterPbufStatus` (`IORING_REGISTER_PBUF_STATUS`)
as plausibly sharing the exact same kernel-writes-through-`arg` shape, but could not get empirical
confirmation because this container's kernel (`7.0.0-31-generic` at the time) returned `-EINVAL`
for both opcodes unconditionally. This candidate was dispatched explicitly to re-check on a later
session, on the chance the container had since moved to a kernel that implements them.

`src/io_uring/mod.rs` still declares both `io_uring_register` and `io_uring_register_with` with
`arg: *const c_void` at the current `HEAD` (287214b8, rustix 1.1.5) - the signature candidate 006
flagged is unchanged, so the suspicion itself is still live; only the empirical-confirmation
question was re-opened here.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Re-check container kernel version | `uname -a`, `/etc/os-release`, `/proc/sys/kernel/io_uring_disabled`, capability bits | Kernel is now `7.0.0-34-generic` (Ubuntu 26.04 "Resolute Raccoon", version-signature `Ubuntu 7.0.0-34.34-generic 7.0.14`) - three point releases past candidate 008's `7.0.0-31-generic`, same oddity (a kernel version with no real-world mainline counterpart as of this date), `io_uring_disabled=0`, no elevated capabilities (`CapEff=0`, `CapBnd=0xcb`), `no_new_privs` set (blocks `sudo`) |
| Real `IORING_REGISTER_PROBE` call via a raw `syscall(SYS_io_uring_register, ...)` (not mediated by rustix) | Real `io_uring_setup`, a `0xFF`-poisoned `io_uring_probe` + trailing `ops[]` buffer sized for 256 entries | `-EINVAL`; buffer provably untouched (the "did it change from poison" check in the C repro never ran its printf because `ret < 0`) |
| Real `IORING_REGISTER_PBUF_RING` (prerequisite) then real `IORING_REGISTER_PBUF_STATUS`, same raw-syscall approach | Real buffer-ring registration (bgid 42, 8 entries, real `mmap`'d ring memory) - succeeded (`ret=0`); then `PBUF_STATUS` with `buf_group=42` and a `0xABABABAB`-poisoned `io_uring_buf_status` | `-EINVAL`; `status.head` printed back out still exactly `0xabababab`, the poison value, provably untouched |
| Escalation: sanity sweep of neighboring opcodes to rule out "io_uring itself is blocked/sandboxed in this container" | `IORING_REGISTER_PERSONALITY` (opcode 9, `arg=NULL`, `nr_args=0`) and `IORING_REGISTER_ENABLE_RINGS` (opcode 12) on the same real `fd`, plus an explicitly bogus opcode 999 for comparison | `PERSONALITY` succeeded for real (`ret=1`, a genuine fresh personality id); `ENABLE_RINGS` returned a distinct, semantically real error (`EBADFD`, because `IORING_SETUP_R_DISABLED` was never set at `io_uring_setup` time) - proving the syscall path is not blanket-blocked. The bogus opcode 999 returned the same flat `-EINVAL` that PROBE and PBUF_STATUS get |
| Escalation: full opcode sweep, `arg=NULL`/`nr_args=0`, opcodes 0 through 36 | Single process, one `io_uring_setup`, 37 sequential `io_uring_register` calls | Every opcode *except* 8 (PROBE) and 26 (PBUF_STATUS) returns an opcode-specific, semantically plausible error for a null/zero-arg call (`EFAULT` for buffer-pointer opcodes, `ENXIO` for file-table opcodes, `EBADFD` for ring-state opcodes, outright success for 3 opcodes that tolerate null/zero) - i.e. every other opcode's real handler is reached and does real argument validation. Only opcodes 8 and 26 fall back to the same flat `-EINVAL` a genuinely unrecognized opcode (999) produces |

Full captured output of all four runs is in `output.txt`.

## Status

**Not reproducible** in this environment, for the second session running. Three escalating
checks (direct raw-syscall probe of each opcode individually with a real, poisoned buffer; a
targeted two-opcode sanity check to rule out a blanket io_uring block; a full 0-36 opcode sweep to
characterize exactly how these two opcodes differ from every neighbor) all converge on the same
conclusion: on this container's kernel, `IORING_REGISTER_PROBE` and `IORING_REGISTER_PBUF_STATUS`
behave identically to a completely unimplemented/unrecognized opcode - they return `-EINVAL`
before ever reaching whatever code would write through the `arg` pointer, regardless of whether
the supplied arguments are well-formed. No partial signal (a buffer that *sometimes* changed, an
inconsistent errno) was seen at any point, so there is no live thread to keep digging on - this is
a clean negative, not a flaky one.

**Fully reproducible**: N/A (not confirmed).

## Class

N/A (not confirmed this session).

## Root cause

Not confirmed empirically in this environment, for the same reason as candidate 008: both opcodes'
real handlers (which the kernel header still documents as writing through the pointee - `ops`/
`ops_len` in `struct io_uring_probe` are filled by `io_probe()`, and `head` in
`struct io_uring_buf_status` is explicitly commented `/* output */` in
`/usr/include/linux/io_uring.h`, unchanged from candidate 008's session) are never reached on this
particular kernel build. The new full-opcode-sweep evidence gathered this session (every
*other* opcode gets a distinct, opcode-specific validation error, only these two get the generic
"unimplemented" `-EINVAL`) makes it more likely - not less - that this is a property of this
specific sandboxed/container kernel (which reports an unusual version string, `7.0.0-34-generic`,
with no real-world mainline counterpart as of 2026-10) rather than something that would reproduce
differently on a standard distro kernel; but this still cannot be verified without access to such
a kernel.

## Introduced by

N/A (not confirmed this session; would be the same as candidate 006's/008's - see candidate 006's
"Introduced by" for the shared `*const c_void` signature history. The signature is still unchanged
at current `HEAD`, 287214b8).

## The fix

No new patch for this candidate - same reasoning as 008. If/when confirmed on a kernel that
actually implements these two opcodes, the fix is the one already verified for candidate 006:
`arg: *const c_void` -> `*mut c_void` on `io_uring_register`/`io_uring_register_with` (and both
backends), since it's a shared function signature across all `IoringRegisterOp` variants, not a
per-opcode fix.

- Project test suite: N/A - no code change was made for this candidate.

## What's not done / open questions

- Still no access, across two separate sessions now, to a kernel that actually implements
  `IORING_REGISTER_PROBE` (mainline since 5.6) or `IORING_REGISTER_PBUF_STATUS` (mainline since
  6.8) in this container type. A future session with access to a different kernel (a VM, a
  different container runtime, or a machine running a standard distro kernel in that version
  range) could complete the empirical confirmation using the exact same raw-syscall approach in
  `scripts/test.c` / `scripts/test3.c` in this folder (poison the buffer, call the real opcode,
  check whether it changed).
- Given this is the *second* consecutive session hitting the identical wall on what appears to be
  the same sandbox/container kernel family (version bumped 7.0.0-31 -> 7.0.0-34, same behavior), a
  third follow-up candidate re-attempting this exact check without a genuinely different kernel
  available is unlikely to produce a different outcome - recorded in
  `___BUGHUNT___/projects/rustix.md` so a future session can decide whether to keep retrying here
  or deprioritize until a different kernel is available.
- Not independently re-checked this session (out of scope, carried over from 008): the
  `tests/io_uring/register.rs` skip-logic fragility note about an all-zero `io_uring_buf_reg`
  legitimately returning `-EINVAL` for `ring_entries=0` rather than "opcode unsupported".
