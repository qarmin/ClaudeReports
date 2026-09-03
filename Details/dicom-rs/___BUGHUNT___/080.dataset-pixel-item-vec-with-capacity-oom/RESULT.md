# Untrusted pixel-data-item length pre-allocates a Vec before any bytes are read (OOM abort)

From `IDEAS.md`'s git-archaeology backlog: `origin/fix/oom-untrusted-vec-capacity` (ca4e045b)
touches `object/src/meta.rs` and `parser/src/dataset/read.rs`, seeded as "likely a separate
preallocation site" from the already-completed candidate 004 (`parser/src/stateful/decode.rs`).
Confirmed distinct: the two `Vec::with_capacity(len)` calls in `parser/src/dataset/read.rs`
(`DataSetReader`'s encapsulated-pixel-data item reading) are a different file and a different
mechanism from 004's typed-value readers, and (for the offset table specifically) an additional,
independent defect beyond what the branch's own commit message describes.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Read the branch's diff (`git show origin/fix/oom-untrusted-vec-capacity`) against current `master` (`df2b9e42`). Confirmed both `object/src/meta.rs` `try_reserve_exact` sites and the two `parser/src/dataset/read.rs` `Vec::with_capacity(len)` sites (offset table + item value, inside `DataSetReader`'s encapsulated-pixel-data handling) are still present, unmodified, on current master. | `object/src/meta.rs` already uses `try_reserve_exact` (fallible allocation, returns `Err` gracefully on failure - not an abort) for every site the branch touches; not a live abort bug. `parser/src/dataset/read.rs` uses plain infallible `Vec::with_capacity(len)`, the abort-prone pattern. Additionally found the offset table's `Vec::with_capacity(len)` allocates a `Vec<u32>` but `len` is a *byte* count - so it requests `len * 4` bytes, a 4x amplification of the attacker's own declared length, not described by the branch's commit message. |
| Standalone repro (release build, `dicom_object::from_reader`, the real public top-level entry point for parsing an untrusted DICOM byte stream) under `ulimit -v 500000` (500 MB cap, representative of a small container) | `___BUGHUNT___/080.../repro`. Two ~236/244-byte crafted files: a minimal valid file-meta group (Explicit VR LE) + a main data set with an encapsulated Pixel Data element (tag 7FE0,0010, VR OB, undefined length) followed by (a) a Basic Offset Table item declaring a 200,000,000-byte length with no actual bytes following, or (b) an empty offset table item followed by a plain fragment item declaring a ~4.29 GiB length with no actual bytes following. | Both scenarios aborted the process with `memory allocation of N bytes failed` / `SIGABRT` (core dumped) - scenario (a) requested exactly `200,000,000 * 4 = 800,000,000` bytes (confirming the 4x amplification), scenario (b) requested exactly the declared `4,294,967,280` bytes. |
| Repeat-run check | 3 runs each, release build, `ulimit -v 500000`, no fixed seed | 3/3 for both scenarios - fully deterministic, no randomness or memory-layout dependence (the abort is driven purely by the crafted file's own declared length fields). |

## Status

**Confirmed** - deterministic, 3/3 runs each for both scenarios, release build, `ulimit -v 500000`
to make the abort fast/deterministic rather than dependent on this machine's actual free RAM (same
methodology as candidate 004).

**Fully reproducible**: **Yes** - the repro calls `dicom_object::from_reader`, the real public
top-level entry point a real application would use to parse an untrusted DICOM byte stream (e.g.
one just received over the network or an upload), on a fully self-contained, hand-crafted
malformed file. Per `CONFIRMATION.md` section 11, a malformed file handed to a parser's real public
`open_file`/`from_reader` entry point *is* the genuine external input - there is no second
participant being impersonated here.

## Class

`crash` / `security` (uncatchable process abort via Rust's default global-allocator-failure
handler, not a panic or `Result::Err` any caller could intercept - triggered by a 236/244-byte,
entirely attacker-controlled input with no actual malformed structure beyond an oversized declared
item length; the same DoS class as candidate 004, but in the `DataSetReader` dataset-token
iterator used for every encapsulated/compressed pixel data element, rather than in the typed
per-element value readers).

## Root cause

`parser/src/dataset/read.rs`'s `DataSetReader::next()`, when iterating the items of an
encapsulated Pixel Data sequence, reads each item's own declared byte length (`len`, a plain `u32`
straight off the wire, attacker-controlled, up to `0xFFFF_FFFE`) and pre-allocates a buffer of
exactly that size *before* attempting to read any of the item's actual bytes:

- Item value (any fragment after the first): `Vec::with_capacity(len)` for a `Vec<u8>` - requests
  `len` bytes directly from the untrusted length.
- Basic Offset Table (always the first item): `Vec::with_capacity(len)` for a `Vec<u32>` - `len`
  is a *byte* count, but `Vec::with_capacity` on a `Vec<u32>` takes an *element* count, so this
  actually requests `len * 4` bytes: a second, independent bug that amplifies the same untrusted
  length fourfold beyond what even the attacker directly declared.

Both use `Vec::with_capacity`, Rust's default infallible allocation path: if the allocator can't
satisfy the request, `handle_alloc_error` aborts the whole process immediately - there is no bound
between the declared length and how much data has actually arrived in the underlying reader.

The subsequent reads (`read_u32_to_vec`/`read_to_vec`, both already existing) are not what's fixed
here for the item-value path - `read_to_vec` (via `std::io::copy` with a `.take(length)` reader)
already grows its target `Vec<u8>` incrementally as bytes actually arrive, so removing the
pre-allocation in front of it fully closes that call site. The offset-table path is only *partly*
closed by the same change: `read_u32_to_vec` calls `StatefulDecoder::read_u32`
(`parser/src/stateful/decode.rs`), which itself does an unguarded `vec.resize(base + n, 0)` sized
directly from the (undivided-by-4, but still untrusted and still large) `n = len >> 2` - the exact
same call site already identified, but not applied/merged, by candidate 004's own fix for
`parser/src/stateful/decode.rs`. Removing the outer `Vec::with_capacity(len)` therefore removes
the 4x amplification and closes the bug for moderately large declared lengths (demonstrated here
with 200,000,000), but a sufficiently large declared length (demonstrated here with the u32-max
range, ~4.29 GiB) still aborts one layer down, in `read_u32`, independently of this fix.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Removed the pre-allocation at both `parser/src/dataset/read.rs` call sites, replacing
`Vec::with_capacity(len)` with `Vec::new()` (matching `origin/fix/oom-untrusted-vec-capacity`'s own
change to this file) - each vector now grows only as bytes actually arrive from the underlying
reader (`read_u32_to_vec`/`read_to_vec`, both unchanged), so a small file with a bogus, huge
declared item length can no longer force a large up-front allocation from these two call sites
before any of it is confirmed to exist in the stream. See `patch.diff`
(`parser/src/dataset/read.rs`, one file, two hunks).

Before/after re-run of `repro/` (see `output.txt`): both scenarios (200,000,000-byte offset table,
~4.29 GiB fragment) aborted with `SIGABRT` before the fix and exited `0` with a graceful
error/result after it, 3/3 runs each.

- Project test suite: `cargo test -p dicom-parser --lib` - 49 passed; 0 failed, before and after
  (identical). `cargo test -p dicom-object --lib` - 75 passed; 0 failed, before and after
  (identical). `cargo test --workspace` - 537 passed; 0 failed, before and after (identical).

`object/src/meta.rs`'s half of the same branch was checked separately and found **not** to be a
live bug: every site the branch touches there already uses `try_reserve_exact` (a fallible
allocation call that returns `Err` gracefully instead of aborting the process on failure) -
confirmed by constructing a 28-byte file-meta group declaring a ~4.29 GiB "Private Information"
(0002,0102) element and observing `FileMetaTable::from_reader` return
`Err("Could not allocate memory")` rather than aborting, 500 MB `ulimit -v`, no crash. No change
was made to `object/src/meta.rs`.

## What's not done / open questions

- The offset-table path's residual risk (a sufficiently large declared length still aborts inside
  `StatefulDecoder::read_u32`) is the same root cause and the same call site already identified by
  candidate 004's own (drafted, verified, but not applied/merged this session either) fix for
  `parser/src/stateful/decode.rs`. Closing it fully means applying that candidate's fix (or an
  equivalent fallible-allocation guard for `read_u32` specifically) - out of scope for this
  candidate's own patch to avoid overlapping/conflicting with 004's already-recorded fix, but
  worth flagging explicitly so a maintainer applying only this candidate's `patch.diff` knows the
  offset-table path is improved, not fully closed, without also applying 004's fix.
  Added as a fresh backlog note in `IDEAS.md` pointing back at both candidates.
- Did not check whether any other reader in the codebase (e.g. the RLE/JPEG-LS/JPEG2000 codec
  wrappers under `encoding/src/`, or `pixeldata/`) has a similar `Vec::with_capacity`/`smallvec!`
  pattern sized from a fragment's declared length rather than from bytes actually available -
  candidates 013/014/015/016/017/018/019 already covered several codec-specific parsing bugs in
  that area but not specifically this allocation pattern; a longer fuzzing campaign (via
  `fuzz-bug-repro`) targeting allocation size specifically would be a reasonable next step if this
  class of bug is a priority to close out project-wide.
