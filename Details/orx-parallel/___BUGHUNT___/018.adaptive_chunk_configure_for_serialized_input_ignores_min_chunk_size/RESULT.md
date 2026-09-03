# adaptive_chunk_configure_for_serialized_input_ignores_min_chunk_size

`ParRunner::nt_state` (`src/runner/par_runner.rs:84-86`) unconditionally calls
`AdaptiveChunkRunner::configure_for_serialized_input` whenever `I::is_source_serialized()` is
true, regardless of the caller's real `Params.chunk_size`. That function
(`adaptive_chunk_runner.rs:72-76`) hardcodes `ChunkSize::Auto` into the chunk-size heuristic and
never looks at `state.min_chunk_size` (which was correctly derived from the caller's real
`ChunkSize::Min(N)`/`Exact(N)` a few lines earlier in `new_state`). Source: manual code reading
(candidate 018 in the batch), cross-referencing `state.rs`'s `selected_chunk_size`/
`choose_fixed_chunk_size` which do clamp to `min_chunk_size` on every return path once exploration
actually runs, unlike the serialized-input fast path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Code reading | Traced `nt_state` -> `configure_for_serialized_input` -> `set_fixed_chunk_size` -> `next_chunk_size` -> `selected_chunk_size`, comparing against `choose_fixed_chunk_size`'s clamping. | Confirmed by inspection: for `ChunkSize::Min(N)` on a serialized source, `next_chunk_size` returns the `ChunkSize::Auto`-derived heuristic value directly, with no `max(_, min_chunk_size)` clamp anywhere on that path. `ChunkSize::Exact` is unaffected because `next_chunk_size` reads the separate `state.fixed_chunk_size` field first, before ever falling through to the value this bug corrupts. |
| 2. Manual boundary-value construction: a `#[cfg(test)]` module added directly inside `adaptive_chunk_runner.rs` (the affected `ParRunner` trait/`AdaptiveChunkRunner` type live in a private module - `mod runner;` in `src/lib.rs` - unreachable from any public API or an external `tests/` integration test; even `.with_diagnostics()`, a provided method on the public-looking `ParRunner` trait, is not callable externally since the module path to import the trait doesn't exist outside the crate) that calls the real `new_state`/`configure_for_serialized_input`/`next_chunk_size` with `Params{chunk_size: ChunkSize::Min(1000)}`, `size_hint=(8, Some(8))`, 1 thread, `cargo test --lib`, debug-assertions test profile | `chosen_chunk_size` came back as `2` (the `ChunkSize::Auto` heuristic's value: `thread_load = 8/1 = 8`, `8 < 1024` so `8/4 = 2`), well below the requested minimum of `1000`. Two control tests in the same module - `ChunkSize::Exact(1000)` (unaffected, returns exactly `1000`) and `ChunkSize::Auto` (still returns `2`, i.e. the fix doesn't change the Auto path) - both pass in every run, isolating the bug to `ChunkSize::Min` specifically. | 
| 3. Repeat run, identical build/inputs | Same result both times (2/2): `chosen_chunk_size=2` both runs - fully deterministic, no timing/layout dependency (`configure_for_serialized_input` is a one-shot state transition that runs before any real exploration sampling happens, so there is no EWMA/timing noise in this particular path at all). | Confirms bit-for-bit repeatability; no further escalation needed past this second tool. |

## Status

**Confirmed.** Reproduces 2/2 runs, debug-assertions test build (`cargo test`'s default test
profile), deterministic (this code path has no timing dependency - `configure_for_serialized_input`
runs once, synchronously, before any chunk is ever processed, so the same inputs always produce
the same `chosen_chunk_size`).

**Fully reproducible**: **No**, with a stated reason. `AdaptiveChunkRunner`, its `State`, and the
`ParRunner` trait that exposes `new_state`/`configure_for_serialized_input`/`next_chunk_size` all
live under `mod runner;` in `src/lib.rs`, which is a private module - there is no path to reach any
of this from outside the crate, and this was verified directly: attempting to call
`.with_diagnostics()` (itself a provided method on `ParRunner`, which the crate's own doc example
in `new_runner.rs` shows being used as `.runner(Runner::adaptive())`, i.e. *without* needing the
trait in scope, since `.runner()` only requires `P: ParRunner` as a bound, not the trait imported)
from an external `examples/` crate fails to compile with `error[E0603]: module 'runner' is private`
even after importing the compiler's own suggested path. Separately: chunk size selection is a pure
internal scheduling parameter and produces no difference in a computation's final *output* (same
items, same sum, etc. regardless of chunk size), so there is no publicly-observable side effect to
assert on even indirectly - the only way to see `next_chunk_size`'s actual return value is to call
it directly. This is not a case of faking a second participant (CONFIRMATION.md section 11): the
repro's `Params{chunk_size: ChunkSize::Min(N)}` is exactly what a real caller's public
`.chunk_size(ChunkSize::Min(N))` produces and hands to `nt_state`/`new_state` - the only reason this
can't be driven from outside the crate is Rust's own module privacy hiding the internal runner
machinery, not an assumption about what an external caller/peer would send.

## Class

logic

## Root cause

`AdaptiveChunkRunner::new_state` (`adaptive_chunk_runner.rs:41-70`) correctly derives
`state.min_chunk_size` from the caller's real `Params.chunk_size` (`ChunkSize::Auto` -> `1`,
`ChunkSize::Min(n)`/`ChunkSize::Exact(n)` -> `n`). But `ParRunner::nt_state`
(`src/runner/par_runner.rs:84-86`) unconditionally calls
`AdaptiveChunkRunner::configure_for_serialized_input` whenever the source is serialized
(`I::is_source_serialized()`, true for e.g. `ConIterOfIter`, the iter-of-iter wrapper added in
commit b9214fbe/#198), and that function (`adaptive_chunk_runner.rs:72-76`) computes
`heuristic::compute_chunk_size(ChunkSize::Auto, size_hint, state.max_num_threads)` - hardcoding
`ChunkSize::Auto`, discarding the caller's actual chunk-size request entirely - then stores the
result via `state.set_fixed_chunk_size(chunk_size)`, which only does
`chosen_chunk_size.store(chunk_size, Relaxed); mode.complete_exploration();`
(`state.rs:65-68`), never consulting `min_chunk_size`.

Downstream, `next_chunk_size` (`adaptive_chunk_runner.rs:82-90`) checks `state.fixed_chunk_size`
first - this is `Some` only for `ChunkSize::Exact` (set correctly and independently in
`new_state`), so `Exact` never even reaches the corrupted value. For `ChunkSize::Min(N)`,
`state.fixed_chunk_size` is `None`, so it falls to `state.mode()` - already `Fixed` because
`set_fixed_chunk_size` called `complete_exploration()` - and calls
`state.selected_chunk_size(size_hint)`. Since `chosen_chunk_size` is already nonzero (set by the
Auto-derived value above), `selected_chunk_size` (`state.rs:147-156`) returns it directly with
**no** `max(_, min_chunk_size)` clamp - unlike the normal exploration-derived path,
`choose_fixed_chunk_size` (`state.rs:170-196`), whose every return path does clamp with
`max(_, self.min_chunk_size)`. Net effect: a caller who explicitly requests
`.chunk_size(ChunkSize::Min(N))` on a serialized source silently gets a chunk size chosen purely by
the `Auto` heuristic (as low as 1-2 in the repro), ignoring their explicit minimum entirely.

## Introduced by

b9214fbe "iter of iter uses fixed chunk heuristic (#198)" (2026-08-30)

`git log -S "configure_for_serialized_input" -- src/runner/runner_variants/adaptive_chunk/adaptive_chunk_runner.rs`
shows this is the commit that introduced both the `configure_for_serialized_input` method itself
and its `ChunkSize::Auto`-hardcoded body - this is also the repository's current HEAD, i.e. the bug
has been present since the feature it belongs to was introduced and has not yet been touched since.

## The fix

Changed `configure_for_serialized_input` (`adaptive_chunk_runner.rs:72-76`) from hardcoding
`ChunkSize::Auto` to reconstructing a `ChunkSize::Min` from the state's own already-correct
`min_chunk_size` field:

```rust
fn configure_for_serialized_input(state: &mut Self::State, size_hint: (usize, Option<usize>)) {
    let min_chunk_size = NonZeroUsize::new(state.min_chunk_size).unwrap_or(NonZeroUsize::MIN);
    let chunk_size = heuristic::compute_chunk_size(
        ChunkSize::Min(min_chunk_size),
        size_hint,
        state.max_num_threads,
    );
    state.set_fixed_chunk_size(chunk_size);
}
```

This is sufficient without special-casing `ChunkSize::Auto`/`Exact` separately, because
`state.min_chunk_size` is already `1` for the `Auto` case (verified by the
`serialized_input_auto_chunk_size_still_uses_heuristic` control test: `heuristic::min_chunk_size`
is `max(min_chunk, auto_chunk_size(...))`, and `max(1, x) == x` for any `x >= 1`, so `Auto`'s
behavior is completely unchanged) and the `Exact` case's result is provably irrelevant regardless
of what this function computes, since `next_chunk_size` never reads `chosen_chunk_size` for
`Exact` (see "Root cause" above; verified by the `serialized_input_exact_chunk_size_is_unaffected`
control test passing both before and after this change).

Verified against the primary repro (`ChunkSize::Min(1000)`, 8-item input, 1 thread) both directly
in the working tree and via `just test` from a fresh clone
(`repro/add_repro_test.diff` then `patch.diff`, see `output.txt`) - `just test`'s
`reproduce-before` step fails with the documented panic (exit code 101, the expected reproduction),
`apply` + `reproduce-after` then pass all 3 tests.

- Project test suite: `cargo test --lib`: 101 passed / 1 failed (this candidate's own added repro
  test; all 99 pre-existing lib tests plus the 2 added control tests already passed) before the
  fix -> 102 passed / 0 failed after. `cargo test --doc`: 260 passed / 0 failed / 16 ignored,
  unaffected by this change (run only after the fix; the pristine `--lib` run above already
  establishes the 99 pre-existing lib tests are unaffected either way).

## What's not done / open questions

- This is the only call site of `configure_for_serialized_input` for the adaptive-chunk runner
  variant (`FixedChunkRunner`'s own `configure_for_serialized_input` - not investigated here - is a
  structurally different implementation that may or may not share this exact pattern; worth a
  separate look if a future session wants to check the fixed-chunk runner's serialized-input path
  for the same class of "ignores the caller's requested minimum" bug).
- Not independently verified whether any currently-shipping call path other than
  `ConIterOfIter`/iter-of-iter (`I::is_source_serialized() == true`) actually exercises this
  function today - the fix is general (keys off `state.min_chunk_size`, not off any
  source-specific detail), so it should be correct regardless, but this was not enumerated
  exhaustively across every `ConcurrentIter` implementation that returns `true` from
  `is_source_serialized()`.
- No broader fuzzing/property-based sweep across arbitrary `(N, size_hint, max_num_threads)`
  combinations was run beyond the one boundary value (`N=1000`, 8-item input, 1 thread) plus the
  two control cases (`Exact`, `Auto`) - the root cause is a straightforward "wrong hardcoded
  variant, no clamp" logic bug with no arithmetic edge cases of its own, so this was judged
  sufficient without an additional fuzz harness.
