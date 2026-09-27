# jpegxl-bits-allocated-unreachable-safe

`transfer-syntax-registry/src/adapters/jpegxl.rs` has two `unreachable!()` sites (lines 153 and
214, in `JpegXlAdapter::decode_frame` and `JpegXlLosslessEncoder::encode_frame` respectively),
each the fallback arm of a `match bits_allocated { 8 => ..., 16 => ..., ..., _ => unreachable!() }`
(the decode side also has arms for `1` and `24`). `bits_allocated` is an attacker-controlled
DICOM attribute (`BitsAllocated`), which made this look like the same "untrusted value drives a
match with an unreachable fallback" shape as several confirmed bugs from the prior session (009,
023, 024, 026). Flagged as an unchecked `unreachable!()` site left over from the prior session's
sweep (`projects/dicom-rs.md`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: read both functions from their start, not just around the `unreachable!()` itself | Static code reading, no build needed | Both functions fetch `bits_allocated` from the DICOM object and, a few lines later but still strictly before the `match bits_allocated { ... }` block, call `ensure_whatever!(bits_allocated == 8 \|\| bits_allocated == 16, "BitsAllocated other than 8 or 16 is not supported")` (lines 52-55 for decode, 188-189 for encode) - any value other than exactly 8 or 16 returns an `Err` immediately, before `bits_allocated` is used again. By the time the later `match` runs, the value is already constrained to `{8, 16}`; the `1` and `24` arms on the decode side are dead code (unreachable but not marked as such - not flagged by this sweep since they're not literal `unreachable!()` calls) and the trailing `_ => unreachable!()` on both sides is genuinely unreachable |

## Status

**Not reproducible** - ruled out by reading; both sites are preceded by an explicit
`ensure_whatever!` guard in the same function that constrains the matched value to exactly the
set of arms the match actually handles.

## Class

n/a

## Root cause

Not applicable (no bug) - unlike candidate 009's `bits_stored()` (which had *no* upstream
validation before reaching an `assert!` deep in `pixeldata/src/lut.rs`), this file validates
`bits_allocated` locally, in the same function, only a few lines before the value is used in the
match that the `unreachable!()` sites guard. Worth noting for future sessions: **the presence of
an untrusted DICOM attribute feeding a match with `unreachable!()` is not automatically a bug -
check for a local `ensure!`/`ensure_whatever!`/early-return guard between the fetch and the match
before assuming it's reachable**, same lesson as `projects/dicom-rs.md` already records for the
`unreachable!()`-sweep technique in general (most sites turn out to be genuinely guarded).

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

None - both call sites were read in full, including their respective guards.
