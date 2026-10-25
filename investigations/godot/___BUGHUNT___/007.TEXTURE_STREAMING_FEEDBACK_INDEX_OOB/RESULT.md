# `TextureStreaming::feedback_buffer_material_index()` out-of-bounds write on an unregistered RID

Found during a broader sweep of this session: after confirming candidates 002-006 all stemmed
from the same family of patterns (locked-writer/unlocked-reader, destructive error paths), this
candidate looked for other classes following the same `mutable Mutex` + index/RID bookkeeping
shape. `grep -rln "mutable Mutex\|mutable BinaryMutex"` across `core/`, `scene/`, `servers/`,
`modules/` turned up `modules/texture_streaming/texture_streaming.h`'s
`RID_IndexedOwner<T>::get_index()` - and, unlike the mutex itself (which turned out to be a
dead end, see "What was tried"), tracing its one real caller led to a genuine, unconditional
out-of-bounds write.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `RID_IndexedOwner<T>::get_index()`/`free()` lock symmetry (`texture_streaming.h:228-295`) | n/a | `get_index()` reads `wrapper->index` (a plain `uint32_t`) with **no lock**, while `free()` writes the same field (on a *different* element, during its swap-remove) under `index_mutex`. Since `uint32_t` reads/writes are atomic on x86-64, this specific race is benign (a stale-but-not-torn read) by itself - **not** pursued further as its own finding |
| Code reading: who uses `get_index()`'s return value, and how | n/a | `TextureStreaming::feedback_buffer_material_index()` (`texture_streaming.cpp:457-472`) uses the index to resize-then-write into `MaterialFeedbackBuffer::rid_map` with **no validity check at all**: `get_index()` returns the real sentinel `INVALID_INDEX` (`0xFFFFFFFF`) for any RID that isn't currently registered (never allocated, or already freed) - whether or not any race is involved - and `index + 1` silently overflows to `0` for that sentinel, skipping the resize that would otherwise make room for `rid_map[index]` |
| Code reading: reachability | n/a | Called from `servers/rendering/renderer_rd/forward_mobile/render_forward_mobile.cpp:2185` and `.../forward_clustered/render_forward_clustered.cpp:886`, once per rendered surface with texture-streaming feedback enabled, on the render thread - a real, normal rendering path, not a contrived one. Not `ClassDB`-bound (confirmed via `grep` of `_bind_methods()`) and requires an actual GPU/renderer backend - unreachable from `--headless` GDScript, hence the C++-mirror fallback (see `repro/README.md`) |
| Standalone C++ mirror repro, ASan, real `core/templates/local_vector.h`/`rid.h` | g++ 15, `-fsanitize=address -g -O1 -DASAN_ENABLED -DTHREADS_ENABLED`, single run (fully deterministic, no threading/timing involved) | **Crashed** - not an ASan-reported heap corruption, but a fatal, intentional `CRASH_BAD_UNSIGNED_INDEX` abort (`GENERATE_TRAP()`, SIGILL) from `LocalVector::operator[]`'s own bounds check, which is compiled in unconditionally (not gated behind `DEBUG_ENABLED`/`DEV_ENABLED`) - see "Root cause" |
| Fix verification: `repro_fixed.cpp` (same mirror, with the same `index == UINT32_MAX` guard as the real fix) | Same build flags | **Clean** - prints the sentinel value back and exits 0, no crash |
| Real-file compile check: `g++ -c` the actual patched `texture_streaming.cpp` against real project headers (`-DTOOLS_ENABLED -DDEBUG_ENABLED -DTHREADS_ENABLED -DRD_ENABLED`) | Type-check only, no link, no GPU needed | **Compiles cleanly** - the patch is syntactically and semantically valid against the real codebase, not just the mirror |

## Status

**Confirmed.**

The standalone mirror deterministically crashes (fatal trap) on the unfixed arithmetic and runs
clean with the fix, on every run (this is not a race - no threading or timing is involved at all,
unlike every other candidate this session). The real, patched file also compiles cleanly against
the actual project headers.

**Fully reproducible**: **Partially, via C++ only** - no GDScript-level confirmation exists or was
attempted for this candidate (the real function requires an actual GPU-backed renderer, which
`--headless` mode cannot provide - see `repro/README.md` for the full justification, matching this
project's policy exception for bugs with zero GDScript reachability). The C++ mirror reproduces
the exact vulnerable arithmetic using Godot's real `LocalVector` from this checkout, and the real
patched source file was independently confirmed to compile correctly, but the real
`TextureStreaming`/`RID_Owner`/renderer classes were not linked or run end-to-end.

## Class

`memory-safety` (out-of-bounds write, caught before causing silent heap corruption by Godot's own
`CRASH_BAD_UNSIGNED_INDEX` bounds check in `LocalVector::operator[]` - still an unconditional
engine crash, not a graceful error)

## Root cause

`modules/texture_streaming/texture_streaming.cpp:457-472`:

```cpp
uint32_t TextureStreaming::feedback_buffer_material_index(RID p_material) {
    ERR_NOT_ON_RENDER_THREAD_V(UINT32_MAX);
    ERR_FAIL_COND_V(p_material.is_null(), UINT32_MAX);

    MaterialFeedbackBuffer *_buffer = feedback_buffer_owner.get_or_null(current_feedback_buffer);
    if (_buffer) {
        uint32_t index = material_info_owner.get_index(p_material);
        if (_buffer->rid_map.size() <= (index + 1)) {
            _buffer->rid_map.resize(index + 1);
        }
        _buffer->rid_map[index] = p_material;    // <-- unchecked
        return index;
    }
    ...
}
```

`material_info_owner.get_index()` (`texture_streaming.h:262-265`) returns
`RID_IndexedOwner::INVALID_INDEX` (`0xFFFFFFFF`) whenever `p_material` isn't currently registered
in `material_info_owner` - this is not an exceptional/can't-happen case, it's the function's
documented behavior for any RID that was never allocated through this owner, or that has already
been freed. The calling code never checks for this sentinel. When `index == 0xFFFFFFFF`:

- `index + 1` overflows (`uint32_t` wraparound) to `0`.
- `_buffer->rid_map.size() <= 0` is false for any non-empty `rid_map` (which it always is in
  practice, per `MaterialFeedbackBuffer::resize()`), so the `resize()` that would otherwise grow
  the vector to fit `index` is **skipped**.
- `_buffer->rid_map[index]` then indexes with `index = 0xFFFFFFFF` into a vector that was never
  grown to accommodate it - an out-of-bounds write of a 16-byte `RID` value at whatever memory
  address `index * sizeof(RID)` bytes past the buffer happens to be.

`LocalVector::operator[]` (`core/templates/local_vector.h:237-244`) guards every access with
`CRASH_BAD_UNSIGNED_INDEX(p_index, count)` (`core/error/error_macros.h:325-330`) - unlike the
`ERR_FAIL_*` family, this macro is **not** conditioned on `DEBUG_ENABLED`/`DEV_ENABLED`, so it
fires in every build configuration, including release/export templates. This converts what would
otherwise be a silent, exploitable heap write into an unconditional, intentional engine abort
(`GENERATE_TRAP()`) - a real crash, but not a worse-than-crash memory-corruption outcome, thanks
to this defense-in-depth check existing in the engine already.

Reachability: `feedback_buffer_material_index()` is called once per rendered surface with texture
streaming feedback enabled, from the real-time renderer (`render_forward_mobile.cpp:2185`,
`render_forward_clustered.cpp:886`). It fires whenever a material's `material_feedback_rid` is
non-null but not (or no longer) registered with `material_info_owner` - plausible both as a pure
lifecycle bug (the feedback RID is allocated lazily/elsewhere and a material reaches the renderer
before or after that window) and, per the lock-asymmetry noted above, slightly more likely under
concurrent renders/frees, though no race is actually required to hit it.

## Introduced by

`not determined`. `texture_streaming` is a comparatively new/less-reviewed module (this session's
`mutable Mutex` sweep flagged it specifically because its locking has the careful, explicit-TSAN-
annotation style of `core/templates/rid_owner.h`'s hand-audited lock-free code elsewhere in the
same sweep, suggesting active, recent development). Not bisected this session.

## The fix

Check for the `INVALID_INDEX` sentinel immediately after obtaining it, matching the existing
`ERR_FAIL_COND_V(p_material.is_null(), UINT32_MAX);` guard already present two lines earlier in
the same function (`+6/-0` against `modules/texture_streaming/texture_streaming.cpp`, see
`patch.diff`):

```cpp
uint32_t index = material_info_owner.get_index(p_material);
ERR_FAIL_COND_V(index == UINT32_MAX, UINT32_MAX);
if (_buffer->rid_map.size() <= (index + 1)) {
    _buffer->rid_map.resize(index + 1);
}
_buffer->rid_map[index] = p_material;
```

This is the smallest, lowest-risk fix: it adds exactly the validity check the function already
performs for its other input (`p_material.is_null()`), using the project's standard
`ERR_FAIL_COND_V` idiom, and changes no other behavior.

Verified: the standalone mirror (`repro_fixed.cpp`) no longer crashes with the same guard applied,
and the real patched file compiles cleanly against the real project headers.

- **Applied for real** to `modules/texture_streaming/texture_streaming.cpp`, confirmed via
  `g++ -c` against real project headers (no GPU/full-engine build attempted - see "Fully
  reproducible" above), then reverted (`git checkout -- modules/texture_streaming/texture_streaming.cpp`)
  before finishing this candidate - main tree confirmed clean (`git status`) afterward.
- The project's headless `--test` suite was **not** re-run for this candidate: `texture_streaming`
  is a rendering-backend module exercised only through actual GPU draw calls, which the existing
  `--headless --test` suite (used for every other candidate this session) does not exercise either
  way - a before/after comparison would show no difference regardless of this fix's correctness.

## What's not done / open questions

- **No GPU/real-engine confirmation** - this is the one candidate this session without any
  real-engine run (headless or otherwise) backing it, purely because the bug lives in a
  GPU-rendering code path this sandboxed environment cannot exercise. The C++ mirror and the
  real-file compile check are the strongest confirmation available here; a future session with
  GPU access should build a real scene with texture streaming enabled and a material whose
  feedback RID is freed mid-render to get an end-to-end confirmation.
- **The `get_index()`/`free()` lock asymmetry noted in "What was tried"** was investigated and
  judged benign on its own (atomic `uint32_t` read/write, no torn reads on x86-64) - not written
  up as a separate finding, but worth noting for a future session auditing `RID_IndexedOwner<T>`
  more thoroughly, especially on architectures without single-instruction aligned word reads.
- Did not check whether other callers of similarly-shaped `*_owner.get_index()`/index-sentinel
  patterns elsewhere in the engine have the same missing-sentinel-check bug - only this one
  function (the one with a clear external caller in the renderer) was audited this session.
