# `IFD::new`'s `Value::Unknown` SubIFD-offset lookup indexes an empty vector

`rawler::formats::tiff::ifd::IFD::new` (`rawler/src/formats/tiff/ifd.rs`), while scanning a TIFF
IFD's directory entries for tags that point at embedded sub-IFDs (`SubIFDs`/`ExifIFDPointer` by
default), has a branch for entries whose TIFF type code isn't one of the twelve standard TIFF
types: `Entry::parse` stores those as `Value::Unknown(type_code, bytes)`, where `bytes` has exactly
`count` elements as declared by the file. If the file declares `count = 0` for such an entry,
`bytes` is empty, and the `Value::Unknown` match arm in `IFD::new` unconditionally indexes
`offsets[0]`, panicking with `index out of bounds: the len is 0 but the index is 0`. This is
GitHub issue #569 ("still open" per this session's task description), reported by an
OSS-Fuzz-style bot via the real `get_decoder -> raw_metadata/raw_image -> develop` pipeline; the
issue's own attached crash file (`compressed.zip`) was not downloaded/tried this session - the bug
was independently reconstructed from first principles (reading `Entry::parse`'s fallback branch
and `IFD::new`'s SubIFD-offset match arms) and confirmed with a hand-built minimal file instead.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `ifd.rs`'s `Value::Unknown(tag, offsets) => sub_ifd_offsets.insert(*tag, vec![offsets[0] as u32])` match arm and traced where `Value::Unknown` gets constructed (`entry.rs`'s catch-all `x =>` branch: `vec![0; count as usize]`, `count` fully attacker-controlled from the file) | code reading only | Root cause identified: a TIFF entry with an unrecognized type code and a declared count of 0 produces an empty `Vec`, and the SubIFD-offset lookup indexes element 0 of it unconditionally |
| Traced which real, un-gated public entry points reach `IFD::new` with `sub_tags` containing `SubIFDs`/`ExifIFDPointer` (`IFD::new_root`'s default) *before* any camera-support check that could reject the file first | code reading only | `rawler::decoders::raf::parse_raf` (the RAF/Fujifilm "main IFD" parse) calls `IFD::new_root(stream, offset + 12)` as the very first thing in `RafDecoder::new`, which itself runs directly from `RawLoader::get_decoder`'s `is_raf()` branch - reachable purely from the 8-byte `"FUJIFILM"` magic check, no camera-model allow-list gate in front of it |
| Manual boundary-value construction: a minimal on-disk `.raf` file (200 bytes) with a real `"FUJIFILM"` header, a valid pointer to an embedded TIFF-style "main IFD", and that IFD's single directory entry set to tag=`SubIFDs` (0x014A), TIFF type=0 (unrecognized), count=0 - fed to `rawler::imgop::develop::process_raw_image` (the real, uncaught-by-`catch_unwind` public entry point `dnglab process-raw` actually calls) | debug build (`cargo run --example`) then release build (via the candidate's own `repro/`), 3 runs each build | Deterministic panic every run, exact message and location match: `rawler/src/formats/tiff/ifd.rs:157:58: index out of bounds: the len is 0 but the index is 0` |
| Same input with tag=`ExifIFDPointer` (0x8769) instead of `SubIFDs`, to vary the input within the same bug class per `CONFIRMATION.md` section 7 | debug build, 1 run before the fix, 1 run after | Panics identically before the fix (same message/location), does not panic after - confirms the fix isn't narrowly scoped to the exact tag value that first triggered it |
| `cargo test -p rawler --lib` (rawler's own existing suite) before and after the fix | debug build | 66 passed/0 failed both before and after (pre-existing suite unaffected either way) |

One approach (manual boundary-value construction) was enough to get a deterministic, real
end-to-end reproduction on the first attempt - the root cause was unambiguous from code reading
alone (a `Vec` of exactly `count` elements indexed at a fixed offset with no length check), so no
escalation to a fuzz harness/miri/sanitizers was needed; this is a plain out-of-bounds index on a
`Vec<u8>`, not raw-pointer or FFI-adjacent UB.

## Status

**Confirmed.** Reproduces deterministically, 3/3 runs in both a debug build and a release build
via the standalone `repro/` (against both a live worktree and, separately, a genuinely fresh
`git clone` of the upstream repo via this candidate's own `just test`) - no randomness or
memory-layout dependence involved (a pure `Vec` length vs. fixed-index-0 comparison bug).

**Fully reproducible**: **Yes**. `repro/` calls `rawler::imgop::develop::process_raw_image`, the
exact public API `dnglab process-raw`'s job code (`dnglab-lib/src/jobs/process_raw.rs`'s
`Raw2Image::internal_exec`) calls, and which itself calls `crate::get_decoder(&rawfile)?`
completely unguarded by `catch_unwind` (unlike `rawler::decode`/`decode_file`, which do wrap the
equivalent call in `panic::catch_unwind` - see `RawLoader::decode`). The input is a single,
self-contained on-disk file (a minimal but real `.raf` container with a valid magic and a real
embedded TIFF structure) - the same kind of external artifact a real (fuzzer-generated or
hand-corrupted) camera raw file would be, handed to its real public parsing entry point. No second
participant is being faked, per `CONFIRMATION.md` section 11 (this is the single-artifact
exception case: a malformed file handed to a parser's real public entry point *is* the genuine
external input).

## Class

`crash`

## Root cause

`rawler/src/formats/tiff/entry.rs`, `Entry::parse`: when the file's declared TIFF type code isn't
one of the twelve standard types (`typ` doesn't match any of `TYPE_BYTE`..`TYPE_DOUBLE`), the
catch-all branch stores the raw entry data as `Value::Unknown(typ, v)` where `v = vec![0; count as
usize]` and `count` is a 4-byte value read directly from the file with no lower-bound validation -
so `count == 0` produces an empty `Vec<u8>`.

`rawler/src/formats/tiff/ifd.rs`, `IFD::new`: while parsing directory entries, any entry whose tag
is in the caller-supplied `sub_tags` list (`SubIFDs`/`ExifIFDPointer` by default, via
`IFD::new_root`) is inspected to find the offset of the sub-IFD it points to:

```rust
Value::Unknown(tag, offsets) => {
  sub_ifd_offsets.insert(*tag, vec![offsets[0] as u32]);
}
```

This unconditionally indexes `offsets[0]`, with no check that `offsets` (the `Vec<u8>` described
above) actually has an element. A TIFF entry with tag=`SubIFDs` (or `ExifIFDPointer`), an
unrecognized type code, and a declared count of 0 produces exactly the empty `Vec` this indexes,
panicking with `index out of bounds: the len is 0 but the index is 0`. The sibling
`Value::Undefined(_)` arm two lines below handles the equivalent "no element to read" case safely
(via `entry.offset()` returning an `Option`) - this is the pattern the fix follows.

This is reachable with no camera-model/support-list gate in front of it via
`rawler::decoders::raf::parse_raf` (Fujifilm RAF format): `RafDecoder::new` calls
`IFD::new_root(stream, offset + 12)` to parse the RAF container's embedded "main IFD" as the very
first thing it does, directly from `RawLoader::get_decoder`'s `raf::is_raf()` branch (gated only
on the 8-byte `"FUJIFILM"` magic).

## Introduced by

416d897a "Code refactoring for more raw formats" (2022-04-10). `git log -S "offsets[0] as u32" --
rawler/src/formats/tiff/ifd.rs` finds exactly one commit introducing this exact line; it is a
large, whole-project restructuring/vendoring commit (the point at which the `rawler` crate as
currently laid out first appears in this repository's tracked history) - the buggy line is present
verbatim from that commit's first version of `ifd.rs`, with no earlier occurrence in this
repository's history to bisect further back into.

## The fix

Mirrors the existing `Value::Undefined(_)` arm two lines below: check whether `offsets` actually
has a first element before indexing it, and skip the SubIFD lookup for that tag (with a
`log::warn!`, matching the existing style used elsewhere in the same function for other
"can't determine a usable offset" cases) instead of indexing unconditionally.

```rust
-              Value::Unknown(tag, offsets) => {
-                sub_ifd_offsets.insert(*tag, vec![offsets[0] as u32]);
-              }
+              Value::Unknown(tag, offsets) => {
+                if let Some(offset) = offsets.first() {
+                  sub_ifd_offsets.insert(*tag, vec![*offset as u32]);
+                } else {
+                  log::warn!("SubIFD entry for tag 0x{:X} has no data, skipping", tag);
+                }
+              }
```

Verified: `repro/` no longer panics against the patched tree, in both a debug and a release build,
3/3 runs each; it now returns `Err` (the crafted file is otherwise too minimal to satisfy later
required tags, so decoding still correctly fails - just without panicking). Also re-verified with
tag=`ExifIFDPointer` substituted for tag=`SubIFDs` in the crafted input (same bug class, different
concrete tag), per `CONFIRMATION.md` section 7: panics before the fix, does not after.

- Project test suite (`cargo test -p rawler --lib`): 66 passed/0 failed before -> 66 passed/0
  failed after (pre-existing suite has no coverage for this exact code path either way, so this is
  "unaffected", not "regression-tested" - see "What's not done" below).

## What's not done / open questions

- No regression test was added to `rawler`'s own test suite (`cargo test -p rawler --lib`'s
  66/66 is the pre-existing suite, unchanged by this fix) - only the standalone `repro/`
  exercises this exact input. A `#[test]` in `ifd.rs` or `entry.rs` directly constructing a
  `Value::Unknown` entry with an empty `Vec` and asserting `IFD::new` doesn't panic would be a
  reasonable, cheap follow-up.
- The issue's own attached crash file (`compressed.zip`, GitHub issue #569) was not downloaded or
  tried this session - this candidate's repro is an independently hand-built minimal input
  reaching the same panic location and message, not a replay of the reporter's exact bytes. The
  root cause and fix should be identical either way (the panic site takes exactly one code path),
  but the original file wasn't cross-checked against this candidate's understanding.
- The same `Value::Unknown` match arm, and the same "unrecognized type + zero count" input shape,
  is reachable through any other caller that parses a TIFF-style structure with non-empty
  `sub_tags` - e.g. `rawler::decoders::rw2::Rw2Decoder::new`'s `IFD::new_root(&mut
  Cursor::new(buf), 0)` re-parse of an embedded Panasonic `CameraIFD` tag's raw bytes, and
  `rawler::formats::jfif`'s `IFD::new_root` call for JFIF/EXIF thumbnails - both use the same
  default `sub_tags` list and the same `IFD::new` code path, so both should already be fixed by
  this same patch (it's the shared code, not the RAF-specific caller, that changed), but neither
  was independently exercised with its own crafted input this session.
- Per `SKILL.md`'s "Critical pitfalls" #7: the broader class of "TIFF entry with unrecognized type
  and a file-controlled small/zero count feeding an unconditional index" was not swept across the
  rest of `ifd.rs`/`entry.rs`/`value.rs` beyond this one match arm - `Value::Unknown`'s other call
  sites (`value.rs`, e.g. the `Display`/size-estimation impls) were skimmed and appear to already
  use `.len()`/iterator-based access rather than a fixed index, but this wasn't exhaustively
  fuzzed or boundary-tested.
