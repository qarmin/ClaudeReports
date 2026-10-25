# prctl_name_no_nul_zero_length_fallback

`src/thread/prctl.rs`'s `name()` (lines 79-85) does `let len = buffer.iter().position(|&x| x ==
0_u8).unwrap_or(0);` after a `PR_GET_NAME` prctl into a 16-byte buffer - if no NUL is found, it
silently treats the name as zero-length rather than surfacing a decode error. Flagged with low
confidence: PR_GET_NAME's documented kernel contract guarantees NUL-termination within
`TASK_COMM_LEN` (16) bytes, so the fallback path looked like dead code under the documented ABI.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Kernel source reading: `kernel/sys.c`'s `sys_prctl` handler for `PR_SET_NAME`/`PR_GET_NAME` (fetched `https://raw.githubusercontent.com/torvalds/linux/master/kernel/sys.c`) | Current upstream `master` | Found an explicit, unconditional `comm[sizeof(me->comm) - 1] = 0;` executed *before* `strncpy_from_user(comm, (char __user *)arg2, sizeof(me->comm) - 1)` in the `PR_SET_NAME` case - this force-nulls the last byte of the local `comm` buffer regardless of what the user-supplied source string contains or whether it has a NUL within 15 bytes. `PR_GET_NAME` then round-trips through `get_task_comm()` -> `strscpy(buf, tsk->comm, TASK_COMM_LEN)`, copying from a `tsk->comm` that is therefore always NUL-terminated within 16 bytes by construction. No code path in this function can leave `tsk->comm` without a NUL within `TASK_COMM_LEN` bytes. |
| Manual boundary-value construction, real syscall, attempt 1: write 16 non-NUL bytes directly to `/proc/self/comm` (bypasses rustix's own `set_name`, which only accepts a `CStr` and thus can never construct an unterminated source) | `write(open("/proc/self/comm", O_WRONLY), "1234567890123456", 16)`, x86_64, this container's real kernel, release-equivalent C binary (`gcc -O0`), 1 run | Write succeeded (returned 16), but a subsequent `prctl(PR_GET_NAME)` returned the truncated name `"123456789012345"` + NUL at byte 15 - truncation happened silently, NUL was still present within 16 bytes. |
| Manual boundary-value construction, real syscall, attempt 2 (adversarial): call `prctl(PR_SET_NAME, ptr)` directly where `ptr` points at a region filled with 20 non-NUL bytes, placed immediately before an `mprotect(PROT_NONE)` guard page (so any kernel code path that scanned past 20 bytes looking for a NUL would fault) | Same container, real kernel syscall, 1 run | `PR_SET_NAME` returned 0 (success); subsequent `PR_GET_NAME` returned `"BBBBBBBBBBBBBBB"` + NUL at byte 15 - still correctly truncated and NUL-terminated, confirming the kernel's explicit force-null (found in the source reading above) rather than relying on scanning the source string for a NUL. |
| Real end-to-end run of the actual public API: a standalone crate depending on this worktree's `rustix` via a path dependency, using `libc::prctl(PR_SET_NAME, ...)` with the same guard-page-adjacent 20-byte non-NUL buffer as attempt 2, then calling the real, unmodified `rustix::thread::name()` | `cargo run --release`, rustix 1.1.5 (this worktree, commit 287214b8), `libc` 0.2.189, x86_64, this container's real kernel, 1 run | Printed `rustix::thread::name() = "BBBBBBBBBBBBBBB" (len=15)` - the real public API call returned the correctly truncated, non-empty name; the `unwrap_or(0)` fallback was never taken (`CString::new` never received an empty slice). |

## Status

**Not reproducible.** No input - including a direct, adversarial `prctl(PR_SET_NAME)` call with a
guard-page-adjacent buffer specifically designed to defeat any "scan the source for a NUL" kernel
implementation - produced a `tsk->comm` without a NUL within 16 bytes, consistent with the kernel
source explicitly forcing `comm[15] = 0` before ever copying from the user-supplied source. This
covers 4 distinct angles (kernel source reading, two adversarial raw-syscall boundary-value tests,
and one real end-to-end test through rustix's own public API) - exceeding the loop's normal soft
cap of 3 because the first three each independently suggested "not reachable" and the fourth
confirms the real library's public entry point agrees in practice, not just in isolated syscall
tests.

**Fully reproducible**: N/A (no reproduction to report - this is a confirmed-safe, not a
confirmed-bug, result).

## Class

N/A - not a bug.

## Root cause

N/A - not a bug, but the mechanism that makes the suspect code dead under the real ABI is itself
worth recording: Linux's `sys_prctl` (`kernel/sys.c`) unconditionally zeroes the last byte of its
local `comm` scratch buffer (`comm[TASK_COMM_LEN - 1] = 0`) *before* copying the user-supplied name
into it with `strncpy_from_user(..., TASK_COMM_LEN - 1)`, so the resulting buffer handed to
`set_task_comm()` (and later read back via `get_task_comm()`/`strscpy()` on `PR_GET_NAME`) is
always NUL-terminated within `TASK_COMM_LEN` (16) bytes, regardless of the source string's content
or length. `rustix`'s `name()` therefore can never observe a 16-byte buffer with no NUL in it when
talking to a genuine Linux kernel through the documented `prctl` ABI - `buffer.iter().position(...)
.unwrap_or(0)`'s `unwrap_or(0)` branch has no reachable real-world trigger.

## Introduced by

N/A - not applicable, no bug confirmed.

## The fix

N/A - no bug found, nothing to fix. (The `unwrap_or(0)` fallback remains defensive-but-dead code
under the real ABI; a maintainer could reasonably choose to turn it into an explicit
`io::Errno::ILSEQ`/panic instead of silently synthesizing an empty name, purely for
defense-in-depth against a hypothetical non-conforming kernel/emulation layer, but that is a style
choice, not a bug fix, and was not done here.)

- Project test suite: N/A, no fix was made (nothing to verify a before/after against). For
  reference, this worktree's `cargo test --workspace` (default features) was not run since no
  change was made to the tree.

## What's not done / open questions

- Only tested against this container's real host kernel (`7.0.0-34-generic`, x86_64, real `prctl`
  syscall, not emulated/gVisor as far as could be determined - `Seccomp_filters: 1` present but no
  `gvisor` marker in `dmesg`). Not tested against: a non-Linux-genuine syscall emulation layer
  (WSL1, a user-mode syscall translator, a heavily modified/embedded kernel) that might not
  replicate the `comm[TASK_COMM_LEN - 1] = 0` force-null behavior found in upstream kernel source -
  out of scope for this session, and the candidate's own low-confidence framing already anticipated
  this.
- Only checked current upstream (`master`) kernel source for the force-null line; did not walk
  kernel git history to confirm how far back this exact guarantee goes (TASK_COMM_LEN and the
  general `prctl(PR_SET_NAME/PR_GET_NAME)` mechanism are old - pre-2.6 era - and the empirical tests
  above ran against a real, current kernel, so this is a minor completeness gap, not a live
  uncertainty about *this* session's conclusion).
- Did not check whether any other `rustix` function has the same "trust `position(...)
  .unwrap_or(0)` on a kernel-filled fixed buffer" shape elsewhere in the codebase; a quick
  `grep -rn "unwrap_or(0)" src/` turned up no other occurrence of this exact pattern in this
  checkout (287214b8), so no sibling candidate was added to `IDEAS.md` for it.
