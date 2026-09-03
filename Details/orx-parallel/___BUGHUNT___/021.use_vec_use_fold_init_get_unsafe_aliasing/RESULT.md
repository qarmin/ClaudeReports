# use_vec_use_fold_init_get_unsafe_aliasing

The starting suspicion (candidate 021 as assigned) was narrower: `UseVec::init_get`
(`src/use_var/use_vec.rs:91-99`) does `unsafe { &mut *self.cache.ptr_mut(thread_idx) }` behind
only a comment ("`get` will be called exactly once by the corresponding thread"), unlike the
sibling `get()` a few lines below which asserts `cache.len() > thread_idx` first, and `UseFold`
(`src/use_var/use_fold.rs:40-52`) inherits the same pattern by forwarding to `UseVec`/nested
`Use::init_get`. Digging into *why* the comment-only invariant currently holds (traced through
`do_spawn_new`/the spawn loop in `src/infallible_use/par_runner.rs` and its `option_use`/
`result_use` siblings) turned up something stronger than "a future violation might one day slip
through": `Use::init_get` (`src/use_var/use.rs:29`) is declared as a **safe** trait method
(`fn init_get(&self, thread_idx: usize) -> &mut Self::Item`), and it is **public** - the trait and
`UseVec` are both re-exported at the crate root (`orx_parallel::{Use, UseVec}`,
`src/lib.rs:62`/`src/use_var/mod.rs:12,15`). Any downstream crate can therefore call
`some_use_vec.init_get(0)` twice (or any number of times) for the same `thread_idx`, using no
`unsafe` block at all in its own code, and get two simultaneously-live `&mut T` references to the
same memory - a plain safe-Rust soundness hole, not merely a fragile-but-currently-unbroken
internal invariant. `UseSlice::init_get` (`src/use_var/use_slice.rs:29`) has the identical shape
and the identical bug.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Code reading: traced every internal call site of `init_get` (`src/infallible_use/par_runner.rs`, `src/option_use/par_runner.rs`, `src/result_use/par_runner.rs`) back to `ParRunner::do_spawn_new`'s own doc comment ("Since this method is called sequentially, returned value will always be `Some(spawned)` if it returns some") and to every `ParRunner` impl's actual `do_spawn_new` body (`FixedChunkRunner`: `(spawned < max).then_some(spawned)`; `AdaptiveChunkRunner`; `WithDiagnostics` delegates through) | N/A | Confirmed the *internal* call sites are safe by construction: `th_idx` is handed out by a single-threaded, monotonically-increasing loop, one value per spawned closure, never reused within one computation - and the public `use_vec`/`use_slice` combinators take `&mut UseVec`/`&mut [U]`, so the borrow checker also prevents two *combinator* chains from touching the same instance concurrently. This ruled out the internal-misuse framing of the original candidate, but also surfaced that `Use::init_get` itself is a safe fn on a public trait with no such protection for a caller who bypasses the combinator layer and uses `Use`/`UseVec` directly. |
| 2. Manual boundary-value construction: a 20-line, single-threaded, zero-`unsafe` program (`repro/src/main.rs`) using only `orx_parallel::{Use, UseVec}` (both public, re-exported) that calls `uv.init_get(0)` twice and writes through both resulting `&mut i32` references | `cargo run --release`, real clone of base_commit b9214fbeb714e4bb8f8858676a818cef2f21b097, 3 repeat runs | Reproduces 3/3: prints `r1 = 222, r2 = 222` every time - writing through `r2` silently overwrote what `r1` pointed at, i.e. the two "distinct" `&mut i32` bindings are the same memory. See `output.txt`. |
| 3. `cargo +nightly miri run` (Stacked Borrows, default) on the exact same repro binary | `miri 0.1.0`, same clone, 2 repeat runs | Reproduces 2/2, deterministically: `error: Undefined Behavior: attempting a write access using <2755> ... but that tag does not exist in the borrow stack for this location`, pointing at `*r1 = 111;`, with the diagnostic explicitly naming `r1`'s creation site and `r2`'s creation site (`src/main.rs:17`/`18`, i.e. the two `init_get(0)` calls) as the conflicting accesses. This is a genuine, tool-confirmed Stacked-Borrows violation, not just an externally-visible logic symptom. See `output.txt`. |

Soft cap of 3 approaches reached with a strong, tool-confirmed signal on the first two escalating
tools tried (a plain safe-code repro already showed the symptom directly; miri then gave the
formal UB diagnostic on the very same binary) - no further escalation (ASAN/fuzzing) was needed or
attempted.

## Status

**Confirmed.** Reproduces 3/3 runs under a plain `cargo run --release` (deterministic - this is
pure single-threaded, single-process aliasing with no concurrency or layout/seed dependence at
all) and 2/2 runs under `cargo +nightly miri run` with a real Stacked Borrows diagnostic pointing
at the exact two `init_get` call sites.

**Fully reproducible**: **Yes**. `repro/src/main.rs` is a single external caller using only
`orx-parallel`'s real, public, exported API (`orx_parallel::{Use, UseVec}`) exactly as any
downstream crate would import and call it - there is no second participant to fake here (this is
the single-artifact/single-caller case `CONFIRMATION.md` section 11 describes as not needing a
second participant at all): the finding is "this public safe fn is unsound when called twice by
one caller", and the repro is exactly that one caller, calling the real entry point twice, for
real, with no `unsafe` anywhere in the calling code.

## Class

memory-safety / UB (an aliased-`&mut` soundness hole reachable from 100% safe caller code)

## Root cause

`Use::init_get` (`src/use_var/use.rs`, pre-fix) is declared as a safe function
(`fn init_get(&self, thread_idx: usize) -> &mut Self::Item`) that hands out a mutable reference
derived from a shared `&self`. `UseVec`'s two impls (`impl Use for UseVec<T, F>` and
`impl Use for &mut UseVec<T, F>`, `src/use_var/use_vec.rs`) and `UseSlice`'s impl
(`src/use_var/use_slice.rs`) implement it by writing into a slot (`ConcurrentOrderedBag::set_value`
for `UseVec`, a raw pointer offset for `UseSlice`) and then returning `&mut *ptr` into that same
slot, unconditionally, on every call - there is no check anywhere in these methods for whether a
previous call already handed out a still-live reference to the same `thread_idx`. Internally, the
crate only ever calls `init_get` in a pattern that happens to avoid this (each spawned worker
thread calls it exactly once, for a `th_idx` value that a sequential, monotonically-increasing
spawn loop guarantees is unique per computation - see approach 1 above), and the public combinator
entry points (`Par::use_vec`/`use_slice`) additionally require `&mut` access to the underlying
`UseVec`/slice, so the borrow checker prevents two *combinator chains* from racing on the same
instance. None of that protects a caller who uses the public `Use`/`UseVec`/`UseSlice` API
directly (bypassing the combinator layer entirely, which is fully supported - `Use` and `UseVec`
are both public/exported precisely so custom `Use` implementations and direct testing/usage are
possible, see `src/use_var/use.rs`'s own doctest, which itself calls `use_var.init_get(0)`
directly on a bare `UseVec`). Because `init_get` is *safe*, nothing stops that caller from calling
it twice for the same `thread_idx` and holding both returned references live simultaneously -
exactly what `repro/src/main.rs` does.

## Introduced by

not determined precisely as a single commit - `git log -S "get will be called exactly once"` and
`git log -S "assert!(self.cache.len()"` on `src/use_var/use_vec.rs` (and its pre-rename path
`src/infallible_use/use_var/use_vec.rs`) show the file's very first commits already had this shape:
an early commit titled "assertion" (08152b26) is where `get`/`get_mut`'s runtime check was added,
but even that commit's *predecessor* already had `init_get`'s (then named `get`'s) unconditional
`unsafe { &mut *... }` with no check, and the trait itself (`fn init_get(&self, ...) -> &mut
Self::Item` as a *safe* fn) has apparently never been marked `unsafe` at any point in this file's
history. This looks like a design decision present since the `Use`/`UseVec` abstraction was first
introduced, not a regression from a specific later change - not worth a full `git bisect` for a
"has this always been broken" question with no realistic "was fine, then broke" boundary to
bisect toward.

## The fix

Marked `Use::init_get` `unsafe fn` in the trait definition (`src/use_var/use.rs`), added a `#
Safety` doc section spelling out the actual contract (at most one live `&mut Self::Item` per
`thread_idx` at a time), and propagated `unsafe fn` through every implementation
(`UseVec`'s two impls, `UseSlice`, `UseFold`) and every internal call site (all ~30 occurrences
across `src/infallible_use/par_runner.rs`, `src/option_use/par_runner.rs`,
`src/result_use/par_runner.rs`), wrapping each in `unsafe { ... }` with a short, once-per-file
safety comment explaining why the existing internal call pattern already satisfies the new
contract (the same `do_spawn_new`-monotonicity argument from approach 1 above). `UseSlice::get`'s
body (previously `self.init_get(thread_idx)`, now calling an `unsafe fn`) was updated to
`unsafe { self.init_get(thread_idx) }` with a one-line safety comment (sound because `get` takes
`&mut self`, so no other live reference can exist). `use.rs`'s own doctest was updated to wrap its
direct `init_get(0)` call in `unsafe { }` to keep compiling.

This converts the soundness hole from "silently produces UB when misused" into "requires an
explicit `unsafe` block to misuse" - the standard Rust discipline for an API that cannot be made
memory-safe purely through its signature. It does not, by itself, add a runtime double-init guard
(that would need extra per-slot tracking state and was judged out of scope for a bounded fix this
session - see "What's not done" below); `repro/src/main.rs`'s exact misuse pattern, and any
variation of it (a different `thread_idx`, three calls instead of two, `UseSlice` instead of
`UseVec`), now requires the caller to write `unsafe` to even attempt it, which is the correctness
guarantee `unsafe fn` exists to provide - checked against `UseSlice` (same fix pattern, same
signature change) as well as `UseVec`, not just the one type the repro happened to use, per
`CONFIRMATION.md` section 7.

Before -> after, re-running `repro/` against the patched clone: the exact same repro
(`repro/src/main.rs`, unmodified) that reproduced 3/3 under `cargo run --release` before the patch
now fails to compile with `error[E0133]: call to unsafe function 'init_get' is unsafe and requires
unsafe function or block`, pointing at both `init_get(0)` call sites - the misuse is no longer
expressible without an explicit `unsafe` block. See `output.txt` for both captures.

- Project test suite: 440 passed / 0 failed / 16 ignored before the patch, 440 passed / 0 failed /
  16 ignored after (identical per-file counts in both runs) - `cargo test --quiet` on the patched
  clone, no new failures. (`--all-features` does not build at all on this exact `base_commit`,
  before or after the patch - two pre-existing, unrelated errors, `E0425` in
  `src/pool/global_pool.rs` and `E0308` in `src/infallible/par_runner.rs` from an incompatible
  feature combination; confirmed identical on a `git stash`ed pristine checkout, so this is not
  something the patch touches or introduces.)

## What's not done / open questions

- No runtime double-init guard was added (e.g. tracking, per slot, whether `init_get` has already
  been called for that `thread_idx`, to turn a misuse into a deterministic panic instead of
  requiring `unsafe`) - `unsafe fn` was judged the correct, minimal fix for a trait method whose
  soundness genuinely cannot be guaranteed generically (a `Use` implementor could be backed by
  arbitrary storage), but a maintainer may additionally want a debug-only assertion in `UseVec`/
  `UseSlice`'s own concrete impls for defense-in-depth against a caller who *does* write the
  `unsafe` block but still gets the contract wrong internally within this crate. Not attempted
  this session (would need, for `UseVec`, a way to distinguish "index not yet written" from "index
  already written" independent of `ConcurrentOrderedBag::len()`, which tracks the *maximum*
  written index across all threads via `fetch_max` - not the same as "was this specific index
  already written" when threads may legitimately write out of numeric order relative to each
  other).
- This is a breaking API change (`init_get` gaining `unsafe`) for any downstream crate that
  implements `Use` itself or calls `init_get` directly (as opposed to only using the `.use_vec()`/
  `.use_slice()`/`.use_fold()` combinators, which are unaffected since their own call sites were
  updated as part of this patch) - left for the project maintainers to judge against their own
  semver policy rather than silently deciding it here.
- `PairPtr::u_v_mut`/`u_ptr` (`src/use_var/pair_ptr.rs`) were read but not specifically targeted -
  they operate on raw pointers already validated by `UseFold::init_get`'s own (now-`unsafe`)
  contract, so they inherit whatever guarantee the caller of `init_get` upholds; not independently
  exploited or fixed as their own candidate.
- The `wasm`/`persistent-pool-rayon`/`experimental` feature combinations were not exercised against
  this specific fix (only the default feature set's test suite was run before/after) - the same
  `init_get` code paths are feature-independent (`use_var/` has no `#[cfg(feature = ...)]` gates),
  so this is expected to be a non-issue, but wasn't independently verified under those features
  this session.
