@tool
extends EditorPlugin
var f := 0
var done := false
func _enter_tree(): set_process(true)
func _count(n: Node) -> int:
	var c := 1 if n.get_class() == "EditorFileDialog" else 0
	for ch in n.get_children(true): c += _count(ch)
	return c
func _find_btn(n: Node, cls: String, prefix: String, inside := false) -> Button:
	var ins = inside or n.get_class() == cls
	if ins and n is Button and n.text.begins_with(prefix): return n
	for ch in n.get_children(true):
		var r = _find_btn(ch, cls, prefix, ins)
		if r: return r
	return null
func _vis(n: Node) -> Node:
	if n.get_class() == "EditorFileDialog" and n.visible: return n
	for ch in n.get_children(true):
		var r = _vis(ch)
		if r: return r
	return null
func _process(_d):
	f += 1
	if done or f % 25 != 0: return
	print("V31 EditorFileDialogs at startup: ", _count(get_tree().root))
	var b := _find_btn(get_tree().root, "EditorAutoloadSettings", "Select Script")
	if b:
		print("V31 pressing '", b.text, "'")
		b.pressed.emit()
		await get_tree().process_frame
		await get_tree().process_frame
		var d = _vis(get_tree().root)
		print("V31 visible dialog after press: ", "yes title='"+d.title+"'" if d else "none")
		print("V31 EditorFileDialogs after: ", _count(get_tree().root))
	else:
		print("V31 note: EditorAutoloadSettings browse button not found")
	done = true
	get_tree().quit()
