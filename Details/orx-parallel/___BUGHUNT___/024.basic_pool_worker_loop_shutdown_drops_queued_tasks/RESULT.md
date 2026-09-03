# basic_pool_worker_loop_shutdown_drops_queued_tasks

`BasicPool::worker_loop` (`src/pool/pool_impl/basic.rs:172-196`) returns immediately once
`state.shutdown` is set, without draining `state.queue`; `Inner::drop` (`basic.rs:30-46`) then pops
any remaining queued `Task`s and calls `unsafe { task.drop() }`, which frees the boxed closure via
`drop_impl` without ever calling `run_fn`/`complete_task()`. If this ever ran while a live
`ScopeRuntime::wait_for_completion()` was still waiting on one of those tasks, that caller would
block forever (the pending counter would never reach 0). The same `worker_loop`/`Inner::drop`
shape (queue-drain-without-running on shutdown) is duplicated in `src/pool/pool_impl/wasm_web.rs`.
This looked suspicious as a classic "cleanup path forgets to run/complete pending work" pattern,
but the candidate itself flagged low confidence: `scoped_computation_impl` takes `&'env self`/
`&'env mut self` for the whole (synchronous, blocking) call, which looked like it might keep the
owning `Arc<Inner>`'s refcount >= 1 for the scope's entire duration by construction.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning over ownership/borrow rules: trace every call site that can reach `Inner::drop` with a non-empty queue | Read `src/pool/pool_impl/basic.rs` (`Inner::drop`, `ScopeRuntime`, `scoped_computation_impl`, `run_in_scope`, all 3 `ParThreadPool` impls for `BasicPool`/`&BasicPool`/`&mut BasicPool`), `src/pool/global_pool.rs`, `src/pool/pool_impl/wasm_web.rs`, and `src/pool/par_thread_pool.rs`'s trait signature | A task can only sit in `state.queue` while its owning scope's `pending` counter is still > 0 (a task is only ever removed from the queue by a worker, which always calls `complete_task()` immediately after, regardless of panic). `scoped_computation`/`scoped_computation_impl` borrow `self` for `'env` and only return after `wait_for_completion()` returns (`pending == 0`), so by the time the call returns and the borrow is released, the queue is guaranteed empty for that scope. Rust's own borrow checker forbids moving/dropping the borrowed `BasicPool`/its `Arc<Inner>` clone while that borrow is outstanding, so the one clone whose `&self`/`&mut self` is actively in use can never be the one that drops `Inner` mid-scope; any *other* clone dropped concurrently only decrements the `Arc` refcount without reaching 0 while the active clone is still alive. The global pool (`static PERSISTENT_POOL: LazyLock<BasicPool>`/`WASM_WEB3_RUNTIME: OnceLock<Arc<Inner>>`) is `'static` and is never dropped during ordinary program execution at all, so that path is not reachable either |
| Debug-assertions build, stress test through the real public API (`Pool::basic` + `Runner::fixed_with_pool` + `Par::for_each`, not `BasicPool`'s internals directly) - concurrently spawn 4 extra clones of a shared `BasicPool` and drop them from separate threads while the *original* clone is in the middle of a real parallel `for_each` computation with in-flight tasks, using a single worker thread (`Pool::basic(1)`) to widen the window during which the queue is non-empty; whole loop wrapped in a 60s watchdog (a hang would fail the test rather than block forever) | `cargo test --test bughunt_024_basic_pool_stress` (debug build, 2 runs: 200 iterations x 64 tasks, then 500 iterations x 256 tasks) | Both runs: `ok`, 1 passed, no hang, no lost task (`counter == expected` every iteration; 700 iterations total in debug) |
| Same stress test under a release build | `cargo test --test bughunt_024_basic_pool_stress --release` (2 runs, same params as above) | Both runs: `ok`, 1 passed, no hang, no lost task (700 iterations total in release) - see `output.txt` for all 4 runs' real captured output |
| Compile-time boundary check: try to write a version of the scenario where the closure passed to `scoped_computation` captures no reference tied to a concrete local lifetime (only owned/`Arc`-cloned data) | `cargo test --test bughunt_024_basic_pool_stress` with an earlier draft of the test using `Arc::clone(&counter)` moved into the closure instead of a `&AtomicUsize` reference | `error[E0597]: pool does not live long enough` / "argument requires that `pool` is borrowed for `'static`" - rustc rejects the attempt outright; the borrow of `pool` cannot outlive the call the way a use-after-drop would require. Not a proof that *no* unsafe/internal misuse could ever trigger this, but concrete evidence that the safe, public-API shape of this call cannot let `pool` be dropped while a scope referencing it is still in flight (see `output.txt`) |

## Status

**Not reproducible.** No path through the crate's real public API (`BasicPool`, the `ParThreadPool`
trait, `Pool::basic`, `Runner::fixed_with_pool`) lets `Inner::drop` run while `state.queue` is
non-empty. Covered: the direct `BasicPool` clone/drop-during-scope interleaving (700+700 = 1400
stress iterations across debug and release builds, 2 runs each, zero failures) and the only two
places an owning `BasicPool`/`Arc<Inner>` could conceivably be dropped in this codebase (a
user-held clone, ruled out by the borrow-checker argument above and the stress test; the global
`'static` pool, never dropped during ordinary execution at all).

**Fully reproducible**: N/A (Not reproducible - nothing reproduced, so there is no repro to run).

## Class

N/A (Not reproducible).

## Root cause

Not a bug reachable in the live code as it exists today. The mechanism the candidate worried about
(`Inner::drop` discarding queued tasks without running them) is real code, and would be a genuine
hang if it ever executed while some `ScopeRuntime::wait_for_completion()` was still waiting on one
of those tasks - but there is no way to reach that state through this crate's public API:

- A `Task` only ever leaves `state.queue` via a worker popping it, and the worker *always* calls
  `runtime.complete_task()` immediately afterward (success or panic, via `catch_unwind`). So a
  non-empty queue implies some scope's `pending` count is still > 0, i.e. some caller is still
  inside `wait_for_completion()` for that scope.
- `scoped_computation`/`scoped_computation_impl` take `&'env mut self`/`&'env self` and only return
  *after* `wait_for_completion()` returns. The Rust borrow checker will not allow the borrowed
  `BasicPool` (the one whose `&self`/`&mut self` is live for that call) to be moved or dropped
  while the call is in progress - so the specific `Arc<Inner>` clone actually driving the in-flight
  scope can never be the one whose drop brings the refcount to 0 mid-call. Any *other* clone can be
  dropped concurrently (this is exactly what the stress test exercises), but that only decrements
  the shared `Arc`'s strong count without reaching 0, since the actively-borrowed clone is still
  alive.
- The crate's only other owner of a `BasicPool`/`Arc<Inner>` is the global default pool
  (`pool::global_pool::PERSISTENT_POOL`, a `'static LazyLock<BasicPool>` on the native/std path, or
  `WASM_WEB3_RUNTIME: OnceLock<Arc<Inner>>` on wasm) - `'static` items are never dropped during
  ordinary program execution, so `Inner::drop` never runs for the global pool at all.
- `wasm_web.rs`'s own copy of this shape explicitly documents the same conclusion in its own
  source: the queue-drain loop in its `Inner::drop` is commented "a scope waits for all submitted
  work; this is a defensive fallback" - i.e. the crate's own authors already treat this as dead
  code guarding an invariant they believe (correctly, per the above) always holds.

The queue-drain-and-silently-discard behavior in both `Inner::drop` impls is still a latent risk
*if* a future change ever adds a way to hold/drop the pool without the current
borrow-for-the-whole-synchronous-call discipline (e.g. an async/non-blocking `scoped_computation`
variant, or a scope type that could outlive the borrow it currently requires) - but there is no
such path today, so there is nothing to reproduce against the current tree.

## Introduced by

N/A - not applicable to a Not-reproducible finding (no bug exists at HEAD to attribute to an
introducing commit).

## The fix

No fix applies to a Not-reproducible finding; no `patch.diff` is included. If defense-in-depth were
still considered worthwhile purely against a *future* regression (not because today's code is
unsafe), the natural place would be a `debug_assert!(state.queue.is_empty())` at the top of
`Inner::drop`, matching `wasm_web.rs`'s own `scoped_computation_impl` which already asserts
`state.queue.is_empty()` after `wait_for_completion()` returns - left as a suggestion, not applied,
since there is no bug to verify a fix against this session.

- Project test suite: N/A, no patch was applied (unmodified tree). Baseline recorded anyway:
  `cargo test --lib` -> 99 passed; 0 failed; 0 ignored (matches the baseline candidates 016/020
  recorded on the same commit).

## What's not done / open questions

- `wasm_web.rs`'s identical `worker_loop`/`Inner::drop` shape was checked by code reading only
  (its `Inner` lives in a `'static OnceLock`, same "never dropped in practice" argument, and its
  own `scoped_computation_impl` already asserts the queue is empty post-scope) - not separately
  stress-tested, since it requires a wasm32 + atomics + real browser/worker environment to actually
  exercise `worker_loop`/threaded scoping at all (candidates 010/011 already cover other bugs in
  that same file's wasm-specific machinery).
- `OncePool` (`transient-pool` feature) was checked briefly and does not share this bug class at
  all - it has no persistent worker queue or `Inner::drop` equivalent (a fresh, transient pool is
  created per scope), so this candidate's mechanism does not apply there.
- This candidate does not attempt to construct an *unsafe*-code-assisted reproduction (e.g.
  transmuting/extending the `&'env` borrow past its real lifetime from outside the crate) - that
  would demonstrate a caller violating Rust's own safety contract, not a bug in this crate's safe
  API surface, and was judged out of scope for what this candidate was actually asking about.
- If a future refactor changes `scoped_computation` to be non-blocking/async (so the borrow no
  longer spans the whole in-flight-task lifetime), this finding's "not reproducible" conclusion
  would need to be re-checked from scratch, since the borrow-checker argument it rests on depends
  specifically on today's fully-synchronous, blocking design.
