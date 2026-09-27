# remainder_content_size_header_omission_sweep_broadened

Re-run of candidate `021`'s sweep for candidate `020`'s exact bug shape (a leftover/available-space
value defined to include a chunk/atom's own header bytes, but reused unmodified as that chunk/
atom's declared content-only size field) with a broader, name-agnostic net, per the `IDEAS.md`
follow-up left by `021` itself: instead of grepping for the specific identifier names
`remainder`/`remaining_size`/`available_space`/`leftover`, this candidate traces data-flow at
*every* `write_u32`/`write_u16`/`write_u64`/`to_be_bytes`/`to_le_bytes` size-field write call site
in every writer file across the whole `lofty/src` tree plus `ogg_pager`, checking whether the
written value was ever computed via any subtraction of a start-of-region offset from an
end-of-region offset (or an equivalent "total space minus content" computation) without a
subsequent header-size correction - regardless of what the intermediate variable is named.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Whole-tree grep to enumerate every writer-adjacent file (`find lofty/src ogg_pager -name '*.rs' \| xargs grep -l 'write_u32\|write_u16\|write_u64\|to_be_bytes\|to_le_bytes\|to_ne_bytes'`), narrowed to files actually responsible for on-disk size-field bookkeeping (`mp4/write.rs`, `mp4/ilst/write.rs`, `ape/tag/write.rs`, `ogg/tag/write.rs`, `iff/wav/tag/write.rs`, `iff/aiff/tag/mod.rs`, `id3/v1/write.rs`, `id3/v2/write/{mod,frame,chunk_file}.rs`, `flac/write.rs`, `flac/block.rs`, `ogg_pager/src/lib.rs`) | Static read, no build needed | Established the full candidate set of size-field write sites, deliberately not limited to files whose name contains "write" in a way a narrower grep might miss (this picked up `id3/v2/write/chunk_file.rs` and `flac/block.rs`/`flac/write.rs`, neither on candidate `021`'s original named list) |
| Manual data-flow trace of every write call in each file: for each `write_u32`/`write_u16`/`write_u64`/`to_be_bytes`/`to_le_bytes` that writes a declared chunk/atom/block content-size field, traced the written value backwards to its definition, specifically checking for any `end - start` (or equivalently-shaped "total available space") computation reused as a content-only size without a header-size correction | Read every writer file listed above in full (not just the matched lines) | Found exactly the same three shapes candidate `021` already covered, plus two new ones this broader net specifically targeted: (1) `mp4/ilst/write.rs`'s `size = end - start` (`build_ilst`, line 638) - safe, MP4's atom-size field is header-inclusive by convention and `start` is captured before the header bytes are written, so `size` already includes them; (2) `flac/write.rs`'s `available_padding = old_metadata_len.saturating_sub(metadata_len).saturating_sub(Block::BLOCK_HEADER_SIZE)` (padding-block sizing) - explicitly and correctly subtracts the block header size before the result is used as the `PADDING` block's content length, i.e. the exact correction candidate `020`'s bug omits; (3) `id3/v2/write/chunk_file.rs`'s JUNK-padding branch - the already-confirmed candidate `020` site (left unpatched in the working tree, as expected - `020`'s fix was reverted per the skill's revert-after-verify rule) |
| Whole-tree grep for the shape itself rather than any specific name (`grep -rn -E '\bw+end\w*\s*-\s*\w*start\w*\b'` across `lofty/src`/`ogg_pager/src`, i.e. any `*end* - *start*` subtraction anywhere, not just ones already named `remainder`/etc.) | Static read | 3 matches total: `mp4/ilst/write.rs:638` (`size = end - start`, already traced above, safe), `ape/read.rs:27` (`stream_len = end - start`, a *read*-path variable, never written back to a size field), `id3/v2/write/chunk_file.rs:256` (`chunk_end - chunk_start`, only used in a `log::trace!` message, not written to any field) - no new writer-side instance turned up by the shape-based grep beyond what the manual trace already found |

## Status

**Not reproducible.**

Covered: every size-field write call site (`write_u32`/`write_u16`/`write_u64`/`to_be_bytes`/
`to_le_bytes` writing a declared chunk/atom/block content-length) in every writer-responsible file
in `lofty/src` (`mp4/write.rs`, `mp4/ilst/write.rs`, `ape/tag/write.rs`, `ogg/tag/write.rs`,
`iff/wav/tag/write.rs`, `iff/aiff/tag/mod.rs`, `id3/v1/write.rs`, `id3/v2/write/mod.rs`,
`id3/v2/write/frame.rs`, `id3/v2/write/chunk_file.rs`, `flac/write.rs`, `flac/block.rs`) and in
`ogg_pager/src/lib.rs`, plus a whole-tree shape-based grep (any `*end* - *start*` subtraction,
independent of variable naming) as a cross-check against the manual trace missing something. No
new instance of candidate `020`'s bug shape was found - the only site matching it is the one
candidate `020` already confirmed and fixed (and whose fix was reverted here, per this skill's
"always revert" rule, so `id3/v2/write/chunk_file.rs`'s JUNK-padding branch is back to its
unpatched, buggy state in the working tree as this candidate's own investigation left it - that is
`020`'s finding, not a new one from this candidate).

## Class

N/A (nothing new confirmed).

## Root cause

Every leftover-space-reuse write site outside candidate `020`'s already-confirmed one turned out to
either (a) not reuse leftover space as a content-size field at all (most writers here always
replace/splice full ranges - `ape/tag/write.rs`, `ogg/tag/write.rs`, `iff/wav/tag/write.rs`,
`iff/aiff/tag/mod.rs`, `id3/v1/write.rs`, `id3/v2/write/mod.rs`, `id3/v2/write/frame.rs`), (b) use a
size convention where the field itself is defined header-inclusive so no correction is needed
(`mp4/write.rs`/`mp4/ilst/write.rs`'s `write_atom_size`/`write_free_atom`, matching MP4's own atom
convention - this reconfirms candidate `021`'s conclusion for MP4, now additionally verified via
data-flow rather than only via the named-writer read), or (c) already perform the exact header
correction candidate `020`'s bug omits (`flac/write.rs`'s `available_padding` computation
explicitly subtracts `Block::BLOCK_HEADER_SIZE` before the value is used as the `PADDING` block's
content length - this is the same "leftover space including a header, reused as content size"
shape as `020`, just implemented correctly, and is new evidence not covered by candidate `021`'s
narrower named-list sweep since `flac/write.rs`/`flac/block.rs` weren't on that list at all).

Broadening the net (data-flow tracing instead of identifier-name grepping, plus covering
`flac/write.rs`/`flac/block.rs`/`id3/v2/write/chunk_file.rs`/`id3/v1/write.rs`/
`id3/v2/write/mod.rs`/`id3/v2/write/frame.rs`, none of which were on candidate `021`'s original
named list) did surface real candidates worth checking, but all resolved to already-known-safe or
already-known-buggy (candidate `020`) sites - no additional occurrence of the bug family exists in
the current codebase.

## The fix

N/A - no new bug found, nothing to fix. (Candidate `020`'s own fix remains a separate, already-
recorded, unapplied patch in its own candidate folder.)

## What's not done / open questions

- This sweep covered `lofty/src` and `ogg_pager/src` only, per the task's stated scope; it did not
  extend into the `fuzz/` crate's harness code itself (which doesn't write size fields, only
  invokes the library's own read/write entry points, so it was not expected to contain this bug
  shape, but was not separately grepped for completeness).
- The shape-based whole-tree grep (`\w*end\w* - \w*start\w*`) only catches subtractions spelled
  with literal `end`/`start` substrings in the variable names; a hypothetical instance using
  entirely unrelated names for both the start and end offsets (not just the leftover-space
  variable, which this candidate's broader net already stopped requiring a specific name for)
  would only have been caught by the manual per-file data-flow trace, not by this specific grep -
  the manual trace is the primary evidence here, the grep is corroborating, not exhaustive on its
  own.
- No fuzzing or debug-assertions execution was performed this round - this is a pure code-reading/
  data-flow-tracing sweep (matching the methodology of candidates `014`/`017`/`021`, which this
  candidate is a direct continuation of), appropriate for a "does this specific arithmetic shape
  recur elsewhere" question where the property being checked (a fixed structural mismatch between
  what a size field is defined to include and what it's assigned) is not something random fuzz
  input timing would reliably surface faster than direct inspection already did in `020`.
