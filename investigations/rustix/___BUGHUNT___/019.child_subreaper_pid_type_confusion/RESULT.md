# child_subreaper_pid_type_confusion

`src/process/prctl.rs`'s `child_subreaper()`/`set_child_subreaper()` wrap
`PR_GET_CHILD_SUBREAPER`/`PR_SET_CHILD_SUBREAPER` with an `Option<Pid>` signature, which implies
the kernel tracks *which process* is the child subreaper. It doesn't - this looked suspicious
because the man page and kernel source only ever describe this as a boolean per-process attribute.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read kernel source (`kernel/sys.c`, `is_child_subreaper`) and `prctl(2)` man page | n/a | Confirmed the kernel stores and returns only a boolean (0/1), never a pid, for both the get and set operations |
| Real two-call round-trip repro: `set_child_subreaper(Some(arbitrary_pid))` then `child_subreaper()` | debug build, real `prctl()` syscalls via the crate's own public API, 5/5 runs | Reproduced every run: the returned "pid" is always `Pid::from_raw(1)` when set, never the pid that was actually passed to `set_child_subreaper` |

## Status

**Confirmed**, reproduces 5/5 runs (debug build, real `prctl()` syscalls, no flakiness - this is a
pure type-level/logic bug, not a layout- or timing-dependent one).

**Fully reproducible**: Yes. `repro/src/bin/before.rs` calls only the crate's real public API
(`set_child_subreaper`/`child_subreaper`) and issues real `prctl()` syscalls against the actual
calling process - there is no second participant involved in this bug (it's a single-process
API/kernel-contract mismatch), so `CONFIRMATION.md` section 11 doesn't apply here.

## Class

logic (API/type-safety: a kernel boolean is round-tripped through a `Pid`-shaped type)

## Root cause

`PR_SET_CHILD_SUBREAPER`'s kernel implementation (`kernel/sys.c`) does
`me->signal->is_child_subreaper = !!arg2;` - it only ever looks at whether `arg2` is zero or
nonzero, discarding everything else about the value. `PR_GET_CHILD_SUBREAPER` symmetrically writes
back only `0` or `1` through the `arg2` out-pointer. There is no pid stored or retrievable at all;
"child subreaper" is a per-process boolean flag (see `prctl(2)`: "the one that is the subreaper
will not be... a process group leader" etc. - it never describes a target pid).

rustix's wrapper nonetheless modeled this as `Option<Pid>`:

- `set_child_subreaper(pid: Option<Pid>)` passes `pid.map_or(0, |p| p.as_raw_nonzero().get())` as
  `arg2` - any nonzero pid value collapses to the same kernel-side effect (`is_child_subreaper =
  true`), silently discarding the specific pid the caller thought they were recording.
- `child_subreaper() -> io::Result<Option<Pid>>` then does `Pid::from_raw(r as RawPid)` on the
  boolean `r` the kernel returns - when the flag is set, `r == 1`, so this always yields
  `Some(Pid::from_raw(1))`, i.e. **PID 1 (init)** - a real, existing process that has nothing to
  do with the caller's own child-subreaper status. A caller who (reasonably, given the type)
  compared this against "my own pid" or "the pid I set" to confirm the setting took effect would
  always get a mismatch, and worse, might mistakenly treat "returns `Some(pid_of_init)`" as
  meaningful information about which process is acting as the subreaper.

## Introduced by

51040e35 "Add support for prctl()." (2022-09-04)

## The fix

Changed both functions to a plain `bool`, matching the kernel's actual contract exactly:

```rust
pub fn child_subreaper() -> io::Result<bool> {
    unsafe {
        let r = prctl_get_at_arg2_optional::<c_uint>(PR_GET_CHILD_SUBREAPER)?;
        Ok(r != 0)
    }
}

pub fn set_child_subreaper(subreaper: bool) -> io::Result<()> {
    unsafe { prctl_2args(PR_SET_CHILD_SUBREAPER, subreaper as usize as *mut _) }.map(|_r| ())
}
```

This is a breaking API change (`Option<Pid>` -> `bool` in both directions), appropriate for a
semver-major bump rather than a patch release - noted here as a fact, not a priority judgment (per
this skill's "no severity labels" rule).

Verified: `repro/src/bin/after.rs` (same round-trip shape as `before.rs`, using the new `bool` API)
round-trips `true`/`false` correctly, 3/3 runs.

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc"` -
  70 passed / 0 failed before -> 70 passed / 0 failed after in the `io` test binary, and no change
  in any other test binary's pass/fail counts (21 test binaries total, one pre-existing unrelated
  failure in both before and after: `ioctl::test_int_setter`, which fails in this sandbox because
  `/dev/net/tun` doesn't exist here - documented in `___BUGHUNT___/README.md`, unrelated to this
  change). No test in the suite exercises `child_subreaper`/`set_child_subreaper` directly in
  either state.

## What's not done / open questions

- This is a breaking API change; applying it for real requires a semver-major release, a decision
  for the maintainers, not something to silently ship behind a patch version. The patch itself
  compiles and passes the full test suite with no required changes elsewhere in the crate.
- No other `prctl` wrapper in this file was found to have the same `Option<Pid>`-for-a-boolean
  shape (the rest of the file's `Option<Pid>`-returning functions, e.g. the real process-group
  ones, correspond to kernel calls that do return a real pid) - not an exhaustive sweep of every
  other `prctl` subcommand across the whole crate, just this immediate file.
- A session on 2026-09-18 found and fixed this but was interrupted before writing it up (fix was
  left applied-but-uncommitted in the working tree, no `RESULT.md`/`SUMMARY.md` row); this
  candidate folder was completed by re-deriving the before/after repro from the leftover
  `_scratch_019/` on 2026-10-01, re-verifying both states and the test suite from scratch rather
  than trusting the earlier session's own (unwritten) claim.
