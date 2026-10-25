# Repro for candidate 047 (vdso_sym_resolution_hard_assert_64bit)

The affected code (`vdso_wrappers.rs::init()`, `src/backend/linux_raw/vdso_wrappers.rs`)
is private, non-parameterized, and always calls the real, `pub(super)`
`vdso::Vdso::new()` (which reads the real kernel-supplied vDSO via the
process's own `AT_SYSINFO_EHDR` auxv entry) - there is no public entry point
and no way to hand it a different vDSO image from outside the crate. The
untrusted input this bug is about (a vDSO that's mapped but genuinely
doesn't export the expected symbol) is a single externally-produced artifact
(like a malformed file - see `CONFIRMATION.md` section 11), not a live
second participant.

Getting a *genuine* kernel to supply such a vDSO was attempted this session:
gVisor's `ptrace` platform needs `CAP_SYS_PTRACE` (absent from this sandbox's
capability bounding set - `CapBnd` in `/proc/self/status` is `00000000000000cb`,
no ptrace bit) and its KVM platform needs `/dev/kvm` (not present either);
both are infeasible in this container. So, like candidate 036, this repro is
a real, in-tree `#[cfg(test)]` unit test exercising the real, unmodified
parsing code with a hand-built minimal vDSO standing in for the kernel's own
ELF image - not the logic, not the entry point, not the caller.

## Why this repro has two separate pieces (unlike candidate 036's one)

Candidate 036's bug and fix both lived inside one already-`pub(super)`,
directly-callable function (`Vdso::sym()`), so the *same* test, calling that
one function, naturally changed outcome once `patch.diff` was applied on
top. Candidate 047's bug lives in *inline*, non-parameterized code inside
`init()` (not itself an independently callable unit), and `Vdso`'s fields are
private to `vdso.rs` - `vdso_wrappers.rs` can't construct one to drive
`init()` with crafted data directly, even though it's in the same `linux_raw`
visibility scope.

So:

- **`add_test.patch`** (this folder) adds exactly one test,
  `test_vdso_missing_symbol_then_hard_assert_panics`, to
  `src/backend/linux_raw/vdso.rs`'s existing `#[cfg(test)] mod tests`. It
  builds a minimal, well-formed, single-bucket classic `DT_HASH` vDSO with
  **no symbols at all** (`bucket[0] == 0`, the standard "nothing hashed to
  this bucket" terminator - not candidate 036's hash-collision-plus-overflow
  construction). Calling the real, unmodified `vdso.sym()` on it returns a
  null pointer through `sym()`'s own normal, already-correct "not found"
  path (not the `.unwrap()`-before-`assert!` path candidate 036 fixed). The
  test then replicates `init()`'s *pre-patch* `clock_gettime`,
  `target_pointer_width = "64"` logic **verbatim** (`let ok = true; if ok {
  assert!(!ptr.is_null()); }`) and shows it panics on this legitimately-null
  `ptr`. This is a frozen snapshot of the vulnerable code shape, applied to
  a *pristine* clone - it demonstrates the bug regardless of whether
  `../patch.diff` has been applied to `vdso_wrappers.rs` elsewhere in the
  tree (it never reads that file), which is why it still "fails" if run
  again after `../patch.diff` - that's expected, see the justfile.

- **`../patch.diff`** (the real fix) does two things in the same file:
  extracts the "is this symbol pointer trustworthy" decision that used to be
  duplicated (and, for several architectures, wrongly hardcoded to "always
  trust it") at three separate call sites (`clock_gettime`, `getcpu`, and
  x86's `__kernel_vsyscall`) into one small shared function,
  `vdso_sym_is_ok`, and bundles its own regression test,
  `vdso_sym_is_ok_rejects_null_and_accepts_nonnull`, that calls this *real*
  patched function directly (not a frozen copy) with a null pointer and
  confirms it's now rejected instead of blindly trusted. This is what
  genuinely demonstrates the fix landed, since - unlike the `add_test.patch`
  test above - it calls the actual code `init()` itself now calls.

## Running it

From a clone of the project at `base_commit` (see `../justfile`, which does
this automatically):

```sh
git apply /path/to/repro/add_test.patch
cargo test --lib --features time,stdio,fs backend::vdso::tests::test_vdso_missing_symbol_then_hard_assert_panics
```

- Against the unpatched tree: the test **panics**
  (`assertion failed: !ptr.is_null()` at `vdso.rs`'s copied `clock_gettime`
  branch) - this is the bug, Status: FAILED.

```sh
git apply /path/to/patch.diff
cargo test --lib --features time,stdio,fs backend::vdso_wrappers::tests::vdso_sym_is_ok_rejects_null_and_accepts_nonnull
```

- Against the tree with `../patch.diff` also applied: this *second*,
  real-code-calling test **passes** - `vdso_sym_is_ok` (the function every
  call site in `init()` now routes through) correctly rejects a null
  pointer. See `../justfile`'s `reproduce-after`.

Verified 5/5 debug-build runs and 1/1 release-build run of the bug
demonstration (deterministic - the crafted vDSO has no symbols at all, not a
fragile hash-collision construction, so there's no seed/layout dependence),
and the fix verified both via the bundled regression test and via the full
project test suite (`cargo test --lib --features time,stdio,fs`): 20 passed
/ 0 failed / 0 ignored on the pristine tree, 21 passed / 0 failed / 0
ignored with `../patch.diff` applied (the one new test). A real end-to-end
sanity check (a separate, throwaway crate depending on this tree's `rustix`
as a path dependency, calling the real public
`rustix::time::clock_gettime(ClockId::Monotonic)` and
`rustix::thread::sched_getcpu()`) was also run against the patched tree on
this host's real kernel vDSO, confirming the fix doesn't disturb the normal
(symbol-present) case: both returned real, correct-looking values.
