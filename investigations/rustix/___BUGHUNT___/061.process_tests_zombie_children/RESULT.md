# process_tests_zombie_children

`clippy::zombie_processes` flags four spots in `tests/process/wait.rs` (lines 14, 41, 69, 104 -
the `Command::new("yes").spawn()` call inside each of `test_waitpid_none`, `test_waitpid_some`,
`test_waitpgid`, `test_waitid`): the `std::process::Child` handle returned by `spawn()` is never
`.wait()`-ed/`.try_wait()`-ed, which clippy's lint description says "might leave behind zombie
processes". Each of these tests does reap the same pid, but through `rustix::process::waitpid`/
`waitpgid`/`waitid` (the very functions under test) rather than through `Child::wait()` - so the
question worth actually checking is whether that different-API reap genuinely avoids a real
zombie at the OS level, or whether clippy's concern is real here too.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo clippy -W clippy::zombie_processes` against the real test file | `cargo clippy --test process --features process,fs -- -W clippy::zombie_processes`, stable toolchain (clippy 0.1.98) | Confirms the lint fires at all 4 locations named in the candidate (wait.rs:14, 41, 69, 104), one per test function - see `output.txt` Approach 1. |
| Direct instrumentation of the real test file (`tests/process/wait.rs`), temporarily added then reverted (`git checkout --`) | A one-off `eprintln!` + `/proc/<pid>/stat` read inserted at the end of `test_waitpid_none`, right before `child` (the never-`.wait()`-ed `std::process::Child`) goes out of scope and is dropped; run via `cargo test --test process --features process,fs wait::test_waitpid_none -- --nocapture` | The real pid spawned by the real test shows `/proc/<pid>/stat` = `None` (no such directory - process fully gone from the process table) at that point, i.e. `rustix::process::waitpid` already fully reaped it a step earlier in the same test. No zombie exists by the time `Child`'s `Drop` runs. See `output.txt` Approach 2. Working tree confirmed clean (`git status --porcelain` empty) immediately after reverting. |
| Standalone `repro/` mirroring `test_waitpid_none`'s exact sequence (spawn -> SIGSTOP -> `rustix::process::waitpid` -> SIGKILL -> `rustix::process::waitpid`, `Child` never `.wait()`-ed) via rustix's public API, plus a control case that spawns+kills a child and never waits on it through *any* API, both checked via real `/proc/<pid>/stat` reads | `cargo run --release` against both a live worktree build and a pristine clone of base_commit via `just test`; repeated 10/10 runs | Case 1 (rustix reaps, `Child::wait()` never called): `/proc/<pid>/stat` = `None` every single run - no zombie. Case 2 (control, no wait call of any kind): `/proc/<pid>/stat` = `Some('Z')` every run - a real, visible zombie, confirming the control methodology itself produces a genuine zombie when one actually isn't reaped. See `output.txt` Approach 3. |

## Status

**Not reproducible** (as an actual zombie-process risk - the clippy lint itself does genuinely
fire, confirmed above, but it does not correspond to a real lingering zombie process in any of
the 3 approaches tried, each covering the same question from a different angle: static lint
confirmation, live instrumentation of the real test, and a standalone mirror + explicit control).
All 3 approaches agree and were each run to a repeatable result (10/10 for the standalone repro;
the live-instrumentation approach was only run once against the real test file, by design, since
it required a temporary source edit that had to be reverted immediately - but it corroborates
the standalone repro's own 10/10 result on the identical code path).

## Root cause

Not a bug: `clippy::zombie_processes` is a purely syntactic lint - it flags any `Command::spawn()`
result that's never consumed by `std::process::Child::wait()`/`try_wait()`/`wait_with_output()`
reachable from every path, with no awareness of what else might reap that same OS pid through a
different API. In all four flagged tests, the spawned `yes` child's pid is in fact reaped - by
`rustix::process::waitpid`/`waitpgid`/`waitid`, which issue the real `wait4(2)`/`waitid(2)` syscall
against that exact pid (these functions are themselves the subject under test, not incidental
plumbing). A `wait4`/`waitid` syscall that successfully retrieves a child's exit/stop status
reaps it at the kernel level regardless of which userspace wrapper issued the call - `std::
process::Child::wait()` is just one such wrapper, not the only one capable of this. So by the time
each test function returns and its `Child` value is dropped (which does nothing on Unix - `Child`
has no `Drop` impl that waits), the pid in question is already gone from the process table, not
sitting around as a zombie. This was verified directly by reading the real pid's `/proc/<pid>/stat`
both from inside the real test (Approach 2) and from a standalone mirror with an explicit "really
would be a zombie" control case for comparison (Approach 3).

## Introduced by

N/A - no bug found, so there's nothing to attribute to an introducing commit. For reference only:
the four tests predate this session's checkout and the overall `#[serial]`-reap pattern looks to
be original test design (reaping through the function under test is the point of a wait()-API
test file), not a later regression.

## The fix

N/A - no bug found, nothing to fix. The underlying clippy observation is still accurate as a
*static* fact (none of the 4 `Child` values are ever `.wait()`-ed directly) and could be silenced
with `#[allow(clippy::zombie_processes)]` on the 4 functions (or on the module) with a comment
explaining the reap-via-rustix pattern, purely to stop the lint firing in CI if that's desired -
but that's a lint-suppression/documentation choice for the maintainers, not a correctness fix,
since no actual zombie-process or CI-flakiness defect was found.

- Project test suite: not run as a before/after pair, since no fix was made (nothing to verify
  against) - see CONFIRMATION.md/SKILL.md's step 7, which is scoped to verifying a bounded fix.
  The specific test file itself (`tests/process/wait.rs`) was run multiple times during
  investigation (Approaches 2 and the live subset of Approach 1's build) and passed every time
  (4/4 tests `ok` in the full-file run used for Approach 1's clippy build, 1/1 in the targeted
  `test_waitpid_none` runs for Approach 2).

## What's not done / open questions

- Only `test_waitpid_none` was directly instrumented (Approach 2); `test_waitpid_some`,
  `test_waitpgid`, and `test_waitid` were not individually re-verified with the same live /proc
  probe - their reaping pattern is structurally identical (signal, then a `rustix::process::wait*`
  call that performs the real reap, with no `NOWAIT`/`NOHANG` flag on the *final* such call in any
  of them), so the standalone repro (Approach 3, which mirrors this exact pattern) and the shared
  root cause are expected to cover them too, but this wasn't independently confirmed pid-by-pid for
  the other three.
- A genuinely different leak scenario was noticed but not investigated this session: if one of the
  `assert_eq!`/`assert!` calls *inside* one of these four tests panics midway (e.g. right after the
  SIGSTOP-reaping `waitpid` call but before the SIGKILL+final-reaping `waitpid` call), the spawned
  child is left alive (stopped, not yet a zombie, but un-reaped and un-killed) when `Child` is
  dropped during unwinding - since the Rust test harness catches the panic and keeps running
  further tests in the same process by default, this could plausibly leave an orphaned stopped `yes`
  process behind after a *failing* test run, which is a real (if narrow, failure-path-only) leak
  distinct from the normal-path question this candidate actually checked. Added to `IDEAS.md` as a
  separate, not-yet-investigated suspect.
- Not tested on any OS other than Linux (the only platform available in this container); the
  `/proc/<pid>/stat` inspection technique is Linux-specific, though the underlying `wait4`/`waitid`
  reaping semantics being relied on are POSIX-standard.
