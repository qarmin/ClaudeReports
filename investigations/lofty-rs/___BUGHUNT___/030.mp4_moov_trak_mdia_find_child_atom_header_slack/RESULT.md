# mp4_moov_trak_mdia_find_child_atom_header_slack

`IDEAS.md` (2026-08-26 entry, left open by candidate `029`) asked whether `lofty/src/mp4/moov.rs`'s
own read-path `find_child_atom` call (`Moov::parse`'s `trak` -> `mdia` lookup, line 54,
`find_child_atom(reader, atom.len, *b"mdia", parse_options.parsing_mode)`) has the same
header-inclusive-budget shape candidate `029` already ruled out as dead code for
`ilst/write.rs`'s two write-path `find_child_atom` calls - noting this read-path call runs against
`AtomReader`'s own live `remaining_size` bound rather than a fixed snapshot, so `029`'s call-order
safety proof doesn't automatically transfer.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code-reading / control-flow tracing of `Moov::parse`, `find_child_atom`, `AtomInfo::read_inner`, `skip_atom`, and `AtomReader` | N/A | Confirmed line 54 passes `atom.len` (`trak`'s full, header-inclusive declared length) as the child-search budget, even though the reader has already been seeked past `trak`'s own 8-byte header by the point this call runs (unlike the sibling `udta` case two lines below at line 61, which correctly uses `atom.len - 8`). Unlike candidate `029`'s write-path calls, there is no earlier, tighter-budgeted re-parse of the same bytes before this call runs - `Moov::parse` calls `find_child_atom` directly as the very first parse of `trak`'s children, so `029`'s "always re-validated first" argument does not apply here. |
| Boundary-value construction: took a real fixture (`lofty/tests/taglib/data/has-tags.m4a`), where `trak`'s only content is `tkhd` (92 bytes) then `mdia` (1040 bytes, ending exactly at `trak`'s real end, which is exactly where `udta` - holding the real `ilst` tag - starts), and incremented only the on-disk `mdia` size field by exactly `ATOM_HEADER_LEN` (8), leaving every other byte (including all of `udta`/`meta`/`ilst`) untouched and unmoved | Debug build, temporary `#[test]`s (`lofty/tests/probe_030_scratch_test.rs`, not committed) calling the real public `Mp4File::read_from` | The mutated file still parsed successfully (`Ok`), but `ilst_tag` was `None` - the entire (untouched, still-present) tag was silently lost, no error surfaced anywhere. Repeated 3/3 runs, fully deterministic. |
| Isolating this candidate's contribution from candidate `026`'s separate, already-documented, still-unpatched `atom_info.rs` off-by-`ATOM_HEADER_LEN` bug (which independently leaks its own 8 bytes of slack at *every* `AtomInfo::read` call site, including this one) - by temporarily applying `026`'s already-verified fix (`if len > reader_size` in `atom_info.rs`) alongside this candidate's fix, then re-testing the same `+8` mutated file | Debug build, same scratch test | With both fixes applied, the `+8` mutated file was correctly rejected (`Err(FileParseError)`) instead of silently losing the tag - confirming this candidate's fix has a real, independent effect, separate from `026`'s. With *only* `026`'s fix applied (this candidate's line still unpatched), the same `+8` file *still* silently lost the tag - proving the bug at line 54 is real and not merely a restatement of `026`. |
| Isolating the reverse direction: on the real, fully unpatched mainline (`026`'s bug present, `atom.len - 8` fix *not* applied), constructing a `+16` mutation instead (`1040 -> 1056`, deliberately exceeding the maximum slack `026`'s bug alone can provide) | Debug build, same scratch test, 3/3 repeat runs | Fully unpatched mainline: `+16` also silently loses the tag (the two bugs' slack stacks: `8 (026) + 8 (this candidate) = 16`). With *only* this candidate's `atom.len - 8` fix applied (`026`'s `atom_info.rs` bug still present, unpatched): `+16` is correctly rejected with `Err`, while `+8` still slips through via `026`'s independent bug alone - isolating this candidate's fix's real, standalone effect on the actual current mainline. |

## Status

**Confirmed**

Deterministic, 3/3 repeat runs (debug build, standalone `repro/` against the project's public API)
against the real, unpatched mainline, plus 3/3 again via the fully standalone `just test`
clone-based reproduction (fresh `git clone` of the real repo at `base_commit`, unmodified working
tree, `+16` variant to isolate this candidate's own fix from candidate `026`'s separate one).

**Fully reproducible: Yes.** This is a single-participant finding (the library's own read path
mishandling a file it is handed) - no second participant/peer is involved, so section 11's "real
second participant" bar doesn't apply. `repro/` runs the real, public `Mp4File::read_from` entry
point end-to-end against a real crafted file (a genuine, real fixture with exactly one 4-byte field
modified, everything else byte-for-byte untouched), exactly as any real caller reading an MP4 file
would.

## Class

logic

## Root cause

`Moov::parse` (`lofty/src/mp4/moov.rs`) handles each top-level child of `moov` via
`reader.next()`, which leaves the reader positioned at the start of that child's *content*, past
its own 8-byte header. For a `trak` child (when `read_properties` is enabled, the default), it
then looks up `trak`'s `mdia` grandchild:

```rust
if let Some(mdia) =
    find_child_atom(reader, atom.len, *b"mdia", parse_options.parsing_mode)?
```

`atom.len` here is `trak`'s *full*, header-inclusive declared length - but the reader has already
consumed `trak`'s own header. `find_child_atom`'s internal running budget therefore starts 8 bytes
too generous, and stays 8 bytes too generous through every subsequent child it walks past inside
`trak` (each iteration only ever subtracts a real child's own `len`, never correcting the initial
offset). By the time `find_child_atom` reaches `mdia` (`trak`'s last real child in the fixture used
here), the effective budget it checks `mdia`'s declared size against is 8 bytes larger than what
is truly available inside `trak`. A crafted file can therefore declare `mdia.len` up to 8 bytes
larger than the real remaining space inside `trak` and still be accepted (two lines below, at line
61, the sibling `udta` case gets this right: `ilst_from_udta(reader, parse_options, atom.len - 8)`
correctly subtracts the header before using the parent's length as a child budget - `trak`'s case
at line 54 does not).

Once accepted, the caller does `skip_atom(reader, mdia.extended, mdia.len)`, which seeks forward by
`mdia.len - 8` from the position right after `mdia`'s own header. With `mdia.len` inflated, this
lands the reader up to 8 bytes past `trak`'s real end - i.e. into the *next* moov sibling's own
header (`udta`, in the fixture used here, which holds the real `ilst` tag). `Moov::parse`'s
top-level loop, `while let Ok(Some(atom)) = reader.next()`, then tries to parse moov's next child
starting from that now-misaligned position; whatever it finds there (garbage bytes from the middle
of `udta`'s header) either fails outright or is unrecognizable, and the loop simply exits - with no
error surfacing anywhere. Every atom after the corrupted one (here, all of `udta`/`meta`/`ilst`,
i.e. the entire tag) is silently dropped, even though every one of its bytes is still physically
present and completely untouched in the file. `Mp4File::read_from` returns `Ok` throughout, with no
indication to the caller that anything was lost.

This is a distinct bug from candidate `026`'s (a separate off-by-`ATOM_HEADER_LEN` bug shared by
every call to `AtomInfo::read`, already documented, not fixed by this candidate) and from candidate
`029`'s already-ruled-safe write-path calls (which are dead code specifically because
`AtomWriter::new_from_file` always re-validates the same bytes with a tight budget first - no such
prior validation exists before this read-path call). The two bugs (`026` and this one) independently
contribute up to 8 bytes of slack each and stack when both are present, as demonstrated above.

## Introduced by

238b40c4 "MP4: Remove `trak` module" (2022-10-11) - best-effort: `git log --follow -S` on the current `find_child_atom(reader, atom.len, ...)` call site (previously named `nested_atom`) lands on this commit as the earliest hit, which inlined `trak`'s parsing logic directly into `moov.rs`; not independently confirmed whether the removed `trak` module's own prior version already had the same header-slack bug before this refactor.

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
every subsequent child it walks past (`len.saturating_sub(atom.len)`), and the same accounting the
`udta` case already uses one branch below. Verified against more than the one triggering input: the
`+16` construction (deliberately exceeding candidate `026`'s independent slack) is correctly
rejected with the fix alone even though candidate `026`'s own separate bug is still present and
unpatched on this checkout; the original, untouched fixture still parses and retains its tag
identically before and after.

- Project test suite (`cargo test -p lofty --lib --tests -- --test-threads=1`, matching candidate
  `026`'s choice to avoid this checkout's pre-existing unrelated parallel-test flakiness): **646
  passed, 0 failed, 73 ignored -> 646 passed, 0 failed, 73 ignored (no change, no new failures)**,
  both before and after the fix (a temporary, non-committed scratch test file used only for this
  candidate's own investigation added 3 further passing tests to both runs; excluded from these
  counts since it isn't part of the project's own suite).
- `repro/` (the `+16` construction): silently loses the tag before the fix, returns a graceful
  `Err(FileParseError { ty: Some(Mp4), .. })` after the fix (3/3 runs each way, plus a clean `just
  test` run from a fresh standalone clone).

## What's not done / open questions

- Not checked whether the same header-inclusive-budget shape recurs anywhere else in `mp4/`
  outside the four already-audited call sites (`ilst/write.rs`'s two calls, cleared as dead code
  by candidate `029`; `properties.rs:489`'s `stbl`-within-`minf` call, sharing the same
  `find_child_atom` definition, not examined by this candidate).
- Candidate `026`'s own separate `atom_info.rs` bug remains unpatched (out of scope here, already
  tracked on its own); as shown above, it independently provides up to 8 bytes of the same kind of
  slack at every `AtomInfo::read` call site in the codebase, including this one, so a caller
  relying only on this candidate's fix should be aware a `+8`-scale version of the same silent-loss
  symptom is still reachable today via `026`'s bug alone until that one is also fixed.
- Only checked the case where `mdia` is `trak`'s *last* real child (as in the fixture used); did
  not check whether a `trak` with additional children after `mdia` changes the observed failure
  mode (still expected to desync the reader the same way, just consuming bytes belonging to a
  later sibling of `mdia` inside `trak` instead of `trak`'s own parent sibling - not verified this
  session).
