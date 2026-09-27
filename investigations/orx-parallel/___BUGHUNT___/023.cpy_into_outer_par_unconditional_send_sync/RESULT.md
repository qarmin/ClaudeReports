# Cpy's unconditional `unsafe impl Send/Sync` inside `into_outer_par`

`src/infallible/recursive/utils.rs:38-44` defines a local helper struct `Cpy<T> { src: *mut T,
dst: *mut T, len: usize }` inside `into_outer_par<'a, T, M, F, R>` with `unsafe impl<T> Send for
Cpy<T> {}` and `unsafe impl<T> Sync for Cpy<T> {}`, unconditionally - `into_outer_par` itself has
no `T: Send` bound anywhere in its own signature. `Cpy<T>` instances are then driven through the
crate's real thread pool via `ops.into_par().runner(runner).for_each(|op: Cpy<T>| { ...
copy_to_nonoverlapping ... })` (utils.rs:67-71), i.e. genuine OS worker threads. Same shape as the
already-confirmed candidates 002 (`UseSlice`) and 003 (`PairPtr`); flagged for a closer look
because the worker closure here only performs a raw `copy_to_nonoverlapping` (no `T::Clone`/
`T::Drop` executed off-thread), unlike 002/003 where worker threads perform a typed `&mut T`
access / a real `Use::Item` value operation.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading / call-graph trace: every current caller of `into_outer_par` (`collect.rs`, `collect_arb.rs`, `fold.rs`, `next.rs`, `next_any.rs`, `reduce.rs`, all in `src/infallible/recursive/execution/`) | Read each function's own public generic bounds | Every one of the six callers already requires `X::I: Send + Sync` (or, for `collect`/`collect_arb`, `Elem<X::I>` whose own auto-derived `Send`/`Sync` in turn requires `X::I: Send`) directly in its own `where` clause, before ever reaching `into_outer_par`. `into_outer_par` and its containing modules (`mod recursive;`, `mod utils;` in `src/infallible/mod.rs` / `src/infallible/recursive/mod.rs`) are both private - not reachable from outside the crate at all. So no current call site can actually instantiate `into_outer_par::<T, ...>` with a genuinely `!Send`/`!Sync` `T` - unlike 002/003, where the public `use_slice` entry point's own bound is `Sync` only (not `Send`), which is what made those two exploitable through the real public API |
| Manual boundary-value construction, bypassing the reachability gap above by calling `into_outer_par` directly from a `#[cfg(test)]` module added inside the same file (same-crate visibility rules let code inside `recursive`'s own descendant modules name the private `utils::into_outer_par` directly): witness type `Rc<Cell<i64>>` (genuinely `!Send`/`!Sync`, non-atomic refcount), `n_locals=8` per-thread accumulators of 200 clones each of one shared `Rc`, driven through `into_outer_par` via the crate's real `default_runner()`/global thread pool | `cargo test --release --lib`, real OS worker threads (confirmed via a `bughunt023-thread {:?}` print of `std::thread::current().id()` inside the real `Cpy` closure - distinct real `ThreadId`s observed every run, e.g. 16-24 distinct IDs per run for a larger workload in a companion test), 50 iterations per test run, checked after each: total length, `Rc::strong_count`, `Rc::ptr_eq` against the shared allocation for every element, `Cell::get()` value, and that every local was fully drained | All 50 iterations passed on every run: no corrupted `Rc::strong_count`, no misplaced/aliased pointer, no corrupted value, no crash. Reran the whole test 3 more times against a genuinely fresh clone via this candidate's own `justfile`/`probe.patch` (`just probe reproduce`) - 3/3 fresh-clone reruns identical (see `output.txt`) |
| `cargo +nightly miri test` (`MIRIFLAGS=-Zmiri-ignore-leaks`, Stacked Borrows default) on the same probe module | Same witness, same assertions, run under Miri | Both the witness test and a companion real-concurrency-confirmation test passed with no UB/aliasing/data-race diagnostic of any kind (see `output.txt`) |

No 4th tool was needed: the soft cap of 3 approaches was reached with three consistent negative
results (a reachability trace showing no current call site can pass a `!Send` `T` at all, plus two
independent runtime checks - native with real OS threads, and Miri - of what happens if the
function is called directly with one anyway), and none of them found even a partial signal to keep
digging on.

## Status

**Not reproducible.**

Two independent facts both point the same way, and neither alone would be sufficient on its own to
close this out, which is why both were checked:

1. **Not reachable today.** Every current caller of `into_outer_par` already establishes `T: Send`
   (directly or via `Elem<T>`'s auto-derived bound) at its own function signature before calling
   it, and `into_outer_par`/its containing modules are private - so there is no way to reach it
   from outside the crate, and no way to reach it with a genuinely `!Send` `T` from inside the
   crate's current code either.
2. **Even when forced to run anyway, nothing breaks.** Bypassing (1) by calling `into_outer_par`
   directly (from a `#[cfg(test)]` module with same-crate visibility into the private path) with a
   real `!Send`/`!Sync` witness (`Rc<Cell<i64>>`), driven through the crate's real thread pool with
   confirmed genuine multi-OS-thread dispatch, produced no corruption, no crash, and no Miri
   diagnostic across 50 native iterations x 4 runs (1 original + 3 fresh-clone reruns) plus one
   Miri run. This is consistent with the root-cause explanation below: the specific operation
   `Cpy<T>` is used for (a disjoint, non-overlapping `ptr::copy_to_nonoverlapping` byte-level
   relocation, never a typed `&T`/`&mut T` read, `Clone`, or `Drop` executed on the worker thread)
   does not actually depend on `Send`/`Sync` for its own safety, unlike 002/003 where worker
   threads perform a real typed operation on `T`'s value (an application-supplied `Clone`/mutation
   through `&mut T`) that a non-atomic refcount or similar invariant can race on.

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed).

## Class

N/A (Not reproducible).

## Root cause

Not applicable in the "confirmed bug" sense. The code pattern itself is real (verified by reading
`src/infallible/recursive/utils.rs:38-44` directly: `Cpy<T>` gets an unconditional `unsafe impl
Send`/`Sync` with no bound on `T`, matching the same shape as candidates 002/003), but this
candidate diverges from those two at the exploitation step, for two independent reasons:

1. **No current caller passes an unbounded `T`.** All six of `into_outer_par`'s current callers
   (`src/infallible/recursive/execution/{collect,collect_arb,fold,next,next_any,reduce}.rs`) bound
   their own generic element type (`X::I`) with `Send + Sync` directly in their own public function
   signatures, and `into_outer_par` itself (and the `recursive`/`utils` modules containing it) is
   never exported from the crate (`mod recursive;` is private in `src/infallible/mod.rs`). So even
   though `into_outer_par`'s own signature has no `T: Send` bound, every actual instantiation of it
   in the current codebase already has one imposed from the caller's side.
2. **The operation performed on `T` by the worker thread doesn't need `Send` anyway.** `Cpy<T>`
   only carries two raw pointers and a length; the worker closure (utils.rs:68-71) destructures it
   and calls `src.copy_to_nonoverlapping(dst, len)` - a disjoint, non-overlapping bytewise memcpy
   that never dereferences `T` as a typed value, never calls any of `T`'s trait methods (no
   `Clone::clone`, no `Drop::drop`), and never lets two threads observe the same memory
   concurrently (each local's `src` range and the corresponding disjoint `dst` range in `outer` are
   computed once, up front, on the calling thread, before any worker starts; `local.set_len(0)` is
   also called on the calling thread before dispatch, so the original backing bytes are never
   written again until a possible later reuse, which only happens after this `for_each` call fully
   synchronizes). `Drop` on the relocated values is only ever invoked later, by whichever thread
   owns the `outer`/`data` `Vec`s afterward - which in every current caller is the single "driver"
   thread running the whole `collect`/`fold`/`reduce`/etc. call, never a worker thread. A relocation
   like this (moving a value's raw representation between two memory addresses without executing
   any of its code) does not exercise the invariant `Send`/`Sync` exist to protect (concurrent or
   cross-thread *use* of the value through its own API, e.g. a non-atomic refcount increment/
   decrement) - which is exactly what the empirical witness test in this candidate confirms: no
   corruption, no aliasing violation, under either native execution with real OS threads or Miri.

This is a real static-analysis-level laxity (the unconditional `unsafe impl` is broader than it
needs to be, and would become directly relevant if a future change ever let a `!Send` `T` reach it,
or if a future refactor changed what the worker closure does with `T` beyond a raw relocation), but
it is not, in the current codebase, an exploitable soundness hole.

## Introduced by

Not determined - not attempted. Per `SKILL.md`, "Introduced by" is scoped to `Confirmed` findings;
this candidate did not reach that status.

## The fix

No fix was written - there is no reproducible defect to verify a fix against or revert cleanly
(per `SKILL.md`, a bounded fix is only attempted for `Confirmed` candidates). For the record, a
defense-in-depth tightening mirroring 002/003's own fix would be straightforward if a maintainer
wants one purely as a preventative measure against a future caller/refactor reintroducing real
exploitability: `unsafe impl<T: Send> Send for Cpy<T> {}` / `unsafe impl<T: Sync> Sync for Cpy<T>
{}` (or simply `T: Send + Sync`, since `Cpy<T>` only ever holds raw pointers, not a live `T`, so
either bound would compile against every current caller unchanged) plus a matching `T: Send`/`Sync`
addition to `into_outer_par`'s own `where` clause so the requirement is visible at the call site
instead of silently enforced only by each caller's own separate, currently-coincidentally-matching
bound. This was not applied or verified this session, since there is nothing observable for it to
fix and applying/reverting an unverified speculative patch would violate the "main project's
working tree is always left clean" rule for a candidate that didn't confirm.

- Project test suite: not run before/after, since no patch was applied (N/A - "before"/"after"
  both mean the identical, unmodified tree here).

## What's not done / open questions

- If `into_outer_par` (or the `recursive`/`utils` modules) is ever made `pub`, or if any future
  caller passes an element type without first establishing `Send` at its own signature, this
  candidate should be re-checked immediately - the reachability gap (point 1 above) is a
  coincidence of the current call graph, not something `into_outer_par`'s own type signature
  enforces.
- If a future change to the worker closure (utils.rs:68-71) ever starts performing a typed
  operation on `T` (e.g. calling `Clone`/`Drop`/reading through a `&T`/`&mut T` reference instead of
  a raw disjoint memcpy), point 2 above would no longer hold and this candidate should also be
  re-checked - the current "safe regardless of Send" conclusion is specific to what the worker
  closure does today, not a general property of `Cpy<T>`'s shape.
- The witness test used `Rc<Cell<i64>>` (non-atomic refcount, the same witness shape as candidates
  002/003) to stay consistent with those; a `!Send`-but-`Sync` witness (e.g. something modeling
  `MutexGuard`'s thread-affinity-on-drop requirement) was not separately constructed, since `Drop`
  is never invoked on the worker thread in this code path regardless of which witness type is used
  (see root cause point 2) - this was judged unlikely to produce a different result and out of
  scope for the soft 3-approach cap.
- `into_outer` (the sequential sibling at utils.rs:5-26, `#[allow(dead_code)]`) was read for
  comparison and does not use `Cpy`/any `unsafe impl Send`/`Sync` at all - it is single-threaded
  and not part of this candidate's suspect area.
