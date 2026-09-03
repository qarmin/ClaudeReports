# Ideas

Unchecked suspects only - see `SUMMARY.md` for what's already been investigated.

Last GitHub issue/PR checked for inspiration: **#197** (repo: orxfun/orx-parallel)

## Backlog (not yet investigated)

Consumed top-to-bottom by default when a session isn't given a specific target - see `SKILL.md`.
Seeded 2026-08-30 from an existing (unverified) code-audit-structured static-audit pass already
present in the repo at `___FINDINGS___/findings.toml` (34 raw findings) - this session's job is to
turn the bug-shaped subset of those into real reproductions, not to re-derive them from scratch.
Findings typed `dead_code`/`duplicate_code`/`invalid_comment`/`copy_paste`/`performance` were left
out of this backlog (not bug-shaped in the sense this skill checks); `architecture` and one
`other` finding about a dangling JS timeout were also left out as not crash/UB/logic-correctness
issues. Ordered here roughly by expected ease of building a real native repro first, hardest
(wasm32 + atomics + real browser workers) last - not a priority ranking, just consumption order.

| Added | Suspect | Why it looks suspicious | Source |
|---|---|---|---|

