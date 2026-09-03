# rw2_v4decompressor_bits_leu16_oob

`rawler/src/decoders/rw2/v4decompressor.rs`'s `BitPumpPanasonic::peek_bits` reads 2 bytes via
`bits::LEu16(self.buffer, idx)` (bits.rs:207) with no bounds check on `idx`, reached while decoding
a Panasonic RW2 raw-format-3/4 ("v4") file - a real, still-open, user-reported crash (GitHub issue
#571, `range end index 24585 out of range for slice of length 24020`), with the actual crash-report
RW2 file attached to that issue. Investigated per candidate 015 from a batch of pre-identified
candidates (already root-cause-suspected before this session started).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Downloaded the real crash-report attachment from GitHub issue #571 (`compressed.zip` -> `sample1_IDX_1_RAND_173729052516569860293996.rw2`, 525764 bytes) | curl, one-off | Got the genuine real-world trigger file, not a hand-authored stand-in |
| Ran the exact code from the issue's own report (`rawler::get_decoder` -> `raw_metadata` -> `raw_image` -> `RawDevelop::develop_intermediate`) against that file, as a `rawler` example binary in a scratch git worktree of this repo | debug build, default (multi-threaded) rayon pool | Panicked immediately - but with racy first-panic location between `bits.rs:207` (LEu16) and the sibling `v4decompressor.rs:12` slice, depending on thread scheduling |
| Same, with `RAYON_NUM_THREADS=1` forced | debug build | Panicked at `bits.rs:207:30` deterministically, 5/5 runs, message "range start index 24583 out of range for slice of length 24020" - near-identical to the issue's own "range end index 24585 out of range for slice of length 24020" (same slice length 24020) |
| Same, release build (`cargo build --release`) | release, `RAYON_NUM_THREADS=1` | Panicked at the same `bits.rs:207:30` location, 5/5 runs - confirms this isn't a debug-assertions-only artifact |
| Root-caused by reading `decode_panasonic_v4`/`BitPumpPanasonic::peek_bits` and the RW2 v4 row-decode loop | code reading | See Root cause below |

No miri/ASAN/fuzzing needed - a real user-reported crash file plus manual boundary-value reasoning
(the existing candidate description already pinned the exact function/line) reproduced this on the
first real attempt; escalation to further tools wasn't needed once a clean, deterministic 5/5
repeat-run repro was in hand.

## Status

**Confirmed.** Reproduces 5/5 runs in a release build against the real crash-report file, with
`RAYON_NUM_THREADS=1` forced to make the specific `bits.rs:207` LEu16 panic (as opposed to a
racy sibling panic - see Root cause) deterministic; also reproduces 5/5 with the default
multi-threaded rayon pool, just with a nondeterministic first-panic location between the two
sibling out-of-bounds reads (same root cause either way).

**Fully reproducible**: **Yes**. `repro/` runs the real, public, top-level `rawler` API exactly as
the GitHub issue's own report does (`rawler::get_decoder` -> `Decoder::raw_metadata` ->
`Decoder::raw_image` -> `RawDevelop::develop_intermediate`), fed the actual crash-report RW2 file
downloaded from the issue's own attachment - a single external file handed to its real public
parsing entry point end-to-end, no second participant to fake (CONFIRMATION.md section 11's
"malformed file handed to a parser's real public entry point" case).

## Class

crash (index-out-of-bounds panic on real, unmodified user-reported file; both debug and release)

## Root cause

`decode_panasonic_v4` (rawler/src/decoders/rw2/v4decompressor.rs) decodes RW2 raw-format-3/4 rows
in parallel via `decompress_strips_fn`/rayon. For each row it computes a byte offset into the raw
file's compressed strip data purely from `width`/`row`:

```
let skip = ((width * row * 9) + (width / 14 * 2 * row)) / 8;
let blocks = skip / 0x4000;
let src = &buf[blocks * 0x4000..];          // (a) can already panic here
let mut pump = BitPumpPanasonic::new(src, split);
```

Neither `skip` nor `blocks` has any relation to how many bytes `buf` (the actual file's strip data)
contains - both are derived purely from the image's declared `width`/`height` (TIFF tags) and the
row index. For a truncated or otherwise malformed RW2 file (fewer compressed bytes than the
declared dimensions would need), `blocks * 0x4000` can already exceed `buf.len()`, panicking at (a)
- this is the `v4decompressor.rs:12:21` panic seen as a sibling in this session's own captured
output.

Even when (a) doesn't panic (the block offset happens to still be in range), `BitPumpPanasonic`'s
own internal cursor keeps advancing by fixed 0x4000-byte "blocks" as more bits are consumed within
the row, entirely decoupled from the *actual* remaining length of `src`:

```
fn peek_bits(&mut self, num: u32) -> u32 {
    if num > self.nbits {
      self.nbits += 0x4000 * 8;
      self.pos += 0x4000;
    }
    let mut byte = ((self.nbits - num) >> 3) ^ 0x3ff0;
    ...
    let bits = LEu16(self.buffer, byte as usize + self.pos - 0x4000) as u32;   // (b) panics here
    ...
}
```

Once enough bits have been consumed that `byte as usize + self.pos - 0x4000` walks past
`self.buffer.len()`, `LEu16` (`bits.rs:207`, `LittleEndian::read_u16(&buf[pos..pos + 2])`) panics
with the exact "range .. index out of range for slice of length .." shape reported in GitHub issue
#571 - this is candidate 015's named panic site.

This is a direct, unmodified port of dcraw/libraw's C decoder for this format, which relies on the
caller padding the input buffer well past its real end (a common C-decoder idiom - reading garbage
past EOF is harmless in C as long as the memory is mapped/allocated) - a guarantee this Rust port
never actually provides, since `buf` here is exactly the file's own (possibly truncated) byte
slice with no padding.

Because rows are decoded in parallel (`par_chunks_mut`/`try_for_each`), which of (a) or (b) panics
*first* - and on which specific row - depends on rayon's thread-scheduling order; both stem from
the identical root cause (unvalidated `skip`/`blocks`-derived offset vs. the true remaining buffer
length) and both were observed in this session's captured output (see `output.txt`).

## Introduced by

b40b2d3e "Add RW2 raw support" (2022-05-19) - the commit that first added `v4decompressor.rs`
(`git log --follow` on the file shows no earlier history; `git blame` on both panicking lines
points at this same commit, unchanged in substance since - later commits in the file's history
only touched compiler-warning/edition cleanups and an unrelated `decompress_strips_fn`
reorganization). Both the unchecked `&buf[blocks * 0x4000..]` slice and the unchecked
`BitPumpPanasonic::peek_bits` read were present verbatim in this original commit.

## The fix

Bounds-check both unchecked reads and treat "past the end of the real file data" as benign zero
bytes instead of panicking - the same convention this project's own candidate 013 fix
(`pumps.rs`'s `BitPumpJPEG::peek_bits`) already established for an equivalent truncated-entropy-data
situation, and consistent with the "pad with zero, don't crash" semantics the original C decoder
this was ported from implicitly relied on:

- `decode_panasonic_v4`: `let src = &buf[blocks * 0x4000..];` -> `let src = buf.get(blocks *
  0x4000..).unwrap_or(&[]);` (empty slice instead of panicking if the block offset is already past
  EOF).
- `BitPumpPanasonic::peek_bits`: the unchecked `LEu16(self.buffer, idx)` call -> a checked
  `self.buffer.get(idx..idx + 2).map(LittleEndian::read_u16).unwrap_or(0)`, returning 0 for any
  read that would run past the buffer's real end.

This keeps `BitPump`'s trait signature unchanged (`peek_bits`/`get_bits` stay infallible, matching
every other bit-pump implementation in this codebase) and doesn't require threading a `Result`
through `decode_panasonic_v4`'s call sites - a truncated file now decodes to a (partially garbage,
past the truncation point) image instead of panicking, matching how a real dcraw-style decoder
degrades on the same malformed input.

Verified:
- Re-ran `repro/` (the real crash-report file, release build) against the patched tree: 5/5 clean
  runs (exit 0, "did not reproduce") with `RAYON_NUM_THREADS=1`, and 5/5 clean runs with the
  default multi-threaded pool - both configurations that panicked 5/5 before the patch.
- Varied the input within the same bug class (CONFIRMATION.md section 7): truncated the same real
  file to 90% of its original length (a different failure boundary than the original file's own
  truncation point) - panicked before the patch, decoded cleanly (no panic) after. More aggressive
  truncations (70%/50%/30%) hit an unrelated, already-correct I/O bounds check further up the RW2
  pipeline before ever reaching this code - out of scope here, not a false negative for this fix.
- Project test suite (`cargo test -p rawler --lib`): 66 passed, 0 failed, both before and after the
  patch - no regression (the existing suite has no RW2 v4 coverage exercising this code either
  way).
- Full self-contained `just test` cycle (fresh clone of the real GitHub repo at base_commit,
  `reproduce-before` panics, `apply`, `reproduce-after` succeeds) was actually run end-to-end, not
  just read - see `output.txt`.

## What's not done / open questions

- `decode_panasonic_v5`/`v6`/`v7`/`v8` (the other RW2 raw-format variants, same file's sibling
  decoders) were not audited this session for the identical "byte offset derived from
  width/height/row, unrelated to actual buffer length" pattern - `v5decompressor.rs`'s own block
  offsetting (`&buf[block_id * V5_BLOCK_SIZE..block_id * V5_BLOCK_SIZE + V5_BLOCK_SIZE]`) looks
  structurally similar and worth a follow-up candidate.
- The fix makes a truncated v4 RW2 file decode to a partially-garbage image (zero-filled past the
  truncation point) rather than surfacing a decode error - this matches candidate 013's own
  precedent and avoids a larger API change (threading `Result` through `decode_panasonic_v4` and
  its two call sites in `rw2.rs`/`unwrapped.rs`), but a maintainer might prefer an explicit
  `Result::Err` instead, consistent with `decode_panasonic_v5`/`v6`/`v7`/`v8`'s own
  `Result<PixU16, String>` signatures - that would be a slightly larger, still-bounded follow-up
  fix if desired.
