# chunk_file_junk_padding_size_off_by_header

Picked from `IDEAS.md`'s `chunk_file.rs`-JUNK-padding-arithmetic-combined-with-`Chunks::correct_position`
follow-up left by candidate `017`. Investigating that combination directly (see "What was tried")
led away from the odd-length/padding-byte-ambiguity angle it named and onto a much more direct,
always-present bug in the same JUNK-padding write branch, found by simply exercising it end-to-end
and checking the resulting file's actual on-disk byte layout (not just its total length, which is
what candidate `017`'s own corruption check happened to compare).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual invariant tracing of the odd-length-chunk padding-byte ambiguity (`Chunks::correct_position`, `lofty/src/iff/chunk.rs`) combined with `existing_tag_len`'s derivation in `find_existing_id3v2_tag` | Traced `chunk_start`/`chunk_end` computation through `Chunks::next`/`skip`/`correct_position` for both resolutions of the ambiguous-padding-byte lookahead (`valid_fourcc(next_fourcc)` true vs. false) | Both `chunk_start` and `chunk_end` are always self-consistent with the underlying reader's position (by construction: `remaining_size` is only ever adjusted by exactly as many bytes as the reader's position moves), and always bounded by `context.stream_length`, so the arithmetic sites already checked in candidate `017` (all three subtraction sites in `write_to_chunk_file`) remain safe regardless of which way the ambiguity resolves - this specific combination does not reopen an underflow risk. This ruled out the original `IDEAS.md` angle, but while reading `write_to_chunk_file`'s JUNK-padding branch (`lofty/src/id3/v2/write/chunk_file.rs`, ~lines 133-145) closely enough to trace this, a second, structurally distinct issue in the same branch became apparent (see below) and was investigated instead, since it lives in the exact suspect function this candidate was scoped to. |
| Manual arithmetic check of the JUNK chunk's declared-size computation itself | Read the JUNK-padding branch's `remainder`/`write_u32` calls directly | The branch's own guard (`existing_tag_len - IFF_CHUNK_HEADER_SIZE >= tag_chunk_size`) exists specifically so the freed-up space (`remainder = existing_tag_len - tag_chunk_size`) has room for the `JUNK` chunk's own 8-byte header - i.e. `remainder` is defined to *include* that header. But the code then writes `remainder` itself (not `remainder - IFF_CHUNK_HEADER_SIZE`) as the `JUNK` chunk's declared *content-only* body-size field, making the `JUNK` chunk's declared on-disk footprint (`header(8) + declared_size`) extend exactly `IFF_CHUNK_HEADER_SIZE` (8) bytes past the true end of the old tag's region on every single invocation of this branch - not input-dependent, not an edge case. |
| Real end-to-end repro via the public `WavFile::read_from`/`set_id3v2`/`save_to` API (`___BUGHUNT___/_reference/scratch_020`, later promoted to `repro/`) | Built a WAV file with an ID3v2 tag placed as a *non-terminal* chunk (a real, distinguishable trailing `TEST` chunk follows it), then shrank the tag via a real `save_to` call with `preferred_padding` set (forcing the JUNK-padding branch), then independently raw-byte-scanned the resulting file (not reusing lofty's own chunk reader) to check whether the real trailing chunk was still intact | The `TEST` chunk is completely lost: the written `JUNK` chunk declares a size 8 bytes larger than the actual leftover space, so a byte-accurate scan (and lofty's own reader, which also trusts declared chunk sizes) skips straight over the `TEST` chunk's 8-byte header and into its content when looking for what comes after `JUNK`. Reproduced deterministically 3/3 runs (release + `overflow-checks=true`, no randomness anywhere in the repro), and again via the standalone `repro/` folder's own inline-built WAV against a fresh clone at `base_commit`. |

## Status

**Confirmed.**

Deterministic, bit-for-bit repeatable: identical output on every run (3/3 in the `_reference`
harness, plus the standalone `repro/`'s own run before/after the patch), no seed/timing dependence
anywhere in the repro - the corruption follows directly and unconditionally from the arithmetic
whenever the JUNK-padding branch executes at all (i.e. whenever a WAV's existing ID3v2 tag shrinks
by at least `IFF_CHUNK_HEADER_SIZE` and is written with `preferred_padding` set to `Some`).

**Fully reproducible: Yes.** `repro/` drives the real, public `WavFile::read_from` / `set_id3v2` /
`save_to` entry points on a real in-memory WAV byte buffer - the single external input (a WAV file)
*is* the genuine external artifact a real caller/attacker would supply; there is no second
participant to fake here (CONFIRMATION.md section 11's single-participant exception applies
directly, same as every other on-disk-file-format candidate in this project).

## Class

logic (silent on-disk data corruption - not a panic/crash; the write itself succeeds and returns
`Ok(())`, but a real trailing chunk's data becomes unreachable/lost on any subsequent read).

## Root cause

`lofty/src/id3/v2/write/chunk_file.rs`, `write_to_chunk_file`, WAV `JUNK`-padding branch
(originally lines 133-145):

```rust
// - `IFF_CHUNK_HEADER_SIZE`, since we need at least `IFF_CHUNK_HEADER_SIZE` bytes to
// write a `JUNK` chunk
if (existing_tag_len - u64::from(IFF_CHUNK_HEADER_SIZE)) >= tag_chunk_size {
	let remainder = existing_tag_len - tag_chunk_size;
	log::trace!("Existing tag large enough to overwrite (padding size: {remainder})");

	file.seek(SeekFrom::Start(existing_id3_tag.start))?;
	write_id3v2_chunk::<_, B>(tag, write_options, &mut file)?;

	file.write_all(&JUNK_CHUNK_NAME)?;
	file.write_u32::<B>(remainder as u32)?;
	return Ok(());
}
```

When an existing ID3v2 tag shrinks, the leftover physical space between the end of the newly
written (smaller) tag and `existing_id3_tag.end` (the start of whatever real chunk data follows, or
EOF) is exactly `remainder = existing_tag_len - tag_chunk_size` bytes. A `JUNK` chunk written into
that space must have a total on-disk footprint (its own 8-byte `fourcc`+`size` header, plus its
declared content) equal to `remainder`, i.e. a declared content size of `remainder -
IFF_CHUNK_HEADER_SIZE`. The code instead writes `remainder` itself into the `JUNK` chunk's declared
size field - the value that was supposed to already account for consuming those 8 header bytes gets
reused unmodified as the *post-header* content length, silently claiming 8 bytes that were never
actually reserved for `JUNK`'s content. The guard condition's own comment (`- IFF_CHUNK_HEADER_SIZE,
since we need at least IFF_CHUNK_HEADER_SIZE bytes to write a JUNK chunk`) already establishes that
`remainder` is meant to include the header; the size-field write simply forgets to subtract it back
out. Any real chunk (or the RIFF stream's own declared-size boundary, in the case of trailing junk)
that physically follows gets silently eaten into by up to 8 bytes' worth of the `JUNK` chunk's
false claim - the following chunk's 8-byte header is fully consumed by a re-parse in the case tested
here (a 251-byte trailing body), losing the chunk entirely; a real next-chunk header only 1-7 bytes
after the boundary would instead misparse a truncated/shifted header and its own now-corrupted
content.

This is a distinct root cause from the wrong-sign `updated_stream_len -=` bug fixed in candidates
`015`/`016` (which used the correct operation with the wrong sign) and from the two subtraction
sites cleared in candidate `017` (which were correctly ordered, just needed proving so) - this one
is a plain off-by-`IFF_CHUNK_HEADER_SIZE` miscount in what value gets written as the `JUNK` chunk's
declared size, present unconditionally whenever this branch runs at all.

## The fix

Compute the `JUNK` chunk's declared content-only size as `remainder - IFF_CHUNK_HEADER_SIZE`
(clearly separated from `remainder`, which still represents the total freed-up space including the
header, matching what the guard condition already checks) instead of writing `remainder` itself:

```rust
let remainder = existing_tag_len - tag_chunk_size;
let junk_content_size = remainder - u64::from(IFF_CHUNK_HEADER_SIZE);
...
file.write_u32::<B>(junk_content_size as u32)?;
```

The guard `existing_tag_len - IFF_CHUNK_HEADER_SIZE >= tag_chunk_size` (equivalently `remainder >=
IFF_CHUNK_HEADER_SIZE`) already guarantees `junk_content_size` can't underflow, so no new bounds
check is needed.

Verified: `repro/`'s trailing `TEST` chunk is intact (correct position, correct size, correct
content, confirmed by an independent raw-byte scan) after the fix, and no longer intact (lost
entirely, consumed by the oversized `JUNK` declaration) before it - see `output.txt` for the actual
captured before/after run output, both via the standalone `repro/`/`justfile` cycle against a fresh
clone and via the larger `_reference/scratch_020` harness run directly in the main project tree.

- Project test suite (`cargo test --release -p lofty --tests` and `--doc`): **782 passed, 0
  failed, 96 ignored** (256 unit + 122 `files` + 56 `fuzz` + 2 `hound` + 15 `picture` + 191
  `taglib` [73 ignored] + 4 `tags` + 176 doctests [23 ignored]) both **before** and **after**
  applying the patch - identical counts, no regression, no pre-existing failure caught this bug
  (no existing test shrinks a non-terminal WAV ID3v2 tag through the JUNK-padding path and then
  checks a real trailing chunk's integrity).

## What's not done / open questions

- The original `IDEAS.md` suspect (odd-length existing `ID3 ` chunk + adversarial padding byte
  combined with these subtraction sites) was traced and found safe by construction (see "What was
  tried" above) - it did not reproduce, but this candidate confirms a different, unrelated bug
  found while reading the same function, so the original suspect itself is now considered checked
  and closed out, not left open.
- Did not check whether the AIFF write path has an equivalent JUNK-chunk-style padding mechanism
  with the same bug - per the existing code comment ("AIFF doesn't have an equivalent, so we'll
  always have to rewrite"), AIFF always takes the full-file-rewrite fallback instead, which
  candidate `017` already cleared independently, so this is believed not applicable there.
- Did not check whether any other RIFF/IFF-family write path in this codebase (or a sibling
  project reusing similar chunk-writing logic) has the same "size field forgets to exclude the
  header it's counting against" shape - this was the only such site found in `chunk_file.rs` during
  this candidate, but a codebase-wide grep for `write_u32` calls immediately following a
  `remainder`/available-space computation was not performed as a separate sweep.
- Did not fuzz this specific branch beyond the boundary case already covered (`remainder ==
  IFF_CHUNK_HEADER_SIZE` exactly, i.e. `junk_content_size == 0`, was exercised implicitly by the
  guard's own minimum but not singled out as its own explicit test case) - considered low-risk
  since the bug is a fixed, input-independent 8-byte miscount, not a boundary-dependent one.
