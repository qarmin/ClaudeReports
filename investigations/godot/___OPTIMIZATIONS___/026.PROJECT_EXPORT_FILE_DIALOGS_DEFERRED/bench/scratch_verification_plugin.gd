@tool
extends EditorPlugin

var frame := 0
var step := 0
var done := false

func _enter_tree(): set_process(true)

func _count_fd(n: Node) -> int:
	var c := 1 if n.get_class() == "EditorFileDialog" else 0
	for ch in n.get_children(true): c += _count_fd(ch)
	return c

func _find_button_under_class(n: Node, cls: String, prefix: String, inside := false) -> Button:
	var ins = inside or n.get_class() == cls
	if ins and n is Button and n.text.begins_with(prefix): return n
	for ch in n.get_children(true):
		var f = _find_button_under_class(ch, cls, prefix, ins)
		if f: return f
	return null

func _visible_fd(n: Node) -> Node:
	if n.get_class() == "EditorFileDialog" and n.visible: return n
	for ch in n.get_children(true):
		var f = _visible_fd(ch)
		if f: return f
	return null

func _process(_d):
	frame += 1
	if done or frame % 20 != 0: return
	if step == 0:
		print("V26 EditorFileDialogs at startup: ", _count_fd(get_tree().root))
		var btn := _find_button_under_class(get_tree().root, "ProjectExportDialog", "Export PCK")
		if btn == null:
			print("V26 note: 'Export PCK/ZIP' button not found under ProjectExportDialog")
			done = true; get_tree().quit(); return
		print("V26 pressing '", btn.text, "'")
		btn.pressed.emit()
		step = 1
		return
	if step == 1:
		var d = _visible_fd(get_tree().root)
		if d: print("V26 OK: visible EditorFileDialog, title='", d.title, "' filters=", d.get_filters().size())
		else: print("V26 FAIL: no visible EditorFileDialog")
		print("V26 EditorFileDialogs after: ", _count_fd(get_tree().root))
		done = true; get_tree().quit()
