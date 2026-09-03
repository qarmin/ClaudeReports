# storescu-command-data-set-type-inconsistent

`storescu/src/main.rs`'s `store_req_command` (builds the C-STORE-RQ command element set) sets
`CommandDataSetType` to `0x0000`:

```rust
DataElement::new(tags::COMMAND_DATA_SET_TYPE, VR::US, dicom_value!(U16, [0x0000])),
```

C-STORE-RQ always carries an accompanying data set (the object being stored). Per PS3.7 Table
6.3-1, the `CommandDataSetType` field's only specifically-defined sentinel is `0x0101`, meaning
"no Data Set present"; any other value technically signals "a Data Set is present." However,
`storescu`'s own sibling tools in this exact codebase - `findscu` (`findscu/src/main.rs:435`,
for C-FIND-RQ, which also carries an Identifier data set) and `movescu`
(`movescu/src/main.rs:507`, for C-MOVE-RQ, same situation) - both use `0x0001` for the identical
case, not `0x0000`. `storescu` is the one outlier, using a value its own sibling tools avoid.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-074): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/storescu-command-dataset-type` (commit `0aa9f6f0`) changing `storescu`'s value from `0x0000` to `0x0001` | Confirmed unmerged |
| Grepped every use of `COMMAND_DATA_SET_TYPE` across the workspace to establish this codebase's own convention for "a Data Set is present" | `grep -rn "COMMAND_DATA_SET_TYPE"` across all SCU/SCP binaries | `echoscu`/`storescp`'s C-ECHO/C-STORE-RSP messages (which carry NO data set) correctly use `0x0101`; `findscu` and `movescu` (which DO carry a data set, the same situation as `storescu`'s C-STORE-RQ) both use `0x0001`; only `storescu` uses `0x0000` for this case - a clear sibling inconsistency, the same shape as several other candidates this session (006/007, 050/051, 063) |
| Temporary in-tree `#[test]` in `storescu/src/main.rs` (kept as a permanent regression test in `patch.diff`): call the private `store_req_command` function and check the resulting `CommandDataSetType` value | `cargo test -p dicom-storescu store_req_command_data_set_type_repro` | Confirms `storescu` currently produces `0x0000` |
| Standalone `repro/` (since `store_req_command` is private in a binary-only crate, replicates the exact vulnerable value directly against the public `dicom-object` API, comparing against the sibling tools' own value) | `cargo run` | Reproduces the same inconsistency deterministically; see `backtrace.txt` |

## Status

**Confirmed** - deterministic (a fixed constant value in the command construction, no
input-dependent behavior).

## Class

`logic` (DICOM protocol interoperability defect - not a crash - producing a C-STORE-RQ command
whose `CommandDataSetType` value diverges from this codebase's own established convention for
the same situation; per the unmerged commit's own stated motivation, some strict SCP
implementations validate this field and may reject C-STORE requests using `0x0000`).

## Root cause

`store_req_command`'s `CommandDataSetType` value appears to have been set independently of
`findscu`/`movescu`'s equivalent command-construction code, without checking for consistency
with those sibling tools already present in the same workspace.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Change the value to `0x0001`, matching `findscu`'s and `movescu`'s own convention for a command
that carries an accompanying data set:

```rust
// data set type: 0x0001 = Data Set present (any value != 0x0101)
DataElement::new(tags::COMMAND_DATA_SET_TYPE, VR::US, dicom_value!(U16, [0x0001])),
```

See `patch.diff` for the exact diff (scoped to `storescu/src/main.rs`; includes the regression
test kept in the `tests` module).

- `cargo build -p dicom-storescu`: builds cleanly with the fix applied.
- `cargo test -p dicom-storescu` (against the patched pristine clone): **2 passed / 0 failed**
  (1 pre-existing + this candidate's own new regression test), confirming zero regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): panics on the value-mismatch assertion before the patch; the patched
  `dicom-storescu` crate builds and its own tests pass after.
