# core-length-undefined-partialeq-not-reflexive

`core/src/header.rs`'s `DataElementHeader` and `SequenceItemHeader` both `#[derive(PartialEq)]`
over a field of type `Length`. `Length` has its own **hand-written** `PartialEq` (`core/src/
header.rs:1193`) that deliberately treats `Length::UNDEFINED` as never equal to *anything*,
including another `Length::UNDEFINED` - a NaN-like semantic, explicit and documented (there is
even a dedicated doc-test: `assert!(!(Length::UNDEFINED >= Length::UNDEFINED));`, and a separate
`Length::inner_eq()` method whose own doc comment says *"Unlike the implemented `PartialEq`, two
undefined lengths are considered equivalent by this method"* - i.e. the crate's own author already
anticipated that some callers need real bitwise equality and provided the tool for it). The
problem: `DataElementHeader`/`SequenceItemHeader`'s `#[derive(PartialEq)]` never uses that tool -
it inherits `Length`'s arithmetic-style non-reflexive equality directly, and that non-reflexivity
then propagates upward through every other `#[derive(PartialEq)]` built on top
(`DataElement` → `Value::Sequence` → `InMemDicomObject`), silently breaking `==`/`assert_eq!` for
**any DICOM object containing a sequence (`SQ`) or encapsulated pixel data with undefined length -
the ordinary, ubiquitous encoding for both.** Found while investigating a JSON round-trip fuzzing
crash (see "What was tried") whose `assert_eq!` failure printed byte-for-byte identical `Debug`
output on both sides - the single strongest possible signal of exactly this kind of
Debug-vs-PartialEq inconsistency.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Short one-off fuzz harness (ad-hoc, not part of this workspace's tracked fuzz infra) built against `dicom-json`'s public API, mirroring `ul/fuzz/fuzz_targets/pdu_roundtrip.rs`'s pattern for JSON: parse -> serialize -> re-parse -> compare, seeded with the leftover (untracked) `json/fuzz/corpus/json_roundtrip` corpus from a prior campaign | `ASAN_OPTIONS=symbolize=0 cargo +nightly fuzz run json_roundtrip -- -max_total_time=150` (150s, ~1400-entry seed corpus) | Crashed on the **first input tried** (`corp: 1/1b` in the log - essentially immediately) with `assertion left == right failed`. Printing both sides' `{:?}` and diffing them byte-for-byte showed **zero difference** - the two `InMemDicomObject` values were textually identical yet `!=` |
| Manual bisection: added a recursive "find the first differing element, and print whether `==` and `{:?}` agree" diagnostic to the same harness | Same harness, same crash input | Isolated the disagreement to a single `SQ` element (`(0004,1220)`) whose one sequence item, compared independently via `InMemDicomObject::eq` (`entries == entries`, `BTreeMap` comparison), *was* found equal - meaning the inequality had to be in the `DataElement`'s `header` field (derived `PartialEq` compares `header == header && value == value`), not its `value`. Traced `header.len`'s type (`Length`) to its hand-written `PartialEq`, which explained the disagreement exactly |
| Minimal, JSON-independent, `dicom-core`/`dicom-object`-only reproduction (four boundary cases: bare `Length::UNDEFINED == Length::UNDEFINED`, `DataElementHeader == itself`, `SequenceItemHeader::Item == itself`, `InMemDicomObject == its own .clone()`), debug and release build, `cargo test`/standalone binary | Constructed directly via the public API, no adversarial input needed at all - an entirely ordinary object containing one `SQ` element | All four print `false`. Reproduced deterministically: 2 in-tree `cargo test` runs (baseline check) + 2 standalone `repro/` runs against a fresh clone of pristine upstream `df2b9e42` (4/4 total, plus the original fuzz-found crash as a fifth independent confirmation) |

## Status

**Confirmed** - deterministic, pure-logic bug with no layout/seed/timing dependency whatsoever;
reproduces on *every* run, for *any* object containing an `SQ` element or encapsulated pixel data
with undefined length (not a rare edge case - this is the normal, default encoding for both). This
is arguably the highest-impact finding of this session: it doesn't require malformed/adversarial
input at all, only the most ordinary DICOM structures.

## Class

`logic` (a fundamental violation of `PartialEq`'s expected reflexivity property - `x == x` should
always hold for any legitimately-constructed value, barring an explicit, well-contained, documented
exception like IEEE-754 float `NaN`. Here the exception leaked far beyond its intended scope. No
crash, no panic, no memory-safety implication in itself - but silently breaks equality-based logic
anywhere in this codebase or downstream code that relies on `==`/`assert_eq!` for `DataElement`,
`DataElementHeader`, `SequenceItemHeader`, `Value`, or `InMemDicomObject`, for any data containing
a sequence or encapsulated pixel data - which is most real-world DICOM data).

## Root cause

`Length`'s `PartialEq` (`core/src/header.rs:1193-1199`) is intentionally not reflexive for the
`UNDEFINED` sentinel value, by design - the type's own doc-test proves this is deliberate, and
`Length::inner_eq()` exists specifically as the "I actually want bitwise equality, including for
two undefined lengths" escape hatch this exact situation calls for. `DataElementHeader` (a `tag` +
`vr` + `len` struct) and `SequenceItemHeader` (an enum with one `Item { len }` variant) both used
plain `#[derive(PartialEq)]` over that `Length` field instead of a hand-written impl using
`inner_eq()` - meaning **any two headers, however identical in every other respect, compare unequal
as soon as either one's length is undefined.** Because `DataElement`, `Value`, and
`InMemDicomObject` all build their own `PartialEq` (derived or hand-written) on top of
`DataElementHeader`/`Value::Sequence`'s comparisons, the non-reflexivity silently propagates all
the way to the top of the object model. This is architecturally the same "a type's own
carefully-reasoned equality semantics get silently inherited somewhere its author didn't intend"
shape as Rust's own famous `f32`/`f64`-in-a-derived-`PartialEq`-struct pitfall - except here the
"NaN" is a `Length` sentinel value, and the containing types are used constantly by this crate's
core object model rather than a niche numeric wrapper.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replaced `#[derive(PartialEq)]` with a hand-written `impl PartialEq` for both types, using
`Length::inner_eq()` (already provided, unused until now) instead of `==` for the length
comparison - `Length`'s own `PartialEq` is left completely untouched, since it's correct and
documented for its own, narrower, arithmetic-comparison use case:

```
 pub struct DataElementHeader {
     pub tag: Tag,
     pub vr: VR,
     pub len: Length,
 }

+impl PartialEq for DataElementHeader {
+    fn eq(&self, other: &Self) -> bool {
+        self.tag == other.tag && self.vr == other.vr && self.len.inner_eq(other.len)
+    }
+}
```

and equivalently for `SequenceItemHeader`'s three variants (`Item { len }` compared via
`inner_eq`, `ItemDelimiter`/`SequenceDelimiter` compared as unit variants). See `patch.diff` for
the exact diff.

Verified against all four boundary cases from the repro (CONFIRMATION.md section 7): `Length::
UNDEFINED == Length::UNDEFINED` remains `false` (unchanged, intentional, untouched by this fix -
this is `Length`'s own documented behavior, not part of the bug); `DataElementHeader == itself`,
`SequenceItemHeader::Item == itself`, and `InMemDicomObject (with an SQ element) == its own clone`
all flip from `false` to `true`.

- Project test suite: **`cargo test -p dicom-core --lib`: 67 passed, 1 failed (this candidate's
  own temporary repro test, reverted before writing this record) -> 68 passed, 0 failed** after the
  fix. **Full workspace (`cargo test --workspace --lib`, all 12 crates with a `#[lib]` target):
  351 passed / 0 failed / 2 ignored both before and after** (with the temporary test excluded from
  this count) - the fix causes zero regressions anywhere in the existing suite, and notably **no
  existing test anywhere in this large, mature codebase happened to already exercise this exact
  reflexivity property** for a sequence-or-encapsulated-pixel-data-bearing object, which is
  presumably why this went unnoticed until a fuzz-found crash surfaced it.

## What's not done / open questions

- `DataSetSequence`'s own `PartialEq` (`core/src/value/mod.rs:1005`) is a *different*,
  already-correct hand-written impl that explicitly disregards its own `length` field entirely
  (comparing only `items()`) - it was not touched, and does not need to be; it was checked and
  ruled safe while root-causing this candidate.
- Only `DataElementHeader` and `SequenceItemHeader` were found and fixed - a broader,
  workspace-wide grep for every other struct/enum with a `Length`-typed field under
  `#[derive(PartialEq)]` (beyond the ones already checked while root-causing this: `DataElement`,
  `PrimitiveDataElement`, `DataElementRef`, `PrimitiveDataElementRef`, all of which derive
  `PartialEq` but only ever *contain* a `DataElementHeader` rather than a raw `Length` field
  directly, so they're automatically fixed by this patch without needing their own changes) was
  not exhaustively re-run outside `core/src/header.rs` and `core/src/value/mod.rs` - a fresh
  `grep -rn "Length" --include="*.rs" | grep -i "derive.*PartialEq"`-style sweep across the rest of
  the workspace (`object`, `parser`, `encoding`, `ul`, `pixeldata`) for any other type with a raw
  `Length` field relying on derived equality would be a reasonable, cheap follow-up for a future
  session, though none were noticed in passing while investigating this candidate.
- The ad-hoc JSON round-trip fuzz harness that found this (not part of this workspace's tracked
  `json/fuzz/` infrastructure, which currently has no `fuzz_targets/*.rs` of its own - only a
  leftover corpus directory from an unrelated prior campaign) was built and run entirely outside
  the main project tree (`/tmp`) and fully discarded after use, per this skill's scope (a
  short, one-off fuzz harness for a single candidate, not a persistent campaign - see
  `fuzz-bug-repro` for that). Reconstructing a permanent `json/fuzz/fuzz_targets/json_roundtrip.rs`
  (matching the orphaned corpus's name) as a tracked addition to this workspace's own fuzzing setup
  would likely be valuable given how quickly it found a real, high-impact bug, but doing so was
  judged out of scope for a bug-hunt candidate (as opposed to a `rust-setup`/maintenance-shaped
  change) and left as a suggestion rather than made here.
- Whether any of this crate's *own* code (not just downstream consumers) silently relies on the
  broken equality somewhere - e.g., deduplication logic, caching keyed by object equality, or a
  test that passes today only because it happens to avoid `SQ`/encapsulated pixel data - was not
  audited; the fix should only ever make previously-`false` comparisons become correctly `true`,
  which cannot itself introduce new incorrect behavior, so this was judged low-risk enough not to
  block the fix, but wasn't exhaustively verified either.
