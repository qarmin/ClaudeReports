# id3v2_iff_tag_growth_subtraction_underflow

Follow-up to candidate `014`'s explicit lead: `lofty/src/id3/v2/write/chunk_file.rs` (WAV/AIFF
ID3v2 write path) and `lofty/src/mp4/ilst/write.rs` (MP4 `ilst` write path) both compute byte
ranges to shift/truncate from lofty's own re-derived bookkeeping of an already-parsed existing
file, the same "subtract-before-check" shape behind candidates `003`/`011`/`013`, but were only
skimmed (not traced) during `014`'s sweep. This candidate traces both files with the same rigor.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading of `write_to_chunk_file` (`chunk_file.rs`) | Traced every subtraction in the "overwrite tag at end of file" fast path (lines 92-113) against the invariants established earlier in the same function | Found `updated_stream_len -= tag_chunk_size - existing_tag_len;` (the branch taken when the *new* tag is larger than the tag it replaces) subtracts a value that has no proven upper bound relative to `updated_stream_len` (the whole file's current length) - unlike its sibling branch (`existing_tag_len > tag_chunk_size`), which is provably safe because `remainder < existing_tag_len <= stream_length` |
| Standalone repro against the real public API (`WavFile::read_from`/`set_id3v2`/`save_to`), debug/`overflow-checks` build | A minimal real WAV file, tag it with a small ID3v2 tag (making it the file's last chunk), then tag it again with a ~200 KB tag | **Panicked**: `attempt to subtract with overflow` at `chunk_file.rs:105`, reproduced deterministically 4/4 runs (release profile with `overflow-checks = true`, no randomness/fixed seed involved) |
| Same repro, `overflow-checks = false` (plain release build) | Same input | No panic, but the `u64` subtraction wraps; `(updated_stream_len as u32) - IFF_CHUNK_HEADER_SIZE` then writes a RIFF header declaring the file is `4294783082 + 8` bytes (~4.29 GB) while the real file on disk is `215802` bytes - silent file corruption instead of a crash |
| Manual invariant tracing of `mp4/ilst/write.rs`'s three subtractions (`moov_len - existing_udta_size`, `udta.len - meta.len`, `meta.len - range.len()`) against `AtomInfo::read`'s bounds check and `ContextualAtom::read`'s recursive "container must be exactly filled by its children" check (`mp4/write.rs`) | N/A (static reasoning) | Every child atom relevant to these three subtractions (`udta` and `moov`, both `IMPORTANT_CONTAINERS` recursively validated at `AtomWriter::new_from_file` time; `meta` as `udta`'s child; `ilst` as `meta`'s child, validated by `write_to`'s own `find_child_atom`/`atom_tree` re-scan) is structurally bounded to `child.len <= immediate_parent.len` by the shared "per-atom check uses a shrinking, header-cancelling budget" pattern that `find_child_atom`/`atom_tree`/`ContextualAtom::read` all share - so none of these three subtractions can go negative, regardless of how the file's declared atom sizes are crafted |
| Boundary-value mutation sweep against `mp4/ilst/write.rs`, real `Mp4File::read_from`/`set_ilst`/`save_to` public API, debug/`overflow-checks` build | `tests/taglib/data/has-tags.m4a` (has a real `udta`/`meta`/`ilst` chain) with its `udta`/`meta`/`ilst`/`moov` declared-size fields each patched by `+0..+16` bytes (33 mutated files total, `catch_unwind`-wrapped) | 0 panics across all 33 mutations - some were rejected with a clean `Err` (`failed to write Mp4 file`), the rest wrote successfully; consistent with the manual analysis above |

## Status

**Confirmed** (for `lofty/src/id3/v2/write/chunk_file.rs`).

**Not reproducible** (for `lofty/src/mp4/ilst/write.rs`) - see the manual-tracing and
mutation-sweep rows above; both point the same direction and no panic was found.

**Fully reproducible: Yes.** `repro/` calls the real public entry point a genuine caller uses to
tag a WAV file (`WavFile::read_from` -> `set_id3v2` -> `save_to`) twice in sequence, exactly as a
real user tagging, then re-tagging, a file would - no hand-constructed internal state is fed
directly to `write_to_chunk_file`.

## Class

`crash` (debug/`overflow-checks` build) / `logic` (release build: silent file corruption via a
wrapped, truncated RIFF header size field).

## Root cause

In `write_to_chunk_file`'s "we can just overwrite the tag at the end of the file" fast path (taken
whenever an existing ID3v2 chunk is the file's last chunk - the common case for a
previously-tagged WAV/AIFF file), the code computes the file's new total length as:

```rust
if existing_tag_len > tag_chunk_size {
    let remainder = existing_tag_len - tag_chunk_size;
    updated_stream_len -= remainder;              // tag shrank: correct
    file.truncate(existing_id3_tag.end - remainder)?;
} else {
    updated_stream_len -= tag_chunk_size - existing_tag_len;   // tag grew: WRONG SIGN
}
```

When the new tag is larger than the one it replaces (`tag_chunk_size >= existing_tag_len`, the
`else` branch), the file's total length should *increase* by `tag_chunk_size - existing_tag_len`
(`+=`), not decrease (`-=`) - the physical `file.write_all`/`write_u8` calls inside
`write_id3v2_chunk` a few lines earlier already really do extend the file by that amount, since the
new tag's bytes are written starting at the old tag's start position. The `-=` computes a value
that has no relationship to the file's real new length: when the growth amount
(`tag_chunk_size - existing_tag_len`) exceeds the file's total *old* length (`updated_stream_len`,
easily true when a small existing tag is replaced with a much larger one, an entirely ordinary
"add more metadata" operation), the `u64` subtraction underflows.

## Introduced by

9c0a0844 "error: implement file/tag encoding errors" (2026-05-02) - `git log -S"tag_chunk_size - existing_tag_len"` finds this as the only commit touching that expression in the relevant file; best-effort, this commit's own diff is a broad error-type refactor so it's plausible (not independently verified this session) that the wrong-sign `else` branch was carried over unchanged from an equivalent pre-existing expression rather than newly written here.

## The fix

Change the sign of that one operator: `updated_stream_len -= tag_chunk_size - existing_tag_len;`
-> `updated_stream_len += tag_chunk_size - existing_tag_len;` (`patch.diff`). This makes
`updated_stream_len` track the file's real new length in both branches (shrink: subtract the
amount truncated off; grow: add the amount appended), which is exactly what the immediately
following `file.write_u32::<B>((updated_stream_len as u32) - IFF_CHUNK_HEADER_SIZE)` needs to write
a correct RIFF/FORM chunk size.

Verified: re-running `repro/` against the patched tree no longer panics, and the written RIFF
header's declared size now exactly matches the file's real physical size (`201134` bytes both
ways - see `output.txt`).

- Project test suite (`lofty` crate), run identically before and after the fix:
  - `cargo test -p lofty --lib`: **256 passed; 0 failed** (before and after, identical)
  - `cargo test -p lofty --test taglib`: **191 passed; 0 failed; 73 ignored** (before and after,
    identical)
  - No test in the existing suite covers "grow an existing trailing ID3v2 chunk in a WAV/AIFF
    file to a size larger than the original" (`test_wav::test_id3v2_tag` only shrinks/removes an
    existing tag), which is why this shipped unnoticed.

## What's not done / open questions

- Only the WAV/AIFF ID3v2 "overwrite at end of file" fast path was found buggy. The same
  function's other paths (`JUNK`-chunk padding path, full-file-rewrite fallback) were read and
  traced safe (see candidate `014`'s original grep sweep and the reasoning above) but weren't
  independently re-fuzzed this session.
- `mp4/ilst/write.rs`'s three analogous subtractions were traced and boundary-mutation-tested but
  not exhaustively fuzzed (33 hand-picked boundary mutations, not a real corpus-driven campaign);
  a longer `fuzz-bug-repro` session against the MP4 `ilst` write path would be a reasonable next
  step if this class of bug is suspected there more broadly.
- Not checked in this session: whether the AIFF path (same function, `BigEndian` instantiation)
  has any AIFF-specific interaction that differs from the WAV case demonstrated here - the bug is
  in shared, format-agnostic code, so it almost certainly reproduces identically for AIFF, but no
  AIFF-specific repro was built.
