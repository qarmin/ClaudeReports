# Correction: the "~408MB of peak RSS is unattributed" figure was measured at the wrong moment

Written 2026-09-10 by candidate
[`039.MALLOC_TRIM_AND_RSS_ATTRIBUTION`](../039.MALLOC_TRIM_AND_RSS_ATTRIBUTION/RESULT.md). The
original `RESULT.md` is left exactly as written - this file records what later measurement changed.

## What the original said

That of 716.5MB peak RSS, Godot's own allocator counter accounted for ~308.2MB, leaving **~408MB
(57%) unattributed**, and that a future session should reach for `valgrind --tool=massif` to explain
the gap. That framing was carried forward as an open lead in `IDEAS.md` across three sessions.

## What is actually true

The ~308.2MB was a sum of `Memory::get_mem_usage()` **checkpoints taken inside
`EditorNode::EditorNode()`**, i.e. partway through startup - not the process's tracked peak. Sampled
at the right moments (candidate 039):

| moment | tracked heap |
|---|---|
| end of `EditorNode`'s constructor (what candidate 006 used) | ~308.2 MB |
| frame 120, startup settled | **425.0 MB** |
| process peak, `get_mem_max_usage()` | **427.5 MB** |
| entry to `Main::cleanup()` (after teardown) | 86.9 MB |

So the real gap between peak RSS (~700.2MB as measured on the same binary in candidate 039; 716.5MB
in the original run) and tracked peak (427.5MB) is **~272.7 MB, not ~408MB** - the original figure
overstated it by roughly 135MB because startup continues allocating after `EditorNode`'s constructor
returns.

## The gap is now itemized, and no heap profiler is needed

From `/proc/<pid>/smaps` at steady state (candidate 039, full data in that candidate's
`bench/measurements.txt`):

| component | size | source |
|---|---|---|
| live tracked engine heap | 425.0 MB | inside `[heap]`'s 488.0 MB mapping |
| heap overhead / free lists / fragmentation | ~62 MB | `[heap]` 488.0 MB minus live 425.0 MB |
| resident pages of the editor binary itself | 101.9 MB | file-backed, `Private_Clean` |
| glibc per-thread malloc arenas | ~57 MB | `[anon]`, quantified by candidate 038 |
| remainder of `[anon]` (thread stacks etc.) | ~49 MB | not separated further |

`IDEAS.md`'s standing recommendation to run `valgrind --tool=massif` for this is therefore
**retired**: it would have spent hours re-deriving the heap half of this breakdown and would not have
revealed the ~101.9MB binary-pages component at all, because massif does not see file-backed pages.

## Does the original conclusion still hold?

Its *per-plugin and per-phase breakdown* - the part every later candidate actually built on - is
unaffected: those are relative checkpoint deltas within the constructor, and candidates 007, 013,
017, 018, 019 and 032 all converted them into measured wins. What changes is only the
"unattributed" framing and its recommended next step.
