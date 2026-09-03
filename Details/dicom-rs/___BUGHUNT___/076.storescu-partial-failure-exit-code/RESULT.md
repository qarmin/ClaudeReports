# storescu-partial-failure-exit-code

`storescu/src/store_sync.rs`'s `inner` (and its async twin in `store_async.rs`) iterate over
every file being stored, calling `send_file` for each one. `send_file`'s response handling logs
an error for any C-STORE-RSP status code outside a small set of known-good/warning values, but
(when `fail_first` is `false`, the default) never records anywhere that a failure happened - it
just continues to the next file:

```rust
_ => {
    error!("Failed to store instance `{}` (status code {:04X}H)", storage_sop_instance_uid, status);
    if fail_first {
        let _ = scu.abort();
        std::process::exit(-2);
    }
    // (else: nothing recorded, loop continues)
}
```

After the loop, `inner` unconditionally returns `Ok(())`. `storescu`'s `main()` treats any `Err`
returned from `run()`/`run_async()` as a reason to `std::process::exit(-2)`, and `Ok(())` as an
implicit successful exit (status `0`) - so **a batch transfer where every single file fails to
store (for any reason other than the two special-cased fatal paths) still exits with status
`0`**, indistinguishable from complete success to any calling script or orchestration tool that
checks the process exit code (a very standard way to detect failures in shell scripts, CI
pipelines, etc.).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-075): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/storescu-partial-failure-exit-code` (commit `c99c9d35`) threading a `had_failure` flag through both the sync and async paths | Confirmed unmerged |
| Traced `run`/`run_async`'s callers in `main.rs` to confirm the exit-code mechanism: `run(app).unwrap_or_else(|e| { ...; std::process::exit(-2); })` (and the async equivalent) | Static code reading | Confirmed: `inner`'s return value is the sole determinant of storescu's own process exit code for this path; `Ok(())` unconditionally means exit `0` |
| **[REBUILT per CONFIRMATION.md section 11]** Real, two-real-participant, real-loopback-TCP reproduction (`repro/run_e2e.sh`): runs the actual `dicom-storescu` binary against `repro/fake_scp` (a minimal but genuine SCP on `dicom-ul`'s own real server association API, reusing `storescp`'s own real C-STORE-RSP-building code verbatim, just with `Status = 0xA700`) | Real association negotiation, real C-STORE-RQ receipt, then a real, legitimately-encoded C-STORE-RSP with a genuine DICOM failure status sent back over the real socket; checks the real `dicom-storescu` process's own exit code | Real `dicom-storescu` process exits **0** (success) despite the C-STORE genuinely failing, exactly as predicted. Reproduced deterministically: verified first directly against the live project tree's own built binaries, then again via `repro/`'s `just test` against a fresh clone (2 independent real end-to-end runs total; see `backtrace.txt`) |
| (superseded, kept for context) Earlier session's single-process reproduction replicating the vulnerable control-flow logic in a standalone script, never running the real binary or a real socket | `cargo run` | Demonstrated the same conclusion but never ran the real `storescu` binary or a real network exchange - retired in favor of the real two-participant repro above once `CONFIRMATION.md` section 11 (added mid-session, after this candidate's original write-up) required it |

## Status

**Confirmed** - deterministic (pure control-flow logic keyed off status codes, no
timing/adversarial-input dependency). Verified via a real end-to-end (real binary + real
dicom-ul-based peer, real loopback TCP) reproduction.

**Fully reproducible: Yes** - `repro/` runs the actual `dicom-storescu` binary against a genuine
SCP built on `dicom-ul`'s own real association API, communicating over a real loopback TCP
socket, checking the real process exit code (see `CONFIRMATION.md` section 11).

## Class

`logic` (silent automation/scripting hazard - not a crash - any tool or script that relies on
`storescu`'s exit code to detect a failed transfer cannot do so for this entire class of
failure, since the process reports success regardless).

## Root cause

The status-handling `match` in `send_file` was written to *log* every failure but never
introduced any mechanism to *propagate* that a failure occurred back to the caller of `inner`
unless `fail_first` was also set (in which case the process exits immediately via
`std::process::exit(-2)` from within the match arm itself, bypassing the normal `Result`
propagation entirely). The non-`fail_first` path was simply never given an equivalent way to
surface an accumulated failure at the end of the batch.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Thread a `had_failure: &mut bool` parameter through `send_file` and `inner` (both the sync and
async versions), set to `true` in the same two match arms that already log an error, and check
it after the loop to return a new `PartialFailure` error variant instead of unconditional `Ok`:

```rust
let mut had_failure = false;
for (message_id, mut file) in (1..).zip(d_files) {
    // ...
    scu = send_file(scu, file, message_id, pbx.as_ref(), verbose, fail_first, &mut had_failure)?;
}
scu.release()...;
if had_failure {
    PartialFailureSnafu.fail()
} else {
    Ok(())
}
```

See `patch.diff` for the exact diff (scoped to `storescu/src/main.rs` - adding the
`PartialFailure` error variant - and both `store_sync.rs` and `store_async.rs`).

- `cargo build -p dicom-storescu --all-features`: builds cleanly with the fix applied (both
  against the live tree and against the patched pristine clone via `repro/`'s
  `reproduce-after` step).
- `cargo test -p dicom-storescu --all-features` (against the patched pristine clone): **1 passed
  / 0 failed**, confirming zero regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): the real `dicom-storescu` process exits 0 before the patch (bug
  reproduced); exits non-zero with a clear "partial failure" message after.

## What's not done / open questions

None remaining - a full live end-to-end reproduction was built and verified (see above).
