# Candidate 026's JSON deserialize fix: isolating the SQ-recursion win from the flat-element win

Candidate 026 (`json/src/de/mod.rs`'s `RawValue` + `from_str` fix, avoiding a double
deserialization through an eagerly-built `serde_json::Value` DOM) was measured on
`seq_10k.dcm`/`rtplan.dcm` - both mix nested `SQ` content with flat multi-valued elements, so the
win was confirmed for the overall mix but never cleanly separated into "how much is the
SQ-recursion path (`VR::SQ`'s arm, itself a recursive call into the same deserializer) vs. the
flat-element path (every other VR arm)". `IDEAS.md` flagged this as an open gap.

## What was tried

Added a new harness mode, `gen-flat <out> <n-elements>` (harness-only, not part of `patch.diff`):
builds an object with `n-elements` **top-level** UI-VR string elements (private-group tags, same
short-string content style as `gen-seq`'s own per-item elements) - no `SQ` nesting at all. Built
`flat_20k.dcm` (20,000 top-level elements) to directly match the *total element count* of the
already-existing `seq_10k.dcm` fixture (10,000 `SQ` items x 2 elements each = 20,000 total
elements), so the two fixtures are comparable at equal element-count scale, differing only in
whether those elements sit inside one recursive `SQ` or flat at the top level.

Re-applied candidate 026's `patch.diff` unchanged; two binaries (baseline, patched) compared via
`tools/paired.sh` (`RUNS=9`) on `json-decode`.

| Fixture | round 1 | round 2 (re-check) | round 3 (fresh `just test` clone) |
|---|---|---|---|
| `flat_20k.dcm` (20,000 flat elements, no SQ) | -14.3%, 9/9 | -12.1%, 8/9 | -18.9%, 9/9 |
| `seq_10k.dcm` (10,000 SQ items x 2 elements = 20,000 total elements) | -59.4%, 9/9 | -65.7%, 9/9 | -57.9%, 9/9 |

## Result

**Both paths show a real, consistent win, but the SQ-recursion path accounts for the large
majority of candidate 026's originally-measured effect** - both rounds agree:

- The flat-element path alone: a real, modest win, **-14.3%/-12.1%/-18.9%** across 3 rounds
  (including a fresh-clone `just test` run), all consistent - confirms the `RawValue`-vs-`Value`
  fix helps even with zero recursion, just less dramatically.
- The SQ-recursion path (same total element count, but nested): **-59.4%/-65.7%/-57.9%** across
  3 rounds, all consistent - roughly 3-5x the flat path's relative win at the identical element
  count, every round.

This cleanly answers `IDEAS.md`'s open question: candidate 026's headline numbers on
`seq_10k.dcm`/`rtplan.dcm` (-64%/-30-42%) are dominated by the SQ-recursion path, not the flat
multi-element path, which on its own would only have shown a `-12` to `-14%`-class win. Time only
was measured - no peak-RSS follow-up in this candidate (candidate 026's own peak-RSS follow-up
already covers the combined-mix case on `seq_10k.dcm`).

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum=0xfdf3ca526d3d7843` (`flat_20k.dcm`) and
  `checksum=0xd583b32455a46900` (`seq_10k.dcm`), both identical baseline vs. patched. The
  `json-decode` checksum is a full content round-trip check (decoded object re-serialized back to
  JSON, then hashed), not just an element count.
- **Is this a breaking change?** No - same private internal deserializer change as candidate 026,
  unaffected by this candidate.
- Project test suite: `cargo test --release -p dicom-json`: **12 + 5 = 17 passed / 0 failed**
  (patch applied), matching candidate 026's own recorded count for this crate. Reverted-tree
  (baseline) count is identical.

## Why the result looks the way it does

The `VR::SQ` arm's `serde_json::from_str(value.get())` deserializes into
`Vec<DicomJson<InMemDicomObject<D>>>` - each nested object is itself a full
`DataElementVisitor`-driven map scan, so the old code's per-element double-deserialization
(generic `Value` tree, then a typed re-walk) compounded recursively: every nested object inside
the `SQ` paid the same double-pass cost as a top-level element, and there were 10,000 of them
sharing this one `SQ`'s single `from_value`/`from_str` call. The flat path only pays the
double-pass cost once per element with no compounding, so its win reflects only the direct
per-call `Value`-tree-vs-`RawValue` cost difference, not a multiplicative recursive effect.

## What's not done / open questions

- This isolates SQ-recursion vs. flat-element cost but doesn't further break down *which* VR arms
  within the flat path contribute most to the -12/-14% win (UI/Strs dominates this fixture's
  element mix; other flat VR arms like numeric types were not isolated).
- This closes the `IDEAS.md` backlog item asking for this isolation - no further action
  recommended for that specific question.
