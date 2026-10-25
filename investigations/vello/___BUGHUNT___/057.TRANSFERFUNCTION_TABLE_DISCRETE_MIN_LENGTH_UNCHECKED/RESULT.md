# TRANSFERFUNCTION_TABLE_DISCRETE_MIN_LENGTH_UNCHECKED

`sparse_strips/vello_common/src/filter_effects.rs:976-989`. `TransferFunction::Table` and
`TransferFunction::Discrete` each carry a public `values: Vec<f32>` field whose doc comments
document a "minimum 2 values required" (`Table`) / "input range is divided into `len(values)`
segments" (`Discrete`) invariant, with no constructor, no validation, and no assertion anywhere in
the crate enforcing it. The suspicion (from `IDEAS.md`) was that a future component-transfer
evaluator doing `values[0]`/`values[n-1]`/segment-division indexing on an empty or 1-element `Vec`
would panic or read out of bounds.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Whole-workspace grep for any consumer of `TransferFunction`/`ComponentTransfer` | `grep -rn` across every `*.rs` in the repo (including `vello_hybrid`, `vello_cpu`, `vello_example_scenes`, `vello_sparse_tests`, all test files) | Zero matches for `TransferFunction::Table`/`TransferFunction::Discrete` construction or consumption anywhere outside the type's own definition in `filter_effects.rs`. `ComponentTransfer` itself (the enum variant that carries `TransferFunction`s) is referenced in exactly one place besides its own definition: `sparse_strips/vello_common/src/filter/mod.rs`'s `PreparedFilter::new`, where it falls into the catch-all `_ => unimplemented!("Other filter primitives not yet implemented")` arm, i.e. it is not evaluated at all yet, for any content. |
| Standalone runtime probe (`vello_common` as a path dependency, both debug and `--release` builds) constructing a deliberately invariant-violating `TransferFunction::Table { values: vec![] }` inside a real `FilterPrimitive::ComponentTransfer`, driven through the real public API (`Filter::from_primitive`, `Filter::filter_expansion`/`source_expansion`, then `PreparedFilter::new`) | `cargo run` (debug, debug-assertions on) and `cargo run --release`, 3 repeat runs each | `filter_expansion`/`source_expansion` both return `Rect::ZERO` without ever touching `values` (both fall into `FilterPrimitive`'s own catch-all `_ => Rect::ZERO` arm for `ComponentTransfer`). `PreparedFilter::new` panics deterministically (3/3 runs, both builds) with `"not implemented: Other filter primitives not yet implemented"` — the same panic that fires for *any* `ComponentTransfer`, including one using only `TransferFunction::Identity` (no `values` field at all). No panic or misbehavior traceable to `values`'s length occurred in any run. |
| Project issue tracker cross-check (`___BUGHUNT___/__issues_raw/`, synced this session) | Read `issues/1695.json` ("Support CSS `filter()` effects") and `issues/1296.json` ("Implement remaining SVG filter primitives/CSS filter functions") | Both confirm `ComponentTransfer` evaluation is a still-open, not-yet-started tracked task upstream (checkbox unchecked in both issues) — consistent with the grep/runtime findings that no evaluator exists yet anywhere in this checkout, and no in-flight PR is adding one either (no PR in the synced cache mentions `ComponentTransfer`). |

## Status

**Not reproducible.**

There is currently no code anywhere in the workspace that reads the `values` field of
`TransferFunction::Table`/`Discrete` at all (no `values[0]`, no `values[n-1]`, no segment-division
indexing), so the specific failure mode this candidate suspected (an evaluator indexing an
empty/1-element `Vec`) cannot be triggered — the evaluator that would do this indexing does not
exist yet. This was verified three ways: exhaustive grep (no consumer found), a real runtime probe
driving a deliberately invalid `values: vec![]` through the actual public `Filter`/`PreparedFilter`
API (no `values`-related panic in 3/3 debug and 3/3 release runs — the only panic observed is a
pre-existing, content-independent `unimplemented!()`), and the project's own open issue tracker
confirming the evaluator is a still-unstarted tracked task.

**Fully reproducible**: N/A (not Confirmed).

## Class

N/A (not Confirmed).

## Root cause (why this turned out to be safe, for now)

`ComponentTransfer` (the only `FilterPrimitive` variant that carries a `TransferFunction`) is
caught by `PreparedFilter::new`'s catch-all arm in `sparse_strips/vello_common/src/filter/mod.rs`:

```
_ => {
    // Other primitives like Blend, ColorMatrix, ComponentTransfer, etc.
    // are not yet implemented
    unimplemented!("Other filter primitives not yet implemented");
}
```

This panics unconditionally for `ComponentTransfer`, before any code would ever get a chance to
index into a `TransferFunction::Table`/`Discrete`'s `values`. Separately, `FilterPrimitive::filter_expansion`/
`source_expansion` (`filter_effects.rs:578-644`, the only other code that runs on a `FilterPrimitive`
before `PreparedFilter::new`) also treat `ComponentTransfer` via their own `_ => Rect::ZERO`
catch-all, so they don't touch `values` either. The documented "minimum 2 values" / per-segment
invariant is real and will matter once a component-transfer evaluator is actually wired up (tracked
upstream in issues #1695 and #1296, both still open/unstarted), but as of this checkout there is
no reachable path that currently violates it — the field is inert data, not yet load-bearing.

## Introduced by

N/A (not Confirmed) — the `TransferFunction` enum itself (with its documented-but-unenforced
invariant) was added in ee4394aa "feat(vello_cpu): Image Filters (Filter Effects) implementation
(#1286)", per `git log -S TransferFunction`. It has never been consumed by any evaluator since.

## The fix

No fix attempted — there is nothing to fix yet, since no code currently reads `values` in a way
that could violate the documented invariant. When a component-transfer evaluator is eventually
implemented (per open issues #1695/#1296), it should validate `values.len() >= 2` for `Table`
(returning/falling back to identity, or asserting, on a shorter vec) and `values.len() >= 1` for
`Discrete`, or the type should gain a validating constructor, at that point — not before, since
adding runtime validation to a field nothing yet reads would be speculative.

- Project test suite: N/A — no fix was made, so no before/after comparison applies. No source in
  the main project's working tree was modified this session.

## What's not done / open questions

- This candidate is inherently about a *future* code path (an evaluator that doesn't exist yet).
  A future session should re-check this exact spot once issue #1695/#1296 lands a real
  component-transfer evaluator — at that point the same invariant becomes load-bearing and worth
  re-investigating for real (boundary values 0/1/2 elements) against the actual indexing code that
  will exist then.
- Did not check whether any *other* language binding or downstream consumer outside this repo
  (e.g. a hypothetical SVG-parsing frontend not present in this checkout) constructs
  `TransferFunction` values directly and could reach a hypothetical evaluator that way — out of
  scope, since no such evaluator exists in this repo to reach.
