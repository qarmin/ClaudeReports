# fd_set_remove_windows_wasi_wrong_swap_index

`src/event/select.rs`'s `fd_set_remove`, inside the `#[cfg(any(windows, target_os = "wasi"))]`
branch (fully safe, public `FD_CLR`-equivalent API), swaps the removed slot with the wrong source
element: it reads `set.fd_array.last()` off the OS's fixed-size `[i32; FD_SETSIZE]` array itself
instead of `set.fd_array[fd_count - 1]` (the last *active* slot in the `fd_count`-bounded slice
computed two lines above), so removing any fd that isn't exactly the last one inserted silently
drops a real active fd and inserts a bogus fd 0 in its place.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `fd_set_insert`/`fd_set_remove`'s windows/wasi branch | n/a | Confirmed the mismatch: line 169 computes `fd_array = &set.fd_array[..fd_count]` (the active slice) but line 173 indexes `set.fd_array` (the full array) via `.last()`, not the active slice or `fd_count - 1` |
| Real end-to-end repro: standalone crate depending on the unmodified public `rustix::event::{fd_set_insert, fd_set_remove, FdSetIter}` API, compiled for `wasm32-wasip1` (installed via `rustup target add`) and run under a freshly-installed real `wasmtime 49.0.1` | `cargo build --target wasm32-wasip1` + `wasmtime run ...wasm`, dev profile, 5 repeat runs of the primary scenario plus 2 boundary scenarios (remove-last-element, drain-to-empty) | Bug reproduced deterministically every run: removing a non-last fd from a 3-member set `[5, 10, 15]` (remove 10) yields `[5, 0]` instead of `[5, 15]`; draining a 2-member set one remove at a time leaves a stray `[0]` instead of ending empty. The remove-last-element boundary case happened to still work under the bug (coincidence: `.last()` only differs from the active-slice-last when `fd_count < FD_SETSIZE`, which the already-last-element case doesn't expose) |

This didn't need escalation past one approach - the bug is pure, deterministic array-index
bookkeeping with no adversarial-input/UB/timing component, so a single targeted end-to-end run
already met `CONFIRMATION.md`'s repeatability bar (5/5 identical, deterministic by construction).

## Status

**Confirmed.**

Reproduces 5/5 runs on the primary scenario (remove a non-last fd from a 3-member set), plus two
boundary scenarios (remove-last-element: correct even with the bug; drain-to-empty: also buggy,
leaves a stray fd 0 behind) - deterministic, no flakiness expected or observed (pure array index
arithmetic, no randomness/timing/concurrency involved).

**Fully reproducible: Yes.** The repro runs the real, unmodified public
`rustix::event::fd_set_insert`/`fd_set_remove`/`FdSetIter` API, compiled for `wasm32-wasip1` (a
real affected target under this crate's own `#[cfg(any(windows, target_os = "wasi"))]` gate, since
the development host is `x86_64-unknown-linux-gnu` and can't natively hit this branch) and executed
under a real wasmtime runtime - not a hand-copied reimplementation of the `FD_SET` struct standing
in for the real one. This is a single-participant logic bug (pure bookkeeping inside one function,
no peer/caller producing adversarial input), so `CONFIRMATION.md` section 11's two-participant
requirement doesn't apply here.

## Class

logic

## Root cause

`fd_set_remove`'s windows/wasi branch (`src/event/select.rs:165-176`):

```rust
let set = unsafe { &mut *fds.as_mut_ptr().cast::<FD_SET>() };
let fd_count = set.fd_count;
let fd_array = &set.fd_array[..fd_count as usize];          // active slice, correctly bounded

if let Some(pos) = fd_array.iter().position(|p| *p as RawFd == fd) {
    set.fd_count = fd_count - 1;
    set.fd_array[pos] = *set.fd_array.last().unwrap();        // BUG: indexes the full array
}
```

`set.fd_array` is the OS-sized fixed array `[i32; FD_SETSIZE]` (64 entries on the targets checked
here), not the logically-active `fd_count`-length prefix. `.last()` on that full array always
returns index `FD_SETSIZE - 1`, regardless of how many entries are actually in use. For any set with
fewer than `FD_SETSIZE` active members (i.e., almost always), that slot holds whatever the array was
default-initialized to (0 in this repro, since the storage comes from `FdSetElement::default()`),
not the real last active fd. The correct swap-remove source is `set.fd_array[fd_count as usize - 1]`
(equivalently, `*fd_array.last().unwrap()` taken from the *slice* computed on the line above, before
`fd_count` is decremented) - the intended classic O(1) swap-remove, implemented against the wrong
array.

## Introduced by

8493118c "Implement `select`. (#1162)" (2024-10-11) - `git blame` on the exact line and `git log -S
"fd_array.last()"` both point to this single commit, which introduced the whole windows/wasi
`FD_SET`-array implementation of `fd_set_insert`/`fd_set_remove` from scratch; the bug has been
present unchanged through every subsequent refactor up to the current HEAD
(287214b889865d8e1406a0ee71cc409b6f6191c8).

## The fix

Index the real last active slot instead of the full array's last slot:

```rust
-            set.fd_array[pos] = *set.fd_array.last().unwrap();
+            set.fd_array[pos] = set.fd_array[fd_count as usize - 1];
```

(`fd_count` here is still the pre-decrement count, i.e. the number of active elements before this
removal, so `fd_count as usize - 1` is exactly the index of the last active element - the same
quantity `fd_array.last()` would have given if `fd_array` (the slice) had been reused instead of
re-deriving from `set.fd_array` directly.) `fd_count >= 1` is guaranteed whenever this line runs,
since `pos` was found by searching a slice of length `fd_count`, so the subtraction can't underflow.

Verified against the standalone `repro/`, built for `wasm32-wasip1` and run under wasmtime:
- Primary scenario (remove non-last fd from `[5, 10, 15]`): before `[5, 0]` (bug) -> after `[5,
  15]` (correct).
- Remove-last-element boundary case: `[1, 2]` both before and after (no regression - this case
  happened to already work under the bug).
- Drain-to-empty case: before `[0]` (stray bogus fd left behind) -> after `[]` (correct).

- Project test suite (`cargo test --features event --lib --tests`, stable toolchain, host
  `x86_64-unknown-linux-gnu`): 23 passed; 0 failed; 1 ignored, both before and after the patch - no
  regressions. (The native host test suite can't exercise the windows/wasi cfg branch itself; the
  `wasm32-wasip1` repro above is what actually drives the affected branch end-to-end.)

## What's not done / open questions

- `fd_set_insert`'s sibling windows/wasi branch (lines 141-153) was read alongside this one and
  does not share this bug - it only ever appends at `fd_array[fd_count]`, never swaps, so there's
  no equivalent wrong-index read there.
- Candidate 031 already confirmed a sibling panic in the non-windows/non-wasi (bitvector) branch of
  both `fd_set_insert`/`fd_set_remove` (negative `RawFd` indexing panic); that candidate explicitly
  did not exercise this windows/wasi `fd_array` branch, and this candidate (102) is the one that
  closes that gap.
- Not checked: whether any in-tree caller of `fd_set_remove` (e.g. `tests/event/select.rs`, or a
  hypothetical downstream crate) happens to always remove fds in last-inserted-first order, which
  would mask this bug in practice on real windows/wasi systems - the existing repro demonstrates
  the bug is reachable via the public API regardless of what any particular caller currently does,
  per "the reader decides what to prioritize" (`SKILL.md`'s "the one rule that matters most").
- Only tested under a real `wasm32-wasip1` target via wasmtime, not real Windows (no Windows
  environment available this session) - the two cfg arms (`windows`, `target_os = "wasi"`) share
  the exact same code path verbatim, and the `FD_SET`/`fd_count`/`fd_array` field layout assumption
  is identical on both (Windows's own `windows_sys::Win32::Networking::WinSock::FD_SET` has the
  same `fd_count: u32` + `fd_array: [SOCKET; FD_SETSIZE]` / `[i32; FD_SETSIZE]`-shaped layout), so
  there's no reason to expect the Windows arm to behave differently, but this wasn't independently
  verified on real Windows this session.
