# PairPtr unconditional `unsafe impl Send`

`src/use_var/pair_ptr.rs:1-7` has `unsafe impl<U, V> Send for PairPtr<U, V> {}` with no `U: Send,
V: Send` bound. `PairPtr` is built in `UseFold::init_get` (`src/use_var/use_fold.rs`) from a raw
pointer into the base `Use`'s per-thread item (`U::Item`) and a pointer into the fold accumulator
(`T`), and is used as `Use::Item` for the `.fold()` combinator (`src/infallible_use/par.rs`). The
`Use` trait (`src/use_var/use.rs`) never bounds `type Item: Send`, and neither `UseFold`'s own
generic bounds nor `.fold()`'s own signature require `Self::Use: Send` - so a `Use` impl whose
`Item` is `!Send` can flow, unchecked, through `PairPtr`'s blanket impl. Source:
`___FINDINGS___/findings.toml` (use_after_free, easy); backlog entry in `IDEAS.md`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: witness `Sync + !Send` type (`GuardLike`, modeled on `std::sync::MutexGuard` - `Sync` via `unsafe impl`, `!Send` via a `PhantomData<*const ()>` field), driven through the crate's real public `Par::use_slice(...).fold(...)` API with `num_threads(8)` | `cargo build`/`cargo run --release`, 8 real OS worker threads, 4096 items | Compiled and ran to completion 5/5 native runs; each of the 8 `GuardLike` slots ended up exclusively mutated by a distinct real OS `ThreadId` (never the constructing/main thread's), confirming the per-thread state genuinely crosses real OS thread boundaries with no `Send` bound ever checked |
| Compile-time delta test: temporarily tightened `PairPtr`'s impl to `unsafe impl<U: Send, V: Send> Send for PairPtr<U, V> {}` (no other change) and rebuilt both the crate itself and the witness program | `cargo build --release` | The crate's own `use_fold.rs:46` (`self.cache.set_value(thread_idx, pair_ptr)`) now fails with `E0277: <U as Use>::Item cannot be sent between threads safely`, pointing directly at `orx_concurrent_ordered_bag::ConcurrentOrderedBag::set_value`'s own `where T: Send` bound - proving `PairPtr`'s unconditional impl is precisely what launders a `!Send` `Use::Item` past that dependency's own Send-safety contract. The witness program itself still compiled (`UseFold`/`fold()` weren't yet bounded), demonstrating the hole is reachable purely from `PairPtr`'s own impl |
| `miri` (nightly, `-Zmiri-ignore-leaks`) on the same witness program | `cargo +nightly miri run --release`, default thread scheduling (miri ran the workload across 2 distinct `ThreadId`s rather than 8, consistent with miri's own thread/scheduling model) | Ran to completion with no data-race/UB diagnostic. This is expected, not a contradiction: no code path in this crate ever lets two OS threads *alias* the same `Use::Item` concurrently (each `thread_idx` maps to one exclusive slot) - the violation here is a type-level Send-safety contract bypass (an ownership handoff across a real thread boundary for a type that must not allow it), not a data race Miri's memory model would flag. Recorded for completeness per HEURISTICS.md's tool selection, not as an additional independent signal |

## Status

**Confirmed**. Reproduces 5/5 native runs (release build, no fixed seed - see `output.txt`), plus a
deterministic (not flaky - it's a compile-time check) delta test that isolates `PairPtr`'s specific
contribution to the hole, run twice with identical results.

**Fully reproducible**: **Yes**. `repro/` drives the crate's real, public
`Par::use_slice(...).fold(...)` API (not a hand-built stand-in for it), executed by the crate's own
real default thread pool spawning genuine OS worker threads. This is a single-participant finding
(how the crate's own generic `.fold()` API reacts to a caller-supplied `Use` implementation whose
`Item` type is `Sync`-but-`!Send`), not an interaction between two independent programs/peers, so
`CONFIRMATION.md` section 11's "real second participant" requirement does not apply (per that
section's own carve-out: a caller-supplied type parameter handed to a real, documented generic
public API is the genuine external input; there is no "other side" being impersonated). The witness
type `GuardLike` is not a fake peer - it models a real-world `Sync`-but-`!Send` type
(`std::sync::MutexGuard` is the standard-library example of exactly this shape) the same way any
downstream caller's own type could be.

## Class

memory-safety (unsound `unsafe impl Send` - a Send-safety-contract bypass reachable through the
crate's real public API, verified against a real dependency's own `T: Send` safety bound rather than
merely "the type system would normally require this")

## Root cause

`PairPtr<U, V>` exists purely to smuggle two raw pointers (one into the base `Use`'s per-thread
item, one into the fold accumulator) across the internal machinery `.fold()` uses
(`orx_concurrent_ordered_bag::ConcurrentOrderedBag`) so that `UseFold` can implement `Use` for
`&mut UseFold<U, T, F>` with `Item = PairPtr<U::Item, T>`. `ConcurrentOrderedBag::set_value` (a
dependency this crate relies on) has its own explicit `where T: Send` bound, because its documented
contract is: a value written into the bag by one thread may later be read/dropped by a different
thread, which is exactly the operation `Send` exists to gate. `PairPtr`'s blanket
`unsafe impl<U, V> Send for PairPtr<U, V> {}` satisfies that bound unconditionally, regardless of
whether `U`/`V` (in practice, `U = U::Item`, the base `Use`'s per-thread state) are actually safe to
hand off between threads. Since neither the `Use` trait nor `UseFold`'s own generics require
`Item: Send`, and `.fold()`'s own signature never required `Self::Use: Send` either, nothing in the
crate stops a caller-supplied `Use` implementation whose `Item` is a genuinely thread-affine,
`!Send` type (a lock guard, a thread-local-arena handle, or any other type whose safety depends on
staying associated with one specific OS thread) from being driven through `.fold()` across real
worker threads with the compiler never checking that requirement.

This is only reachable via the public API today in conjunction with the separately-confirmed
candidate 002 (`___BUGHUNT___/002.use_slice_sync_missing_send/`, `UseSlice`'s own unconditional
`unsafe impl Sync`): `Par::use_slice` is currently the *only* one of the three public entry points
(`use_new`/`use_vec`/`use_slice`) whose bound (`U: Sync`, not `U: Send`) lets a `!Send` `Use::Item`
into a pipeline at all - `use_new`/`use_vec` both require `U: Send` directly. That does not make this
finding a duplicate of 002: `PairPtr`'s own blanket impl is an independent piece of unsound API
surface (proven above by the delta test, isolated from `UseSlice` entirely) that silently defeats a
real dependency's own Send-safety contract, and would become newly exploitable on its own the moment
any other code path - a future combinator, a future custom `Use` implementation, or any future fix to
candidate 002 that still allows some other route to a `!Send` `Use::Item` - hands `UseFold` a
`!Send` item; nothing in `PairPtr`'s own definition or `UseFold`'s bounds would stop it.

## Introduced by

195bd70f "implement XapUsePair" (2026-06-01) - this commit introduced `src/use_var/pair_ptr.rs`
with the unconditional `unsafe impl<U, V> Send for PairPtr<U, V> {}` already present, unchanged
since (confirmed via `git log -S "unsafe impl<U, V> Send for PairPtr" -- src/use_var/pair_ptr.rs`
and `git log --follow` on the file).

## The fix

Three coordinated changes, all verified together (a `PairPtr`-only fix does not compile - see
below):

1. `src/use_var/pair_ptr.rs`: `unsafe impl<U, V> Send for PairPtr<U, V> {}` ->
   `unsafe impl<U: Send, V: Send> Send for PairPtr<U, V> {}`.
2. `src/use_var/use_fold.rs`: added `U::Item: Send` to `UseFold`'s struct/impl/`Use`-impl bounds
   (required once (1) is in place, since `UseFold` is itself generic over any `U: Use` and must
   prove `PairPtr<U::Item, T>: Send` at `self.cache.set_value(...)`, not just at whatever the
   eventual caller happens to instantiate it with).
3. `src/infallible_use/par.rs`: added `Self::Use: Send` to `.fold()`'s own `where` clause (the
   public-API-facing bound; without it, `fold()`'s internal `UseFold::new(u, ...)` call fails to
   satisfy (2) for any `ParUse` impl whose `Self::Use` isn't already known-Send).

Verified: re-running `repro/`'s `pair-ptr-send-repro` against the patched tree now fails to compile
with `error[E0277]: ... cannot be sent between threads safely`, pointing directly at the new
`Self::Use: Send` bound on `ParUse::fold` (see `output.txt`'s `reproduce-after` section) - the
correct outcome for a Send-soundness fix is rejecting the unsound program at compile time, not a
runtime behavior change. Checked against a second input in the same class per `CONFIRMATION.md`
section 7: a plain `.reduce()`/`use_slice` pipeline (not going through `.fold()`/`PairPtr` at all)
with the same `!Send` witness type still compiles unchanged after this fix - correctly showing the
fix is scoped to `.fold()`/`UseFold`/`PairPtr` specifically and does not (and is not intended to)
touch the separate, already-tracked `UseSlice` issue (candidate 002).

- Project test suite (in the isolated worktree, `cargo test --release`, doctests + unit + all
  integration test binaries): **440 passed, 0 failed** before the fix; **440 passed, 0 failed**
  after the fix (identical - every existing caller already uses a `Send` `Self::Use`, so the new
  bounds change nothing for any currently-supported usage).

## What's not done / open questions

- This finding's practical exploitability today is gated behind candidate 002 (`UseSlice`'s own
  unconditional `Sync` impl) being the only current entry point for a `!Send` `Use::Item` - see
  "Root cause" above for why this is still a distinct, independently-worth-fixing hole rather than
  a duplicate. Both candidates' patches are independent and can be applied together without
  conflict (verified: `patch.diff` here only touches `pair_ptr.rs`/`use_fold.rs`/`infallible_use/
  par.rs`; 002's patch touches `use_slice.rs`/`infallible/par.rs` and its `option`/`result`
  siblings - no overlapping lines).
- Did not attempt to construct an independent, PairPtr-exclusive hardware-level crash (distinct from
  002's already-confirmed `Rc` refcount corruption via `UseSlice`) - analysis suggests this isn't
  currently possible via the public API, since `use_slice` is the only route in and any exploit
  necessarily also exercises 002's hole first. If 002 is fixed independently first, a follow-up
  session should re-check whether any *other* future entry point (a new combinator, a relaxed bound
  elsewhere) reopens a way to trigger this specific hole with a genuinely observable runtime
  consequence.
- `option_use`/`result_use` variants of `.fold()` were checked (`grep -rn "fn fold"`) and do not
  exist in this codebase (only `infallible_use::par::ParUse::fold` defines `.fold()`), so no sibling
  fix is needed there.
