# `Image` has no locking anywhere in the class

From `IDEAS.md`'s backlog, picked as the strongest remaining sibling after candidate 034's
`MultiMesh` rule-out and candidate 035's confirmed `BitMap` fix: `Image` keeps a genuine local
`Vector<uint8_t> data` buffer (`core/io/image.h:295`) plus `width`/`height`/`format`/`mipmaps`
scalars, touched directly by its own accessors - the same shape as candidates 016/027/035's
confirmed `Curve`/`Curve2D`/`Curve3D`/`BitMap` bugs. Confirmed zero `Mutex` anywhere in the header
before investigating.

`Image` has an unusually large public API (~108 public methods per `core/io/image.h`) - per this
project's own "Scope lesson" (noted after candidate 018's `PopupMenu`), this candidate's fix is
**deliberately scoped** to the methods exercised by the confirmed repro plus the shared internal
mutator, not a full-class sweep - see "What's not done" below.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `core/io/image.cpp`/`.h` - confirmed zero `Mutex` anywhere in the header; `Image::resize()` builds a temporary scaled `Image dst` and finishes with `_copy_internals_from(dst)`, which overwrites `format`/`width`/`height`/`mipmaps`/`data` as 5 separate unsynchronized steps; `set_pixel()`/`get_pixel()` read `width`/`height` then index `data` directly, all with no lock | Static read | Byte-for-byte the same "resize races reader/writer on the same local buffer" shape as candidates 016/027/035; confirmed `resize`/`set_pixel`/`get_pixel`/`set_data` are all `ClassDB`-bound, reachable from GDScript |
| GDScript-level repro (`repro/`: shared `Image` (1000x1000 RGBA8), 2 threads oscillating `resize(1000,1000)`/`resize(2,2)`, 4 threads hammering `set_pixel(500,500,...)`/`get_pixel(500,500)` for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/036`, `bin/`/`.sconsign5.dblite` hardlinked from the main tree) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 30 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **Crashed on all 3 attempts** - a real ASan heap-use-after-free, same crash site (`core/io/image.cpp:1042` in `_scale_bilinear<4, unsigned char, (ImageScaleType)0>`, reached from `Image::resize()`) across all 3 runs: two concurrent `resize()` calls race each other, one reading `this->data`/`width`/`height` inside `_scale_bilinear` while the other thread's `resize()` has already freed/reallocated that same buffer via `_copy_internals_from` |
| Fix (`_THREAD_SAFE_CLASS_` on the header + `_THREAD_SAFE_METHOD_` on `resize()`, `set_pixel()`, `get_pixel()`, `set_data()`, `_copy_internals_from()` - the methods directly exercised by the confirmed race plus the shared internal mutator every other data-reassigning path (e.g. `convert()`) routes through), incremental rebuild (ccache-warm hardlinked worktree, ~11.5 min - `image.h` is a very widely-included header, matching `projects/godot.md`'s documented "~10-11 min for a widely-included header" case) | Same build/run | **Clean 8/8 runs**, "survived" printed every time |
| Project test suite before/after (revert to unfixed source, rebuild ~11.5 min, run; reapply fix) | `bin/godot.linuxbsd.editor.x86_64.san --headless --test`, absolute `XDG_DATA_HOME`/`XDG_CONFIG_HOME` | Identical both before and after: `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - same pre-existing `test_ip.cpp` IPv6 environment failure already documented in every prior candidate's baseline |

## Status

**Confirmed.** Reproduced on 3/3 unfixed runs (a real ASan heap-use-after-free, same crash site
every time - deterministic enough to reproduce on the very first attempt and every attempt after),
and 8/8 clean runs with the fix applied.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving the actual `Image` class through nothing but the public GDScript API.
No second participant is involved - a same-process crash reachable from GDScript `Thread`s calling
`resize()`/`set_pixel()`/`get_pixel()` concurrently on a shared `Image`.

## Class

`memory-safety` (heap-use-after-free - two concurrent `resize()` calls race each other on the same
`data`/`width`/`height` fields; one thread's `_scale_bilinear` reads the buffer while another
thread's `_copy_internals_from` has already freed/reallocated it via `Vector`'s `CowData`).

## Root cause

`Image::resize()` ends with:

```cpp
void Image::resize(int p_width, int p_height, Interpolation p_interpolation) {
	...
	Image dst(p_width, p_height, false, format);
	... // _scale_bilinear/_scale_nearest reads this->data/width/height into dst, unguarded
	_copy_internals_from(dst);
	...
}

void Image::_copy_internals_from(const Image &p_image) {
	format = p_image.format;
	width = p_image.width;
	height = p_image.height;
	mipmaps = p_image.mipmaps;
	data = p_image.data;          // <-- reassigns/frees the old backing buffer, no lock
}
```

`data` is a plain `Vector<uint8_t>` (`CowData`-backed) with zero synchronization anywhere in the
class. Two threads calling `resize()` concurrently both read `this->data`/`width`/`height` (inside
the scaling loop, e.g. `_scale_bilinear`) while the other thread's own `_copy_internals_from()` can
concurrently free/replace that exact buffer - confirmed via ASan: `heap-use-after-free ... in
_scale_bilinear<4, unsigned char, (ImageScaleType)0> core/io/image.cpp:1042`, with the `freed by`
stack showing `CowData<unsigned char>::_unref()` from the concurrent thread's own `resize()` call.
`set_pixel()`/`get_pixel()` share the identical unguarded-read shape against the same fields,
though this particular repro's crashes all happened to land on the resize-vs-resize path rather
than resize-vs-pixel-access, both reachable through the same missing lock.

Reachability: `resize`/`set_pixel`/`get_pixel`/`set_data` are all bound, GDScript-callable public
methods - `Image` is an ordinary shareable `Resource`, reachable from any thread with no special
setup.

## Introduced by

not determined - `core/io/image.cpp` has existed since the initial open-source commit
(0b806ee0fc9097fa7bda7ac0109191c9c5e0a1ac "GODOT IS OPEN SOURCE", 2014-02-09; confirmed via `git
log --follow`) and has never had any locking added since.

## The fix

`+10/-0` against `core/io/image.cpp`, `+3/-0` against `core/io/image.h` (see `patch.diff`): add
`_THREAD_SAFE_CLASS_` (declares a `mutable Mutex`, `core/os/thread_safe.h` - needed an explicit
`#include`, same as candidate 035's `BitMap`) next to the private members, and
`_THREAD_SAFE_METHOD_` at the top of `resize()`, `set_pixel()`, `get_pixel()`, `set_data()`, and
`_copy_internals_from()`.

Verified: 8/8 clean runs of `repro/` against the fix (vs. 3/3 crashing unfixed).

- **Applied for real** in an isolated worktree (`.worktrees/036`, created from and removed back
  into the main project's own clean tree - never touched the main working tree directly), rebuilt
  the ASan editor binary, ran `repro/` and the full test suite against both the unfixed and fixed
  source, then discarded the worktree (`git worktree remove --force`) - main tree confirmed clean
  (`git status`) throughout and at the end.
- Project test suite: `1431 test cases | 1430 passed | 1 failed | 3 skipped` both before (unfixed
  source, freshly rebuilt) and after the fix - identical to the established baseline (same
  pre-existing `test_ip.cpp` IPv6 failure, unrelated to this change). No new failures.

## What's not done / open questions

- **This fix is deliberately narrow, not a full-class sweep** (same documented tradeoff as
  candidate 018's initial `PopupMenu` scope, before 030 closed out the rest) - `Image` has ~108
  public methods, the vast majority (`crop`, `fill`, `blit_rect`, `convert`, `compress`,
  `generate_mipmaps`, every `save_*`/`load_*`, etc.) are not covered by this patch and remain
  reachable off the main thread with no locking at all. A future candidate could do the same
  scripted-regex-insertion approach candidate 030 used for `PopupMenu`'s ~80 remaining methods to
  close this out fully.
- **`get_data()` was deliberately left unguarded, not an oversight.** It returns `const
  Vector<uint8_t>&` (`_LIFETIME_BOUND_`) - a reference into `data` itself. Locking inside
  `get_data()` would release the lock the moment the function returns, while the caller still holds
  and dereferences the now-unprotected reference - the same "caller outlives the lock" shape
  candidate 015 (`CSGShape3D`) hit and had to fix by extending the lock to the *caller's* use of the
  pointer, not just the accessor. Properly fixing `get_data()` would need an API change (return by
  value, or document/enforce that the caller must not hold the reference across other threads'
  calls) - out of scope for this candidate's narrow fix.
- `convert()`, `crop()`, `fill()`, `blit_rect()`, and other mutators that call `_copy_internals_from()`
  or write `data` directly were not independently given their own repro - the shared
  `_copy_internals_from()` lock covers the one path exercised here, but a direct writer that
  doesn't route through it (if one exists) would not be covered.
