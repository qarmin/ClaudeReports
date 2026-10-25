# 039. `malloc_trim()` frees almost nothing, and where the editor's 700MB of RSS actually lives

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Metric(s) measured**: steady-state and peak whole-process RSS, per-mapping RSS
  (`/proc/self/smaps`), Godot's tracked heap at three different moments, and `malloc_trim()`'s own
  cost
- **Workload**: empty-project editor startup,
  `--headless --editor --path harness/empty_project --quit-after 300`
- **No `patch.diff`**: measurement/characterization only. The change this candidate set out to
  evaluate was measured and found not worth making, so nothing was patched into real source. The
  probe used to measure it is kept as `bench/trim_probe_instrumentation.diff`.

## What was tried

Candidate 038 showed that ~57MB of startup RSS is glibc per-thread arena space, but the only fix
found for it (`mallopt(M_ARENA_MAX, 1)`) costs +7.34% editor startup time. This candidate tested the
follow-up recorded in `IDEAS.md`, which would attack the same memory without touching allocation
throughput: call `malloc_trim(0)` once startup has settled, handing free heap space back to the OS.

Rather than an A/B build, the measurement was taken **from inside the process** in a single run: at
frame 120, read own `VmRSS`, call `malloc_trim(0)`, read `VmRSS` again, and print the difference
alongside `Memory::get_mem_usage()`. That makes the "how much is even available to reclaim"
question answerable directly, with no baseline arm to drift.

## Result

**`malloc_trim(0)` releases 188-192 kB of ~717 MB resident - 0.027%.** Four runs, `ret=1` (glibc did
release something; there is simply almost nothing to release), cost 86-269 us. Under candidate 038's
`MALLOC_ARENA_MAX=1` config it releases even less, 52-60 kB.

So the hypothesis behind the lead - that a large part of the editor's RSS is freed-but-retained heap
- **is wrong**, and no `malloc_trim()` placement anywhere in the engine would change that. This is a
null result for the fix, but it also redirects the whole question, because the probe answered where
the memory actually is.

**Steady-state RSS is not meaningfully below peak**: last `VmRSS` before exit is only ~1-2MB under
`VmHWM` (715116 vs 717136 kB). For this workload "peak RSS" and "idle RSS" are the same number, so
every earlier candidate's peak-RSS figure also describes what a user watching the editor's idle
memory would see.

**Full attribution of the 700.2 MB, from `/proc/<pid>/smaps` at steady state:**

| mapping | RSS | what it is |
|---|---|---|
| `[heap]` | 499608 kB (488.0 MB) | the main malloc heap |
| `[anon]` | 108172 kB (105.6 MB) | mmap'd anonymous: glibc arenas (~57MB per candidate 038), thread stacks |
| `bin/godot.linuxbsd.editor.x86_64` | 104360 kB (101.9 MB) | the executable's own resident text/data pages |
| `libc.so.6` | 1880 kB | |

`Private_Dirty` is 611228 kB (all anonymous) and `Private_Clean` is 103000 kB, matching the split
above: ~611MB of genuinely dirtied anonymous memory plus ~103MB of file-backed binary pages.

**Godot's own tracked heap, measured at three moments** - the number that resolves the workspace's
longest-standing open question:

| moment | `get_mem_usage()` | |
|---|---|---|
| frame 120 (startup settled) | 445627885 B | **425.0 MB** |
| process peak (`get_mem_max_usage()`) | 448283037 B | 427.5 MB |
| entry to `Main::cleanup()` | 91165755 B | 86.9 MB |

## Does it change behavior/output?

N/A - nothing was patched into real source. No test-suite comparison applies for the same reason
(the probe build's suite is the pristine one: 1427 cases, 1426 passed, 1 failed, 3 skipped, the known
`test_ip.cpp` IPv6 failure).

## Why the result looks the way it does

`malloc_trim()` can only return pages that are free *and* releasable - contiguous space at the top of
the heap, or whole free pages inside it. The editor's heap is not mostly-free-and-retained: it holds
**425.0 MB of live tracked allocations** inside a 488.0 MB `[heap]` mapping, so only ~62MB is
allocator overhead, free lists and fragmentation, and essentially none of that is in trimmable shape.
That is why the call succeeds and still frees under 200 kB.

**This corrects candidate 006's headline open question.** Candidate 006 reported that ~408MB of the
716.5MB peak RSS was "NOT attributed by Godot's own allocator counter", and every session since has
carried that as a mystery needing an external heap profiler. That figure came from comparing peak RSS
against ~308.2MB of tracked heap - but the 308.2MB was a checkpoint total captured **at the end of
`EditorNode`'s constructor**, i.e. partway through startup, not the process's tracked peak. Measured
at the right moment, the tracked peak is 427.5 MB and the gap is **272.7 MB**, not ~408MB. And that
gap is now itemized rather than mysterious: ~101.9MB is the binary's own resident pages, ~57MB is
glibc arenas (candidate 038), ~62MB is heap overhead/fragmentation, and the remainder is thread
stacks and the rest of `[anon]`. A `valgrind --tool=massif` run - which `IDEAS.md` has recommended
for two sessions - would have spent hours re-deriving the heap half of a breakdown that `smaps`
gives in one run, and would not have shown the binary-pages component at all.

The ~101.9MB of resident binary pages is the single largest previously-unidentified block. The
editor binary is ~184MB on disk (`tests=yes`, unstripped of symbols in the sense that `debug_symbols`
is off but the binary is still large), and startup touches roughly half of it.

## What's not done / open questions

- **Whether the ~101.9MB of resident binary pages is reducible** is untested. It is a function of
  how much of the binary startup actually executes/reads, so plausible levers are build-level
  (`-Wl,--gc-sections`, PGO-driven text layout so hot code shares fewer pages, or the `optimize=size`
  build) rather than source-level. Note this component is `Private_Clean`, i.e. file-backed and
  evictable under memory pressure, so it is less costly than an equal amount of dirty heap - worth
  saying explicitly in any "Godot uses 700MB" discussion.
- **The ~62MB of heap overhead/fragmentation** (488.0MB mapping vs 425.0MB live) was not broken down
  by allocation size class; glibc's `malloc_info()` would do that in one more probe.
- `[anon]`'s 105.6MB is only partly attributed (arenas ~57MB per candidate 038); thread stacks and
  any driver/ICU anonymous mappings were not separated.
- The steady-state figures are for an **empty** project at frame 120; a real project with assets
  loaded would shift the heap component substantially.
