# encoding-pixeldatawriter-offset-table-not-cumulative

`encoding/src/adapters.rs`'s `PixelDataWriter::encode` default implementation (used by any
transfer syntax adapter that doesn't override `encode()` itself - the standard way to produce
encapsulated multi-frame pixel data) computes each Basic Offset Table (BOT) entry using only the
*current* frame's own encoded length:

```rust
for frame in 0..frames {
    let mut frame_data = Vec::new();
    out = self.encode_frame(src, frame, options.clone(), &mut frame_data)?;
    offset_table.push(frame_data.len() as u32 + 8 * (frame + 1));
    dst.push(frame_data);
}
```

Per PS3.5 Annex A.4, Basic Offset Table entries must be **cumulative** byte offsets from the
start of the first fragment to the start of each frame's own first fragment - `entry[0]` must
always be `0`, and `entry[n]` is the sum of `(fragment length + 8-byte item header)` for every
*preceding* fragment. The code above instead computes, for each frame, "this frame's own length
plus `8 * (frame index + 1)`" - which is neither the correct cumulative value nor even internally
consistent between frames (it doesn't account for any *other* frame's length at all). For a
2-frame image with frame lengths 4 and 6 bytes, the correct BOT is `[0, 12]`; this computation
produces `[12, 22]`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-073): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/pixel-data-writer-offset-table` (commit `a6fd7c49`) correcting this exact computation | Confirmed unmerged |
| Reused this crate's own existing test double (`encoding/src/adapters.rs`'s `tests::TestDataObject`, which implements `PixelDataObject`) plus a minimal custom `PixelDataWriter` (`IdentityPixelDataWriter`, writing frame 0 as 4 bytes and frame 1 as 6 bytes - distinct lengths make a wrong cumulative computation obvious) to exercise the default `encode()` method directly, added as a temporary in-tree `#[test]` (kept as a permanent regression test in `patch.diff`) | `cargo test -p dicom-encoding basic_offset_table_is_cumulative_repro` | Produces `offset_table == [12, 22]` instead of the correct `[0, 12]` |
| Standalone `repro/` (path-dependency on a pristine clone of `df2b9e42`, via `just test`), using the public `dicom_encoding::adapters` API with a self-contained minimal `PixelDataObject`/`PixelDataWriter` pair | `cargo run` | Reproduces the same incorrect offsets deterministically; see `backtrace.txt` |

## Status

**Confirmed** - deterministic, reproduces every run (pure arithmetic on fixed, known frame
lengths, no timing/adversarial-input dependency).

## Class

`logic` (silent data corruption in generated DICOM output - any multi-frame image encoded via
this default `encode()` method into a transfer syntax that stores a Basic Offset Table gets a
wrong, non-cumulative table baked into the file, breaking any downstream reader that uses the
BOT for random-access frame seeking, per the standard's own definition of what the table means).

## Root cause

The computation `frame_data.len() as u32 + 8 * (frame + 1)` appears to have conflated "this
frame's own encoded size plus its own item header" with "the cumulative position of this frame
within the whole fragment stream" - it never actually accumulates anything across iterations of
the loop, so each entry only reflects the current frame's own length and index, not the
preceding frames' contributions the standard requires.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Track a running `cumulative_offset`, push its *current* value before encoding advances it (so
`entry[0]` is always `0`), then increment it by the just-encoded frame's length plus the 8-byte
item header overhead, using saturating arithmetic to avoid any theoretical overflow for
extremely large cumulative sizes:

```rust
let mut cumulative_offset: u32 = 0;
for frame in 0..frames {
    let mut frame_data = Vec::new();
    out = self.encode_frame(src, frame, options.clone(), &mut frame_data)?;
    offset_table.push(cumulative_offset);
    cumulative_offset = cumulative_offset
        .saturating_add(frame_data.len() as u32)
        .saturating_add(8);
    dst.push(frame_data);
}
```

See `patch.diff` for the exact diff (scoped to `encoding/src/adapters.rs`; includes the
regression test kept in the `tests` module).

- `cargo build -p dicom-encoding`: builds cleanly with the fix applied.
- `cargo test -p dicom-encoding --lib` (against the patched pristine clone): **41 passed / 0
  failed** (40 pre-existing + this candidate's own new regression test), confirming zero
  regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): panics on the wrong-offsets assertion before the patch; produces the
  correct cumulative table after.
