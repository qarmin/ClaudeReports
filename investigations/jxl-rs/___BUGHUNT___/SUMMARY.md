<!--
Copy into ___BUGHUNT___/SUMMARY.md. Keep it to this table - no prose, no severity column. Append
one row per candidate as soon as its RESULT.md is written, not batched at the end.

Markdown table hygiene (every row, no exceptions):
- One row = one physical line. Never let a cell's content contain a raw newline - if you're
  tempted to wrap a long root-cause summary across lines for readability, don't; write it as one
  long line instead. A newline inside a cell ends the table at that point (GFM has no multi-line
  cells).
- Never insert a blank line between two table rows, or between the header/separator and the first
  row - a blank line ends the table, splitting it into two separate tables (or a table plus loose
  text) when rendered. This is not a hypothetical: an earlier session on a real project appended
  rows with a blank line before each one (matching how they'd been drafted one at a time), which
  silently rendered as N separate one-row tables instead of one N-row table - always append with a
  single edit that inserts the new row directly under the last existing row, no surrounding blank
  line.
- Escape any literal `|` that must appear inside a cell's own text (e.g. quoting code that
  contains a pipe, or an "X or Y" phrase) as `\|` - an unescaped one is parsed as a new column
  boundary and silently shifts every following cell in that row.
- After writing or editing a row, sanity-check it renders as one row: the line should start with
  `|`, end with `|`, and contain exactly as many unescaped `|` characters as the header row.

**Structural rule, no exceptions: every physical line of prose lives above the table, and nothing
is ever appended below it.** Top to bottom this file is always: title, Status/Class/Fully
reproducible legend, sort-order note, any other free-text workspace note - then the table, then
nothing. If something needs to be said after the table already exists (a correction, a new
caveat noticed later), move it up into the prose block above the table in the same edit, don't
tack it on at the end "for now".
-->

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
| 001 | Confirmed | crash | Yes | NO_OUTPUT_PIPELINE_PANIC | Empty render-stage list indexed at `border_pixels_per_stage[0]` when all output formats are `None` | 939ceec Implement the low-memory render pipeline (#408) |  | [`001.NO_OUTPUT_PIPELINE_PANIC/RESULT.md`](001.NO_OUTPUT_PIPELINE_PANIC/RESULT.md) |
| 002 | Not reproducible | n/a | n/a | Unchecked accesses (predict.rs, ans.rs, channels.rs) | Invariants hold; ASAN+debug-assertions test run and fuzz clean | n/a |  | [`002.UNSAFE_UNCHECKED_ACCESS_AUDIT/RESULT.md`](002.UNSAFE_UNCHECKED_ACCESS_AUDIT/RESULT.md) |
| 003 | Not reproducible | n/a | n/a | Pixel format / option / chunking fuzz after 001 | Only the 001 panic found | n/a |  | [`003.PIXEL_FORMAT_OPTION_FUZZ_AFTER_FIX/RESULT.md`](003.PIXEL_FORMAT_OPTION_FUZZ_AFTER_FIX/RESULT.md) |
| 004 | Not reproducible | n/a | n/a | Colour type / sample type / premultiply / orientation consistency | All paths agree | n/a |  | [`004.OUTPUT_FORMAT_DIFFERENTIAL/RESULT.md`](004.OUTPUT_FORMAT_DIFFERENTIAL/RESULT.md) |
| 005 | Not reproducible | n/a | n/a | Chunked + flush_pixels vs single-shot decode | Bit-identical on all inputs | n/a |  | [`005.CHUNKED_VS_WHOLE_DECODE_DIFF/RESULT.md`](005.CHUNKED_VS_WHOLE_DECODE_DIFF/RESULT.md) |
| 006 | Not reproducible | n/a | n/a | Scalar/SSE4.2/AVX2/AVX-512 output | Same status, <=1e-3 relative difference | n/a |  | [`006.SIMD_LEVEL_DIFFERENTIAL/RESULT.md`](006.SIMD_LEVEL_DIFFERENTIAL/RESULT.md) |
| 007 | Inconclusive | n/a | n/a | 32-bit usize overflow (wasm32 replay) | No difference seen but oracle weak | n/a |  | [`007.WASM32_OVERFLOW_REPLAY/RESULT.md`](007.WASM32_OVERFLOW_REPLAY/RESULT.md) |
| 008 | Not reproducible | n/a | n/a | jxl-image-rs-integration adapter | No panics on 15k inputs | n/a |  | [`008.IMAGE_RS_INTEGRATION_REPLAY/RESULT.md`](008.IMAGE_RS_INTEGRATION_REPLAY/RESULT.md) |
| 009 | Confirmed | crash | Yes | set_pixel_format extra_channel_format length | Length never validated in set_pixel_format; assert_eq! in render pipeline panics later | 7a62f9d Use JxlOutputBuffer in SaveStage (#378) |  | [`009.SET_PIXEL_FORMAT_EC_LEN_PANIC/RESULT.md`](009.SET_PIXEL_FORMAT_EC_LEN_PANIC/RESULT.md) |
| 010 | Not reproducible | n/a | n/a | start_new_frame / scan_frames_only | No mismatch with sequential decode | n/a |  | [`010.SEEK_AND_SCAN_FUZZ/RESULT.md`](010.SEEK_AND_SCAN_FUZZ/RESULT.md) |
| 011 | Not reproducible | n/a | n/a | 32-bit (i686) test suite + corpus | Bit-identical to x86_64; only graceful ImageOutOfMemory | n/a |  | [`011.I686_32BIT_SUITE_AND_CORPUS/RESULT.md`](011.I686_32BIT_SUITE_AND_CORPUS/RESULT.md) |
| 012 | Not reproducible | n/a | n/a | aarch64 NEON vs scalar | Agrees within 1e-3 | n/a |  | [`012.AARCH64_NEON_VS_SCALAR/RESULT.md`](012.AARCH64_NEON_VS_SCALAR/RESULT.md) |
| 013 | Not reproducible | n/a | n/a | high_precision on/off | Agree within 3.2e-4 | n/a |  | [`013.HIGH_PRECISION_VS_DEFAULT/RESULT.md`](013.HIGH_PRECISION_VS_DEFAULT/RESULT.md) |
| 014 | Not reproducible | n/a | n/a | Allocation failure aborts | No abort in 1.1M execs under ulimit -v | n/a |  | [`014.ALLOC_FAILURE_ABORT/RESULT.md`](014.ALLOC_FAILURE_ABORT/RESULT.md) |
| 015 | Inconclusive | n/a | n/a | Simple vs low-memory pipeline, 16 vs 32-bit modular buffers | One assertion in test-only simple pipeline; no output mismatch | n/a |  | [`015.PIPELINE_AND_MODULAR_BUFFER_DIFF/RESULT.md`](015.PIPELINE_AND_MODULAR_BUFFER_DIFF/RESULT.md) |
| 016 | Not reproducible | n/a | n/a | Patches / splines parser unit fuzz | No crash with consistent reference frames | n/a |  | [`016.PATCHES_AND_SPLINES_PARSER_UNIT_FUZZ/RESULT.md`](016.PATCHES_AND_SPLINES_PARSER_UNIT_FUZZ/RESULT.md) |
