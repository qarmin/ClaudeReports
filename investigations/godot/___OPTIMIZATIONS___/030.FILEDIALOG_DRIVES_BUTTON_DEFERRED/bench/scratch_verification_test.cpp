/**************************************************************************/
/*  test_file_dialog_drives_scratch.cpp                                   */
/**************************************************************************/
/*                         This file is part of:                          */
/*                             GODOT ENGINE                               */
/*                        https://godotengine.org                         */
/**************************************************************************/

// SCRATCH VERIFICATION for ___OPTIMIZATIONS___/030 - not a real correctness test, temporary only.

#include "tests/test_macros.h"

TEST_FORCE_LINK(test_file_dialog_drives_scratch)

#include "core/io/dir_access.h"
#include "core/os/os.h"
#include "scene/gui/file_dialog.h"
#include "scene/gui/menu_button.h"
#include "scene/main/scene_tree.h"
#include "scene/main/window.h"

namespace TestFileDialogDrivesScratch {

static int count_menubuttons(Node *p_node) {
	int n = Object::cast_to<MenuButton>(p_node) ? 1 : 0;
	for (int i = 0; i < p_node->get_child_count(true); i++) {
		n += count_menubuttons(p_node->get_child(i, true));
	}
	return n;
}

TEST_CASE("[SceneTree][FileDialog] SCRATCH drives button and platform drive count") {
	Ref<DirAccess> da = DirAccess::create(DirAccess::ACCESS_FILESYSTEM);
	REQUIRE(da.is_valid());
	MESSAGE("SCRATCH_DRV platform drive_count=", da->get_drive_count());

	FileDialog *fd = memnew(FileDialog);
	SceneTree::get_singleton()->get_root()->add_child(fd);
	MESSAGE("SCRATCH_DRV MenuButtons in a fresh FileDialog: ", count_menubuttons(fd));

	// Exercise the code path that decides whether the drives button is shown, and the dir change
	// path whose guard reads drives->is_visible().
	fd->set_access(FileDialog::ACCESS_FILESYSTEM);
	fd->set_current_dir(OS::get_singleton()->get_cache_path());
	MESSAGE("SCRATCH_DRV after set_access + set_current_dir: MenuButtons=", count_menubuttons(fd), " current_dir='", fd->get_current_dir(), "'");
	CHECK(!fd->get_current_dir().is_empty()); // the dir-change path still works

	memdelete(fd);
}

} // namespace TestFileDialogDrivesScratch
