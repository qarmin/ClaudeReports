# VR::UR decode/encode charset handling - re-checked, not actually asymmetric

From `IDEAS.md`'s backlog, itself noticed while working candidate 002
(`002.writer-charset-not-switched`): candidate 002's own investigation notes recorded that decode
forces `VR::UR` through `DefaultCharacterSetCodec` (in the `read_value_strs` exclusion list) while
encode's `convert_text_untrailed` does not special-case `UR` at all - the mirror image, in the
opposite direction, of the actual confirmed VR::UI bug. Re-checked properly this time by tracing
which function VR::UR is *actually* dispatched to (candidate 002 only inspected the exclusion list
inside `read_value_strs`, not which VRs really reach that function).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `parser/src/stateful/decode.rs`'s `read_value`/`read_value_preserved` dispatch tables (both), and `convert_text_untrailed` in `parser/src/stateful/encode.rs`. | `VR::UR` is dispatched to `read_value_str` (**singular** - shared with `UT`/`ST`/`LT`), not `read_value_strs` (**plural**, where the exclusion-list arm `(_, VR::UR) => false` that candidate 002 flagged actually lives). `read_value_str` always decodes with `self.text` (the currently declared, possibly non-default character set) for every VR routed to it, with no exception for `UR` - so the `read_value_strs` exclusion list's `VR::UR` arm is dead code for both of `read_value`/`read_value_preserved`'s real dispatch paths (no code path found that calls `read_value_strs` with a UR header). On the encode side, `convert_text_untrailed`'s explicit "always default repertoire" list (AE, AS, CS, DA, DS, DT, IS, TM, UI) also does not include `UR`, so encode falls through to its own `_ => self.text.encode(text)` branch - the SAME "use the declared charset" behavior as decode. Decode and encode are therefore **symmetric** for `VR::UR`: both consistently use the currently active declared character set, neither forces default. |

## Status

**Not reproducible** - traced the full path for both decode and encode without constructing a
test input, since the code reading was conclusive: there is no decode/encode disagreement to
exploit. (Per PS3.5 6.1.2.3, `UR` should technically always use the default repertoire regardless
of what's declared - this library doesn't do that - but that's a spec-conformance gap, not a
round-trip-breaking bug, since both sides agree with each other consistently.)

## Class

N/A (not confirmed).

## Root cause

N/A - the suspect turned out to be safe. See "What was tried" for the mechanism: `VR::UR` never
reaches the function (`read_value_strs`) where the flagged exclusion-list entry lives; the function
it actually reaches (`read_value_str`) and its encode-side counterpart
(`convert_text_untrailed`'s `_` fallback) already agree with each other.

## The fix

N/A - no fix needed, nothing to fix.

## What's not done / open questions

- This confirms `VR::UR` is spec-non-conformant (uses declared charset instead of always-default)
  but NOT a round-trip-breaking bug in this library specifically, because decode and encode agree.
  If DICOM interoperability with a strictly spec-conformant peer (which would only ever encode
  `UR` in the default repertoire) is a concern, that's a separate, much lower-value correctness
  gap - not pursued further here, no concrete failure constructed.
- Confirms (again) that the original seed for this idea - reading only the `read_value_strs`
  exclusion list without checking whether the VR actually reaches that function - was an
  incomplete analysis. Worth remembering for future sessions: always trace the *actual* dispatch
  path (`read_value`'s `match header.vr()`), not just a VR's presence/absence in one helper
  function's internal list, before concluding an asymmetry exists.
