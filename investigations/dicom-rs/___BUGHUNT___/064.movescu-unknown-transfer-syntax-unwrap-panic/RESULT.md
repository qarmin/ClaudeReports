# movescu-unknown-transfer-syntax-unwrap-panic

`movescu/src/store_async.rs`'s `run_store_async` (movescu briefly acts as a temporary SCP to
receive the C-STORE sub-operations that make up a C-MOVE, exactly like candidate 058's
`storescp` finding) unwraps `TransferSyntaxRegistry.get(ts)` unconditionally, where `ts` is
`&presentation_context.transfer_syntax` - the negotiated presentation context's transfer syntax
- panicking the whole receiving loop if it's ever a UID the registry doesn't recognize:

```rust
let obj = InMemDicomObject::read_dataset_with_ts(
    instance_buffer.as_slice(),
    TransferSyntaxRegistry.get(ts).unwrap(),
)
```

This is the second half of the same unmerged commit as candidate 063 (`origin/fix/scu-tools-
unwraps`), split off per this session's one-bug-per-candidate discipline (a structurally
different bug - a `None`-unwrap panic on a negotiated transfer syntax, not an I/O write
failure), and it is the same exact bug shape and mechanism as already-confirmed candidate 058
(`storescp-unknown-transfer-syntax-unwrap-panic`), just in a different binary.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-063): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/scu-tools-unwraps` (commit `e082188c`), bundling this fix with candidate 063's storescu fix in one commit | Confirmed unmerged; split into two candidates (this one covers the movescu half) |
| Compared against already-confirmed candidate 058 (`storescp`, identical `TransferSyntaxRegistry.get(ts).unwrap()` shape on a negotiated transfer syntax) | Read 058's `RESULT.md` and reused its exact repro/justfile pattern | Confirmed this is the same bug class in a sibling network-facing binary, not a duplicate (different crate, different call site) |
| Standalone `repro/` (reusing 058's established technique: since `movescu` has no library target, replicate the exact vulnerable line directly against the public `dicom-transfer-syntax-registry` API) | `cargo run` | Reproduces the panic deterministically: `called Option::unwrap() on a None value`; see `backtrace.txt` |

## Status

**DOWNGRADED to Inconclusive** (was previously marked Confirmed; revised per an updated
`CONFIRMATION.md` section 11, added mid-session, requiring a real second participant over a real
socket for findings about how one participant reacts to another's input). This candidate's only
repro called `TransferSyntaxRegistry.get(ts).unwrap()` directly with a manually-chosen
unrecognized UID - it never ran the real `movescu` binary or a real DICOM peer.

Checked for real reachability the same way as candidate 058 (which this candidate mirrors
exactly): `movescu/src/store_async.rs`'s `ServerAssociationOptions` setup (around its own
`.accept_any()`/`.with_transfer_syntax(...)` calls, identical pattern to `storescp`) only ever
offers transfer syntax UIDs already present in `TransferSyntaxRegistry`. `dicom-ul`'s negotiation
can only select from what was offered, so a real peer negotiating a C-STORE sub-operation against
`movescu`'s temporary listener cannot cause `presentation_context.transfer_syntax` to hold an
unrecognized UID. No genuine end-to-end reproduction was built as a result, for the same reason as
058 - this candidate's reachability claim is now actively contradicted by `movescu`'s own real
setup, not just unconfirmed.

## Class

n/a (Inconclusive - see Status)

## Root cause

Same as candidate 058: the code assumes a negotiated presentation context's transfer syntax is
always one the registry recognizes, with no local check at the point of use.

## The fix

Replace `.unwrap()` with `.whatever_context("unknown transfer syntax")?`, propagating a handled
error instead of panicking - identical fix shape to candidate 058:

```rust
TransferSyntaxRegistry
    .get(ts)
    .whatever_context("unknown transfer syntax")?,
```

See `patch.diff` for the exact diff (scoped to `movescu/src/store_async.rs` only).

- `cargo build -p dicom-movescu --all-features`: builds cleanly with the fix applied (both
  against the live tree and against the patched pristine clone via `repro/`'s
  `reproduce-after` step).
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced. `dicom-movescu` has no `--lib` target (binary-only crate, like
  candidates 042/057/058/062/063), so a build-success check is this candidate's equivalent
  "after" verification.
- `repro/` (`just test`): panics before the patch; the patched `dicom-movescu` crate builds
  cleanly after.

## What's not done / open questions

**Resume note for a future session**: same as candidate 058 - the panic mechanism is real, the
drafted fix (`patch.diff`) is still believed correct as defensive hardening, but reachability
through `movescu`'s own real negotiation setup now appears actively blocked. A future session
would need to find a genuine gap in `dicom-ul`'s negotiation logic itself, or a `movescu`
configuration path that doesn't restrict its offer list to registry-known UIDs, before
re-confirming this as live.
