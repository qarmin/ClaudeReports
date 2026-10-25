# raw_dir_next_zero_reclen_stall

`RawDir::next`/`is_buffer_empty` (`src/fs/raw_dir.rs:195-230`) does `self.offset +=
usize::from(dirent.d_reclen)` with no check that `d_reclen != 0`. A `d_reclen` of 0 leaves
`self.offset` unchanged, so `is_buffer_empty()` (`self.offset >= self.initialized`) never becomes
true again, and `next()` re-parses the exact same bytes forever. Flagged as a candidate because
this is the exact same bug class, same missing invariant check, as confirmed candidate 043
(`src/backend/linux_raw/fs/dir.rs::Dir::read`) - but in a different file/struct (the public
`RawDir` API, which has no `any_errors`-style latch and no bounds check at all on the raw buffer,
unlike `Dir`). Candidate 044 examined this same file/function for a different, ruled-out
uninitialized-read concern (a hypothetical `getdents64` record split across the buffer boundary)
and did not check this stall path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value reasoning (code reading) | Traced `d_reclen=0` through `RawDir::next`: `self.offset += usize::from(dirent.d_reclen)` leaves `self.offset` unchanged; `is_buffer_empty()` (`self.offset >= self.initialized`) stays `false` forever since `self.offset` never catches up to `self.initialized`; no error-latch field exists in `RawDir` (unlike `Dir`'s `any_errors`) to stop re-parsing. | Confirmed the mechanism purely by reading - matches the candidate description exactly, and matches confirmed candidate 043's identical mechanism in the sibling `Dir::read`. |
| In-crate `#[cfg(test)]` test added directly to `src/fs/raw_dir.rs`, driving the real public `RawDir::new`/`RawDir::next` entry points against a buffer that *was* populated by one real `getdents64` syscall on a real directory fd, reaching into `RawDir`'s private fields only to corrupt the 2 `d_reclen` bytes of the next record about to be parsed (same technique candidate 043's own `repro_test.patch` already established for the sibling `Dir` type) | debug build, `--features "std,all-apis"`, `cargo test --lib -- fs::raw_dir::tests::raw_dir_next_zero_reclen_stalls_forever --exact`, 5 repeat runs with a single real file in the tmp directory, 3 further repeat runs with a boundary-value variation (5 real files, corrupting the 4th-consumed record instead of the 1st, so the corrupted record sits at a different, later buffer offset) | **Reproduces 5/5 runs** (single-file layout) **and 3/3 runs** (5-file, later-offset layout), bit-for-bit identical panic message every time: `RawDir::next` returns the exact same `RawDirEntry` for all 50,000 iterations attempted, `self.offset` never advances. |
| Real-FUSE end-to-end repro attempt (the only way to make a genuine kernel `getdents64` call actually return `d_reclen=0`, since the real VFS never does) | Not re-attempted as a separate exercise this candidate - already exhaustively tried and found infeasible in this exact sandboxed container by sibling candidates 043 and 044 against the identical constraint (no `/dev/fuse`, no `CAP_SYS_ADMIN`, `sudo` blocked, no package-manager/network access) | **Infeasible in this sandboxed container** (same conclusion reused, not re-derived - see candidates 043/044's own `RESULT.md` for the full list of things tried). Candidate 044 additionally established that even a real, working FUSE mount would not help: the kernel's VFS (`fs/readdir.c`) enforces the "no partial/malformed record" invariant generically, above any individual filesystem driver, so no real backend - buggy or malicious - can violate it from userspace regardless of sandboxing. |

## Status

**Confirmed**

Reproduces 5/5 runs under the original single-file layout, and 3/3 runs under a second,
different-buffer-offset layout (debug build, `--features "std,all-apis"`), deterministic
(bit-for-bit identical panic message each run - no flakiness, no dependency on heap
layout/ASLR/hashmap order, since the corrupted buffer offset is computed deterministically from
the real syscall's own return value each run).

**Fully reproducible**: **No** - justified.

The bug is specifically about how `RawDir::next` (one participant) reacts to a malformed
`getdents64` record produced by the kernel/filesystem-backend syscall layer (the other
participant, per CONFIRMATION.md section 11's framing) - and the real Linux VFS never emits
`d_reclen == 0`, so demonstrating this for real requires an actually-misbehaving backend (a
broken FUSE/9p driver, or a corrupted on-disk image surfaced through one). This candidate's own
description already flags that caveat, and it is the identical caveat sibling candidates 043 and
044 already established for this exact file and bug class in this exact sandboxed container:
- No `/dev/fuse` device node exists.
- No `CAP_SYS_ADMIN` (`capsh --print`'s "Current IAB" line explicitly lists it excluded from the
  bounding set, verified by candidate 043/044).
- `sudo` is blocked outright by the kernel's "no new privileges" flag.
- No package-manager or crates.io network access to install `fuse3`/`strace` tooling or pull in a
  `ptrace`-based fault injector as a fallback.
- Independently of sandboxing: the kernel's own VFS (`fs/readdir.c`) enforces the "never emit a
  partial/malformed `linux_dirent64` record" invariant generically, above any individual
  filesystem driver - so even a real, working malicious FUSE mount could not violate it from
  userspace (candidate 044's own conclusion, directly applicable here since it concerns the exact
  same kernel-side guarantee this candidate's `d_reclen` field also depends on).

Per CONFIRMATION.md section 11's explicit exception ("a finding that's clearly real from code
reading alone but where building a genuine end-to-end repro is impractical or infeasible in the
time available... may still be written up"): this is written up as Confirmed using the closest
practical substitute - `repro/repro_test.patch` drives the real, unmodified `RawDir::new`/
`RawDir::next` public entry points against a buffer that *was* populated by one real `getdents64`
syscall on a real directory, and only overwrites the 2 bytes of the one `d_reclen` field a buggy
backend would have gotten wrong (everything else in the buffer - `d_ino`, `d_off`, `d_type`,
`d_name`, every other record - is exactly what the real kernel returned). This reuses the exact
same technique candidate 043's own `repro_test.patch` already established and verified for the
sibling `Dir::read` bug in this same codebase.

## Class

logic (unbounded-loop / denial-of-service via a missing invariant check on untrusted-but-normally-
trusted kernel data)

## Root cause

`RawDir::next`'s iteration-advance logic is `self.offset += usize::from(dirent.d_reclen)` with no
lower bound on `d_reclen`. Unlike the sibling `Dir::read` (which at least has an upper-bound
`assert!(self.buf.len() - pos >= d_reclen as usize)`, albeit also missing the `d_reclen > 0`
check candidate 043 found), `RawDir::next` has no bounds check on `d_reclen` at all - its only
defense is the doc comment "Assumption: the kernel will not give us partial structs." Combined
with `is_buffer_empty()`'s own check (`self.offset >= self.initialized`) and `next()` only
refilling the buffer via a fresh `getdents_uninit` syscall when `is_buffer_empty()` is true, a
`d_reclen` of 0 is a fixed point: every subsequent call re-derives the identical `self.offset`,
re-parses the identical bytes, and yields the identical `RawDirEntry`, with nothing in the loop
ever able to break out of it on its own. The kernel's own VFS (`fs/readdir.c`) always emits a
`d_reclen` that accounts for at least the fixed-size header plus a NUL-padded name, so this is
unreachable from any correctly-behaving backend - but nothing in this function defends against a
backend that doesn't uphold that contract.

## Introduced by

1b061427 "Add zero-compromise directory iteration (#457)" (2022-11-23) - the commit that added
`RawDir` itself, including the exact unchecked `self.offset += usize::from(dirent.d_reclen)` line
(confirmed via `git log -S "self.offset += usize::from(dirent.d_reclen)" --all -- src/fs/raw_dir.rs`,
which returns only this one commit). The surrounding file has been touched by several later
commits (811e5eaa "Fix another UB in raw dir" #474, af0af278 "Fix UB due to misaligned RawDir
pointer" #464, bc47644a "Simplify RawDir iteration" #515, d78fac6a "Add method to know when buffer
will be refilled" #524, and others - see `git log --follow -- src/fs/raw_dir.rs`), but none of
them added a `d_reclen > 0` check; the missing check has been present, unchanged, since the file's
very first commit.

## The fix

Add an explicit `d_reclen == 0` check right after the unaligned-struct read, before it's used to
advance `self.offset`: discard the remainder of the current buffer (`self.offset =
self.initialized`, so the next call's `is_buffer_empty()` is true and triggers a fresh
`getdents64` instead of re-parsing the corrupted bytes) and return `Some(Err(io::Errno::BADMSG))`
- `EBADMSG` ("bad message") is the same error code candidate 043's sibling fix in `Dir::read`
already uses for "the data doesn't have the structure it's supposed to", already imported/
available via `io::Errno` with no new dependency. Unlike `Dir`, `RawDir` has no `any_errors`-style
latch field to permanently stop iteration after an error (its existing convention, e.g. on a
`getdents_uninit` syscall error, is to let a subsequent call retry), so this fix follows that same
existing convention rather than introducing a new kind of terminal state: it clears the exact
corrupted record from view and lets the next call re-fetch cleanly, rather than leaving the
caller stuck re-observing the same corrupted bytes. +16/-1 lines, `src/fs/raw_dir.rs` only.

Per CONFIRMATION.md section 7, the fix was re-verified against more than just the one triggering
input/offset: with the corrupted record positioned at a different, later buffer offset (5 real
files consumed before corruption, instead of 1), it held in all 3 of those repeat runs too.

Before -> after, re-running `repro/`'s test (`cargo test --features=std,all-apis --lib --
fs::raw_dir::tests::raw_dir_next_zero_reclen_stalls_forever --exact`):
- Before (pristine + `repro_test.patch` only): panics every time with `REPRODUCED: RawDir::next
  returned the same unchanged entry for 50000 straight iterations (d_reclen=0 stall)` - 3/3 runs
  captured in `output.txt` (5/5 runs total were actually run during investigation; see "What was
  tried" above).
- After (`patch.diff` also applied): `ok` every time - 3/3 runs captured in `output.txt`.

Project test suite (`cargo test --quiet --workspace --features "std,all-apis,use-libc"
--no-fail-fast`), with the repro test included in both counts for the "after" run (it's part of
the same file):
- Before (repro test added, no fix): **411 passed, 2 failed, 5 ignored** (the 2 failures are the
  new repro test itself, plus the one pre-existing unrelated failure below).
- After (fix applied): **412 passed, 1 failed, 5 ignored** (the +1 passed/-1 failed vs. before is
  the new repro test itself now passing; no other change).
- The one failure in both runs, `ioctl::test_int_setter` ("No such file or directory"), is a
  pre-existing, unrelated environmental failure - this container has no `/dev/net/tun`, which that
  test depends on (already documented in this workspace's own `README.md`/`projects/rustix.md`
  from earlier candidates, including 043's own identical note). No new failures introduced by the
  fix beyond the repro test itself resolving.

## What's not done / open questions

- `RawDir::next`'s upper-bound case (a `d_reclen` *larger* than the remaining buffer) is not
  checked at all currently - it would read past `self.initialized` into whatever uninitialized or
  stale bytes happen to follow in the backing buffer (a real memory-safety concern, not just a
  stall), since `RawDir` has no equivalent of `Dir`'s `assert!(self.buf.len() - pos >= d_reclen as
  usize)`. Out of scope for this candidate (a different failure mode) but worth a dedicated look,
  since it sits one line below this fix in the same function and depends on the exact same
  untrusted field.
- The `libc`-backend equivalent (`getdents_uninit` in `src/backend/libc/fs/syscalls.rs`) feeds the
  same `RawDir::next` code path (confirmed: `linux_dirent64` resolves to `linux_raw_sys`'s type in
  this container regardless of backend, since `linux_raw_dep` is set whenever the `linux-raw-sys`
  crate is a dependency at all, not only when the `linux_raw` backend is selected) - the fix above
  covers both backends without any `#[cfg]` split, not a separate uncovered path.
- A longer, dedicated fuzzing campaign (via `fuzz-bug-repro`, run separately) feeding arbitrary
  byte buffers directly into a test-only `RawDir`-construction helper could surface further
  `getdents64`-field-trust issues in this same function (e.g. `d_off`/`d_type`/`d_ino` are loaded
  but never range-checked either, though nothing downstream currently indexes with them the way
  `d_reclen` is used to advance `offset`) - same open item candidate 043 left for the sibling
  `Dir::read`.
