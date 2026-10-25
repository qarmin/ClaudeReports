extends SceneTree

# Measures PackedScene::instantiate() cost for a node whose attached script overrides
# _get_property_list() with many Dictionary properties - PR godotengine/godot#120100's own
# "Scenario B" shape (candidate 051's discovery pass).

const N := 5000

func _initialize() -> void:
	var NodeScript := load("res://prop_list_node.gd")
	var root: Node = NodeScript.new()
	root.name = "Root"
	for i in range(NodeScript.PROP_COUNT):
		root.set("dict_prop_%d" % i, {"k": i})

	var packed := PackedScene.new()
	var err := packed.pack(root)
	if err != OK:
		print("PACK_FAILED err=%d" % err)
		quit()
		return
	root.free()

	var warmup := int(N * 0.1)
	for i in range(warmup):
		var inst := packed.instantiate()
		inst.free()

	var t0 := Time.get_ticks_usec()
	for i in range(N):
		var inst := packed.instantiate()
		inst.free()
	var elapsed := Time.get_ticks_usec() - t0
	print("packed_scene_instantiate_us_per_iter=%f" % (elapsed / float(N)))
	quit()
