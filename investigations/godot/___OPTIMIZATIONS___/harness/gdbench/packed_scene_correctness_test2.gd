extends SceneTree

func _initialize() -> void:
	var fail := false
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

	# GEN_EDIT_STATE_INSTANCE path (editor "instance child scene" - removed the blanket
	# `value.duplicate(true)` this candidate's patch drops).
	var inst_a := packed.instantiate(PackedScene.GEN_EDIT_STATE_INSTANCE)
	var inst_b := packed.instantiate(PackedScene.GEN_EDIT_STATE_INSTANCE)
	inst_a.set("dict_prop_0", {"mutated": true})
	var b_val = inst_b.get("dict_prop_0")
	if b_val != {"k": 0}:
		fail = true
		print("GEN_EDIT_STATE_INSTANCE ALIASING BUG: inst_b.dict_prop_0 = %s (should be {k:0}, unaffected by inst_a's mutation)" % b_val)
	inst_a.free()
	inst_b.free()

	print("ALL_TESTS_" + ("FAIL" if fail else "PASS"))
	quit()
