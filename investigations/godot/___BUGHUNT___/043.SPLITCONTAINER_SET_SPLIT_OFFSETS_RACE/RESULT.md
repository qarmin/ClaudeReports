# `SplitContainer::set_split_offsets()`/`get_split_offsets()` have no locking anywhere in the class

Continues the external thread-safety sweep's `Control` widget lead (candidates 039 `Label`, 040
`LineEdit`, 041 `ItemList`, 042 `Tree`). `SplitContainer` keeps its split-position state in a
`PackedInt32Array split_offsets` (`CowData`-backed), touched directly by `set_split_offset(s)`/
`get_split_offset(s)`/`clamp_split_offset`, with zero `Mutex`/`_THREAD_SAFE_CLASS_` anywhere in the
header - the same `CowData`-refcount race shape already confirmed for `String` (candidates 038/040)
and `Vector`-backed containers (035/036/037/041/042), just through `PackedInt32Array` this time.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/gui/split_container.h`/`.cpp` - confirmed zero `Mutex` anywhere in the header; `set_split_offsets()` directly assigns `split_offsets = p_offsets` (a `PackedInt32Array`) with no lock, `get_split_offsets()` returns it by value also with no lock | Static read | Confirmed `set_split_offsets`/`get_split_offsets` (and the singular `set_split_offset`/`get_split_offset`/`clamp_split_offset`) are all `ClassDB`-bound, reachable from GDScript |
| GDScript-level repro (`repro/`: shared `SplitContainer`, 3 threads alternately assigning `.split_offsets` between a 200-element and a 1-element `PackedInt32Array` (forcing `CowData` reallocation), 3 threads reading `.split_offsets`/`.size()` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/043`, `bin`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - real ASan heap-use-after-free every time, inside `SafeNumeric<unsigned long>::decrement()` (`core/templates/safe_refcount.h`), reached through `CowData<int>::_unref() <- Vector<int>::~Vector() <- Variant::PackedArrayRef<int>::~PackedArrayRef()` - the `PackedInt32Array`'s own refcount, corrupted by the concurrent unguarded assignment |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on `set_split_offset`, `get_split_offset`, `set_split_offsets`, `get_split_offsets`, `clamp_split_offset` - the 5 methods that directly touch `split_offsets`), incremental rebuild (ccache-warm hardlinked worktree, ~4m10s) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~3m37s, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (a real ASan heap-use-after-free every time, on
`PackedInt32Array`'s own `CowData` refcount - deterministic enough to crash on the first attempt),
and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `SplitContainer` class through nothing but the public
GDScript API. No second participant is involved - a same-process crash reachable from GDScript
`Thread`s calling `.split_offsets` setter/getter concurrently on a shared `SplitContainer`.

## Class

`memory-safety` (heap-use-after-free on `PackedInt32Array`'s internal `CowData<int>` refcount -
concurrent `set_split_offsets()` assignments, and `get_split_offsets()`'s unguarded copy, race the
refcount increment/decrement that backs the array's copy-on-write semantics - same mechanism
already confirmed for `String` in candidates 038/040, just through a packed array instead).

## Root cause

```cpp
void SplitContainer::set_split_offsets(const PackedInt32Array &p_offsets) {
	if (split_offsets == p_offsets) {
		return;
	}
	split_offsets = p_offsets;        // <-- unguarded PackedInt32Array assignment
	...
}

PackedInt32Array SplitContainer::get_split_offsets() const {
	return split_offsets;             // <-- unguarded copy
}
```

`split_offsets` (`PackedInt32Array`, `CowData<int>`-backed) is a plain member with zero
synchronization anywhere in the class. Two threads racing `set_split_offsets()` against each
other, or against `get_split_offsets()`'s copy, can both observe the same refcount and both decide
to free (or one frees while the other still holds a reference it believes is valid) - confirmed via
ASan: `heap-use-after-free ... in SafeNumeric<unsigned long>::decrement()`, reached through
`CowData<int>::_unref() <- Vector<int>::~Vector() <- Variant::PackedArrayRef<int>::~PackedArrayRef()`.

Reachability: `set_split_offset(s)`/`get_split_offset(s)`/`clamp_split_offset` are all bound,
GDScript-callable public methods, with `split_offsets` exposed as a regular script property -
`SplitContainer` is an ordinary `Container`-derived node, reachable from any thread with no special
setup once added to the tree.

## Introduced by

7e55f1d1bf305f137d05937cb144ac1831c73159 "Allow SplitContainer to have multiple children"
(2025-04-01) - the commit that introduced `split_offsets` as a `PackedInt32Array` (replacing a
single scalar `split_offset`), already missing the guard.

## The fix

`+10/-0` against `scene/gui/split_container.cpp`, `+3/-0` against `scene/gui/split_container.h`
(see `patch.diff`): add `_THREAD_SAFE_CLASS_` next to the private members, and
`_THREAD_SAFE_METHOD_` to `set_split_offset()`, `get_split_offset()`, `set_split_offsets()`,
`get_split_offsets()`, `clamp_split_offset()` - the 5 methods that directly touch `split_offsets`.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed).

- **Applied for real** in an isolated worktree (`.worktrees/043`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- `SplitContainer` has 48 public methods; this fix covers only the 5 that touch `split_offsets`
  directly. The class's other mutable state (`default_dragger_positions`, `dragger_positions`,
  `valid_children`, `dragging_area_controls` - all `LocalVector`s rebuilt during layout/`_resort()`)
  remains completely unguarded, though these are primarily touched during the main-thread layout
  pass rather than through direct public setters/getters like `split_offsets` is.
- **5 widgets remain** from the original external-sweep list: `CodeEdit`, `GraphEdit`,
  `OptionButton`, `TabBar`, `TextEdit`.
