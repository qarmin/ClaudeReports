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

No particular sort order is implied - candidates land here in the order they were investigated. A
candidate that combines several suspected issues only appears here if explicitly requested and
investigated as its own row (this skill checks one bug per candidate by default - see `SKILL.md`).

| # | Status | Class | Fully reproducible | Candidate | Root cause (short) | Introduced by | Fixed by | Link |
|---|---|---|---|---|---|---|---|---|
