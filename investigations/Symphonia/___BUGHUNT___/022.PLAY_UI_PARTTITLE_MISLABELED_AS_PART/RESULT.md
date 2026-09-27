# PLAY_UI_PARTTITLE_MISLABELED_AS_PART

`fmt_tag` in `symphonia-play/src/ui.rs` (the match arm at line 907, flagged by `cargo clippy
--workspace --all-targets --all-features -- -W clippy::pedantic` as `clippy::match_same_arms`)
formats `StandardTag::PartTitle(v)` with the label `"Part"` - the exact same label used for the
distinct `StandardTag::Part(v)` variant two lines above it (`symphonia-core/src/meta.rs:305` vs
`:307`). Every sibling `*Number`/`*Total`/sort-order variant in the same match block gets its own
distinguishing label (`"Part Number"`, `"Part Total"`, and `SortPartTitle` already prints `"Part
(Sort Order)"`), so `PartTitle` printing plain `"Part"` looks like a copy-paste miss rather than
an intentional choice.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: traced `StandardTag::Part`/`PartTitle`'s only real producer in the tree (`symphonia-format-mkv/src/tags.rs:483-487`, the Matroska `PART`-level `TITLE` tag) and confirmed no code anywhere in the workspace currently constructs `StandardTag::Part` (only `PartTitle` is ever produced, by the Matroska reader) | grep across the whole workspace | Confirms the two variants are semantically distinct (per `symphonia-core/src/meta.rs:305-308`) and that `PartTitle` is the one with a real, reachable producer today |
| Real end-to-end repro: hand-built a real, well-formed Matroska byte stream (`EBML{DocType:"matroska"} > Segment > Info, Tracks{}, Tags > Tag > Targets{TargetTypeValue=40 ("PART")}, SimpleTag{TagName:"TITLE", TagString:"My Part Title"}`) - the standard Matroska tagging structure a real muxer would write for a multi-disc "part" title - fed to the real, compiled `symphonia-play` binary via `--probe-only`, driven through the real `symphonia::default::get_probe()` -> real `MkvReader` -> real `symphonia_play::ui::print_format`/`fmt_tag` path | release build, 3 repeat runs | Reproduces 3/3 runs: printed line is `Part                 My Part Title` (key `"Part"`, not `"Part Title"`), deterministic, no seed/layout dependence |

No sanitizer/miri/fuzzing was applicable - this is a pure logic (mislabeling) bug in a `match`
arm with no unsafe code, parsing hazard, or arithmetic involved, so a single clean, real
end-to-end reproduction plus direct code reading (per `HEURISTICS.md`'s guidance for this bug
class) was sufficient; the soft 3-approach cap was not needed.

## Status

**Confirmed.** Reproduces 3/3 runs, deterministic, release build, real end-to-end path (real
`.mkv` bytes -> real `MkvReader` -> real compiled `symphonia-play --probe-only` binary).

**Fully reproducible**: **Yes**. The crafted `.mkv` file is a real, standard Matroska byte
stream (not a hand-constructed internal `StandardTag` value) parsed by the real Matroska tag
reader, and the actual compiled `symphonia-play` binary is what's run and whose actual stdout is
inspected - not a stand-in for either. This is a single-process, single-participant finding (a
file's own metadata being pretty-printed by the binary that opened it, not one participant
reacting to another's live input over a channel), so per `CONFIRMATION.md` section 11 there is no
second participant to fake here - the "genuine input" is the real file bytes, exactly as for a
parser-input finding.

## Class

logic

## Root cause

`fmt_tag`'s `match std_tag` block (`symphonia-play/src/ui.rs`) maps each `StandardTag` variant to
a `FormattedTag` with its own display label. The `PartTitle` arm was written with the same
literal `"Part"` label as the pre-existing, semantically-different `Part` arm instead of `"Part
Title"` - which the analogous `SortPartTitle` arm (`"Part (Sort Order)"`) and every other
`*Title`/`*Number`/`*Total` sibling in the same block otherwise consistently distinguishes. This
is display-only: the underlying `StandardTag` enum (`symphonia-core/src/meta.rs`) and the
Matroska tag mapping (`symphonia-format-mkv/src/tags.rs`) are both correct and already keep the
two variants distinct - only `symphonia-play`'s own pretty-printer conflates them in its output.

## Introduced by

93f8b406 "core (meta): Add standard tags for movies and TV shows." (2025-01-11). `git blame`
shows the pre-existing `Part`/`PartTotal`/`Performer` arms (label `"Part"`/`"Part Total"`/etc.)
predate this commit (from 5c7078ee, 2024-11-09), but this commit is the one that inserted the new
`PartNumber`/`PartTitle` arms in between them - `PartNumber` got its own correct label ("Part
Number") in the same commit, while `PartTitle` was given the pre-existing `Part` arm's label
instead of a new one.

## The fix

`patch.diff` changes the `PartTitle` arm's label from `"Part"` to `"Part Title"`
(`symphonia-play/src/ui.rs:907`), matching the naming convention already used by every sibling
arm in the same match block (`"Part Number"`, `"Part Total"`, and `SortPartTitle`'s `"Part (Sort
Order)"`). This is a single-string, one-line change with no other code path affected; there is no
narrower/wider variant of this fix to consider (per `CONFIRMATION.md` section 7, there's no
additional boundary value to vary - every `PartTitle` value hits the same arm regardless of its
string content).

Verified by re-running `repro/` against the patched clone (`just test`, both a fresh clone and a
scratch worktree in the main project were used): before the patch, the printed key is `"Part"`
("RESULT: BUG REPRODUCED"); after `git apply patch.diff` and rebuilding `symphonia-play`, the
printed key is `"Part Title"` ("RESULT: FIXED"). See `output.txt` for the actual captured
before/after stdout.

- Project test suite (`cargo test --workspace`, run in the main project's own working tree, not
  the throwaway clone): **106 passed, 0 failed** before the patch, **106 passed, 0 failed** after
  (identical per-crate counts, no new failures - the workspace has no test that asserts on
  `symphonia-play`'s printed tag labels).

## What's not done / open questions

- `StandardTag::Part` itself is never actually constructed anywhere in the current workspace (no
  format reader/metadata module produces it) - only `PartTitle` is reachable today (via
  Matroska). The mislabeling is real and confirmed regardless (any future producer of
  `StandardTag::Part`, or a user matching on the printed string, would silently conflate the two
  today), but there is currently no live code path that would print `"Part"` for an *actual*
  `StandardTag::Part` value to visually compare side-by-side against the `PartTitle` output in
  the same run.
- Only this one match arm was checked against its exact sibling-naming convention; a broader sweep
  of `fmt_tag`'s ~300 other arms for a similar copy-paste-label mismatch (distinct variants
  sharing a literal label) was not performed this session - `clippy::match_same_arms` under
  `--all-features --all-targets` did not flag any other arm as identical to a *different*
  variant's arm besides this one, but clippy's lint only catches literally-identical arm bodies,
  not "different label than the naming convention implies", so a manual review would be needed to
  rule out more of these.
