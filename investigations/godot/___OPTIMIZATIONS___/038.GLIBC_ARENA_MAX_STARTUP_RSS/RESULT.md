# 038. glibc per-thread malloc arenas cost ~57MB of editor-startup RSS (`mallopt(M_ARENA_MAX, 1)`)

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **File**: `platform/linuxbsd/godot_linuxbsd.cpp` (+11/-0)
- **Metric(s) measured**: whole-process peak RSS (`VmHWM`), Godot's tracked peak heap
  (`get_mem_max_usage()`), and wall-clock time (editor startup + 4 runtime benches)
- **Workload**: empty-project editor startup,
  `--headless --editor --path harness/empty_project --quit-after 1`, plus `harness/gdbench` modes
- **Platform note**: glibc-specific (glibc 2.43, 24 cores). No effect on musl/macOS/Windows.

## What was tried

This candidate came out of candidate 037, where a probe of `MALLOC_ARENA_MAX=1` (a lead already in
`IDEAS.md` from candidate 034/035's bimodality work) turned out to move **the baseline** far more
than it moved the thing being tested.

glibc gives each thread its own malloc arena, up to `8 * ncores` (192 on this 24-core machine), and
each arena's pages stay resident once touched. Godot's `WorkerThreadPool` plus the render/resource
threads make that cap reachable during startup, so the arenas themselves hold tens of MB of
resident memory containing no engine data.

The patch calls `mallopt(M_ARENA_MAX, 1)` at the very top of `main()` in the linuxbsd entrypoint,
guarded by `#ifdef __GLIBC__`, before any thread is spawned.

## Result

**Peak RSS: -57.0 MB (-8.13%)**, 716742.5 -> 658470.5 kB mean (700.0 -> 643.0 MB), 8 runs per arm,
distributions do not overlap (default 716036-718068, arena=1 655764-659792). All runs exit 0.

**Editor startup wall time: +183 ms (+7.34%)**, 2495.75 -> 2679.00 ms mean, 4 runs per arm. In the
separate interleaved 8-pair run that first found this, the patched arm was slower in 7/8 pairs
(2474.5 -> 2641.3 ms mean, +6.7%).

**Godot's own tracked counters do not move at all**: `usage=91165395 max=448283037`, byte-identical
under every arena setting tested. All ~57MB is glibc arena space, invisible to the engine's own
allocator wrapper.

**Single-threaded runtime benches are unaffected** (3 interleaved pairs each, `array_append`,
`dict_insert`, `node_instance`, `string_concat` - all within noise), which is what one expects when
only one thread is allocating.

**There is no intermediate sweet spot.** The saving is essentially binary rather than proportional
to the cap: reliable at `MALLOC_ARENA_MAX=1` (8/8 runs low), bimodal with ~57MB amplitude at 2, 3
and 4 (whether the extra arenas get created depends on thread timing), and absent at 8+ / default.
Every setting that sometimes avoids the arenas also costs startup time, so 1 is the only setting
that gets the memory back every time. Full sweep in `bench/arena_sweep.txt`.

**In-engine equivalence verified**: the patched binary and the pristine binary run with
`MALLOC_ARENA_MAX=1` land in the same place (660932/655448/662060 vs 659552/658400/655928 kB),
both ~57MB under the pristine default (716504/716136/717164 kB) - i.e. the one-line `mallopt()` call
reproduces the environment variable exactly, so this is actionable in-engine and not just a
deployment tweak.

## Does it change behavior/output?

No functional change - `mallopt()` only tunes the allocator; no engine code path differs.

**Test suite, both arms, identical**: 1427 cases, 1426 passed, 1 failed, 3 skipped; 424066
assertions, 423066 passed, 1000 failed. The single failure is the known `test_ip.cpp` IPv6 check
(no IPv6 in this container), present in both arms.

The real cost is the measured one: allocation across all Godot threads is serialized onto a single
arena, which is what the +7.34% startup time is. Two things that follow from that and were **not**
measured here, and which matter for judging the trade:

- Only editor **startup** time was measured. A workload with heavier sustained multi-threaded
  allocation (importing many assets, baking lightmaps, a threaded game loop) would contend harder
  than startup does, so +7% is a lower bound on the time cost, not a general figure.
- Only an **empty** project was measured. A real project's startup allocates more, so both the MB
  saved and the contention could scale differently.

## Why the result looks the way it does

`VmHWM` counts resident pages, and a glibc arena becomes resident as soon as a thread allocates
from it, independent of how much of it is in use. With up to 192 arenas permitted and a thread pool
sized to the machine, startup touches many of them; capping at 1 means every thread allocates from
the main heap instead, so those pages never exist. That the engine's own counters stay
byte-identical is the proof that nothing about Godot's allocation changed - only where the
allocator put it.

This also explains part of candidate 006's long-standing open question (~408MB of peak RSS not
attributed by `Memory::get_mem_usage()`): at least ~57MB of that gap is glibc arena overhead, not
engine data, not GPU/driver memory and not ICU/font data. It is very likely the same mechanism
behind the ~6MB bimodality candidates 034/035 characterized, at a different scale: the same
"does this arena get created on this run" coin-flip, visible here at ~57MB amplitude when the cap
sits at 2-4.

## What's not done / open questions

- **The time/memory trade is not resolved, deliberately.** -57MB for +7% startup is a real choice
  someone else has to make (and per this skill's rules, this file does not make it). An obvious
  middle path was **measured and rejected** - see the sweep: intermediate caps do not give a
  reliable saving.
- **`malloc_trim()` was not tried** and is the more promising follow-up for real users, because it
  attacks the same memory without touching allocation throughput. It cannot help *peak* RSS (a
  high-water mark cannot be lowered after the fact), but it could cut steady-state `VmRSS` after
  startup settles, which is what someone watching the editor's idle memory in a task manager
  actually sees. Recorded in `IDEAS.md`.
- **Steady-state (`VmRSS`) rather than peak was not measured** for this patch either, for the same
  reason - the harness exits immediately after startup via `--quit-after 1`.
- Other platforms are untouched and unmeasured; the patch is glibc-only by construction. Godot
  currently calls `mallopt()` nowhere in the repo.
