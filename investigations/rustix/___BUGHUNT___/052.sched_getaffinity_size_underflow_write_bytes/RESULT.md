# sched_getaffinity_size_underflow_write_bytes

`src/backend/linux_raw/thread/syscalls.rs`'s `sched_getaffinity` passes `size_of::<RawCpuSet>()`
as the requested size to the raw `__NR_sched_getaffinity` syscall, gets back `size` (the byte count
the kernel reports it wrote), then computes `rest = bytes.wrapping_add(size)` and
`rest.write_bytes(0, core::mem::size_of::<RawCpuSet>() - size)` with no explicit check that
`size <= size_of::<RawCpuSet>()` before the subtraction. If the kernel ever returned a `size`
greater than the requested length, this `usize` subtraction would underflow and `write_bytes`
would be called with an enormous byte count - an unbounded out-of-bounds write. Flagged as the
same "trust the kernel syscall contract" category as candidates 011 and 027 in this workspace, but
with a more severe failure mode if the trust turned out to be misplaced.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Kernel source review of `sys_sched_getaffinity`'s actual implementation, across the entire history of the syscall (current `torvalds/linux` master, plus tags v6.6, v5.4, v4.19, v4.4, v3.10, v3.0, v2.6.32 - spanning every maintained kernel line and back to the syscall's early-2.6-era shape) | Fetched each version's `kernel/sched/syscalls.c` (or `kernel/sched/core.c` / `kernel/sched.c` for older tags) directly from GitHub and read the actual `SYSCALL_DEFINE3(sched_getaffinity, ...)` body | **The kernel's own contract guarantees the returned size can never exceed the caller-requested `len`, in every version checked.** Current kernels (v3.0 onward, confirmed through today's master) compute `unsigned int retlen = min(len, cpumask_size())` and return `retlen` - a `min()` against the caller's own `len`, so the return value is *structurally* bounded by what was requested. Older kernels (v2.6.32 and earlier) instead reject the call outright with `-EINVAL` whenever `len < cpumask_size()` (`if (len < cpumask_size()) return -EINVAL;`), before ever computing a return value - so the unconditionally-returned `cpumask_size()` is also always `<= len` in that version. Both mechanisms independently enforce the exact invariant `rest.write_bytes`'s subtraction depends on; there is no kernel version in this history with a gap between the two mechanisms that could return more than `len`. |
| Real end-to-end execution: temporarily instrumented the actual `sched_getaffinity` function (in this worktree only, reverted before finishing) with an `eprintln!` printing the kernel-reported `size` vs. the requested `size_of::<RawCpuSet>()`, built a `cargo` example calling rustix's real public `rustix::thread::sched_getaffinity` API, and ran it on this container's real kernel (`7.0.0-34-generic`, x86_64, 24 logical CPUs) for: the current thread, `pid 1` (init), and again immediately after narrowing the current thread's own affinity down to 1 CPU via a real `sched_setaffinity` call | `cargo run --example getaffinity_probe --features="std,thread,process"`, 3 real calls in one run | All three real syscalls returned `size=8` (one 64-bit word, enough for the 24 active CPUs actually present) against a requested/allocated `size_of::<RawCpuSet>()=128` - i.e. `size` was always strictly less than the buffer, consistent with the kernel contract confirmed above. Full output saved in `output.txt`. |

No live/partial signal appeared in either approach (both are clean, well-explained negatives), so
the investigation stopped at 2 of the allowed 3 escalating approaches rather than reaching for a
third (e.g. fault-injecting a larger-than-requested return value via `ptrace` to force the
underflow synthetically) - doing so would only demonstrate what a hand-faked, non-existent kernel
behavior causes, not a real reproduction; see "Status" below.

## Status

**Not reproducible.** The underflow this candidate describes requires the real
`__NR_sched_getaffinity` syscall to return a `size` larger than the `len` rustix itself requested
(`size_of::<RawCpuSet>()`, which is also the exact size of the destination buffer). Every
version of the Linux kernel's own implementation of this syscall, from the early-2.6 era through
today's `torvalds/linux` master, independently and structurally prevents this: either by capping
the returned value with `min(len, cpumask_size())`, or (in older kernels, before that `min()` was
introduced) by rejecting the call with `-EINVAL` whenever `len` is too small to hold
`cpumask_size()` bytes in the first place, before any value is ever returned. A real kernel
genuinely cannot produce the input this bug needs - this isn't a case where a hand-authored stand-in
for the kernel would be needed to prove it's reachable (CONFIRMATION.md section 11's exception),
it's that the premise itself (the kernel violating this documented, independently-reconfirmed
return-value contract) does not occur on any real, currently-relevant Linux kernel. The real
execution test corroborates this directly: the real kernel on this machine always returns a
`size` well under the 128-byte buffer.

## Root cause (why the suspect is safe)

The `size <= size_of::<RawCpuSet>()` invariant that `write_bytes`'s subtraction silently depends on
is enforced entirely on the kernel side of the syscall boundary, not by any check in rustix's own
code - but it genuinely holds there, by construction, for every kernel version checked (see table
above). This is architecturally identical to candidate 011's finding (`errno.rs`'s
`debug_assert!`+`assume()` pair trusting the syscall return-value range): rustix's `linux_raw`
backend's whole design is built on trusting documented raw-syscall contracts rather than redundantly
re-validating every kernel-controlled value in userspace. The one way this invariant could
realistically break is a non-standard or deliberately broken kernel (not a stock Linux kernel
following the documented ABI) - out of scope for what this crate claims to support, and not
something any kernel-version check in this session's audit found evidence of.

**Note for a future session**: this candidate does not rule out the identical pattern anywhere else
in the crate that also post-processes a kernel-reported size with an unchecked subtraction (see
"What's not done" below) - only this specific `sched_getaffinity` call site was investigated this
session.

## The fix

No fix applied - nothing to fix; a defensive `size = size.min(size_of::<RawCpuSet>())` clamp before
the subtraction would be a free, harmless belt-and-suspenders addition (it would turn an unbounded
OOB write into, at worst, a silently-too-small zero-fill if the premise were somehow ever violated
by a non-conforming kernel), but there is no demonstrated bug it fixes against any real kernel in
this crate's supported range. Not applied, since nothing here was confirmed broken.

- Project test suite: not run against any patch (no patch was made). For reference, this worktree's
  baseline `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast` run: 411
  passed, 1 failed (`ioctl::test_int_setter`, pre-existing/unrelated - needs `/dev/net/tun`, not
  present in this sandbox container; already documented in `projects/rustix.md` and reproduced on
  the untouched tree by prior candidates, e.g. 027).

## What's not done / open questions

- Other call sites in the crate with the same shape (a kernel-reported size subtracted from a fixed
  requested/allocated size with no explicit clamp) were not swept this session - e.g.
  `getrandom`-adjacent code or other `*_getaffinity`-like "partial write, zero the rest" patterns,
  if any exist elsewhere in `linux_raw`. Worth a dedicated sweep (`grep -n "write_bytes(0,"` across
  `src/backend/linux_raw/`) in a future session, same spirit as candidate 011's own suggestion to
  check other "trust the syscall return value" sites.
- Not checked: the 32-bit-on-64-bit-kernel compat syscall path (`compat_sys_sched_getaffinity`, for
  a 32-bit rustix build running under a 64-bit kernel) - the generic kernel source review above
  only directly covers the native (non-compat) syscall entry point; the compat entry point
  presumably shares the same underlying `cpumask_size()`/`min()` logic (it's a thin pointer-marshalling
  wrapper around the same scheduler code, not a separate reimplementation), but this wasn't
  independently confirmed by reading the compat wrapper's own source this session.
- The historical-kernel-version review (`v2.6.32` through current master) does not constitute
  running those older kernels for real; it is based entirely on reading each version's own source
  as published on GitHub, not an executed test against an actual v2.6.32/v3.0/etc. kernel binary.
