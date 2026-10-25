// Standalone reproduction of the out-of-bounds write in
// TextureStreaming::feedback_buffer_material_index() (modules/texture_streaming/texture_streaming.cpp:457-472),
// using Godot's REAL core/templates/local_vector.h directly from this checkout.
//
// mini_feedback_buffer_material_index mirrors the real function's body exactly
// (resize-then-write on an index derived from RID_IndexedOwner::get_index(),
// which returns the real INVALID_INDEX sentinel 0xFFFFFFFF for an unregistered/
// already-freed RID - see texture_streaming.h:228-295's RID_IndexedOwner<T>,
// not reproduced here since the sentinel value is the only thing that matters
// for this specific bug: `index + 1` overflows when index == UINT32_MAX,
// skipping the resize that would otherwise make room for `rid_map[index]`.
#include "core/templates/local_vector.h"
#include "core/templates/rid.h"

#include <cstdio>

static constexpr uint32_t INVALID_INDEX = 0xFFFFFFFF;

// Mirrors TextureStreaming::feedback_buffer_material_index() (texture_streaming.cpp:457-472)
// body exactly, operating on a LocalVector<RID> standing in for MaterialFeedbackBuffer::rid_map.
// The real function gets `index` from material_info_owner.get_index(p_material), which
// returns INVALID_INDEX for any RID not currently registered (never allocated, or already
// freed) - reproduced here by just passing that sentinel value directly, since the bug is in
// this function's own arithmetic/bounds handling, not in how the sentinel gets produced.
uint32_t mini_feedback_buffer_material_index(uint32_t index, RID p_material, LocalVector<RID> &rid_map) {
	if (rid_map.size() <= (index + 1)) {
		rid_map.resize(index + 1);
	}
	rid_map[index] = p_material;
	return index;
}

int main() {
	printf("Reproducing out-of-bounds write in feedback_buffer_material_index()\n");
	printf("(modules/texture_streaming/texture_streaming.cpp:457-472) via the\n");
	printf("INVALID_INDEX (0xFFFFFFFF) sentinel get_index() returns for an\n");
	printf("unregistered/already-freed material RID.\n");
	fflush(stdout);

	LocalVector<RID> rid_map;
	rid_map.resize(4); // small, realistic starting size

	printf("Calling mini_feedback_buffer_material_index() with index == INVALID_INDEX...\n");
	fflush(stdout);
	uint32_t idx = mini_feedback_buffer_material_index(INVALID_INDEX, RID(), rid_map);
	printf("Returned index: %u (if we get here, no crash - ASan should have caught the OOB write)\n", idx);
	return 0;
}
