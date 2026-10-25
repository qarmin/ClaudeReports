// SCRATCH verification for ___OPTIMIZATIONS___/044 - temporary, reverted before patch.diff capture.
#include "scene/gui/tree.h"

#include "scene/gui/line_edit.h"
#include "scene/gui/text_edit.h"
#include "scene/main/scene_tree.h"
#include "scene/main/window.h"
#include "tests/test_macros.h"

TEST_FORCE_LINK(test_tree_scratch_044)

namespace TestTreeScratch044 {

static int count_class(Node *p_node, const String &p_class) {
	int n = p_node->is_class(p_class) ? 1 : 0;
	for (int i = 0; i < p_node->get_child_count(true); i++) {
		n += count_class(p_node->get_child(i, true), p_class);
	}
	return n;
}

TEST_CASE("[SceneTree][Tree][Scratch044] inline multiline cell editor") {
	Tree *tree = memnew(Tree);
	SceneTree::get_singleton()->get_root()->add_child(tree);
	tree->set_columns(1);

	TreeItem *root = tree->create_item();
	root->set_cell_mode(0, TreeItem::CELL_MODE_STRING);
	root->set_text(0, "hello\nworld");
	root->set_editable(0, true);
	root->set_edit_multiline(0, true);

	MESSAGE("SCRATCH044 textedit_before_edit=", count_class(tree, "TextEdit"));
	MESSAGE("SCRATCH044 lineedit_before_edit=", count_class(tree, "LineEdit"));

	tree->set_selected(root, 0);
	const bool edited = tree->edit_selected(true);
	MESSAGE("SCRATCH044 edit_selected_returned=", edited);
	MESSAGE("SCRATCH044 is_editing=", tree->is_editing());
	MESSAGE("SCRATCH044 textedit_after_edit=", count_class(tree, "TextEdit"));

	// The multiline editor must exist, be visible, and carry the cell's text.
	TextEdit *te = nullptr;
	List<Node *> stack;
	stack.push_back(tree);
	while (!stack.is_empty()) {
		Node *n = stack.front()->get();
		stack.pop_front();
		if (TextEdit *cand = Object::cast_to<TextEdit>(n)) {
			te = cand;
			break;
		}
		for (int i = 0; i < n->get_child_count(true); i++) {
			stack.push_back(n->get_child(i, true));
		}
	}
	CHECK(edited);
	REQUIRE(te != nullptr);
	MESSAGE("SCRATCH044 textedit_visible=", te->is_visible(), " text='", te->get_text(), "'");
	CHECK(te->get_text() == "hello\nworld");
	CHECK(te->is_visible());

	memdelete(tree);
}

} // namespace TestTreeScratch044
