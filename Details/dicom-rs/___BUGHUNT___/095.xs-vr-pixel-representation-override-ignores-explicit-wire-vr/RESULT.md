# Xs-VR pixel-representation override ignores explicit wire VR

`parser/src/stateful/decode.rs`'s `StatefulDecoder::decode_header` calls
`determine_vr_based_on_pixel_representation` and unconditionally overwrites `header.vr` to
`VR::SS` whenever _Pixel Representation_ (0028,0103) is signed and the tag's dictionary entry is
`VirtualVr::Xs` (e.g. SmallestImagePixelValue, LargestImagePixelValue, PixelPaddingValue), with no
regard for which transfer syntax is in use. Noticed while working candidates 092/093/094 (a
related but distinct class: there the header VR and the `PrimitiveValue` variant were internally
inconsistent with each other; here they stay internally consistent with each other but
inconsistent with what the file's own Explicit VR encoding said).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `determine_vr_based_on_pixel_representation` and its one call site in `decode_header` | n/a | Confirmed the override is applied regardless of decoder type - no check anywhere for whether the current transfer syntax is Explicit VR (where the wire already carries a real VR) vs Implicit VR (where it doesn't) |
| Manual boundary-value construction: built a real, well-formed Explicit VR Little Endian P10 file (file meta + dataset) via `dicom_object::InMemDicomObject`/`with_meta`/`write_to_file`, with Pixel Representation=1 and `SmallestImagePixelValue` (0028,0106, dictionary VR `Xs`) explicitly declared as `VR::US` on the wire (verified by scanning the written bytes for the literal `US` VR code before reading it back) | debug build, `cargo test -p dicom-object`, read back via the public `dicom_object::open_file` entry point | Reproduced: decoded header VR came back as `SS`, value as `Primitive(I16([-1]))` instead of the wire-declared `Primitive(U16([65535]))` - 4/4 repeat runs, deterministic (pure logic bug, not layout/seed-dependent) |
| Varied the input within the same bug class: a second Xs tag (PixelPaddingValue, 0028,0120) under Explicit VR *Big Endian* (transfer syntax 1.2.840.10008.1.2.2) | same debug build/test harness | Same override fires there too - confirms the bug is general to "any Explicit VR transfer syntax + any Xs-dictionary tag", not specific to one tag or one byte order |

Two escalating approaches were enough - the first (code reading) already located the exact
mechanism, and manual boundary-value construction (the fitting tool per `HEURISTICS.md` for a
logic bug with a documented invariant to violate - "Explicit VR's own declared VR is
authoritative") reproduced it immediately and repeatably; no need for a third.

## Status

**Confirmed**: reproduces 4/4 runs (see `output.txt`), deterministically, on every attempt - both
directly against the main project's working tree (`cargo test -p dicom-object`) and against a
fresh `git clone` of the real upstream repo at `base_commit` via the candidate's own `repro/`
(`just test` / `just reproduce-before`).

**Fully reproducible**: **Yes**. The repro drives the real, public, end-to-end entry point a real
caller would use - `dicom_object::InMemDicomObject::write_to_file` to produce a genuine on-disk P10
file (with the wire bytes independently verified to actually contain the `US` VR code, not just
assumed), then `dicom_object::open_file` to read that exact file back. Per `CONFIRMATION.md`
section 11, a malformed-or-otherwise file handed to a parser's own public `open_file` entry point
*is* the genuine external input - there is no second participant being impersonated here, so no
loopback-socket-style two-participant repro is needed.

## Class

logic

## Root cause

`StatefulDecoder::determine_vr_based_on_pixel_representation` (parser/src/stateful/decode.rs)
implements a heuristic that is only actually needed for **Implicit VR** transfer syntaxes: there,
the wire carries no VR at all, so `ImplicitVRLittleEndianDecoder::decode_header` resolves a
dictionary `VirtualVr::Xs` entry via `VirtualVr::relaxed()`, which always maps `Xs` to `VR::US`
unconditionally (core/src/dictionary/data_element.rs). Since that guess can be wrong for signed
pixel data, the override existed to correct it to `SS` when _Pixel Representation_=1 - a real and
necessary correction *for Implicit VR only*.

The bug is that the override runs unconditionally in `decode_header`, with no check on which
decoder produced the header. Under **Explicit VR** (LE or BE), `ExplicitVRLittleEndianDecoder`/
`ExplicitVRBigEndianDecoder::decode_header` reads the two-character VR code directly from the
wire - it is not a guess, it is the file's own authoritative declaration, which is the entire
purpose of Explicit VR as a transfer syntax. The override still fires in this case and silently
replaces that already-correct, wire-declared VR with `SS` whenever Pixel Representation happens to
be signed, discarding what the file actually said and reinterpreting the same underlying bytes
under a different VR (`U16` -> `I16`) with a different numeric meaning.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added an `is_implicit_vr(&self) -> bool` method to the `DecodeFrom<S>` trait (dicom-encoding),
defaulting to `false`, so no downstream/external decoder implementation breaks by having a new
required method:

- `ImplicitVRLittleEndianDecoder` overrides it to `true` (it is always implicit).
- `AdaptiveVRLittleEndianDecoder` (a decoder for non-conformant files that declare Explicit VR but
  actually contain Implicit VR) overrides it to read its own per-instance `state: Cell<VrState>`,
  since that decoder's mode is only known once it has probed the first non-delimiter element -
  exactly the same element `decode_header` just decoded when the check runs.
- `ExplicitVRLittleEndianDecoder`/`ExplicitVRBigEndianDecoder` keep the default (`false`).
- The forwarding impls of `DecodeFrom` for `&T` and `Box<T>` (used by the boxed `DynDecoder<S>`
  trait object that most real callers actually get from `TransferSyntax::decoder_for`) were updated
  to delegate `is_implicit_vr` to the inner decoder instead of silently falling back to the
  trait's own default.

`StatefulDecoder::determine_vr_based_on_pixel_representation` now additionally requires
`self.decoder.is_implicit_vr()` before applying the `Xs` -> `SS` correction.

Verified against more than just the one triggering input (CONFIRMATION.md section 7): re-ran the
fixed build against both the original Explicit VR *Little Endian* case (tag 0028,0106) and a second
boundary case, Explicit VR *Big Endian* with a different Xs tag (0028,0120) - both now correctly
keep the wire-declared `VR::US`. Also re-ran the pre-existing `decode_and_use_pixel_representation`
unit test (Implicit VR, the legitimate use case the override exists for) - still passes, confirming
the fix didn't regress the correct behavior it's meant to preserve.

- Project test suite (`cargo test --workspace --no-fail-fast`): before the fix, every pre-existing
  suite passed with the same counts as after (0 failures anywhere in the workspace); the only
  failures were the 2 new scratch repro tests added for this candidate (expected, they assert the
  post-fix behavior). After the fix, all suites pass, including those 2 - no regressions, no new
  failures. Exact same pass counts per crate before/after (see `output.txt` for the summary); full
  per-crate breakdown available by re-running `cargo test --workspace --no-fail-fast` against
  `patch.diff` applied/unapplied.

## What's not done / open questions

- `VirtualVr` has two other ambiguous variants relaxed unconditionally by `relaxed()`: `Ox`/`Px`
  (OB or OW) and `Lt` (US or OW). This session did not check whether any other call site applies a
  similar "correct an Implicit-VR dictionary guess" override without gating it on
  `is_implicit_vr()` the same way `Xs`/Pixel-Representation did - worth a follow-up look, added to
  `IDEAS.md`.
- The `AdaptiveVRLittleEndianDecoder`'s `is_implicit_vr()` implementation was verified by code
  reading (its `state` `Cell` is authoritative for the element `decode_header` just decoded) but
  not exercised by a dedicated repro in this session - only `ExplicitVRLittleEndianDecoder`,
  `ExplicitVRBigEndianDecoder`, and `ImplicitVRLittleEndianDecoder` were exercised directly. Worth
  a follow-up repro specifically through the adaptive decoder if it's reachable from a public,
  documented entry point.
