# wasm_web_pool_scoped_computation_active_scope_addr_aliasing

`WasmWebPool::scoped_computation_impl` (`src/pool/pool_impl/wasm_web.rs:360-403`) publishes the
currently-active scope's `ScopeRuntime` as a single process-wide address
(`WorkerState::active_scope_addr`) instead of giving each queued `Task` its own reference to its
scope's runtime (unlike `BasicPool`'s `Task`, which carries its own `Arc<ScopeRuntime>` - see
`src/pool/pool_impl/basic.rs`). This looked suspicious because a second, nested (reentrant) call
to `WasmWebPool::scoped_computation` - made through the crate's real public
`ParThreadPool::scoped_computation` entry point, while an outer call is still on the stack -
overwrites that single slot, and the only guard against it is a `debug_assert!` that is compiled
out in release builds. Source: `___FINDINGS___/findings.toml`, via `IDEAS.md`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Code reading + comparison with `BasicPool`'s already-correct design (per-task `Arc<ScopeRuntime>` instead of a single shared slot) | N/A | Confirmed the design asymmetry: `BasicPool` supports concurrently-active scopes safely by construction; `WasmWebPool` does not, and the only thing standing in the way of a nested call silently aliasing the slot is a `debug_assert!`. |
| 2. Attempted a real browser (`playwright install chromium`) to drive the existing `wasm_bindgen_test(run_in_browser)` harness end-to-end for the strongest possible (genuinely cross-worker-thread) variant | `npx playwright install chromium`, ~5 minutes across two attempts (120s and 300s timeouts, plus one `nohup` background attempt left running ~3 minutes with no progress) | Download never completed in this sandboxed environment (no visible network activity on the process after the initial redirect, no error either) - abandoned this specific escalation; did not block the investigation since the candidate's own description already lists "reentrant" (not just cross-thread "concurrent") as a valid trigger, and that path doesn't need a browser at all (see below). |
| 3. Real end-to-end reproduction via a whitebox test hook (see `repro/add_repro_hooks.diff`) that calls the actual public `ParThreadPool::scoped_computation` entry point, nested, on the same thread - compiled to a real wasm32+atomics binary (nightly + `-Z build-std=panic_abort,std`, matching this project's own CI wasm build flags) and executed under a real wasm engine (`wasmtime` 27.0.0, embedded via its Rust API with the WASM threads/shared-memory proposal enabled) - no browser, no hand-authored bytes standing in for a caller: the entry point is invoked directly, nested, by one real caller | debug-assertions build (cargo dev profile default), 3 binaries: no nesting (baseline), nested once (2 levels), nested twice (3 levels); 3 repeat runs of each | Baseline completes normally every time (3/3). Both nested cases (2 and 3 levels) trap every time (3/3 each) with an `unreachable` instruction inside `scoped_computation_impl`'s call chain (wasm backtrace shows the trap originating in the inner, nested `scoped_computation_impl` call, invoked from inside the outer scope's still-running closure) - this is the `debug_assert!(state.active_scope_addr.is_none())` firing. |
| 4. Causal isolation: temporarily disabled just that one `debug_assert!` line (simulating what release builds do, since `debug_assert!` is compiled out there) and re-ran the nested case | Same debug build, same nested binary, only the one assert line removed | The nested call now completes silently with no error at all (return code 0) - confirming this is exactly the guarded invariant, and that release builds have no defense against it whatsoever (silent state corruption, not just a debug-only panic). |

## Status

**Confirmed.** Reproduces 3/3 runs for both the 2-level and 3-level nesting cases (deterministic -
this is pure single-threaded reentrancy, not layout/seed-dependent), debug-assertions build,
`wasm32-unknown-unknown` + `atomics`/`bulk-memory`/`mutable-globals`, executed under `wasmtime`
27.0.0 with `wasm_threads(true)`. The baseline (non-nested) case completes normally 3/3.

**Fully reproducible**: **Yes**, for the reentrant trigger the candidate description itself lists
("two concurrent/**reentrant** `WasmWebPool::scoped_computation` calls"): the repro drives the
real, unmodified `scoped_computation_impl` through the crate's real public
`ParThreadPool::scoped_computation` trait method, called a second time by one real caller while
the first call is still active - no second participant is faked, since there is only one caller
making a real nested call (see `CONFIRMATION.md` section 11's carve-out for single-participant
repros). The other half of the candidate's description - genuinely **concurrent** cross-worker-
thread calls racing against each other - was not independently exercised this session (no headless
browser available to drive the project's own `wasm_bindgen_test(run_in_browser)` tests, and
`std::thread::spawn` is unsupported on this project's `wasm32-unknown-unknown` + custom-worker
target combination outside of the browser's own Worker bootstrap); see "What's not done" below.

## Class

crash (reentrancy / shared-state aliasing; silently corrupts state instead of crashing once the
one guarding `debug_assert!` is compiled out in release)

## Root cause

`scoped_computation_impl` stores the address of its stack-local `ScopeRuntime` into
`WorkerState::active_scope_addr`, a single field shared by the whole pool
(`src/pool/pool_impl/wasm_web.rs:364-376`, pre-fix). `worker_loop` later reads that same field to
find which `ScopeRuntime` a dequeued `Task` belongs to
(`src/pool/pool_impl/wasm_web.rs:207-234`, pre-fix). Both reads/writes assume at most one scope is
ever active pool-wide, an invariant enforced only by `debug_assert!(state.active_scope_addr.is_none())`.
A second, nested call to `scoped_computation` (made from inside the first call's still-running
user closure, before it calls `wait_for_completion`) overwrites that slot with its own
`ScopeRuntime`'s address. In a debug build this trips the `debug_assert!` immediately on entry
(confirmed in approach 3 above). In a release build (approach 4 above) it does not: the inner
scope silently takes over the slot, and when the inner scope finishes it clears the slot back to
`None` - even though the outer scope is still conceptually active. Any task the outer scope queues
afterwards (or any task of the outer scope still in flight when a worker dequeues it while the
slot points at the inner scope) would have its completion attributed to the wrong `ScopeRuntime`
(the outer scope's own pending-count could then never reach zero, hanging its
`wait_for_completion` forever, or a worker could dereference a `ScopeRuntime` that has already
returned from `scoped_computation_impl` and may no longer be valid). `BasicPool`
(`src/pool/pool_impl/basic.rs`) does not have this problem because each `Task` there carries its
own `Arc<ScopeRuntime>` clone directly - there is no shared "which scope is active" slot to alias
in the first place.

## Introduced by

c76762bc84010656f193f68a75f4c1bc67e2e144 "reorganization" (2026-07-10). This commit replaced the
original `rayon_core::scope`-backed `WasmWebPool` (which had no scope-tracking state of its own -
Rayon owned all of that) with the current custom worker-backed runtime, introducing
`WorkerState::active_scope_addr` and the single-slot design from scratch. Found via
`git log -S "active_scope_addr" -- src/pool/pool_impl/wasm_web.rs`, which returns exactly this one
commit, and confirmed by inspecting the commit's diff (adds the field, `worker_loop`'s
`.expect("active scope must be set...")` read, and the `scoped_computation_impl` writes, all in
one hunk).

## The fix

Ported `BasicPool`'s already-correct design: give each scope its own heap-allocated
`Arc<ScopeRuntime>` and hand a clone of it directly to every `Task` created under that scope,
instead of publishing a single process-wide "active scope" address that every worker has to look
up through shared, mutable pool state.

- `WorkerState::active_scope_addr` removed entirely - there is no longer a single "which scope is
  active" slot to alias.
- `ScopeRef::runtime` changed from `*const ScopeRuntime` to `Arc<ScopeRuntime>` (plus a
  `runtime_arc()` accessor for cloning it into a `Task`).
- `Task` gains its own `runtime: Arc<ScopeRuntime>` field, set at `Task::new(work, runtime)` time
  (mirroring `BasicPool::Task`).
- `worker_loop` reads `task.runtime` directly instead of looking up a runtime pointer from shared
  pool state.
- `scoped_computation_impl` allocates `Arc::new(ScopeRuntime::new())` instead of a stack-local
  `ScopeRuntime`, and no longer touches any "active scope" bookkeeping at all (so the removed
  `debug_assert!(state.queue.is_empty())` at scope end goes with it too - it was only valid under
  the old single-active-scope assumption; with per-task runtimes, the queue may legitimately still
  hold another, still-active scope's tasks).

See `patch.diff` (net +24/-28 lines against `src/pool/pool_impl/wasm_web.rs`, no other files
touched).

Verified: re-running the same 2-level and 3-level nested repro binaries against the patched tree -
both now complete normally (3/3 runs each), same as the baseline (see `output.txt` and
`repro/add_repro_hooks.diff` + `repro/`'s own driver, which run this exact before/after cycle from
a fresh clone via `just test`).

- Project test suite (native, `cargo test --release`; this bug is wasm32-only so the wasm-only
  code paths themselves aren't covered by the native suite): **439 passed, 0 failed, 16 ignored
  (doctests) both before and after the fix** - identical counts, no regressions. `cargo +nightly
  check --target wasm32-unknown-unknown --no-default-features --features wasm -Z
  build-std=panic_abort,std` (mirroring this project's own CI "Check-wasm" job) also still passes
  after the fix.

## What's not done / open questions

- The genuinely **concurrent**, cross-worker-thread half of the candidate's own description (two
  real Worker threads each running `worker_loop`, one of them calling `scoped_computation` while
  another scope is mid-flight) was not independently exercised this session - it needs either a
  real headless browser (unavailable in this sandbox; a `playwright install chromium` attempt did
  not complete, see "What was tried" row 2) or a from-scratch replication of wasm-bindgen's
  multi-instance-sharing-one-`SharedMemory` bootstrap under `wasmtime` (judged out of scope for one
  session). The root cause and the fix are identical either way (there is exactly one shared slot
  either a nested call on one thread or a second worker thread can alias), so this is not expected
  to behave differently, but it was not directly observed.
- Not checked whether any other backend shares this "single shared slot" pattern - `BasicPool`
  already uses the correct per-task `Arc<ScopeRuntime>` design (that's exactly what this fix
  copies), and a quick look at `OncePool`/`SequentialPool`/`rayon_core.rs` didn't turn up an
  equivalent slot, but a full audit of those backends for the same bug class wasn't performed as
  part of this candidate.
- The temporary whitebox test hook added for the repro (`__bughunt_repro_nested_scope[_depth]` in
  `repro/add_repro_hooks.diff`) is investigation-only scaffolding, not part of `patch.diff` - it is
  not meant to be merged.
