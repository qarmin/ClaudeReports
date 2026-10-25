# `Curve2D`/`Curve3D` have no locking anywhere in either class

`scene/resources/curve.h`/`.cpp` declares `Curve2D`/`Curve3D`, siblings in the same file of the
already-fixed `Curve` class (candidate 016), with the identical shape: an unguarded
`LocalVector<Point> points` member plus a lazily-rebuilt `baked_cache_dirty`/`baked_point_cache`/
`baked_dist_cache`/etc. cache, mutated by `add_point()`/`remove_point()`/`set_point_position()`/
etc. and read by `sample()`/`sample_baked()`/`get_point_position()`/`tessellate()`/etc., with
**zero** synchronization - confirmed via `grep -n "Mutex" scene/resources/curve.h` returning no
matches for either class before this fix. Both are ordinary `Resource`s (e.g. `Path2D`/`Path3D`'s
`curve` property) that can be shared across multiple owners and accessed concurrently, e.g. from a
background thread doing procedural path generation while the main thread samples it for navigation/
physics. Flagged in candidate 016's own "What's not done" section as a follow-up with the same
expected fix shape, not yet independently confirmed - this candidate builds a dedicated repro for
each class rather than assuming the fix transfers unchanged (per candidate 014's lesson about not
assuming identical-looking siblings behave identically).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `curve.h`/`.cpp` lock-usage audit (`grep -n "Mutex"`) for `Curve2D`/`Curve3D` | n/a | Zero matches for either class - no synchronization primitive of any kind |
| GDScript-level repro (`gdscript_repro/`, modeled on candidate 016's methodology: one shared `Curve2D` + one shared `Curve3D`, 3+3 worker `Thread`s mixing `add_point`/`remove_point`/`samplef`/`sample_baked`/`get_point_position`), against a real, self-built ASan editor binary | `scons platform=linuxbsd target=editor use_asan=yes tests=yes debug_symbols=yes cpp_compiler_launcher=ccache c_compiler_launcher=ccache` (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb, built in an isolated git worktree with a hardlinked `bin/`), `--headless --path gdscript_repro` under `timeout -k 5 25` | **Crashed on every attempt, 5/5 repeat runs** - a mix of real, fully-symbolized fatal bounds-check traps (`SIGILL`, e.g. `Curve2D`'s `sample`/`sample_baked` at `curve.cpp:768`/`1026` and `Curve3D`'s at `curve.cpp:1476`/`1937`, reading a `LocalVector<Point>`/`Vector<real_t>` concurrently resized by another thread's `add_point`/`remove_point`) and one real ASan `attempting double-free` heap-corruption report (run 1) |
| Fix (`_THREAD_SAFE_CLASS_`/`_THREAD_SAFE_METHOD_` across Curve2D's 26 and Curve3D's 38 public/`ClassDB`-bound methods touching `points`/`baked_*_cache`/`bake_interval`/`closed`/`up_vector_enabled`), same build/run, 8 runs | Same build/run | **Clean 8/8 runs**, no crashes, no ASan errors |
| Project test suite before/after | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Before (unpatched): `1431 \| 1430 passed \| 1 failed \| 3 skipped` (established baseline, pre-existing `test_ip.cpp` IPv6 failure). After (patched): one run showed an extra, non-reproducing `[TranslationCSV] CSV import` failure (flaky doctest/locale-state interaction, confirmed unrelated - see `output.txt`); a second run against the same patched binary matched the baseline exactly, `1431 \| 1430 passed \| 1 failed \| 3 skipped` |

## Status

**Confirmed.** Reproduced on every attempt, 5/5 repeat runs of the repro script (unpatched), and
8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`gdscript_repro/`), run against a real,
self-built ASan editor binary, driving the actual `Curve2D`/`Curve3D` classes through nothing but
their public GDScript API (`add_point()`/`remove_point()`/`samplef()`/`sample_baked()`/
`get_point_position()`). No second participant is involved (not a peer/wire-input finding) - this
is a same-process data race between GDScript `Thread`s sharing one `Curve2D`/`Curve3D` resource
each.

## Class

`memory-safety` (concurrent unsynchronized `LocalVector`/`Vector` mutation and lazy-cache rebuild -
same mechanism as candidate 016's `Curve` finding, manifesting as either a real ASan heap
corruption report or a fatal, intentional bounds-check trap depending on the exact interleaving)

## Root cause

`Curve2D::points`/`Curve3D::points` (`LocalVector<Point>`) are mutated by `add_point()`/
`_add_point()`/`remove_point()`/`_remove_point()`/`clear_points()`/`set_point_count()`/`_set_data()`
/etc. and read by `sample()`/`samplef()`/`get_point_position()`/`tessellate()`/`_get_data()`/etc.,
all with zero locking. Separately, `baked_point_cache`/`baked_dist_cache`/`baked_tilt_cache`/
`baked_up_vector_cache`/`baked_forward_vector_cache`/`baked_cache_dirty` form a lazy cache rebuilt
by `_bake()` whenever `sample_baked()`/`get_baked_length()`/`get_closest_point()`/etc. find it
dirty - also completely unguarded. Any two threads calling into this public API concurrently - one
resizing/reordering `points` via `add_point`/`remove_point`, another reading via `sample`/
`sample_baked`/`get_point_position`/`tessellate` - race on the same backing storage. Depending on
timing this either corrupts memory in a way ASan catches directly (the `attempting double-free`
report captured in run 1, from a `Vector`/`LocalVector` backing buffer being freed/resized out from
under a concurrent reader) or lands on a transiently out-of-range index that Godot's own bounds
check catches as a fatal, intentional trap - both are real manifestations of the identical
underlying race, just differing in which specific unsynchronized read loses the race against a
concurrent resize. Identical mechanism to candidate 016's `Curve`, confirmed independently for both
`Curve2D` and `Curve3D` (both code paths crashed across the 5 unpatched repeat runs).

Reachability: both `Curve2D` and `Curve3D` are ordinary, `GDREGISTER_CLASS`-registered `Resource`s,
fully GDScript-instantiable and freely shareable across threads with no special setup - no scene
tree, no node, no editor context required to reach this race.

## Introduced by

`not determined` (predates tracked history in the meaningful sense). Both `Curve2D` and `Curve3D`
have existed in `scene/resources/curve.cpp` since 0b806ee0fc "GODOT IS OPEN SOURCE" (2014-02-09,
the initial open-source commit) with no `Mutex` ever added to either class at any point since -
confirmed via `git log --oneline --diff-filter=A --all -- scene/resources/curve.cpp` returning only
that initial commit. The same situation as candidate 016's own `Curve` finding and candidate 012's
`InputMap`: not a regression from a later refactor, the classes' original, unchanged thread-safety
posture across the engine's entire public history. (`points` was migrated from a plain `Vector` to
`LocalVector` later, in 8bfb5d74b3 "Use LocalVector for Curve and Gradient" - this changed the
container type but not the absence of locking, so it is not the introducing commit for the race
itself.)

## The fix

Add `_THREAD_SAFE_CLASS_` (a recursive `Mutex`, `core/os/thread_safe.h`) as a private member of
both `Curve2D` and `Curve3D`, and `_THREAD_SAFE_METHOD_` as the first statement of effectively every
public method (plus the two `ClassDB`-bound-but-private `_get_data()`/`_set_data()` methods, reached
via the internal `"_data"` property used for serialization) that touches `points`, `baked_*_cache`,
`bake_interval`, `closed`, or `up_vector_enabled` - matching the exact convention candidate 016
established for the sibling `Curve` class (`+133/-0` across `scene/resources/curve.h`/`.cpp`, see
`patch.diff`):

```cpp
// curve.h
#include "core/os/thread_safe.h"
...
class Curve2D : public Resource {
	...
	_THREAD_SAFE_CLASS_

	LocalVector<Point> points;
	...
};

// curve.cpp
Vector2 Curve2D::sample_baked(real_t p_offset, bool p_cubic) const {
	_THREAD_SAFE_METHOD_
	...
}
```

26 methods instrumented on `Curve2D`, 38 on `Curve3D` (counts verified by diffing the full list of
`public:`-section method definitions against the patched `.cpp` - every public method plus
`_get_data`/`_set_data` has exactly one `_THREAD_SAFE_METHOD_`, no more, no fewer). Private helpers
(`_add_point`, `_remove_point`, `mark_dirty`, `_bake_segment{2,3}d*`, `_calculate_tangent`, `_bake`,
`_find_interval`, `_sample_baked`, `_sample_posture`, `_compose_posture`, `_sample_baked_tilt`,
`_tessellate_even_length`, `_filter_property`) were deliberately left uninstrumented - every caller
of each is itself one of the instrumented public methods, so they always run with the lock already
held by their caller. As with candidate 016, `_THREAD_SAFE_CLASS_`'s `Mutex` is **recursive**, which
is load-bearing here: several instrumented methods call other instrumented methods on the same
thread while already holding the lock (e.g. `add_point()` -> `_add_point()` is private/unlocked, but
`samplef()` -> `sample()`, `remove_point()` -> `set_closed()` on `Curve3D`, `get_points()` ->
`_get_data()` all call one instrumented public/bound method from another) - a non-recursive
`BinaryMutex` would self-deadlock on the first such call.

Verified: 8/8 clean runs of `gdscript_repro/` against the fix (vs. crashing reliably - 5/5 - on
unfixed runs).

- **Applied for real** to `scene/resources/curve.h`/`.cpp` in an isolated git worktree
  (`.worktrees/027`, hardlinked `bin/`/`.sconsign5.dblite` from the main tree per this project's own
  disk-space-saving convention), rebuilt the ASan editor binary (~3 min with `ccache` warm), ran
  `gdscript_repro/` and the test suite against the fix, then removed the worktree entirely before
  finishing this candidate - main tree confirmed clean (`git status`) throughout, since the fix was
  never applied to the main tree's own working copy at all.
- Project test suite (`bin/godot.linuxbsd.editor.x86_64.san --headless --test`, same ASan +
  debug_symbols binary, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME`): before (unpatched, main tree's
  own prebuilt binary) `1431 test cases | 1430 passed | 1 failed | 3 skipped` - matches the
  established baseline (pre-existing `test_ip.cpp` IPv6 failure). After (patched): first run showed
  `1431 | 1429 passed | 2 failed | 3 skipped` (the baseline `test_ip.cpp` failure plus one
  `[TranslationCSV] CSV import` failure); isolating that one test case alone
  (`--test-case="*CSV import*"`) passes cleanly even on the *unpatched* binary, and a second full
  run against the same patched binary with a fresh `XDG_DATA_HOME` came back exactly at the
  baseline (`1431 | 1430 passed | 1 failed | 3 skipped`) - confirming the extra failure was a
  one-off doctest ordering/locale-state flake (unrelated global `TranslationServer` singleton state
  leaking between test cases), not a regression introduced by this patch. No new deterministic
  failures from the fix.

## What's not done / open questions

- The other 9 classes named in the original external-sweep backlog entry that candidate 016 also
  left open (`BitMap`, `Image`, `MultiMesh`, `CryptoKey`, `GLTFDocument`, `NavigationPolygon`,
  `ENetConnection`, `StreamPeerGZIP`) were not investigated this session.
- `get_bake_interval()`/`is_closed()`/`is_up_vector_enabled()` etc. (trivial single-field getters)
  were instrumented anyway (unlike candidate 016's deliberate exception for `Curve`'s own
  single-`real_t` getters) for consistency across the whole class, since the per-call cost of an
  already-warm recursive `Mutex` lock is negligible and it closes even the lower-severity
  torn-read gap candidate 016 left open for `Curve`. This is a slightly broader scope than 016's,
  noted here rather than silently diverging.
- `Curve2D`/`Curve3D`'s `property_helper`-mediated per-point properties (`point_0/position`,
  `point_0/in`, etc., used by the editor's inspector) route through the already-instrumented
  `set_point_position`/`get_point_position`/etc., so they're covered transitively - not
  independently exercised by `gdscript_repro/`, which only calls the direct public methods.
