# storescp-unknown-transfer-syntax-unwrap-panic

`storescp`'s C-STORE receiving loop (`store_sync.rs`, identically in `store_async.rs`) looks up
the negotiated presentation context's transfer syntax UID in the global registry and unwraps the
result unconditionally:

```rust
let ts = &presentation_context.transfer_syntax;
let obj = InMemDicomObject::read_dataset_with_ts(
    instance_buffer.as_slice(),
    TransferSyntaxRegistry.get(ts).unwrap(),
)
```

If `ts` is ever a transfer syntax UID this crate's registry doesn't recognize, `.get(ts)` returns
`None` and `.unwrap()` panics the whole server process for that connection.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056/057): `git log --all --grep="#768"` | Searched all refs for the same external report already seen fixing sibling issues | Found commit `866a1942` (see candidate 057, which adopted the other half of the same commit) fixing this exact `.unwrap()` alongside the path-traversal issue, on an unmerged branch |
| `git merge-base --is-ancestor 866a1942 df2b9e42` | N/A | Returned false - confirmed this fix was never merged, so the panic is live in the exact commit this session audits |
| Direct reproduction of the exact vulnerable line, using the public `dicom-transfer-syntax-registry` API (`TransferSyntaxRegistry.get(ts).unwrap()` with an unrecognized UID) | Minimal, hand-built path-dep binary, no network/association code involved | Confirmed: panics every time with `called 'Option::unwrap()' on a 'None' value`, exactly matching the mechanism described. Reproduced deterministically: in-tree checks during investigation + 2 independent `just test` runs against a fresh clone of pristine upstream `df2b9e42` (see `backtrace.txt`) |
| Static analysis of `dicom-ul`'s association negotiation (`ServerAssociationOptions::choose_ts`, `is_supported`/`is_supported_with_repo`), to determine whether an accepted presentation context's `transfer_syntax` field could concretely end up holding an unrecognized UID in `storescp`'s specific usage | Code reading of `ul/src/association/server.rs` | For `storescp`'s exact configuration (registers every non-unsupported entry from `TransferSyntaxRegistry`, or exactly the two hardcoded uncompressed UIDs), `choose_ts` only accepts a proposed transfer syntax when `is_supported_with_repo` (which itself queries `TransferSyntaxRegistry`) confirms it - and a *rejected* presentation context is given a hardcoded, always-valid fallback UID (`"1.2.840.10008.1.2"`) rather than an arbitrary one. No concrete gap in this validation was found by reading alone; see "What's not done" |

## Status

**DOWNGRADED to Inconclusive** (was previously marked Confirmed; revised per an updated
`CONFIRMATION.md` section 11, added mid-session, which requires that any finding about how one
participant reacts to another's input be demonstrated with a real second participant over a real
socket, not a hand-built stand-in). This candidate's only repro called the vulnerable library
function (`TransferSyntaxRegistry.get(ts).unwrap()`) directly with a manually-chosen unrecognized
UID string - it never ran the real `storescp` binary, never ran a real DICOM peer, and never
demonstrated that a real negotiated `presentation_context.transfer_syntax` can actually hold an
unrecognized value.

An attempt was made this session to instead confirm reachability via a real two-participant setup
(`storescp`'s own listener negotiating against a real peer). That attempt instead **found evidence
against reachability**: `storescp`'s `ServerAssociationOptions` setup (`store_sync.rs`/
`store_async.rs`, around the `.accept_any()`/`.with_transfer_syntax(...)` calls) only ever offers
transfer syntax UIDs already present in `TransferSyntaxRegistry` (either the two hardcoded
uncompressed UIDs, or every registry entry, depending on configuration) - `dicom-ul`'s negotiation
(`choose_ts`) can only select from what was offered, so a real peer negotiating against a real
`storescp` listener cannot cause `presentation_context.transfer_syntax` to end up holding a UID
the registry doesn't recognize. No genuine end-to-end reproduction was built as a result - the
panic mechanism itself remains real (confirmed via direct function-level testing), but its
reachability through the real negotiation path this crate actually uses now appears **actively
contradicted**, not just unconfirmed. See "What's not done" for what would need to change for this
to become reachable.

## Class

n/a (Inconclusive - see Status)

## Root cause

`storescp` trusts that `presentation_context.transfer_syntax` - populated during association
negotiation, a process implemented entirely in a *different* crate (`dicom-ul`) - will always be a
transfer syntax UID `dicom-transfer-syntax-registry`'s global registry recognizes, and never
checks this itself before calling `.unwrap()`. This dependency is real (as the static analysis
above shows, for `storescp`'s specific configuration) but is an implicit cross-crate invariant,
not something the type system or a local check enforces - it would silently break if `dicom-ul`'s
negotiation logic ever changed, if `storescp`'s own configuration changed (e.g. a future code path
constructing `ServerAssociationOptions` without registering every transfer syntax first), or if
any other, not-yet-identified path exists to populate that field without going through
`choose_ts`'s validation.

## The fix

Replaced the `.unwrap()` with `.whatever_context(...)?`, matching this project's own established
error-handling convention (`snafu`'s `Whatever` error type, used pervasively throughout
`storescp`) - an unrecognized transfer syntax now cleanly fails that specific incoming instance
with a descriptive error, instead of terminating the process:

```rust
let obj = InMemDicomObject::read_dataset_with_ts(
    instance_buffer.as_slice(),
    TransferSyntaxRegistry
        .get(ts)
        .whatever_context("unknown transfer syntax")?,
)
```

Applied identically to both `store_sync.rs` and `store_async.rs`. See `patch.diff` for the exact
diff.

- `cargo build -p dicom-storescp --all-features`: builds cleanly with the fix applied (both
  `store_sync.rs` and `store_async.rs`).
- `cargo test -p dicom-storescp --all-features`: **1 passed, 0 failed** (the crate's only existing
  test, a CLI-argument-parsing smoke test - unaffected by this fix).
- Full workspace (`cargo test --workspace --lib`; `dicom-storescp` is a `--bin`-only crate with no
  `--lib` target): **351 passed / 0 failed / 2 ignored** before and after - unchanged, confirming
  this fix has zero effect outside `storescp`.

## What's not done / open questions

- **Resume note for a future session**: the panic mechanism (`.unwrap()` on an unrecognized
  transfer syntax) is real and still present in `storescp/src/store_sync.rs` and
  `store_async.rs`, and the drafted fix (`patch.diff`, unchanged from the original
  investigation) is still believed correct as defensive hardening - but reachability through
  this crate's own real negotiation setup now appears actively blocked (see Status). Before
  re-confirming this as a live bug, a future session should either: (a) find a genuine gap in
  `dicom-ul`'s `choose_ts`/negotiation logic itself that lets an unoffered/unrecognized UID
  through despite `storescp`'s own offer list, or (b) find some other `storescp` code path/
  configuration (e.g. `--promiscuous`, or a future change to how `ServerAssociationOptions` is
  built) that doesn't restrict its offer list to registry-known UIDs. Absent either of those, this
  specific line is defense-in-depth for a currently-unreachable condition, not a live
  vulnerability, and should not be re-marked Confirmed without a real two-participant reproduction.
- The fix itself is harmless and still worth keeping if this code is ever touched again (replacing
  `.unwrap()` with a handled error costs nothing and guards against future changes to the
  negotiation logic), but is no longer being claimed as fixing a demonstrated live bug this
  session.
