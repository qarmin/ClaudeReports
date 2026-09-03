# ul-pdata-strict-mode-not-honored

`ClientAssociationOptions`/`ServerAssociationOptions` (`ul/src/association/{client,server}.rs`)
expose a `.strict(bool)` builder option, defaulting to `true`, documented as controlling "whether
to receive PDUs in strict mode" - and it genuinely does, for every PDU received during association
setup (ASSOCIATE-RQ/AC/RJ, RELEASE-RQ/RP, ABORT): `read_pdu_from_wire`/`read_pdu_from_wire_async`
(`ul/src/association/mod.rs`) thread `self.strict` all the way into `pdu::read_pdu`'s own `strict`
parameter, which rejects any PDU whose declared `pdu_length` exceeds the negotiated
`max_pdu_length` immediately, before buffering more data. But `receive_pdata()` (also in
`association/mod.rs`), used for every P-Data-TF PDU during the actual bulk data-transfer phase
(where most of an association's untrusted network bytes actually flow), constructs its
`PDataReader` without ever consulting `self.strict` at all - `ul/src/association/pdata.rs`'s
`PDataReader::read`/`poll_read` hardcode `read_pdu(&mut buf, max_pdu_length, false)`, unconditionally
disabling the exact same enforcement for this one PDU type. Found by reading `ul/src/association/
pdata.rs`'s `PDataReader::read` right after candidates `032`/`033` (a systematic follow-up per
HEURISTICS.md's "check the sibling side" - `032`/`033` were the raw `pdu::reader`/`pdu::writer`
level, this is the layer above, `association/`, not yet checked).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reading: traced every `read_pdu`/`read_pdu_from_wire` call site's `strict` argument across `ul/src/association/{mod,client,server,pdata}.rs` | Static code reading, no build needed | Every association-handshake PDU read site passes `self.strict` through; both P-Data-TF read sites (`pdata.rs`'s sync `Read` impl and its `#[cfg(feature = "async")]` `AsyncRead` impl) hardcode `false`, and `PDataReader` (constructed by `receive_pdata()`, a default method on the `Association`/`AsyncAssociation` traits) has no `strict` field or configuration surface of any kind |
| Hand-built boundary input (a legitimately well-formed, fully-written `Pdu::PData` with one 50,000-byte PDV payload, encoded via the crate's own public `write_pdu`), debug build, `cargo test` (temporary unit test added to `ul/src/association/pdata.rs`, reverted before this record was written) | `PDataReader::new(stream, max_pdu_length: MINIMUM_PDU_SIZE /* 1018 */, &mut read_buffer)` then `Read::read()` on it, via the public `dicom_ul::association::PDataReader` API | `read()` returned `Ok(50000)` - the reader fully accepted and delivered a PDV payload ~49x larger than the `max_pdu_length` passed to the very same constructor, with no error, no truncation, no rejection. Reproduced 3/3 in-tree unit-test runs, then 2/2 more via the standalone `repro/` built against a fresh clone of pristine upstream `df2b9e42` (5/5 total) |
| Attempted fix: thread a `strict: bool` field through `PDataReader` (builder method `.strict(bool)`, default `true` to match the crate's existing `ClientAssociationOptions`/`ServerAssociationOptions` default), add `fn strict(&self) -> bool` to the `Association` trait (implemented by `ClientAssociation`/`AsyncClientAssociation`/`ServerAssociation`/`AsyncServerAssociation`, all of which already carry a private `strict: bool` field used for handshake PDUs), and wire `receive_pdata()` to call `.strict(self.strict())`. Full patch preserved for reference at `attempted-fix-not-applied.diff` in this folder (not applied - see Status/root cause below) | `cargo build -p dicom-ul --all-features` (compiled cleanly), then `cargo test -p dicom-ul --all-features` | **Broke 2 of the crate's own pre-existing tests**: `test_read_large_pdata_and_finish` and its async sibling `test_async_read_large_pdata_and_finish`, both of which construct a P-Data-TF PDU whose PDV payload is exactly `MINIMUM_PDU_SIZE` (1018) bytes, using that same 1018 value as `max_pdu_length`. With strict enforcement wired in, both now fail with `PduTooLarge { pdu_length: 1024, max_pdu_length: 1018 }` - see "Root cause" below for why |

## Status

**Confirmed** - deterministic, pure-logic bug with no layout/seed/timing dependency; reproduces
5/5 runs (3 in-tree unit-test runs + 2 standalone `repro/` runs against a fresh clone of pristine
upstream `df2b9e42`), identical accepted byte count every time. **No bounded fix was applied this
session** - see below.

## Class

`logic` (a documented, opt-in configuration setting - `strict` mode, `true` by default - silently
has no effect for one specific, high-traffic PDU type, contradicting its own doc comment: "whether
to receive PDUs in strict mode" makes no distinction between PDU types).

## Root cause

Two independent things are both real and both matter here:

1. **The inconsistency itself**: `PDataReader` (`ul/src/association/pdata.rs`) was written as a
   fully separate code path from `read_pdu_from_wire`/`read_pdu_from_wire_async`
   (`ul/src/association/mod.rs`, used for every other PDU type) and never received the `strict`
   plumbing those functions have - not a copy/paste drift like earlier candidates in this
   workspace, but a code path that appears to have been written without `strict` in mind at all
   (no field, no parameter, no `TODO`).
2. **Why a naive fix is unsafe**: the wire's `pdu_length` field (what `strict` mode actually
   bounds against `max_pdu_length`, per `pdu::reader::read_pdu`) covers the *entire* P-Data-TF PDU
   body - which includes not just the PDV payload bytes but also the PDV item's own 4-byte
   Item-length field, 1-byte Presentation-context-ID, and 1-byte Message-control-header (6 bytes of
   overhead per `ul/src/association/pdata.rs`'s own `setup_pdata_header`: `pdu_len = data_len + 4 +
   2`). A PDV payload sized at exactly `max_pdu_length` bytes - a completely natural construction,
   and exactly what the crate's *own* `test_read_large_pdata_and_finish` test does - therefore
   produces a `pdu_length` of `max_pdu_length + 6`, which a spec-literal `strict` check correctly
   (per PS3.8's definition of `pdu_length`) but disruptively rejects. This 6-byte discrepancy
   between "PDV payload size" and "wire `pdu_length`" is not visible anywhere in the public API -
   nothing tells a caller of `send_pdata`/`PDataWriter` that PDV payloads must be sized at
   `max_pdu_length - 6` (or less) to stay within a peer's negotiated `strict` ceiling on the
   receiving end. This is very likely *why* the original implementation hardcoded `false` here:
   turning strict mode on for P-Data without first fixing this off-by-a-few-bytes trap would
   immediately reject a class of well-behaved senders (including, demonstrably, this crate's own
   test suite), trading one gap for a new, more immediately disruptive one.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

**No bounded fix applied this session.** The minimal, obvious fix (thread `self.strict` through to
`PDataReader`, matching every other PDU type) was implemented, built, and tested - the full diff is
preserved at `attempted-fix-not-applied.diff` in this folder for reference - but it surfaces the
compatibility break described above under the crate's own default settings (`strict: true` by
default on both `ClientAssociationOptions` and `ServerAssociationOptions`), which is a real risk
for real callers, not just this candidate's own test construction. Per `SKILL.md`'s candidate loop
step 6, this is a case where "a fix requiring an architecture change is a valid 'no bounded fix
found' outcome" - closing this gap correctly needs one of:

- Fixing (or at minimum, clearly documenting) the ~6-byte overhead so a `PDataWriter`/`send_pdata`
  caller can size PDV payloads to stay within a peer's negotiated `max_pdu_length` when it enforces
  `strict` mode - a decision about which layer (byte accounting in `PDataWriter`, or the meaning of
  `max_pdu_length` itself) should absorb that overhead, which only the maintainers can make; or
- Giving `PDataReader`/`receive_pdata()` its own independently-defaulted `strict` setting (e.g.
  defaulting to `false` regardless of the association's own `strict` default) with clear
  documentation explaining the asymmetry is intentional - which contradicts the `.strict(bool)`
  option's own current doc comment and would need that comment rewritten to avoid re-promising
  uniform enforcement.

Both are legitimate design decisions, not bugs with a single obviously-correct patch - hence no
`patch.diff` for this candidate. The tree was left clean (all attempted changes reverted via `git
checkout`) before this record was written.

- Project test suite: **N/A for a "before/after" comparison** in the usual sense, since no fix was
  kept - but the attempted fix's own test run is recorded above as a finding in its own right: it
  turned 2 previously-passing tests (`test_read_large_pdata_and_finish`,
  `test_async_read_large_pdata_and_finish`) into failures, which is exactly the signal that
  stopped this candidate from shipping a patch.

## What's not done / open questions

- Whether any *other* part of this codebase (e.g. `storescp`/other bundled CLI tools, or downstream
  users of `dicom-ul`) already works around this gap by validating P-Data sizes some other way was
  not checked - only the library's own public API surface was audited.
- The exact provenance of the 6-byte PDV-item overhead (item-length field + presentation-context-id
  + message-control-header) was traced precisely (see `setup_pdata_header`), but whether this is
  the *only* discrepancy between "payload size" and "wire pdu_length" for P-Data specifically, or
  whether other PDU types have a similar (currently masked, because they're not size-sensitive in
  the same way) gap was not exhaustively re-derived - only P-Data was in scope this candidate,
  since it's the only PDU type using `PDataReader`.
- Whether the crate's maintainers consider the current hardcoded-lenient P-Data behavior
  intentional (there is indirect textual evidence for this: `Association::local_max_pdu_length`'s
  own doc comment already says "The current implementation is not required to fail and/or abort
  the association if a larger PDU is received") was not confirmed by reading any commit history or
  issue tracker for this specific hardcoded `false` - only inferred from the code and doc comments
  present in this checkout.
