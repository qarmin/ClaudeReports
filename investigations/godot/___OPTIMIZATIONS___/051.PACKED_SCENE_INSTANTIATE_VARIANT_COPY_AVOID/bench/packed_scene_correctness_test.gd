extends SceneTree

func _initialize() -> void:
	var fail := false
	var NodeScript := load("res://prop_list_node.gd")
	var root: Node = NodeScript.new()
	root.name = "Root"
	for i in range(NodeScript.PROP_COUNT):
		root.set("dict_prop_%d" % i, {"k": i, "extra": "value_%d" % i})

	var packed := PackedScene.new()
	var err := packed.pack(root)
	if err != OK:
		print("PACK_FAILED err=%d" % err)
		quit()
		return
	root.free()

	var inst := packed.instantiate()
	for i in range(NodeScript.PROP_COUNT):
		var expected := {"k": i, "extra": "value_%d" % i}
		var actual = inst.get("dict_prop_%d" % i)
		if actual != expected:
			fail = true
			print("MISMATCH at dict_prop_%d: expected=%s actual=%s" % [i, expected, actual])
	inst.free()

	# Second instance check independence (no shared-buffer aliasing across instances).
	var inst2 := packed.instantiate()
	inst2.set("dict_prop_0", {"modified": true})
	var inst3 := packed.instantiate()
	var val3 = inst3.get("dict_prop_0")
	if val3 != {"k": 0, "extra": "value_0"}:
		fail = true
		print("ALIASING BUG: inst3.dict_prop_0 = %s (should be unaffected by inst2's mutation)" % val3)
	inst2.free()
	inst3.free()

	print("ALL_TESTS_" + ("FAIL" if fail else "PASS"))
	quit()
