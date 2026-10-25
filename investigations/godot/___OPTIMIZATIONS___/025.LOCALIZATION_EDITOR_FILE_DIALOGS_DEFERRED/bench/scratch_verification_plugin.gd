@tool
extends EditorPlugin

var frame := 0
var step := 0
var done := false

func _enter_tree():
	set_process(true)

func _count_fd(node: Node) -> int:
	var n := 0
	if node.get_class() == "EditorFileDialog":
		n += 1
	for c in node.get_children(true):
		n += _count_fd(c)
	return n

# Find a Button whose text starts with the given prefix, under a node whose class matches.
func _find_button_under_class(node: Node, cls: String, prefix: String, inside := false) -> Button:
	var now_inside = inside or node.get_class() == cls
	if now_inside and node is Button and node.text.begins_with(prefix):
		return node
	for c in node.get_children(true):
		var f = _find_button_under_class(c, cls, prefix, now_inside)
		if f: return f
	return null

func _visible_fd(node: Node) -> Node:
	if node.get_class() == "EditorFileDialog" and node.visible:
		return node
	for c in node.get_children(true):
		var f = _visible_fd(c)
		if f: return f
	return null

func _process(_d):
	frame += 1
	if done or frame % 20 != 0:
		return
	if step == 0:
		print("VERIFY025 EditorFileDialogs in tree at startup: ", _count_fd(get_tree().root))
		# The Localization tab's "Add..." button lives under LocalizationEditor.
		var btn := _find_button_under_class(get_tree().root, "LocalizationEditor", "Add")
		if btn == null:
			print("VERIFY025 note: could not locate LocalizationEditor 'Add' button (LocalizationEditor may not be built yet)")
			done = true
			get_tree().quit()
			return
		print("VERIFY025 pressing LocalizationEditor button: '", btn.text, "'")
		btn.pressed.emit()
		step = 1
		return
	if step == 1:
		var dlg = _visible_fd(get_tree().root)
		if dlg:
			print("VERIFY025 OK: visible EditorFileDialog appeared, title='", dlg.title, "' filters=", dlg.get_filters().size())
		else:
			print("VERIFY025 FAIL: no visible EditorFileDialog after pressing Add")
		print("VERIFY025 EditorFileDialogs in tree after: ", _count_fd(get_tree().root))
		done = true
		get_tree().quit()
