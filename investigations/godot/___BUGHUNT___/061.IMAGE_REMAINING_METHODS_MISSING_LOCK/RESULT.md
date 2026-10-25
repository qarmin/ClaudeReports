# `Image`'s ~69 other public methods are also unlocked, same race as candidate 036

Follow-up to candidate 036 (`IMAGE_PIXEL_RACE`), which found and fixed the race in exactly 5 methods
(`resize()`, `set_data()`, `_copy_internals_from()`, `get_pixel()`, `set_pixel()`) and explicitly
flagged the other ~95 public methods touching `mutable Vector<uint8_t> data` as a real, not-yet-
confirmed exposure to the identical bug (`IDEAS.md`: "`Image::get_data()` has ~95 of its ~108 public
methods still unguarded"). This candidate confirms that exposure with its own independent repro
(deliberately avoiding `resize()`/`set_data()`/`get_pixel()`/`set_pixel()` entirely) and fixes the
whole remaining surface via a scripted sweep, the same approach candidate 030 used to close out
`PopupMenu`'s ~80 remaining methods.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Enumerated every `ClassDB::bind_method`-registered name in `core/io/image.cpp` (`grep -oP`) - 74 unique public instance methods (plus `load_from_file`, a `bind_static_method`, correctly excluded since a static method has no `this` to lock) | Static read | Got the exact, complete method list to sweep, rather than guessing which ones matter |
| Wrote `lock_image_methods.py` (kept in this folder): a small regex script - `re.compile(r'(^[^\n(]*\bImage::' + method + r'\([^{\n]*\{\n)', re.MULTILINE)` - that finds each method's definition and inserts `_THREAD_SAFE_METHOD_\n\n` right after the opening brace. Excluded the 5 methods candidate 036 already locked, and deliberately excluded `get_data()` (see below) | Ran against a worktree with candidate 036's patch already applied | **68/69 methods patched automatically**; 1 (`copy_from`) not found because its `ClassDB`-bound name differs from its actual C++ method name (`copy_internals_from`) - fixed by hand, same treatment |
| GDScript-level repro (`repro/`: one 256×256 `Image`, 3 writer threads alternating `crop()`+`flip_x()` and `crop()`+`convert()`+`convert()` back, 3 reader threads calling `get_size()`/`is_empty()`/`get_used_rect()` - deliberately avoiding every method candidate 036 already locked), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 15 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro`, run first against the **existing unpatched main-tree binary** (candidate 036's fix never merged, same fix-verify-revert workflow as every other candidate) | **Crashed on 3/3 attempts** - a `heap-use-after-free` inside glibc's `memcpy` (reached through `Image`'s pixel-buffer resize path), a `FATAL` bounds-check abort (exit 134), and a fully-symbolized `heap-use-after-free core/io/image.cpp:543 in _convert<...>` - three different concrete manifestations of the same underlying `data` buffer race |
| Fix (scripted sweep, see above), same build/run | Same build/run, 10 attempts | **Clean 10/10** |
| Project test suite before/after | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` with the fix applied - identical to the established baseline, no new failures |

## Status

**Confirmed.** 3/3 unfixed crashes (three distinct concrete signatures from the same root cause),
10/10 clean with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `Image` through nothing but its own public GDScript API (`crop()`,
`flip_x()`, `convert()`, `get_size()`, `is_empty()`, `get_used_rect()`). No second participant is
involved - this is a same-process data race between GDScript `Thread`s sharing one `Image`.

## Class

`memory-safety` (concurrent unsynchronized `Vector<uint8_t>`/`CowData` resize vs. resize/read on
`Image::data` - the identical shape candidate 036 confirmed for `resize`/`set_pixel`/`get_pixel`,
just reached through a different, much larger set of entry points - same relationship candidate 030
established between its own fix and candidate 018's original `PopupMenu` finding).

## Root cause

Same field, same missing lock, as candidate 036: `Image::data` (`mutable Vector<uint8_t> data`) has
zero synchronization outside the 5 methods candidate 036's own fix covers. Every other public method
that reads or mutates image contents - `crop`, `flip_x`/`flip_y`, `convert`, `fill`/`fill_rect`,
`blit_rect`/`blend_rect` (and their `_mask` variants), `generate_mipmaps`/`clear_mipmaps`,
`compress`/`decompress`, `rotate_90`/`rotate_180`, `shrink_x2`, `resize_to_po2`, every `load_*_from_buffer`,
every `save_*`/`save_*_to_buffer`, `get_size`/`get_width`/`get_height`/`is_empty`/`has_mipmaps`/
`get_used_rect`/`get_region`, etc. - does a plain, unlocked read or mutation of `data`/`width`/
`height`/`format`/`mipmaps`, with no mutual exclusion against any other thread doing the same. Two
threads racing any pair of these (e.g. one calling `crop()` while another calls `get_size()`, or two
threads both calling `convert()`) hit the same `CowData<uint8_t>` reallocation/bounds-check race as
candidate 036, just via a different pair of call sites.

## Introduced by

Not independently dated - same situation as candidate 036's own lookup; the ~69 methods this
candidate fixes were each added at different times, but none of them were ever guarded, consistent
with this being the class's original, unchanged thread-safety posture rather than a regression.

## The fix

`patch.diff` is a complete, standalone patch (candidate 036's own 5-method fix, plus this
candidate's ~69-method scripted addition) - applying it alone to unpatched HEAD fixes the whole
`data`/`width`/`height`/`format`/`mipmaps` race across effectively the entire public surface. `+149/-0`
in `core/io/image.cpp` (5 manual insertions from 036 + 69 from the sweep, `_THREAD_SAFE_METHOD_` as
the first statement of each method body), no further `.h` changes needed beyond 036's own
`_THREAD_SAFE_CLASS_`/`#include` addition.

`Mutex` (as opposed to `BinaryMutex`) is recursive, so methods that internally call other
now-locked methods on the same instance (`get_pixelv()`→`get_pixel()`, `set_pixelv()`→
`set_pixel()`) don't deadlock.

Verified sufficient against the repro: 10/10 clean runs, after 3/3 unfixed crashes with the
identical repro content. Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the fix
applied - identical to the established baseline.

## What's not done / open questions

- **`get_data()` is deliberately left unlocked.** It returns `const Vector<uint8_t> &`, a reference
  into `Image`'s own backing buffer - locking just the accessor function doesn't protect a caller
  that keeps using the returned reference after the function (and its lock) has already returned,
  the same "caller outlives the lock" shape first identified in candidate 015's `CSGShape3D`. Fixing
  this properly would need an API change (return a copy, or document caller discipline) rather than
  a mechanical lock insertion - out of scope for this scripted sweep.
- **`copy_internals_from()`** (bound as `copy_from`) reads fields from a *different* `Image` instance
  (`p_image->format`/`width`/`height`/`mipmaps`/`data`) while only holding `this`'s own lock - the
  same cross-object locking limitation documented in candidate 042 (`Tree`/`TreeItem`). Locking
  `this` protects the write side but not a concurrent mutation of `p_image` by another thread holding
  *its* lock. Not fixed here (would need a defined lock-ordering convention across two different
  `Image` instances); flagged for the `Tree`/`TreeItem` investigation to reference if a general
  pattern/solution for this engine-wide gap is ever designed.
- Private helpers (`_convert`, `_get_dst_image`, format-specific internal compressors/decompressors,
  etc.) still read/write `data` without their own lock - safe today only because every caller of
  these is one of the now-locked public methods above (the recursive mutex covers that case
  correctly) or internal rendering/import code that only ever runs on the main thread.
- Next per the coordinator's priority-3 list: the `Tree`/`TreeItem` cross-object lock gap, then the
  ~22 remaining unaudited `_THREAD_SAFE_CLASS_` files (`scene_tree.h`/`rendering_device.h` flagged
  highest-value).
