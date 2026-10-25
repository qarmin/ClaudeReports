# pid_from_child_narrowing_cast

`Pid::from_child` (`src/pid.rs:68-73`) does `let id = child.id(); unsafe {
Self::from_raw_unchecked(id as i32) }`, casting `std::process::Child::id(): u32` to `i32` with no
range check, then relies on `from_raw_unchecked`'s own `debug_assert!(raw > 0)` (`src/pid.rs:61`) -
which is stricter than that function's documented `# Safety` contract ("the caller must guarantee
`raw` is non-zero", and `from_raw`'s own doc wording suggests negative values are tolerated,
merely "unexpected"), and is compiled out entirely in release builds. Looked suspicious as an
unchecked narrowing cast on an OS-supplied value at a public API boundary.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end exercise of the actual public API | debug-assertions build (`cargo run --example`, then again via `repro/`'s release build against a fresh clone), 2000 + 2000 real `std::process::Command` children each run, asserting `Pid::from_child(&child).as_raw_pid() > 0` and matches `child.id()` every iteration | No failure in 4000 total real spawns; observed real OS-assigned ids stayed in a tight range (~310k-319k), ~2.147 billion away from the `i32` overflow boundary |
| Cross-platform kernel/OS source research (the hard ceiling every platform `src/pid.rs`'s module compiles on actually enforces for a real pid, not just this container's current setting) | Read Linux `include/linux/threads.h`, FreeBSD/macOS(xnu)/OpenBSD/NetBSD `sys/proc.h`/`proc_internal.h`, illumos `pid_allocate()`; also checked this container's own `/proc/sys/kernel/pid_max` (read-only, already at the kernel's hard max) | Linux: 4194304 (64-bit `PID_MAX_LIMIT`); FreeBSD/macOS/OpenBSD: 99999; NetBSD: 30000; illumos: a bounded `[minpid, maxpid)` cycling pool. All many orders of magnitude below 2^31 |
| Manual boundary-value construction on the unsafe primitive directly (root-cause mechanism only, not a reproduction - see "Fully reproducible" below) | Hand-fabricated `u32 = 3_000_000_000` (what a real OS pid allocator would, but structurally cannot, ever produce) fed to the same `Pid::from_raw_unchecked` call `from_child` makes, in both debug and release builds | Debug: panics at `src/pid.rs:61`, `assertion failed: raw > 0`. Release: returns normally with `as_raw_pid() == -1294967296`. Confirms the debug/release split described by the candidate exists *in isolation* - but this input is never genuinely produced by `child.id()` on any platform this code targets |

## Status

**Not reproducible.** `Pid::from_child`'s cast genuinely cannot misbehave via the real end-to-end
path (a real `std::process::Child` spawned by the OS, then handed to the real public
`Pid::from_child`) on any of the platforms `src/pid.rs`'s module is compiled for
(`#[cfg(not(any(windows, target_os = "wasi")))]` at `src/lib.rs:368` - Windows and WASI never
compile this code at all). Every Unix-like OS rustix supports enforces its own hard ceiling on real
process IDs, and every one of them sits far below the `2^31` boundary at which `id as i32` would
become non-positive:

- Linux: `PID_MAX_LIMIT` = 4,194,304 (64-bit), smaller still on 32-bit. This container's own
  `/proc/sys/kernel/pid_max` already reads 4194304 and refuses writes (read-only filesystem) -
  i.e. this is the kernel's actual hard ceiling, not merely today's sysctl setting.
- FreeBSD / macOS (xnu) / OpenBSD: `PID_MAX` = 99999 (hardcoded, "to keep it in 5 columns in ps").
- NetBSD: `PID_MAX` = 30000.
- illumos: `pid_allocate()` cycles a bounded `[minpid, maxpid)` pool rather than using the raw
  32-bit range.

2000 real spawned children in each of two independent runs (a worktree-local debug build and a
fresh-clone release build via `repro/`'s own `just test`) all produced ids in a narrow ~310k-319k
range, consistent with these ceilings and roughly 2.147 billion away from the overflow boundary.
**4/4 runs** (2 approaches x debug/release each) showed no failure; this is a structural ceiling,
not a probabilistic absence.

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed) - but noted explicitly per
CONFIRMATION.md section 11: `repro/`'s `boundary_mechanism` binary, which does show the
debug-panic/release-silent-negative-Pid split described by the candidate, does so only by
hand-fabricating the `u32` value a real OS pid allocator would need to produce and is structurally
incapable of producing. That binary is kept in `repro/` purely to document the mechanism (and is
clearly labeled as such in its own source comments and in the justfile), not offered as a
confirmation of the real bug - only `repro/`'s `repro` binary (real spawned children, real public
`Pid::from_child`) is the actual investigation result.

## Class

N/A (Not reproducible).

## Root cause

Not a reachable bug: `from_child`'s unchecked cast and `from_raw_unchecked`'s stricter-than-the-doc
`debug_assert!(raw > 0)` are both textually real, but every real process ID that `child.id()` can
ever return on a platform where this code compiles is bounded by the host OS's own PID allocator
far below the point where the `u32 -> i32` cast could turn non-positive. The debug_assert/doc
mismatch between `from_raw` (`debug_assert!(raw >= 0)`, doc says negative is tolerated but
"unexpected") and `from_raw_unchecked` (`debug_assert!(raw > 0)`, doc says only "non-zero" is
required) is a real, independently-observable inconsistency in the source (demonstrated in
isolation by `repro/`'s `boundary_mechanism` binary) - but it has no demonstrated real trigger
through `Pid::from_child` specifically, because no genuine caller of that function can ever supply
an `id()` anywhere near the boundary where the mismatch would matter.

## Introduced by

N/A (Not reproducible - no bug to date). For reference only: `Pid::from_child`'s current shape
traces back to f9a321ef "Factor out code used by multiple modules." (earliest commit `git log -S
"fn from_child"` finds touching `src/pid.rs`); not treated as an "introducing" commit since no bug
was confirmed.

## The fix

No fix applied - nothing demonstrated broken via the real API. A defensive `u32::try_from(id)`
(falling back to `from_raw_unchecked(i32::MAX)` or panicking with a clearer message) would be
free, harmless belt-and-suspenders, and would also resolve the `from_raw`/`from_raw_unchecked`
debug_assert-vs-doc inconsistency noted above if the project wants to tighten that up regardless
of today's unreachability - but there is no demonstrated bug it fixes against any real OS this
crate targets, so nothing was patched this session.

- Project test suite: not run against any patch (no patch was made). For reference, this
  worktree's own `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`
  baseline (unmodified source, same baseline already documented by prior candidates e.g. 027/052):
  411 passed, 1 failed (`ioctl::test_int_setter`, pre-existing/unrelated - needs `/dev/net/tun`,
  not present in this sandbox container).

## What's not done / open questions

- Redox and Haiku (both listed as CI targets in `Cargo.toml` and both compile `src/pid.rs`'s
  module per its `cfg`) were not independently source-verified for their own PID ceilings this
  session - only Linux, the BSD family, macOS, and illumos were. Nothing suggests either would
  differ from the pattern seen everywhere else (a bounded, small-integer pid space), but this is
  an honest gap, not a verified fact.
- The debug_assert/doc mismatch between `from_raw` (`debug_assert!(raw >= 0)`) and
  `from_raw_unchecked` (`debug_assert!(raw > 0)`) noted in "Root cause" above is real and visible
  in the source independent of this candidate's specific `from_child` call site - worth a
  maintainer decision on whether to tighten/relax one to match the other, even though no reachable
  bug was found through `from_child` this session. Not raised as a new candidate since it's the
  same code already investigated here, not a new suspect.
- Did not check whether any *other* call site in the crate (beyond `from_child`) ever casts a
  `u32`/`usize` of genuinely unbounded (non-OS-pid) provenance into `from_raw_unchecked` - this
  session only covers the one call site named in the candidate.
