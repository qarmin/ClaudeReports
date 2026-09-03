# FixedChunkRunner/AdaptiveChunkRunner unconditional `unsafe impl Sync`

`src/runner/runner_variants/fixed_chunk/fixed_chunk_runner.rs:9` and
`src/runner/runner_variants/adaptive_chunk/adaptive_chunk_runner.rs:14` both declare
`unsafe impl<P: ParThreadPool> Sync for ...<P> {}` on a struct holding a real `pool: P` field, and
`ParThreadPool` (`src/pool/par_thread_pool.rs:48`) has no `Sync` supertrait bound - the same shape
as the already-confirmed candidates 002 (`UseSlice`) and 003 (`PairPtr`). Both structs are publicly
constructible with a caller-supplied `P` via `Runner::fixed_with_pool`/`Runner::adaptive_with_pool`
(`src/runner/new_runner.rs`), so a caller-supplied `P` that is `!Sync`/`!Send` (nothing in
`ParThreadPool`'s definition prevents this) ends up wrapped in a struct the crate unconditionally
asserts is `Sync`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading / call-graph trace: every internal caller of `ParRunner::pool()`/`pool_mut()` across all six executor variants (`src/infallible/par_runner.rs`, `src/infallible_use/par_runner.rs`, `src/option/par_runner.rs`, `src/option_use/par_runner.rs`, `src/result/par_runner.rs`, `src/result_use/par_runner.rs`) and `src/sort/sort_slice.rs` | Grepped every `move \|s\|`/`move \|\|` closure passed to `scoped_computation`/`run_in_scope` across the whole `src/` tree | In every case `self`/`runner` is used only to call `.pool()`/`.pool_mut()`/`.nt_state()` on the single calling ("driver") thread, strictly *before* `scoped_computation` is invoked; the closures actually handed to `scoped_computation`/`run_in_scope` (which is where real worker OS threads execute) never capture `self`/`runner`/`pool` - only `iter`, `state`, `results`, `x` (type-parameterized `Self::method(...)` static dispatch is used instead of an instance reference). `BasicPool::scoped_computation_impl` (`src/pool/pool_impl/basic.rs:260`) additionally confirms the outer closure `f` itself runs synchronously on the calling thread (`catch_unwind(AssertUnwindSafe(\|\| f(&scope_ref)))`), not on a spawned thread. So the crate's own internal execution never actually shares `&Self`/`&P` across two real, concurrently-running threads for any of the current executor variants |
| Manual boundary-value construction + real compile/run probe: a standalone crate (this candidate's own `repro/`) defines a witness `RcPool` (`ParThreadPool` impl holding `Rc<Cell<i64>>`, hence genuinely `!Sync`/`!Send`), builds it via the crate's real public `Runner::fixed_with_pool`/`Runner::adaptive_with_pool`, statically asserts the resulting `FixedChunkRunner<RcPool>`/`AdaptiveChunkRunner<RcPool>` are `Sync`, then shares `&runner` into a genuine second OS thread via `std::thread::scope` (possible only because of the Sync impl) and tries to reach the inner `RcPool`/`Rc` from that thread through the public API | `cargo run --release` / `cargo test --release`, real OS thread via `std::thread::scope`, run 4 times total (see Repeatability) | `FixedChunkRunner<RcPool>`/`AdaptiveChunkRunner<RcPool>` are confirmed `Sync` (the structural premise is real) and `&runner` is confirmed shareable into a real second thread - but nothing reachable from that thread's `&FixedChunkRunner<RcPool>` touches `RcPool`/its `Rc` at all: no crash, no corruption, no observable effect beyond the reference existing. `Rc::strong_count` never changes, no panic, no sanitizer/miri needed - there is simply no operation to perform |
| Compile-time reachability probes for *why* nothing is reachable: (a) `let _ = runner.pool;` (direct field access) and (b) `let _: &dyn orx_parallel::ParRunner;` (naming the trait that owns `pool()`/`pool_mut()`), both against the same pristine clone | `cargo build --release` | (a) `error[E0616]: field \`pool\` of struct \`...FixedChunkRunner\` is private`. (b) `error[E0405]: cannot find trait \`ParRunner\` in crate \`orx_parallel\`` - `mod runner;` in `src/lib.rs` is a private module and only `runner::Runner` (the concrete factory struct) is re-exported at the crate root; `ParRunner` itself (and the `pool()`/`pool_mut()` methods that would be needed to reach `P`) is not part of the crate's public API at all, confirmed by an actual compiler error rather than by reading privacy rules |

No `miri`/ASAN escalation was needed past these two approaches: the second approach already
produced a concrete, repeatable, real-execution result (the witness type compiles, runs, and does
nothing unsound), and the third grounds *why* with a real compiler diagnostic rather than
speculation - between them they cover both "can the type-level premise be constructed for real" and
"is there any way to actually exploit it", which is the whole question this candidate raises. A
fuzzing harness or `miri` run would have nothing to explore here: there is no reachable code path
left to fuzz once the field and the trait are both confirmed compiler-rejected.

## Status

**Not reproducible.**

The unconditional `Sync` impl is real (confirmed: `FixedChunkRunner<RcPool>`/
`AdaptiveChunkRunner<RcPool>` compile and are genuinely `Sync` for a `!Sync`/`!Send` `RcPool`), and
it is genuinely usable through the crate's real public constructors
(`Runner::fixed_with_pool`/`Runner::adaptive_with_pool`) - so far this is structurally identical to
confirmed candidates 002/003. It diverges from those two at the exploitation step: 002/003 both
demonstrated the crate's *own* internal execution engine actually handing the unsoundly-`Sync`
struct's contents (a raw pointer into a caller's slice/fold accumulator) across real, concurrently-
running OS worker threads it spawns itself, from within its normal `.for_each()`/`.reduce()`/
`.fold()` execution. For `FixedChunkRunner`/`AdaptiveChunkRunner`, no such internal sharing exists
(see the call-graph trace above: `pool()`/`pool_mut()` are only ever called on the single calling
thread, before any worker thread starts), and the only accessor that could reach the wrapped `P`
from outside (`ParRunner::pool()`/`pool_mut()`) is on a trait that is not part of the crate's public
API at all (real `E0405`), with no alternative public field access either (real `E0616`). Sharing
`&FixedChunkRunner<P>`/`&AdaptiveChunkRunner<P>` into a second real thread is possible (the Sync
impl really does allow it), but inert: there is no operation - safe or otherwise, public API only -
that a second thread can perform on that reference to touch the wrapped `P` at all.

This was checked once via direct iteration (4 full `cargo run`/`cargo test` cycles against the
worktree while developing the probe) and re-verified via `just clean && just test &&
just probe-privacy` against a genuinely fresh clone of `base_commit` (see `output.txt`) - identical
outcome both times, as expected for a compile-time-gated, non-racy observation (there is no runtime
nondeterminism to average over here; whether `FixedChunkRunner<P>: Sync` holds, and whether `.pool`/
`ParRunner` are reachable, are static facts, not something that could flake between runs).

## Class

n/a (not reproducible)

## Root cause

`ParThreadPool` genuinely has no `Sync` bound, and `FixedChunkRunner<P>`/`AdaptiveChunkRunner<P>`
genuinely grant `Sync` unconditionally regardless of `P` - the same type-level laxity as 002/003.
The reason this one turns out safe in the current codebase, unlike those two, is a **combination of
two independent facts**, either of which alone would already block the exploit:

1. **No internal sharing.** Unlike `UseSlice`/`PairPtr` (whose entire purpose is to be handed to
   worker closures that *do* run on other real threads via `Self::Pool::run_in_scope`),
   `FixedChunkRunner`/`AdaptiveChunkRunner` are only ever touched via `pool()`/`pool_mut()` by the
   single thread driving a given computation, strictly before any worker thread is spawned for that
   computation. Every one of the six executor variants (`infallible`/`infallible_use`/`option`/
   `option_use`/`result`/`result_use`) follows this identical pattern, and `sort_slice.rs`'s
   experimental sort does too.
2. **No public accessor.** Even if some future internal change *did* start sharing `&Self`/`&P`
   across real threads, or if external code tried to reach in on its own, `ParRunner` (the trait
   that owns `pool()`/`pool_mut()`) is not exported from the crate at all (`mod runner;` is private
   in `src/lib.rs`, and only the concrete `Runner` factory type is re-exported, not the trait) - and
   `pool: P` is a private field with no other accessor. So there is currently no way, safe or
   otherwise (short of the caller's own `unsafe`/`transmute`, which would be a self-inflicted
   soundness hole unrelated to this crate), to reach `P` through a shared `&FixedChunkRunner<P>`/
   `&AdaptiveChunkRunner<P>` reference at all.

Point 2 is worth flagging explicitly: the doc comment on `Runner` (`src/runner/new_runner.rs:13`)
says "You can also implement a compatible runner type yourself and pass it directly to
`.runner(...)`", which is not actually possible today (`ParRunner` cannot be named or implemented
from outside the crate - confirmed via a real `error[E0405]`, not just privacy-rule reasoning) -
suggesting either the trait is meant to become public later (which would make this candidate's
`Sync` impl newly and directly exploitable the same way 002/003 already are, without anything else
needing to change) or the doc comment is stale. Either way, this is a preexisting condition of the
type-level laxity, not something this session's probe artificially avoided.

## Introduced by

Not determined - not attempted. Per `SKILL.md`'s candidate loop, "Introduced by" is scoped to
`Confirmed` findings; this candidate did not reach that status, so no `git log -S`/`git bisect`
archaeology was performed. A quick `git log --follow` on both files shows the unconditional `Sync`
impl already present at every rename/reorganization point captured in `git log`, consistent with
002/003's own finding that this style of impl has been present since each runner type's
introduction, but this was not pinned down to a single commit for this candidate.

## The fix

No fix was written - there is no reproducible defect to verify a fix against in the current
codebase (per `SKILL.md`: a bounded fix is only attempted for `Confirmed` candidates). For the
record, a defense-in-depth tightening mirroring 002/003's own fix would be straightforward if a
maintainer wants one purely as a preventative measure (not because anything is unsound today):
`unsafe impl<P: ParThreadPool + Sync> Sync for FixedChunkRunner<P> {}` (and the same for
`AdaptiveChunkRunner`) - this was not applied or verified this session since there is nothing for it
to fix yet, and applying/reverting an unverified speculative patch would violate `SKILL.md`'s "the
main project's working tree is always left clean" rule for a candidate that didn't confirm.

- Project test suite: not run before/after, since no patch was applied (N/A - "before"/"after" both
  mean the identical, unmodified tree here).

## What's not done / open questions

- If `ParRunner` is ever made public (the `Runner` doc comment already implies this is the intended
  direction), this candidate should be re-checked immediately - making the trait nameable would
  hand external callers exactly the `.pool()` accessor this candidate found missing, and the
  unconditional `Sync` impl would then be as directly exploitable as 002/003 already are, via the
  identical witness-type technique used in this candidate's `repro/`.
- Did not check whether any *other* internal code path (outside the six executor variants and
  `sort_slice.rs` checked here) might someday capture `self`/`pool` inside a closure handed to
  `scoped_computation`/`run_in_scope` - the call-graph trace covered every current call site of
  `.pool()`/`.pool_mut()` in the tree (`grep -rn "\.pool()\|\.pool_mut()"`), so this is a complete
  check of the *current* codebase, not a guarantee against a future regression.
- `WithDiagnostics<R>` (`src/runner/runner_variants/with_diagnostics.rs`) wraps an inner `R:
  ParRunner` and delegates `pool()`/`pool_mut()` straight through - checked, and it does not change
  the analysis (still single-driver-thread-only access, still gated behind the same private trait).
- New suspect noticed while tracing this: the `Runner` doc comment's claim that a caller "can also
  implement a compatible runner type" appears to be inaccurate against the current crate (real
  `E0405` when trying), which is either stale documentation or an intentionally-not-yet-shipped API
  - worth a maintainer follow-up independent of this candidate.
