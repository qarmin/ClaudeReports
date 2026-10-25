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
	var root := get_tree().root
	print("VERIFY032 ", tag,
		" EditorFileDialog=", _count(root, "EditorFileDialog"),
		" ScriptCreateDialog=", _count(root, "ScriptCreateDialog"),
		" ShaderCreateDialog=", _count(root, "ShaderCreateDialog"),
		" CreateDialog=", _count(root, "CreateDialog"))

func _find_visible(node: Node, cls: String) -> Node:
	if node.get_class() == cls and node.visible:
		return node
	for c in node.get_children(true):
		var f = _find_visible(c, cls)
		if f: return f
	return null

func _find_popup_with_item(node: Node, prefix: String) -> PopupMenu:
	if node is PopupMenu:
		for i in range(node.item_count):
			if node.get_item_text(i).begins_with(prefix):
				return node
	for c in node.get_children(true):
		var f = _find_popup_with_item(c, prefix)
		if f: return f
	return null

func _process(_d):
	frame += 1
	if done or frame % 15 != 0:
		return
	if step == 0:
		_counts("startup")
		print("VERIFY032 calling ScriptEditor.open_script_create_dialog()")
		EditorInterface.get_script_editor().open_script_create_dialog("Node", "res://new_script.gd")
		step = 1
		return
	if step == 1:
		var dlg = _find_visible(get_tree().root, "ScriptCreateDialog")
		if dlg:
			print("VERIFY032 OK visible ScriptCreateDialog title='", dlg.title, "'")
			var img = dlg.get_texture().get_image()
			img.save_png("res://screenshot_script_create_dialog.png")
		else:
			print("VERIFY032 FAIL no visible ScriptCreateDialog")
		_counts("after_open_script_create_dialog")
		if dlg:
			dlg.hide()
		step = 2
		return
	if step == 2:
		# EditorPlugin.get_script_create_dialog() is the GDScript-exposed SceneTreeDock accessor.
		var d = get_script_create_dialog()
		print("VERIFY032 EditorPlugin.get_script_create_dialog() -> ", "null" if d == null else d.get_class())
		_counts("after_editorplugin_accessor")
		step = 3
		return
	if step == 3:
		var pm := _find_popup_with_item(get_tree().root, "New Shader")
		if pm == null:
			print("VERIFY032 FAIL: no popup with a 'New Shader' item")
		else:
			var idx := -1
			for i in range(pm.item_count):
				if pm.get_item_text(i).begins_with("New Shader"):
					idx = i
			print("VERIFY032 emitting 'New Shader' item id from popup (idx=", idx, ")")
			pm.id_pressed.emit(pm.get_item_id(idx))
		step = 4
		return
	if step == 4:
		var dlg = _find_visible(get_tree().root, "ShaderCreateDialog")
		print("VERIFY032 visible ShaderCreateDialog: ", "no" if dlg == null else "yes title='" + dlg.title + "'")
		_counts("after_new_shader")
		done = true
		get_tree().quit()
