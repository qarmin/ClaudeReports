# 045. `optimize=size` trades 25MB of resident binary pages for 17-42% on hot runtime paths

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Metric(s) measured**: binary size on disk, per-mapping resident memory (`/proc/<pid>/smaps`),
  peak RSS, Godot's tracked peak heap, editor startup wall time, and 7 runtime benches
- **Workload**: empty-project editor startup + `harness/gdbench` modes
- **No `patch.diff`**: this is a **build-configuration** comparison (`optimize=size`), not a source
  change, so there is nothing to patch. Both arms are the same source tree.

## What was tried

Candidate 039 itemized the editor's ~700MB of startup RSS and found that **101.9MB of it is the
editor binary's own resident text/data pages** - the single largest component no candidate in this
workspace had ever looked at, since every other candidate attacked engine heap allocations (a
separate 425MB). It flagged the levers as build-level and recorded `optimize=size` as the first
thing to try.

This candidate builds the same tree twice, changing only `optimize=size`, and measures both what it
saves and what it costs.

## Result

**Binary on disk: -38.5 MB (-20.9%)**, 175.4 -> 138.7 MB.

**Resident binary pages: -25.5 MB (-24.9%)**, 104988 -> 78856 kB mean.

**Peak RSS: -25.13 MB (-3.59%)**, 700.1 -> 675.0 MB mean, 6 interleaved pairs, lower in 6/6 with
non-overlapping distributions.

**The saving is entirely binary pages**, confirmed three independent ways: `[heap]` is 499696 kB in
both arms (identical), `[anon]` is 108304 vs 108308 kB (identical), and Godot's own tracked counters
agree within **24 bytes** (`max=448282949` vs `448282925`). Nothing about engine allocation changed,
which is exactly what candidate 039's attribution predicted.

**Cost 1 - editor startup: +212.8 ms (+8.56%)**, 2486.2 -> 2699.0 ms, slower in 6/6 pairs.

**Cost 2 - runtime, and this is the decisive one**: the loss is concentrated in
computation-dominated paths, while allocation-dominated ones are untouched.

| bench | default | `optimize=size` | delta |
|---|---|---|---|
| `regex` | 223.2 ms | 317.8 ms | **+42.4%** |
| `vector_math` | 502.0 ms | 598.3 ms | **+19.2%** |
| `variant_call` | 1061.0 ms | 1247.7 ms | **+17.6%** |
| `array_append` | 314.3 ms | 319.3 ms | +1.6% |
| `string_concat` | 405.3 ms | 410.3 ms | +1.2% |
| `node_instance` | 223.2 ms | 225.4 ms | +1.0% |
| `dict_insert` | bimodal (~315 / ~406) | ~410 | not counted, see below |

`dict_insert` is **bimodal in the default arm** (two clusters at ~315 and ~406 ms across 5 runs)
while `optimize=size` sits consistently in the slow cluster. Medians would give +28%, but a bimodal
arm cannot support a clean delta, so it is reported and excluded rather than counted - the same
discipline candidate 034 established for the bimodal RSS harness.

## Does it change behavior/output?

No source difference exists between the arms, so no behavioral change is possible by construction.

**Test suite, both builds**: 1427 cases, 1426 passed, 1 failed, 3 skipped (the known `test_ip.cpp`
IPv6 failure) - identical, as expected.

## Why the result looks the way it does

`-Os` gives up the inlining, unrolling and auto-vectorization that `-O3`-style optimization applies,
which is precisely what `Variant` dispatch (a large switch over types, heavily inlined), vector math
(small always-inline operator functions) and regex matching (tight character loops) depend on. Work
whose cost is dominated by allocation and pointer chasing rather than arithmetic - appending to
arrays, building strings, instantiating Nodes - barely notices, which is why those three sit at ~1%.

The RSS side is straightforward: ~24.9% less machine code means ~24.9% fewer resident text pages,
and startup touches roughly the same *fraction* of a smaller binary.

Worth keeping in perspective when weighing this: the pages saved are `Private_Clean` (file-backed,
per candidate 039), so the kernel can evict them under memory pressure, whereas the 425MB of engine
heap is `Private_Dirty` and cannot be. 25MB of evictable mappings is a genuinely cheaper 25MB than
25MB of heap - which makes the 17-42% runtime cost harder to justify for an editor binary, since the
editor runs tool scripts and game code through exactly the `Variant`/vector paths that regress.

## What's not done / open questions

- **PGO / hot-cold text layout was not tried**, and it is the more promising version of this idea:
  it could reduce the *touched fraction* of the binary (fewer resident pages) while keeping or
  improving speed, instead of trading one for the other. Recorded in `IDEAS.md`.
- **`-Wl,--gc-sections` / `-ffunction-sections` was not measured separately** from `optimize=size`;
  it might recover part of the 38.5MB of on-disk size with no codegen change at all.
- Only the **editor** target was measured. `target=template_release` builds already use different
  flags, and a memory-constrained platform (mobile/web) may well be a case where this trade is
  already made deliberately upstream - not checked.
- The runtime benches are GDScript-level; no C++-level microbenchmark or `perf` profile was taken to
  confirm *which* inlining decisions account for the regex/Variant regressions.
