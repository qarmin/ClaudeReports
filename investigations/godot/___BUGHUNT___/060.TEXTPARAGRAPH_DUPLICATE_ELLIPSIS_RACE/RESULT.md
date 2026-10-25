# `TextParagraph::duplicate()` reads `el_char` unguarded - a third accessor candidate 038 never looked at

From the coordinator's priority-3 list (`TextParagraph`'s 13 remaining unlocked methods, 038
follow-up). Most of those 13 are plain scalar/enum/`RID` getters (low risk, confirmed as such in
candidate 059's sibling reasoning about torn word-sized reads) - but `duplicate()` stood out because
it copies `el_char`, the exact `String` (`CowData`-backed) field candidate 038 already proved
race-prone via `set_ellipsis_char()`/`get_ellipsis_char()`. `duplicate()` reads `el_char` through a
third path neither of 038's two guarded accessors covers.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/text_paragraph.cpp` - `duplicate()` reads ~11 fields of `this` unguarded, including `copy->el_char = el_char;` (line ~348). `el_char` is the same `String` field candidate 038 found racing via `set_ellipsis_char()`'s write vs. `get_ellipsis_char()`'s read - `duplicate()` is a third, independent read site 038's narrow fix never touched | Static read | Identified `duplicate()` as a confirmed-mechanism (not just "plain scalar, low risk") candidate, since it shares the exact `CowData` field 038 already proved exploitable |
| GDScript-level repro (`repro/`: one `TextParagraph`, 3 writer threads alternating `set_ellipsis_char(".")`/`set_ellipsis_char("-")`, 3 reader threads calling `duplicate()` then `.get_ellipsis_char()` on the copy), against the existing unpatched ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, reused directly from the main tree's own `bin/`, no rebuild needed for the first run) in an isolated git worktree (`.worktrees/060`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on 3/3 attempts** - `heap-use-after-free core/templates/safe_refcount.h:138 in SafeNumeric<unsigned long>::conditional_increment()`, inside `TextParagraph::set_ellipsis_char()` itself (`text_paragraph.cpp:552`) - confirms `set_ellipsis_char()` is **not yet guarded in the main tree** (candidate 038's fix only ever existed as its own `patch.diff`, never merged, per this project's fix-verify-revert workflow) |
| First fix attempt: added `_THREAD_SAFE_METHOD_` to `duplicate()` only (candidate 038's own fix not yet applied in this worktree) - rebuilt, reran | Same build/run, 10 attempts | **Still crashed 10/10** - but at a *different* line (`text_paragraph.cpp:559`, `el_char == c` inside `set_ellipsis_char()`'s own early-exit check), confirming `set_ellipsis_char()` genuinely has no lock of its own in the unpatched tree and was racing my newly-locked `duplicate()` - not a new bug, just proof that candidate 038's own fix needs to be applied *together* with this one |
| Complete fix: applied candidate 038's own `patch.diff` (locking `set_ellipsis_char()`/`get_ellipsis_char()`) on top of the `duplicate()` lock from the previous step - rebuilt, reran | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite with the complete fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes, 10/10 crashing with only half the needed fix applied (a
different crash site, same underlying unguarded-`el_char` cause), 10/10 clean once both
`duplicate()`'s read and candidate 038's original write/read guards are all in place together.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `TextParagraph`'s public GDScript API (`set_ellipsis_char()`/
`duplicate()`).

## Class

`memory-safety` (identical mechanism to candidate 038 - `String`'s `CowData<char32_t>` refcount
race on `el_char`, just reached through a third accessor 038's narrower fix never covered).

## Root cause

Identical to candidate 038 - see that candidate's `RESULT.md` for the full mechanism. The only new
information here: `TextParagraph::duplicate()` (`text_paragraph.cpp:329`) reads `el_char` directly
(`copy->el_char = el_char;`) as part of copying ~11 fields from `this` into a new instance, with no
lock of its own - a third call path into the same race 038 already proved exploitable via
`get_ellipsis_char()`, independently confirmed here.

## Introduced by

Same as candidate 038 - not independently dated, `duplicate()` has had this unguarded read since it
was introduced, same as `set_ellipsis_char()`/`get_ellipsis_char()`.

## The fix

`patch.diff` is a complete, standalone patch (candidate 038's own two-accessor fix, plus this
candidate's addition) - applying it alone to unpatched HEAD fixes the whole `el_char` race across
all three accessors. Diffstat: `+8/-0` in `scene/resources/text_paragraph.cpp` total (038's original
`+4/-0` plus this candidate's own `+2/-0` for `duplicate()`, counted against the file's current
unpatched state).

- `duplicate()`: added `_THREAD_SAFE_METHOD_` at the top of the function, covering the read of
  `el_char` (and, incidentally, every other field it copies - cheap insurance against any other
  field this candidate didn't specifically investigate turning out to share the same issue).
- `set_ellipsis_char()`/`get_ellipsis_char()`: candidate 038's own unmodified fix, reapplied here
  since it's a prerequisite for this candidate's crash to actually go away (locking only
  `duplicate()` leaves `set_ellipsis_char()`'s own unguarded internal read of `el_char` - the
  `el_char == c` early-exit check - still racing).

Verified: 10/10 clean runs of `repro/` with the complete fix applied (vs. 3/3 crashing with nothing
applied, and 10/10 *still* crashing, at a different line, with only `duplicate()`'s half of the fix).
Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the fix applied - identical to the
established baseline.

## What's not done / open questions

- The other 12 methods from the original "13 remaining unlocked methods" list (`get_rid`,
  `get_dropcap_rid`, `get_alignment`, `get_justification_flags`, `get_break_flags`,
  `get_text_overrun_behavior`, `get_max_lines_visible`, `get_line_spacing`, `get_dropcap_lines`,
  `is_dirty`) were re-confirmed by this candidate's own reading to be plain scalar/enum/`RID`
  getters that never touch `el_char` or any other `CowData`-backed field - not independently
  repro'd, left as a low-priority torn-word-read risk per the same reasoning as candidate 059's
  `material.cpp` RID finding.
- Next per the coordinator's priority-3 list: `Image`'s ~95 remaining unlocked methods via a
  scripted sweep (candidate 030's approach), then the `Tree`/`TreeItem` cross-object lock gap, then
  the ~22 remaining unaudited `_THREAD_SAFE_CLASS_` files.
