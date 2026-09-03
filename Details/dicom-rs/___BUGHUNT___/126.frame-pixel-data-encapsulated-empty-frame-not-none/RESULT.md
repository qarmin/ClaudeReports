# frame-pixel-data-encapsulated-empty-frame-not-none

Follow-up to candidate 124's own "what's not done" note: while adversarially stress-testing
`encoding/src/adapters.rs`'s `PixelDataObject::frame_pixel_data` encapsulated-pixel-data branch
(the offset-table-walking "Other cases of multi-frame objects" arm), candidate 124 observed - but
did not investigate further - that a `frame` whose own Basic Offset Table entry can never be
reached by the real accumulated fragment offsets returns `Some(0 bytes)` instead of `None`,
contradicting the function's own documented contract ("Returns `None` if there is no such frame").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the function's source (`encoding/src/adapters.rs:216-248`) to understand the exact mechanism candidate 124 had already surfaced via its own repro output | - | Root-caused: the loop only sets `frame_data`/returns non-empty when `offset >= base_offset` is satisfied for at least one fragment; if `base_offset` (taken directly from the object's own offset-table entry, attacker/malformed-input controlled) is never reached, the loop still runs to completion and returns `Some(Cow::Owned(vec![]))` - an empty-but-`Some` result, not `None` |
| Built a real end-to-end repro (`repro/src/main.rs`) via the actual public production entry point - `FileDicomObject<InMemDicomObject>`'s `PixelDataObject` impl (`object/src/lib.rs:1315`, which does not override `frame_pixel_data`, so it uses the shared default trait method under test unmodified) - constructing a genuine encapsulated Pixel Data element (`PixelFragmentSequence`, 3 fragments across 2 frames so the offset-table-walking branch is taken, not the 1:1 shortcut) with a legitimate offset table and, separately, one with a bogus unreachable second entry | debug/release build, 4 cases (2 frames x 2 object variants) | Well-formed offset table `[0, 36]`: frame 0 = `Some(20 bytes)`, frame 1 = `Some(10 bytes)` (both correct). Malformed offset table `[0, 1000000]` (same fragments/frame count, only the offset table entry is corrupted): frame 0 = `Some(30 bytes)` (a separate, secondary correctness quirk - frame 0 wrongly absorbs frame 1's fragment too, since there is no valid `next_offset` to stop it at; not fixed by this candidate, see "What's not done" below), frame 1 = `Some(0 bytes)` - the bug, reproduced 3/3 runs (deterministic, pure function, no randomness) |
| Applied a bounded fix (track whether any fragment was actually included; return `None` if not) and re-ran the same repro against the patched tree | same repro binary, no changes | Malformed-table frame 1 now correctly returns `None`; well-formed-table frame 0/1 results unchanged (`Some(20)`/`Some(10)`), confirming the fix doesn't disturb the legitimate multi-fragment-per-frame path |

## Status

**Confirmed.** Reproduces deterministically every run (pure function over caller-supplied data, no
I/O/randomness/timing dependency) - verified 3/3 runs both via the in-tree repro and via a
standalone fresh-clone `just test` run (see `justfile`).

**Fully reproducible**: **Yes**. The repro drives the real, unmodified public entry point
(`PixelDataObject::frame_pixel_data`, as implemented by `FileDicomObject<InMemDicomObject>`, the
production type real callers use) with a genuinely constructed `InMemDicomObject` built entirely
through the crate's own public `put_element`/`DataElement::new`/`FileMetaTableBuilder` API - not a
hand-implemented stand-in trait object. This is a single external artifact (an in-memory DICOM
object whose own encapsulated Pixel Data / Basic Offset Table is internally inconsistent, the same
shape a malformed or adversarial real file's Pixel Data Sequence could produce) with no second
participant to fake, so `CONFIRMATION.md` section 11 does not apply.

## Class

logic

## Root cause

`frame_pixel_data`'s offset-table-walking branch (`encoding/src/adapters.rs`, the `Some(number_of_fragments)`
arm reached whenever fragments and frames aren't 1:1) looks up `base_offset` directly from the
object's own Basic Offset Table entry for the requested frame, then walks fragments accumulating
`offset`, including a fragment's bytes into `frame_data` only once `offset >= base_offset`. If
`base_offset` is larger than any offset the real fragments can ever reach (e.g. a corrupted/lying
offset-table entry, or more generally any offset-table value that doesn't correspond to a real
fragment boundary), the loop runs to completion having included nothing, and the function still
returns `Some(Cow::Owned(frame_data))` with `frame_data` empty - it never distinguishes "this frame
legitimately has 0 bytes" (not physically possible for real encoded pixel data; every DICOM Item
in Pixel Data always carries at least some encoded bytes) from "no fragment ever matched this
frame's declared start". This directly contradicts the function's own doc comment: "Returns `None`
if there is no such frame or there is no pixel data at all."

## Introduced by

61e43337 "[encoding] Add function to access frame pixeldata" (2026-02-09) - the commit that
introduced `frame_pixel_data` (and this exact offset-table-walking loop) in its current shape;
`git log -S` on the loop's own `let mut frame_data = Vec::new();` line confirms no later commit
touched this specific control flow before the fix applied here.

## The fix

Added a `included_any: bool` flag, set `true` only when a fragment is actually appended to
`frame_data` inside the loop; after the loop, if nothing was ever included, return `None` instead
of falling through to `Some(Cow::Owned(frame_data))` with an empty vector. This is the minimal
change that restores the documented "no such frame -> `None`" contract without altering the
legitimate multi-fragment-per-frame path at all (verified: the well-formed-offset-table case in
`repro/` produces byte-for-byte identical results before and after the fix - `Some(20 bytes)` for
frame 0, `Some(10 bytes)` for frame 1 either way).

- Project test suite: `cargo test --workspace --exclude dcms --lib`: 351 passed / 0 failed / 2
  ignored both before and after the fix (identical), no regressions.

## What's not done / open questions

- The **separate** quirk observed alongside this one (frame 0, in the malformed-offset-table case,
  wrongly absorbs 30 bytes instead of the correct 20 - silently pulling in frame 1's fragment too,
  because there's no valid `next_offset` boundary to stop it once the table's second entry is
  corrupted) was deliberately **not fixed** in this candidate: it's a distinct failure mode (wrong,
  non-empty data returned for a frame whose *own* offset table entry is fine, caused entirely by
  the *next* frame's entry being bad) that would need its own separate reasoning about what the
  "correct" fallback behavior should be (stop at the last known-good boundary? propagate `None` for
  the affected frame too? both are defensible and it's a product decision, not a bug fix this
  candidate is confident enough to make unilaterally). Left as a fresh `IDEAS.md` backlog item.
- Whether a similarly-malformed offset table is reachable through the *parser*'s own validation
  (i.e. would `dicom-parser`/`dicom-object`'s normal file-reading path ever hand a `PixelSequence`
  with an internally-inconsistent offset table to application code, or does something upstream
  already reject this) was not checked - this candidate constructs the object directly via the
  public in-memory API, which is a legitimate way a caller can reach this state regardless (e.g.
  programmatic construction, or a lenient/partial parse), but whether a normal `open_file`-style
  read of a real malformed file also reaches it wasn't independently verified this session.
