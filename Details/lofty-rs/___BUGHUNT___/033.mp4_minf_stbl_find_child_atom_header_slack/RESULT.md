# mp4_minf_stbl_find_child_atom_header_slack

`IDEAS.md` (2026-08-26 entry, left open by candidate `030`) asked whether `lofty/src/mp4/properties.rs`'s
own `find_child_atom` call (`Minf::parse`'s `minf` -> `stbl` lookup, line 489,
`find_child_atom(reader, len, *b"stbl", parse_mode)`) has the same header-inclusive-budget shape
candidate `030` already confirmed and fixed for `moov.rs`'s `trak` -> `mdia` call - candidate `030`
explicitly left this call site unexamined, out of scope, in its own "what's not done" section.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code-reading / control-flow tracing of `read_properties`, `Minf::parse`, `find_child_atom`, `AtomInfo::read_inner`, and `AtomReader` | N/A | Confirmed the exact same shape as candidate `030`: `read_properties` seeks the reader to `minf_info.start + 8` (past `minf`'s own 8-byte header) before calling `Minf::parse(reader, minf_info.len, parse_mode)` - passing `minf`'s *full*, header-inclusive declared length, unmodified, as the search budget for `find_child_atom`'s lookup of `stbl` within `minf`. `len` is not used anywhere else in `Minf::parse`/`parse_inner` besides this one call, so the fix is a single, local, non-confounded change. |
| Boundary-value construction: took the same real fixture candidate `030` used (`lofty/tests/taglib/data/has-tags.m4a`), where `minf`'s only content is `smhd`(16)+`dinf`(36)+`stbl`(907), `stbl` being `minf`'s (and `mdia`'s, and `trak`'s) last real child, ending exactly where the real `udta`/`meta`/`ilst` tag atom begins - and incremented only the on-disk `stbl` size field (byte offset 1862) by exactly 16 (deliberately exceeding the max 8 bytes candidate `026`'s separate, still-unpatched `atom_info.rs` bug can provide on its own, to isolate this candidate's own contribution), leaving every other byte untouched | Debug build, temporary `#[test]`s (`lofty/tests/probe_033_scratch_test.rs`, not committed) calling the real public `Mp4File::read_from` | The mutated file still parsed successfully (`Ok`), with `duration`/`channels` both unchanged from the original - `stbl`'s malformed, oversized declared length was silently tolerated instead of being rejected. Repeated 3/3 runs, fully deterministic. In this exact fixture, the resulting overrun happens to land exactly at end-of-file (since `stbl` is `minf`'s last real child, `minf` is `mdia`'s and `trak`'s, and `udta` - the atom whose header the overrun reads into - is `moov`'s last child and the file's own end), so no crash or visible property corruption occurs here; see the next row for a variant where the same overrun is made to cause concrete harm. |
| Constructing a variant that makes the same overrun's consequence concrete rather than merely "accepted when it should be rejected": same +16 `stbl` mutation, plus renaming the fourcc of the real `udta` atom immediately following `trak`/`mdia`/`minf`/`stbl`'s shared true end (offset 2773, `udta` -> `stsd`) so the bytes `Minf::parse`'s desynced inner loop (`while read < stbl.len`) reads past `stbl`'s real end are interpreted as a plausible `stsd` atom instead of harmlessly skippable data | Debug build, same scratch test file, plus a `read_properties(false)` control run on the identical mutated file | The renamed-fourcc file's real, already-correctly-parsed `stsd_data` (containing the real `channels`/`sample_rate`/codec info) is silently overwritten with 2331 bytes of the real `meta` atom's content (misread as if it were `stsd` content); re-parsing that garbage as `stsd` then fails, so `Mp4File::read_from` returns `Err(FileParseError)` for a file whose only "malformation" was a legitimate-looking atom sitting exactly where this bug's overrun reads from. The `read_properties(false)` control run on the same file returned `Ok`, confirming the top-level `Moov::parse` tolerates the renamed atom on its own with no error - isolating the `Err` to specifically the `read_properties`/`Minf::parse` path, i.e. to this candidate's own bug. |
| Isolating this candidate's contribution from candidate `026`'s separate, already-documented, still-unpatched `atom_info.rs` off-by-`ATOM_HEADER_LEN` bug (which independently leaks its own 8 bytes of slack at *every* `AtomInfo::read` call site, including this one) - applying `026`'s already-verified fix (`if len > reader_size` in `atom_info.rs`) alone (properties.rs left unpatched) against the +16 mutation (plain, and the fourcc-renamed variant) | Debug build, same scratch tests | With only `026`'s fix applied, both the plain +16 file and the fourcc-renamed +16 file are correctly rejected (`Err`) - since `026`'s own bug alone tops out at 8 bytes of slack, a +16 lie already exceeds what `026` alone can explain, confirming this candidate's own call site independently contributes a further 8 bytes that `026`'s fix does not cover. | 

## Status

**Confirmed**

Deterministic, 3/3 repeat runs (debug build, standalone `repro/` against the project's public API)
against the real, unpatched mainline, plus a 3/3 clean run again via the fully standalone `just
test` clone-based reproduction (fresh `git clone` of the real repo at `base_commit`, unmodified
working tree, release build).

**Fully reproducible: Yes.** This is a single-participant finding (the library's own read path
mishandling a file it is handed) - no second participant/peer is involved, so section 11's "real
second participant" bar doesn't apply. `repro/` runs the real, public `Mp4File::read_from` entry
point end-to-end against a real fixture (`lofty/tests/taglib/data/has-tags.m4a`) with exactly one
4-byte field modified on disk, everything else byte-for-byte untouched, exactly as any real caller
reading an MP4 file would.

## Class

logic

## Root cause

`read_properties` (`lofty/src/mp4/properties.rs`) locates the audio `trak`'s `minf` atom, then
does:

```rust
reader.seek(SeekFrom::Start(minf_info.start + 8))?;
let Some(Minf { stsd_data, stts }) = Minf::parse(reader, minf_info.len, parse_mode)? else {
```

The explicit seek moves the reader 8 bytes past `minf`'s own header, into its content - but
`minf_info.len` (`minf`'s *full*, header-inclusive declared length) is passed unmodified as the
`len` budget. Inside `Minf::parse`/`parse_inner`, this `len` is used exactly once:

```rust
let Some(stbl) = find_child_atom(reader, len, *b"stbl", parse_mode)? else {
```

`find_child_atom`'s internal loop tracks its own running budget by starting at this `len` and only
ever subtracting each real child's own declared `len` as it walks past it
(`len = len.saturating_sub(atom.len)`), so the initial 8-byte overcount is never corrected -
it persists through every child `find_child_atom` walks past inside `minf`, including the final
check against the matched `stbl` atom's own declared size (performed by
`AtomInfo::read_inner`'s `(len - ATOM_HEADER_LEN) > reader_size` bounds check, where `reader_size`
is this same running budget). A crafted `stbl` can therefore declare a size up to 8 bytes larger
than the space truly remaining inside `minf` and still be accepted.

Once accepted, `Minf::parse`'s own loop over `stbl`'s children (`while read < stbl.len { ... }`)
uses this inflated `stbl.len` as its bound, so it attempts to read one atom's worth of data past
`stbl`'s real end - into whatever bytes physically follow in the file (in the fixture used here,
`minf`/`mdia`/`trak` all end at the same offset, so this lands on the very next real sibling
atom's own header, `udta`). Depending on what those bytes happen to look like, the observed
consequence varies:

- If they form a harmless/unrecognized atom that can be fully skipped without running past the end
  of the file or the `AtomReader`'s own remaining-size bound (as in the plain +16 test above,
  where the overrun happens to land exactly at end-of-file), the malformed, oversized `stbl.len`
  is silently tolerated with no observable difference in the returned `Mp4Properties` - a
  correctness bug (a file that should be rejected as malformed is accepted) without an immediately
  visible symptom in this particular fixture's byte layout.
- If they happen to look like a plausible `stsd` or `stts` atom (the only two fourccs
  `Minf::parse`'s loop treats specially), the loop overwrites the *already correctly parsed*
  `stsd_data`/`stts` with garbage read from whatever real data follows in the file, and re-parsing
  that garbage then fails - turning what would otherwise be a perfectly readable file into a
  spurious `Err(FileParseError)`, as demonstrated by the fourcc-renamed variant above.

This is the same root-cause shape as candidate `030`'s `moov.rs` `trak` -> `mdia` finding (an
initial `find_child_atom` budget left header-inclusive despite the reader already having consumed
the parent's own header), examined here for the first time at this separate call site, and
independent of (but additive with) candidate `026`'s separate `atom_info.rs` off-by-`ATOM_HEADER_LEN`
bug, which contributes its own further 8 bytes of slack at the same `AtomInfo::read` call inside
`find_child_atom` (not fixed by this candidate; still present and unpatched on the real,
unpatched mainline, and independently verified above to be insufficient on its own to explain
a +16 lie).

## Introduced by

d067933b "MP4: Improve audio bitrate calculation" (2024-05-06) - `git log -S` on `minf_info.len` in `lofty/src/mp4/properties.rs` finds this as the only commit introducing the `reader.seek(minf_info.start + 8)` + unmodified `minf_info.len` pattern.

## The fix

Changed the `find_child_atom` call to pass `minf`'s header-*exclusive* remaining length, matching
the sibling pattern candidate `030` already established for the equivalent `moov.rs` call:

```rust
let Some(stbl) = find_child_atom(reader, len - 8, *b"stbl", parse_mode)? else {
```

This is sufficient because `len - 8` is exactly the number of content bytes that genuinely remain
inside `minf` once its own header has already been consumed by the caller's preceding
`reader.seek(SeekFrom::Start(minf_info.start + 8))` - the same accounting `find_child_atom`'s own
internal loop already performs correctly for every subsequent child it walks past
(`len.saturating_sub(atom.len)`). `len` is used nowhere else in `parse_inner`, so this is a
single, local, non-confounded change.

Verified against more than the one triggering input: the plain +16 construction (isolated from
candidate `026`'s separate, still-unpatched slack) is correctly rejected with the fix alone; the
fourcc-renamed +16 variant (demonstrating concrete harm, not just "accepted when it should be
rejected") is also correctly rejected with the fix alone; and the original, untouched fixture
still parses identically before and after (`duration=3.708s`, `channels=Some(2)`, unchanged).

- Project test suite (`cargo test -p lofty --lib --tests -- --test-threads=1`, matching candidates
  `026`/`030`/`031`/`032`'s choice to avoid this checkout's pre-existing unrelated parallel-test
  flakiness): **646 passed, 0 failed, 73 ignored -> 646 passed, 0 failed, 73 ignored (no change,
  no new failures)**, both before and after the fix (counts exclude the temporary, non-committed
  scratch test file used only for this candidate's own investigation).

## What's not done / open questions

- Not checked whether the same header-inclusive-budget shape recurs at any further call site of
  `find_child_atom` beyond the four now audited across candidates `029` (two calls, cleared as
  dead code), `030`/`032` (the `moov.rs` `trak` -> `mdia` call, confirmed and fixed), and this
  candidate (the `properties.rs` `minf` -> `stbl` call, confirmed and fixed here) - a full grep for
  every remaining call to `find_child_atom` across `lofty/src/mp4/` was not performed this
  session.
- This candidate's fix, unlike candidates `031`/`032`, was not additionally landed as its own
  "reconfirm against current mainline and land for real" candidate - it was found, root-caused,
  and verified in the same session it was discovered in, matching candidate `030`'s original
  session structure (found + fixed + reverted, not yet re-applied permanently). A future session
  may want to do for this candidate what `032` did for `030`: reconfirm fresh against current
  mainline and land the fix as its own dedicated candidate, if the project's maintainers want the
  fix applied for real rather than left as a verified `patch.diff`.
- Candidate `026`'s own separate `atom_info.rs` bug remains unpatched (out of scope here, already
  tracked on its own); as shown above, it independently provides up to 8 bytes of the same kind of
  slack at every `AtomInfo::read` call site in the codebase, including this one, so a caller
  relying only on this candidate's fix should be aware a `+8`-scale version of the same
  silently-accepted-malformed-length symptom is still reachable today via `026`'s bug alone until
  that one is also fixed.
- Only checked the case where `stbl` is `minf`'s *last* real child (as in the fixture used, which
  matches every real-world MP4 file this session observed - `stbl` is conventionally the last
  child of `minf` per the ISO/IEC 14496-12 box ordering convention); did not construct a
  non-conventional fixture with additional real atoms after `stbl` inside `minf` to check whether
  the observed failure mode changes when the overrun consumes a *sibling-within-minf*'s bytes
  instead of a sibling of `minf`'s ancestor chain - not verified this session.
