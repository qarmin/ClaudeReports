# storescp-transfer-syntax-registry-unwrap-safe

`storescp/src/store_sync.rs:235` (and the identical pattern in `store_async.rs`) calls
`TransferSyntaxRegistry.get(ts).unwrap()`, where `ts` is `&presentation_context.transfer_syntax`
taken from `association.presentation_contexts()` - a value negotiated from the remote peer's
(attacker-controlled) association request. An `.unwrap()` on a registry lookup keyed by
attacker-influenced data is exactly HEURISTICS.md's top pattern (".unwrap()/.expect()/panic! on
data derived from external input"), and `storescp`/`storescp/src/store_async.rs` (the bundled DICOM
C-STORE SCP - a network-facing server processing untrusted associations) had not been checked in
either bug-hunt session before this one.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: traced every code path that can produce a `PresentationContextNegotiated.transfer_syntax` value in `ul/src/association/server.rs`'s association-request handling (`ServerAssociationOptions`'s negotiation logic), to determine whether every possible value is guaranteed to be a `TransferSyntaxRegistry`-resolvable UID before `storescp` ever sees it | Static code reading across `ul/src/association/server.rs` and `storescp/src/store_{sync,async}.rs`, no build needed | Every negotiated presentation context's `transfer_syntax` field is set from exactly one of three sources, all traced to be registry-safe: (1) the accepted case, `self.choose_ts(pc.transfer_syntaxes)`, which (in the non-empty-allowlist branch) requires `is_supported(ts)` - a direct `TransferSyntaxRegistry` lookup - to be true before a candidate is returned, and (in the empty-allowlist branch) delegates to `choose_supported`, which filters via the identical `is_supported` check; (2) the "Abstract Syntax Not Supported" rejection fallback, hardcoded to the literal `"1.2.840.10008.1.2"` (Implicit VR Little Endian - guaranteed always-registered, per this crate's own doctest: `assert!(is_supported("1.2.840.10008.1.2"))`); (3) the "Transfer Syntaxes Not Supported" rejection fallback, the identical hardcoded literal. No fourth path exists - `storescp` never sees a `transfer_syntax` value that bypassed all three |

## Status

**Not reproducible** - ruled out by tracing every value-producing path for the field this
`.unwrap()` depends on back to its source; all three are registry-safe by construction.

## Class

n/a

## Root cause

Not applicable (no bug) - unlike this session's candidate `034` (which found a real gap in a
*different* part of the same negotiation/reception pipeline), this specific `.unwrap()` is
protected by the negotiation logic already filtering every accepted-or-rejected transfer syntax
value down to registry-known UIDs before it's ever exposed via `presentation_contexts()`. Note,
however, that `storescp` does **not** check `presentation_context.reason` before using its
`transfer_syntax` to decode incoming P-Data (i.e., it doesn't distinguish "this presentation
context was actually accepted" from "this presentation context was rejected, and its
`transfer_syntax` field is just the safe placeholder") - a client that ignores a rejection and
sends data under a rejected presentation context ID anyway would have that data decoded as Implicit
VR Little Endian (the placeholder), which is a *protocol-conformance* gap (accepting data the
server itself said it wouldn't accept) but not a crash/panic risk, since the placeholder is always
a valid, decodable transfer syntax - out of scope as a "bug" for this candidate's specific
`.unwrap()`-safety question, but worth flagging as a separate, lower-severity-shaped observation
for a future session focused on protocol-conformance rather than crash-safety.

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

- The protocol-conformance observation above (data sent under a rejected presentation context ID
  still gets decoded rather than rejected) was not investigated as its own candidate - it's a
  logic/spec-conformance question, not a crash risk, and HEURISTICS.md scopes this skill toward
  confirmed reproducible bugs; noted here as a lead for a future session or for
  `code-audit-structured` instead.
- `storescp/src/store_async.rs`'s identical pattern was confirmed to share the exact same
  negotiation code path (both sync and async servers construct `PresentationContextNegotiated`
  through the same `ServerAssociationOptions` logic in `server.rs`) but was not independently
  re-derived line-by-line beyond confirming the pattern match.
- The rest of `storescp/src/main.rs` and `storescp/src/transfer.rs`, and the other CLI tools
  (`echoscu`, `findscu`, `movescu`, `scpproxy`, `toimage`, `fromimage`, `dump`) remain largely
  unchecked this session - only this one `.unwrap()` was investigated, prompted by
  HEURISTICS.md's top-ranked pattern.
