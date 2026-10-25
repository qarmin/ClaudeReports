@tool
extends EditorPlugin

var frame := 0

func _enter_tree():
	set_process(true)

func _count(node: Node, cls: String) -> int:
	var n := 0
	if node.get_class() == cls:
		n += 1
	for c in node.get_children(true):
		n += _count(c, cls)
	return n

func _process(_d):
	frame += 1
	if frame != 20:
		return
	var root := get_tree().root
	print("VERIFY036 startup EditorFileDialog=", _count(root, "EditorFileDialog"),
		" DependencyEditor=", _count(root, "DependencyEditor"),
		" SceneImportSettingsDialog=", _count(root, "SceneImportSettingsDialog"))
	get_tree().quit()
