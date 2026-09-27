# pdu-writer-u16-chunk-length-truncation

`ul/src/pdu/writer.rs`'s `write_chunk_u16` and `write_chunk_u32` - the two shared helpers used to
write every length-prefixed sub-item across the entire A-ASSOCIATE-RQ/AC/RJ PDU encoder - compute
their length field with `data.len() as u16` / `data.len() as u32` and then unconditionally write
the *full, untruncated* `data` afterward. For any chunk whose actual content exceeds the target
integer's range (65535 bytes for the `u16` variant, `u32::MAX` for the `u32` one), the declared
length and the real byte count on the wire disagree - the PDU is silently corrupted, with no error
raised anywhere on the write path. Found via a clippy pedantic sweep (`cast_possible_truncation`)
targeted at `ul/`, `json/`, `transfer-syntax-registry/`, and the CLI tool crates per this session's
own risk-map-guided discovery round (the same technique that found candidate 133, in the same
directory).

`write_chunk_u16` is reachable with genuinely unbounded, caller-controlled data through several
public API paths that perform no length validation at all: `UserIdentity::new()`'s
`primary_field`/`secondary_field` (exposed via `ClientAssociationOptions::username`/`password`/
`kerberos_service_ticket`/`saml_assertion`/`jwt`), and `UserVariableItem::SopClassExtendedNegotiationSubItem`'s
raw `data` field. A Kerberos service ticket carrying a large PAC (many group memberships) is a
realistic real-world case that can exceed 64 KiB; this crate does not need anything adversarial to
hit this, an oversized SAML assertion or JWT would do the same.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo clippy -p dicom-ul --no-deps -- -W clippy::pedantic` (default features, to avoid unrelated build failures in `--all-features` builds of `transfer-syntax-registry`'s optional native codec deps) | df2b9e42 | `cast_possible_truncation` flagged `data.len() as u16`/`data.len() as u32` in both `write_chunk_u16`/`write_chunk_u32`, plus the already-fixed candidate 133 site (still present on this unpatched checkout, as always) |
| Read every call site of `write_chunk_u16` (grep, ~20 sites) | `ul/src/pdu/writer.rs` | Most sites write small, spec-fixed-size or already-bounded fields (AE titles, single-byte flags, negotiated syntax strings). Three genuinely unbounded, caller-supplied `Vec<u8>`/`&[u8]` sites identified: `UserIdentity::primary_field()`/`secondary_field()` and `SopClassExtendedNegotiationSubItem`'s `data` |
| Standalone `repro/`: real end-to-end SCU/SCP association over a real loopback TCP socket, SCU calls the real public `ClientAssociationOptions::kerberos_service_ticket()` with a 70,000-byte string | release build, `dicom-ul` public API only | Reproduced: the SCU's real `write_pdu` call succeeds in *sending* the corrupted PDU (declared secondary-field length `4464` = `70000 mod 65536`, but 70,000 real bytes follow it on the wire) - the corruption only surfaces later as an opaque `failed to receive pdu`/`failed to receive pdu` failure on both sides, not a clean upfront rejection. A control case (1,000-byte ticket) establishes normally on both the pristine and patched trees |
| Repeat-run check | `cargo run --release` re-run multiple times on the pristine and patched trees | Deterministic - the corrupted length is a pure function of the ticket length, no randomness involved |

## Status

**Confirmed** - deterministic, reproduces on every run, release build.

**Fully reproducible**: **Yes**. `repro/` runs a real `dicom-ul` SCU and a real `dicom-ul` SCP as
two participants on a real loopback TCP socket, each using the crate's own public API and its own
genuine send/receive path. No PDU bytes are hand-authored: the corrupted length and the resulting
association failure are what the SCU's real `write_pdu` call and the SCP's real PDU reader
actually produced/encountered.

## Class

logic (protocol/wire-format correctness - a silently self-corrupting encoder)

## Root cause

`write_chunk_u16`/`write_chunk_u32` build the chunk's content into a `Vec<u8>` first, then cast
its length directly to the target wire-format integer width with `as`, which truncates silently
in every Rust build profile (unlike arithmetic overflow, an `as` truncating cast never panics even
in a debug build). The subsequent `writer.write_all(&data)` has no awareness of the cast that just
happened and writes every byte of `data` regardless - so the two halves of the same chunk (its
declared length, and its actual content) come from the same value read at two different widths,
guaranteed to disagree once that value exceeds the narrower one's range.

Every one of the ~20+ length-prefixed sub-items in the A-ASSOCIATE-RQ/AC/RJ PDU format funnels
through one of these two functions, so any future caller-controlled field routed through either
one inherits the same exposure by construction, not just the three sites checked directly in this
session.

## Introduced by

b17d1bc0 "Added dicom-ul types and scpproxy." (2019-09-02) - both `write_chunk_u16`'s and
`write_chunk_u32`'s truncating casts are present, in essentially their current form, in the commit
that first introduced this module.

## The fix

`patch.diff` (+20/-2, one file):

- Added a `ChunkTooLarge { len: usize, max: u32, backtrace }` variant to `WriteChunkError`.
- `write_chunk_u32`: replaced `data.len() as u32` with `u32::try_from(data.len())`, mapping a
  conversion failure to `ChunkTooLargeSnafu { len: data.len(), max: u32::MAX }` instead of
  silently truncating.
- `write_chunk_u16`: same pattern, `u16::try_from(data.len())`, `max: u32::from(u16::MAX)`.

Both functions now return a clear, local error *before* writing a single byte of the mismatched
length/data pair, for any of their ~20+ call sites at once - a single fix in the shared helper,
matching this project's own established convention for a defect that lives in the helper itself
(not the "shared-helper-serves-incompatible-callers" case candidate 055 documented, since every
caller here wants the exact same correct behavior: refuse rather than corrupt).

Checked against more than the one triggering input (`CONFIRMATION.md` section 7): a legal
1,000-byte ticket still establishes normally after the patch (no false positive for ordinary-sized
fields), and the association-establishment test suite (which exercises many other `write_chunk_u16`/
`write_chunk_u32` call sites with small, legitimate payloads) is unaffected.

- `cargo test -p dicom-ul --all-features` (lib + doctests): 33 passed / 0 failed / 1 ignored (lib)
  + 20 doctests passed, identical before and after.
- `cargo test --workspace --exclude dcms --lib`: 351 passed / 0 failed / 2 ignored, identical
  before and after.

## What's not done / open questions

- Only `ClientAssociationOptions::kerberos_service_ticket()` was exercised end-to-end. The other
  `UserIdentity`-producing builders (`username`, `password`, `username_password`, `saml_assertion`,
  `jwt`) and `UserVariableItem::SopClassExtendedNegotiationSubItem`'s raw `data` field go through
  the exact same `write_chunk_u16` call and are fixed by the same patch, but were not each given
  their own standalone repro - reasoned to be safe by the shared-helper fix, not independently
  re-verified one by one.
- The SCP (acceptor) side was not separately audited for the same class: `ServerAssociationOptions`
  writing an A-ASSOCIATE-AC also goes through `write_pdu` and hence the same two helpers (e.g. an
  overly long negotiated response item), but no specific overlong-response-field scenario was
  constructed to trigger it from that side.
- The exact DICOM-level consequence of receiving such a corrupted PDU (what a *non-`dicom-rs`* SCP
  implementation would do with it) was not investigated - this crate's own reader happens to fail
  the association cleanly enough in the repro, but a receiver with laxer parsing could behave
  differently.
