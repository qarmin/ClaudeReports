# Writing pixel data with a primitive/sequence value under the encapsulated-pixel-data tag/VR/length signature panics

Found in the same systematic `unreachable!()` sweep as `025` (triggered by two hits in a row in
`object/src/meta.rs`). `parser/src/dataset/mod.rs`'s `DataElementTokens::next` (the iterator
that turns a `DataElement` into a stream of `DataToken`s for writing) classifies which kind of
token to emit for an element using `impl From<DataElementHeader> for DataToken`: the specific
combination `(VR::OB, Tag(0x7fe0, 0x0010))` with an **undefined length** is recognized as the
start of encapsulated pixel data (`DataToken::PixelSequenceStart`), independently of what the
element's actual stored `Value` is. The `PixelSequenceStart` branch then matches on
`elem.into_value()` expecting `Value::PixelSequence`, and falls through to `unreachable!()` for
`Value::Primitive`/`Value::Sequence`. The sibling `SequenceStart` branch (a few lines above,
handling the equivalent VR::SQ mismatch) already anticipates and gracefully handles exactly this
kind of tag/VR-vs-value inconsistency with a code comment saying "this can only happen in
malformed data... but we try to handle it gracefully anyway" - the `PixelSequenceStart` branch
was simply never given the same treatment.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `parser/src/dataset/mod.rs`'s `DataElementTokens::next`, `SequenceStart` branch (handles `Value::Primitive`/`Value::PixelSequence` mismatches gracefully with a `tracing::warn!` and a synthesized fallback token) vs. `PixelSequenceStart` branch (only handles the expected `Value::PixelSequence` case, `unreachable!()` otherwise). Traced `impl From<DataElementHeader> for DataToken` to find exactly which tag/VR/length combination triggers `PixelSequenceStart` classification: `(VR::OB, Tag(0x7fe0, 0x0010))` with `header.len.is_undefined()`. | Identified that constructing a `DataElement` with this exact tag/VR/length signature but an ordinary `Value::Primitive` (or `Value::Sequence`) reaches the unhandled arm. |
| Standalone repro (release build, public API: `DataElement::new_with_len`, `write_dataset_with_ts`) | `___BUGHUNT___/026.../repro`. Built a `DataElement` with tag `(0x7fe0, 0x0010)`, VR `OB`, `Length::UNDEFINED`, and an ordinary `PrimitiveValue::U8` value, put it into an `InMemDicomObject`, and called `write_dataset_with_ts`. | Reproduced deterministically: panicked with `internal error: entered unreachable code` at `parser/src/dataset/mod.rs:644:77`. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically (fully deterministic). |
| Reachability check | Nothing in `DataElement`'s public constructors (`new`, `new_with_len`) validates that a given tag/VR/length combination is paired with a consistent `Value` variant - it is entirely possible to construct exactly this element by hand via the ordinary public API, and this is precisely the situation the `SequenceStart` branch's own comment already anticipates for the VR::SQ case ("this can only happen in malformed data (wrong VR)"). | N/A (confirms this is a foreseeable, not merely theoretical, input shape - the crate's own code already handles the structurally identical VR::SQ case). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build.

## Class

`crash` (unhandled `unreachable!()` panic when writing an object containing an element whose
tag/VR/length signature identifies it as encapsulated pixel data, but whose actual value is not
a pixel data fragment sequence).

## Root cause

`DataElementTokens::next`'s `DataToken::PixelSequenceStart` branch (`parser/src/dataset/mod.rs`)
assumes `elem.into_value()` is always `Value::PixelSequence`, because that's the only case the
`(VR::OB, Tag(0x7fe0, 0x0010))`-with-undefined-length signature is *supposed* to represent - but
the signature is derived purely from the header (tag/VR/length), while the actual value is a
separate, independently-settable field on the same `DataElement`. The structurally identical
mismatch for `VR::SQ` (handled by the neighboring `SequenceStart` branch) was already anticipated
and handled gracefully; this arm just never received the same treatment.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added graceful handling to the `PixelSequenceStart` branch's `Value::Primitive`/`Value::Sequence`
arms, mirroring the neighboring `SequenceStart` branch: for a mismatched primitive value, warn
and re-synthesize the token stream as an ordinary element header + primitive value pair (letting
the encoder recompute the correct length from the value itself, as it already does for the "wrong
VR on the SQ path" case); for a mismatched sequence value, warn and skip the element (emitting no
tokens) rather than panicking. See `patch.diff` (`parser/src/dataset/mod.rs`, one match arm split
into two handled cases).

Verified:
- `repro/` re-run against the patched tree no longer panics: the object now writes successfully.
- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, identical
  both before and after the patch (no regressions; no existing test exercised this tag/VR/value
  combination).

## What's not done / open questions

- Did not check whether the equivalent read/decode path (`parser/src/dataset/read.rs`) can ever
  itself produce a `DataToken::PixelSequenceStart` paired with a non-`PixelSequence` value when
  parsing a crafted file (as opposed to this candidate's write-path repro, which relies on a
  caller constructing the mismatched `DataElement` directly via the public API) - the decoder
  likely always constructs the pixel-sequence value type in lockstep with recognizing this same
  tag/VR/length signature, making the read side less likely to hit this specific defect, but this
  was not independently verified.
- Given the two hits in `object/src/meta.rs` (`023`/`024`) and now this one, `unreachable!()` as
  used in this codebase does not reliably indicate a truly-impossible state - three separate,
  independently-reachable instances were found and fixed this session. The remaining ~15
  occurrences not yet audited (`encoding/src/adapters.rs`'s and
  `encoding/src/transfer_syntax/mod.rs`'s were checked and confirmed safe, guarded by
  uninhabited-type constructions `NeverPixelAdapter`/`NeverAdapter`; `core/src/header.rs`'s
  `EmptyObject` likewise; `object/src/mem.rs`'s four `AttributeSelector`-walk sites were checked
  and confirmed safe, guarded by a private-field construction invariant) still leave
  `parser/src/stateful/decode.rs:868`, `core/src/value/range.rs:397`, `object/src/collector.rs:468`,
  `transfer-syntax-registry/src/adapters/jpegxl.rs` (2 sites), and `json/src/de/mod.rs:365`
  unaudited due to running low on session time.
- Not fuzzed - found by the same systematic `unreachable!()` audit as `025`.
