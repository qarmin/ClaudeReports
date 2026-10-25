@tool
extends EditorPlugin

var frame := 0
var step := 0
var done := false

func _enter_tree():
	set_process(true)

func _count(node: Node, cls: String) -> int:
	var n := 0
	if node.get_class() == cls:
		n += 1
	for c in node.get_children(true):
		n += _count(c, cls)
	return n

func _counts(tag: String) -> void:
	print("VERIFY034 ", tag, " CreateDialog=", _count(get_tree().root, "CreateDialog"))

func _find_visible(node: Node, cls: String) -> Node:
	if node.get_class() == cls and node.visible:
		return node
	for c in node.get_children(true):
		var f = _find_visible(c, cls)
		if f: return f
	return null

func _process(_d):
	frame += 1
	if done or frame % 15 != 0:
		return
	if step == 0:
		_counts("startup")
		# SceneTreeDock's "Add Child Node" button is the real entry point for TOOL_NEW.
		var st := EditorInterface.get_selection()
		print("VERIFY034 triggering scene tree dock Add Child Node shortcut")
		var dock := _find_dock(EditorInterface.get_base_control(), "SceneTreeDock")
		if dock == null:
			print("VERIFY034 FAIL: no SceneTreeDock")
		else:
			var btn := _find_button(dock)
			if btn == null:
				print("VERIFY034 FAIL: no add button in SceneTreeDock")
			else:
				btn.pressed.emit()
		step = 1
		return
	if step == 1:
		var dlg = _find_visible(get_tree().root, "CreateDialog")
		print("VERIFY034 visible CreateDialog: ", "no" if dlg == null else "yes title='" + dlg.title + "'")
		_counts("after_add_child_node")
		done = true
		get_tree().quit()

func _find_dock(node: Node, cls: String) -> Node:
	if node.get_class() == cls:
		return node
	for c in node.get_children(true):
		var f = _find_dock(c, cls)
		if f: return f
	return null

func _find_button(node: Node) -> Button:
	if node is Button and node.tooltip_text == "Add/Create a New Node.":
		return node
	for c in node.get_children(true):
		var f = _find_button(c)
		if f: return f
	return null
