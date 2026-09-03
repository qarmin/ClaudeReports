# aiff_id3v2_tag_growth_subtraction_underflow

Follow-up to candidate `015`, which confirmed a wrong-sign `u64` subtraction in
`lofty/src/id3/v2/write/chunk_file.rs`'s `write_to_chunk_file` (the ID3v2 write path shared by
WAV and AIFF) but only exercised the WAV (`LittleEndian`) instantiation, added to `IDEAS.md` as an
explicit follow-up. This candidate independently reproduces the same bug through the AIFF
(`BigEndian`) instantiation of the same function.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading: confirmed `lofty/src/id3/v2/write/mod.rs` dispatches AIFF to the exact same `write_to_chunk_file` function as WAV, differing only in the `ByteOrder` type parameter (`BigEndian` vs `LittleEndian`); the buggy line itself (`updated_stream_len -= tag_chunk_size - existing_tag_len;`) does not reference the byte-order parameter at all | N/A (static reasoning) | Confirms the bug is not endianness-dependent - the AIFF path was expected to reproduce identically |
| Standalone repro against the real public API (`AiffFile::read_from`/`set_id3v2`/`save_to`), debug/`overflow-checks` build | A minimal real AIFF file (`FORM`/`AIFF` + `COMM` + `SSND`), tag it with a small ID3v2 tag (making it the file's last chunk), then tag it again with a ~200 KB tag | **Panicked**: `attempt to subtract with overflow` at `chunk_file.rs:105`, reproduced deterministically 4/4 runs (release profile with `overflow-checks = true`, no randomness/fixed seed involved) |
| Same repro, `overflow-checks = false` (plain release build) | Same input | No panic, but the `u64` subtraction wraps; the FORM header ends up declaring a `4294768432`-byte file while the real file is `201144` bytes - silent file corruption instead of a crash, mirroring candidate `015`'s WAV finding exactly |

## Status

**Confirmed.**

**Fully reproducible: Yes.** `repro/` calls the real public entry point a genuine caller uses to
tag an AIFF file (`AiffFile::read_from` -> `set_id3v2` -> `save_to`) twice in sequence, exactly as
a real user tagging, then re-tagging, a file would - no hand-constructed internal state is fed
directly to `write_to_chunk_file`.

## Class

`crash` (debug/`overflow-checks` build) / `logic` (release build: silent file corruption via a
wrapped, truncated FORM header size field).

## Root cause

Identical mechanism to candidate `015`, independently confirmed through the AIFF entry point
rather than assumed from the WAV case. In `write_to_chunk_file`'s "overwrite the tag at the end of
the file" fast path (taken whenever an existing ID3v2 chunk is the file's last chunk):

```rust
if existing_tag_len > tag_chunk_size {
    let remainder = existing_tag_len - tag_chunk_size;
    updated_stream_len -= remainder;              // tag shrank: correct
    file.truncate(existing_id3_tag.end - remainder)?;
} else {
    updated_stream_len -= tag_chunk_size - existing_tag_len;   // tag grew: WRONG SIGN
}
```

When the new tag is larger than the one it replaces, the file's total length should *increase* by
`tag_chunk_size - existing_tag_len` (`+=`), not decrease (`-=`) - the physical `write_id3v2_chunk`
call a few lines earlier already extends the file by that amount. Because this code is
format-agnostic (the only thing that differs between WAV and AIFF call sites is the `ByteOrder`
type parameter `B`, used solely for reading/writing multi-byte integers, not in this
arithmetic), the underflow reproduces identically for AIFF: growing a small trailing ID3v2 chunk
to something larger drives `updated_stream_len` (starting at the file's small old total length)
negative in `u64`.

## The fix

Same one-character fix as candidate `015`, applied to the same shared function: change
`updated_stream_len -= tag_chunk_size - existing_tag_len;` to
`updated_stream_len += tag_chunk_size - existing_tag_len;` (`patch.diff`). Since this is the exact
same line candidate `015` already fixed, applying it once fixes both the WAV and AIFF
instantiations - this candidate exists to independently verify that claim for AIFF specifically,
not to propose a second, different fix.

Verified: re-running `repro/` against the patched tree no longer panics, and the written FORM
header's declared size now exactly matches the file's real physical size (`201144` bytes both
ways - see `output.txt`).

- Project test suite (`lofty` crate), run identically before and after the fix:
  - `cargo test -p lofty --lib`: **256 passed; 0 failed** (before and after, identical)
  - `cargo test -p lofty --test taglib`: **191 passed; 0 failed; 73 ignored** (before and after,
    identical)
  - No test in the existing suite covers "grow an existing trailing ID3v2 chunk in an AIFF file to
    a size larger than the original" for either format, which is why this shipped unnoticed for
    both WAV and AIFF.

## What's not done / open questions

- As with candidate `015`, only the "overwrite tag at end of file" fast path was exercised for
  AIFF. AIFF has no `JUNK`-chunk padding path (AIFF always falls through to the full-file-rewrite
  path instead, per the comment in `chunk_file.rs` itself: "Unfortunately, AIFF doesn't have an
  equivalent, so we'll always have to rewrite"), so that branch of candidate `015`'s open question
  doesn't apply to AIFF at all; the full-file-rewrite fallback path was read and traced safe (same
  reasoning as `015`) but not independently re-fuzzed this session.
- This candidate and `015` together now cover both real callers of `write_to_chunk_file`
  (`FileType::Wav` and `FileType::Aiff`, the only two dispatch arms in
  `lofty/src/id3/v2/write/mod.rs`); no further format-specific instantiation of this function
  exists to check.
