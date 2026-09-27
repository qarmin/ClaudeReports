# storescu-empty-pdata-response-panic

`storescu/src/store_sync.rs` and `store_async.rs` (identically) index the first Presentation
Data Value of the SCP's C-STORE-RQ response PDU unconditionally:

```rust
match rsp_pdu {
    Pdu::PData { data } => {
        let data_value = &data[0];
        ...
```

Same bug class and mechanism as already-confirmed candidate 062 (`echoscu`): `Pdu::PData { data:
Vec<PDataValue> }` can legitimately be empty for a P-DATA-TF PDU declaring PDU-length 0 - a
completely valid wire encoding, not malformed/truncated - so any SCP that responds to a
C-STORE-RQ with such a PDU crashes `storescu` instead of getting a handled error. This bug is
fixed in two files at once (both `storescu` response-handling siblings) since they're identical
code, matching this session's established precedent for bundling structurally identical fixes in
one candidate (e.g. candidate 057's `store_sync.rs`+`store_async.rs` patch, candidate 056's five
call sites fixed together).

Notably, this workspace's *other* SCU tools already guard against exactly this: `findscu/src/
main.rs:316` and `movescu/src/main.rs:404` both have an explicit `if data.is_empty() { error!(..);
break; }` check immediately before the same `&data[0]` pattern - checked and confirmed NOT
vulnerable. Only `storescu`'s two response-handling files (and, until candidate 062, `echoscu`)
were missing this guard.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| While reviewing the diff context around candidate 063 (a different bug in the same two `storescu` files), noticed the identical `&data[0]` pattern already confirmed vulnerable in candidate 062's `echoscu` | Grepped `data\[0\]` across every SCU-style binary in the workspace: `storescu/src/store_sync.rs:126`, `storescu/src/store_async.rs:125`, `findscu/src/main.rs:316`, `movescu/src/main.rs:404` | Found 4 occurrences total |
| Read the surrounding context of each of the 4 occurrences to check for an existing guard | Static code reading | `findscu` and `movescu` already guard with `if data.is_empty() { ...; break; }` right before the index - confirmed NOT vulnerable, not written up. Only the two `storescu` files lack any such guard |
| Standalone `repro/` (reusing candidate 062's exact technique: parse a real 6-byte wire-level P-DATA-TF PDU with PDU-length 0 through the public `dicom_ul::pdu::read_pdu`, then replicate the vulnerable `&data[0]` pattern) | `cargo run` | Reproduces the panic deterministically; see `backtrace.txt` |

## Status

**Confirmed** - deterministic, reproduces every run, no fixed seed needed (a fixed 6-byte input
that is valid per the wire format's own rules).

## Class

`crash` (panic / index-out-of-bounds on a client tool, triggered by a completely valid but
minimal response PDU from the SCP `storescu` connects to).

## Root cause

Same as candidate 062: the response handling assumed the responding SCP always includes at
least one Presentation Data Value in its C-STORE response, with nothing in the wire format or in
`dicom_ul`'s parser enforcing that assumption.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replace the direct index with `.first()`, propagating a handled error via a new `EmptyResponse`
error variant (added to `storescu/src/main.rs`'s existing `Error` enum, following its established
style) instead of panicking - applied identically to both `store_sync.rs` and `store_async.rs`:

```rust
let data_value = data.first().context(EmptyResponseSnafu)?;
```

See `patch.diff` for the exact diff (scoped to `storescu/src/main.rs`, `store_sync.rs`, and
`store_async.rs`).

- `cargo build -p dicom-storescu --all-features`: builds cleanly with the fix applied (both
  against the live tree and against the patched pristine clone via `repro/`'s
  `reproduce-after` step).
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced. `dicom-storescu` has no `--lib` target (binary-only crate),
  so a build-success check is this candidate's equivalent "after" verification.
- `repro/` (`just test`): panics before the patch; the patched `dicom-storescu` crate builds
  cleanly after.
