# Explicit VR Big Endian decoder reports the wrong error message on a truncated short-form value length

Found during a discovery round (no seeded lead): structurally diffed `encoding/src/decode/
explicit_le.rs` against its sibling `explicit_be.rs` (same logic, mirrored for the opposite byte
order - exactly the kind of parallel-implementation pair that already produced real, confirmed
bugs this session in `parser/src/dataset/read.rs` vs `lazy_read.rs`, candidates 006/007), looking
for any place the two files diverge beyond the expected endianness swap.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Structural diff | `diff` between `explicit_le.rs` and `explicit_be.rs` with endianness-specific identifiers normalized away (`LittleEndian`/`BigEndian` both replaced with a placeholder), so only genuine logic differences remain visible. | Found exactly one non-cosmetic difference: in the short-form-VR branch of `decode_header` (VRs like AE, CS, US, etc., which use a 2-byte value length field per PS3.5 7.1.2), `explicit_le.rs` reads the 2 length bytes with `.context(ReadLengthSnafu)` while `explicit_be.rs`'s equivalent line uses `.context(ReadItemLengthSnafu)` instead - a different, wrong Snafu error variant used only in the big-endian sibling. |
| Verification against the other decoder siblings | `grep ReadItemLengthSnafu\|ReadLengthSnafu` across `implicit_le.rs`, `adaptive_le.rs`, and `explicit_le.rs` too. | Confirmed the mismatch is isolated to `explicit_be.rs` - every other decoder correctly uses `ReadItemLengthSnafu` only at its item/sequence-delimiter-length branch and `ReadLengthSnafu` at its actual element-value-length branches, matching the pattern `explicit_be.rs` should have followed. |
| Standalone repro (release build, `dicom-encoding`'s public API: `ExplicitVRBigEndianDecoder::decode_header`) | `___BUGHUNT___/012.../repro`. Hand-built 6 bytes: a valid tag `(0010,0040)` followed by VR bytes `"CS"` (a short-form VR), then **EOF** - no length bytes follow at all. Called `decode_header` directly and inspected the resulting error's `Display` text. | Reproduced deterministically: the error message read "Failed to read the header's item length field" (`ReadItemLength`'s wording) instead of the correct "Failed to read the header's element length field" (`ReadLength`'s wording) - genuinely misleading, since nothing about this specific failure point involves a sequence item at all. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic (pure parsing logic, no I/O timing dependency). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`logic` (wrong diagnostic information: the error variant returned when parsing fails at this
specific point describes an unrelated failure mode - "item length" instead of "element/value
length" - which would mislead anyone debugging a real truncated or corrupted Explicit VR Big
Endian file at exactly this byte offset).

## Root cause

`ExplicitVRBigEndianDecoder::decode_header`'s short-form-VR branch (the 2-byte value-length read
for VRs like AE/AS/CS/US/etc.) was written with `.context(ReadItemLengthSnafu)` instead of
`.context(ReadLengthSnafu)` - almost certainly a copy-paste slip from the item-delimiter branch a
few lines above (line 35, `Tag(0xFFFE, ...)` handling, which correctly uses `ReadItemLengthSnafu`
since *that* branch genuinely does read an item/sequence delimiter's length field). The
little-endian sibling `explicit_le.rs` has the identical structure but uses the correct
`ReadLengthSnafu` at this exact position, confirming this is a divergence introduced only in the
big-endian file, not an intentional difference.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed the one `.context(ReadItemLengthSnafu)` to `.context(ReadLengthSnafu)` in
`encoding/src/decode/explicit_be.rs`'s short-form-VR value-length read, matching
`explicit_le.rs`'s already-correct equivalent. See `patch.diff` (one line changed).

Verified: `repro/` re-run against the patched tree now reports "Failed to read the header's
element length field" for the identical truncated input.

- Project test suite: `cargo test -p dicom-encoding --lib` - 40 passed / 0 failed, identical both
  before and after the patch. `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored,
  identical both before and after (no regressions, no newly-passing tests - no existing test
  asserted on this specific error variant's text for a truncated Big Endian short-form VR).

## What's not done / open questions

- This is purely a diagnostic-message defect - the actual parse outcome (an `Err`, correctly
  surfaced to the caller either way) is unaffected; only the *reason* attached to that error was
  wrong. Not itself a crash, memory-safety, or data-corruption bug, unlike most of this session's
  other findings - included because it's a real, confirmed, previously-unknown defect, per this
  skill's "no severity ranking, just facts" policy.
- Did not perform the same structural-diff comparison between `implicit_le.rs` and any big-endian
  implicit-VR counterpart, since implicit VR little endian has no big-endian sibling in this
  codebase (Implicit VR is only ever little-endian per the DICOM standard itself - there is no
  "Implicit VR Big Endian" transfer syntax to compare against). `adaptive_le.rs` also has no
  direct sibling to diff against (it's a little-endian-only adaptive/probing decoder).
