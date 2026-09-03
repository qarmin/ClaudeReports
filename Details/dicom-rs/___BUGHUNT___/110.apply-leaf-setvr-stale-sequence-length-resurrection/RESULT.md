# apply-leaf-setvr-stale-sequence-length-resurrection

Follow-up to candidate 109, which confirmed `items_mut()`/`update_value` correctly reset a
sequence's owning `DataElement.header.len` to `Length::UNDEFINED` after an item insert/remove,
but flagged - without verifying end-to-end - that `apply_leaf`'s `AttributeAction::SetVr` arm
(`object/src/mem.rs`) reconstructs the element via `DataElement::new(header.tag, new_vr, value)`,
which re-derives `header.len` from `value.length()`, and for a `Value::Sequence` that reads
`DataSetSequence`'s own private `length` field - a field 109 showed is never kept in sync with
`header.len` by item mutations. This candidate built the missing end-to-end repro.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `apply_leaf`'s `SetVr` arm, `DataElement::new`/`new_with_len`, `Value::length()`/`DataSetSequence::length()`, `DataElementTokens::Start` (writer token construction), `ExplicitLengthSqItemStrategy::NoChange` handling in `parser/src/dataset/write.rs` | n/a | Confirmed the full mechanism by reading alone: `DataElement::new` sets `header.len = value.length()`; for `Value::Sequence` that reads the private `length` field set once at parse time and never updated by `items_mut()`; the writer's `DataToken::from(header)` forwards `header.len` verbatim into `SequenceStart{ len }`; `NoChange` strategy writes that `len` as-is with no delimiter |
| Direct reproduction, debug build (`object/examples/scratch_110_setvr_seq_len.rs`, scratch file removed before finishing this candidate) - real end-to-end round trip: hand-built well-formed wire bytes for a top-level SQ (0008,1140) with an EXPLICIT length and 1 item, parsed via the crate's real `InMemDicomObject::read_dataset_with_ts`, mutated via the public `obj.update_value_at(tag, \|v\| { seq.items_mut().push(new_item) })` (grows the sequence), then `obj.apply(AttributeOp::new(tag, AttributeAction::SetVr(VR::SQ)))` on the same tag, then re-serialized via the real `write_dataset_with_ts_options` with `ExplicitLengthSqItemStrategy::NoChange`, then re-parsed via the same real `read_dataset_with_ts` | 3/3 repeat runs, debug build | Reproduced every run: `header.len` was `Length(Undefined)` right after `items_mut()`, then flipped back to `Length(20)` (the stale pre-mutation explicit length) right after `SetVr`; the round-tripped output failed to re-parse with `Could not read data set token` |
| Same technique, opposite boundary (`items_mut().remove(1)` shrinking a 2-item sequence to 1 item instead of growing it) | 3/3 repeat runs, debug build | Reproduced identically: `header.len` resurrected to the stale `Length(40)`, round-trip failed the same way - confirms the bug is the general `SetVr`-after-item-mutation mechanism, not specific to growing the sequence |

## Status

**Confirmed.** Reproduces 3/3 runs for both the growth (push) and shrink (remove) boundary
variants, debug build, deterministic (fixed input, no randomness in this code path).

**Fully reproducible: Yes.** The repro parses a well-formed wire-format input through the crate's
real `InMemDicomObject::read_dataset_with_ts`, mutates it through the crate's real public
`update_value_at`/`items_mut()` and `apply`/`AttributeOp` API (the same entry points a real
caller would use), serializes through the crate's real `write_dataset_with_ts_options`, and
detects the corruption by re-parsing the writer's own output through the same real reader. This
is a single-participant round-trip through the library's own real serialize/decode pair (no
peer/caller is being stood in for by hand-authored bytes) - the same shape CONFIRMATION.md
section 11 treats as needing no second participant, and the same technique candidate 109's own
accepted repro used.

## Class

logic

## Root cause

`InMemDicomObject::apply_leaf`'s `AttributeAction::SetVr` arm removes the element, splits it into
`(header, value)`, and reconstructs it with `DataElement::new(header.tag, new_vr, value)`.
`DataElement::new` unconditionally sets `header.len = value.length()`. For `Value::Sequence(seq)`,
`Value::length()` delegates to `DataSetSequence::length()`, which just returns `self.length` - a
private field populated once, at parse time (`InMemDicomObject::build_object`), from the wire's
declared length, and never touched again by any of the sequence's own mutating methods
(`items_mut()`, `truncate()`). Candidate 109 already established that `header.len` itself (the
field the writer actually serializes, via `DataToken::from(header)` in
`parser/src/dataset/mod.rs`) *is* correctly reset to `Length::UNDEFINED` by
`DataElement::update_value`'s post-mutation hook whenever the value becomes a sequence - but that
correct state lives only in `header.len`, not in `DataSetSequence.length`. `SetVr`'s
reconstruction bypasses `header.len` entirely and rebuilds a fresh `header.len` from the stale
`DataSetSequence.length` instead, silently undoing whatever correct invalidation a prior
`items_mut()`-based mutation had already performed on the same tag. When the object is then
written with `ExplicitLengthSqItemStrategy::NoChange` (the strategy that writes cached explicit
lengths verbatim instead of always forcing them undefined), the stale length is serialized as-is,
producing a sequence whose declared byte length no longer matches its actual encoded item content
- unparseable by the crate's own reader.

## The fix

In `object/src/mem.rs`'s `AttributeAction::SetVr` arm, stop recomputing the length from
`value.length()` unconditionally. Instead: for `Value::Sequence`, always use `Length::UNDEFINED`
(always a valid encoding for a sequence, and consistent with the invariant `update_value` already
enforces elsewhere) rather than trusting the vestigial `DataSetSequence.length` field; for
`Value::Primitive`/`Value::PixelSequence`, keep using `value.length()` as before (correct in both
cases - primitive length is always freshly derived from the actual bytes, and
`PixelFragmentSequence::length()` is hardcoded to always return `Length::UNDEFINED` already, so it
was never affected by this bug). Verified against both boundary variants (push-growth and
remove-shrink) per CONFIRMATION.md section 7, not just the one that was found first - both are
fixed by the same change, since both hit the identical code path.

```diff
-                    let e = DataElement::new(header.tag, new_vr, value);
+                    let len = match &value {
+                        Value::Sequence(_) => Length::UNDEFINED,
+                        Value::Primitive(_) | Value::PixelSequence(_) => value.length(),
+                    };
+                    let e = DataElement::new_with_len(header.tag, new_vr, len, value);
```

Before/after re-running `repro/` against the patched tree: both variants go from
`BUG REPRODUCED: re-parsing the writer's own output failed: Could not read data set token` (exit
code 1) to `OK: round-trip preserved the sequence correctly` (exit code 0). See `output.txt` for
full captured output of both runs.

- Project test suite: `cargo test -p dicom-object --lib` - 75 passed, 0 failed before; 75 passed,
  0 failed after (identical pass count, no new failures, no regressions).

## What's not done / open questions

- Only `SetVr` was checked as a second-order path that reads `DataSetSequence.length` after a
  prior `items_mut()`-based mutation. Candidate 109's own grep for `.length()`/`HasLength::length`
  callers on `Value::Sequence`/`DataSetSequence` values found `DataElement::new()` (called at
  parse time, and here in `SetVr`) as the only caller reachable from `dicom-object`'s public
  surface - no other public API was found to have the same exposure, but a future session
  re-grepping after any new code lands near `apply_leaf`/`DataElement::new` call sites would be
  worth doing.
- `PixelFragmentSequence::length()` is hardcoded to always return `Length::UNDEFINED`
  (`core/src/value/mod.rs`), so `Value::PixelSequence` was never susceptible to this specific bug
  - confirmed by reading, not separately fuzzed.
- The underlying design smell - `DataSetSequence` carrying its own private `length` field that
  can silently diverge from the owning `DataElement.header.len` the moment any item mutation
  happens, with no invariant enforcing they stay equal - is not fixed at the source; this patch
  only removes the one reachable path (`SetVr`) that reads the stale field back out. A more
  thorough fix (e.g. removing `DataSetSequence.length` entirely, since 109 found nothing on the
  read path other than parse-time construction and this now-patched `SetVr` call ever consults
  it) would close the whole class permanently but is a larger, more invasive change out of scope
  for a bounded fix this session.
