# Decode/encode charset-repertoire asymmetry for VR=UI (and other always-default VRs)

Grew out of `IDEAS.md`'s second seeded lead (a prior fuzz-bug-repro campaign's unmerged "bug-04":
"the writer doesn't switch to the data set's declared `SpecificCharacterSet` before encoding, so
any data set element with non-default-repertoire text can fail to round-trip"). The original
patch reference no longer exists on disk (only a partial leftover bundle for an unrelated bug
survived from that campaign), so this was re-derived from scratch against current upstream
`master` (`df2b9e42`) rather than reusing old, unverifiable claims. The re-derived root cause
turned out to be different from - and simpler than - the old memory's description (see "What's
not done" below for how they diverge).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading (encode side) | `parser/src/stateful/encode.rs`'s `encode_text_element`/`convert_text_untrailed` - confirmed the mid-stream charset switch on encountering tag (0008,0005) DOES work (`try_new_codec`), and that 9 VRs (AE, AS, CS, DA, DS, DT, IS, TM, UI) are unconditionally forced through `DefaultCharacterSetCodec` regardless of the currently active declared charset. | The simple "SpecificCharacterSet before PatientName" case (ascending tag order, matches decode) round-trips correctly - built and ran a repro confirming this before looking further (see `repro/src/main.rs`'s earlier revision; superseded once the real asymmetry was found, not kept as a separate negative result since it was a stepping stone within this same candidate, not a distinct suspect). |
| Code reading (decode side) | `parser/src/stateful/decode.rs`'s `read_value_strs` and the `CharacterSetOverride` enum's own doc comment (`None`/standard behavior: "Use the declared character set for LO, LT, PN, SH, ST, UC, UT. Use the default character repertoire for AE, AS, CS.") - compared this list against encode's 9-VR list and against PS3.5 6.1.2.3's actual "always default repertoire" VR set (AE, AS, CS, DA, DS, DT, IS, TM, UI, UR). | Found the real asymmetry: decode's exclusion list (AE, CS, AS, UR) is missing DA, DS, DT, IS, TM, UI - all of which encode already forces to default. Of these, DA/DS/DT/IS/TM are decoded through their own dedicated functions (`read_value_da` etc.) which already hardcode `DefaultCharacterSetCodec` and never consult the declared charset regardless of this list - only **VR::UI** actually reaches `read_value_strs` on the normal `read_value` dispatch path, so it's the one live gap. |
| Standalone repro (release build, hand-built explicit-VR-LE wire bytes + `object`'s public dataset-level API: `InMemDicomObject::read_dataset_with_ts`/`write_dataset_with_ts`) | `___BUGHUNT___/002.../repro`. Wire bytes: `(0008,0005) CS "ISO_IR 126"` followed by `(0008,0016) UI <Greek text encoded via the ISO_IR_126 codec>`. | Reproduced deterministically: parsing succeeds (`SOPClassUID` decodes to the correct Greek text, proving the declared charset - not the default repertoire - was used for a VR=UI field), but immediately calling `write_dataset_with_ts` on the very same, just-parsed object fails with `EncodeCustom { message: "unrepresentable character" }`, because encode unconditionally forces VR=UI through the Latin-1-only default codec. |
| Repeat-run check | `for i in 1..5: ./target/release/repro` (pre-fix) | 5/5 runs reproduced identically - deterministic, no randomness in this code path. |

## Status

**Confirmed** - deterministic, 5/5 runs, release build, no fixed seed needed.

## Class

`logic` (a value that a legitimate call to `read_dataset_with_ts` accepts and returns successfully
cannot then be handed straight to `write_dataset_with_ts` without an error - no crash/panic/UB,
but a self-inflicted round-trip break with no malicious input required: only a data set declaring
a non-default `SpecificCharacterSet` with any VR=UI element that legitimately contains a
declared-charset-encoded byte outside the Latin-1/default repertoire).

## Root cause

`parser/src/stateful/decode.rs`'s `read_value_strs` (used by `read_value`'s normal dispatch for
VR::UI, among others) and `parser/src/stateful/encode.rs`'s `convert_text_untrailed` (used by
every real write) each maintain their own independent list of "which VRs always use the default
character repertoire, ignoring the currently declared `SpecificCharacterSet`". These lists were
never kept in sync:

- Encode's list (`convert_text_untrailed`): AE, AS, CS, DA, DS, DT, IS, TM, **UI**.
- Decode's list (`read_value_strs`'s `use_charset_declared` exclusion, i.e. the `None`/default
  `CharacterSetOverride`): AE, CS, AS, UR - **missing UI** (DA/DS/DT/IS/TM are separately, already
  correctly hardcoded to the default codec in their own dedicated `read_value_*` functions, so
  they were never actually at risk via this list on the normal decode path - only via the
  alternate `read_value_preserved` path, not exercised by this candidate's repro).

Because decode's list omits UI, a VR=UI element's value is decoded using whatever
`SpecificCharacterSet` the data set has declared (via the `(0008,0005)` mid-stream switch), which
can legitimately produce a Rust `String` containing characters outside Latin-1's repertoire (e.g.
Greek, under `ISO_IR 126`). But encode's list still forces every VR=UI value through
`DefaultCharacterSetCodec::encode`, which uses `EncoderTrap::Strict` and hard-fails on any
character outside U+0000-U+00FF. The object is fully valid and readable the moment it's parsed,
and only breaks the instant something tries to write it back out - `open_file` -> `write_all` (or
any equivalent read/write round trip) on a real-world file matching this shape would fail with no
other symptom.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Extended decode's exclusion list in `read_value_strs` (and the matching list used by
`read_value_preserved`, which shares the same match expression) to include DA, DS, DT, IS, TM, UI
- exactly encode's list - so a value that decode forces to non-default is never a value encode
would also force to default, and vice versa. Updated `CharacterSetOverride::None`'s doc comment to
state the corrected, complete list and reference PS3.5 6.1.2.3. Left `CharacterSetOverride::AnyVr`
(the explicit opt-in to use the declared charset for genuinely everything) and `UR`'s existing
decode-side-only forcing untouched - `UR`'s encode side already doesn't special-case it, but that
asymmetry (encode using the declared charset for VR=UR while decode forces default) was not part
of the confirmed bug (URIs are ASCII-only by definition in practice) and touching it risked
changing already-tested `UR` behavior without a concrete failing case driving it - noted as a
follow-up below rather than folded into this fix.

See `patch.diff` (`parser/src/stateful/decode.rs`, two hunks: the doc comment and the match arm).
Verified: `repro/` re-run against the patched tree no longer fails on `write_dataset_with_ts` (the
Greek-encoded UI value now decodes as `DefaultCharacterSetCodec`-would per the corrected,
spec-aligned behavior - self-consistent round trip, no crash).

- Project test suite: `cargo test -p dicom-parser -p dicom-object --lib` - 49 + 75 passed / 0
  failed, identical both before and after the patch. `cargo test --workspace --lib` - 351 passed /
  0 failed / 2 ignored, identical both before and after (no regressions, no newly-passing tests -
  no existing test exercised this VR::UI/non-default-charset combination).

## What's not done / open questions

- This candidate's actual finding diverges from the old campaign memory's description of "bug-04"
  (which described a 4-part fix: "encoder charset-switch VR guard, writer pre-priming via
  token-stream scan, tag-order-safe `IntoTokens`, decoder VR=SH charset-switch"). That patch no
  longer exists to compare against, and re-deriving the problem from the current code found a
  narrower, more precisely-targeted root cause (a two-list synchronization gap, not a
  pre-priming/architecture gap) that fully explains and fixes the confirmed repro. It's possible
  the old campaign's fix targeted a related-but-different manifestation (e.g. something involving
  `SQ` nesting or `IntoTokens` ordering) that this candidate did not attempt to reconstruct or
  rule out - if a future session has reason to suspect a *sequence*-nesting-specific charset bug
  (as opposed to the flat-dataset case confirmed here), that's still open.
- `VR::UR`'s asymmetry in the OTHER direction (decode forces default, encode does not) was noticed
  but not fixed - no concrete failing case was constructed for it (URI content is ASCII by
  definition per RFC 3986, so `self.text`-vs-default divergence should never actually produce
  different bytes in practice), so it wasn't folded into this bounded fix. Worth a look if anyone
  wants full symmetry rather than just eliminating the confirmed crash class.
- Only `InMemDicomObject`'s dataset-level `read_dataset_with_ts`/`write_dataset_with_ts` API was
  exercised. The full-file path (`open_file`/`write_all`, which also involves the file meta group)
  was not separately re-tested for this exact scenario - the file meta group's own strings are
  unaffected (they're always the fixed `DefaultCharacterSetCodec`, per candidate 001's
  investigation of `object/src/meta.rs`), so no interaction is expected, but not explicitly
  verified end-to-end through `open_file`.
- Not fuzzed - found by direct reasoning from a previously-seeded lead plus manual construction of
  hand-built wire bytes, per `HEURISTICS.md`'s guidance for "state-machine invariant that isn't
  checked, only assumed" (the invariant here being "decode and encode agree on which VRs are
  charset-affected"). A short fuzz campaign against `parser/fuzz` (if a suitable target existed)
  seeded with data sets declaring various non-default `SpecificCharacterSet` values might find
  further VR-specific instances of the same class.
