# Record toolchain version (rustc/cargo) in every future RESULT.md that measures compiled-code artifacts

This candidate is a process/documentation fix to the `perf-optimize` skill itself, not a
performance patch to hayro's source. It updates
`~/.claude/skills/perf-optimize/templates/RESULT_template.md` so that every future candidate whose
evidence includes a compiled-code artifact (call count, instruction count, disassembly-based
claim, inlining/vectorization observation - not a plain wall-clock timing) records
`rustc --version`/`cargo --version` up front.

## Why

Candidate 013 (`hayro-jpeg2000_idwt_horizontal_suspect_investigation`) and candidate 014
(`hayro-jpeg2000_idwt_horizontal_restructured_inlining`), both in this same workspace, each
counted real `__fearless_simd_kernel` calls in a compiled `irreversible_filter_97i_horizontal_simd`
binary - 92 in 013, 28 in 014 for what candidate 014 believed was the identical unsplit function.
Neither recorded `rustc --version` at measurement time. Candidate 016
(`hayro_jpeg2000_idwt_horizontal_call_count_reconciliation`) spent a full separate investigation
re-running both candidates' own stated build methods on one session's current toolchain, got 28
both ways (instruction-for-instruction identical disassembly after address normalization), ruling
out build method, the diagnostic module's presence, and `fearless_simd` version as explanations.
The remaining, unruled-out explanation left standing was `rustc`/LLVM toolchain drift between the
two original sessions (`hayro` has no `rust-toolchain.toml` pin, and this machine's `stable`
rustup toolchain has an mtime showing it was updated at some point) - but this could not be
confirmed or refuted after the fact, precisely because neither original session recorded which
`rustc` it used. See `___OPTIMIZATIONS___/016.hayro_jpeg2000_idwt_horizontal_call_count_reconciliation/RESULT.md`
for the full reconciliation writeup, and `projects/hayro.md`'s "Environment gotchas" section
(already updated by that session) for the standing note this candidate now also encodes into the
template itself, so it applies to every project this skill is used on, not just hayro.

## What was tried

Not applicable in the usual "implementation variant" sense - there is no code being measured or
compared here, so there is no baseline/patched timing table.

The concrete change: `templates/RESULT_template.md`'s leading HTML comment (already used to remind
every candidate about the `patch.diff` mandatory-vs-N/A decision) gained a second paragraph
instructing that `rustc --version`/`cargo --version` (or the equivalent for whatever toolchain is
in use) be recorded whenever a candidate's evidence includes a compiled-code artifact, with a one-
sentence pointer to the 013/014/016 story as the concrete reason this matters. A wall-clock-only
candidate is explicitly exempted - the instruction is scoped to compiled-artifact evidence
(call/instruction counts, disassembly, inlining/vectorization claims), not every candidate.

This session's own toolchain, recorded here as a demonstration of the new instruction being
followed:

```
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
```

## Result

Not a performance measurement - Impact: N/A. The deliverable is the template edit itself
(`~/.claude/skills/perf-optimize/templates/RESULT_template.md`), which is outside the hayro repo
and applies to every future project this skill runs against, not just hayro. No hayro source,
test, or benchmark was built, run, or changed for this candidate - confirmed via a scratch git
worktree (`git worktree add /tmp/hayro-worktree-033 HEAD`) that showed a clean status with nothing
to do, then removed. The main hayro working tree was never touched.

## Does it change behavior/output?

- **Bit-exact?** N/A - no hayro source, test, or compiled artifact was touched by this candidate.
- **Is this a breaking change?** No. It only adds a reminder paragraph to a skill template file
  consumed by future perf-optimize sessions; it changes no runtime behavior of hayro and no public
  API/file format.
- Project test suite: not run for this candidate - there is no source change to hayro to
  invalidate or re-verify. (N/A, not "skipped": the candidate loop's build/test/measure steps
  apply to a code change under test, and this candidate makes none.)
- **patch.diff**: correctly absent. This candidate's fix is a documentation change to the
  perf-optimize skill's own template (`~/.claude/skills/perf-optimize/templates/RESULT_template.md`),
  which lives outside the tested project (hayro) and is not a dependency hayro vendors - per
  SKILL.md's own "patch.diff is mandatory whenever the fix is a change to the tested project (or a
  dependency it vendors)", a change to the harness/tooling itself does not get a patch.diff against
  the project being optimized. Fabricating one against hayro's source to fill the slot would be
  incorrect, per the same section's explicit warning against that.

## Why the result looks the way it does

This candidate's "fix" is entirely outside the code being optimized - it targets a gap in this
skill's own record-keeping discipline that a previous investigation (016) surfaced but did not
itself close (016 recorded the gotcha in `projects/hayro.md`, a hayro-specific file, but never
updated the template that every future candidate - on hayro or any other project - actually copies
from). Closing it here means the instruction now travels with the template itself rather than
depending on every future session having separately read `projects/hayro.md`'s prose.

## What's not done / open questions

- This does not retroactively add toolchain-version notes to any existing `RESULT.md` in this or
  any other project's workspace (013/014/016 already carry the story of the gap in their own
  prose; 016 in particular already documents the missing-version issue explicitly). Only new
  candidates going forward are affected.
- The instruction is scoped to `rustc --version`/`cargo --version` since every project this skill
  has been used on so far is Rust; if this skill is ever used on a C/C++ project, the same
  paragraph's "or the equivalent for the toolchain in use" phrasing covers it, but no C/C++-
  specific example command was added since none of this skill's current `projects/*.md` files
  need one yet.
