# core-object-encoding-cast-truncation-sweep-safe

`cargo clippy --pedantic` (`clippy::cast_possible_truncation`, 86 hits across `core`, `object`,
`parser`, `encoding`) was run as a discovery technique after candidate `043` confirmed a real bug
in exactly this shape (`width as u16` silently truncating without a prior bounds check). Every
site was read to check whether the same unguarded-truncation pattern was present elsewhere in the
core parse/encode stack.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo clippy -p dicom-core -p dicom-object -p dicom-parser -p dicom-encoding --message-format=json -- -W clippy::pedantic`, filtered to `clippy::cast_possible_truncation`, then each of the 86 flagged sites read individually | Static analysis + code reading, ~10 minutes | Fell into three groups, all safe: (1) **`encoding/src/encode/{explicit_le,explicit_be}.rs`'s `length as u16`** (writing a short-form VR's 16-bit Value Length field) - guarded by an explicit `if length > u16::MAX as u32 { return WriteHeaderTooLongSnafu {..}.fail(); }` immediately before the cast, in both files; (2) **`object/src/mem.rs`'s 13 `step_index: i as u32` sites** (all in `AttributeSelector` navigation error-reporting code) - the cast only affects the *displayed* step index inside an error message, never data or control flow, and would need over 4 billion navigation steps in a single selector to even become inaccurate; (3) **`core/src/value/primitive.rs`'s 13 `c.len() as u32` sites (`calculate_byte_len` per-variant match) and `object/src/meta.rs`'s 8 similar `.len() as u32` sites** (file-meta attribute length calculation, the same function class candidate `001` from the 2026-08-24 session fixed a *different* bug in) - these would only truncate for a `PrimitiveValue`/string held in memory with more than `u32::MAX` (~4.29 billion) elements/bytes, which requires tens of gigabytes of RAM to construct in the first place; a real vector that large would already have hit practical memory limits (or `try_reserve`-style allocation failures, per the already-fixed `004` class from the prior session) well before this cast's own truncation point could matter |

## Status

**Not reproducible** - every site checked is either already guarded by an explicit runtime check
immediately before the cast, affects only cosmetic error-message content, or requires a
multi-gigabyte in-memory value to construct a triggering input at all (impractical, and arguably
dominated by unrelated allocation-size concerns rather than being its own distinct bug).

## Class

n/a

## Root cause

Not applicable (no bug) - `clippy::cast_possible_truncation` is a broad pedantic lint that flags
every narrowing numeric cast regardless of whether a guard exists nearby or whether the practical
input space can ever reach the truncating range; per HEURISTICS.md's anti-pattern guidance
("style-only clippy warnings with no correctness implication" / "no plausible way to construct the
triggering input"), most of clippy's own hits in a codebase already this thoroughly boundary-tested
turn out to be exactly this. Candidate `043`'s real bug was found by reading application code
(`fromimage`) that had *no* proportionate reason to expect its inputs (arbitrary external image
files) to already respect DICOM's own limits - the core parse/encode stack checked here, by
contrast, already derives its lengths either from validated wire data (bounded by the format's own
field widths) or from values whose realistic size is bounded by available memory long before
`u32::MAX`.

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

- This sweep covered `core`, `object`, `parser`, `encoding` only - `pixeldata`, `ul`, `json`, and
  the CLI tools were not re-swept with this same lint query (though `pixeldata`'s equivalent
  overflow-shaped bugs this session, `037`/`038`, were found by manual reasoning rather than
  clippy, and might not even be `cast_possible_truncation`-shaped in the first place - they were
  `*`/`+` overflow, not narrowing casts).
- `clippy::cast_sign_loss` (20 hits) and `clippy::cast_possible_wrap` (8 hits) - a related pair of
  lints for signed/unsigned and wrap-around-shaped casts respectively - were not swept this
  session, only `cast_possible_truncation`; a future session could reasonably apply the same
  "read every hit after a confirmed real bug in this shape" technique to those two lints as well.
