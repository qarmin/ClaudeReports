# noreturn_syscall_seccomp_trap_not_reproducible

Sourced from **open** GitHub issue #1433 ("UB if noreturn syscall actually returns"), found while
sampling the raw issue cache in `__issues_raw/` for leads not yet turned into a candidate (98 open
issues fetched by an earlier session; only ~6 had been used by candidates 001-008). The issue
argues that under an adversarial/unusual seccomp-bpf filter, *any* syscall - including ones rustix
treats as `-> !` (never returning), like `exit`/`exit_group` - can have its syscall instruction
"return" without the kernel actually performing the requested action (e.g. a filter that returns
`SECCOMP_RET_ERRNO` for `exit_group` instead of `SECCOMP_RET_ALLOW`). If the asm wrapper for such a
syscall has nothing after the `syscall`/`svc`/`ecall` instruction, execution falls through into
whatever code follows in memory - real, arbitrary-severity UB, worse than a plain panic. The issue
proposes two fixes: an infinite loop, or a trap instruction (`ud2` on x86-64) right after the
syscall instruction so the process dies immediately instead of falling through.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading of every `syscall1_noreturn`/`indirect_syscall1_noreturn` implementation across all 12 supported architectures (`src/backend/linux_raw/arch/{x86_64,x86,aarch64,arm,thumb,mips,mips32r6,mips64,mips64r6,powerpc,powerpc64,riscv64,s390x}.rs`, plus the vDSO-outline variant in `vdso_wrappers.rs`) | Whole `linux_raw` backend | Every single architecture already implements the issue's proposed fix (option 2, trap instruction): `ud2` (x86-64, x86), `brk #0x1` (aarch64), `udf #16` (arm, thumb), `teq $0,$0` (all 4 MIPS variants), `trap` (powerpc, powerpc64), `unimp` (riscv64), `j .+2` (s390x) - all placed immediately after the syscall instruction inside the same `asm!` block, all under `options(noreturn)`. |
| Call-site audit (`grep -rn "syscall_noreturn!\|_noreturn(" src/backend/linux_raw/` excluding `/arch/`) | Whole `linux_raw` backend | Only two call sites use this path at all: `runtime_448b8ad740e2a26f.rs`'s private `exit_thread`/`exit_group` (both in the `runtime` module, which is `pub(crate)`-only - fully unreachable from any downstream crate - **and has zero internal callers anywhere else in rustix either**, confirmed by grepping for `runtime::exit_group`/`runtime::exit_thread` crate-wide), and the vDSO outline-asm `indirect_syscall1_noreturn` (same trap-instruction protection, only reachable through the same dead `exit_group`/`exit_thread` path). |
| Real empirical repro: a real seccomp-bpf filter (installed via genuine `prctl(PR_SET_SECCOMP, ...)`, not simulated) that intercepts the real `SYS_exit_group` syscall number and returns `SECCOMP_RET_ERRNO(0)` instead of `SECCOMP_RET_ALLOW` - making the actual `syscall` instruction return normally with `rax=0` without the kernel ever terminating the process, exactly the scenario the issue describes - followed by a real call to the crate's own (same-crate-only-testable, since `exit_group` is private) `exit_group(0)` | A temporary `#[test]` added directly inside `src/runtime_448b8ad740e2a26f.rs`'s own `mod tests` (see `repro/test_addition.diff` - reverted from the main tree immediately after use, never left applied), run via `cargo test --lib --features "std,all-apis" bughunt_018_noreturn_seccomp_repro -- --test-threads=1 --nocapture` (`all-apis` without `use-libc`, since the `runtime` module is gated `#[cfg(linux_raw)]` and doesn't exist under the `libc` backend) | The process was killed by **SIGILL (signal 4)** immediately after the seccomp-intercepted `exit_group(0)` call - confirmed 2/2 runs (once standalone, once again via `just test` against a fresh clone). The `unreachable!()` panic message that would indicate the real bug (issue #1433 actually reproducing) was never reached in either run. |

## Status

**Not reproducible**. This is a genuinely empirically-verified negative result, not just a
code-reading conclusion: a real seccomp-bpf filter was installed and did successfully intercept the
real `exit_group` syscall (proving the "syscall may return unexpectedly" premise from the issue is
real and triggerable), and rustix's existing `ud2` trap instruction fired exactly as designed,
killing the process via SIGILL before any UB could occur. The specific vulnerable pattern the issue
describes (`-> !` function falling through after an unexpectedly-returning syscall) does not exist
in the current codebase on any of the 12 supported architectures examined - **and** the one path
where it could theoretically matter (`exit_thread`/`exit_group`) is currently unreachable dead code
(private module, zero internal callers), so even a latent gap on an architecture this container
can't test (e.g. csky, loongarch64) would not be externally triggerable in the released crate today.

## Root cause

N/A - no bug found. The maintainers appear to have already implemented the issue's own proposed fix
across the entire `linux_raw` backend at some point before this session's checkout (287214b8);
GitHub issue #1433 is simply stale (still open, but its concern was already addressed in code)
rather than describing a live defect.

## Introduced by

N/A - not applicable, no bug confirmed. (For reference/context only, not a "fix" being attributed:
the trap-instruction pattern is present in every `_noreturn` function across every arch file in the
current tree, suggesting it was part of the original design rather than a later patch - not
separately bisected since there's no bug to date.)

## The fix

N/A - no bug found, nothing to fix. Given the underlying concern in issue #1433 is already resolved
in code, the accurate follow-up here is a documentation/triage action on the issue itself (closing
it with a pointer to this candidate's findings), not a code change - left for whoever reads this
report to action on GitHub, since this workspace doesn't post to the tracker itself.

## What's not done / open questions

- Not tested on any architecture other than x86_64 (the only one available in this container) -
  the empirical seccomp repro is x86_64-specific (`#[cfg(target_arch = "x86_64")]` on the test, and
  the BPF program hardcodes `SYS_exit_group = 231`, x86_64's syscall number). The other 11
  architectures' trap instructions were verified by code reading only (see the first row above),
  not by an equivalent real seccomp repro on that hardware.
- Not verified whether the `libc` backend (as opposed to `linux_raw`) has an analogous concern -
  the `libc` backend calls libc's own `exit`/`_exit`/`syscall()` wrappers, which are a completely
  different code path not examined here (the `runtime` module this issue is about doesn't even
  compile under the `libc` backend, per its `#[cfg(linux_raw)]` gate).
- This candidate resolves the *specific* code paths issue #1433 links to (`x86_64.rs`'s
  `syscall1_noreturn`, `runtime/syscalls.rs`'s `exit_thread`); it does not re-audit every other
  syscall wrapper in the crate for a similar "assumes-it-never-returns" pattern outside the
  `_noreturn`-suffixed functions specifically named by the issue.
