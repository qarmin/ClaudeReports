# `Object` allocates a heap `Mutex` for signals plus a separate inline instance-binding mutex - merge into one inline mutex

Found via this session's once-per-session GitHub issue/PR inspiration sync (`gh-issues-prs-check/fetch_issues.py raw-sync`), matching still-open PR
[godotengine/godot#119985](https://github.com/godotengine/godot/pull/119985) ("Merge the two `Object` mutexes to save on RAM (1/4 down) and allocation time (~20% down)").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `core/object/object.h`/`.cpp` to understand the two mutexes | code reading | Confirmed: every `Object` (that doesn't override `_uses_signal_mutex()` to return `false`, which only `Node` does) heap-allocates a `Mutex` via `memnew` in `_postinitialize()` for signal locking, *in addition to* an always-present inline `BinaryMutex _instance_binding_mutex` used for GDExtension instance bindings. PR #119985 replaces both with one inline `Mutex _control_mutex` used for both purposes. |
| Apply PR's diff via `git apply` | against `base_commit` | Does not apply cleanly - 3 hunks in `object.cpp` fail due to unrelated intervening changes since the PR was opened |
| Apply via `patch -p1` (fuzzy offsets) | same | Applies cleanly (all hunks succeed, one with fuzz 1) - `git diff --numstat`: `+26/-36` across `core/object/object.cpp`, `core/object/object.h`, `scene/main/node.h` |
| Benchmark using the existing `node_instance` harness mode (`Node.new()`/`.free()`) | `harness/gdbench/bench.gd` | **Not usable for this candidate** - `Node` already overrides `_uses_signal_mutex()` to return `false` (a pre-existing optimization, "Node uses thread guards instead"), so `Node` objects never got the heap-allocated `Mutex` in the first place; this harness mode cannot show the improvement. Built two new standalone probes instead (see `bench/`) that use plain `Object.new()`, which *does* hit the removed allocation. |
| `bench/timing_probe.gd`: 500,000 `Object.new()`/`.free()` iterations (10% warmup discarded), 5 repeat runs each build | `--headless --script timing_probe.gd` | See Result |
| `bench/memory_probe.gd`: `OS.get_static_memory_usage()` before/after allocating and retaining 200,000 `Object.new()` instances (deterministic, no free) | `--headless --script memory_probe.gd` | See Result - `OS::get_static_memory_usage()` is Godot's own internal allocation-byte counter (`Memory::alloc_static`/`memnew` bookkeeping), not `/proc`-based RSS (`/usr/bin/time` is not installed in this container) - it is fully deterministic across repeat runs (identical value all 3 runs, both builds) |

## Status

N/A (not a bug-hunt candidate) - this is a `perf-optimize` measurement.

## Result

**Time** (`bench/timing_probe.gd`, 500,000-iteration `Object.new()`/`.free()` loop, 5 runs each, mean us/iteration):
- Baseline: 0.092838, 0.109576, 0.093608, 0.094688, 0.117406 -> mean 0.1016 us/iter
- Patched: 0.089246, 0.087356, 0.089360, 0.088084, 0.096176 -> mean 0.0900 us/iter
- **-11.4%** mean, though noisier than a typical candidate at this absolute scale (~90-100 nanoseconds/iteration is close to measurement-noise territory for a `Time.get_ticks_usec()`-based timer); direction is consistent (every patched run is at or below every baseline run except baseline's own two low outliers 0.0928/0.0936 vs patched's high outlier 0.0962), consistent with removing one `memnew`/heap allocation + matching `memdelete` per `Object` lifecycle.

**Memory** (`bench/memory_probe.gd`, `OS.get_static_memory_usage()` delta for 200,000 retained `Object` instances, 3 runs each, fully deterministic - identical across all 3 runs per build):
- Baseline: 71,132,256 bytes delta -> **355.66 bytes/object**
- Patched: 61,532,256 bytes delta -> **307.66 bytes/object**
- **-48.00 bytes/object exactly** (9,600,000 bytes / 200,000 objects), a clean, deterministic, fully-reproducible reduction - this is the more trustworthy of the two numbers given its zero run-to-run variance, and directly confirms the PR title's RAM-savings claim (though the PR's own "1/4 down" refers to a different baseline framing than this test's whole-`Object`-including-RTTI/vtable size; the 48-byte absolute reduction is what actually reproduces here).

Both dimensions improve; neither regresses.

## Does it change behavior/output?

No engine-observable behavior change intended - `_control_mutex` is used for exactly the same two purposes (`ObjectSignalLock`'s pairwise-ordered locking for signal emission, and instance-binding list access) that the two separate mutexes covered, with the same lock-ordering discipline (`ObjectSignalLock` still sorts by mutex address to avoid deadlock when locking two objects' mutexes together). Full project test suite: **1427 test cases, 1426 passed, 1 failed, 3 skipped** (the same pre-existing `test_ip.cpp` IPv6 failure documented in `README.md`) - **identical before and after**, run to completion with no hang/deadlock/crash in either build.

One theoretical risk was checked by reading rather than by a dedicated stress test: merging the two mutexes means a callback invoked *while* `_control_mutex` is held for one purpose (e.g. an instance-binding `reference_callback` fired during `_instance_binding_reference`) that re-enters and tries to lock `_control_mutex` again for the *other* purpose (e.g. connecting a signal) would now contend on the same mutex where before it used a different one. `Mutex` in this codebase (`core/os/mutex.h`) wraps a **non-recursive** `std::mutex`, so a same-thread re-entrant lock would deadlock. This was not exercised by a dedicated recursive-callback stress test this session (out of scope for the time available) - the full test suite passing without hanging is the only evidence gathered against it. Flagged in "What's not done" below rather than left unstated.

- Project test suite: 1426/1427 passing -> 1426/1427 passing (no change, no new failures, no hang).

## What's not done / open questions

- No dedicated stress test for the re-entrant-lock deadlock risk described above (a callback registered via `set_instance_binding`'s `reference_callback` that itself triggers signal connect/emit on the same `Object`, from the same thread, while `_control_mutex` is already held) - the full test suite passing is suggestive but not a targeted proof. This is exactly the kind of subtlety a maintainer reviewing the real upstream PR would need to check across GDExtension usage patterns, not something this session's time budget could fully rule out.
- Time measurement is noisier than ideal given the small absolute per-iteration cost (~90-100ns) - a lower-noise measurement (e.g. `perf stat` cycle counts instead of wall-clock, or a much larger n) would tighten the confidence interval; the memory number is the stronger, fully-deterministic evidence here.
- Did not check `sizeof(Object)` directly (e.g. via a scratch `static_assert`/`printf`) to cross-validate the 48-bytes/object memory delta against the actual struct layout change (removing one 8-byte pointer + a heap allocation's malloc bookkeeping overhead, replaced by one inline `Mutex` whose size needs comparing against the removed `BinaryMutex`) - the measured 48 bytes is treated as an empirical black-box result, not decomposed into its structural cause.
