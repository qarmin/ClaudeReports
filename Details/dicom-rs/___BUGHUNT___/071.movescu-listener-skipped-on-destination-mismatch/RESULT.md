# movescu-listener-skipped-on-destination-mismatch

`movescu/src/main.rs`'s `main` function has an early-return branch:

```rust
if Some(app.move_destination.clone()) != Some(app.calling_ae_title.clone()) {
    run_move_scu(app.clone(), progress).unwrap_or_else(|err| {
        error!("{}", snafu::Report::from_error(err));
        std::process::exit(-2);
    });
    return;
}
```

If the `--move-destination` AE title differs from the `--calling-ae-title` (movescu's own
identity when opening the C-MOVE association), this branch runs only `run_move_scu` - the
function that sends the C-MOVE-RQ - and returns immediately, **without ever starting the local
async SCP listener** (`run_async`, which binds the `-p`/`--port` TCP port and accepts the
remote SCP's incoming C-STORE sub-operations). The normal (non-early-return) path always starts
`run_async` first, then `run_move_scu`.

`--move-destination` and `--calling-ae-title` are two independently-configurable AE titles with
different purposes (one is the identity used to open the query/move association; the other is
the AE title the remote SCP is instructed to route the resulting images *to*), and both default
to the same value (`"STORE-SCP"`) only coincidentally - it's an entirely ordinary, arguably the
*more correct* configuration for them to differ (e.g. registering the receiving AE under a
distinct title in the remote PACS's AE table). Whenever they do differ, and the operator's
intent is for this exact movescu process to be the one receiving the images (the common case for
this tool), the listener never starts and every incoming C-STORE connection attempt from the
remote SCP fails outright.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-070): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/movescu-listener-early-return` (commit `956b7b0d`) removing exactly this early-return block | Confirmed unmerged |
| Read `run_async`'s body to confirm it's the one responsible for binding the TCP listener (`tokio::net::TcpListener::bind`) and accepting the C-STORE connection, and that the early-return path is the *only* code path that skips calling it | Static code reading | Confirmed: every other path spawns `run_async` as a task before calling `run_move_scu` |
| Live end-to-end reproduction (`repro/check_listener.py`): start a fake "MOVE SCP" (a plain Python TCP listener that accepts the connection and never replies, so `run_move_scu` blocks instead of exiting immediately), launch the real `dicom-movescu` binary with `--calling-ae-title CALLER --move-destination DEST` (deliberately different) and `-p 22000`, then probe whether port 22000 is open while movescu is still running | Ran against both a pristine clone (`just reproduce-before`) and the same clone after applying the fix (`just reproduce-after`) | **Before**: port 22000 never opens (`Connection refused`) - the listener was skipped, exactly as predicted. **After**: port 22000 opens immediately - the listener starts correctly |

## Status

**Confirmed** - deterministic, reproduces every run (the branch condition depends only on two
CLI string arguments being unequal, with no timing/adversarial-input dependency); reproduced via
a full live process-level test against the actual compiled binary, not just static reading.

## Class

`logic` (silent functional failure - not a crash - for an entirely ordinary CLI configuration:
every incoming C-STORE connection attempting to reach this movescu instance is refused, with no
error message pointing at the actual cause, whenever the destination and calling AE titles
differ).

## Root cause

The early-return block appears to have been written under the assumption that
`move_destination != calling_ae_title` means "the destination is some other machine, so this
process doesn't need to receive anything" - but nothing about these two independently-configured
AE title strings actually implies that; they commonly differ even when the destination is this
exact movescu instance.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Remove the early-return block entirely, so the listener is always started (matching the
already-correct behavior of the "titles match" path) regardless of how the two AE titles compare:

```diff
-    if Some(app.move_destination.clone()) != Some(app.calling_ae_title.clone()) {
-        run_move_scu(app.clone(), progress).unwrap_or_else(|err| {
-            error!("{}", snafu::Report::from_error(err));
-            std::process::exit(-2);
-        });
-        return;
-    }
-
     let runtime = tokio::runtime::Builder::new_multi_thread()
```

See `patch.diff` for the exact diff (scoped to `movescu/src/main.rs` only).

- `cargo build -p dicom-movescu`: builds cleanly with the fix applied (both against the live
  tree and against the patched pristine clone via `repro/`'s `build-after` step).
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced. `dicom-movescu` has no `--lib` target (binary-only crate), so
  this candidate's live end-to-end script is the equivalent "after" verification.
- `repro/check_listener.py` (`just test`): reports the listener port closed before the patch,
  open after.
