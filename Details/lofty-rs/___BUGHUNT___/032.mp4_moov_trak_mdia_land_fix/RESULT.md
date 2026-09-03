# mp4_moov_trak_mdia_land_fix

`IDEAS.md` (2026-08-26 entry, noticed while working candidate `031`) asked to land candidate
`030`'s already-verified `lofty/src/mp4/moov.rs` `trak`->`mdia` fix (`atom.len - 8`) as its own
real candidate: `030`'s own session (and `031`'s, when it temporarily applied `030`'s fix in
isolation to verify scope) both applied this fix only temporarily and reverted it afterward, so it
was never actually landed as a `patch.diff` of its own, and remains unpatched on mainline. This
session formalizes that fix into its own candidate folder and re-confirms the bug fresh against
current mainline before landing it, the same way candidate `031` did for candidate `026`'s fix.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code-reading: re-read `lofty/src/mp4/moov.rs`'s current `Moov::parse` at the current `HEAD` (`5c6565e3`) | N/A | Confirmed line 54 is unchanged since candidate `030`'s session: still passes `atom.len` (`trak`'s full, header-inclusive declared length) as the `find_child_atom` budget for `mdia`, even though the reader has already been seeked past `trak`'s own 8-byte header by this point - the bug is still live, unpatched, on today's mainline. |
| Boundary-value reconfirmation: rebuilt candidate `030`'s own `+16` mutation of a real fixture (`lofty/tests/taglib/data/has-tags.m4a`, `mdia`'s on-disk size field incremented by 16, every other byte including the real `udta`/`meta`/`ilst` tag left untouched) and read it via the real public `Mp4File::read_from` API | Debug build, temporary `#[test]` (`lofty/tests/probe_032_scratch_test.rs`, not committed), 3 repeat runs | Fully unpatched mainline: `read_from` returns `Ok`, `ilst()` is `None` on all 3 runs - the untouched, still-present tag is silently lost. Identical to `030`'s own original finding, reconfirmed fresh this session rather than assumed unchanged. |
| Applying the fix directly to `lofty/src/mp4/moov.rs` (`atom.len - 8`) and re-running the same scratch test 3 more times, then running the project's full test suite before and after | Debug build (scratch test), then `cargo test -p lofty --lib --tests -- --test-threads=1` for the suite | With the fix applied: the same `+16` file is correctly rejected with `Err(FileParseError { ty: Some(Mp4), .. })` on all 3 runs. Test suite: **647 passed, 0 failed, 73 ignored -> 647 passed, 0 failed, 73 ignored** (646 from the project's own suite + 1 from the temporary scratch test, unchanged before/after). |

Three approaches, all producing a real, consistent signal (no clean negatives) - this candidate's
job was landing an already-confirmed fix, not open-ended discovery, so the "dig" here is
reconfirmation against current mainline plus the mandatory before/after verification, rather than
searching for a new bug.

## Status

**Confirmed**

Reproduces deterministically: the `+16` boundary-value mutation of a real fixture reproduces 3/3
runs on unpatched mainline (matching candidate `030`'s own prior finding, reconfirmed fresh this
session) and is gone 3/3 runs after the fix (debug build, direct against the main project's
working tree), plus again via the fully standalone `just test` clone-based reproduction in this
candidate's own `repro/` (fresh `git clone` of the real repo at `base_commit`, unmodified working
tree, release build).

**Fully reproducible: Yes.** This is a single-participant finding (the library's own read path
mishandling a file it is handed) - no second participant/peer is involved, so section 11's "real
second participant" bar doesn't apply. `repro/` runs the real, public `Mp4File::read_from` entry
point end-to-end against a real fixture (`lofty/tests/taglib/data/has-tags.m4a`) with exactly one
4-byte field modified on disk, everything else byte-for-byte untouched, exactly as any real caller
reading an MP4 file would.

## Class

logic

## Root cause

Identical to candidate `030`'s already-documented root cause (unchanged, still present verbatim on
today's mainline): `Moov::parse` (`lofty/src/mp4/moov.rs`) handles each top-level child of `moov`
via `reader.next()`, which leaves the reader positioned at the start of that child's *content*,
past its own 8-byte header. For a `trak` child (when `read_properties` is enabled, the default),
it looks up `trak`'s `mdia` grandchild:

```rust
if let Some(mdia) =
    find_child_atom(reader, atom.len, *b"mdia", parse_options.parsing_mode)?
```

`atom.len` here is `trak`'s *full*, header-inclusive declared length - but the reader has already
consumed `trak`'s own header. `find_child_atom`'s internal running budget therefore starts 8 bytes
too generous, and stays 8 bytes too generous through every subsequent child it walks past inside
`trak`. By the time `find_child_atom` reaches `mdia`, the effective budget it checks `mdia`'s
declared size against is 8 bytes larger than what is truly available inside `trak` (two lines
below, at line 61, the sibling `udta` case gets this right: `ilst_from_udta(reader, parse_options,
atom.len - 8)` correctly subtracts the header before using the parent's length as a child budget -
`trak`'s case at line 54 does not).

Once accepted, the caller does `skip_atom(reader, mdia.extended, mdia.len)`, which seeks forward
using the inflated `mdia.len`, landing the reader up to 8 bytes past `trak`'s real end - into the
next moov sibling's own header (`udta`, in the fixture used here, which holds the real `ilst`
tag). `Moov::parse`'s top-level loop then tries to parse moov's next child starting from that
now-misaligned position; whatever it finds there either fails outright or is unrecognizable, and
the loop simply exits with no error surfacing anywhere. Every atom after the corrupted one (here,
all of `udta`/`meta`/`ilst`, i.e. the entire tag) is silently dropped, even though every one of its
bytes is still physically present and completely untouched in the file. `Mp4File::read_from`
returns `Ok` throughout, with no indication to the caller that anything was lost.

This is a distinct bug from candidate `026`'s/`031`'s (a separate off-by-`ATOM_HEADER_LEN` bug
shared by every call to `AtomInfo::read`, landed separately by candidate `031`, not touched by
this fix) - the two independently contribute up to 8 bytes of slack each and stack when both are
present, as `030` and `031` both already demonstrated. This candidate's `+16` reproduction is
deliberately chosen to isolate this fix's own effect from `026`'s/`031`'s, since both bugs remain
unpatched simultaneously on the real mainline this session ran against (per this skill's own rule
that every candidate reverts its fix before finishing, `031`'s landed fix is not actually present
on disk between sessions either).

## Introduced by

Same as sibling candidate 030 (this candidate's own Root cause states it's identical, unchanged, still present verbatim on mainline): 238b40c4 "MP4: Remove `trak` module" (2022-10-11).

## The fix

Changed the `find_child_atom` call to pass `trak`'s header-*exclusive* remaining length, matching
the sibling `udta` case two lines below:

```rust
if let Some(mdia) = find_child_atom(
    reader,
    atom.len - 8,
    *b"mdia",
    parse_options.parsing_mode,
)?
```

This is sufficient because `atom.len - 8` is exactly the number of content bytes that genuinely
remain inside `trak` once its own header has already been consumed by the preceding `reader.next()`
call - the same accounting `find_child_atom`'s own internal loop already correctly performs for
every subsequent child it walks past, and the same accounting the `udta` case already uses one
branch below. Verified against more than the one triggering input this session: the original,
untouched fixture still parses and retains its tag identically before and after; the `+16`
construction is correctly rejected with the fix alone (3/3 runs, debug, plus a clean `just test`
run from a fresh standalone clone).

- Project test suite (`cargo test -p lofty --lib --tests -- --test-threads=1`, matching candidates
  `026`/`030`/`031`'s choice to avoid this checkout's pre-existing unrelated parallel-test
  flakiness): **647 passed, 0 failed, 73 ignored -> 647 passed, 0 failed, 73 ignored (no change,
  no new failures)**, both before and after the fix (646 from the project's own suite + 1 from a
  temporary, non-committed scratch test file used only for this candidate's own investigation,
  deleted before this candidate folder was finalized).
- `repro/` (the `+16` construction, standalone clone-based `just test`): silently loses the tag
  before the fix (`read_from -> Ok`, `ilst` present = `false`), returns a graceful
  `Err(FileParseError { ty: Some(Mp4), .. })` after the fix.

## What's not done / open questions

- This fix alone does **not** close candidate `026`'s/`031`'s separate `atom_info.rs` bug - that
  was landed independently as candidate `031`'s own `patch.diff`, but per this skill's own rule
  every candidate's fix is reverted before the session ends, so on any fresh checkout of current
  mainline, both bugs are present simultaneously and stack (up to 16 bytes of combined slack at
  this call site) until both candidates' patches are applied together by whoever actually adopts
  them.
- Same open items `030` already left unaddressed, not re-investigated this session:
  `properties.rs:489`'s `stbl`-within-`minf` `find_child_atom` call shares the same
  `find_child_atom` definition and was flagged in `IDEAS.md` as not yet examined for the same
  header-inclusive-budget shape; and only the case where `mdia` is `trak`'s *last* real child was
  checked (matching the fixture used), not a `trak` with additional children after `mdia`.
- Whether `030`'s and `026`'s (`031`'s) patches apply and pass the test suite cleanly *together*
  in one combined tree was not re-verified this session (each was verified independently against
  an otherwise-unpatched tree); `IDEAS.md` already carries a separate backlog note asking for this
  combined check (originally about `023`+`025`, not this pair specifically) - worth extending to
  this pair too.
