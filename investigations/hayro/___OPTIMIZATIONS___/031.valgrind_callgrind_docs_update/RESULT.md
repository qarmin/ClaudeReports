# Verify/promote candidate 020's valgrind/callgrind + `ulimit -n 4096` finding into README.md and COMMANDS.md

This is a documentation-consistency task, not a performance candidate: candidate 020 discovered
that `valgrind`/`callgrind` is available in this sandbox (`perf` is not) and that the sandbox's
default `ulimit -n` (2,147,483,584) makes `valgrind` fail outright with `Private file creation
failed` unless lowered first. Candidate 020's own `RESULT.md` ("What's not done") flagged this as
"only recorded in this RESULT.md and the candidate's own justfile - worth promoting to
COMMANDS.md/README.md if valgrind gets used again in a future session." No source/build/measure
cycle applies here - there is no code to patch, build, or benchmark.

`rustc`/`cargo` versions: not applicable, no compiled artifact evidence produced by this
candidate.

## What was tried

Checked both target files for the promised content before writing anything new:

- `___OPTIMIZATIONS___/README.md`, "Instrumentation added for this workspace" section: already
  contains a full paragraph ("**Update (candidate 020):** `valgrind`/`callgrind` **is** available
  in this sandbox...") documenting availability, the exact `ulimit -n` gotcha and workaround
  command, and a pointer to `020.resolved_font_cache_callgrind/RESULT.md` + its `justfile` for a
  full worked example (probe binary, baseline/patched callgrind runs, `callgrind_annotate
  --auto=yes`).
- `~/.claude/skills/perf-optimize/COMMANDS.md`: already contains a dedicated "valgrind/callgrind
  (when available and perf is not)" section - availability check (`which valgrind`), the exact
  `bash -c 'ulimit -n 4096; valgrind --tool=callgrind ...'` workaround with a comment explaining
  why it's needed, `callgrind_annotate` usage (flat and per-line/`--auto=yes`), a
  baseline-vs-patched diff workflow note, and a bonus `cachegrind`/`--branch-sim`/`--cache-sim`
  subsection.

Both files already state the exact invocation and the `ulimit -n 4096` workaround this candidate
was asked to add - nothing was missing, so no edit was made to either file.

Re-verified the workaround is still accurate on this machine right now (not just trusted from
candidate 020's writeup), via a minimal, unrelated C smoke binary run under `valgrind
--tool=callgrind` with and without the fix, each invocation wrapped in the shared build lock:

```
$ ulimit -n
2147483584

$ flock .../.locks/build.lock -c "valgrind --tool=callgrind --callgrind-out-file=cg.out -- ./t"
--PID:0:libcfile Valgrind: FATAL: Private file creation failed.
   The current file descriptor limit is 2147483572.
--PID:0:libcfile Exiting now.

$ flock .../.locks/build.lock -c "bash -c 'ulimit -n 4096; valgrind --tool=callgrind --callgrind-out-file=cg2.out -- ./t'"
==PID== Callgrind, a call-graph generating cache profiler
==PID== Collected : 132216
(succeeds, writes cg2.out)
```

## Result

No documentation change was needed - both README.md's "Instrumentation added for this workspace"
section and COMMANDS.md's "valgrind/callgrind" section already contain the working invocation and
the `ulimit -n 4096` workaround, evidently added during (or shortly after) candidate 020's own
session despite that candidate's own "What's not done" section describing the promotion as
outstanding. Freshly re-ran the documented failure/fix pair on this machine and it reproduces
exactly as documented: default `ulimit -n` (2,147,483,584) -> `Private file creation failed`;
`ulimit -n 4096` in the same shell -> succeeds.

This candidate has no wall-clock/RSS/allocation measurement to report - it is a documentation
completeness check, not a performance change.

## Does it change behavior/output?

- N/A - measurement/investigation only, no change to the tested project's (hayro's) own source or
  a vendored dependency. No `patch.diff` for hayro is included, correctly: there was nothing to
  patch in hayro's source, and the two files this candidate concerns (`___OPTIMIZATIONS___/README.md`,
  the skill's own `COMMANDS.md`) were already accurate and were not modified either, so there is
  no diff to capture there either.
- No `cargo build`/`cargo test` cycle ran for this candidate (nothing in hayro's own source
  changed) - project test suite before/after: **N/A**, no source touched.
- **Is this a breaking change?** No.

## Why the result looks the way it does

Candidate 020 apparently already carried out the promotion its own "What's not done" section
described as outstanding (or a session between 020 and this one did) - both target files read as
fully updated already, word-for-word consistent with what candidate 020 measured (same `ulimit -n`
value, same workaround command, same pointer back to 020's own worked example). This candidate's
job reduced to verifying that claim for real rather than trusting it blindly, which it did via a
fresh, independent repro of the failure/fix pair on this machine.

## What's not done / open questions

- Nothing left to do on the documentation itself - both files already match what this candidate
  was asked to add.
- Not independently checked whether every other session/skill file that might reference profiling
  tooling (e.g. a project-specific `projects/hayro.md`, if one exists) also mentions this - out of
  scope for what was asked (README.md + COMMANDS.md specifically).
