extends SceneTree

func _init():
	var root := Node.new()
	root.name = "Root"
	var child := Node.new()
	child.name = "Child"
	root.add_child(child)
	child.owner = root
	var packed := PackedScene.new()
	packed.pack(root)

	# Each node record starts with: parent, owner, type, name, instance, index, property count, group count.
	# Corrupt the child's owner field (record 2 starts at int 8) so it points far outside node_paths.
	var bundled: Dictionary = packed.get("_bundled")
	var nodes: PackedInt32Array = bundled["nodes"]
	print("node records: ", nodes)
	nodes[9] = (1 << 30) | 0xff0000 # owner id with the "is path" flag and an index far beyond node_paths (which is empty)
	bundled["nodes"] = nodes
	packed.set("_bundled", bundled)

	var path := "/tmp/bughunt_081.scn"
	ResourceSaver.save(packed, path)
	var loaded: PackedScene = ResourceLoader.load(path, "", ResourceLoader.CACHE_MODE_IGNORE)
	print("loaded: ", loaded, " can_instantiate: ", loaded.can_instantiate())
	var inst := loaded.instantiate()
	print("instantiated: ", inst)
	quit()
