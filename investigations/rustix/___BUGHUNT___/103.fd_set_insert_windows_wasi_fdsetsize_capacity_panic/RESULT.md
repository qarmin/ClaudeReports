# fd_set_insert_windows_wasi_fdsetsize_capacity_panic

`src/event/select.rs`'s `fd_set_insert` (a fully safe public function backing `FD_SET`) has two
independent implementations gated by `#[cfg(...)]`: a bitvector path for most Unix targets (no
upper bound on the number of distinct fds, modulo the caller's own buffer size - see candidate
031's analysis of its sibling bug), and a `windows`/`target_os = "wasi"` path that reinterprets the
caller's buffer as a fixed-capacity `FD_SET` struct (`fd_count` + a fixed-size `fd_array`). The
`windows`/`wasi` path slices `set.fd_array[..fd_count as usize + 1]` whenever a new, not-yet-present
fd is inserted, with no check that `fd_count` is still within `fd_array`'s fixed compile-time
length (`FD_SETSIZE` - 64 on Windows via `windows_sys`, 1024 on wasi via libc's wasi binding).
Once a caller has inserted `FD_SETSIZE` distinct fds, inserting one more panics with an
out-of-bounds slice-end index, unlike the Unix bitvector path, which has no such cap.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `src/event/select.rs`, `tests/event/select.rs` | n/a | Confirmed `fd_set_insert`'s `#[cfg(any(windows, target_os = "wasi"))]` branch (lines 141-152) slices `set.fd_array[..fd_count as usize + 1]` unconditionally whenever the fd being inserted isn't already present, with no check against `fd_array`'s fixed length; confirmed no existing test in `tests/event/select.rs` exercises the FD_SETSIZE boundary on either target |
| Checked the real `FD_SETSIZE`/`FD_SET` layout for both affected targets, against the real vendored dependencies this crate actually uses | `grep FD_SETSIZE` in `windows-sys-0.59.0`'s `WinSock/mod.rs` (`pub const FD_SETSIZE: u32 = 64u32`, `pub struct FD_SET { fd_count: u32, fd_array: [SOCKET; 64] }`) and in `libc-0.2.189`'s `wasi/mod.rs` (`pub const FD_SETSIZE: size_t = 1024`) | Confirmed both targets' real `FD_SET` has a fixed-size `fd_array` (64 for Windows, 1024 for wasi) baked into the struct layout, independent of how large a buffer the caller actually allocates - the compile-time array bound is what panics, not a buffer-sizing mistake |
| Manual boundary-value construction via Miri cross-interpretation of the real, unmodified, public `rustix::event::fd_set_insert`/`fd_set_num_elements`/`FdSetElement` API, for both affected targets (`cargo +nightly miri run --target wasm32-wasip1` and `--target x86_64-pc-windows-gnu`, both with a prebuilt `rust-std` via plain `rustup target add`, no `-Zbuild-std` needed) - a standalone crate sizes a buffer per the documented `fd_set_num_elements(FD_SETSIZE, nfds)` convention, inserts `FD_SETSIZE` distinct fds, then inserts one more not-yet-present fd | debug build, 3 repeat runs per target | **Reproduces 3/3 runs on both targets, deterministically.** wasm32-wasip1: Miri's wasi shim can't execute the `write` syscall used by `println!`/the default panic hook/`catch_unwind`'s uncaught-panic reporter, so the panic surfaces as a full process abort - but Miri's own abort backtrace names the exact panicking frame: `rustix::event::fd_set_insert at .../src/event/select.rs:148:45: 148:70` (the cited line). x86_64-pc-windows-gnu: `catch_unwind` works normally under Miri for this target, so the repro reports a clean, distinguishing exit code (12) for "panicked as expected", confirming the exact same code path |
| Negative control: re-ran both targets with only `FD_SETSIZE - 1` fds filled before the extra insert | same harness, 1 run each | No panic on either target (clean exit 0) - confirms the harness and the boundary condition are both correctly isolated, not "always panics regardless of input" |
| Fix verification + a second, differently-shaped input in the same bug class (CONFIRMATION.md section 7): filled to `FD_SETSIZE - 1`, then made **two** consecutive inserts of new fds (the first fills to exactly `FD_SETSIZE`, the second is the one that used to panic), and checked the resulting `FdSetIter::new(&fds).count()` after the fix | debug build, both targets | Pre-fix: panics on the second insert, as expected (sanity-checked by temporarily reverting the patch). Post-fix: no panic on either target, and `count() == FD_SETSIZE` exactly (confirms the fix silently drops the excess insert without under- or over-counting, not just "doesn't crash") |

## Status

**Confirmed.** Reproduces 3/3 runs on both affected targets (wasm32-wasip1 via Miri's abort
backtrace naming the exact panicking line; x86_64-pc-windows-gnu via `catch_unwind` reporting a
clean, distinguishing exit code), plus a second, differently-shaped boundary input (two consecutive
inserts at capacity) reproducing identically. This is a pure, deterministic slice-bounds panic
driven by fixed control flow (`fd_count == FD_SETSIZE`) - no layout/seed/timing dependence.

**Fully reproducible**: **Yes**. `repro/` calls the real, unmodified, public
`rustix::event::fd_set_insert`/`fd_set_num_elements`/`FdSetElement` API directly, sizing its buffer
exactly as the function's own documented convention (`fd_set_num_elements(set_count, nfds)`)
describes, then inserting `FD_SETSIZE + 1` distinct, legitimate `RawFd` values - a caller inserting
more distinct fds than it reserved space for is itself the genuine external input (a caller-supplied
sequence of `i32`/`u64` values), not a stand-in for a second participant's reaction to anything;
there is no peer/wire/upstream input being faked here (same shape as candidate 031's confirmation of
`CONFIRMATION.md` section 11 not applying). No real Windows or wasi hardware/runtime was available
in this sandbox (no `wine`, no `wasmtime`/`wasmer`, confirmed via `which`/`apt-cache policy`) - Miri
cross-interpretation of the real, unmodified source for both targets was used instead, the same
technique this project's own `projects/rustix.md` risk map already documents as reusable for
Apple/BSD targets (candidates 035, 039, 058, 081) and extended here to Windows and wasi for the
first time.

## Class

crash

## Root cause

`fd_set_insert`'s `#[cfg(any(windows, target_os = "wasi"))]` branch (`src/event/select.rs:141-152`)
reinterprets the caller's `&mut [FdSetElement]` buffer as a `*mut FD_SET` - a struct with a fixed
compile-time-sized `fd_array` field (`[i32; 1024]` on wasi via libc's `wasi::FD_SETSIZE`, `[SOCKET;
64]` on Windows via `windows_sys`'s `WinSock::FD_SETSIZE`). When the fd being inserted isn't already
present, the function computes `&mut set.fd_array[..fd_count as usize + 1]` to grow the live view by
one slot before writing the new value - but never checks that `fd_count + 1` still fits within
`fd_array`'s fixed length. Once a caller has already inserted exactly `FD_SETSIZE` distinct fds
(whether by following the crate's own `fd_set_num_elements(FD_SETSIZE, nfds)` buffer-sizing
convention literally, or by inserting more fds than any `set_count` it originally passed to size the
buffer), the next new, not-yet-present insert computes a slice end index one past `fd_array`'s real
length, and Rust's own slice-bounds check panics - regardless of how large the caller's underlying
byte buffer actually is, since `fd_array`'s length is fixed by the struct definition itself, not by
the slice the caller passed in. This is architecturally the same `FD_SETSIZE` limitation the
function's own `select()` sibling doc comment says this crate avoids ("this `select` wrapper differs
from POSIX in that `nfds` is not limited to `FD_SETSIZE`") - true for the bitvector (non-
Windows/wasi) path, but not actually true for this path, which silently reintroduces the classic
POSIX 64-fd (Windows) / 1024-fd (wasi) `select()` ceiling as an unhandled panic instead of either
lifting the limit or returning a documented error. Same function, same originating commit, and the
same missing-bounds-check shape as candidate 031's sibling finding in the non-Windows/wasi branch of
the very same function (and `fd_set_remove`, candidate 102) - but a structurally different root
cause (an unvalidated compile-time array-length cap vs. an unvalidated sign/range conversion), so
kept as its own candidate per this workspace's one-bug-per-folder convention.

## Introduced by

8493118c "Implement `select`. (#1162)" (2024-10-11) - the same commit that introduced the entire
`select`/`fd_set_insert`/`fd_set_remove` module, including this exact
`&mut set.fd_array[..fd_count as usize + 1]` line, from the start (confirmed via `git log -S
"fd_count as usize + 1" -- src/event/select.rs`, which returns only this one commit, and `git show
8493118c -- src/event/select.rs`, which shows the line being added fresh as part of the new file,
not modified from a prior version).

## The fix

Replaced the unchecked slice-and-grow with a single `get_mut` lookup into the fixed-size
`fd_array`, bounded automatically by its real length - if the index is out of range (the set is
already at `FD_SETSIZE` capacity), the insert is silently a no-op instead of panicking, mirroring
candidate 031's fix style for the sibling branch of the same function:

```rust
if !fd_array.contains(&(fd as _)) {
    if let Some(slot) = set.fd_array.get_mut(fd_count as usize) {
        *slot = fd as _;
        set.fd_count = fd_count + 1;
    }
}
```

Also added a doc comment on `fd_set_insert` stating this behavior explicitly (an already-full
`FD_SETSIZE`-capacity set silently ignores a new insert on Windows/wasi), matching the style
candidate 031 used for the sibling branch's negative/out-of-range-`fd` case.

Verified:
- Re-ran the exact `repro/` (Miri cross-interpretation, both targets) against the patched tree: no
  panic on either target (clean exit 0), vs. panicking 3/3 before the patch.
- Checked the fix against a second, differently-shaped input in the same bug class
  (`CONFIRMATION.md` section 7): two consecutive new inserts starting from `FD_SETSIZE - 1` (the
  second one being the one that previously panicked) - post-fix, neither panics and
  `FdSetIter::new(&fds).count()` is exactly `FD_SETSIZE` on both targets, confirming `fd_count` is
  correctly capped rather than silently corrupted (over- or under-counted).
- Checked the normal in-range case is unaffected: filling fewer than `FD_SETSIZE` fds and reading
  them back via `FdSetIter` behaves identically before and after (not separately scripted - follows
  directly from the existing `tests/event/select.rs` suite passing unchanged, see below).
- Project test suite, `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`
  (host `x86_64-unknown-linux-gnu`, the only target this suite can actually execute real I/O
  against in this sandbox): **411 passed, 1 failed, identical before and after** - the one failure
  is the pre-existing, environment-caused `ioctl::test_int_setter` (`/dev/net/tun` missing in this
  container, already documented in `projects/rustix.md`), not a regression from this patch. No
  Windows- or wasi-specific test target exists in this suite that could exercise the patched branch
  natively in this sandbox (consistent with no Windows/wasi hardware or runtime being available at
  all here) - the Miri cross-interpretation repro is the only execution of this exact branch
  available.

## What's not done / open questions

- `fd_set_remove`'s own `windows`/`wasi` branch (same file, a few lines below) was investigated
  separately as candidate 102 (shares the same reinterpreted-`FD_SET` shape but a different bug
  mechanism - confirm by reading that candidate's own `RESULT.md`, not duplicated here).
- No real Windows or wasi hardware/runtime was available to execute the patched branch's actual
  compiled machine code end-to-end (only Miri's interpreter) - a future session with access to
  `wine`/a real Windows VM, or `wasmtime`/`wasmer`, could re-confirm this natively; not expected to
  change the outcome, since the bug is a pure, deterministic Rust slice-bounds check with no
  platform-specific runtime behavior involved before the panic point.
- `fd_set_bound` (same file) also reinterprets the buffer as `FD_SET` and reads `fd_array[..
  fd_count as usize]`, but never grows past `fd_count` - reviewed and confirmed not to share this
  bug shape (same reasoning as candidate 031's review of this function), not a separate candidate.
- Whether any real-world caller actually hits this in practice depends on whether they insert more
  distinct fds than the `set_count` they originally passed to `fd_set_num_elements` (or than
  `FD_SETSIZE`, if they didn't rely on that helper at all) - not surveyed across crates depending on
  `rustix::event::select` this session.
