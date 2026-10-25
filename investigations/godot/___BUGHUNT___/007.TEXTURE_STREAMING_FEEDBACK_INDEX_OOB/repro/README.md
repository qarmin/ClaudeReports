# Why this candidate has a C++ mirror, not a GDScript repro

`TextureStreaming::feedback_buffer_material_index()` (`modules/texture_streaming/texture_streaming.cpp:457-472`)
is called only from the real-time rendering backend (`servers/rendering/renderer_rd/forward_mobile/render_forward_mobile.cpp:2185`
and `.../forward_clustered/render_forward_clustered.cpp:886`), once per rendered surface, on the
render thread - there is no way to reach it from a `--headless` GDScript run (no GPU/display
context, no renderer backend instantiated at all in `--headless` mode). It is also not
`ClassDB`-bound (confirmed via `grep` of `TextureStreaming::_bind_methods()`,
`texture_streaming.cpp:111-129` - only LOD/memory-budget getters/setters and `flush_texture_streaming`
are exposed to script). This genuinely has no GDScript-reachable entry point, matching this
project's policy exception for falling back to a C++-level repro.

`repro.cpp`/`repro_fixed.cpp` reproduce the bug's exact arithmetic directly - `rid_map[index]` with
`index` fed the real `RID_IndexedOwner::INVALID_INDEX` sentinel (`0xFFFFFFFF`) that
`material_info_owner.get_index()` returns for any unregistered/already-freed material RID - using
Godot's real `core/templates/local_vector.h` from this checkout. The full real-class chain
(`RID_Owner<T, true>`, `RID_IndexedOwner<T>`, the real `TextureStreaming`/`Object`/`ClassDB`
machinery) was judged unnecessary to link standalone: the vulnerable code is pure arithmetic
(`index + 1` overflowing when `index == UINT32_MAX`, skipping the resize that would otherwise make
room for the write) that doesn't depend on how the sentinel value is produced, only on what the
calling code does with it once received - and an earlier attempt to link the real `RID_Owner`
pulled in the full `String`/`Variant` machinery just for unused `ERR_FAIL_V_MSG` diagnostic
strings (same issue documented in candidates 002/003), not worth the added complexity for a bug
whose root cause is this self-contained.
