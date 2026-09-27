# wasm_web_start_workers_premature_ready

`src/pool/pool_impl/wasm_web_start_workers.js` is the wasm-web worker bootstrap script loaded by
each spawned Worker in the browser-hosted threaded runtime (`src/pool/pool_impl/wasm_web.rs`'s
`init_wasm_thread_pool`/`start_workers`). Inside the worker's init handler, `postMessage({ type:
"orx_parallel_worker_ready" })` is sent *before* checking `typeof pkg.wasm_web_start_worker !==
"function"`. Since the main thread's `startWorkers()` uses `Promise.race([onWorkerReady,
onWorkerInitError, onWorkerError, onWorkerTimeout])` per worker, a worker whose `pkg` module lacks
`wasm_web_start_worker` announces itself ready, wins the race, and only then throws and posts
`orx_parallel_worker_error` - a message nothing is still listening for by that point.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Read the full bootstrap handler + `startWorkers()`'s `Promise.race` | Statement order is unconditional: `postMessage(ready)` always precedes the `typeof` check on every execution path through the `try` block - not a race/timing artifact, a straight-line ordering bug |
| Real browser reproduction (Chromium via Playwright, real `Worker` thread, real `postMessage`) | Real, unmodified `wasm_web_start_workers.js` served from a local Node static server at the same relative nesting depth wasm-bindgen's local-snippet packaging uses (`snippets/<pkg>-<hash>/src/pool/pool_impl/...`), imported and driven by its real, unmodified exported `startWorkers()`; a minimal stub `pkg` module (real `default(init)` succeeds, `wasm_web_start_worker` intentionally absent, simulating any build/artifact mismatch that leaves that export missing) | `startWorkers()` resolves (init reported as successful) after ~11-15ms; a Worker-object spy (installed by subclassing `window.Worker` before importing the module under test, observing the *real* messages the real worker thread posted, since `startWorkers()` doesn't expose the underlying `Worker` instances) shows the real message order was `orx_parallel_worker_ready` then `orx_parallel_worker_error`. 5/5 repeat runs, deterministic every time |

Soft cap not needed - the second approach reproduced immediately and deterministically; no third approach was required.

## Status

**Confirmed**. Reproduces 5/5 runs (real Chromium, real Worker thread, no fixed seed involved -
the bug is a straight-line statement-ordering defect, not timing-dependent, so it is expected to
be 100% deterministic, and observed as such every run).

**Fully reproducible**: **Yes**. `repro/` runs the real, unmodified
`src/pool/pool_impl/wasm_web_start_workers.js` file end to end: a real Chromium browser (via
Playwright) loads a page that imports the file's real exported `startWorkers()` and calls it for
real; `startWorkers()` spawns a real `Worker` (a second, independently-scheduled thread) which runs
the file's real bootstrap handler for real, and the two communicate over the real `postMessage`/
`message`-event channel - nothing about the ready/error message exchange itself is hand-simulated.
The only stand-in is the `pkg` module the worker `import()`s (the wasm-bindgen generated package,
not shipped by this repo in JS form) - it is a minimal 3-line stub whose `default(init)` resolves
and which simply omits `wasm_web_start_worker`, i.e. it supplies the *input condition* the bug
depends on (an artifact where that export is missing), not a fake of the worker's own reaction to
it - the worker's actual reaction (posting ready, then throwing, then posting error) is the crate's
own real code running for real. See "What's not done" below for the one thing this repro
deliberately works around (an unrelated, already-broken filename reference in the same file), which
does not touch the ordering logic under test.

## Class

logic

## Root cause

In the worker bootstrap handler (lines ~11-24 of the file at the base commit):

```js
const pkg = await import("../../../../..");
await pkg.default(init);
postMessage({ type: "orx_parallel_worker_ready" });
if (typeof pkg.wasm_web_start_worker !== "function") {
    throw new Error("wasm worker entrypoint is missing: expected wasm_web_start_worker");
}
pkg.wasm_web_start_worker();
```

The "ready" signal is posted unconditionally right after the wasm module's own `default(init)`
promise resolves, before the code has verified the worker actually has a usable entrypoint to run.
`startWorkers()`'s `Promise.race([onWorkerReady, onWorkerInitError, ...])` treats the first
settled promise as the final outcome for that worker - once "ready" wins, the subsequent
`orx_parallel_worker_error` message (and the `onWorkerInitError` promise racing against it) is
still delivered to the main thread's already-attached listener, but nothing is awaiting it anymore:
the caller of `init_wasm_thread_pool`/`start_workers` on the Rust side receives a resolved promise
and proceeds as if that worker is live and running `worker_loop`, when in fact that worker's async
IIFE already returned after the `catch` branch and will never process any task. Any work later
routed to that worker would go unprocessed with the pool believing initialization fully succeeded.

## Introduced by

d60d50f10c0d8b904be66ba661c4161ab0a76b09 "fix orx initializer" (2026-07-11). Before this commit,
the file (then named `worker_helpers.js`) unconditionally called `pkg.wasm_web3_start_worker()`
right after `postMessage(ready)`, with no existence check at all - so the missing-entrypoint case
simply threw a `TypeError` from calling a non-existent function, after the same premature `ready`.
This commit added the explicit `typeof` guard and error message, but appended it *after* the
pre-existing `postMessage(ready)` line instead of moving the ready signal to after the check - so
the same premature-ready ordering defect that existed informally before this commit was carried
forward into the new, more explicit guard, unchanged.

## The fix

Move the `postMessage({ type: "orx_parallel_worker_ready" })` call to after the
`wasm_web_start_worker` existence check (see `patch.diff`): the worker now only announces itself
ready once it has confirmed it can actually run, and `startWorkers()`'s `Promise.race` now
correctly rejects (via `onWorkerInitError`) instead of resolving, for the same input.

Re-running `repro/` against the patched file (5/5 runs): `startWorkers()` now rejects with
`worker 0 init failed: wasm worker entrypoint is missing: expected wasm_web_start_worker`, and the
worker-spy shows only a single `orx_parallel_worker_error` message with no `orx_parallel_worker_ready`
ever posted first. See `output.txt` for a captured before/after run.

- Project test suite (`cargo test --lib`): 99 passed; 0 failed - before the patch. 99 passed; 0
  failed - after the patch (identical result; this file has no Rust-side test coverage, the
  patch only reorders two JS statements with no Rust-visible effect).

## What's not done / open questions

- While building this repro, a **separate, unrelated** defect in the same file was noticed: the
  July 20 2026 rename commit (ea3f98f1, "renaming") renamed `worker_helpers.js` to
  `wasm_web_start_workers.js`, but the file's own `startWorkers()` still spawns its worker via
  `new Worker(new URL("./worker_helpers.js", import.meta.url), ...)` - a filename that no longer
  exists anywhere in the tree. This repro works around it by serving the identical real file
  content under both the real and the stale filename (see the justfile's `reproduce-before`/
  `reproduce-after` comments) purely so the worker can load at all; it does not touch the ordering
  logic this candidate is about. This looks like it would break worker spawning entirely in any
  real deployment and is worth its own separate candidate - added to `IDEAS.md`, not investigated
  further here (one bug per candidate).
- Not checked: whether any other `Promise.race`-style "first signal wins" pattern exists elsewhere
  in the wasm-web runtime's JS or Rust glue that could have the same class of premature-success
  issue - only this one bootstrap handler was examined.
- `repro/`'s Playwright/Chromium dependency needs `npm install` (network access to the npm
  registry) to run from a completely fresh clone; a `node_modules/playwright` (with its Chromium
  browser binary already downloaded) was available in this sandbox and used directly.
