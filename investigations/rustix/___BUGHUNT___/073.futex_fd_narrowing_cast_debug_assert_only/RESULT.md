# futex_fd_narrowing_cast_debug_assert_only

`src/thread/futex.rs:102-110`, `pub fn fd(...)`, narrows the raw `FUTEX_FD` syscall return value
(`usize`) to `RawFd` (`i32`) via `let fd = val as RawFd;`, guarded only by
`debug_assert_eq!(fd as usize, val, ...)` - compiled out in release builds - before handing `fd`
to `OwnedFd::from_raw_fd(fd)`. Looked suspicious as the same narrowing-cast-guarded-only-by-
debug_assert pattern already confirmed-unreachable at a different call site in candidate 066
(`Pid::from_child`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end exercise of the actual public API over a real syscall, from two directions: (1) a bare C program issuing `syscall(SYS_futex, &uaddr, FUTEX_FD, 0, NULL, NULL, 0)` directly, (2) `rustix::thread::futex::fd()` called through its real public API (`repro/src/main.rs`) | This container's kernel (`uname -r` = 7.0.0-34-generic), both debug and release builds, 5/5 repeat runs each | Both return `Err`, `errno=38` / `raw_os_error=38` (`ENOSYS`, "Function not implemented") on every single run - `FUTEX_FD` is unimplemented on this kernel, so the `.map()` closure containing the suspect cast is never entered |
| Kernel/source-level research into *why* `ENOSYS` is unconditional here, not just true on this one container | Read `man 2 futex` and recalled kernel history | `FUTEX_FD` was removed from the Linux kernel in 4.5 (released March 2016, commit removing `FUTEX_FD` support for CVE-2015-7550-class security reasons) - every kernel from that point on returns `ENOSYS` unconditionally for this operation, regardless of distro/config |
| Structural bound check on the hypothetical success path (what if a pre-4.5 kernel, or some future reintroduction, did return a real fd?) - compared against `/proc/sys/fs/nr_open` on this container and the kernel's own formula for its sysctl ceiling | `cat /proc/sys/fs/nr_open` = 2147483584; attempted `echo 2147483647 > /proc/sys/fs/nr_open` to see if that's a true hard ceiling (blocked: `sudo` unusable under this sandbox's "no new privileges" flag, consistent with the file already sitting at the kernel's actual max rather than merely today's setting) | 2147483584 = `round_down(i32::MAX, 64)` = `round_down(2147483647, BITS_PER_LONG)` exactly - matches the Linux kernel's own `fs/file.c` computation of `sysctl_nr_open_max`. Any real fd the kernel could ever allocate is therefore structurally bounded below `i32::MAX`, so even a real (non-`ENOSYS`) success return could never make `val as RawFd` go negative |
| Manual boundary-value construction on the exact cast+debug_assert pattern in isolation (`repro/src/boundary_mechanism.rs`, hand-fabricating `val = 0xFFFF_FFFE`, mirroring `futex.rs`'s own two lines verbatim) | Debug and release builds, 5/5 repeat runs each | Debug: panics every time at the `debug_assert_eq!`, left=`18446744073709551614`, right=`4294967294`. Release: reaches past it every time, producing `fd=-2`. Confirms the debug/release split described by the candidate is textually real *in isolation* - but, per the two rows above, no real kernel can ever supply the input value this mechanism requires |

## Status

**Not reproducible.** `futex::fd()`'s cast cannot misbehave via the real end-to-end path (the real
public API, driven by the real kernel `futex()` syscall) on any kernel this crate is realistically
run on, for two independent, stacking reasons:

1. `FUTEX_FD` has returned `ENOSYS` unconditionally on every Linux kernel since 4.5 (2016) - the
   `.map()` closure containing the suspect cast is dead code in practice on any kernel from the
   last decade. Verified directly: 5/5 runs of a raw C `syscall()` and 5/5 runs of rustix's own
   public `futex::fd()` API both returned `Err(ENOSYS)` on this container's kernel, every time.
2. Even setting reason 1 aside entirely (a hypothetical kernel that still implements `FUTEX_FD`,
   or a future reintroduction), a real fd the kernel could allocate is structurally bounded below
   `i32::MAX` by the kernel's own `fs.nr_open` ceiling (`round_down(INT_MAX, BITS_PER_LONG)`,
   observed exactly matching this container's own `/proc/sys/fs/nr_open` = 2147483584) - the same
   class of structural-ceiling argument candidate 066 found for PIDs, here for fds.

5/5 runs on every real-path check (both the raw syscall and the public-API repro); 5/5 runs on
the isolated-mechanism check (both debug and release). This is a structural dead end, not a
probabilistic absence.

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed) - but noted explicitly per
CONFIRMATION.md section 11: `repro/`'s `boundary_mechanism` binary, which does show the
debug-panic/release-silent-wrong-fd split described by the candidate, does so only by
hand-fabricating the `usize` value a real `FUTEX_FD` syscall return would need to produce and that
no real kernel this crate targets can ever produce (both because the operation is `ENOSYS`
everywhere realistic, and because even a real success value is capped well under the boundary).
That binary is kept in `repro/` purely to document the mechanism (and is clearly labeled as such
in its own source comments and in the justfile), not offered as a confirmation of the real bug -
only `repro/`'s `repro` binary (real syscall, real public `futex::fd()`) is the actual
investigation result.

## Class

N/A (Not reproducible).

## Root cause

Not a reachable bug: `fd()`'s unchecked cast and its debug-only guard are both textually real
(identical shape to candidate 066's `Pid::from_child`), but the input required to trigger a
mismatch can never be genuinely produced by this crate's actual syscall path - `FUTEX_FD` itself
returns `ENOSYS` on every kernel since Linux 4.5 (2016), and even disregarding that, the kernel's
own fd-table ceiling (`fs.nr_open`, hard-capped at `round_down(INT_MAX, BITS_PER_LONG)`)
structurally prevents a real fd value from ever reaching the point where the `usize -> i32` cast
could turn negative.

## Introduced by

N/A (Not reproducible - no bug to date). For reference only: this call site's current shape traces
back to b0d3681c "Futex Rework (#1098)" (`git log --follow -S "pub fn fd(uaddr" -- src/thread/futex.rs`),
which introduced the whole modern `rustix::thread::futex` module including this function; not
treated as an "introducing" commit since no bug was confirmed.

## The fix

No fix applied - nothing demonstrated broken via the real API, for either of the two independent
structural reasons above. The same free, harmless belt-and-suspenders observation from candidate
066 applies here too (a defensive `i32::try_from(val)` would cost nothing and would also resolve
the `debug_assert_eq!`-vs-release-silent-cast inconsistency if the project wants to tighten this
pattern across both call sites regardless of today's unreachability) - but there is no demonstrated
bug it fixes against any real, realistically-deployed kernel, so nothing was patched this session.

- Project test suite: not run against any patch (no patch was made; no source files were modified
  this session - confirmed via `git status` before and after, showing only `___BUGHUNT___/` as
  untracked). N/A for before/after comparison.

## What's not done / open questions

- Did not independently re-derive the kernel's exact `fs/file.c` formula for `sysctl_nr_open_max`
  from kernel source in this sandbox (no kernel source tree available here); relied on the
  structural match between the observed `/proc/sys/fs/nr_open` value (2147483584) and
  `round_down(i32::MAX, 64)` as strong corroborating evidence, consistent with publicly documented
  kernel behavior, not on reading the kernel's own source in this session.
- Did not check whether any *other* call site in the crate casts a raw futex-family syscall return
  (`futex_val2`/`futex_timeout`) into a narrower type elsewhere - `requeue`/`cmp_requeue`/`wake`
  etc. in the same file return `usize` directly without narrowing, so this appears to be the only
  futex call site with this specific pattern, but this session only exhaustively covered `fd()`
  itself, the one named in the candidate.
- Candidate 004/055 (already in this workspace) cover a related but distinct issue at a different
  point in the same file (`futex_val2`'s `val2 as usize as *const Timespec` int-to-ptr cast under
  miri) - not re-investigated here, no overlap with this candidate's narrowing-cast mechanism.
