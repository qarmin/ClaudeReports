# fd_set_insert_negative_rawfd_panic

`src/event/select.rs`'s `fd_set_insert`/`fd_set_remove` (both fully safe, non-`unsafe` public
functions backing `FD_SET`/`FD_CLR`) take a `fd: RawFd` (plain `i32`, trivially negative) and, on
every non-Windows/non-wasi target, do `let fd = fd as usize; fds[fd / BITS]` with no validation at
all. A negative `fd` (e.g. a caller forwarding an unchecked `-1` error sentinel, a very common
POSIX/C convention for "no valid fd") wraps to a huge `usize`, producing an out-of-bounds slice
index - a process abort reachable through a fully safe public API, unlike the sibling `select()`
which is `unsafe` for a different, documented reason (I/O safety of the fds already in the sets,
not index validation).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `src/event/select.rs` | n/a | Confirmed both `fd_set_insert` (line 134-139) and `fd_set_remove` (line 158-163) cast `fd as usize` unconditionally before indexing `fds[fd / BITS]`, with no bounds/sign check anywhere in either function or their callers |
| Manual boundary-value construction: standalone scratch crate calling the real, unmodified, public `rustix::event::fd_set_insert`/`fd_set_remove` directly with `fd = -1`, `-2`, `i32::MIN`, debug build | `cargo run` (dev profile), 5 repeat runs | Panicked 5/5 runs, deterministically: `index out of bounds: the len is 4 but the index is 288230376151711743` at `select.rs:138:9` (insert) / `:162:9` (remove) - `288230376151711743 == (-1i32 as usize) / BITS` on this 64-bit host |
| Same boundary values against a `--release` build | `cargo run --release` | Panicked identically - this is a plain Rust slice-bounds-check panic (always enforced, not an `overflow-checks`-gated debug-only assertion), so release vs. debug makes no difference here |
| Extra boundary case: a positive but too-large `fd` for the caller's own (correctly-sized-for-a-smaller-`nfds`) storage | 1-element `[FdSetElement; 1]`, `fd = 100_000` | Also panicked (`index out of bounds: the len is 1 but the index is 1562`) - same root cause, no sign conversion even needed, just a plain OOB index |

## Status

**Confirmed**. Fully deterministic - 5/5 runs in a debug build, and separately reproduced in a
release build and via the standalone `repro/` against a completely fresh clone (2/2 full
`just test` runs, see `output.txt`). This is a pure arithmetic/indexing bug (Rust's slice bounds
check is always enforced, debug or release), so there is no layout/seed/timing dependence at all -
every call with an out-of-range `fd` panics every single time.

**Fully reproducible**: **Yes**. `repro/` calls the real, unmodified, public
`rustix::event::fd_set_insert`/`fd_set_remove` API directly with a negative `RawFd` - a negative
`RawFd` is itself the genuine external input (a caller-supplied `i32`, nothing more), not a stand-in
for a second participant's reaction to anything: there is no peer/wire/upstream-caller input being
faked here, so `CONFIRMATION.md` section 11's two-participant bar does not apply (this is the same
"single external artifact handed to a public entry point" shape as a malformed file handed to a
parser's real `open_file`).

## Class

crash

## Root cause

`fd_set_insert`/`fd_set_remove`'s non-Windows/non-wasi branch (`src/event/select.rs:134-153` /
`:158-176`) computes `fds[fd as usize / BITS]` directly from the caller-supplied `RawFd` with no
validation that `fd` is non-negative or that the resulting index actually fits within the
caller-provided `fds` slice. `RawFd` is a plain `i32`; any negative value (most commonly `-1`, the
standard POSIX/C "invalid fd" sentinel many APIs return on error, or a type-confused value from an
upstream bug) reinterprets as a huge `usize` via the `as` cast, and indexing a 4-element array (or
any real-world-sized `FdSetElement` array) at that index is always out of bounds, so the function
panics via Rust's own slice-bounds check. Both functions are declared as ordinary safe `pub fn`s
(not `unsafe fn`), so this is a safe-API panic reachable purely from a caller's logic mistake, not
from violating any `unsafe` contract - unlike `select()` itself in the same file, which is `unsafe`
specifically because of a different invariant (all fds in the sets must already be open), with no
mention of index-range safety at all.

## Introduced by

8493118c "Implement `select`. (#1162)" (2024-10-11) - this commit introduced `select`,
`fd_set_insert`, `fd_set_remove`, and the rest of this module, with the exact
`let fd = fd as usize; fds[fd / BITS]...` shape in both functions from the start (confirmed via
`git log -S "let fd = fd as usize" -- src/event/select.rs`, which returns only this one commit, and
`git show 8493118c -- src/event/select.rs`, which shows both occurrences being added fresh, not
modified from a prior version).

## The fix

Guard the index computation in both functions: convert `fd` to `usize` via `usize::try_from` (fails
cleanly for any negative value instead of wrapping), then use `fds.get_mut(...)` instead of direct
indexing so an index that's in-range-after-conversion but still too large for the caller's own
`fds` slice is also handled safely. Both cases now silently have no effect instead of panicking -
documented explicitly in both functions' doc comments (`If fd is negative, or doesn't fit within
fds, this has no effect.`):

```rust
pub fn fd_set_insert(fds: &mut [FdSetElement], fd: RawFd) {
    #[cfg(not(any(windows, target_os = "wasi")))]
    {
        if let Ok(fd) = usize::try_from(fd) {
            if let Some(element) = fds.get_mut(fd / BITS) {
                element.0 |= 1 << (fd % BITS);
            }
        }
    }
    ...
}
```

(identical shape for `fd_set_remove`, with `&= !(1 << ...)` instead of `|=`). See `patch.diff` for
the full diff (both functions, plus the new doc-comment lines).

Verified:
- Re-ran the exact `repro/` against the patched tree (both in-place in the working copy and via a
  completely fresh `just test` clone, see `output.txt`): no panic for `fd = -1, -2, i32::MIN`, and
  for the separate too-large-positive-fd case (`fd = 100_000` against a 1-element array) - all
  seven negative/out-of-range sub-cases that previously panicked now report `completed, no panic`.
- Checked the fix against the normal in-range case too (CONFIRMATION.md section 7, not just the
  failing input): `fd_set_insert(&mut fds, 5)` followed by `FdSetIter::new(&fds).collect()` still
  shows `[5]`, and `fd_set_remove(&mut fds, 5)` then shows `[]` - the legitimate case is byte-for-
  byte unchanged.
- Project test suite, default features, before and after the patch (same `cargo test --workspace`
  invocation both times): **22 passed; 0 failed; 1 ignored** (plus 7 doc-tests, also unaffected) -
  identical before and after, no regressions, no new failures.

## What's not done / open questions

- The sibling Windows/wasi branch of both functions (the `FD_SET`/`fd_array` code path) was not
  investigated this session - it has a structurally different implementation (a dynamic
  `fd_count`/`fd_array` pair behind a raw pointer cast) and wasn't exercised at all here (this
  container is Linux-only); it may or may not share an analogous unguarded-index issue and would
  need a Windows or wasi environment to check for real.
- `fd_set_bound` (same file, a few lines below) also indexes `fds` directly but only ever with
  indices derived from `fds.len()` itself (`fds.iter().rposition(...)`), not from a caller-supplied
  `fd` - reviewed and confirmed not to share this bug shape, not a separate candidate.
- No fuzzing was used (and none was needed): this is a direct, fully deterministic boundary-value
  bug with no complex input space to explore - the three escalating-tool soft cap in `SKILL.md`
  wasn't needed past the first (manual boundary-value construction) approach, which found the bug
  immediately and with total certainty.
