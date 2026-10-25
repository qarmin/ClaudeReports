# `Curve` has no locking anywhere in the class

`scene/resources/curve.h`/`.cpp` declares `LocalVector<Point> _points` and a lazily-rebuilt
`Vector<real_t> _baked_cache`/`bool _baked_cache_dirty`, both mutated by `add_point()`/
`remove_point()`/`set_point_value()`/etc. and read by `sample()`/`sample_baked()`/
`get_point_position()`/etc., with **zero** synchronization of any kind - confirmed via `grep -n
"Mutex" scene/resources/curve.h` returning no matches. `Curve` is an ordinary `Resource`, not tied
to the scene tree or any node-level thread guard - it can be (and commonly is) shared across
multiple owners and accessed concurrently, e.g. from a background thread doing procedural
generation or baking. Found via an external 6-thread-bombardment sweep
(`~/Projekty/Godot/Qarminer/tools/thread/SWEEP_RESULTS.md`, 2026-10-05) that grouped `Curve`
(along with `Curve2D`/`Curve3D`, `BitMap`, `Image`, `MultiMesh`, `CryptoKey`, `GLTFDocument`,
`NavigationPolygon`, `ENetConnection`, `StreamPeerGZIP`) as "plain unsynchronized containers, no
scene-tree involvement" - `Curve` was picked from that list of 11 as the simplest API surface to
investigate first, per that backlog entry's own suggestion.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `curve.h`/`.cpp` lock-usage audit (`grep -n "Mutex"`) | n/a | Zero matches - no synchronization primitive of any kind exists in the class |
| GDScript-level repro (`gdscript_repro/`, modeled on the external sweep's random-method-hammering methodology: one shared `Curve`, 6 worker `Thread`s mixing `add_point`/`remove_point`/`sample`/`sample_baked`/`get_point_position`), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes` (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb), `--headless --path gdscript_repro` under `timeout -k 5 25` | **Crashed on the first run and reliably (5/5) across repeat runs** - a real, fully-symbolized `handle_crash` backtrace (`Curve::sample_baked` at `curve.cpp:560` reading a `LocalVector<Curve::Point>`/`Vector<real_t>` concurrently resized by another thread's `add_point`/`remove_point`), plus separately-observed real ASan heap-use-after-free reports from the same race |
| Fix (`_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` across ~34 public methods touching the guarded fields), same build/run, 8 runs | Same build/run | **Clean 8/8 runs** |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline from candidates 002-015 (same pre-existing `test_ip.cpp` IPv6 environment failure) |

## Status

**Confirmed.** Reproduced on the first run and 5/5 repeat runs of the (corrected) repro script
(every attempt tried), and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `Curve` class through nothing but its public
GDScript API (`Curve.add_point()`/`.remove_point()`/`.sample()`/`.sample_baked()`/
`.get_point_position()`). No second participant is involved (not a peer/wire-input finding) - this
is a same-process data race between GDScript `Thread`s sharing one `Curve` resource.

## Class

`memory-safety` (concurrent unsynchronized `LocalVector`/`Vector` mutation and lazy-cache rebuild -
same general mechanism as this session's other container races, manifesting as either a real ASan
heap-use-after-free or a fatal, intentional bounds-check trap depending on the exact interleaving)

## Root cause

`Curve::_points` (`LocalVector<Point>`) is mutated by `add_point()`/`_add_point()`/`remove_point()`/
`_remove_point()`/`clear_points()`/`set_point_count()`/etc. and read by `sample()`/
`sample_local_nocheck()`/`get_point_position()`/`get_point()`/etc., all with zero locking.
Separately, `_baked_cache`/`_baked_cache_dirty` form a lazy cache rebuilt by `_bake()` whenever
`sample_baked()` finds it dirty - also completely unguarded. Any two threads calling into this
public API concurrently - one resizing/reordering `_points` via `add_point`/`remove_point`, another
reading via `sample`/`sample_baked`/`get_point_position` - race on the same backing storage.
Depending on timing this either corrupts memory in a way ASan catches directly (a heap
use-after-free on the `Vector<real_t>`/`LocalVector<Point>` backing buffer mid-resize) or lands on
a transiently out-of-range index that Godot's own bounds check catches as a fatal, intentional trap
(`CRASH_BAD_UNSIGNED_INDEX`-style) - both are real manifestations of the identical underlying race,
just differing in which specific unsynchronized read loses the race against a concurrent resize.

Reachability: `Curve` is an ordinary, `GDREGISTER_CLASS`-registered `Resource`, fully
GDScript-instantiable and freely shareable across threads with no special setup - no scene tree,
no node, no editor context required to reach this race.

## Introduced by

`not determined` (predates tracked history in the meaningful sense). `Curve`'s source has existed
since 0b806ee0fc "GODOT IS OPEN SOURCE" (2014-02-09, the initial open-source commit) with no
`Mutex` ever added to the class at any point since - the same situation as candidate 012's
`InputMap` finding: not a regression from a later refactor, the class's original, unchanged
thread-safety posture across the engine's entire public history.

## The fix

Add `_THREAD_SAFE_CLASS_` (a recursive `Mutex`, `core/os/thread_safe.h`) as a private member and
`_THREAD_SAFE_METHOD_` as the first statement of effectively every public method that touches
`_points`, `_baked_cache`/`_baked_cache_dirty`, or `_min_value`/`_max_value`/`_min_domain`/
`_max_domain`/`_bake_resolution` - matching the exact convention candidates 008/009/010/012 already
established in this workspace (`+75/-1` across `scene/resources/curve.h`/`.cpp`, see `patch.diff`):

```cpp
// curve.h
private:
	_THREAD_SAFE_CLASS_
	LocalVector<Point> _points;
	...

// curve.cpp
real_t Curve::sample_baked(real_t p_offset) const {
	_THREAD_SAFE_METHOD_
	...
}
```

~34 methods instrumented, including every `set_point_*`/`get_point_*`/`add_point*`/`remove_point`/
`clear_points`/`clean_dupes`/`update_auto_tangents`/`get_index`/`sample`/`sample_local_nocheck`/
`sample_baked`/`bake`/`set_bake_resolution`/`get_limits`/`set_limits`/`set_min_value`/
`set_max_value`/`set_min_domain`/`set_max_domain`/`get_data`/`set_data`/`ensure_default_setup`/
`get_point_count` (the last one defined inline in the header). Private helpers (`_add_point`,
`_remove_point`, `_bake`, `mark_dirty`, `_filter_property`) were deliberately left uninstrumented -
every caller of each is itself one of the instrumented public methods, so they always run with the
lock already held by their caller. `_THREAD_SAFE_CLASS_`'s `Mutex` is **recursive**
(`core/os/mutex.h`, `MutexImpl<std::recursive_mutex>`), which is load-bearing here: several
instrumented methods call other instrumented methods on the same thread while already holding the
lock (e.g. `add_point()` -> `_add_point()` -> `get_index()`, `set_point_value()` ->
`update_auto_tangents()`, `ensure_default_setup()` -> `add_point()`/`set_min_value()`/
`set_max_value()`) - a non-recursive `BinaryMutex` would self-deadlock on the very first such call.

Verified: 8/8 clean runs of `gdscript_repro/` against the fix (vs. crashing reliably - 5/5 - on
unfixed runs).

- **Applied for real** to `scene/resources/curve.h`/`.cpp`, rebuilt the ASan editor binary (full
  rebuild of dependents since `curve.h` is a widely-included header, ~2m13s), ran
  `gdscript_repro/` and the test suite against the fix, then reverted (`git checkout --
  scene/resources/curve.h scene/resources/curve.cpp`) before finishing this candidate - main tree
  confirmed clean (`git status`) afterward.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan +
  debug_symbols binary, absolute `XDG_DATA_HOME`): `1431 test cases | 1430 passed | 1 failed | 3
  skipped`, identical to the established baseline (same pre-existing `test_ip.cpp` IPv6 failure,
  unrelated to this change). No new failures.

## What's not done / open questions

- **`Curve2D`/`Curve3D`** (siblings in the same file, named in the same external-sweep backlog
  entry) have the identical unguarded-`LocalVector`-plus-lazy-bake-cache shape (`points`/
  `baked_cache_dirty`/`baked_point_cache`/etc. in both classes) but were not independently
  confirmed or fixed this session - `Curve` was picked as "the simplest API surface" per the
  backlog's own suggestion; `Curve2D`/`Curve3D` remain open follow-up candidates with the same
  expected fix shape.
- The other 9 classes named in the same original backlog entry (`BitMap`, `Image`, `MultiMesh`,
  `CryptoKey`, `GLTFDocument`, `NavigationPolygon`, `ENetConnection`, `StreamPeerGZIP`) were not
  investigated this session.
- `get_min_value()`/`get_max_value()`/`get_min_domain()`/`get_max_domain()`/`get_value_range()`/
  `get_domain_range()`/`get_bake_resolution()` (all trivial inline getters in `curve.h`, returning
  a single `real_t`/`int`) were **not** instrumented - these read a single POD field each, not a
  container, so the worst case is a torn/stale read rather than a crash; left as a known,
  lower-severity gap rather than guessed at, consistent with similar notes in candidates 012/015.
