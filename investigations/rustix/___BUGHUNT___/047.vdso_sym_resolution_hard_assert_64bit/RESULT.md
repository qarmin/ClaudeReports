# vdso_sym_resolution_hard_assert_64bit

In `src/backend/linux_raw/vdso_wrappers.rs`'s `init()`, once `vdso::Vdso::new()` finds any mapped
vDSO, the `clock_gettime` lookup (lines 530-545 at candidate time) hardcodes
`#[cfg(target_pointer_width = "64")] let ok = true;` with no null check at all - unlike the
32-bit branches right next to it, which do `let ok = !ptr.is_null();` - and then does a real
(non-`debug_assert`) `assert!(!ptr.is_null())` on the result of `vdso.sym(...)`. The `getcpu`
lookup (lines 586-603) has the identical shape for `x86_64`/`riscv64`/`powerpc`/`powerpc64`/
`s390x`, and x86's `__kernel_vsyscall` lookup (lines 613-616) asserts unconditionally with no `ok`
variable at all. All three assume every vDSO that passes ELF-header validation necessarily exports
the exact expected symbol+version - which doesn't hold for a vDSO that's mapped but genuinely
doesn't export it (a minimal vDSO under gVisor/runsc or a similarly restricted kernel/sandbox, for
example). Flagged as a candidate distinct from candidate 036 (an `.unwrap()`-before-`assert!`
*overflow* path inside the ELF hash-table walk) - this is a *missing-symbol* path, and `sym()`'s
own normal "not found" handling (returning a null pointer) is already correct; the bug is entirely
in `init()` trusting that result unconditionally on these architectures.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `init()`'s three vDSO lookup blocks (`clock_gettime`, `getcpu`, x86 `__kernel_vsyscall`) and `Vdso::sym()`'s own "not found" path | n/a | Confirmed `sym()`'s normal miss path (nothing hashes to the queried bucket, or the chain loop exhausts without a match) returns `null_mut()` cleanly with no panic anywhere in that path - the bug is purely `init()` trusting a non-null-checked `ptr` on `target_pointer_width = "64"` architectures (`clock_gettime`) and on `x86_64`/`riscv64`/`powerpc`/`powerpc64`/`s390x` (`getcpu`), plus x86's unconditional `__kernel_vsyscall` assert |
| Attempted escalation: a genuine kernel/sandbox that maps a vDSO without the expected symbol (gVisor/runsc, qemu-user) | Checked `/proc/self/status` (`CapBnd` = `00000000000000cb`, no `CAP_SYS_PTRACE` bit - needed by gVisor's `ptrace` platform) and `/dev/kvm` (absent - needed by gVisor's KVM platform); also no root/privileged container access in this sandbox | Infeasible in this environment - documented so a future session with different sandbox capabilities doesn't need to re-check this from scratch |
| Manual boundary-value construction: a hand-built, in-process, minimal classic-`DT_HASH` `Vdso` (one bucket, `bucket[0] == 0` - a genuinely empty symbol table, not a hash collision) added as a `#[cfg(test)]` test directly in `vdso.rs` (this project's own test module), replicating `init()`'s exact pre-patch `clock_gettime`/`target_pointer_width = "64"` logic verbatim on the real (unmodified) `vdso.sym()`'s resulting null pointer | debug build, `cargo test --lib --features time,stdio,fs`, repeated 5x | Panicked 5/5 runs, deterministically, at `src/backend/linux_raw/vdso.rs:620:13`, `assertion failed: !ptr.is_null()` - exactly the claimed bug, and fully deterministic (an empty symbol table, not a seed/layout-dependent construction) |
| Same test, release build | `cargo test --release --lib --features time,stdio,fs` | Panicked 1/1 run, same location/message - confirms this is a real `assert!`, not a debug-assertion-only manifestation |
| Fix: extract the "is this symbol pointer trustworthy" decision (duplicated, and on several architectures wrongly hardcoded to "always trust it", across all three call sites) into one shared `vdso_sym_is_ok` function, call it uniformly at all three sites, bundle a regression test calling that real function directly | debug + release | Full project test suite green (20 -> 21 passed, the one new test, 0 failed either way); real end-to-end sanity check (a throwaway crate using this patched tree's real public `rustix::time::clock_gettime`/`rustix::thread::sched_getcpu`, against this host's real kernel vDSO) still resolves correctly through the vDSO fast path |

## Status

**Confirmed**. Reproduces 5/5 debug-build runs + 1/1 release-build run, fully deterministic (the
crafted vDSO has an empty symbol table by construction - `sym()`'s own normal "not found" path,
not a layout/seed-dependent overflow condition).

**Fully reproducible**: **No**, with justification. `vdso_wrappers.rs::init()` is private and
non-parameterized - it always calls the real `vdso::Vdso::new()`, which reads the process's own
real `AT_SYSINFO_EHDR` auxv entry (the real kernel's real vDSO). There is no public entry point to
hand it a different vDSO image, and `Vdso`'s own fields are private to `vdso.rs` (not even
`vdso_wrappers.rs`, despite being in the same `linux_raw` visibility scope, can construct one).
Getting a *genuine* kernel/sandbox to supply a vDSO that's mapped but missing the expected symbol
was attempted this session: gVisor's `ptrace` platform needs `CAP_SYS_PTRACE` (this container's
capability bounding set lacks it) and its KVM platform needs `/dev/kvm` (also absent) - both
infeasible here. Per `CONFIRMATION.md` section 11, this falls into the same "a single external
artifact, no live second participant" category as a malformed file handed to a parser's public
entry point - except, as with candidate 036, there's no *public* entry point to call even with
hand-constructed bytes, since the parser itself is private. See `repro/README.md` for the full
reasoning and for why this candidate's repro needed two separate test additions (one frozen
pre-patch snapshot for `reproduce-before`, one calling the real patched function for
`reproduce-after`) rather than candidate 036's single shared test.

## Class

crash

## Root cause

`vdso_wrappers.rs::init()` looks up several vDSO symbols (`clock_gettime`, `getcpu`,
`__kernel_vsyscall`) via the real, unmodified `Vdso::sym()`, which correctly returns a null
pointer when a symbol genuinely isn't present (its own "not found" path, distinct from candidate
036's overflow-before-bounds-check path). `init()` itself, however, special-cased what to do with
that result per architecture: for `clock_gettime` on every `target_pointer_width = "64"`
architecture, and for `getcpu` on `x86_64`/`riscv64`/`powerpc`/`powerpc64`/`s390x`, it hardcoded
`let ok = true;` - skipping the null check the 32-bit sibling branches already had - then
immediately did `assert!(!ptr.is_null())`, aborting the whole process if the vDSO genuinely didn't
export that symbol. x86's `__kernel_vsyscall` lookup had the same shape with no `ok` variable at
all - an unconditional `assert!(!ptr.is_null())`. On every real x86_64/aarch64/etc. Linux kernel in
ordinary use, these symbols are always present, so the assumption holds in practice - but a
restricted/sandboxed kernel's own minimal vDSO (gVisor/runsc, certain emulators) can map
successfully while still omitting one of these symbols, hitting the hard assert instead of the
syscall fallback `minimal_init()` already installed moments earlier in the same function.

## Introduced by

631ba2417337656f7832d95820f794ee9216df0b "Fix VDSO `clock_gettime` detection on older Linux
versions." (2022-01-31) - this is the commit that introduced the differentiated `ok` logic for the
`clock_gettime` branch (the one this candidate's repro reproduces): before it, every architecture
shared one unconditional `assert!(!ptr.is_null())` with no `ok` variable at all (confirmed via
`git log -S'let ok = true' -- src/imp/linux_raw/vdso_wrappers.rs`); this commit split it into a
hardcoded-`true` 64-bit branch and a null-checked 32-bit branch, creating the asymmetry this
candidate is about. The `getcpu` branch's identical shape was introduced separately, later, in
15278a1279e6d5d57adf36520cfd8d07d2ff4385 "Implement `rustix::process::sched_getcpu`. (#921)"
(2023-11-07) - same bug class, different call site, not independently re-investigated as its own
candidate (see "What's not done" below). The x86 `__kernel_vsyscall` unconditional assert has no
"introducing" commit in this same sense - it has been unconditional since the original
1b91f18eac5dc91df9ab37ac8beefab5a6e78c4f "Linux vDSO support." that first added x86 vDSO support,
i.e. it predates the `ok`-splitting pattern entirely and was never "fixed" for 32-bit the way the
other two branches were.

## The fix

Extracted the "is this symbol pointer trustworthy" decision - which used to be duplicated across
all three call sites, and on several architectures wrongly hardcoded to "always trust it" with no
null check - into one small shared function:

```rust
fn vdso_sym_is_ok<T>(ptr: *mut T) -> bool {
    !ptr.is_null()
}
```

and called it uniformly at all three sites (`clock_gettime`, `getcpu`, x86 `__kernel_vsyscall`),
replacing the old `let ok = true; ... assert!(!ptr.is_null());` / unconditional-assert patterns
with `if vdso_sym_is_ok(ptr) { /* store */ }` - a missing symbol now silently falls back to the
syscall path `minimal_init()` already installed (exactly the same fallback the 32-bit branches,
and every other call site in this file, already used), instead of aborting the process. This also
let two now-redundant `#[cfg]`-split `ok`-determination blocks (whose arms, after the fix, compute
the identical expression for every architecture actually compiled in that block) collapse to a
single unconditional line, including one previously dead/unreachable `ok = false` arm in the
`getcpu` block (unreachable because the enclosing block's own `#[cfg]` already excludes every
architecture that arm covered).

Verified:
- Re-ran the repro against the patched tree (`patch.diff`'s own bundled regression test,
  `vdso_sym_is_ok_rejects_null_and_accepts_nonnull`, calling the real patched function directly
  with a null and a non-null pointer): passes.
- `cargo clippy --lib --features time,stdio,fs` on the patched tree: no new warnings in
  `vdso_wrappers.rs`; two pre-existing, unrelated warnings only (a `build.rs`
  `clippy::manual_filter` lint, and `clippy::wrong_self_convention` on
  `Timespec::to_sec_usec`), present identically on the pristine tree.
- A real end-to-end sanity check: a separate, throwaway crate depending on the patched tree's
  `rustix` as a path dependency, calling the real public `rustix::time::clock_gettime` and
  `rustix::thread::sched_getcpu` against this host's real kernel vDSO - both still resolve
  correctly through the vDSO fast path (confirms the fix doesn't disturb the common,
  symbol-present case).
- Project test suite (`--features time,stdio,fs --no-fail-fast`):
  - Before fix (pristine tree, no patches at all): lib unittests **20 passed / 0 failed / 0
    ignored**.
  - After fix (`patch.diff` applied, adds one test): lib unittests **21 passed / 0 failed / 0
    ignored**.
  - Two unrelated, pre-existing failures present identically before and after (same as recorded
    in candidate 036 and `projects/rustix.md`): `tests/io`'s `ioctl::test_int_setter` (sandbox has
    no real tty device, `ENOENT`), and two doctests (`timespec::Timespec::checked_add`/
    `checked_sub`) that need the `event` feature, not included in the `time,stdio,fs` feature set
    used here.

## What's not done / open questions

- The `getcpu` branch (`x86_64`/`riscv64`/`powerpc`/`powerpc64`/`s390x`, introduced separately by
  15278a12) and the x86 `__kernel_vsyscall` branch share the identical "hardcoded trust" bug
  shape and are fixed by the same patch, but neither has its own independent, dedicated repro test
  in `repro/` (only the `clock_gettime` branch does) - verified instead via code symmetry (the
  same `vdso_sym_is_ok` call now guards all three sites) and the bundled regression test's direct
  unit coverage of `vdso_sym_is_ok` itself, plus the full project test suite and the real
  end-to-end sanity check passing. A session with access to a genuine minimal-vDSO
  kernel/sandbox (see "Fully reproducible" above) could tighten this to three fully independent
  end-to-end reproductions, one per call site.
- No genuine end-to-end repro with a real restricted-kernel-supplied vDSO (gVisor/runsc, a
  stripped-down embedded kernel, qemu-user's own synthetic vDSO where applicable) was possible in
  this sandboxed environment - see "Fully reproducible" above. A session with `CAP_SYS_PTRACE` or
  `/dev/kvm` available (for gVisor) could tighten this further.
- Not checked this session: whether any other internal caller relies on `init()`'s old
  hard-abort behavior as an (unintentional) sanity check that the vDSO is "fully formed" before
  proceeding - a brief read of every call site in this file shows none do (they all already
  tolerate a null `CLOCK_GETTIME`/`GETCPU`/`SYSCALL` static via their own `None => init_*()`
  fallback, which is exactly the normal "not yet initialized" case at process start).
