# io_uring_register_arg_const_should_be_mut

`io_uring_register`/`io_uring_register_with` declare their `arg` parameter `*const c_void`, but
for `IoringRegisterOp::RegisterRingFds` the kernel genuinely writes through that pointer. Sourced
from GitHub issue #1545 (`semver bump` label).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/io_uring/mod.rs`, both backends' `io_uring_register`/`io_uring_register_with`, `io_uring_rsrc_update` struct, `tests/io_uring/register.rs` | Found the maintainers had already partially addressed this - see "Introduced by" |
| Real syscall repro | real `io_uring_setup` + real `io_uring_register(RegisterRingFds, ...)` on this host's real kernel (7.0.0-31-generic, x86_64), reading the `offset` field before/after | 3/3 runs show the kernel genuinely overwrites `offset` in place |
| `git log -S`/`git blame`, `git log --follow` | on the `arg: *const c_void` signature and `RegisterRingFds` support | Found the exact already-merged commit acknowledging this issue |
| `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast` before/after fix, both backends | | No regressions |

## Status

**Confirmed**. 3/3 identical runs before the fix and 3/3 identical runs after the fix (deterministic:
each run creates a brand-new empty io_uring instance, so the same slot index `0` is assigned every
time).

**Fully reproducible**: Yes. A real `io_uring_setup` + `io_uring_register(IoringRegisterOp::RegisterRingFds,
...)` call sequence on the real kernel, through rustix's own public API, with no synthetic/hand-waved
pointer.

## Class

`UB` / type-soundness. The kernel genuinely writes through the `arg` pointer for
`IORING_REGISTER_RING_FDS` (confirmed empirically, not just trusted from the issue text), while
rustix's public function signature declares that pointer `*const c_void`. A caller who legitimately
derives that pointer from a `&T` would be handing the kernel a pointer Rust's aliasing model
assumes nothing writes through - a real soundness footgun, not just an ergonomics nit.

## Root cause

`struct io_uring_rsrc_update { offset, resv, data }` is used as the `arg` payload for
`IORING_REGISTER_RING_FDS`/`IORING_UNREGISTER_RING_FDS`. Per the kernel's ring-fd registration
handler, `data` is the input (fd to register) and `offset` doubles as an in/out slot selector: pass
`u32::MAX` (`-1`) to request auto-allocation, and the kernel overwrites `offset` in place with the
slot index it actually assigned before returning. rustix exposes `arg` as `*const c_void` in the
public `io_uring_register`/`io_uring_register_with` signatures even though this one opcode requires
write access to the pointee - a real, if narrow, `*const`/`*mut` contract violation.

## Introduced by

The `*const c_void` signature itself dates to the original io_uring API addition, commit
495ae5cf "Add Linux io_uring APIs. (#290)", long before `RegisterRingFds` existed. The mismatch
became live UB once `IORING_REGISTER_RING_FDS` support was added (59f7b712/b184b838, "Some io_uring
additions"/"io_uring cleanups... 6.2"). Notably, **the maintainers are already aware of this exact
issue**: commit 3e853c8c "Fix UB in an io_uring test, and add a safety comment. (#1547)" (2025-11-09,
already merged into this worktree's HEAD at 287214b8) fixed the UB in `tests/io_uring/register.rs`
(which used to cast a `&T` shared reference straight to `*const c_void` for a pointee the kernel
mutates) and added the exact doc comment found during this investigation: "If `opcode` is
`IoringRegisterOp::RegisterRingFds`, `arg` must point to mutable memory, despite being `*const`."
That commit's message explicitly says: "We should fix the API too, but that's a semver break, so
I've filed #1545." **This candidate is not a fresh finding - it is the exact, already-filed,
already-triaged issue #1545, currently intentionally deferred to a semver-major bump.** It is
recorded here anyway because this workspace's job is to independently verify a real reproduction
and a working, tested fix - which the GitHub issue alone does not have - and because
`___BUGHUNT___` is meant to be a complete record of what's live in the current codebase regardless
of whether it's already tracked elsewhere.

## The fix

See `patch.diff`: change `arg`'s type from `*const c_void` to `*mut c_void` in the public function
signatures (`io_uring_register`, `io_uring_register_with`), both backend implementations, and the
one existing test call site that had to work around the old signature with an explicit `*const`
cast (removing that now-unnecessary cast and its linked-issue comment). Also removed the doc-comment
callout about `arg` needing to "point to mutable memory, despite being `*const`" - it's no longer
needed once the type itself says so.

**Breaking-change note**: Yes, this is public-API-breaking (signature change on two public
`unsafe fn`s). Any external caller currently passing a `*const c_void` (e.g. from a `&T` cast) will
fail to compile and must change to `*mut c_void`. This matches the issue's own `semver bump` tag
and the maintainer's own note in 3e853c8c that the real fix must wait for a semver-major release.
This session implemented and tested it per the "bounded fix" scope only - not proposing it ship as-is
in a patch release; a maintainer applying this would do so as part of a deliberate major-version bump.

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast` -
  411 passed / 1 failed before, identical 411 passed / 1 failed after (the one failure,
  `io::ioctl::test_int_setter`, needs `/dev/net/tun`, unrelated, confirmed present with or without
  the patch). The dedicated `tests/io_uring/main.rs` binary
  (`register::test_io_uring_register_with`, `register::io_uring_buf_ring_can_be_registered`) passed
  2/2 both before and after. Both `linux_raw` (default) and `libc`
  (`RUSTFLAGS="--cfg rustix_use_libc"`) backends confirmed to compile cleanly with the patch applied.

## What's not done / open questions

- This fix is a breaking API change deliberately deferred by the maintainers to a semver-major
  release (per 3e853c8c's own commit message) - it should not be applied as-is to a patch release;
  a maintainer would need to bundle it with rustix's next major version bump.
- Two other `io_uring_register` opcodes were flagged as plausibly sharing the same write-through-a-
  `*const`-pointer shape, but were **not independently verified** this session (time-boxed):
  `IoringRegisterOp::RegisterProbe` (`IORING_REGISTER_PROBE` - the kernel fills in the caller-supplied
  `struct io_uring_probe`'s ops array/`ops_len`) and `IoringRegisterOp::RegisterPbufStatus`
  (`IORING_REGISTER_PBUF_STATUS` - `struct io_uring_buf_status.head` is plausibly kernel-reported).
  Added to `IDEAS.md` as unconfirmed leads for a future candidate.
