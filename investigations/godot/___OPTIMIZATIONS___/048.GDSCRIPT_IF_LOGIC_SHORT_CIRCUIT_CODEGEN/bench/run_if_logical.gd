extends SceneTree
func _initialize():
	var script = load("res://modules/gdscript/tests/scripts/runtime/features/if_logical_op.gd")
	var inst = script.new()
	inst.test()
	quit()
