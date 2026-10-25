@tool
extends EditorPlugin

func _enter_tree() -> void:
	call_deferred("_run")

func _count(n: Node, cls: String) -> int:
	var c := 1 if n.is_class(cls) else 0
	for ch in n.get_children(true):
		c += _count(ch, cls)
	return c

func _find_button(n: Node, needle: String) -> Button:
	if n is Button:
		var b := n as Button
		var hay := (b.tooltip_text + " " + b.get_accessibility_name() + " " + b.text).to_lower()
		if needle.to_lower() in hay:
			return b
	for ch in n.get_children(true):
		var r := _find_button(ch, needle)
		if r != null:
			return r
	return null

func _windows(n: Node, out: Array) -> void:
	if n is Window:
		out.append(n)
	for ch in n.get_children(true):
		_windows(ch, out)

func _run() -> void:
	var root := get_tree().root
	print("VERIFY046 startup EditorFileDialog=", _count(root, "EditorFileDialog"))

	# Drive the real "Select Folder" button of the Find-in-Files panel (deferred by this patch).
	var b := _find_button(EditorInterface.get_base_control(), "select folder")
	if b == null:
		print("VERIFY046 ERROR: 'Select Folder' button not found")
	else:
		print("VERIFY046 pressing button tooltip='", b.tooltip_text, "' acc='", b.get_accessibility_name(), "'")
		b.pressed.emit()
		await get_tree().process_frame
		await get_tree().process_frame
		print("VERIFY046 after_select_folder EditorFileDialog=", _count(root, "EditorFileDialog"))
		var ws := []
		_windows(root, ws)
		for w in ws:
			if w.visible and w.get_class() == "EditorFileDialog":
				print("VERIFY046 visible_dialog title='", w.title, "' file_mode=", w.file_mode)
	print("VERIFY046 DONE")
	get_tree().quit()
