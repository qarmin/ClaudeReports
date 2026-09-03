# XapSync unconditional `unsafe impl Sync` without an `X: Sync` bound

`src/infallible/recursive/xap_sync.rs:27` declares `unsafe impl<X: Xap> Sync for XapSync<X> {}`
unconditionally, but the public `Xap` trait (`src/infallible/xap.rs:6`) is only bounded
`Copy + Send`, never `Sync`. Every recursive execution entry point
(`src/infallible/recursive/execution/{reduce,fold,collect,collect_arb,next,next_any}.rs`) wraps
the caller-supplied `Xap`/closure chain in `XapSync::new(xap)` once per BFS "wave", then captures
it **by reference** (no `move`) into a `Fn`-closure passed to `.for_each()`/`.flat_map()`/`.reduce()`,
which the crate's own real thread pool (`std::thread::scope`-based, real OS worker threads) then
calls concurrently from every spawned worker for that wave - i.e. every worker thread calls
`Xap::xap(&self, ...)` on the exact same shared instance, not its own copy. This is structurally
different from every sibling `Xap`-implementing wrapper (e.g. `FnMap` in
`src/infallible/fun/map/basic.rs:15`, and the crate's own *non-recursive* execution path in
`src/infallible/par_runner.rs`), which only ever add `unsafe impl Send`, and give each spawned
worker thread its own bitwise `Copy` of the closure instead of sharing one instance.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: witness type `Racy(*mut i64)` (`#[derive(Clone, Copy)]`, `unsafe impl Send` only, deliberately no `Sync`), driven through the real public `par_recursive(...).num_threads(8).map(h).for_each(...)` entry point, `h` capturing `Racy` pointing at a stack-local counter and doing a non-atomic read-yield-write increment through it (widens the race window) | `cargo run` (debug build), 8 worker threads, `n = 200_000` items, 10 consecutive runs against a fresh clone of `base_commit` | Reproduced 10/10: final counter consistently ~27000-30500 instead of the expected 200000 (~85% of increments lost to the unsynchronized race) - see `output.txt` |
| Same witness type, same closure, driven instead through the crate's *other*, non-recursive execution path (`(0..n).into_par().num_threads(8).for_each(h)`, `src/infallible/par_runner.rs`), which never constructs `XapSync` and already gives each worker its own `Copy` of the closure | `cargo run` (debug build), same `n`/thread count, 3 consecutive runs, same pristine clone | Reproduced 3/3, same magnitude of lost updates - see "The fix" below for what this implies |
| `cargo +nightly miri run` on a scaled-down variant (`n = 2_000`, 4 threads) of the same repro | `nightly-2026-08-21`, miri, single run (feasibility check) | Ran to completion without flagging a data race and without the program's own counter mismatch firing either - inconclusive on this specific run (most likely: miri's own scheduler happened not to interleave the two worker threads' unsynchronized accesses to the counter in that particular execution; a data-race diagnostic from miri does not need to reproduce every run to be meaningful, but this run simply didn't hit one). miri also emitted an unrelated "the main thread terminated without waiting for all remaining threads" leak-check warning against this crate's own custom thread-pool shutdown, suggesting the pool's shutdown handshake isn't fully miri-clean regardless of this candidate - not pursued further (out of scope; noted here for anyone picking `miri` back up on this crate) |
| `-Zsanitizer=thread` (TSan) build via `cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu` | nightly toolchain, `rust-src`/`llvm-tools` installed | Compiled successfully, but the sandboxed execution environment this session ran in refuses `personality(ADDR_NO_RANDOMIZE)` ("Operation not permitted", even with the sandbox override flag set) which TSan itself requires at startup - environment limitation, not evaluated further this session |

The first approach alone already gave a clean, strongly repeatable (10/10, ~85% loss every time)
signal, satisfying the soft escalation cap; the second approach was pursued anyway because it
changes the conclusion materially (see "The fix").

## Status

**Confirmed.** Reproduces 10/10 runs, debug build, no fixed seed, real OS threads (no
sanitizer/miri needed - the observed effect, a hugely wrong aggregate count from lost non-atomic
updates, is unambiguous on its own). See "The fix" below for an important caveat discovered while
verifying a patch: the same class of race is *also* independently reproducible (3/3) via a
completely different, already "correctly designed" execution path in this same crate that never
touches `XapSync` at all - so while the flagged code (`XapSync`'s blanket `Sync` impl, and its
"share one instance's `&self` across concurrently-running real threads" usage pattern) is real,
type-level-unsound, and directly demonstrated to carry a real race through its own exact code
path, it is **not** what uniquely exposes the crate to this class of race in practice.

**Fully reproducible**: **Yes**. `repro/src/main.rs` drives the crate's real, public
`par_recursive(...).map(...).for_each(...)` entry point (not a hand-built stand-in for it),
executed by the crate's own real default thread pool spawning genuine OS worker threads. This is
a single-participant finding (how the crate's own generic recursive-execution engine reacts to a
caller-supplied `Copy + Send` closure) - CONFIRMATION.md section 11's "real second participant"
requirement does not apply (per that section's own carve-out: the user-supplied closure *is* the
genuine external input a real caller would pass through the real public API; there is no "other
side" being impersonated).

## Class

data-race

## Root cause

`Xap: Copy + Send` (`src/infallible/xap.rs:6`) never requires `Sync` of any user-supplied closure
chain. Every *non-recursive* execution entry point in this crate (`src/infallible/par_runner.rs`)
respects that: it captures the `Xap`/closure value by `move` into each spawned worker thread's own
closure, relying on `Xap: Copy` to give every thread an independent bitwise copy, so `Sync` is
never needed. The *recursive* (`par_recursive`) execution entry points instead need to reuse the
same `Xap` value across multiple separate "waves" of a BFS-style tree traversal (one `for_each`/
`flat_map`/`reduce` call per depth level, the same value needed again on the next iteration) - to
do that without giving up the value on the first wave, `XapSync::new(xap)` wraps it and the six
execution functions capture it **by reference** (no `move`, since it's read again in the next loop
iteration) into the closure passed to that wave's `for_each`/`flat_map`/`reduce`. That closure
needs to be `Send` (required by the internal `for_each`/`flat_map` bound, itself needed because the
crate's thread pool genuinely runs it from spawned OS threads), and `&XapSync<X>: Send` requires
`XapSync<X>: Sync` - which `X: Xap`'s own `Copy + Send` bound does not provide. `XapSync<X>`
papers over the gap with an unconditional `unsafe impl<X: Xap> Sync for XapSync<X> {}`, which is
not actually guaranteed sound for every `X: Xap` (only `Copy + Send` is), and every worker thread
within one wave ends up calling `Xap::xap(&self, ...)` on the literal same `XapSync<X>` instance
concurrently, with no synchronization - confirmed to carry a real, repeatable data race for a
witness `Copy + Send`-but-`!Sync` closure (see "What was tried").

## Introduced by

b67d293a "xap sync" (2026-08-26). This commit added `xap_sync.rs` (including the unconditional
`unsafe impl Sync`) and switched the recursive execution entry points over to it in the same
change (`git log --follow --diff-filter=A -- src/infallible/recursive/xap_sync.rs`).

## The fix

Attempted, but **not included as `patch.diff`** - see below for why.

The type-level inconsistency itself is trivially closable: remove the unconditional
`unsafe impl<X: Xap> Sync for XapSync<X> {}`, and switch each of the six execution functions'
`for_each`/`flat_map`/`reduce` closures from implicit by-reference capture to explicit `move`
capture of `xap` (a bitwise `Copy`, since `X: Xap: Copy`, so `move`-capturing it does not consume
the outer function's own `xap` binding - later loop iterations still see a valid value), matching
the same "each thread gets its own copy" design every sibling wrapper and the non-recursive
execution path already use. The one other externally-visible free variable these closures already
capture, `extend: E: Fn(&X::I) -> I + Send + Sync`, is pre-bound to `let extend = &extend;` so
`move` captures that shared reference (itself `Copy`) instead of moving the (possibly non-`Copy`)
`E` value itself out of the enclosing function. This diff (`attempted_fix.diff` in this folder,
+18/-14 across 7 files) compiles cleanly and the full project test suite (`cargo test --lib
--tests` and `--doc`) passes identically before and after: **180 passed; 0 failed** (lib +
integration tests) and **260 passed; 0 failed; 16 ignored** (doctests) in both cases - see
`output.txt`.

However, re-running the exact same `repro/src/main.rs` against the patched clone still reproduces
the race **10/10**, same magnitude (~85% lost updates) as before the patch (see `output.txt`,
"AFTER attempted_fix.diff") - this is *not* a verified fix per CONFIRMATION.md section 6, so it is
deliberately **not** shipped as `patch.diff`. The reason it doesn't help, discovered while
investigating why: `std::cell::Cell<T>`, `RefCell<T>`, and `UnsafeCell<T>` are all `!Copy` in std
(verified directly: `Cell<i64>`/`UnsafeCell<i64>` both fail a `fn is_copy<T: Copy>(_: T)` check),
so there is no way to build a *safe* (no `unsafe impl Send` written by the user) `Copy + Send`
closure with genuine racy interior mutability at all - the only way to get a `Copy + Send`-but-
`!Sync` closure with real racy state is to close over a raw pointer to externally-owned mutable
memory (exactly what the witness `Racy` type does) and manually assert `unsafe impl Send` for it.
For a raw-pointer-based captured value, "one shared `&self` instance" and "N independent per-thread
bitwise copies of the pointer" are **behaviorally identical** with respect to the race: the pointer
*value* is the same either way, so every thread still ends up racing on the same pointee regardless
of which design XapSync uses. This was independently confirmed by reproducing the identical race
(3/3) through the crate's *other* execution path (`(0..n).into_par().for_each(...)`,
`src/infallible/par_runner.rs`), which was already using the "correct" per-thread-copy design
before this candidate was even opened, and never constructs `XapSync` at all.

In short: `XapSync`'s own `unsafe impl Sync` is a real, independent soundness inconsistency (it
grants `Sync` without any bound that would justify it, unlike every sibling wrapper, which is
worth fixing on its own type-hygiene merits and is what the attempted patch does) - but it is not,
on its own, what makes the demonstrated data race achievable, since the identical race is already
reachable through the crate's existing, unrelated, "already correctly designed" execution path.
Actually eliminating this whole class of race would need a broader decision the maintainers should
make (e.g. requiring every user-supplied `Xap`/closure to be `Sync` everywhere, which is a breaking
API change removing exactly the flexibility - per-thread-local `Copy` state - the `Copy + Send`-only
design intentionally allows) rather than a same-file bounded fix, so no `patch.diff` is included
here.

## What's not done / open questions

- The same bug pattern (unconditional `unsafe impl Sync` without a compensating bound) was already
  independently confirmed in candidates 002 (`UseSlice`) and 003 (`PairPtr`), and ruled unreachable
  in candidate 019 (`FixedChunkRunner`/`AdaptiveChunkRunner`). Unlike 002/003 (where adding the
  missing bound genuinely closes the exposure, since those wrap a caller's own `T`/element type
  whose `Send`-ness is meaningful and enforceable), this candidate's flagged spot turns out to be
  the first case in this series where the "obvious" bound fix does not actually close the
  demonstrated exposure, because the exposure is not unique to it - see "The fix".
- Whether the crate's maintainers consider the underlying `Copy + Send`-without-`Sync` design (used
  throughout, not just in `XapSync`) an accepted tradeoff or something to reconsider is a design
  decision, not something this candidate can resolve with a same-file patch.
- The `miri` "main thread terminated without waiting for all remaining threads" warning noticed
  while probing this crate's custom thread pool under `miri` (see "What was tried") was not
  investigated further - it may be worth a separate candidate if `miri` support for this crate's
  pool shutdown path is ever wanted.
- `XapSync`'s own unconditional `Sync` impl was not checked against every other possible caller of
  the recursive execution path beyond the six files listed (e.g. any future new recursive
  entry point would inherit the same gap by construction, since it's on `XapSync` itself).
