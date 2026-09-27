# Decoder-side VR/PrimitiveValue mismatch (follow-up to candidates 092/093) - not found

Follow-up from `IDEAS.md`'s backlog: candidates 092/093 confirmed that `parser/src/stateful/
encode.rs` can be handed a `DataElementHeader`/`PrimitiveValue` pair whose VR and value variant
disagree (per PS3.5's "always textual" vs "genuinely binary" VR classification), but only when
that pair was assembled via the public mutation APIs (`DataElement::new`/`put`/`update_value`).
This candidate checks whether `parser/src/stateful/decode.rs` (the decode side) can independently
produce that same kind of mismatch purely by parsing untrusted bytes, with no such API call
involved - specifically chasing the concrete lead named in the backlog,
`determine_vr_based_on_pixel_representation` (`decode.rs:1145`, returns `Some(VR::SS)` for
ambiguous "US or SS" tags when Pixel Representation is signed).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: traced every arm of `read_value`/`read_value_preserved`/`read_value_bytes` (`decode.rs:988-1080`) against the concrete `read_value_*` function each dispatches to, and what `PrimitiveValue` variant each of those functions unconditionally constructs | Static reading, no build | Every dispatch arm's `header.vr()` match target is a VR whose corresponding `read_value_*` function always returns the one `PrimitiveValue` variant classified as "compatible" with that VR by candidate 092/093's own fix (including `VR::DS`->`F64` and `VR::IS`->`I32`, which 092's fix explicitly special-cased as legitimate, non-mismatched pairings). `decode_header` (`decode.rs:956-975`) mutates `header.vr` in place via `determine_vr_based_on_pixel_representation` *before* the header is used for anything else - the same, already-corrected header object is what both `read_value`'s dispatch and the caller (`dataset/read.rs`'s `last_header`/`DataToken::ElementHeader`, and `lazy_read.rs`'s identical pattern) see and store. No path found where the VR used to pick the read function differs from the VR attached to the resulting element. |
| Existing test, real run: `cargo test -p dicom-parser decode_and_use_pixel_representation -- --nocapture` (Implicit VR LE, Pixel Representation=1 then PixelPadding) | Debug build, existing in-tree test, unmodified | `test ... decode_and_use_pixel_representation ... ok` - confirms `header_2.vr == VR::SS` after the override, matching what `read_value` would then dispatch to (`read_value_ss` -> `I16`). No mismatch. |
| Boundary-value construction, real run: temporary test added to `parser/src/stateful/decode.rs` (reverted after, see below) using **Explicit VR LE** instead of Implicit, with the wire *explicitly* declaring `VR::US` (not `VR::SS`) for the PixelPadding-like tag, after Pixel Representation=1 | Debug build, `cargo test -p dicom-parser tmp_bughunt_explicit_vr_pixel_representation_override -- --nocapture`, then `git checkout -- parser/src/stateful/decode.rs` immediately after capturing output | `header_2 = DataElementHeader { tag: Tag(0x0028, 0x0120), vr: SS, len: Length(2) }` / `value = I16([-1])`. The override fires unconditionally even though Explicit VR's own wire encoding already gave a concrete, unambiguous VR (`US`) - a real, distinct logic bug (the decoder silently discards what Explicit VR explicitly declared and reinterprets the same 2 bytes as a different sign), but `header.vr` (`SS`) and the resulting `PrimitiveValue` variant (`I16`) still agree with each other - not the VR-vs-PrimitiveValue-variant mismatch this candidate was chasing. Filed as a new, separate `IDEAS.md` lead instead of chased further here (out of this candidate's scope). |

## Status

**Not reproducible.** Covered: (1) full static trace of all three `read_value*` dispatch tables
and the one place (`decode_header`) that can rewrite a header's VR before dispatch, confirming the
rewrite always happens in place on the exact header object later reused for dispatch and for the
emitted `DataElement`/`DataToken`; (2) the project's own existing unit test for the exact named
lead (Implicit VR); (3) a hand-built boundary case designed to be the most adversarial variant of
the same lead (Explicit VR with a wire-declared VR that disagrees with the heuristic), run for
real, which still produced an internally-consistent VR/value pair. Three converging approaches,
no signal of a VR/`PrimitiveValue`-variant mismatch anywhere in the decode path.

One code path was **not** exhaustively hand-verified beyond reading: `ValueReadStrategy::Raw`
(`dataset/read.rs`, `read_value_bytes`) deliberately returns `PrimitiveValue::U8` (raw bytes) for
*every* VR except `SQ`, including textual VRs like `UI`/`PN`/`LO` - by its own doc comment this is
intentional, documented, opt-in behavior ("All primitive values are fetched as raw byte buffers,
without any form of decoding or interpretation... This strategy is not recommended"), not a latent
decoder bug; the resulting bytes are exactly the original wire bytes, so re-encoding them verbatim
(which is what the current, unpatched `encode.rs`'s raw-byte fallback does for a `U8` value) is a
lossless, correct round-trip rather than corruption. It was not re-run through the actual encoder
this session to double check, since it does not fit the "decoder itself produces an inconsistent
pairing" framing this candidate is checking (the VR/value pairing here is consistent with what
`Raw` promises: unconditionally raw for every VR) - noted rather than silently assumed safe.

## Class

N/A (not reproduced).

## Root cause

Not applicable to the searched-for bug class - see "Status" above for why the suspect turned out
to be safe: `decode_header` is the single point where a header's VR can be corrected after being
read off the wire, and it does so by mutation on the same object instance that dispatch and
downstream storage/emission both use, so `read_value`'s per-VR dispatch table (each arm's target
function always builds the one `PrimitiveValue` variant matching that VR, per 092/093's own
classification) can never disagree with the VR ultimately attached to the produced element. This
is structurally different from the encode-side bug (092/093), where the mismatch is possible
specifically *because* the public mutation API lets a caller set an arbitrary VR and an arbitrary
value independently of one another - decode.rs has no equivalent independent-assignment path.

## The fix

Not applicable - no bug found to fix in the decoder for this specific class.

- Project test suite: N/A (no fix applied; existing suite left untouched - the one pre-existing
  test relevant to this lead, `decode_and_use_pixel_representation`, was re-run and passes: see
  "What was tried" above).

## What's not done / open questions

- The Explicit-VR override finding surfaced while chasing this lead (`decode_header` forcing
  `VR::SS`/`VR::US` based on Pixel Representation even when Explicit VR's own wire encoding already
  declared a concrete, different VR) is a real, distinct, reproducible decoder-side logic bug - but
  it is a "decoder overrides/ignores the wire's own explicit VR declaration" bug, not a "VR
  disagrees with the stored `PrimitiveValue` variant" bug, so it is out of this candidate's scope.
  Added to `IDEAS.md` as its own new lead rather than chased here (per `SKILL.md`'s "one bug per
  candidate folder").
- `ValueReadStrategy::Raw`'s universal `U8`-for-every-VR behavior was read and reasoned about but
  not independently re-verified this session by actually running a Raw-strategy decode -> re-encode
  round trip through the real `write_dataset_with_ts`; noted as documented/intentional rather than
  confirmed harmless end-to-end.
- Did not check `object`/`json` crates' own decode-adjacent construction paths (e.g. building an
  `InMemDicomObject` from parsed tokens) for a *second* place where a VR could be attached
  independently of the value that was actually decoded for it - only `parser/src/stateful/
  decode.rs` and the two `DataSetReader`/`LazyDataSetReader` consumers that store/reuse its headers
  were checked.
