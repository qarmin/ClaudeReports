# find_child_atom_call_site_full_sweep

`IDEAS.md`'s top backlog row (added while working candidate `033`) asked for an explicit, exhaustive
grep of every `find_child_atom(` call site across `lofty/src/mp4/`, rather than continuing to find
them one at a time via cross-references left in each candidate's own "what's not done" section -
candidates `029`, `030`, and `033` had each independently rediscovered the same header-inclusive-
budget shape starting from a different single call site.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `grep -rn "find_child_atom" --include="*.rs" .` across the whole repo (not just `lofty/src/mp4/`, to also rule out a re-export or macro-generated call site outside that directory) | N/A | Exactly 5 matches on the identifier as a call/definition: the `pub(super) fn find_child_atom` definition itself (`lofty/src/mp4/read/mod.rs:122`), and 4 call sites - `moov.rs:54` (`trak`->`mdia`), `ilst/write.rs:80` (`moov`->`udta`), `ilst/write.rs:109` (`udta`->`meta`), `properties.rs:489` (`minf`->`stbl`). The remaining matches outside `lofty/src/` are prose/comments inside prior candidates' own `RESULT.md`/`repro/src/main.rs` files, not source call sites. |
| Confirmed `find_child_atom`'s visibility (`pub(super)`, i.e. visible only within `lofty::mp4`) rules out any call site existing outside `lofty/src/mp4/` at all, so the repo-wide grep above is conclusive, not just a spot check | N/A | `lofty/src/mp4/read/mod.rs:122`: `pub(super) fn find_child_atom<R>(...)` |
| Cross-checked each of the 4 call sites found against this session's own `SUMMARY.md` to confirm each was already fully investigated, not just noticed | N/A | `moov.rs:54` -> candidate `030` (Confirmed, logic) / `032` (fix landed and verified, then reverted per this skill's own working-tree-always-clean rule - `moov.rs:54` on current mainline still reads `find_child_atom(reader, atom.len, *b"mdia", ...)`, unfixed, with `032`'s `patch.diff` available to apply). `ilst/write.rs:80` and `:109` -> candidate `029` (Not reproducible - proved unreachable dead code, by construction of `write_to`'s call order). `properties.rs:489` -> candidate `033` (Confirmed, logic) / `034` (fix landed and verified, then reverted; `properties.rs:489` on current mainline still reads `find_child_atom(reader, len, *b"stbl", ...)`, unfixed, with `034`'s `patch.diff` available to apply). |

## Status

**Not reproducible.**

Not a new bug: this candidate is the completeness check itself. The exhaustive grep (repo-wide, not
just the `mp4/` directory) turns up exactly the 4 call sites already known from candidates `029`,
`030`/`032`, and `033`/`034`, plus the `pub(super)` definition confirming no 5th call site can exist
anywhere else in the crate. No new instance of the header-inclusive-budget shape was found. This
closes out the backlog row it came from with a documented negative result, rather than leaving the
"is the sweep actually exhaustive" question open.

## Class

N/A

## Root cause (why nothing new turned up)

`find_child_atom` is declared `pub(super)` in `lofty/src/mp4/read/mod.rs`, which restricts every
possible caller to modules within `lofty::mp4` - a crate-wide grep is therefore guaranteed to find
every call site, not just a sample. There are only 4, and all 4 were already independently found
and fully investigated by earlier candidates before this sweep ran:

- `moov.rs:54` (`trak`->`mdia`, read path) - real bug, `Confirmed` by `030`, fix verified by `032`.
- `ilst/write.rs:80` (`moov`->`udta`, write path) and `:109` (`udta`->`meta`, write path) - looks
  like the same shape in isolation, but proven unreachable: `write_to` always calls
  `AtomWriter::new_from_file` first, which re-parses the same bytes with a correctly tight,
  zero-tolerance budget and rejects anything that could ever reach either `find_child_atom` call
  with a too-generous size (`029`).
- `properties.rs:489` (`minf`->`stbl`, read path) - real bug, `Confirmed` by `033`, fix verified by
  `034`.

The "three separate candidates each independently rediscovered this" pattern the backlog row
worried about was a real inefficiency in *how* they were found (one cross-reference at a time)
rather than a sign that more instances existed to find - the shape's total surface area was already
fully enumerated by the time this sweep ran, this sweep just proves that formally instead of by
absence-of-further-leads.

## The fix

None - this candidate found no new site needing one. `moov.rs:54`'s and `properties.rs:489`'s
already-verified fixes remain unapplied on mainline (by design - this skill always reverts, see
`SKILL.md`'s "the one rule that matters most") but are available as `patch.diff` in candidates `032`
and `034` respectively, for a maintainer to apply. `ilst/write.rs:80`/`:109` need no fix (dead code,
per `029`).

- Project test suite: not run (no change made to the working tree this candidate - N/A).

## What's not done / open questions

- Whether `030`'s/`032`'s and `033`'s/`034`'s two independently-verified `patch.diff` fixes apply
  and pass the suite cleanly *together* in one combined tree (already tracked as its own open
  `IDEAS.md` backlog row, unrelated to this sweep's own scope) is still not checked.
- This sweep only covers `find_child_atom`, the specific function named in the backlog row. The
  sibling `atom_tree` function (`lofty/src/mp4/read/mod.rs`) has its own, structurally different
  history of bugs (candidates `023`/`025`) and was out of scope here; a similar exhaustive call-site
  sweep for `atom_tree` was not performed this session (it currently has exactly one call site,
  `ilst/write.rs::save_to_existing`, so is lower-priority for a dedicated sweep, but wasn't
  formally grepped end-to-end here for repo-wide re-exports the way `find_child_atom` was).
