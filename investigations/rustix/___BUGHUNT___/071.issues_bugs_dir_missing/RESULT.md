# issues_bugs_dir_missing

Meta-candidate, not a code suspect: the batch's source note claimed `___ISSUES___/bugs/` (the
triaged-issue output a prior `gh-issues-prs-check` run would normally leave behind, per `SKILL.md`'s
"Discovery" step 4 - "read (never run) `___ISSUES___/bugs/` if it exists") does not exist in this
repo checkout, so there are no confirmed-live-but-unreproduced issue files to triage from that
directory, and suggested re-checking if `gh-issues-prs-check` is ever run against this repo in the
future. Investigated as a verification-of-premise candidate (there is no code path to exercise, no
`unsafe`/parser/arithmetic surface here), matching the already-established precedent of candidates
028/029 in this same workspace (meta GitHub-issue-backlog candidates).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Filesystem check for `___ISSUES___/bugs/` and `___ISSUES___/` against a fresh `git worktree` checkout of current HEAD (287214b8), not just the possibly-cluttered main working directory | `git worktree add ___BUGHUNT___/.worktrees/071 HEAD`, then `ls ___ISSUES___/bugs/`, `ls ___ISSUES___/`, `find . -maxdepth 1 -iname "___*"` | Both paths confirmed absent (`No such file or directory`) in a clean checkout - not a stale/relative-path artifact of the main working directory, and not masked by anything else matching `___*` at the repo root besides `___BUGHUNT___` itself |
| `git log --all --diff-filter=D -- '___ISSUES___*'` / `git log --all --oneline -- '___ISSUES___*'` across all branches, to rule out "it existed and was deleted" | Ran against the full repo history (all refs) | No output at all - `___ISSUES___/` has never been a tracked path in this repository's history. Consistent with it being `gh-issues-prs-check`'s own ad hoc, untracked scratch output (never committed), not a removed feature |
| Re-ran this workspace's own substitute discovery mechanism (`SKILL.md`'s "GitHub issue/PR inspiration", used instead of a full `gh-issues-prs-check` triage tree) to check whether the *underlying gap* the missing directory would represent (an un-mined open-issue backlog) is actually still open | `python3 ~/.claude/skills/gh-issues-prs-check/fetch_issues.py --dir ___BUGHUNT___/__issues_raw raw-sync --since 1693` (`gh` authenticated as `qarmin`, network available) | `Raw-synced 0 issue(s), 0 PR(s) numbered > 1693` - `MAX_NUMBER_SEEN=1693`, matching `IDEAS.md`'s already-recorded watermark exactly. This workspace's `__issues_raw/` cache (104 issue + 53 PR JSON files) is current, and candidate 029 (same workspace, same day) already read every bug-shaped item up to #1693 and found each one either a duplicate of an already-`Confirmed` candidate or out of scope (doc lint, feature request) - so the backlog gap a missing `___ISSUES___/bugs/` would otherwise leave is, in practice, already closed by this workspace's own tooling |

Three approaches is the soft cap for this candidate loop; all three agree and none showed even a
partial signal pointing to any further gap worth digging into, so stopping here per `SKILL.md`'s
"3 clean negatives means move on."

## Status

**Not reproducible** - there is no bug being claimed here to reproduce; this is a confirmed, true
statement about the repo's state (`___ISSUES___/bugs/` genuinely does not exist, in both the main
working directory and a fresh worktree checkout of HEAD, and never has existed in tracked history),
and the underlying concern it points at (an un-mined open-issue backlog) is independently confirmed
*not* to be a live gap: this workspace's own `__issues_raw/`/`IDEAS.md` watermark mechanism already
covers the same role and is current as of this session (0 new issues/PRs past #1693, matching the
already-recorded watermark).

**Fully reproducible**: n/a (no `Confirmed` finding - this field is only required when Status is
Confirmed, per `RESULT_template.md`).

## Class

n/a (no `Confirmed` finding).

## Root cause

Not a code bug - there is no root cause in the rustix source to state. The directory's absence is
expected: `gh-issues-prs-check` (the skill that would populate `___ISSUES___/bugs/`) has never been
run against this repo in its full triage-tree form; this `___BUGHUNT___/` workspace instead uses its
own lighter-weight substitute (`SKILL.md`'s "GitHub issue/PR inspiration" step, writing only to
`__issues_raw/` + a watermark line in `IDEAS.md`), which was already run earlier this same day
(candidate 029) and is independently re-confirmed current by this candidate.

## Introduced by

n/a (no code bug; nothing to attribute to a commit).

## The fix

n/a - nothing to fix. No action is needed on the missing `___ISSUES___/bugs/` directory itself: per
`SKILL.md`, if a future session ever does run `gh-issues-prs-check` against this repo (producing a
real `___ISSUES___/bugs/` triage tree), a future discovery round should read it per step 4 of
"Discovery" at that time - nothing to pre-emptively build now.

- Project test suite: N/A, no code change was made or attempted by this candidate.

## What's not done / open questions

- `gh-issues-prs-check`'s full triage-tree categorization (bugs / new-features / tool-improvements /
  ui-ux / performance / too-hard-architectural / already-implemented / needs-info-or-invalid) was
  not run against this repo by this candidate - only its underlying raw-fetch utility
  (`fetch_issues.py raw-sync`) was re-invoked, consistent with `SKILL.md`'s explicit statement that
  this skill "does not call any other skill to do its work (not `code-audit-structured`, not
  `fuzz-bug-repro`, not `gh-issues-prs-check`)". Running the full `gh-issues-prs-check` skill itself
  (a separate, explicit invocation) would produce the actual `___ISSUES___/bugs/` tree the original
  note was looking for, if that's ever wanted independently of this bug-hunt workspace.
- This candidate does not re-triage the open-issue backlog itself (already done exhaustively by
  candidate 029 the same day, up to watermark #1693) - only re-verifies that the watermark is still
  current as of this run.
