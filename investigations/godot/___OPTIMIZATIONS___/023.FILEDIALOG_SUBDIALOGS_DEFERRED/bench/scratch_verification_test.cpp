/**************************************************************************/
/*  test_file_dialog_scratch.cpp                                          */
/**************************************************************************/
/*                         This file is part of:                          */
/*                             GODOT ENGINE                               */
/*                        https://godotengine.org                         */
/**************************************************************************/

// SCRATCH VERIFICATION for ___OPTIMIZATIONS___/023 - not a real correctness test, temporary only.
// Verifies that FileDialog's lazily-constructed sub-dialogs (a) do not exist before their flow is
// used, and (b) are created and shown when their flow actually runs.

#include "tests/test_macros.h"

TEST_FORCE_LINK(test_file_dialog_scratch)

#include "core/io/dir_access.h"
#include "core/io/file_access.h"
#include "core/os/os.h"
#include "scene/gui/dialogs.h"
#include "scene/gui/file_dialog.h"
#include "scene/main/scene_tree.h"
#include "scene/main/window.h"

namespace TestFileDialogScratch {

// Counts direct children (including internal ones) whose class matches, and how many are visible.
static void count_subdialogs(Node *p_node, int &r_total, int &r_visible) {
	r_total = 0;
	r_visible = 0;
	for (int i = 0; i < p_node->get_child_count(true); i++) {
		Node *c = p_node->get_child(i, true);
		AcceptDialog *d = Object::cast_to<AcceptDialog>(c);
		if (d) {
			r_total++;
			if (d->is_visible()) {
				r_visible++;
			}
		}
	}
}

TEST_CASE("[SceneTree][FileDialog] SCRATCH deferred sub-dialogs are absent until their flow runs") {
	FileDialog *fd = memnew(FileDialog);
	SceneTree::get_singleton()->get_root()->add_child(fd);

	int total = 0, visible = 0;
	count_subdialogs(fd, total, visible);
	MESSAGE("SCRATCH_FD fresh instance: AcceptDialog-derived children=", total, " visible=", visible);
	// With the patch, none of the 5 sub-dialogs should exist yet.
	CHECK(total == 0);

	memdelete(fd);
}

TEST_CASE("[SceneTree][FileDialog] SCRATCH exterr flow creates and shows its dialog") {
	FileDialog *fd = memnew(FileDialog);
	SceneTree::get_singleton()->get_root()->add_child(fd);
	fd->set_file_mode(FileDialog::FILE_MODE_SAVE_FILE);
	fd->set_access(FileDialog::ACCESS_FILESYSTEM);

	int before_total = 0, before_visible = 0;
	count_subdialogs(fd, before_total, before_visible);

	// Empty filename in save mode -> the "Invalid extension, or empty filename." error dialog.
	fd->set_current_file("");
	fd->emit_signal(SceneStringName(confirmed));

	int after_total = 0, after_visible = 0;
	count_subdialogs(fd, after_total, after_visible);
	MESSAGE("SCRATCH_FD exterr flow: before total=", before_total, " -> after total=", after_total, " visible=", after_visible);
	CHECK(after_total > before_total); // the dialog was lazily created
	CHECK(after_visible >= 1); // and actually shown

	memdelete(fd);
}

TEST_CASE("[SceneTree][FileDialog] SCRATCH confirm_save (overwrite) flow creates and shows its dialog") {
	// Create a real file so the overwrite-warning branch is taken.
	const String dir = OS::get_singleton()->get_cache_path().path_join("scratch_023_fd");
	Ref<DirAccess> da = DirAccess::create(DirAccess::ACCESS_FILESYSTEM);
	da->make_dir_recursive(dir);
	const String existing = dir.path_join("already_here.txt");
	{
		Ref<FileAccess> f = FileAccess::open(existing, FileAccess::WRITE);
		REQUIRE(f.is_valid());
		f->store_string("x");
	}

	FileDialog *fd = memnew(FileDialog);
	SceneTree::get_singleton()->get_root()->add_child(fd);
	fd->set_file_mode(FileDialog::FILE_MODE_SAVE_FILE);
	fd->set_access(FileDialog::ACCESS_FILESYSTEM);
	fd->set_current_dir(dir);

	int before_total = 0, before_visible = 0;
	count_subdialogs(fd, before_total, before_visible);

	fd->set_current_file("already_here.txt");
	fd->emit_signal(SceneStringName(confirmed));

	int after_total = 0, after_visible = 0;
	count_subdialogs(fd, after_total, after_visible);
	MESSAGE("SCRATCH_FD confirm_save flow: before total=", before_total, " -> after total=", after_total, " visible=", after_visible);
	CHECK(after_total > before_total);
	CHECK(after_visible >= 1);

	memdelete(fd);
	da->remove(existing);
	da->remove(dir);
}

} // namespace TestFileDialogScratch
