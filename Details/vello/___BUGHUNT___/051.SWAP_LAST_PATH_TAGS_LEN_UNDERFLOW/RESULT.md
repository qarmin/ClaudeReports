# SWAP_LAST_PATH_TAGS_LEN_UNDERFLOW

`vello_encoding/src/encoding.rs:514-517`, public `Encoding::swap_last_path_tags` does
`let len = self.path_tags.len(); self.path_tags.swap(len - 1, len - 2);` with no guard that
`len >= 2` before the subtraction. All 3 current call sites in `vello/src/scene.rs`
(`draw_blurred_rounded_rect_in`, `fill`, `stroke`) only call it after `encode_shape`/
`stroke_gpu_inner` already pushed at least one `PathTag` and a subsequent `encode_transform` call
returned `true` (pushing exactly one more `PathTag::TRANSFORM`), so `len >= 2` holds today by
construction inside `vello`'s own scene builder - but `Encoding` (including its `path_tags: Vec<PathTag>`
field and `swap_last_path_tags` itself) is `pub` in the separately-published `vello_encoding`
crate, so any external consumer of that crate's public API can call `swap_last_path_tags` on an
`Encoding` with fewer than 2 path tags directly, with no unsafe/FFI/peer involved.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug-assertions build | `Encoding::new()` (0 path tags) and `Encoding::new()` + one pushed `PathTag` (1 path tag), calling `swap_last_path_tags()` directly via the crate's own public API, `cargo run` (dev profile) | Reproduces: `thread 'main' panicked at .../encoding.rs:516:29: attempt to subtract with overflow` for len=0, and the same panic (different column) for len=1 |
| Same inputs, `cargo run --release` | release profile | Reproduces with a different panic: `index out of bounds: the len is 0 but the index is 18446744073709551615` (the `len - 1`/`len - 2` wrap to `usize::MAX`/`usize::MAX - 1` under release's wrapping arithmetic, then `Vec::swap` panics on the out-of-range index) |
| Repeat-run check | 3x `cargo run` (dev) for the len=0 case | Panics identically all 3/3 runs - fully deterministic, no flakiness (pure arithmetic on a freshly constructed value, no timing/allocator dependence) |

No escalation beyond manual boundary-value construction was needed (per `HEURISTICS.md`, this is
exactly the "arithmetic on untrusted sizes/indices" class where a debug build plus 0/1/boundary
values is the whole toolset needed) - the very first approach already produced a deterministic,
100%-repeatable crash.

## Status

**Confirmed.** Reproduces 3/3 runs (debug build) for the len=0 case, and deterministically
(1/1, but the mechanism is pure arithmetic with no external state so it is not flaky) for the
len=1 case; release build reproduces via a different panic message (index out of bounds instead
of subtract-with-overflow), also deterministically.

**Fully reproducible: Yes.** `swap_last_path_tags` is a plain public library function on the
public `Encoding` type (not a peer/wire/FFI boundary) - the repro calls the real, public
`vello_encoding` API (`Encoding::new()`, the public `path_tags` field, `swap_last_path_tags()`)
exactly as any external crate consumer would, with no internal-only access and no second
participant to fake. `CONFIRMATION.md` section 11 does not apply here (nothing produced the input
from "the other side" - the caller directly controls the `Encoding`'s state, which is the entire
point of the finding: a caller *can* legally construct such state through the crate's own public
API).

## Class

logic / crash (unguarded arithmetic underflow leading to a panic; the debug build's overflow
check and the release build's OOB index check both catch it, but as an unhandled panic in either
case - not a silent memory-safety issue since `Vec::swap` bounds-checks in both profiles).

## Root cause

`swap_last_path_tags` (`vello_encoding/src/encoding.rs:514-517`) computes
`self.path_tags.swap(len - 1, len - 2)` without checking `len >= 2` first. For `len == 0`:
`len - 1` alone underflows (`usize` subtraction), panicking in debug builds with "attempt to
subtract with overflow"; in release builds it wraps to `usize::MAX`, and `len - 2` wraps to
`usize::MAX - 1`, so `Vec::swap(usize::MAX, usize::MAX - 1)` then panics with "index out of
bounds" instead (since `Vec::swap`'s own bounds check still fires - it is not UB, just a
different, less informative panic). For `len == 1`: `len - 1 == 0` is fine, but `len - 2`
underflows the same way. The 3 current callers in `vello/src/scene.rs` happen to never trigger
this because they only call `swap_last_path_tags` right after two path tags were just pushed
(one by `encode_shape`/`stroke_gpu_inner`, one by a `true`-returning `encode_transform`) - but
that invariant lives entirely in the caller's discipline, not in the function itself, and nothing
in `Encoding`'s public API prevents a different (or future) caller from invoking it with 0 or 1
path tags.

## Introduced by

dbe7f277 "Public encoding API" (2023-01-06). This commit ("This exposes significantly more of
the previously internal encoding guts") is the commit that first added `swap_last_path_tags` to
the codebase at all (verified: `git log -S "fn swap_last_path_tags" --all` finds only this commit;
the function did not exist, public or private, before it) - it introduced the function already
`pub` and already missing the `len >= 2` guard, exactly as it exists today (the file has since
moved from `src/encoding/encoding.rs` to `vello_encoding/src/encoding.rs` via
d2bd9514/db2fefdc's crate-reorganization commits, which only moved the file, not its content).

## The fix

Added an early return when `len < 2`, making the function a no-op instead of panicking when there
are fewer than 2 path tags to swap (there is nothing sensible to swap with 0 or 1 elements, so
leaving the stream unchanged is the correct behavior, not an arbitrary papering-over):

```rust
pub fn swap_last_path_tags(&mut self) {
    let len = self.path_tags.len();
    if len < 2 {
        return;
    }
    self.path_tags.swap(len - 1, len - 2);
}
```

Verified: re-ran `repro/` against the patched tree (`repro/` case "0" and case "1") - no panic in
either debug or release; also re-ran the len==2 case ("2") to confirm the fix does not change
behavior for the intended, already-guaranteed-safe call pattern (the two tags are still swapped
correctly). See `output.txt` for the full captured `just test` run (before: 2 panics + 1 pass;
after: 3 passes, correct swap in the len==2 case).

- Project test suite (`cargo test -p vello_encoding -p vello`): 31 passed, 0 failed before the
  patch; 31 passed, 0 failed after the patch (identical - the patch only changes behavior for the
  previously-panicking `len < 2` inputs, which no existing test exercises). `vello`'s own lib has
  0 unit tests; doc-tests unaffected (0 run / 1 ignored, same before and after).

## What's not done / open questions

- Only `vello_encoding`'s and `vello`'s own test suites were run (not the full workspace, e.g.
  `sparse_strips`/`vello_hybrid`/`vello_cpu`), since `swap_last_path_tags` and its only 3 callers
  are confined to `vello_encoding` and `vello/src/scene.rs`; no other crate in the workspace calls
  it.
- No other function in `vello_encoding` was audited for the same "unguarded `len - N` subtraction
  on a caller-controlled `Vec`" pattern this session - see `NewSuspectsNoticed` in the final report
  for one lead noticed in passing (`Encoding::encode_end_clip`'s `n_open_clips -= 1` is guarded by
  an `if self.n_open_clips > 0` check, so it was checked and ruled out, not left open).
- The fix chosen (silent no-op for `len < 2`) matches the only sensible interpretation of "swap
  the last two tags" when there aren't two tags, but a maintainer might instead prefer a
  `debug_assert!`-only guard (panic in debug, still-unsound in release) if they consider `len < 2`
  to always be a caller bug that should be loud rather than silent - this is a judgment call left
  open rather than decided unilaterally.
