# `BitMap` has no locking anywhere in the class

From `IDEAS.md`'s backlog: `BitMap` was flagged (alongside `Image`, `MultiMesh`, `CryptoKey`,
`GLTFDocument`, `NavigationPolygon`, `ENetConnection`, `StreamPeerGZIP`) as a plain unsynchronized
container with zero `Mutex` anywhere in its header. Unlike candidate 034's `MultiMesh` (ruled out -
no local buffer, storage delegated to `RenderingServer` via `RID`), `BitMap` keeps a genuine local
`Vector<uint8_t> bitmask` plus `width`/`height` scalars, touched directly by every accessor - the
same shape as candidates 016/027's confirmed `Curve`/`Curve2D`/`Curve3D` bugs.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `scene/resources/bit_map.cpp`/`.h` - confirmed zero `Mutex` anywhere in the header, and `create()`/`resize()` mutate `bitmask`/`width`/`height` as 3 separate unsynchronized steps while `set_bit()`/`get_bit()` read `width`/`height` then index `bitmask` directly with no lock | Static read | Byte-for-byte the same "resize races reader/writer on the same local buffer" shape as candidates 016/027; confirmed `create`/`set_bit`/`get_bit`/`resize` are all `ClassDB`-bound (`scene/resources/bit_map.cpp:708-720`), reachable from GDScript |
| GDScript-level repro (`repro/`: shared `BitMap`, 2 threads oscillating `create(Vector2i(1000,1000))`/`create(Vector2i(2,2))`, 4 threads hammering `set_bit(500,500,true)`/`get_bit(500,500)` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/035`, `bin/`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - 2 real ASan reports (`attempting double-free` in `Memory::realloc_static`, `heap-use-after-free` in `CowData<unsigned char>::size()`), both inside `CowData<unsigned char>` operations reached through `BitMap::create()`'s `bitmask.resize()` racing against itself/other accessors |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on the 15 methods that touch `bitmask`/`width`/`height` directly: `create`, `create_from_image_alpha`, `set_bit_rect`, `get_true_bit_count`, `set_bit`, `get_bit`, `get_size`, `_set_data`, `_get_data`, `_march_square`, `clip_opaque_to_polygons`, `grow_mask`, `resize`, `convert_to_image`, `blit`; pure one-line forwarders `set_bitv`/`get_bitv`/`shrink_mask` left unguarded since the method they forward to already locks, and `Mutex` is recursive so no deadlock risk either way), incremental rebuild (ccache-warm hardlinked worktree, ~105s - first rebuild after the edit, touches a widely-included test file) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (two independently symbolized ASan reports: a
double-free and a heap-use-after-free, both inside `CowData<unsigned char>`), and 8/8 clean runs
with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `BitMap` class through nothing but the public GDScript API.
No second participant is involved - a same-process crash reachable from GDScript `Thread`s calling
`create()`/`set_bit()`/`get_bit()` concurrently on a shared `BitMap`.

## Class

`memory-safety` (heap corruption - concurrent `create()` calls racing `bitmask.resize()` against
each other, and against `set_bit()`/`get_bit()`'s direct unguarded indexing, corrupts the
`CowData<uint8_t>` backing store; manifests as either a double-free or a use-after-free depending
on timing).

## Root cause

`BitMap::create()`:

```cpp
void BitMap::create(const Size2i &p_size) {
	ERR_FAIL_COND(p_size.width < 1);
	ERR_FAIL_COND(p_size.height < 1);
	ERR_FAIL_COND(static_cast<int64_t>(p_size.width) * static_cast<int64_t>(p_size.height) > INT32_MAX);

	Error err = bitmask.resize(Math::division_round_up(p_size.width * p_size.height, 8));   // <-- unguarded resize
	ERR_FAIL_COND(err != OK);

	width = p_size.width;
	height = p_size.height;

	memset(bitmask.ptrw(), 0, bitmask.size());
}
```

and `BitMap::set_bit()`/`get_bit()`:

```cpp
void BitMap::set_bit(int p_x, int p_y, bool p_value) {
	ERR_FAIL_INDEX(p_x, width);          // <-- reads width/height with no lock
	ERR_FAIL_INDEX(p_y, height);
	int ofs = width * p_y + p_x;
	int bbyte = ofs / 8;
	...
	uint8_t b = bitmask[bbyte];          // <-- indexes bitmask with no lock
	...
	bitmask.write[bbyte] = b;
}
```

`bitmask` is a plain `Vector<uint8_t>` (`CowData`-backed) with zero synchronization anywhere in the
class. Two threads calling `create()` concurrently race each other's `bitmask.resize()` ->
`CowData::_realloc_exact()` -> `Memory::realloc_static()`, corrupting the shared allocation
(observed as `attempting double-free`). A thread calling `set_bit()`/`get_bit()` concurrently with
another thread's `create()` reads a stale `width`/`height` (computed against the *old* size) and
then indexes `bitmask` - which may already have been freed/resized by the concurrent `create()` -
observed as `heap-use-after-free` inside `CowData<unsigned char>::size()`.

Reachability: `create`/`set_bit`/`get_bit`/`resize` are all bound, GDScript-callable public methods
(`scene/resources/bit_map.cpp:708-720`) - `BitMap` is an ordinary shareable `Resource`, reachable
from any thread with no special setup.

## Introduced by

not determined - `scene/resources/bit_map.cpp` has existed since the initial open-source commit
(0b806ee0fc9097fa7bda7ac0109191c9c5e0a1ac "GODOT IS OPEN SOURCE", 2014-02-09; confirmed via `git
log --follow`) and has never had any locking added since. The specific unguarded shape (`create()`
mutating `bitmask`/`width`/`height` as 3 separate steps, `set_bit`/`get_bit` reading them with no
lock) predates tracked history in this form.

## The fix

`+30/-0` against `scene/resources/bit_map.cpp`, `+3/-0` against `scene/resources/bit_map.h` (see
`patch.diff`): add `_THREAD_SAFE_CLASS_` (declares a `mutable Mutex`, `core/os/thread_safe.h`) next
to the class's private members, and `_THREAD_SAFE_METHOD_` at the top of every method that touches
`bitmask`/`width`/`height` directly - `create`, `create_from_image_alpha`, `set_bit_rect`,
`get_true_bit_count`, `set_bit`, `get_bit`, `get_size`, `_set_data`, `_get_data`, `_march_square`,
`clip_opaque_to_polygons`, `grow_mask`, `resize`, `convert_to_image`, `blit` - 15 methods total.
`set_bitv`/`get_bitv`/`shrink_mask` (one-line forwarders to `set_bit`/`get_bit`/`grow_mask`) were
left unguarded since the method they forward to already takes the lock and `Mutex`
(`core/os/mutex.h`) is recursive, so there's no deadlock risk either way - this matches the same
scoping candidates 016/027 used for `Curve`/`Curve2D`/`Curve3D`.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed, with 2
independently-symbolized distinct ASan failure modes).

- **Applied for real** in an isolated worktree (`.worktrees/035`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- The remaining siblings named in the same `IDEAS.md` backlog entry (`Image`, `CryptoKey`,
  `GLTFDocument`, `NavigationPolygon`, `ENetConnection`, `StreamPeerGZIP`) were not investigated
  this session - per the lesson from candidate 034's `MultiMesh` rule-out, each should be checked
  for a genuine local mutable buffer (this class's own shape) before assuming the same bug
  transfers; `Image` in particular is a strong next candidate (it keeps a local `Vector<uint8_t>
  data` buffer analogous to `bitmask`).
- `_opaque_to_polygons_bind` (private, called only by the bound `_bind_methods` wrapper) was left
  unguarded since it only calls the already-locked `clip_opaque_to_polygons()` and doesn't touch
  the fields directly - not independently verified as safe beyond that reasoning.
