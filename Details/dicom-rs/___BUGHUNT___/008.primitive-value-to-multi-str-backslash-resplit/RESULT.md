# PrimitiveValue::to_multi_str() corrupts multi-valued strings containing a literal backslash

Found during a discovery round (no seeded lead): DICOM uses `\` (backslash) as the wire-format
value-multiplicity separator for multi-valued text elements. `core/src/value/primitive.rs`'s
`PrimitiveValue::to_multi_str()` looked suspicious for exactly this reason - a doc comment on the
method it's implemented next to (`to_str()`) already documents joining `Strs` items with `\`, which
raised the obvious question of whether anything re-splits that joined string on the same character
without accounting for a stored item that itself legitimately contains one.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `core/src/value/primitive.rs`'s `to_multi_str()` (`PrimitiveValue::Strs` arm) and `to_str()` (`PrimitiveValue::Strs` arm, used by the former). | Confirmed `to_multi_str()` calls `self.to_str()` (which joins all `Strs` items with `\`) and then **re-splits that joined string on `\`** to rebuild the "multi" vector - a reconstitute-then-resplit round trip that's lossy whenever any *individual stored item* contains a literal `\` character, since the resplit can't distinguish an item's own embedded separator-like character from the join's real separators. |
| Standalone repro (release build, `dicom-core`'s public API: `PrimitiveValue::to_multi_str()`) | `___BUGHUNT___/008.../repro`. Built `PrimitiveValue::Strs(["A\\B", "C"])` (2 items, the first containing one literal backslash) directly via the public constructor, called `.to_multi_str()`. | Reproduced deterministically: result was `["A", "B", "C"]` (3 items) instead of the correct `["A\B", "C"]` (2 items) - the stored item's own backslash was misinterpreted as a value-multiplicity separator, permanently losing the original item boundary. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic, no randomness in this code path (pure in-memory transformation, no I/O). |
| Reachability check | `grep -rn to_multi_str` across the workspace | Confirmed this is not a dead/internal-only method: `core/src/header.rs`'s `HasValue::to_multi_str` and `core/src/value/mod.rs`'s `Value::to_multi_str` both delegate straight to it, and `json/src/ser/value.rs` calls it twice (lines 32 and 187) as part of the DICOM+JSON serializer's handling of most multi-valued text VRs - a real, externally-reachable code path, not just an internal helper. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`logic` (silent data corruption: a value with a specific item count/structure is transformed into
a different item count/structure with no error raised anywhere in the call chain).

## Root cause

`to_multi_str()`'s `PrimitiveValue::Strs` arm was implemented as `self.to_str().split('\\')...`
rather than working with the already-available `values: &[String]` slice directly. `to_str()`
exists to produce a single wire-format-compatible string (items joined by `\`, matching how the
value would actually be encoded on the wire) - reusing it as an intermediate step for
`to_multi_str()` conflates two different operations: "encode to wire format" (lossy by design,
since a stored item's own `\` becomes indistinguishable from a real separator once joined) and
"list the stored items" (which needs no encoding at all, since the items are already discrete in
memory - `Strs` is `SmallVec<[String; 2]>`, never a single joined string). The join was pure
unnecessary work performed only to be undone by the very next line, and that undoing step is what
introduces the data loss.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed `to_multi_str()`'s `PrimitiveValue::Strs` arm to map directly over the existing `values`
slice, trimming each item's own trailing DICOM padding (space or NUL) independently - no join, no
resplit, so an item's internal content (including any literal `\`) is never touched:

```rust
PrimitiveValue::Strs(values) => Cow::Owned(
    values
        .iter()
        .map(|s| s.trim_end_matches([' ', '\u{0}']).to_string())
        .collect(),
),
```

See `patch.diff` (`core/src/value/primitive.rs`, one function). This mirrors the fix a prior,
now-lost fuzz-bug-repro campaign session independently arrived at for the same bug (per its own
notes, preserved only in this project's auto-memory, not in any surviving code) - that session
recorded first trying to fix only the JSON serializer's `AsStrings` wrapper, which broke an
existing test (`write_full_file_to_json`) that expected trailing DICOM padding to still be
trimmed; fixing `to_multi_str()` itself, keeping the same per-item `trim_end_matches` call, avoids
that regression while removing only the lossy resplit - independently re-verified true for this
patch too (see test suite results below, no failures).

Verified: `repro/` re-run against the patched tree now reports `["A\B", "C"]` (2 items, backslash
preserved) instead of `["A", "B", "C"]`.

- Project test suite: `cargo test -p dicom-core -p dicom-json --lib` - 67 + 12 passed / 0 failed,
  identical both before and after the patch. `cargo test --workspace --lib` - 351 passed / 0
  failed / 2 ignored, identical both before and after (no regressions, no newly-passing tests - no
  existing test constructed a multi-valued string item containing a literal backslash).

## What's not done / open questions

- Only `PrimitiveValue::Strs` was affected and fixed - the sibling `PrimitiveValue::Str` (single
  string, no multiplicity) and the other typed variants (`Date`, `Time`, numeric types, etc.)
  build their multi-string representation directly from typed values with no join/resplit step at
  all, so they don't share this bug class.
- Not checked: whether any DICOM value legitimately contains a literal, non-separator backslash in
  practice for VRs that use `to_multi_str()` (mostly text VRs like LO, SH, PN, UC, CS) - DICOM's
  own value representation rules generally discourage `\` in element content specifically because
  it's the separator character, so real-world impact may be limited to non-conformant but
  plausible inputs (which is exactly the class of input a fuzzer or malicious sender might produce
  even if compliant senders wouldn't).
- Not fuzzed - found by direct code reading (a join/resplit pattern is a `HEURISTICS.md`-worthy
  "state-machine invariant that isn't checked" red flag on its own) then confirmed by hand
  construction of a two-item value with an embedded backslash.
