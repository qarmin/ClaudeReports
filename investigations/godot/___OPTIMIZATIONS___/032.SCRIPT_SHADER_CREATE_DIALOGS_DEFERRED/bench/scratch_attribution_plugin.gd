@tool
extends EditorPlugin

# Scratch tool that produced bench/attribution_64_file_dialogs.txt: walks the whole editor scene
# tree (including internal children) and prints each EditorFileDialog/FileDialog together with its
# chain of ancestor classes, which is what attributes the 64 instances to their owning editors.

var frame := 0
var done := false

func _enter_tree():
	set_process(true)

func _chain(node: Node) -> String:
	var parts: PackedStringArray = []
	var n := node.get_parent()
	var depth := 0
	while n != null and depth < 8:
		parts.append(n.get_class())
		n = n.get_parent()
		depth += 1
	return "/".join(parts)

func _walk(node: Node, out: Array) -> void:
	if node.get_class() == "EditorFileDialog" or node.get_class() == "FileDialog":
		out.append(node.get_class() + " <- " + _chain(node))
	for c in node.get_children(true):
		_walk(c, out)

func _process(_d):
	frame += 1
	if done or frame < 20:
		return
	done = true
	var out: Array = []
	_walk(get_tree().root, out)
	print("ATTR total=", out.size())
	for s in out:
		print("ATTR ", s)
	get_tree().quit()
