<!--
Copy into NN.CANDIDATE_NAME/RESULT.md. No "verdict" line (applied/deferred/etc) - just facts.
The reader (usually the project maintainer, possibly not you) decides what to do with this.

Before finishing this file: does this folder need a patch.diff? Only if the fix is a change to
the TESTED PROJECT's own source (or a dependency it vendors) - see SKILL.md's "patch.diff is
mandatory whenever the fix is a change to the tested project". If so, it must exist. If instead
this is a measurement-only investigation, or an app-level `.slint`/example-app authoring pattern
demonstrated only in the test harness, there is correctly NO patch.diff - never fabricate one
against the harness itself to fill the gap. State explicitly below in "Does it change
behavior/output?" which of these two states this candidate is in - don't leave it implicit.

If this candidate's evidence includes any compiled-code artifact (a call count, an instruction
count, a disassembly-based claim, an inlining/vectorization observation) - not just a wall-clock
timing - record `rustc --version` and `cargo --version` (or the equivalent for the toolchain in
use) somewhere in this file, e.g. as the first line of "What was tried". A wall-clock-only
candidate doesn't need this. This exists because candidates 013/014 (hayro project) each measured
a real call count from a compiled binary without recording the toolchain that produced it; the two
numbers later turned out to disagree 92-vs-28, and candidate 016 had to spend a whole separate
investigation reconciling them - concluding the most likely explanation was undetectable in
hindsight: unrecorded `rustc`/LLVM version drift between the two original sessions.
-->

# <candidate name>

One or two sentences: what this touches and why it looked worth trying (a hot spot from
profiling, a heuristic from HEURISTICS.md, a lead from IDEAS.md).

## What was tried

<Table: implementation variant | timing (single-core, `taskset -c N`) | timing (multi-core) |
notes. Include the baseline as the first row. If several variants were tried and lost, list all
of them - a "only the winner's number" table looks like a win that wasn't tried hard enough.
"Performance" isn't only time - if the candidate's mechanism (avoiding a clone/allocation, reusing
a buffer) plausibly affects memory, add a peak-RSS or allocation-count column too (or a second
table) even when time is the primary metric; a memory-only win with no time change is still a real
result, not a null one - don't drop it just because it doesn't fit a time-only table.>

| Variant | single-core p50 | multi-core p50 | notes |
|---|---|---|---|
| baseline | | | |
| | | | |

## Result

<Plain statement, no verdict label: is the delta real, marginal (within the noise floor measured
per MEASUREMENT.md), or a regression? State the number and how confident it is (paired runs,
noise floor, spread) - see MEASUREMENT.md sections 2b-4. Say explicitly which dimension(s) this
covers (time, peak RSS, allocation count) - don't let "Result" silently mean "time" only.>

## Does it change behavior/output?

<This is the section that matters most for a reader deciding whether to take the patch.>

- **Bit-exact?** yes / no
- If no: what exactly changes, for whom, under what conditions - in plain terms (e.g. "rounds
  down instead of to nearest for values ending in .5, affects about 1 in 4000 inputs, max visual
  difference 1 LSB"). Point at a saved visual artifact (diff image / PPM) if the output is visual.
- **Is this a breaking change?** (changes a documented behavior, public API, file format output,
  etc.) yes / no - and what breaks if yes.
- Project test suite: failing before -> failing after (list new failures explicitly, not just a
  count).

## Why the result looks the way it does

<Required even when nothing moved - "already optimal", "not actually on the hot path per
profiling", "confound X ruled it out" are all valid, informative answers. This is what stops the
next session from re-trying the same dead end.>

## What's not done / open questions

<Explicit bullet list: what wasn't applied to a vendored dependency yet, what wasn't
regenerated (snapshots/golden files), anything a maintainer would need to decide before taking
this patch.>
