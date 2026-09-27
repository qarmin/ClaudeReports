# VirtualVr Ox/Px/Lt relaxed() override may ignore Explicit VR file's own wire-declared VR

`core/src/dictionary/data_element.rs`'s `VirtualVr::relaxed()` unconditionally maps `Ox`/`Px` ->
`VR::OW` and `Lt` -> `VR::OW` when a dictionary lookup can't resolve them precisely, the same
function whose `Xs` -> `US` case candidate 095 confirmed and fixed as a real bug (an override that
discarded an Explicit VR file's own wire-declared VR). Noticed while working candidate 095 as an
explicit follow-up: are `Ox`/`Px`/`Lt` affected by the same class of bug at either of the only two
`.relaxed()` call sites in the repo (`encoding/src/decode/implicit_le.rs:101`,
`encoding/src/decode/adaptive_le.rs:107`)?

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of both `.relaxed()` call sites and everything that calls into them | n/a | `implicit_le.rs`'s `decode_header` is only ever instantiated for the Implicit VR Little Endian transfer syntax, where by construction the wire never carries a VR at all for any tag - `relaxed()` there is not "overriding" anything, it is the *only* source of a VR, so there is nothing to discard. `adaptive_le.rs`'s `resolve_vr`/`relaxed()` is only reached from the `VrState::Implicit`-locked branch or the `Unknown`-probe branch that itself just decided the data is Implicit; the `VrState::Explicit` branch and `decode_explicit_header` never consult the dictionary or call `relaxed()` at all - they read the wire's own 2-byte VR code directly. Unlike candidate 095's bug (`StatefulDecoder::determine_vr_based_on_pixel_representation`, a *separate, later* post-decode correction step in `parser/src/stateful/decode.rs` that ran unconditionally regardless of which decoder produced the header, overwriting an already-correctly-decoded wire VR), there is no equivalent later correction step anywhere in the codebase for `Ox`/`Px`/`Lt` (confirmed by `grep -n "VirtualVr::Ox\|VirtualVr::Px\|VirtualVr::Lt"` across the whole repo outside dictionary/test code: no hits besides the `relaxed()` definition itself and the `adaptive_le.rs` probe-compatibility helper) |
| Manual boundary-value construction: a temporary unit test added to `parser/src/stateful/decode.rs`'s own test module, decoding a well-formed Explicit VR Little Endian header for OverlayData (6000,3000, dictionary `VirtualVr::Ox`) with wire VR explicitly `OB` (the non-`relaxed()`-default alternative) through the real `ExplicitVRLittleEndianDecoder` | debug build, `cargo test -p dicom-parser --lib` | Decoded `header.vr == VR::OB`, i.e. the wire-declared VR was preserved exactly, not overridden to the `relaxed()` default `OW` - deterministic (reran twice, same result both times; pure logic, no flakiness expected) |
| Same manual construction, second boundary case: PixelData (7FE0,0010, dictionary `VirtualVr::Px`) with wire VR explicitly `OB`, decoded through `AdaptiveVRLittleEndianDecoder` (the other of the two real `.relaxed()` call sites, exercising its `VrState::Unknown` probe -> locks to `Explicit` path since `OB` probes as a valid, dictionary-compatible VR code) | same debug build/test run | Decoded `header.vr == VR::OB` as well - wire VR preserved, `relaxed()` never invoked for this header |

Two escalating approaches were enough: code reading located the exact mechanism and ruled out the
existence of an `Xs`-style later correction step for `Ox`/`Px`/`Lt`, and manual boundary-value
construction (the fitting tool per `HEURISTICS.md` for a logic bug with a documented invariant to
check) then confirmed both real call sites concretely preserve an Explicit VR file's own
wire-declared VR for these dictionary variants. No third tool was needed - there was no partial
signal to chase further.

## Status

**Not reproducible.** The suspected mechanism (an unconditional relaxed-default override
discarding an Explicit VR file's own wire-declared VR for `Ox`/`Px`/`Lt`, analogous to candidate
095's `Xs` bug) does not exist in the current codebase. Candidate 095's bug was specific to a
*separate* post-decode correction function (`determine_vr_based_on_pixel_representation`) that ran
regardless of decoder type; no equivalent function exists anywhere for `Ox`/`Px`/`Lt`. The two
`.relaxed()` call sites themselves are both structurally gated already: `implicit_le.rs` only runs
under an inherently VR-less transfer syntax, and `adaptive_le.rs` only reaches `relaxed()` once it
has itself determined (by decoder-locked state or live probing) that the data being decoded is
genuinely Implicit VR - its Explicit-VR branch never touches the dictionary at all.

## Class

n/a (not reproduced)

## Root cause

Not applicable as a bug - the suspected mechanism is absent. For completeness, the actual
mechanism that *is* present: `implicit_le.rs::decode_header` and `adaptive_le.rs::resolve_vr` both
special-case Pixel Data (7FE0,0010) and Overlay Data (60xx,3000) directly to `VR::OW` *before*
falling back to a plain dictionary lookup + `relaxed()` for anything else - so even the fallback
`relaxed()` path for those two specific `Ox`/`Px` tags is dead code in practice (the direct
special-case always wins for the tags that matter most), and for `Lt` (LUT Data-related tags) there
is no such special case, but the `relaxed()` result is still only ever used when the decoder has
already independently established there's no wire VR to preserve in the first place.

## The fix

Not applicable - no bug found to fix.

## What's not done / open questions

- This only checked the two known `.relaxed()` call sites and the class of bug candidate 095 was
  about (silent override of an Explicit VR file's own declared VR). It did not re-audit
  `VirtualVr::relaxed()`'s callers for unrelated concerns (e.g. whether `UN`-fallback-then-`OW`
  reinterpretation for a genuinely ambiguous Implicit VR `Ox`/`Px`/`Lt` tag is itself always the
  medically-correct choice) - that's a separate question from what this candidate was scoped to.
- Removed the corresponding backlog row from `IDEAS.md` since it's now fully investigated; no new
  call sites of `.relaxed()` were found beyond the two already known.
