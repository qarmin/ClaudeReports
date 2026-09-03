# storescu-async-pdata-write-unwrap-panic

`storescu/src/store_async.rs`'s `send_file` (used whenever a C-STORE-RQ's command and data set
are too large to fit in a single PDU together, so the data set is sent as a separate P-Data
fragment) panics on any write failure:

```rust
let mut pdata = scu.send_pdata(pc_selected.id);
pdata.write_all(&object_data).await.unwrap();
//.whatever_context("Failed to send C-STORE-RQ P-Data")?;
```

The commented-out `.whatever_context(...)?` line right next to the `.unwrap()` shows this was a
half-finished error-handling change - the intended graceful path was written but never actually
wired in, and the `.unwrap()` above it was left in place instead.

This crate's own **synchronous** sibling, `storescu/src/store_sync.rs`, already handles the
exact same operation correctly: `pdata.write_all(&object_data).context(WriteIOSnafu)?;` - the
`WriteIOSnafu` error variant this candidate's fix reuses was already defined and already used
for this purpose elsewhere in this same crate, just not applied to the async code path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-062): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/scu-tools-unwraps` (commit `e082188c`), which bundles this fix together with a separate movescu bug in one commit | Confirmed unmerged; split into two candidates per this session's one-bug-per-candidate discipline (the movescu half is a separate, not-yet-investigated `IDEAS.md` backlog entry) |
| Compared against `storescu/src/store_sync.rs`'s already-correct handling of the identical operation | Static code reading | Confirmed `WriteIOSnafu` already exists and is already used correctly in the sync sibling - this is a "fix applied to one branch but not its sibling" bug, the same shape as candidates 006/007 and 050/051 |
| **[REBUILT per CONFIRMATION.md section 11]** Real, two-real-participant, real-loopback-TCP reproduction (`repro/run_e2e.sh`): runs the actual `dicom-storescu` binary (`-c 1`, forcing the async code path) against `repro/fake_scp` (a minimal but genuine SCP on `dicom-ul`'s own real server association API), sending a genuine ~20MB DICOM file (`repro/make_big_file`) | Real association negotiation, real C-STORE-RQ receipt, then `fake_scp` abruptly drops the real TCP connection before reading the P-Data payload - a real OS-level connection reset on the real socket `storescu` is writing to | Real `dicom-storescu` process panics exactly as predicted: `called Result::unwrap() on an Err value: Os { code: 104, kind: ConnectionReset, ... }` at `storescu/src/store_async.rs:112:53`. A smaller (64KB) file was tried first and did *not* trigger the panic - the OS silently absorbed the write into its send buffer, and the failure only surfaced later as an already-handled error when awaiting the response; the ~20MB size was needed for the write itself to genuinely fail. Reproduced deterministically: verified first directly against the live project tree's own built binaries, then again via `repro/`'s `just test` against a fresh clone (2 independent real end-to-end runs total; see `backtrace.txt`) |
| (superseded, kept for context) Earlier session's single-process reproduction using a custom in-process `AsyncWrite` mock that always returns `Err`, never running the real binary or a real socket | `cargo run` | Demonstrated the same conclusion but never ran the real `storescu` binary, a real peer, or a real network failure - retired in favor of the real two-participant repro above once `CONFIRMATION.md` section 11 (added mid-session, after this candidate's original write-up) required it |

## Status

**Confirmed** - deterministic, reproduces every run once the payload size is large enough for a
real connection reset to surface as a failed write rather than being absorbed by the OS send
buffer (see the real end-to-end repro's own note on file size). Verified via a real end-to-end
(real binary + real dicom-ul-based peer, real loopback TCP, real OS-level connection reset)
reproduction.

**Fully reproducible: Yes** - `repro/` runs the actual `dicom-storescu` binary against a genuine
SCP built on `dicom-ul`'s own real association API, communicating over a real loopback TCP
socket, with a real OS-level connection failure (see `CONFIRMATION.md` section 11).

## Class

`crash` (panic instead of a handled error, on an entirely ordinary network failure condition -
a TCP connection dropping mid C-STORE transfer - in a client tool's async code path).

## Root cause

Incomplete refactor: the async P-Data write's error-handling was clearly intended to mirror the
sync sibling's already-correct `.context(WriteIOSnafu)?` handling (visible as a commented-out,
never-activated line directly beneath the `.unwrap()`), but the change was never actually
applied, leaving the working `.unwrap()` panic in place.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replace `.unwrap()` (and delete the stale commented-out line) with the same
`.context(WriteIOSnafu)?` pattern already used by the sync sibling for the identical operation:

```rust
pdata.write_all(&object_data).await.context(WriteIOSnafu)?;
```

See `patch.diff` for the exact diff (scoped to `storescu/src/store_async.rs` only - adds
`WriteIOSnafu` to this file's existing `use crate::{...}` import list, since it was already
defined but not yet imported here).

- `cargo build -p dicom-storescu --all-features`: builds cleanly with the fix applied (both
  against the live tree and against the patched pristine clone via `repro/`'s
  `reproduce-after` step).
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced. `dicom-storescu` has no `--lib` target (binary-only crate,
  like candidates 042/057/058/062), so a build-success check is this candidate's equivalent
  "after" verification, following the same established pattern as those candidates.
- `repro/` (`just test`): the real `dicom-storescu` binary panics before the patch (real
  connection reset from `fake_scp`); reports a clean `WriteIO` error after.

## What's not done / open questions

While investigating this candidate, the exact same "unwrap a `TransferSyntaxRegistry.get(ts)`"-
and "index `&data[0]` on a response PDU without checking for empty"-shaped patterns were noticed
in several *other* SCU tools in this workspace (`findscu`, `movescu`, and `storescu`'s own
response handling) - logged as fresh `IDEAS.md` backlog entries for a future candidate rather
than folded into this one, to keep this candidate scoped to the single bug it confirms and
fixes.
