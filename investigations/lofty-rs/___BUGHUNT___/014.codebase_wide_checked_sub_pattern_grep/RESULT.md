# codebase_wide_checked_sub_pattern_grep

A targeted, whole-`lofty/src`-tree sweep for the "subtract-a-constant-before-checking" shape that
produced three prior confirmed bugs in this project (candidate `003`: `stsd.read_u16()? as u8 + 1`
truncate-then-add; candidate `011`: `dfla.len - 12 < 18` subtract-before-check; candidate `013`:
`section_len - 2` unchecked right next to an earlier `checked_sub`-guarded copy of the identical
computation) - looking for any further, not-yet-found instance of the same shape outside
`picture/mod.rs`/`mp4/properties.rs`, per the `IDEAS.md` lead left by candidate `013`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grep + manual code reading | `grep -rn` across `lofty/src` for `checked_sub`/`checked_add`/`checked_mul` (found 8 files using them), then every raw `X - <literal>` / `X - Y` subtraction and every `as u8`/`as u16`/`as i8`/`as i16` truncating cast, followed by tracing each candidate site back to its nearest preceding bounds check by reading the surrounding function | ~25 individual subtraction/cast sites inspected across `mp4/read/mod.rs`, `mp4/atom_info.rs`, `mp4/moov.rs`, `mp4/properties.rs`, `mp4/ilst/read.rs`, `mp4/ilst/write.rs`, `wavpack/properties.rs`, `wavpack/read.rs`, `ape/properties.rs`, `ape/tag/header.rs`, `id3/v2/header.rs`, `id3/v2/read.rs`, `id3/v2/items/popularimeter.rs`, `id3/v2/items/sync_text.rs`, `id3/v2/write/chunk_file.rs`, `iff/chunk.rs`, `iff/wav/tag/write.rs`, `flac/write.rs`, `flac/properties.rs`, `util/io.rs`, `ogg_pager/src/packets.rs` - every one traced back to an explicit prior guard (`if len < N`, `if size < N`, `verify()`, a `match`/`cmp::Ordering` branch, or a provable monotonic-accumulator invariant) that makes the subtraction/cast safe. The only 3 sites where the guard was *missing* are the already-known, already-confirmed candidates `003`/`011`/`013` themselves (still present in the unpatched tree at grep time). |
| `cargo clippy -p lofty --lib -- -W clippy::arithmetic_side_effects` | Debug profile, default lint config | 333 warnings (mostly noise: ordinary `+=`/`-=` bookkeeping on internally-controlled counters). Spot-checked the subtraction-shaped ones not already covered by the manual pass (e.g. `ogg_pager/src/packets.rs`'s `packet_size - *already_read_bytes_count`) - traced as safe: `packet_size` is a monotonically-growing accumulator that is always `>=` the previously recorded `already_read_bytes_count` by construction (set from a prior, smaller `packet_size` snapshot). No new unguarded site found. |

## Status

**Not reproducible.**

The sweep covered every file in `lofty/src` (plus the `ogg_pager` workspace crate) that performs
`checked_sub`/`checked_add`/`checked_mul`-style bounds-checked arithmetic anywhere, on the theory
that a file already using the checked form somewhere is exactly where a second, unchecked copy of
the same shape (candidate `013`'s pattern) is most likely to have been missed - plus a broader
`as u8`/`as u16` truncating-cast grep across the whole tree (candidate `003`'s shape) and a
`clippy::arithmetic_side_effects` pass as a second, independent signal source. No new instance of
either shape survived tracing back to its guard. Not attempted: a project-wide fuzz burst (the
grep-and-trace approach is a static, exhaustive check of *this specific pattern*, which a fuzzer
would only sample probabilistically; the 3 already-confirmed instances of this pattern were all
found by fuzzing in earlier candidates, not by this grep, so this candidate's job was specifically
to check whether grep could find *more* of what fuzzing already found - it didn't).

## Class

N/A (not reproduced).

## Root cause

N/A - this candidate is a discovery sweep, not a single suspect site. For each site inspected, the
reason it's safe is recorded in the "What was tried" table's file list above (traced back to an
explicit prior guard or a provable invariant) rather than repeated per-site here.

## The fix

N/A - no bug found to fix.

- Project test suite: not run (no code change was made; nothing to verify).

## What's not done / open questions

- This sweep was scoped to the exact shape described in the `IDEAS.md` lead (a raw subtraction/
  truncating-cast on a parsed-from-file integer, sitting near an already-`checked_sub`-guarded
  sibling computation or otherwise missing an obvious guard). It did not re-examine every
  `unsafe` block, every `wrapping_*`/`saturating_*` call, or arithmetic that doesn't fit this
  specific "second occurrence" shape - those are out of scope for this candidate and would need
  their own sweep.
- `id3/v2/write/chunk_file.rs` and `mp4/ilst/write.rs` have several more subtraction sites on the
  *write* path (computing byte ranges to shift/truncate from an already-parsed existing file);
  these were skimmed but not traced with the same rigor as the read-path sites above, since they
  operate on lofty's own re-derived range bookkeeping rather than directly on freshly-read file
  bytes - worth a dedicated pass if this pattern is suspected there specifically.
- No further leads for this exact pattern remain in `IDEAS.md`; the backlog entry that spawned
  this candidate has been removed.
