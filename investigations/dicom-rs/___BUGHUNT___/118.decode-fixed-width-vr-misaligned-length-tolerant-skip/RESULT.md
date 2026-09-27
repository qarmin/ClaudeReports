# decode-fixed-width-vr-misaligned-length-tolerant-skip

Found via this session's GitHub issue raw-sync: upstream issue `Enet4/dicom-rs#698` ("Potential
mismatch between self.position and reader position in StatefulDecoder", open, no linked PR as of
`df2b9e42`) reports a real, non-malicious user file with a multi-valued `UL` tag whose declared
value length (10 bytes) is 2 bytes larger than its element count needs (2 x UL = 8 bytes) - dcmtk
and dcmjs both parse this file fine (skipping the 2 trailing bytes), but dicom-rs fails right after
this tag. This is a live, real-world confirmation of already-Confirmed candidate 108's exact root
cause (`parser/src/stateful/decode.rs`'s fixed-width VR readers silently under-consuming a
misaligned declared length, desynchronizing the decoder's position bookkeeping from the actual
stream position) - candidate 108's own fix chose to *reject* a misaligned length outright with a
clear error, which stops the silent corruption but still leaves this exact real-world file
unparseable, unlike dcmtk/dcmjs. This candidate implements and verifies an alternative,
interoperability-preserving fix: tolerate the misalignment by skipping the slack bytes (with a
warning), matching what the other toolkits already do.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading of the 9 fixed-width VR readers in `parser/src/stateful/decode.rs` (`read_value_tag`/`_ss`/`_fl`/`_ul`/`_us`/`_uv`/`_sl`/`_sv`/`_od`) | n/a | Confirmed the exact mechanism candidate 108 already documented: `n = len >> k` computes the element count correctly, but the decoder's own `self.position += len as u64` bookkeeping assumes `len` bytes were consumed from the actual stream, when only `n * elem_size` bytes actually were - the difference (`len % elem_size`) is left unconsumed in the real reader, corrupting the parse of whatever comes next |
| Standalone repro reusing candidate 108's own two-element wire-bytes technique (`InMemDicomObject::read_dataset_with_ts` on hand-encoded Explicit VR Little Endian bytes), adapted to the exact shape reported in #698 (a `UL` tag, declared length 10, 2 real `UL` values + 2 trailing garbage bytes, followed by a normal `PN` element) | debug build, 3 repeat runs, against unpatched `df2b9e42` | Reproduced 3/3: `parse error: Could not read data set token` - the file fails to parse at all, exactly matching the issue's own complaint ("parsing fails right after this tag") |
| Same repro, 3 repeat runs, against a tree with this candidate's own fix applied | debug build | The file parses successfully every time: `elem1` correctly holds `[1, 2]` (its 2 real `UL` values, garbage skipped) and `elem2` (`PN`, "AB") parses correctly too - matching the lenient behavior the issue reporter observed from dcmtk/dcmjs |

No further tools were needed - the mechanism was already fully diagnosed by candidate 108; this
session's work was independently re-deriving that the still-live (unfixed) defect explains a real
user's file, and designing/verifying an alternative fix.

## Status

**Confirmed** - deterministic, pure parsing-logic bug with no layout/seed/timing dependency:
reproduces 3/3 runs both for the original (still-live, unpatched) desync bug and for this
candidate's own fix (3/3 successful parses).

**Fully reproducible**: Yes. The repro drives the real, public `InMemDicomObject::read_dataset_with_ts`
entry point on genuinely valid-shape Explicit VR Little Endian wire bytes - a single external
artifact (the file itself) with no second participant to fake; per `CONFIRMATION.md` section 11
this single-process shape is the correct one here (same reasoning as candidate 108's own repro).

## Class

logic

## Root cause

See candidate 108's `RESULT.md` for the original, already-Confirmed root cause: `parser/src/
stateful/decode.rs`'s 9 general fixed-width-VR value readers compute the number of whole elements
via `len >> k` but then advance `self.position` by the *full* declared `len`, without ever
consuming the `len % elem_size` remainder bytes from the actual stream - leaving the real reader
`len % elem_size` bytes behind where the decoder's own bookkeeping believes it to be, so the next
element's header is misread starting from garbage. Candidate 108 fixed this by rejecting a
misaligned length outright (a hard, immediate `InvalidValueLength` error) - which correctly
eliminates the silent corruption, but does not let a real file like the one in #698 parse
successfully, unlike dcmtk/dcmjs, which both tolerate this exact kind of slack.

## Introduced by

Same as candidate 108: `origin/fix/charset-roundtrip`'s (43b795c8) decode.rs hunk first flagged
the general shape; the underlying `len >> k` pattern itself predates that branch and was not
independently re-bisected this session (see candidate 108's own "Introduced by" for the fuller
context) - this candidate is a design refinement of 108's fix, not a newly-discovered defect
location.

## The fix

Added a `skip_misaligned_remainder(header, len, elem_size)` helper to the same `impl` block as
`require_known_length` (used by all 9 affected functions): after decoding the `n = len / elem_size`
whole elements, if `len % elem_size != 0`, it logs a `tracing::warn!` (naming the tag, declared
length, element size, and remainder) and discards exactly the remainder bytes from the real stream
(`std::io::copy` into `std::io::sink()`, mirroring the existing `skip_bytes` trait method's own
implementation, inlined here because the generic-`TC` inherent `impl` block these functions live in
cannot call the `StatefulDecode` trait's `skip_bytes` - that trait is only implemented for the
default `TC = SpecificCharacterSet`). Wired into all 9 functions, called right after their own
decode step and before the existing `self.position += len as u64` line, which needed no further
change (it was already correct as a total once the remainder is actually consumed from the
stream). This is a strictly more lenient alternative to candidate 108's own reject-outright fix -
it closes the same "silent desync" safety issue candidate 108 already confirmed, while additionally
letting real-world non-conformant-but-common files (like the one in #698) parse successfully,
matching dcmtk/dcmjs's own behavior. If either fix is ever actually merged, the other's approach
should not also be applied (they're alternative resolutions of the same defect, not additive) - see
"What's not done" for a note on which one might be more appropriate for this project to adopt.

Verified against the repro: before the fix, the file fails to parse (3/3 runs); after the fix, the
file parses successfully with both elements correctly interpreted, 3/3 runs - see `output.txt` for
the full before/after capture, including the warning-free "FIXED" output.

- Project test suite: `cargo test -p dicom-parser -p dicom-object` - 75+9+49+22+0 passed, 0 failed
  across all reported groups, before AND after (identical counts, zero regressions - no existing
  test exercises a misaligned length on any of these 9 VRs, matching candidate 108's own finding).
  Full workspace `cargo test --workspace --exclude dcms` (dcms excluded: untracked scratch crate
  belonging to a different session, not part of this workspace's tracked tree) - 0 failures
  anywhere before or after.

## What's not done / open questions

- **This candidate and candidate 108 are alternative, mutually-exclusive fixes for the same root
  cause** (reject vs. tolerate-and-skip) - a maintainer needs to choose one, not apply both. Given
  issue #698 is a real user complaint about interoperability with other conformant toolkits (not a
  security/malicious-input report), and PS3.5's own leniency principle ("be liberal in what you
  accept"), tolerate-and-skip (this candidate) may be the more appropriate choice for this specific
  defect class - but that is a project policy decision, not something this session decides.
- Not independently re-verified: whether `read_u32_to_vec` (candidate 068's own separately-fixed
  target, the basic offset table reader) would benefit from the same tolerant-skip treatment
  instead of - or in addition to - its own existing fix; not in scope for this candidate, which
  only touches the 9 functions candidate 108 already covers.
- The specific real file attached to issue #698 itself was not downloaded/tested directly this
  session (no network access) - this candidate's repro instead hand-constructs a minimal wire-byte
  sequence with the exact same declared-length-vs-element-count shape described in the issue's own
  prose (a VM=2 `UL` tag declaring length 10 instead of 8), which is sufficient to confirm the
  mechanism and the fix without needing the original attachment.
