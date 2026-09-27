# PrimitiveValue::truncate(0) silently ignores single-valued Str elements

Found during a discovery round (no seeded lead): while auditing `object/src/mem.rs`'s
`InMemDicomObject::apply` (the `AttributeOp`/`AttributeAction` application logic, used for
attribute editing - e.g. de-identification pipelines), the `AttributeAction::Truncate(limit)`
case delegates to `PrimitiveValue::truncate`, whose match arm `PrimitiveValue::Empty |
PrimitiveValue::Str(_) => { /* no-op */ }` looked suspicious: `Str` (a single string value) was
grouped with `Empty` as always-no-op, regardless of `limit` - worth checking against the method's
own documented contract.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `core/src/value/primitive.rs`'s `PrimitiveValue::truncate` and `core/src/ops.rs`'s `AttributeAction::Truncate` doc comment: "Truncate a value or a sequence **to the given number of items** ... Does nothing if the attribute does not exist or **the cardinality of the element is already lower than or equal to the given size**." A `Str` value has a cardinality of exactly 1 - per this contract, `truncate(0)` should reduce it (1 > 0, so the "already <=" no-op condition doesn't hold), while `truncate(N)` for any `N >= 1` should correctly no-op. The implementation instead treated `Str` as *always* no-op regardless of `limit`, contradicting its own documented behavior specifically for the `limit == 0` case. |
| Standalone repro (release build, `dicom-core`'s public API: `PrimitiveValue::truncate`, `HasLength::is_empty`) | `___BUGHUNT___/020.../repro`. Called `.truncate(0)` on `PrimitiveValue::Str("Doe^John")`. | Reproduced deterministically: the value remained `Str("Doe^John")` (`is_empty()` returned `false`) instead of becoming empty. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic (pure in-memory transformation, no I/O or randomness). |
| Reachability check | `grep -rn "AttributeAction::Truncate\|\.truncate("` across the workspace | `AttributeAction::Truncate` is a real, public part of the attribute-editing API (`object/src/mem.rs`'s `InMemDicomObject::apply`), the kind of generic attribute-editing operation a de-identification/anonymization pipeline built on this library could plausibly use to reduce a field to nothing - if such a caller used `Truncate(0)` expecting the documented "reduce to N items" semantics to clear a single-valued PHI field (e.g. `PatientName`, itself VR=PN which is internally a `Str`), the field would silently keep its original, unredacted value. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`logic` (a documented API contract silently violated for one specific input, with a plausible
privacy-relevant consequence if used for redaction purposes - though no actual downstream
de-identification code in this repository was found using `Truncate` for that purpose; the risk is
about what a *library consumer* might reasonably expect from the documented contract, not a
concrete exploit against this codebase itself).

## Root cause

`PrimitiveValue::truncate`'s match arm merged `PrimitiveValue::Empty` (correctly always a no-op,
cardinality 0 is `<=` any `limit`) and `PrimitiveValue::Str(_)` (cardinality 1, should only be a
no-op when `limit >= 1`) into the same always-no-op branch - almost certainly because `Str` has no
`Vec`-like container to call `.truncate(limit)` on directly (unlike every other variant, which
delegates straight to the corresponding collection's own `.truncate()`), and the single-item case
was never given its own explicit `limit == 0` check to match the general cardinality-based
contract the method's own documentation states.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed the `Str` arm to check `limit == 0` explicitly and, if so, replace the value with
`PrimitiveValue::Empty` (matching what "reducing a cardinality-1 value to 0 items" means) -
otherwise still a no-op, exactly matching the documented contract for every possible `limit`
value. See `patch.diff` (`core/src/value/primitive.rs`, one function).

Verified: `repro/` re-run against the patched tree now correctly reports `Empty`/`is_empty() ==
true` for `truncate(0)`. The repro also asserts `truncate(1)` still leaves a `Str` value
completely unchanged (the correct no-op case), confirming the fix is precisely scoped to the
`limit == 0` boundary and doesn't affect any other value.

- Project test suite: `cargo test -p dicom-core --lib` - 67 passed / 0 failed, identical both
  before and after the patch. `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored,
  identical both before and after (no regressions, no newly-passing tests - no existing test
  called `truncate(0)` on a `Str`/`Empty` value).

## What's not done / open questions

- Only `PrimitiveValue::Str` was affected - every other variant already delegates to a real
  collection's own (correct) `.truncate()` method, so none of them share this bug.
- Did not search beyond this repository for real-world callers relying on the old (buggy)
  behavior - a change here is technically a behavior change for anyone calling `truncate(0)` on a
  `Str` value and expecting (relying on) the old no-op, though that would mean relying on
  behavior that directly contradicts the method's own documented contract, which is an unlikely
  and not reasonably supportable expectation to preserve.
- Not fuzzed - found by direct code reading, comparing an implementation against its own doc
  comment's stated contract (a `HEURISTICS.md`-style "state-machine invariant that isn't checked,
  only assumed" pattern, here manifesting as "the code's actual behavior for one specific input
  doesn't match what the same function's doc comment promises").
