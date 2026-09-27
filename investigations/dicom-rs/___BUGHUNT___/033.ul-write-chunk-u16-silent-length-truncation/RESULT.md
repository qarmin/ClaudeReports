# ul-write-chunk-u16-silent-length-truncation

`ul/src/pdu/writer.rs`'s `write_chunk_u16` (a shared helper used by ~20 call sites across every
DICOM Upper Layer PDU variable-item/sub-item writer - Application Context, Presentation Context,
Abstract/Transfer Syntax, all User Information sub-items including User Identity Negotiation)
computes the wire's 2-byte Item-length prefix as `data.len() as u16`, an infallible truncating
cast, with no check that the already-built `data` buffer actually fits in 16 bits. Found via
manual code reading of `ul/src/pdu/writer.rs` (a network-facing PDU encoder, not covered by
either bug-hunt session so far - the prior session's 26 candidates were entirely
file/pixel-data-focused; this session's earlier `ul/src/pdu/reader.rs` reading, candidate 032,
prompted checking the encode side too, per HEURISTICS.md's "arithmetic on sizes" and this
project's own accumulated lesson about checking both encode and decode sides for the same field).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reading of every `write_chunk_u16`/`write_chunk_u32` call site to find one whose input data can realistically exceed `u16::MAX` (65535) bytes via the public API | Static code reading, no build needed | `UserIdentity::new(positive_response_requested, identity_type, primary_field: Vec<u8>, secondary_field: Vec<u8>)` (`ul/src/pdu/mod.rs:534`) takes two fully unbounded `Vec<u8>` fields with no length validation anywhere in the constructor - written into the wire via `write_chunk_u16` at `ul/src/pdu/writer.rs:1110`/`1123` (line numbers as of `df2b9e42`). `UserIdentityType::SamlAssertion` (wire value 4) is a realistic case where the field's real-world content (a SAML assertion, an XML security token) can plausibly approach or exceed 64KB, not just a contrived stress value |
| Hand-built boundary input (`primary_field` = 70,000 bytes, one byte past what a `u16` length prefix can represent), debug build, `cargo test` (temporary unit test added to `ul/src/pdu/writer.rs`, reverted before this record was written) | `write_pdu(&mut out, &Pdu::AssociationRQ { .. user_variables: vec![UserVariableItem::UserIdentityItem(user_identity)] .. })` via the public `dicom_ul::write_pdu` | `write_pdu` returned `Ok(())` for a 70,113-byte encoded buffer, with the sub-item's declared Item-length field silently wrapped to `70000 mod 65536 = 4464`; feeding that buffer straight back into this crate's own `read_pdu` fails with `ReadUserVariable` (a downstream parse desync, not a crash - the writer produced a genuinely self-inconsistent PDU with no error at any point). Reproduced 3/3 in-tree unit-test runs, then 2/2 more via the standalone `repro/` built against a fresh clone of pristine upstream `df2b9e42` (5/5 total) |

## Status

**Confirmed** - deterministic, pure-logic bug with no layout/seed/timing dependency; reproduces
5/5 runs (3 in-tree unit-test runs + 2 standalone `repro/` runs against a fresh clone of pristine
upstream `df2b9e42`), same truncated length and same downstream `read_pdu` failure every time.
Debug and release builds both show it identically (the truncating cast is ordinary `as` behavior
on both, not something instrumentation-only reveals).

## Class

`logic` (a public encoding API silently produces a malformed, self-inconsistent wire message and
reports success - no panic, no crash, no memory-safety implication, but a real protocol
correctness violation that a genuine DICOM SCU sending, say, a large SAML assertion for user
identity negotiation would hit silently, with the failure only surfacing much later - and far less
diagnosably - on the receiving end's parse).

## Root cause

`write_chunk_u16` builds an arbitrary-length `Vec<u8>` chunk via a caller-supplied closure, then
writes a `u16` big-endian length prefix computed as `data.len() as u16` before writing the full
`data` unconditionally. Rust's `as` cast between integer types performs a silent truncating modulo
reduction rather than saturating or erroring - so any chunk whose built content exceeds 65,535
bytes gets a wrong (wrapped-around) length prefix while the *entire* actual data still gets written
after it. Nothing downstream in `write_chunk_u16` or any of its ~20 callers checks `data.len()`
against `u16::MAX` before this cast. `write_chunk_u32` (used for the outer, whole-PDU-body length,
and for the P-Data-TF PDU's per-fragment length) has the structurally identical pattern with `u32`,
though a `u32::MAX` (~4 GiB) chunk is a much less practically reachable size for this crate's data
model - included in the fix defensively for consistency, not because a reachable trigger was found
for it this session (see "What's not done" below).

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a length check (`ensure!(data.len() <= u16::MAX as usize, ChunkTooLargeSnafu { .. })` /
the equivalent `u32::MAX` check for `write_chunk_u32`) immediately before each truncating cast, and
a new `WriteChunkError::ChunkTooLarge { length, max }` variant so callers get a clear, actionable
error instead of a corrupted wire message. Because the fix lives in the two shared helper
functions rather than at each of the ~20 individual call sites, it protects all of them uniformly
by construction - a future new call site automatically inherits the protection, rather than
needing its own copy of the check.

Verified: re-ran the exact `UserIdentity` repro against the patched code - `write_pdu` now returns
`Err(WriteChunk { .. ChunkTooLarge { length: 70000, max: 65535 } .. })` instead of `Ok(())`, and no
malformed bytes are written at all. Because the fix is in the shared helper rather than per-call-
site, it was not necessary to separately re-test every one of the ~20 other callers with an
oversized input to satisfy CONFIRMATION.md section 7's "vary the input" bar in the usual sense -
the mechanism itself is what changed, not a per-input special case, so every caller of
`write_chunk_u16`/`write_chunk_u32` shares the identical guarded code path post-fix; this was
confirmed by reading (not just assuming) that every one of the ~20 call sites found by
`grep -n "write_chunk_u16\|write_chunk_u32"` goes through these same two functions with no
alternate, unguarded path to the wire.

- Project test suite (`cargo test -p dicom-ul --lib`): **19 passed, 1 failed (this candidate's own
  temporary repro test, reverted before writing this record) -> 20 passed, 0 failed** after the
  fix, with the temporary test included. No other test in the `dicom-ul` crate's existing suite
  (association handshake tests, PDU round-trip tests, etc.) was affected by the added validation -
  none of them construct a chunk anywhere near the new size limits.

## What's not done / open questions

- Only `UserIdentity`'s `primary_field` was used as the concrete trigger; the identical
  vulnerability existed in all ~20 other `write_chunk_u16` callers (Application Context Name,
  Abstract/Transfer Syntax Names, SOP Class UID, Implementation Version Name, the `secondary_field`
  sibling, `UserVariableItem::Unknown`'s raw data, etc.) and in the sole `write_chunk_u32` caller
  chain (the whole-PDU-body wrapper and P-Data-TF's per-fragment length) - all are fixed by the
  same two-function patch, but only the `UserIdentity` path was independently exercised end-to-end
  with a real oversized input this session.
- The `write_chunk_u32` path's `ChunkTooLarge` guard is defensive/consistency-driven, not backed
  by a demonstrated reachable trigger this session - constructing a >4 GiB in-memory `Vec<u8>` to
  prove it live was out of scope for this candidate's time budget (and impractical on typical
  hardware for a chunk that has to be fully built in memory first via the closure-based API
  regardless).
- Whether any *caller* of `write_pdu` in this codebase (e.g. `ul/src/association/*.rs`'s
  higher-level association state machine, `storescu`/other CLI tools) already validates field
  sizes before constructing a `Pdu`, making this specific gap unreachable in practice through those
  particular call paths, was not audited - only the `write_pdu`/`write_chunk_u16` public API
  surface itself was checked, which is reachable directly by any caller of this crate regardless of
  what the bundled CLI tools happen to do.
- `ul/src/pdu/reader.rs` (the decode side) was checked in the same session (candidate 032, not
  reproducible) - this candidate closes the loop by checking the encode side for the same class of
  bug, per `SKILL.md`'s "critical pitfalls" #7 ("the same underlying bug can live in more than one
  place").
