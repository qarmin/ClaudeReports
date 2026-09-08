---
name: refresh-reports
description: Refresh the published code-audit reports in this repo against the projects' current code - re-clone each project, drop findings whose code changed, fix up moved locations, fill in missing metadata (difficulty, project description, commit), and re-render the HTML with today's toml_to_report.py. Works in resumable batches, mechanically wherever possible. Usage - /refresh-reports [count | project names]
user-invocable: true
---

# Refresh published reports

The reports in this repo were generated over many months by `code-audit-structured`,
against code that has since moved on. This skill brings them back up to date
**without re-auditing anything**: a finding is kept only if the code snippet at
its first location - the site the description is about - still exists in the
project's current checkout. Further locations that no longer match are pruned
from the finding, so a multi-site finding survives its secondary sites being
refactored away.

Scope, deliberately narrow:

- **In scope:** dead findings out, moved locations fixed, missing `difficulty` /
  off-schema types re-decided, project metadata (`description`, `commit`, `branch`)
  completed, HTML re-rendered in the current format.
- **Out of scope:** re-judging whether a finding is a false positive, adding new
  findings (that is a fresh `code-audit-structured` run), and everything under
  `Details/` (bug-hunt and optimization workspaces have their own tooling).

## Cost model - read this before deviating

Verification is mechanical and free: `verify_report.py` compares the snippet
embedded in the published report against the fresh checkout. There are 45k+
findings across ~280 reports; re-reading them with an LLM is not an option. Only
two things may consume tokens:

1. resolving a project's git URL when the report is too old to carry one, and
2. assigning `difficulty` (and any off-schema type) to findings that survived.

Never read project source to "double-check" a mechanical result, and never read
a `findings.toml` or report HTML into context - they are megabytes each.

## Workflow

Run everything from the repo root. `just refresh` is `prepare` with defaults.

### 1. Prepare a batch

```
python3 .claude/skills/refresh-reports/refresh.py prepare --limit 10
```

Alphabetical order, skipping projects refreshed within the last 30 days. Per
project it clones (`--depth 1`), rebuilds a `findings.toml` from the report,
verifies every finding, and then either finishes the project outright or parks
it for the difficulty pass. State lands in `tmp/refresh-state.json` after every
single project, so an interrupted run is resumed by re-running the same command.

Each project comes back with one of these statuses:

| Status | Meaning | What you do |
|---|---|---|
| `done` | rendered and installed as `Project_<today>.html` | nothing |
| `done-empty` | every finding was fixed upstream, report removed from the repo | nothing |
| `awaiting-difficulty` | verified, waiting on the LLM pass | step 2 |
| `needs-url` | report predates embedded metadata and no URL is on file | step 3 |
| `needs-decision` | a safeguard tripped, report left untouched | step 4 |
| `clone-failed` / `render-failed` | tooling or network problem, report left untouched | report it to the user |

### 2. Difficulty pass (the only LLM step)

`prepare` writes `tmp/work/<project>/batches/batch-NNN.json` (≈50 findings each,
carrying only type/severity/description/fix/locations - no source code).

Spawn **one subagent per batch file**, in parallel, with this prompt (substitute
the batch path). Do not do this inline - batch payloads are large and belong in a
subagent's context, not yours:

> You are assigning a `difficulty` label to code-audit findings. Work only from the data in the batch file - do NOT clone, read, or search any project source. Do not use any tool other than Read and Write.
>
> 1. Read the batch file: `<ABSOLUTE PATH TO batch-NNN.json>`. Each entry has `idx`, `needs` (which fields to decide), `type`, `severity`, `description`, `fix`, `locations`.
> 2. For every entry whose `needs` contains "difficulty", pick exactly one of:
>    - `"very_easy"`: mechanical one-liner. The `fix` shows the literal before/after change; someone with zero knowledge of the codebase can verify and apply it by reading only the quoted lines. If the fix cannot show an exact replacement, it is not very_easy.
>    - `"easy"`: simple fix. Needs a few lines of surrounding context but no deep knowledge of the module or tech stack; an unfamiliar reviewer applies it in under 10 minutes.
>    - `"medium"`: moderate effort. Requires understanding the module's invariants, data flow, or API contract, but the scope is contained to one function or file.
>    - `"hard"`: complex fix. Requires deep knowledge of the architecture, a third-party library's internals, or platform-specific behavior; likely touches several files or subsystems; real risk of introducing new bugs.
> 3. For every entry whose `needs` contains "type", also pick one type from exactly this list: panic, overflow_underflow, memory_leak, use_after_free, copy_paste, logic_error, invalid_comment, dead_code, performance, incorrect_error_handling, api_misuse, duplicate_code, architecture, other. (`other` covers data races, TOCTOU, missing locks and unclosed handles. Never answer `custom` - that type belongs to findings the user imports themselves.)
> 4. Write your answer to `<same path with -result.json suffix>` in exactly this shape (include `"type"` only for entries that needed one):
>    `{"results": [{"idx": 0, "difficulty": "easy"}, {"idx": 3, "difficulty": "medium", "type": "logic_error"}]}`
>
> Every `idx` from the batch that needed a decision must appear exactly once. Do not invent indices. Reply with one line: how many findings you labelled and the count per difficulty value.

Then complete every parked project:

```
python3 .claude/skills/refresh-reports/refresh.py finish
```

`finish` merges the answers, renders, installs the HTML and deletes the
checkout. `severity` is never touched - re-rating impact without reading code
would be guessing. A finding left without an answer renders as `medium`.

### 3. `needs-url` - resolve the repository

`projects.toml` gets a stub entry with a `note`. Resolve the URL like this, in
order:

1. Look at the paths the report references - they identify the project cheaply
   and reliably. `tmp/work/<project>/baseline.json` holds them:
   `python3 -c "import json;print(sorted({l['ref'].rsplit(':',1)[0] for f in json.load(open('tmp/work/<project>/baseline.json'))['findings'] for l in f['locations']})[:15])"`
   (For example `src/ansi.rs` + `src/params.rs` under the name `vte` means the
   Rust crate `alacritty/vte`, not GNOME's C library of the same name.)
2. Confirm the candidate resolves and serves one of those paths:
   `git ls-remote --exit-code -h <url>` and
   `curl -s -o /dev/null -w '%{http_code}' https://raw.githubusercontent.com/<owner>/<name>/HEAD/<a path from the report>`
3. When the paths are too generic to identify the project (`src/decoder.rs` and
   friends exist in every decoder), search for a line of its code instead - the
   report embeds the exact source it was written against:
   `gh search code "<a distinctive line or constant from baseline.json>" --limit 5`
   This is decisive where path guessing is not: for `rust_h264`, two plausible
   H.264 crates served the report's paths and both were wrong, while searching
   for `DC_RASTER_TO_BLOCK_P` named `roticv/rust_h264` outright.
4. Only if none of that identifies the project, ask the user. Never guess: a
   wrong URL is exactly what the safeguards below exist to catch, and they cost
   a full clone to trigger.

Write `git_url` (and `repo_url`) into `projects.toml`, drop the `note`, and
re-run `prepare --only <project>`. The URL is durable - later cycles reuse it.

### 4. `needs-decision` - a safeguard tripped

Verification refuses to write a result when more than 90% of findings drop, or
when fewer than half the referenced paths exist in the checkout. That pattern
means "wrong repo, wrong branch, or a rewrite from scratch" far more often than
it means "everything got fixed". The report is left untouched.

Check `tmp/work/<project>/stats.json` plus the URL and branch in
`projects.toml`. If the checkout is genuinely right (upstream really did rewrite
the tree), re-run with `--only <project> --force`. If it is the wrong repo, fix
`projects.toml` first. If you cannot tell, hand the case to the user with the
numbers - do not force it.

### 5. Commit the batch

Local commit only, never push - the repo is public and the user reviews before
publishing.

```
git add -A && git commit -m "Refresh N reports (X findings kept, Y dropped)"
```

## Unattended runs

There is no cron built in. To grind through the backlog in one sitting, drive
this skill from `do-until` or `/loop`, one `prepare --limit 10` + difficulty pass
+ `finish` + commit per iteration. Two rules for such a run: stop on
`needs-decision` rather than forcing it, and keep an eye on
`clones_on_disk_mb` in the `prepare` output - a batch that happens to contain
several giants (libreoffice, wine, mesa) is better run with `--limit 3`.

## Files

| File | Role |
|---|---|
| `refresh.py` | orchestrator: selection, cloning, state, rendering, installing |
| `extract_report.py` | published HTML to `findings.toml` + `baseline.json` (snippets) |
| `verify_report.py` | snippet matching, location fixups, safeguards, project metadata |
| `difficulty_pass.py` | batches out undecided findings, merges the answers back |
| `report_toml.py` | shared enums, TOML I/O, snippet helpers |

Rendering always goes through `~/.claude/skills/code-audit-structured/toml_to_report.py`
(override with `--renderer`), which is what makes a refreshed report come out in
today's format rather than the one it was born in. Every script takes `test` as
its first argument to self-test, and `verify_report.py` also has `--dry-run`.

Only the schema `code-audit-structured` writes today is accepted, and nothing is
rewritten behind your back. A finding with no `difficulty`, or with a type
outside the current list, goes to the difficulty pass to be decided; a severity
outside the current list aborts that report, because silently rewriting it would
misreport how bad the finding is.

Finding IDs (`PANIC_1`, `CPY_36`, ...) are renumbered by the renderer, so they
shift when findings drop out. That is expected and accepted here.

Installing a refreshed report normally deletes the project's older files. A file
named in that project's `keep` list in `projects.toml` is the exception and is
never deleted - some older reports are linked to from outside this repo, so the
URL has to keep working:

```toml
["wine"]
git_url = "https://gitlab.winehq.org/wine/wine.git"
keep = ["wine_20260823.html"]
```

`find_reports.py` honours the same list, so collecting a fresh audit does not
remove a pinned file either. A pinned file is never used as the source of a
refresh: the newest report for the project is.
