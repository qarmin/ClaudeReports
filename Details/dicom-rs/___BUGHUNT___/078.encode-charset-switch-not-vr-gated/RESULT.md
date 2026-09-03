# Encoder's SpecificCharacterSet codec-switch trigger is not VR-gated, unlike the decoder's

From `IDEAS.md`'s backlog entry "object: charset inference for (0008,0005) doesn't check VR is
text-compatible", seeded from an unmerged branch `origin/fix/charset-non-text-vr` (`6242b75f`).
That branch's own diff turned out to modify `charset_from_tokens`/`InMemDicomObject::infer_charset`
functions that do not exist anywhere in current `master` (`df2b9e42`) - they were only ever
introduced by a sibling, also-unmerged branch (`origin/fix/charset-roundtrip`, `43b795c8`, itself
based on the same parent commit `401be47d`), and `6242b75f` both introduces and immediately
"fixes" its own from-scratch copy of those functions in one commit. Neither branch's premise
describes a real defect in current `master`: `write_dataset_with_ts`/`write_dataset_with_ts_cs`
on `master` never scans the token stream or infers a charset from (0008,0005) at all - they just
take an explicit `SpecificCharacterSet` argument (defaulting to `SpecificCharacterSet::default()`),
so there is nothing in `object/src/lib.rs`/`object/src/mem.rs` to check the VR of in the first
place.

Continuing the check into the real, existing charset-inference mechanism (as instructed by the
backlog entry's own "verify this is not already covered ... before writing it up" caveat) found a
genuinely distinct, real defect in `parser/src/stateful/encode.rs`: unlike the decoder
(`parser/src/stateful/decode.rs`'s `read_value_cs`, only reachable for `VR::CS` via both of
`read_value`/`read_value_preserved`'s dispatch tables - already independently confirmed VR-gated
while investigating candidate 002), the encoder's own (0008,0005)-triggered codec switch
(`encode_text_element`/`encode_texts_element` in `parser/src/stateful/encode.rs`) checks only the
element's *tag*, never its VR, before calling `try_new_codec` and mutating the encoder's active
character set for everything written afterward.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading (verify the branch's premise against current master) | `grep -rn "infer_charset\|charset_from_tokens"` across the whole repo; read `object/src/mem.rs`'s actual `write_dataset_with_ts*` functions (lines ~2056-2075) and `object/src/lib.rs`'s current charset-related code. | Confirmed `charset_from_tokens`/`infer_charset` do not exist anywhere in current `master` - the write path always uses an explicitly-passed `SpecificCharacterSet` (default `SpecificCharacterSet::default()`), never inferred from a scan of (0008,0005). The literal defect described in `IDEAS.md`/the seed branch does not apply to current `master`; this half of the check is `Not reproducible` (see below). |
| Code reading (find the *real* charset-inference mechanism instead) | `parser/src/stateful/decode.rs`'s `read_value_cs` (decode-side switch) and its two dispatch tables (`read_value`, `read_value_preserved`) - both route `VR::CS` and only `VR::CS` through `read_value_cs`, so decode already only trusts (0008,0005) when explicitly tagged `VR::CS`, confirming (independently of candidate 002, which established this for a different reason) that decode is safe. Then read `parser/src/stateful/encode.rs`'s `encode_text_element`/`encode_texts_element`, the encode-side mirror: both check only `de.tag == Tag(0x0008, 0x0005)`, with no `de.vr` check at all, before switching `self.text`. Since `encode_primitive_element` dispatches to these two functions based on the *value's Rust variant* (`PrimitiveValue::Str`/`Strs`), not on VR, any text-shaped value at tag (0008,0005) - decoded via `read_value_strs` for VR `AE`/`AS`/`PN`/`SH`/`LO`/`UC`/`UI`, none of which switch the *decoder's* charset - triggers the *encoder's* charset switch on write regardless. |
| Standalone repro (release build, `object`'s public API only: `InMemDicomObject::read_dataset_with_ts`/`write_dataset_with_ts`) | `___BUGHUNT___/078.../repro`. Hand-built explicit-VR-LE wire bytes: `(0008,0005) LO "ISO_IR 126"` (a text VR, but not `CS` - the decoder's actual gate) followed by `(0010,0010) PN` with raw byte `0xE9` (Latin-1 `é` under the default repertoire, which is what decode legitimately uses here since VR=LO never routes through `read_value_cs`). | Reproduced deterministically: parsing succeeds and decodes `PatientName` as `"é"` (proving decode correctly never switched charset for the VR=LO-tagged element), but calling `write_dataset_with_ts` on the very same, just-parsed object fails with `EncodeCustom { message: "unrepresentable character" }` - the encoder switched to the Greek (`ISO_IR 126`) codec on encountering the VR=LO (0008,0005) element (its check has no VR gate at all), then failed to re-encode `"é"` under that codec. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` (pre-fix, release build, main working tree) | 3/3 runs reproduced identically - deterministic, no randomness in this code path. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

**Fully reproducible**: **Yes** - the repro drives the real, unmodified public API
(`InMemDicomObject::read_dataset_with_ts` / `write_dataset_with_ts`) through the real parser/encoder
code paths end-to-end; the "second participant" here is simply well-formed-but-non-conformant wire
bytes handed to the library's own decoder (there is no separate peer process/binary in this
finding - it is a self-round-trip bug: an object the library itself just parsed cannot be written
back out by the same library), consistent with how candidates 001/002/006/007 in this same charset
cluster were already accepted as fully reproducible on the same basis.

## Class

`logic` (round-trip content/failure mismatch: a `SpecificCharacterSet` declaration with a
non-`CS` but still text-shaped VR is correctly ignored by the decoder, per the decoder's own
VR-gated dispatch, but is blindly trusted by the encoder regardless of VR - so decode and encode
disagree about whether the element ever changed the active character set, corrupting or failing
the encoding of every subsequent text element until the file's true end or another (0008,0005)
element under a VR the encoder also fires on).

## Root cause

`parser/src/stateful/encode.rs`'s `encode_text_element` (used for `PrimitiveValue::Str`) and
`encode_texts_element` (used for `PrimitiveValue::Strs`) each independently check only
`de.tag == Tag(0x0008, 0x0005)` before calling `self.try_new_codec(...)`, which unconditionally
overwrites the single shared `self.text` field used for all subsequent charset-affected text
encoding. `encode_primitive_element` dispatches to these two functions based on the *value's*
Rust enum variant, not on the element's declared VR - so any tag-(0008,0005) element whose value
happens to be `Str`/`Strs` (which includes every VR that `parser/src/stateful/decode.rs`'s
`read_value_strs`/`read_value_str` route to, i.e. `AE`/`AS`/`PN`/`SH`/`LO`/`UC`/`UI`/`UT`/`ST`/`UR`/
`LT`, not just `CS`) reaches the unconditional codec switch on encode. The decoder's equivalent
switch (`read_value_cs`, called from `read_value_cs` only) is reachable *exclusively* for
`VR::CS` via both of decode's dispatch tables - so decode and encode disagree on which VRs are
allowed to declare a character set change, and only encode's list is unbounded (every VR, via any
text-shaped Rust value).

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added `&& de.vr == VR::CS` to both tag checks in `encode_text_element` and `encode_texts_element`
(`parser/src/stateful/encode.rs`), matching the decoder's real gate (only `VR::CS` elements can
switch the active character set). See `patch.diff`.

Verified: `repro/` re-run against the patched tree no longer fails (`write_dataset_with_ts`
succeeds, encoder never switched to the Greek codec since VR=LO no longer qualifies) - 3/3 runs.

- Project test suite:
  - Before fix: `cargo test -p dicom-parser -p dicom-object --lib` -> 49 passed + 75 passed, 0
    failed. `cargo test --workspace --lib` -> 351 passed, 0 failed, 2 ignored.
  - After fix: `cargo test -p dicom-parser -p dicom-object --lib` -> 49 passed + 75 passed, 0
    failed (identical). `cargo test --workspace --lib` -> 351 passed, 0 failed, 2 ignored
    (identical - no regressions, no newly-passing tests; no existing test exercised a
    non-`CS`-VR (0008,0005) element).

## What's not done / open questions

- The originally-seeded `IDEAS.md` lead (`infer_charset`/`charset_from_tokens` in
  `object/src/lib.rs`/`object/src/mem.rs`) is confirmed **not applicable** to current `master` -
  those functions do not exist there. If either unmerged branch (`fix/charset-non-text-vr`,
  `fix/charset-roundtrip`) is ever revived and actually merged, whatever VR-gating it adds should
  be re-checked against this candidate's finding, since introducing that pre-priming mechanism
  would add a second, independent place needing the same `VR::CS` gate this patch adds to the
  mid-stream encode-side switch.
- Only `VR::CS` was used as the "wrong" VR in the repro (the actual PS3.5-conformant VR). Other
  VRs that reach `encode_text_element`/`encode_texts_element` (`AE`, `AS`, `PN`, `SH`, `LO`, `UC`,
  `UI`, and, via `encode_element_as_text`'s different call path, `UT`/`ST`/`UR`/`LT` through
  `read_value_str`) were reasoned about but not each individually re-verified with their own
  repro - the fix is VR-general (checks `VR::CS` specifically, not a denylist of "wrong" VRs), so
  this is expected to close all of them, but wasn't exhaustively re-tested one-by-one.
- Not fuzzed - found by direct reasoning from a seeded lead (once the lead's literal premise was
  shown not to apply) plus manual construction of hand-built wire bytes, following the same
  pattern as candidate 002. A short fuzz campaign against `parser/fuzz`, seeded with data sets
  declaring (0008,0005) under various non-`CS` VRs, might be worth pointing at this code via a
  separate `fuzz-bug-repro` session.
