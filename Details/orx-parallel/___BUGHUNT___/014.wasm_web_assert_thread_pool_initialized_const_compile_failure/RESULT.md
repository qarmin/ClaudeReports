# wasm_web_assert_thread_pool_initialized_const_compile_failure

`src/pool/pool_impl/wasm_web.rs:311-317`'s `assert_wasm_thread_pool_initialized()` wraps its
atomics-feature check in an inline `const { assert!(cfg!(target_feature = "atomics"), ...) }`
block. This looked suspicious because the same function's very next statement (`assert_eq!` on
`WASM_WEB3_THREAD_POOL_STATE`) is an ordinary runtime check, suggesting the surrounding design
intends a runtime error for a misconfigured/uninitialized pool rather than a hard compile failure -
flagged as tractable (purely compile-time-checkable, no worker/browser harness needed) in this
project's bug-hunt risk map (`projects/orx-parallel.md`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct build of the crate itself | `cargo build --target wasm32-unknown-unknown --features wasm --lib` (in-tree, no atomics RUSTFLAGS) | Hard compile failure, error[E0080], first try |
| Repeat-run for determinism | Same command, run twice in a row, diffed full output | Byte-identical output both times, exit code 101 both times |
| Real end-to-end downstream repro | A separate `repro/` crate depending on `orx-parallel` via `path = "../clone"` with `default-features = false, features = ["wasm"]` (exactly docs/wasm.md's documented dependency snippet), built for wasm32 without atomics RUSTFLAGS | Same E0080 failure, transitively, through the real dependency-compilation path a downstream consumer would hit |
| Sanity check: atomics-enabled build | Same in-tree crate, `RUSTFLAGS='-C target-feature=+atomics ...' cargo build --target wasm32-unknown-unknown --features wasm --lib -Z build-std=panic_abort,std` (nightly, per docs/wasm.md's own example command) | Builds cleanly, confirming the const block's condition is genuinely reachable-true on the documented atomics build and the bug is specific to the non-atomics case |

Only one real escalation was needed (a plain build attempt) since this is a deterministic
compile-time constant-evaluation failure, not a runtime/input-dependent condition - there is no
flakiness for a second or third tool to help characterize. The downstream-crate build and the
atomics-enabled sanity check were done as confirmation/scoping steps, not as separate "escalating
tools" in the miri/ASAN/fuzz sense (none of those apply to a compile-time `const` assertion).

## Status

**Confirmed.** Reproduces 3/3 runs (2 direct in-tree runs + 1 through the standalone `repro/`
downstream crate), byte-identical output every time - this is `rustc` evaluating a `const {}`
block at compile time, which is inherently deterministic, not a flaky runtime condition.

**Fully reproducible**: **Yes**. The finding is about the crate author's own build (not one
participant reacting to another's runtime input), so there is no second-participant concern here
(see CONFIRMATION.md section 11's scope - it targets peer/caller interactions, not a compiler
evaluating a crate's own source under a stated feature/target combination). `repro/` is a real,
standalone downstream crate that depends on the unmodified `orx-parallel` crate through its real
public API and Cargo's real dependency-resolution/build pipeline, and is driven through the exact
documented consumer workflow (`docs/wasm.md`'s "Which feature to enable" snippet, then a plain
`cargo build --target wasm32-unknown-unknown` with no special RUSTFLAGS) - not a hand-simulated
stand-in for what a consumer would do.

## Class

logic (a design/behavior mismatch between "how the code degrades" as documented vs. "what happens
at compile time" - not a memory-safety/UB/crash-at-runtime issue; the affected artifact is the
build itself, not a running program).

## Root cause

`mod wasm_web;` (and everything it exports, including `WasmWebPool`) is compiled whenever
`feature = "wasm"` and `target_arch = "wasm32"` hold - see `src/pool/pool_impl/mod.rs:24-25`. This
is deliberately *not* additionally gated on `target_feature = "atomics"`: only the
`init_wasm_parallel_runtime`/`wasm_web_runtime_info`/`wasm_web_start_worker` exports are (mod.rs
lines 28-36), so that a non-atomics wasm32 build can still see the `WasmWebPool` type (its
`ParThreadPool` impl exists unconditionally) and get a normal runtime panic if it's ever actually
used - exactly matching `docs/wasm.md`'s own documented wrapper pattern ("If the build is not
atomics-enabled, `init_wasm_parallel_runtime(...)` is not available") and the pre-existing runtime
`assert_eq!` two lines below the suspect block.

`assert_wasm_thread_pool_initialized()` is a plain (non-generic) free function, not a top-level
`#[cfg(...)]`-gated item, so its whole body - including any inline `const {}` block inside it - is
evaluated by rustc as part of ordinary MIR construction for *every* build where this function is
part of the crate, independent of whether anything anywhere in the compiled crate/binary graph
actually calls it. Wrapping the feature check in `const { assert!(cfg!(target_feature =
"atomics"), ...) }` therefore turns "the crate contains a function that would panic at runtime if
called on a non-atomics build" into "the crate fails to compile at all whenever `feature = "wasm"`
+ `target_arch = "wasm32"` hold without `target_feature = "atomics"`" - which is the normal,
plain, undecorated `cargo build --target wasm32-unknown-unknown --features wasm` a consumer would
run first, before ever opting into the documented, separate atomics RUSTFLAGS step. The crate does
not even need `WasmWebPool` to be instantiated or any parallel computation to be attempted; merely
compiling the library (or any crate that depends on it with this feature/target combination) is
enough to hit the `error[E0080]`.

A plain runtime `if !cfg!(target_feature = "atomics") { panic!(...) }` (or an `assert!` without the
`const {}` wrapper) does not have this problem: `cfg!()` still resolves to a compile-time-constant
`true`/`false`, but the `assert!`/`panic!` macro expansion itself is ordinary runtime code, only
reached if and when `assert_wasm_thread_pool_initialized` (and therefore `runtime()`, and
therefore `WasmWebPool`'s scoped-computation methods) is actually called at runtime.

## Introduced by

a245335a "clippy fixes" - this commit changed a plain `assert!(cfg!(target_feature = "atomics"),
...)` (present since 71754b5e "init_thread_pool is implemented and exposed", where it was
originally an `if !cfg!(...) { panic!(...) }`) into the current `const { assert!(...) }` form,
apparently to silence clippy's `assertions_on_constants` lint (which fires once `cfg!()` has
macro-expanded to a literal `true`/`false` token seen by `assert!`) - without noticing that
wrapping in `const {}` changes a lint-only style tweak into a behavior change (runtime panic ->
compile-time hard error). Confirmed via `git log -S 'const {' --oneline --
src/pool/pool_impl/wasm_web.rs` plus reading that commit's diff for this exact hunk.

## The fix

Removed the `const {}` wrapper, restoring a plain runtime `assert!(cfg!(target_feature =
"atomics"), "...")`, and moved `#[allow(clippy::assertions_on_constants)]` onto the enclosing
function (an attribute directly on the `assert!` statement is silently ignored by rustc - it
warned `unused_attribute` when tried inline, since the attribute lands on the macro invocation
itself) so the same clippy lint a245335a was presumably reacting to stays silenced without
reintroducing the compile-time hard failure:

```rust
#[allow(clippy::assertions_on_constants)]
fn assert_wasm_thread_pool_initialized() {
    assert!(
        cfg!(target_feature = "atomics"),
        "Wasm web threading requires atomics-enabled wasm build flags; see docs/wasm.md."
    );

    assert_eq!(
        WASM_WEB3_THREAD_POOL_STATE.load(Ordering::SeqCst),
        WASM_WEB3_THREAD_POOL_INITIALIZED,
        "Wasm web thread pool is not initialized. Call and await init_wasm_parallel_runtime(...) before running parallel computations."
    )
}
```

Verified:

- `cargo build --target wasm32-unknown-unknown --features wasm --lib` (in-tree): fails before
  (E0080, exit 101) -> succeeds after (exit 0, zero warnings).
- `repro/` standalone downstream crate (`cargo build --target wasm32-unknown-unknown`): fails
  before -> succeeds after (see `output.txt`).
- Atomics-enabled build (docs/wasm.md's own documented RUSTFLAGS + nightly `-Z
  build-std=panic_abort,std`) still builds cleanly with the patch applied (only pre-existing,
  unrelated warnings) - the fix does not disturb the intended atomics-enabled path.
- Project test suite (`cargo test --lib`, native x86_64 target, where `wasm_web.rs` is not even
  compiled - `cfg(target_arch = "wasm32")`): 99 passed / 0 failed, identical before and after.

## What's not done / open questions

- The runtime panic message and behavior for someone who *does* build with atomics disabled and
  then actually calls a `WasmWebPool` method were not separately exercised at runtime (would need
  a browser/worker harness, out of scope for this purely compile-time-checkable candidate per the
  task description) - only that the crate now compiles again; the pre-existing `assert_eq!`
  right below it (unmodified by this patch) already covers the "not yet initialized" runtime case
  and was not touched.
- Did not check whether any other `const { assert!(cfg!(...)) }` pattern exists elsewhere in the
  crate with the same footgun - a targeted `grep -rn 'const {' src` at the time of this candidate
  found only this one occurrence in `wasm_web.rs`.
- Candidates 010/011 (this project's own prior work) cover the JS-side worker-bootstrap races in
  the same file's runtime behavior; this candidate is unrelated to those and does not affect or
  depend on them.
