# mp4_minf_stbl_land_fix

`IDEAS.md` (2026-08-26 entry, noticed while working candidate `033`) asked to land candidate
`033`'s already-verified `lofty/src/mp4/properties.rs` `Minf::parse` `stbl`-within-`minf` fix
(`len` -> `len - 8`) as its own real candidate, the same way candidate `032` did for candidate
`030`'s `moov.rs` fix: `033`'s own session found, root-caused, and verified this fix, but only
ever applied it temporarily and reverted it afterward (per this skill's own "main tree always left
clean" rule), so it was never actually landed as a `patch.diff` of its own and remains unpatched
on mainline. This session formalizes that fix into its own candidate folder and re-confirms the
bug fresh against current mainline before landing it, the same way candidates `031`/`032` did for
candidates `026`/`030`'s fixes.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code-reading: re-read `lofty/src/mp4/properties.rs`'s current `Minf::parse` at the current `HEAD` (`5c6565e3`) | N/A | Confirmed line 489 is unchanged since candidate `033`'s session: still passes `len` (`minf`'s full, header-inclusive declared length) as the `find_child_atom` budget for `stbl`, even though the reader has already been seeked 8 bytes past `minf`'s own header by the caller (`read_properties`) before `Minf::parse` ever runs - the bug is still live, unpatched, on today's mainline (current `HEAD` is exactly candidate `033`'s own `base_commit`, so nothing changed on this file between that session and this one). |
| Boundary-value reconfirmation: rebuilt candidate `033`'s own `+16` mutation of a real fixture (`lofty/tests/taglib/data/has-tags.m4a`, `stbl`'s on-disk size field at byte offset 1862 incremented by 16, every other byte including the real `udta`/`meta`/`ilst` tag and the audio properties data untouched) and read it via the real public `Mp4File::read_from` API | Debug build, temporary `#[test]` (`lofty/tests/probe_034_scratch_test.rs`, not committed), 3 repeat runs | Fully unpatched mainline: `read_from` returns `Ok`, `duration`/`channels` both parse as if nothing were wrong, on all 3 runs - the malformed, oversized `stbl.len` is silently tolerated instead of being rejected. Identical to `033`'s own original finding, reconfirmed fresh this session rather than assumed unchanged. |
| Applying the fix directly to `lofty/src/mp4/properties.rs` (`len - 8`) and re-running the same scratch test 3 more times, then running the project's full test suite before and after | Debug build (scratch test), then `cargo test -p lofty --lib --tests -- --test-threads=1` for the suite | With the fix applied: the same `+16` file is correctly rejected with `Err(FileParseError { ty: Some(Mp4), .. })` on all 3 runs. Test suite: **647 passed, 0 failed, 73 ignored -> 647 passed, 0 failed, 73 ignored** (unchanged before/after; counts include the temporary scratch test used for this reconfirmation). |

Three approaches, all producing a real, consistent signal (no clean negatives) - this candidate's
job was landing an already-confirmed fix, not open-ended discovery, so the "dig" here is
reconfirmation against current mainline plus the mandatory before/after verification, matching
candidates `031`/`032`'s own "land for real" session structure.

## Status

**Confirmed**

Reproduces deterministically: the `+16` boundary-value mutation of a real fixture reproduces 3/3
runs on unpatched mainline (matching candidate `033`'s own prior finding, reconfirmed fresh this
session) and is gone 3/3 runs after the fix (debug build, direct against the main project's
working tree), plus again via the fully standalone `just test` clone-based reproduction in this
candidate's own `repro/` (fresh `git clone` of the real repo at `base_commit`, unmodified working
tree, release build) - actually run end-to-end this session, not just read.

**Fully reproducible: Yes.** This is a single-participant finding (the library's own read path
mishandling a file it is handed) - no second participant/peer is involved, so section 11's "real
second participant" bar doesn't apply. `repro/` runs the real, public `Mp4File::read_from` entry
point end-to-end against a real fixture (`lofty/tests/taglib/data/has-tags.m4a`) with exactly one
4-byte field modified on disk, everything else byte-for-byte untouched, exactly as any real caller
reading an MP4 file would.

## Class

logic

## Root cause

Identical to candidate `033`'s already-documented root cause (unchanged, still present verbatim on
today's mainline). `read_properties` (`lofty/src/mp4/properties.rs`) locates the audio `trak`'s
`minf` atom, then does:

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
ever subtracting each real child's own declared `len` as it walks past it, so the initial 8-byte
overcount is never corrected - it persists through every child `find_child_atom` walks past inside
`minf`, including the final check against the matched `stbl` atom's own declared size. A crafted
`stbl` can therefore declare a size up to 8 bytes larger than the space truly remaining inside
`minf` and still be accepted.

Once accepted, `Minf::parse`'s own loop over `stbl`'s children (`while read < stbl.len { ... }`)
uses this inflated `stbl.len` as its bound, reading one atom's worth of data past `stbl`'s real
end - into whatever bytes physically follow in the file (in the fixture used here, this lands on
the real, valid `udta` atom header that immediately follows). This is the same root-cause shape as
candidate `030`'s `moov.rs` `trak` -> `mdia` finding (landed for real by candidate `032`), examined
here at the separate `properties.rs` `minf` -> `stbl` call site, and independent of (but additive
with) candidate `026`'s separate `atom_info.rs` off-by-`ATOM_HEADER_LEN` bug (landed for real by
candidate `031`, not touched by this fix) - the two independently contribute up to 8 bytes of
slack each and stack when both are present, as `033`'s own session already demonstrated.

## Introduced by

Same as sibling candidate 033 (this candidate's own Root cause states it's identical, unchanged, still present verbatim on mainline): d067933b "MP4: Improve audio bitrate calculation" (2024-05-06).

## The fix

Changed the `find_child_atom` call to pass `minf`'s header-*exclusive* remaining length, matching
the pattern candidates `030`/`032` already established for the equivalent `moov.rs` call:

```rust
let Some(stbl) = find_child_atom(reader, len - 8, *b"stbl", parse_mode)? else {
```

This is sufficient because `len - 8` is exactly the number of content bytes that genuinely remain
inside `minf` once its own header has already been consumed by the caller's preceding
`reader.seek(SeekFrom::Start(minf_info.start + 8))` call - the same accounting `find_child_atom`'s
own internal loop already performs correctly for every subsequent child it walks past. `len` is
used nowhere else in `parse_inner`, so this is a single, local, non-confounded change.

Verified against more than the one triggering input this session: the original, untouched fixture
still parses and retains its properties identically before and after (`duration=3.708s`,
`channels=Some(2)`, unchanged); the `+16` construction is correctly rejected with the fix alone
(3/3 runs, debug, direct against the main tree, plus a clean `just test` run from a fresh
standalone clone).

- Project test suite (`cargo test -p lofty --lib --tests -- --test-threads=1`, matching candidates
  `026`/`030`/`031`/`032`/`033`'s choice to avoid this checkout's pre-existing unrelated
  parallel-test flakiness): **647 passed, 0 failed, 73 ignored -> 647 passed, 0 failed, 73 ignored
  (no change, no new failures)**, both before and after the fix (counts include a temporary,
  non-committed scratch test file used only for this candidate's own reconfirmation, deleted
  before this candidate folder was finalized - candidate `033`'s own reported count of 646
  excluded its own equivalent scratch test).
- `repro/` (the `+16` construction, standalone clone-based `just test`): silently accepts the
  malformed input before the fix (`read_from -> Ok`, malformed `stbl.len` tolerated), returns a
  graceful `Err(FileParseError { ty: Some(Mp4), .. })` after the fix.

## What's not done / open questions

- This fix alone does **not** close candidate `026`'s separate `atom_info.rs` bug (landed
  independently by candidate `031`) or candidate `030`'s separate `moov.rs` bug (landed
  independently by candidate `032`) - per this skill's own rule every candidate's fix is reverted
  before the session ends, so on any fresh checkout of current mainline, all three bugs are
  present simultaneously until every one of these three candidates' patches is applied together by
  whoever actually adopts them (`IDEAS.md` already carries a backlog note asking whether `030`'s
  and `026`'s patches apply/pass together; this candidate's own fix should be added to that same
  combined-verification request).
- Same open items `033` already left unaddressed, not re-investigated this session: a full grep
  for every remaining call site of `find_child_atom` across `lofty/src/mp4/` beyond the four now
  audited (candidate `029`'s two calls, cleared as dead code; candidate `030`/`032`'s `moov.rs`
  call; this candidate's `properties.rs` call) was not performed; and only the case where `stbl`
  is `minf`'s *last* real child was checked (matching every real-world fixture seen so far), not a
  `minf` with additional real children after `stbl`.
