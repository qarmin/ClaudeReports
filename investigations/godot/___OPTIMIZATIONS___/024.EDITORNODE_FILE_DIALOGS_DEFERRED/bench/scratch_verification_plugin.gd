@tool
extends EditorPlugin

var frame := 0
var step := 0
var done := false

func _enter_tree():
	set_process(true)

func _find_popup(node: Node, name: String) -> PopupMenu:
	if node is PopupMenu and node.name == name:
		return node
	for c in node.get_children():
		var f = _find_popup(c, name)
		if f: return f
	return null

func _count_file_dialogs(node: Node) -> int:
	var n := 0
	if node.get_class() == "EditorFileDialog":
		n += 1
	for c in node.get_children(true):
		n += _count_file_dialogs(c)
	return n

func _visible_file_dialog(node: Node) -> Node:
	if node.get_class() == "EditorFileDialog" and node.visible:
		return node
	for c in node.get_children(true):
		var f = _visible_file_dialog(c)
		if f: return f
	return null

func _process(_d):
	frame += 1
	if done or frame % 15 != 0:
		return
	var base = EditorInterface.get_base_control()
	if step == 0:
		print("VERIFY024 EditorFileDialogs in tree at startup: ", _count_file_dialogs(get_tree().root))
		var pm := _find_popup(base, "Project")
		var idx := -1
		for i in range(pm.item_count):
			if pm.get_item_text(i).begins_with("Pack Project as ZIP"):
				idx = i
		if idx == -1:
			print("VERIFY024 ERROR: Pack Project as ZIP item not found"); get_tree().quit(); return
		print("VERIFY024 triggering Pack Project as ZIP (idx=", idx, ")")
		pm.id_pressed.emit(pm.get_item_id(idx))
		step = 1
		return
	if step == 1:
		var dlg = _visible_file_dialog(get_tree().root)
		if dlg:
			print("VERIFY024 OK: a visible EditorFileDialog appeared, title='", dlg.title, "'")
			var img = dlg.get_texture().get_image()
			img.save_png("res://screenshot_pack_zip_dialog.png")
			print("VERIFY024 screenshot saved")
		else:
			print("VERIFY024 FAIL: no visible EditorFileDialog after Pack Project as ZIP")
		print("VERIFY024 EditorFileDialogs in tree after: ", _count_file_dialogs(get_tree().root))
		done = true
		get_tree().quit()
