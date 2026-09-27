# decode-read-value-cs-unreachable-safe

`parser/src/stateful/decode.rs:868`, inside `StatefulDecoder::read_value_cs` (used for VR::CS,
e.g. _SpecificCharacterSet_ decoding), has `let parts = match &out { PrimitiveValue::Strs(parts)
=> parts, _ => unreachable!() };` right after calling `self.read_value_strs(header)?`. Flagged as
an unchecked `unreachable!()` site left over from the prior session's sweep
(`projects/dicom-rs.md`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: read `read_value_strs`'s full body (`parser/src/stateful/decode.rs:468-508`) to check every return path | Static code reading, no build needed | `read_value_strs` has exactly one `return`/tail expression: `Ok(PrimitiveValue::Strs(parts?))`, unconditionally, for every `len` including 0 - `self.buffer.split(|v| *v == b'\\')` on an empty buffer still yields one element (an empty slice), so even a zero-length value decodes to `Strs(c![""])`, never `PrimitiveValue::Empty` or any other variant. There is no code path in `read_value_strs` that returns anything other than `Ok(PrimitiveValue::Strs(_))` or an `Err` (which `?` propagates past the `match` in `read_value_cs` entirely, never reaching it) |

## Status

**Not reproducible** - ruled out by reading; no code path can reach the `unreachable!()` arm.

## Class

n/a

## Root cause

Not applicable (no bug) - `read_value_strs`'s only success return is `PrimitiveValue::Strs`,
unconditionally, so `read_value_cs`'s match on its result can never see any other variant. The
`unreachable!()` here correctly documents a real invariant, unlike the four other sites this
session's discovery round grouped it with, of which one (`json/src/de/mod.rs:365`, candidate 027)
turned out to be reachable.

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

None - this exhausts the suspect (single, small function with no loops/branches affecting the
return type).
