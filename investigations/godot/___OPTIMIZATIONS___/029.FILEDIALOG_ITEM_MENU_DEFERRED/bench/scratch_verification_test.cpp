/**************************************************************************/
/*  test_file_dialog_itemmenu_scratch.cpp                                 */
/**************************************************************************/
/*                         This file is part of:                          */
/*                             GODOT ENGINE                               */
/*                        https://godotengine.org                         */
/**************************************************************************/

// SCRATCH VERIFICATION for ___OPTIMIZATIONS___/029 - not a real correctness test, temporary only.

#include "tests/test_macros.h"

TEST_FORCE_LINK(test_file_dialog_itemmenu_scratch)

#include "scene/gui/file_dialog.h"
#include "scene/gui/item_list.h"
#include "scene/gui/popup_menu.h"
#include "scene/main/scene_tree.h"
#include "scene/main/window.h"

namespace TestFileDialogItemMenuScratch {

static void count_popupmenus(Node *p_node, int &r_total, int &r_visible) {
	r_total = 0;
	r_visible = 0;
	for (int i = 0; i < p_node->get_child_count(true); i++) {
		PopupMenu *pm = Object::cast_to<PopupMenu>(p_node->get_child(i, true));
		if (pm) {
			r_total++;
			if (pm->is_visible()) {
				r_visible++;
			}
		}
	}
}

TEST_CASE("[SceneTree][FileDialog] SCRATCH item_menu is deferred until right-click") {
	FileDialog *fd = memnew(FileDialog);
	SceneTree::get_singleton()->get_root()->add_child(fd);

	int before_total = 0, before_visible = 0;
	count_popupmenus(fd, before_total, before_visible);
	MESSAGE("SCRATCH_IM direct PopupMenu children before right-click: ", before_total);

	// Emit the ItemList signal FileDialog wires to _empty_clicked, with a right-click.
	ItemList *fl = fd->get_file_item_list();
	REQUIRE(fl != nullptr);
	fl->emit_signal("empty_clicked", Vector2(10, 10), (int)MouseButton::RIGHT);

	int after_total = 0, after_visible = 0;
	count_popupmenus(fd, after_total, after_visible);
	MESSAGE("SCRATCH_IM direct PopupMenu children after right-click: ", after_total, " visible=", after_visible);
	CHECK(after_total >= 1); // the context menu exists after the right-click, in both builds
	CHECK(after_visible >= 1); // and it is shown

	memdelete(fd);
}

} // namespace TestFileDialogItemMenuScratch
