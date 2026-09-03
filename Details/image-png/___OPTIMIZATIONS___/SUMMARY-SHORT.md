<!--
Copy into ___OPTIMIZATIONS___/SUMMARY-SHORT.md. A filtered, condensed view of SUMMARY.md meant to
be small enough to paste whole into a GitHub issue/comment (GitHub truncates very long bodies) -
this file's whole point is staying well under that limit, so keep every cell terse.

Rows: only candidates whose SUMMARY.md Impact is exactly "Strong win" or "Small win" (including a
compound label like "Strong win + crash fix" - keep the whole label) AND whose SUMMARY.md "Fixed
by" cell is blank (i.e. the win is still available, not already applied upstream independently).
Everything else (No change, Mixed, Small/Strong regression, N/A, or a Strong/Small-win row that's
since been superseded upstream per a CORRECTION.md) is omitted here - it still lives in
SUMMARY.md, this file is a subset view, not a replacement. No "Fixed by" column exists in this
file: by construction every row here is still-available, so the column would always be empty -
point readers at SUMMARY.md if they want the already-applied-upstream history too. Re-check this
filter whenever a CORRECTION.md is added for a row already in this file - remove that row
immediately, don't wait for the next full regeneration.

This inherits SUMMARY_template.md's scope rule automatically (an app-level `.slint`/harness-only
authoring finding never gets a SUMMARY.md row in the first place, so it can't leak in here either) -
but if you ever hand-add a row instead of regenerating from SUMMARY.md, re-check that rule
yourself: don't let a large-looking number from a harness-only change end up in this file, which is
exactly the kind of thing meant to be pasted straight into a GitHub issue as "here's what got
faster" - a wrong entry there is a public, embarrassing mistake, not just an internal one.

**Candidate** and **Result** stay in the same two lanes as `SUMMARY_template.md`: **Candidate** is
the finding in plain language (what's slow, or what the patch changes, and why - zero numbers, e.g.
"Gradient stops collected into a Vec per primitive per frame"), **Result** is the technical,
measured evidence, condensed. Condense, don't copy-paste SUMMARY.md's Result cell verbatim: one
clause, the single headline number/effect, no methodology narration (paired-run counts, noise-floor
caveats, "on top of candidate X" chains, multi-round history) - that detail stays in SUMMARY.md and
the linked RESULT.md. If SUMMARY.md's own Candidate text is long, shorten it here too, but keep it
words-first (a description, not a label). **No Link column** - this file is meant to be pasted
as-is into a GitHub issue/comment in a different repo than the one the local `NNN.NAME/RESULT.md`
paths resolve in, so a relative link here would just be dead text; point readers at the full
`___OPTIMIZATIONS___/SUMMARY.md` (and its candidate folders) in the surrounding issue prose instead,
not per-row.

**Fix lines column** (`+A/-D`, e.g. `+14/-6`): total added/deleted lines in that candidate's
`patch.diff`, summed across every file it touches. Compute with `git apply --numstat patch.diff`
(columns: added, deleted, path) against the same base commit the patch was captured from - or
capture it via `git diff --numstat` at the same moment `patch.diff` itself is captured (step 4-5 of
the candidate loop), before the revert, so it never needs re-deriving later. This is a cheap,
self-sufficient "is this a big or small change" signal a reader can use without opening any local
file - unlike a bare yes/no flag, a line count means something on its own even pasted into a GitHub
comment in a different repo.

Same markdown table hygiene as SUMMARY_template.md applies: one row per physical line, no blank
line between rows, escape any literal `|` inside cell text as `\|`. Same structural rule too: all
prose lives above the table, nothing is ever appended below it.

Regenerate wholesale (don't hand-patch line by line) whenever SUMMARY.md changes in a way that
adds/removes/reclassifies a Strong-win/Small-win row - it's a filtered derivative of SUMMARY.md,
not an independently-maintained ledger, so SUMMARY.md is always the source of truth if the two
ever disagree.
-->

# Optimization wins (Strong win / Small win only)

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions) and the full methodology behind each number.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 001 | <Strong win / Small win> | <plain-language description of what's slow or what the patch changes, e.g. "Gradient stops collected into a Vec per primitive per frame"> | <+A/-D, e.g. "+14/-6"> | <one-clause headline number, e.g. "-12.4% frame time"> |
