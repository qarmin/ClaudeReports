# WithDiagnostics's TaskCounts/ThreadLifetimes raw-pointer aliasing UB

`TaskCounts::push` and `ThreadLifetimes::begin`/`end` in
`src/runner/runner_variants/with_diagnostics.rs:84-119` take `&self` and obtain a `&mut Vec<usize>`
/ `&mut Option<Instant>` for a given `th_idx` via a raw-pointer cast
(`&mut *(self.0.as_ptr().add(th_idx) as *mut Vec<usize>)`) on plain `Vec<Vec<usize>>` /
`Vec<Option<Instant>>` fields that are not wrapped in `UnsafeCell`. Per the Rust reference,
mutating data reachable only through a shared `&T` without interior mutability is undefined
behavior, independent of whether any two threads actually touch the same slot concurrently. This
looked like a good `miri` candidate: real `unsafe`, a raw-pointer cast bypassing the borrow
checker, and it's reachable end-to-end through the crate's own public API
(`.runner_with_diagnostics()`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Real end-to-end repro through the crate's public API (`repro/src/main.rs`, `multithread` bin): `(0..400).par().runner(Runner::fixed_with_pool(FixedPool(4))).num_threads(4).chunk_size(4).runner_with_diagnostics().map(...).reduce(...)`, run under `cargo +nightly miri run`, default aliasing model (Stacked Borrows), `MIRIFLAGS="-Zmiri-disable-isolation"` | 4 genuine `std::thread::scope` worker threads actually ran concurrently (confirmed via the diagnostics printout: "Used threads: 4", disjoint per-thread task counts), computed the correct sum (79800) both times. No Miri diagnostic of any kind was raised for the aliasing pattern. 2/2 identical runs. See `output.txt` "approach 1". |
| 2. Same repro, same binary, under Tree Borrows (`MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-tree-borrows"`) | Same result: correct output, no aliasing diagnostic, 2/2 identical runs. See `output.txt` "approach 2". |
| 3. Isolated single-threaded minimal reproduction of the *exact* code shape (`repro/src/bin/minimal.rs`, `minimal` bin) - `TaskCounts::push` copied verbatim, called twice on the *same* `th_idx` from the same `&TaskCounts` in one thread (no concurrency at all, so if this pattern is detectable by Miri's aliasing checkers independent of threading, this is the simplest possible witness) - under both Stacked Borrows and Tree Borrows | No diagnostic under either aliasing model, 2/2 runs each (4/4 total). Prints the expected `[[1, 2], [3]]`. See `output.txt` "approach 3"/"approach 3b". |

A 4th approach (ASAN/TSAN) was considered but not run: real code reading of the orchestration path
(`src/infallible/par_runner.rs`'s `next`/`fold`/etc., which drive `begin_thread`/`complete_chunk`/
`complete_thread`) confirms each `th_idx` value (`0..max_num_threads`) is claimed by exactly one
spawned thread for the lifetime of a single computation (`while let Some(th_idx) =
Self::do_spawn_new(spawned, st) { spawned += 1; ... }` - monotonically increasing, never reused
within one computation), so writes to different `th_idx` slots are always to disjoint byte ranges
even under real concurrency - there is no overlapping memory access for ASAN/TSAN to ever flag as a
race, so this tool would not add information beyond what approaches 1-2 already covered with a real
race detector built into Miri's thread scheduler (which also raised nothing here).

## Status

**Not reproducible.** Tried 3 escalating approaches (a real end-to-end multithreaded repro through
the crate's own public API under Miri/Stacked Borrows, the same under Miri/Tree Borrows, and an
isolated single-threaded minimal reproduction of the exact code pattern under both aliasing models)
- all four (2 models x 2 repro shapes) came back clean, each repeated twice with identical results
(8/8 runs total, no diagnostic).

Root-causing *why* Miri doesn't catch this (rather than assuming it's a false alarm) matters here:
`self.0.as_ptr()` returns a raw pointer that was already stored as data inside the `Vec<Vec<usize>>`
struct (its `ptr: NonNull<_>` field) at construction time. Miri's retagging (both Stacked Borrows and
Tree Borrows) is triggered by reference-typed values, and is shallow through raw-pointer fields: when
`push`'s `&self` parameter gets retagged, that only assigns a fresh tag to the bytes of `*self` (the
`TaskCounts` struct's own header, which is the Vec's `ptr`/`len`/`cap` fields) - not to the *heap
allocation* that `ptr` points at, since that's one more level of indirection through a raw pointer,
which Miri's retag pass does not recurse into. The heap buffer's own borrow-tracking state is
whatever it was left at after the last actual reference-typed access to it (in practice, a
permissive tag from when `TaskCounts::new()` originally built the outer `Vec` via unique/owned
access, since it's then only ever touched again through this same raw-pointer path). No competing
access ever forces that tag to be downgraded, so there is nothing for the per-location aliasing
check to flag - this specific pattern is a real, verified blind spot of Miri's current aliasing
models for "a `&self`-shared type whose only mutation path is a raw pointer computed via
`Vec::as_ptr()`/`.add(i)` into its own backing store", not a limitation specific to this candidate's
threading. This matches (and gives a mechanistic explanation for) `findings.toml`'s own caveat that
"in practice each `th_idx` is only ever touched by one thread at a time" - there is no dynamically
observable symptom (no data race, no aliasing-checker diagnostic) for any tool available in this
session to catch, even though the code is still textbook UB by the letter of the aliasing rules
(mutating data behind `&T` without `UnsafeCell`).

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed).

## Class

N/A (Not reproducible).

## Root cause

Not applicable in the "confirmed bug" sense - see the mechanistic explanation above for why the
*tools* available this session cannot detect this. The underlying code pattern itself is real and
matches `findings.toml`'s description exactly (verified by reading
`src/runner/runner_variants/with_diagnostics.rs:84-119` directly): `TaskCounts`/`ThreadLifetimes`
mutate through raw pointers derived from `&self` with no `UnsafeCell`, and per-thread writes are
disjoint by construction (confirmed via `src/infallible/par_runner.rs`'s `th_idx` allocation logic:
each spawned thread receives a strictly-increasing, never-reused `th_idx` for the lifetime of one
computation), so it is real-but-currently-undetectable-by-dynamic-tooling UB rather than a
crash/logic bug with an observable symptom.

## Introduced by

Not looked up - this candidate never reached `Confirmed`, and `SKILL.md`'s "Introduced by" lookup
is only for confirmed findings.

## The fix

Not attempted - no verified reproduction to attempt a fix against or verify a fix relative to (per
`SKILL.md` step 6, a bounded fix is only attempted for a `Confirmed` candidate). For the record,
`findings.toml`'s own suggested fix (wrap the per-slot storage in `UnsafeCell`, e.g.
`Vec<UnsafeCell<Vec<usize>>>` / `Vec<UnsafeCell<Option<Instant>>>`, or switch to
`Vec<Mutex<...>>`/atomics) is a reasonable direction if a maintainer wants to remove the
theoretical-UB status regardless of whether any tool can currently observe it - this is a
correctness-hygiene fix (defense against future compiler versions/optimizations, not a fix for an
observed bug), left to the project's own discretion since nothing here demonstrates it changes any
currently-observable behavior.

## What's not done / open questions

- No optimizer-exploitation angle was attempted (constructing a release/LTO build where LLVM's
  `noalias`/`readonly` attributes on `&self`-typed parameters might let it cache/reorder reads
  across the raw-pointer mutation) - a quick check of the emitted LLVM IR for the isolated
  `TaskCounts::push` pattern (`cargo rustc --release -- --emit=llvm-ir`, `opt-level=3`,
  `#[inline(never)]` to keep the function boundary) showed no `readonly`/`noalias` attribute
  attached to the `self` parameter in this exact shape, so a call-site miscompilation exploiting
  those attributes seems unlikely for current rustc/LLVM versions, but this was not exhaustively
  explored (e.g. under LTO across a real crate boundary, or a future/different LLVM version) and
  is not a proof of absence.
- Whether a newer/different UB-detection tool (e.g. a future Miri release, or GenMC-style
  exhaustive interleaving exploration) would catch this pattern was not checked - worth retrying if
  Miri's retagging ever grows deeper (non-shallow) tracking through raw-pointer fields.
- `ThreadLifetimes::begin`/`end` share the exact same code shape as `TaskCounts::push` (a raw
  pointer computed via `.as_ptr().add(th_idx)` on a plain `Vec`, cast to `*mut`, dereferenced as
  `&mut`) - not repeated as a separate repro since the isolated `minimal.rs` test already
  establishes the pattern itself is undetectable by Miri regardless of which field it's applied to.
