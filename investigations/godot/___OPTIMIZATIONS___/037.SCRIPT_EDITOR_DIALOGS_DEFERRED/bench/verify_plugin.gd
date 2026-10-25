@tool
extends EditorPlugin

func _enter_tree() -> void:
	call_deferred("_run")

func _count(cls: String) -> int:
	return _count_rec(get_tree().root, cls)

func _count_rec(n: Node, cls: String) -> int:
	var c := 0
	if n.is_class(cls):
		c += 1
	for ch in n.get_children(true):
		c += _count_rec(ch, cls)
	return c

func _find_menu_item(n: Node, needle: String) -> Array:
	# Returns [PopupMenu, id] for the first menu item whose text contains needle.
	if n is PopupMenu:
		var pm := n as PopupMenu
		for i in pm.item_count:
			if needle.to_lower() in pm.get_item_text(i).to_lower():
				return [pm, pm.get_item_id(i)]
	for ch in n.get_children(true):
		var r := _find_menu_item(ch, needle)
		if not r.is_empty():
			return r
	return []

func _run() -> void:
	print("VERIFY037 startup EditorFileDialog=", _count("EditorFileDialog"),
		" EditorHelpSearch=", _count("EditorHelpSearch"),
		" ConfirmationDialog=", _count("ConfirmationDialog"))

	# Force the deferred file dialog through the real Script editor File menu item.
	var hit := _find_menu_item(EditorInterface.get_base_control(), "new text file")
	if hit.is_empty():
		print("VERIFY037 ERROR: 'New Text File' menu item not found")
	else:
		var pm: PopupMenu = hit[0]
		print("VERIFY037 firing menu='", pm.name, "' id=", hit[1])
		pm.id_pressed.emit(hit[1])
		await get_tree().process_frame
		await get_tree().process_frame
		print("VERIFY037 after_new_text_file EditorFileDialog=", _count("EditorFileDialog"))
		# Report the visible dialog so we know the right window actually appeared.
		for w in _windows(get_tree().root):
			if w.visible and w.get_class() == "EditorFileDialog":
				print("VERIFY037 visible_file_dialog title='", w.title, "'")

	# Force the deferred help search dialog via the real Search Help menu item.
	var hit2 := _find_menu_item(EditorInterface.get_base_control(), "search help")
	if hit2.is_empty():
		print("VERIFY037 ERROR: 'Search Help' menu item not found")
	else:
		var pm2: PopupMenu = hit2[0]
		print("VERIFY037 firing menu='", pm2.name, "' id=", hit2[1])
		pm2.id_pressed.emit(hit2[1])
		await get_tree().process_frame
		await get_tree().process_frame
		print("VERIFY037 after_search_help EditorHelpSearch=", _count("EditorHelpSearch"))
		for w in _windows(get_tree().root):
			if w.visible and w.get_class() == "EditorHelpSearch":
				print("VERIFY037 visible_help_search title='", w.title, "'")

	print("VERIFY037 DONE")
	get_tree().quit()

func _windows(n: Node) -> Array:
	var out := []
	if n is Window:
		out.append(n)
	for ch in n.get_children(true):
		out += _windows(ch)
	return out
