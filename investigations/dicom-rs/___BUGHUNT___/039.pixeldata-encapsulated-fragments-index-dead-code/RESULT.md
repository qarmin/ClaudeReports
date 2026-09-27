# pixeldata-encapsulated-fragments-index-dead-code

`pixeldata/src/lib.rs`'s `decode_pixel_data_frame` override (for `FileDicomObject<InMemDicomObject<D>>`)
has a branch, reached only when the transfer syntax's `Codec` is `EncapsulatedPixelData(None, _)`
(no registered per-codec decoder), that indexes `fragments[frame as usize]` directly with no bounds
check. Flagged in `IDEAS.md`'s backlog by candidate `038` as a plausible sibling of that candidate's
confirmed bug (same file, same shape: direct unchecked index on a caller-supplied `frame`), left
unverified there for lack of a suitable transfer syntax to test with.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: grepped `transfer-syntax-registry/src/entries.rs` for every transfer syntax entry, checking each one's `Codec` variant | Static code reading, no build needed | Every currently-registered transfer syntax that uses `Codec::EncapsulatedPixelData` supplies `Some(_)` for its decoder - including "Encapsulated Uncompressed Explicit VR Little Endian" (`1.2.840.10008.1.2.1.98`), which might plausibly be assumed to have no decoder (being "uncompressed") but is in fact registered with a real adapter (`UncompressedAdapter`, whose `decode_frame` just copies the matching fragment). `grep -n "EncapsulatedPixelData(None" entries.rs` found zero matches across the whole registry |
| Hand-built input: took a real single-frame native file (`pydicom/CT_small.dcm`), re-stored its pixel data as a single-fragment `PixelSequence` under `1.2.840.10008.1.2.1.98`, and requested the out-of-range frame `1` (temporary unit test added to `pixeldata/src/lib.rs`, reverted before this record was written) | `obj.decode_pixel_data_frame(1)` via the public `PixelDecoder` trait | Returned a graceful `Err(DecodePixelData { source: FrameRangeOutOfBounds })` - confirms the request was routed through `UncompressedAdapter::decode_frame`, which delegates to `PixelDataObject::frame_pixel_data(frame)` (a separately, properly bounds-checked method), never reaching the suspected `fragments[frame as usize]` branch at all |

## Status

**Not reproducible** - the suspected branch is genuinely unreachable via any transfer syntax
currently in this crate's registry; every `Codec::EncapsulatedPixelData` entry supplies a real
decoder, so the `None` case (and the unguarded `fragments[frame as usize]` code that handles it)
is dead code as of this checkout.

## Class

n/a

## Root cause

Not applicable (no bug reachable today) - the `fragments[frame as usize]` branch exists to handle
a `Codec::EncapsulatedPixelData(None, _)` transfer syntax (encapsulated pixel data with no
registered decoder function at all), but no such transfer syntax is currently registered. This is
still latent, unguarded code (unlike `038`'s bug, which was confirmed live in a code path that
*is* exercised by an existing registry entry) - a future transfer syntax added to the registry with
`Codec::EncapsulatedPixelData(None, _)` would immediately re-expose this exact panic. Worth a
one-line note for whoever adds a new transfer syntax variant to this registry: **an
`EncapsulatedPixelData(None, _)` entry activates this specific unguarded code path** - not
discovered by reading `entries.rs` alone, only by tracing where each `Codec` variant is consumed
downstream.

## The fix

N/A - no bug found, no fix needed. **Not preemptively hardening the dead branch this session**
(consistent with "attempt a bounded fix" only applying to confirmed reproductions) - but flagging
it as a known landmine in `projects/dicom-rs.md` for whoever next touches the transfer syntax
registry or this specific match arm.

## What's not done / open questions

- Whether a downstream crate (outside this workspace) could register a custom transfer syntax with
  `Codec::EncapsulatedPixelData(None, _)` via the registry's public extension API, and thereby
  reach this branch without any changes to this repository, was not investigated - if the registry
  is extensible by third parties, this could already be live for someone else's registered
  transfer syntax even though nothing in *this* codebase's own entries triggers it.
