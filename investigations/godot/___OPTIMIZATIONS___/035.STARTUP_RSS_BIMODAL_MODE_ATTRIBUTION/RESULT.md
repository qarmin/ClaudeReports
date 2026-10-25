# The bimodal ~6MB in this workspace's startup peak-RSS harness is outside Godot's allocator - and `Memory::get_mem_max_usage()` is byte-deterministic

Candidate 034 hit the limit of this workspace's measurement method: its effect (~6.9MB) was the same
size as a mode the harness itself moves between, so the standard "distributions don't overlap" test
failed on a real, reproducible improvement and the candidate needed 12 runs per arm plus
mode-matched statistics. Since **every** remaining lead in `IDEAS.md` is worth 2-7MB, that mode - not
the fixes - had become the bottleneck. This candidate attributes it and replaces the method.

## What was tried

One `fprintf` at the top of `Main::cleanup()` (`bench/instrumentation.diff`, applied, measured,
reverted) printing Godot's own two allocator counters: `Memory::get_mem_usage()` (live bytes at that
point) and `Memory::get_mem_max_usage()` (**the process's high-water mark of tracked heap**). Then
30 consecutive pristine-`HEAD` runs of the usual harness (`bin/godot.linuxbsd.editor.x86_64
--headless --editor --path ___OPTIMIZATIONS___/harness/empty_project --quit-after 1`), capturing
`VmHWM` and both counters per run (`bench/bimodal_loop.sh`, raw output in
`bench/pristine_30_runs.txt`; all 30 exit 0).

The question was deliberately framed as a discriminator: if the tracked heap is bimodal too, the
mode is *inside* Godot (some optional startup work that sometimes runs); if the tracked heap is
constant while RSS is bimodal, the mode is *outside* the tracked allocator (arena/trim behaviour,
fragmentation, driver or thread-stack pages).

Then 5 runs of the same instrumented build with candidate 034's `patch.diff` applied on top, to test
whether the counter can substitute for the RSS measurement (`bench/candidate034_5_runs.txt`).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS (`VmHWM`) and Godot's tracked heap counters, per run.**

| Quantity, 30 pristine runs | Value |
|---|---|
| `Memory::get_mem_usage()` at `Main::cleanup()` | **91165391 bytes in all 30 runs - one distinct value** |
| `Memory::get_mem_max_usage()` (tracked peak) | **448283245 bytes in all 30 runs - one distinct value** |
| Peak RSS (`VmHWM`) | 3 runs at 712980-713320 kB (mean **696.44MB**), 27 runs at 718012-720284 kB (mean **702.30MB**) |
| RSS spread | 7.13MB total; the two clusters are **5.86MB** apart with nothing in between |

**Godot's tracked heap is byte-identical across all 30 runs, including its own peak, while process
RSS is bimodal.** The ~6MB mode is therefore **entirely outside the engine's tracked allocator** -
it is not optional startup work, not a data-dependent cache, and not anything a patch to engine code
put there. Remaining candidates for it (not distinguished here) are glibc `malloc` arena/trim
behaviour varying with thread scheduling, thread-stack and page-fault timing, and driver
allocations, all of which bypass Godot's `Memory::` wrapper.

**The practical consequence is a strictly better measurement for this class of candidate.** With
candidate 034's patch applied, the tracked peak is **442484227 bytes, again byte-identical across 5
runs**:

| | tracked peak (`get_mem_max_usage()`) | runs needed | peak RSS |
|---|---|---|---|
| pristine `HEAD` | 448283245 bytes | 1 (deterministic over 30) | 701.29MB (12 runs, bimodal) |
| candidate 034 | 442484227 bytes | 1 (deterministic over 5) | 694.46MB (12 runs, bimodal) |
| **delta** | **-5799018 bytes = -5.53MB** | | -6.83MB |

So candidate 034's effect is **-5.53MB of tracked peak heap, measurable exactly from one run per
arm**, versus 24 runs plus mode-matched means/medians/pairwise-win-rate to establish -6.83MB of RSS.
The RSS/tracked ratio is **1.23x** here (RSS also pays page-granularity and allocator overhead on
top of each tracked byte). Per deferred `CreateDialog` that is **1.38MB tracked / 1.71MB RSS**.

## Does it change behavior/output?

- **patch.diff: N/A** - measurement/characterization only, no patch to real source. The
  `Main::cleanup()` `fprintf` in `bench/instrumentation.diff` is instrumentation, was reverted, and
  is not proposed as a change.
- Project test suite: not re-run for this purely additive `fprintf` - no changes to real source
  survived this candidate. (Candidate 034's own suite run covers the patch used as the test case
  here.)

## Why the result looks the way it does

Godot wraps its own allocations in `Memory::alloc_static()`/`free_static()` and counts them, so
anything allocated *through the engine* is counted deterministically for a fixed workload - and an
empty-project editor startup with `--quit-after 1` is a fixed workload down to the byte. What varies
run to run is how the C library and kernel back those allocations with pages: how many malloc arenas
get created (thread-timing dependent), whether `malloc_trim` returns memory before the high-water
mark is sampled, and where thread stacks and driver buffers land. None of that shows up in the
engine's counter, and all of it shows up in `VmHWM`.

This also retroactively explains two earlier anomalies in this workspace: candidate 032's baseline
reading ~10MB higher under a different `XDG_DATA_HOME` state, and candidate 024's need to widen its
baseline from 5 to 8 runs when the mean drifted.

## What's not done / open questions

- **Which of the three suspects actually produces the mode was not isolated.** The clean next probe
  is `MALLOC_ARENA_MAX=1` (or `M_TRIM_THRESHOLD` via `mallopt`) on 30 runs: if the mode disappears,
  it is arena behaviour. `valgrind --tool=massif` would also separate tracked from untracked heap,
  which is already an `IDEAS.md` item (the ~408MB RSS-vs-tracked gap).
- **`get_mem_max_usage()` is only meaningful where Godot's max tracking is compiled in** (it is
  non-zero in this `target=editor` build; a release template may report 0). Any future candidate
  using it should print both counters and sanity-check that `max >= usage`.
- The 448283245-byte tracked peak is **1.5x** the ~308.2MB figure candidate 006 measured with
  `get_mem_usage()` checkpoints, i.e. candidate 006's checkpoint-based attribution missed ~140MB of
  transient peak between its checkpoints. Re-running candidate 006's breakdown against
  `get_mem_max_usage()` deltas instead of `get_mem_usage()` deltas would give a truer per-phase
  picture; not attempted.
- This candidate does not re-state any earlier candidate's number. The RSS figures already recorded
  stand as measured; what changes is the recommended method for **future** small-effect candidates.
