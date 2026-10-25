# Ideas

Unchecked suspects only — see `SUMMARY.md` for what's already been investigated.

**This session's known-issue list is NOT a candidate backlog.** The 33 open, in-scope
`I-miscompile` GitHub issues for this repo live in `_reference/known_miscompile_issues.md`
instead of here, with a short "still reproduces?" recon column — they exist purely as
pattern-mining input for the differential-fuzzer generator built this session (see
`README.md`), not as candidates to be promoted into numbered `NNN.NAME/` folders with their own
`RESULT.md`/`SUMMARY.md` row. Per the user's explicit instruction: confirming an already-known,
already-filed issue is not a "finding" in this workspace — only a genuinely NEW miscompilation
the fuzzer turns up gets a candidate folder and a `SUMMARY.md` row.

Last GitHub issue/PR checked for inspiration: **#163911** (repo: rust-lang/rust) — seeded
directly from `gh issue list --label I-miscompile --state open` on 2026-10-07, see
`_reference/known_miscompile_issues.md`.

## Backlog (not yet investigated)

Empty at session start. A new row here is a genuinely novel suspect noticed while building or
running the differential fuzzer (not from the known-issue list above) that hasn't been triaged
into its own candidate folder yet.

| Added | Suspect | Why it looks suspicious | Source |
|---|---|---|---|
