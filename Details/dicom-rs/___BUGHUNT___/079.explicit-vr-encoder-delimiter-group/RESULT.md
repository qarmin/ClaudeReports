# explicit-vr-encoder-delimiter-group

`encoding/src/encode/explicit_be.rs` and `explicit_le.rs` - `ExplicitVRBigEndianEncoder`/
`ExplicitVRLittleEndianEncoder`'s shared `Encode::encode_element_header` - have no guard for a
tag in the reserved group `0xFFFE` (Item, Item Delimitation Item, Sequence Delimitation Item).
Those tags are, by the DICOM standard, always encoded as a bare 4-byte tag + 4-byte length (8
bytes total, no VR field, no reserved bytes) in every transfer syntax - even ones, like Explicit
VR LE/BE, that otherwise always write VR explicitly. Flagged by an unmerged fix branch
(`origin/fix/explicit-vr-encoder-delimiter-group`, 779a3c09) touching exactly these two files;
checked for overlap with already-completed candidate 049 (`parser/src/stateful/encode.rs`) before
writing up, and found to be a genuinely distinct exposure - see "Root cause" below.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: compared this candidate's two files against candidate 049's already-fixed location (`parser::stateful::encode::StatefulEncoder::encode_element_header`) to determine whether 049's guard already covers this spot | N/A | 049's guard lives one layer up, in `dicom-parser`, and runs *before* delegating to exactly the `dicom-encoding::Encode::encode_element_header` implementations this candidate is about; it does not touch `encoding/src/encode/explicit_be.rs`/`explicit_le.rs` at all. Grepping every real (non-test) call site of `Encode::encode_element_header` in this workspace found only two: `parser/src/stateful/encode.rs` (covered by 049) and `parser/src/dataset/write.rs` (which itself calls through `StatefulEncoder`, also covered by 049) - so *within this workspace's own crates*, 049's fix happens to shadow this bug. But `dicom-encoding`'s `Encode` trait and its concrete encoders are public API, independent of `dicom-parser` - any external caller using `dicom-encoding` directly is not protected by 049's guard at all. Confirmed genuinely distinct, not a duplicate |
| Manual boundary-value construction: a small standalone program driving `dicom-encoding`'s own public `Encode`/`Decode` traits directly (`ExplicitVRLittleEndianEncoder`/`Decoder`, `ExplicitVRBigEndianEncoder`/`Decoder`) - encode a `DataElementHeader` for each of the three group-`0xFFFE` tags with a representative VR each (`UN`, short-form `UI`, long-form `OB`), then decode the bytes back with the same transfer syntax's own decoder; an ordinary tag as a control case | Debug build, `dicom-encoding` used directly via its path dependency, no `dicom-parser`/`dicom-object` involved | All three reserved tags round-tripped incorrectly (wrong byte count and/or garbage length after decode); the ordinary tag round-tripped correctly. Reproduced deterministically: 2+ runs during investigation plus a fresh `just test` run against a pristine clone of upstream `df2b9e42` (see `output.txt`) |

Only one real tool was needed here since the very first attempt (reasoning from the documented
invariant - group `0xFFFE` is always a bare tag+length on the wire, in every transfer syntax - and
constructing an input that violates it) produced a clean, deterministic, immediately-explainable
mismatch; no fuzzing/sanitizer escalation was needed for this arithmetic-free logic bug.

## Status

**Confirmed**. Deterministic - reproduces on every run (2 in-tree runs during investigation, plus
one full `just test` from a pristine clone), for any of the three group-`0xFFFE` tags, with any
VR, in both Explicit VR Little Endian and Big Endian.

**Fully reproducible**: **Yes**. `repro/` drives `dicom-encoding`'s own real, public `Encode` and
`Decode` trait implementations directly - the same API any real caller of this crate (with or
without `dicom-parser` in between) would use - with no hand-authored "pretend decoder" or
"pretend encoder" standing in for either side of the round trip. This is a single-process,
single-participant finding (an encoder violating its own decoder's documented wire-format
assumption for reserved tags) with no second peer/caller to fake, so `CONFIRMATION.md` section
11's two-participant repro requirement does not apply here (same shape as candidate 049's own
repro).

## Class

`logic` (a write/read round-trip asymmetry, same class as candidate 049: the encoder silently
produces bytes that violate the DICOM wire format's own reserved-tag convention, and this crate's
own matching decoder for that transfer syntax cannot parse the result back correctly - no crash,
no memory-safety issue, just wrong bytes on the wire).

## Root cause

DICOM reserves group `0xFFFE` exclusively for the Item, Item Delimitation Item, and Sequence
Delimitation Item structural markers, and mandates they always be encoded as a bare tag + 4-byte
length (no VR field, no reserved bytes) - in *every* transfer syntax, including Explicit VR LE/BE
which otherwise always write an explicit VR field. Both `ExplicitVRLittleEndianDecoder` and
`ExplicitVRBigEndianDecoder`'s `decode_header` already encode this: they check `group == 0xFFFE`
first and, if so, read exactly 4 more bytes as the length with no VR field at all
(`encoding/src/decode/explicit_le.rs:33-41`, mirrored in `explicit_be.rs`). The corresponding
encoders' `encode_element_header`, however, had no matching check: it dispatches purely on
`de.vr()`, so a `DataElementHeader` whose tag happens to be in group `0xFFFE` falls straight into
either the 8-byte "short-form VR" branch (VR text + 2-byte length, e.g. for `VR::UI`) or the
12-byte "long-form VR" branch (VR text + 2 reserved bytes + 4-byte length, e.g. for `VR::UN`/
`VR::OB`) - both of which write a VR field the matching decoder never expects for this group,
producing a decoded length built out of what were actually VR-text/reserved bytes (e.g. `UN`'s
ASCII bytes `0x55 0x4e` misread as part of a little-endian length, yielding the corrupted
`Length(20053)` seen in `output.txt`).

This is the same class of defect as candidate 049 (`parser::stateful::encode::encode_element_header`
missing an equivalent guard), but at a different, additional layer: 049's fix lives in
`dicom-parser`, one level above the two functions this candidate is about, and runs *before*
delegating to them - which happens to make it an effective guard for every call site reachable
through `dicom-parser` (both call sites in this workspace's own crates go through it, per the
"What was tried" table above). It provides no protection at all, however, for any caller that uses
`dicom-encoding`'s own public `Encode` trait and concrete encoders directly, bypassing
`dicom-parser` entirely - which is a legitimate, intended use of a crate that exposes this trait
and these encoder types as public API. Fixing the guard directly in `explicit_be.rs`/
`explicit_le.rs` (as done here) protects that direct-use case as well, in addition to being closer
to the actual mechanism (the decoder's own documented group-`0xFFFE` special case) than 049's
higher-layer guard.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Applied the fix from `origin/fix/explicit-vr-encoder-delimiter-group` (779a3c09) verbatim: an
early-return fast path at the top of `encode_element_header` in both files, triggered whenever
`de.tag().group() == 0xFFFE`, that writes only the bare 8-byte tag+length shape (mirroring the
decoder's own special case) and returns before the VR-dispatch `match` is ever reached:

```rust
if de.tag().group() == 0xFFFE {
    let mut buf = [0u8; 8];
    LittleEndian::write_u16(&mut buf[0..], de.tag().group());
    LittleEndian::write_u16(&mut buf[2..], de.tag().element());
    LittleEndian::write_u32(&mut buf[4..], de.length().0);
    to.write_all(&buf).context(WriteHeaderSnafu)?;
    return Ok(8);
}
```
(and the `BigEndian` equivalent in `explicit_be.rs`). Unlike 049's guard, this does not reject the
write - it makes it correct, matching what the legitimate internal delimiter-writing methods
(`encode_item_header`, `encode_item_delimiter`, `encode_sequence_delimiter`) already produce and
what the decoder already expects; either approach (reject vs. correct-in-place) is valid here,
this candidate followed the upstream branch's own choice rather than reusing 049's stricter
"reject" style, since a normal `DataElementHeader::new(tag_in_group_fffe, ...)` reaching this
low-level function is not necessarily a programming error the way it is one layer up in
`dicom-parser` (a caller building raw delimiter bytes through this public API directly is a
plausible legitimate use, unlike `dicom-object`'s higher-level API where 049 argued the tag should
never legitimately appear at all).

Verified via `repro/` (see `output.txt`): before the patch, all three group-`0xFFFE` tags
round-trip incorrectly in both transfer syntaxes; after, all three round-trip correctly (bare
8-byte tag+length, decoded length matches what was written) and the ordinary-tag control case is
unaffected either way.

- `cargo test -p dicom-encoding --lib`: **40 passed / 0 failed** before the patch -> **40 passed /
  0 failed** after (the crate's own existing `encode_items`/`decode_items` tests, which only
  exercise the dedicated delimiter-writing methods, not this path, are unaffected either way).
- Full workspace (`cargo test --workspace --lib`, all crates with a `#[lib]` target): **351
  passed / 0 failed / 2 ignored** before the patch -> **351 passed / 0 failed / 2 ignored** after
  - zero regressions anywhere in the existing suite.

## What's not done / open questions

- Implicit VR (`implicit_le.rs`) was checked and does **not** need this fix: its header format for
  any VR is already a bare tag(4)+length(4) = 8 bytes with no VR field at all, so writing an
  ordinary element header for a group-`0xFFFE` tag already produces exactly the same shape a real
  delimiter would - there is nothing to distinguish there. This matches why the upstream fix
  branch only touched the two Explicit VR files.
- Whether a real, in-the-wild caller of `dicom-encoding` (outside this workspace) ever actually
  constructs a `DataElementHeader` with a group-`0xFFFE` tag and passes it through
  `encode_element_header` directly (rather than going through the dedicated delimiter methods, or
  through `dicom-parser`) was not established - only that the public API allows it and silently
  misbehaves if it happens, which is the same reachability caveat 049 itself carried for its own
  layer.
- No new regression test was added to `encoding/src/encode/explicit_be.rs`/`explicit_le.rs`'s own
  test modules in this session (the fix was verified via the standalone `repro/` and the existing
  test suite instead); adding one there (asserting the 8-byte shape and successful decode for each
  of the three tags, mirroring 049's `encode_element_header_with_reserved_tag_fails`) would be a
  reasonable low-cost follow-up if this patch is taken.
