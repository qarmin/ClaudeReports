# matroskademux FLAC headers + qtdemux closed-caption-box bounds fixes - both verified correct

Continuation of the "recent bounds-check-hardening cluster" lead from candidate #008 (same
pattern: an unsigned-subtraction-based bounds check that looks like it could underflow when read
as an isolated diff hunk). Checked the two remaining IDEAS.md items of this shape.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `a75bd8187b` "matroskademux: Make sure enough data is available when parsing FLAC headers" diff in isolation | `while (off < codec_data_size - 4)` (changed from `- 3`) and `if (len > codec_data_size - 4 \|\| off + len > codec_data_size - 4)` - both subtract 4 from `codec_data_size` (a `gsize`), which underflows if `codec_data_size < 4` | Looked like the same underflow shape as candidate #008 |
| Code reading: the full function, `gst_matroska_parse_flac_stream_headers()` in `subprojects/gst-plugins-good/gst/matroska/matroska-ids.c` (~line 275-320) | A pre-existing, unmodified guard a few lines above the loop (line 287): `if (codec_data == NULL \|\| codec_data_size < ((4) + (4 + 34))) return NULL;` guarantees `codec_data_size >= 42` before the loop - `codec_data_size - 4` (>= 38) can never underflow | **Rules out the bug** - same "diff-in-isolation misses the guard a few lines above" pattern as candidate #008 |
| Code reading: `55e1f6ec4f` "qtdemux: Ensure enough data is available for reading closed caption boxes" diff in isolation | Changed `if (size >= atom_length + new_atom_length)` (overflow-prone addition) to `if (new_atom_length > 8 && size - atom_length >= new_atom_length)` (subtraction-based) inside `extract_cc_from_data()`, `subprojects/gst-plugins-good/gst/isomp4/qtdemux.c` (~line 6582) | Same underflow shape again: is `size - atom_length` safe? |
| Code reading: the enclosing `if` in the same function (unchanged by this commit, immediately above): `if (size > atom_length + 8)` | This guarantees `size > atom_length` (since `size > atom_length + 8 > atom_length`) before the new `size - atom_length` subtraction is ever reached - no underflow possible. The added `new_atom_length > 8` conjunct additionally rejects a sub-atom claiming a length too small to even contain its own 8-byte header, before that length is trusted for the buffer copy that follows | **Rules out the bug** - correct, and the extra `new_atom_length > 8` check closes a second, smaller gap (a degenerate zero/tiny sub-atom length) beyond just the overflow the commit message mentions |

## Status

**Not reproducible** for both - no bug in either. Three for three now (with candidate #008) on
this specific "recent unsigned-subtraction bounds-check hardening" pattern across three different
demuxers (avi, matroska, isomp4) - all three fixes hold up once the full function context
(specifically, a pre-existing minimum-size guard a few lines above each diff hunk) is taken into
account, which `git show`'s diff view alone does not surface.

## Root cause

N/A (not a bug in either case).

## What's not done / open questions

- Given three consecutive correct verifications of this exact pattern, this specific line of
  investigation (re-checking recent unsigned-subtraction bounds-check commits for reintroduced
  underflows) has been reasonably thoroughly explored for the leads that were flagged this
  session - not worth continuing further on *this specific pattern* without a new lead surfacing
  it again, though the general heuristic (always read a security-fix diff in its full function
  context, not just the hunk) remains worth applying to any future candidate.
- The other four avidemux commits noted as still-open in candidate #008's own follow-up
  (`44383c6de7`, `0bcc6564c7`, `c8383e4848`, `bb8fb5a9bf`) remain unchecked.
