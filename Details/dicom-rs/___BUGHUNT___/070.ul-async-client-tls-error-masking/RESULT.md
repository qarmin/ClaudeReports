# ul-async-client-tls-error-masking

`ul/src/association/client.rs`'s `ClientAssociationOptions::establish_impl_async` (behind the
`async-tls` feature) has a fallback path for when reading the association response PDU fails: it
tries to determine whether the failure happened because the peer unexpectedly responded with a
TLS handshake instead of a plain PDU, by feeding the raw bytes already read into a
`tokio_rustls::LazyConfigAcceptor` and checking whether *that* fails with
`rustls::Error::InappropriateMessage` (the specific error rustls produces when it's given bytes
that are a valid TLS record type but not a ClientHello - i.e., a legitimate one-sided TLS
mismatch):

```rust
Err(err) => {
    if let Ok(rustls::Error::InappropriateMessage { .. }) = err.downcast::<rustls::Error>() {
        error!("Received TLS response to non-TLS request!");
        return super::TlsNotSupportedSnafu.fail()
    }
    // Recieved a valid TLS message, means the server expects TLS
    return super::TlsNotSupportedSnafu.fail()
}
```

The comment ("Received a valid TLS message, means the server expects TLS") describes a belief
that doesn't match the code above it: if the `if let` doesn't match, that means the bytes did
**not** produce an `InappropriateMessage` - which could be *any* other outcome (the downcast
failing entirely, or a completely different, unrelated `rustls::Error` variant) - not
specifically "a valid TLS message." Regardless of which of these actually occurred, the code
unconditionally returns the same `TlsNotSupportedSnafu`, discarding `e` - the original,
real error from the actual attempt to read the association response PDU - and replacing it with
a specific, likely-misleading diagnosis for what could be a completely unrelated connection
failure (a dropped connection, garbled/corrupted data, a firewall reset, etc.).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-069): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/ul-tls-error-mapping` (commit `3d056f8f`) fixing this exact fallback | Confirmed unmerged |
| Standalone script (outside `___BUGHUNT___`, reused as `repro/`'s content) feeding ordinary garbage bytes (`[0x01..0x08]`, not a deliberately crafted TLS handshake or any other special edge case) directly to a real `tokio_rustls::LazyConfigAcceptor`, then downcasting the resulting error to `rustls::Error` | `cargo run` (async, via `#[tokio::main]`) | The downcast **succeeds** (it really is a `rustls::Error`), but the variant is `InvalidMessage(InvalidContentType)` - not `InappropriateMessage`. Confirms the buggy fallback branch is taken for entirely ordinary malformed/garbled responses, not just contrived inputs |
| Traced `e`'s scope to confirm it's still available (not moved) at the point the fix needs to return it | Static code reading | `e` (`resp`'s error) and `err` (the `LazyConfigAcceptor`'s own error) are distinct bindings; `e` remains valid throughout the whole `match` arm |
| **[REBUILT per CONFIRMATION.md section 11, after an initial downgrade]** Real, two-real-participant, real-loopback-TCP reproduction (`repro/`): a real `dicom-ul` async client (`ClientAssociationOptions::establish_async`, `async-tls` feature enabled) connects to a genuinely separate tokio task acting as a garbled peer | The peer task accepts the real TCP connection, reads the real A-ASSOCIATE-RQ the client sends, then writes real ordinary garbage bytes back over the real socket before closing it | Real client's `establish_async()` call genuinely returns `Err` with the message "TLS not enabled, but peer seems to be sending TLS data" - misreporting an entirely ordinary garbled-peer failure as a TLS issue, discarding the real underlying error. Reproduced deterministically: verified first directly against the live project tree's own `dicom-ul` crate, then again via `repro/`'s `just test` against a fresh clone (2 independent real end-to-end runs total; see `backtrace.txt`) |

## Status

**Re-CONFIRMED via a real end-to-end reproduction.** This candidate was briefly downgraded to
Inconclusive mid-session (its first repro fed hand-picked garbage bytes to a real
`LazyConfigAcceptor` in-process, never running the real async client) per an updated
`CONFIRMATION.md` section 11 - a genuine two-real-participant repro was subsequently built (see
above) and the finding is Confirmed again.

**Fully reproducible: Yes** - `repro/` uses `dicom-ul`'s own real, public async client API
against a genuinely separate tokio task, communicating over a real loopback TCP socket (see
`CONFIRMATION.md` section 11).

## Class

`logic` (misleading/incorrect error propagation - not a crash - masking the real cause of an
association failure with a specific but usually-wrong diagnosis, in the async TLS-aware client
connection path).

## Root cause

The `if let ... { A } else { B }` structure was written as if the `else` branch could only be
reached by "a valid TLS message" (per its own comment), but that's not what "the `if let`
pattern didn't match" actually means - it also covers downcast failure and every *other*
`rustls::Error` variant, none of which indicate the peer is TLS-only. The `else` branch's
`TlsNotSupportedSnafu` was left in from an earlier, less complete version of this check (visible
in a commented-out duplicate `if let` right above the final line in the pre-fix code),
suggesting an incomplete refactor rather than a deliberate design choice.

## The fix

Only take the `TlsNotSupportedSnafu` path when the downcast genuinely confirms
`InappropriateMessage`; otherwise propagate the original error `e`:

```rust
if let Ok(rustls::Error::InappropriateMessage { .. }) = err.downcast::<rustls::Error>() {
    error!("Received TLS response to non-TLS request!");
    return super::TlsNotSupportedSnafu.fail();
}
// downcast failed or error is not InappropriateMessage -
// not a TLS indication, propagate the original error
return Err(e);
```

See `patch.diff` for the exact diff (scoped to `ul/src/association/client.rs` only; also
removes a stale, already-dead commented-out duplicate of the same check).

- `cargo build -p dicom-ul --features async-tls`: builds cleanly with the fix applied (both
  against the live tree and against the patched pristine clone via `repro/`'s
  `reproduce-after` step).
- `cargo test -p dicom-ul --features async-tls --lib` (against the live tree, fix applied):
  **33 passed / 0 failed** - unchanged from before the fix, confirming zero regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.

## What's not done / open questions

None remaining - a full live end-to-end reproduction was built and verified (see above).
