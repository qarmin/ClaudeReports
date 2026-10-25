# Bug-hunt results

**Status**: **Confirmed** (a real, run reproduction exists) - **Not reproducible** (checked,
couldn't confirm - see the candidate's `RESULT.md` for exactly what was tried) - **Inconclusive**
(a signal was seen but fell short of `CONFIRMATION.md`'s bar). **Class** (only when Confirmed):
purely descriptive bug category - `crash` / `UB` / `logic` / `data-race` / `memory-safety` /
`security` / other. **Fully reproducible** (only when Confirmed): **Yes** if the repro actually runs
the real affected entry point end-to-end (the real binary, or the real public API a genuine caller
would use), driven by a real second participant when the finding is about one participant's
reaction to another's input (see `CONFIRMATION.md` section 11) - **No** only for the rare, explicitly
justified exception where a genuine end-to-end repro was impractical this session (the
justification lives in that candidate's `RESULT.md`, not here). **A `Confirmed` row without a
`Yes`/justified-`No` in this column should not exist** - a candidate whose only repro fakes a second
participant's behavior instead of actually running it does not get written up as `Confirmed` at all,
per `CONFIRMATION.md` section 11; it stays `Inconclusive` or goes back to `IDEAS.md` until a real
end-to-end repro is built. **Introduced by** (only when Confirmed, best-effort): the short commit
hash (`git log -S`/`git blame`/`git bisect` - see `COMMANDS.md`) that introduced the buggy code, if
it was worth tracking down and could actually be identified this session - written as **plain text,
not inside backticks** (e.g. `abc1234 "commit subject"`, not `` `abc1234` "commit subject" ``) so
GitHub/GitLab auto-links the hash to the commit instead of rendering it as inert code - or
`not determined` (with the reason briefly noted in `RESULT.md`: too costly to bisect this session,
history unavailable, present since the file's initial commit, etc.) which is a complete, honest
answer, not a gap to apologize for. **Fixed by** (only when Confirmed, best-effort, checked at
row-creation time and worth re-checking on later sessions/`git pull`s): blank/empty if still
present in current `HEAD` (the default, ordinary case) - or the short commit hash + subject of the
commit that fixed it, if the bug has since been fixed **upstream, independently of this
workspace's own (always-reverted) patch** (a maintainer applied their own fix, a dependency bump
happened to fix it, etc.), same plain-text-not-backticks format as **Introduced by**. Finding one
means the Status cell stays `Confirmed` (it was real) but a `CORRECTION.md` per `SKILL.md`'s "When
a later candidate invalidates an earlier one's confirmation" should exist alongside it stating the
fixing commit and that the original repro no longer reproduces against current `HEAD` - this
column is the at-a-glance pointer to that `CORRECTION.md`, not a replacement for it. None of these
columns is a priority ranking - see `SKILL.md`'s "the one rule that matters most".

Rows are kept sorted by the **#** column (ascending) - append in whatever order candidates get
investigated, then run `just tidy` (or `just sort-summaries`) to reorder; sorting is mechanical and
must never be done by hand. The **#** is a plain number so it sorts numerically: a string sort would
render `1, 10, 11, 2`. The order carries no meaning beyond readability - it is not a priority
ranking, see `SKILL.md`'s "the one rule that matters most". A
candidate that combines several suspected issues only appears here if explicitly requested and
investigated as its own row (this skill checks one bug per candidate by default - see `SKILL.md`).

| # | Status | Class | Fully reproducible | Candidate | Root cause (short) | Introduced by | Fixed by | Link |
|---|---|---|---|---|---|---|---|---|
| 001 | Confirmed | other | Yes | zune-png `crates/zune-png/src/decoder.rs` module-level and `more_frames` doctest examples fail to compile | Doc examples reference a free function `post_process_image_apng` that was never implemented (the real APNG compositing API is the `ApngContext` struct/`process_frame` method) and a second example uses an undefined `decoder` variable outside any setup, so `cargo test --doc -p zune-png` fails with E0432/E0425 on 2 of 11 doctests | 6332c284 "png: Improve docs" (2026-05-15) | | [`001.PNG_DECODER_DOCTEST_PHANTOM_APNG_FN/RESULT.md`](001.PNG_DECODER_DOCTEST_PHANTOM_APNG_FN/RESULT.md) |
| 002 | Confirmed | logic | Yes | zune-jpeg `decoder::planar_layout_helpers::set_options_refreshes_cached_dispatch` test compares `idct_1x1_func` fn-pointer addresses obtained from two different call sites | `idct_int_1x1` is `#[inline]`, so under the crate's default 16-codegen-unit `--release` profile rustc emits it as two separate local symbols 158720 bytes apart (confirmed via `nm`), so the address `assert_eq!` fails deterministically (8/8 runs); passes under a debug build | ca04b109 "fix(jpeg): reset output state across raw replays" | | [`002.JPEG_IDCT_1X1_FNPTR_CODEGEN_UNIT_DUP_TEST/RESULT.md`](002.JPEG_IDCT_1X1_FNPTR_CODEGEN_UNIT_DUP_TEST/RESULT.md) |
| 003 | Confirmed | logic | Yes | zune-inflate `StreamingDecoder::decode_chunk` never resets `StreamingBitStreamReader::position` (an index into the current chunk slice) between calls, so any decode spanning more than one `decode_chunk` call starves silently | position stays stale from the previous chunk, so `refill`/`refill_slow` reads zero further bytes even on fresh input; reproduces on a minimal 1-byte payload split into 1-byte chunks (Error: CorruptData) and on 30/59 of the crate's own lib tests; zune-png's `process_zlib_stream` is unaffected only because it independently calls the undocumented `reset_position()` workaround itself | 2901c37b "png: Initial streaming version" | | [`003.ZUNE_INFLATE_STREAMING_POSITION_RESET/RESULT.md`](003.ZUNE_INFLATE_STREAMING_POSITION_RESET/RESULT.md) |
