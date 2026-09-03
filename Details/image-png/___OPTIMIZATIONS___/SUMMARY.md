<!--
Copy into ___OPTIMIZATIONS___/SUMMARY.md. Keep it to this table - no prose, no verdict column.
Append one row per candidate as soon as its RESULT.md is written, not batched at the end.

Markdown table hygiene (every row, no exceptions):
- One row = one physical line. Never let a cell's content contain a raw newline - if you're
  tempted to wrap a long Result across lines for readability, don't; write it as one long line
  instead. A newline inside a cell ends the table at that point (GFM has no multi-line cells).
- Never insert a blank line between two table rows, or between the header/separator and the first
  row - a blank line ends the table, splitting it into two separate tables (or a table plus loose
  text) when rendered.
- Escape any literal `|` that must appear inside a cell's own text as `\|` - an unescaped one is
  parsed as a new column boundary and silently shifts every following cell in that row.
- After writing or editing a row, sanity-check it renders as one row: the line should start with
  `|`, end with `|`, and contain exactly as many unescaped `|` characters as the header row.

**Structural rule, no exceptions: every physical line of prose lives above the table, and nothing
is ever appended below it.** Top to bottom this file is always: title, Impact legend, sort-order
note, any other free-text workspace note (a fresh-start explanation, a global caveat) - then the
table, then nothing. If something needs to be said after the table already exists (a correction, a
new caveat noticed later), move it up into the prose block above the table in the same edit, don't
tack it on at the end "for now" - a SUMMARY.md with a trailing note under the table is exactly the
mixed-configuration this rule exists to prevent, catch it in the same session it happens, not at a
later cleanup.

**Scope: code changes to the tested project (or a vendored dependency) only.** A candidate whose
finding is an app-level authoring pattern - "structure your own `.slint`/config/usage this way and
it's faster", demonstrated only in the test harness/example app, with no change to the tested
project's own engine/library code - does NOT get a row here, no matter how large the measured
number is. It didn't optimize the project being tested; it changed how the harness itself is
written. Its own candidate folder and `RESULT.md` still exist and are still accurate documentation -
they're just not "optimization results" in the sense this table tracks, and listing them here reads
as if the tested project got faster when it didn't. See `SKILL.md`'s "`patch.diff` is mandatory
whenever the fix is a change to the tested project" section for the same distinction applied to
`patch.diff`. This was violated for three candidates in one project's workspace (a row-structure
finding, an off-screen-relayout finding, and a `clip: true` finding - all real, all measured, all
listed here as "Strong win"/"Small win" before a user caught it) - don't repeat it: before adding a
row, ask "did this change the tested project's own source, or just the harness/example app that
exercises it?" and only add the row for the former.
-->

# Optimization results

**Impact** (derived from the Result column, for scanning at a glance - not a recommendation, this
classifies the measured magnitude/direction only, it doesn't decide what to do about it): **Strong
win** (large, confirmed improvement) - **Small win** (real but modest improvement) - **No change**
(null result / within noise) - **Mixed** (improves one thing, regresses another) - **Small/Strong
regression** (would make things worse) - **N/A** (pure measurement/characterization, no patch
compared against a baseline).

Sort by impact (biggest real win first; noise/regressions at the bottom). A candidate that
combines several individually-tested fixes only appears here if it was explicitly requested and
measured as its own row (this skill tests one fix at a time by default - see `SKILL.md`).

**Candidate** and **Result** are two different lanes, not two phrasings of the same thing.
**Candidate**: the finding itself, in plain language - what's slow, or what the patch changes, and
why - written so a reader understands the problem with zero numbers (e.g. "Every tracked property
read allocates a dependency node, every frame", not a short label like "dependency node pooling").
**Result**: the technical, measured evidence - the actual numbers (timings, allocation counts, RSS,
paired-run counts), what was compared against what, and any measurement caveats. A reader should be
able to grasp the finding from Candidate alone, and judge the evidence from Result alone, without
either column repeating the other.

**Fixed by** (only for a Strong-win/Small-win/Mixed row, best-effort, checked at row-creation time
and worth re-checking on later sessions/`git pull`s): blank/empty if the underlying inefficiency is
still present in current `HEAD` (the default, ordinary case, since this workspace's own patch is
always reverted, never applied for real) - or the short commit hash + subject of the commit that
already addressed it, if a maintainer independently applied the same (or an equivalent) fix
upstream since this candidate was measured. Finding one doesn't change the Impact cell (the
original measurement was real) but means a `CORRECTION.md` per `SKILL.md`'s "When a later
candidate invalidates an earlier one's numbers" should exist alongside it, and the row should come
out of `SUMMARY-SHORT.md` if it's there - see that template.

| # | Impact | Candidate | Result | Changes behavior? | Fixed by | Link |
|---|---|---|---|---|---|---|
| 001 | <Strong win / Small win / No change / Mixed / Small regression / Strong regression / N/A> | <plain-language description of what's slow or what the patch changes, e.g. "Gradient stops collected into a Vec per primitive per frame"> | <e.g. "-12.4%, paired 14/14" / "within noise (+0.3%)" / "+8% regression"> | <"no, bit-exact" / "yes - see RESULT.md"> | <blank / abc1234 short subject, see CORRECTION.md> | [`001.NAME/RESULT.md`](001.NAME/RESULT.md) |
