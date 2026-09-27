# remainder_content_size_header_omission_sweep

A whole-`lofty/src`-tree grep sweep for the exact bug shape confirmed in candidate `020`
(`chunk_file.rs`'s WAV `JUNK`-padding branch: a computed leftover/available-space value that is
*defined* to include a chunk/atom's own header bytes gets written unmodified as that
chunk/atom's declared *content-only* size field, silently claiming header-sized bytes it never
reserved) - per the `IDEAS.md` lead left by candidate `020`, explicitly naming `mp4/ilst/write.rs`'s
`udta`/`meta`/`ilst` atom-size bookkeeping, `ape`, and other IFF/RIFF-family writers as candidates
worth checking for a sibling instance.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grep + manual code reading across `lofty/src` (and the `ogg_pager` workspace crate) for `remainder`/`remaining_size`/`available_space`/`leftover` identifiers, then tracing every `write_u32`/`write_u64` call that follows one back to where that value was computed | `grep -rn` for the identifier family, then `grep -B3` around every `write_u32`/`write_u64` call site to find any immediately preceded by a remainder/leftover/available-space computation | Exactly one site in the whole tree matches the shape: `lofty/src/id3/v2/write/chunk_file.rs:143` (`file.write_u32::<B>(remainder as u32)?;`), which is candidate `020`'s already-confirmed bug, still present in the tree at grep time (this session ran before that candidate's fix was reverted). No second, independent occurrence of the pattern was found anywhere else. |
| Manual read of `lofty/src/mp4/ilst/write.rs`'s `udta`/`meta`/`ilst` free-atom-reuse padding path (`available_space`/`remaining_space` computed at lines 263-316, consumed by `write_free_atom` at line 426 and by `pad_atom`'s `diff_abs`-sized `free`-atom write at line 400) | Traced `write_free_atom(writer, size)`'s own body: it writes `size` as the atom's on-disk **total** size field (header + content, per MP4 atom-size convention, unlike IFF/RIFF's content-only size field), then writes exactly `size - ATOM_HEADER_LEN` content bytes | Structurally safe and *not* an instance of this shape: MP4's atom-size field means "whole atom including header", so passing the total available/freed space straight through as `size` is the *correct* value for that field - there is no header-bytes-omitted-from-content-size mismatch here, unlike RIFF/IFF's `JUNK`/`LIST`/etc. chunk-size field, which is content-only. The `available_space - ilst_len > 8` guard (strictly `> 8`, not `>= 8`) also guarantees `size - ATOM_HEADER_LEN >= 1`, so no separate underflow either. This is the same conclusion candidate `015` reached for this file's subtraction sites, now independently re-confirmed for this specific size-field-semantics shape. |
| Manual read of `lofty/src/ape/tag/write.rs` (the only APE tag-write path) for any leftover-space-reuse/padding mechanism | Read the whole file's `write_u32`/`write_u64` call sites (footer/header size and item-count fields) | APE has no padding/leftover-space-reuse chunk at all - every write recomputes its size field directly from freshly-serialized data lengths (e.g. `size + 32`), never from a previously-computed "remaining space" value. Nothing for this bug shape to attach to. |
| Manual read of `lofty/src/iff/wav/tag/write.rs` and `lofty/src/iff/aiff/tag/mod.rs` (the other two IFF/RIFF-family chunk writers in the tree, besides the already-covered `chunk_file.rs`) | Grepped both for `JUNK`/`remainder`/`remaining`/`available`/`write_u32` | Neither writes a `JUNK`/padding chunk at all: `wav/tag/write.rs` has an explicit `// TODO: Write JUNK chunk for padding` comment (the mechanism doesn't exist yet on this path), and both files' only header-size-relative arithmetic is the opposite direction (`file_bytes.len() - IFF_CHUNK_HEADER_SIZE` when *computing* a size field from a real buffer length, not reusing a pre-computed remainder) - not this bug shape. |
| Same grep repeated against `ogg_pager/src/*.rs` (the project's own workspace-local OGG paging crate, since `IDEAS.md`'s "other IFF/RIFF-family writers" language plausibly extends to it) | `grep -rn` for `remainder`/`remaining`/`available`/`leftover` | `paginate.rs`'s `remaining_page_size` is a monotonically-decremented content-packing budget (how many more content bytes still fit in the current OGG page), never written back out as a size field itself - no `write_u32`/`write_u64` consumes it. Not this bug shape. |

## Status

**Not reproducible.**

The sweep covered every `write_u32`/`write_u64` call site in `lofty/src` (plus `ogg_pager`) that
follows a `remainder`/`remaining_size`/`available_space`/`leftover`-named computation, plus a
manual read of every chunk/atom writer in the IFF/RIFF/MP4 family (`chunk_file.rs`,
`mp4/ilst/write.rs`, `ape/tag/write.rs`, `iff/wav/tag/write.rs`, `iff/aiff/tag/mod.rs`) and the
workspace-local `ogg_pager` crate, since those were the specific candidates named in the `IDEAS.md`
lead. The only site matching this exact shape is candidate `020`'s already-confirmed bug itself;
no second, independent instance was found. Not attempted: a fuzz burst (this is a static pattern
grep across a small, enumerable set of chunk/atom-size-writing call sites, not a large input
space a fuzzer would sample better than an exhaustive manual read of every hit).

## Class

N/A (not reproduced).

## Root cause

N/A - this candidate is a discovery sweep, not a single suspect site. For each writer inspected,
the reason it's safe from this specific shape is recorded per-row in the "What was tried" table
above (MP4's atom-size field is header-inclusive by convention so `mp4/ilst/write.rs`'s reuse is
correct as written; APE has no leftover-space-reuse mechanism at all; WAV/AIFF's other writers
don't yet implement `JUNK`-style padding on their own paths).

## The fix

N/A - no new bug found to fix; candidate `020`'s fix (already verified and reverted in that
candidate's own session) is unaffected by this sweep.

- Project test suite: not run (no code change was made this candidate; nothing to verify).

## What's not done / open questions

- This sweep was scoped to the exact shape named in the `IDEAS.md` lead (a `remainder`/
  `available_space`/leftover-style identifier feeding a size-field write without accounting for
  the container's own header-size convention). It did not re-examine every `write_u32`/`write_u64`
  call in the tree unconditionally - only ones textually near one of those identifier names or
  living in a chunk/atom writer already known to do leftover-space reuse - so a copy of this bug
  shape using a differently-named local variable (not `remainder`/`remaining_size`/
  `available_space`/`leftover`) could still exist unnoticed.
- AIFF's write path was independently re-confirmed here (not just assumed from the code comment)
  to have no `JUNK`-equivalent chunk mechanism on its own tag-write file (`iff/aiff/tag/mod.rs`);
  this also closes out the open question candidate `020` left about AIFF specifically.
