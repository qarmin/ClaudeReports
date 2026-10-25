extends SceneTree
func _initialize():
	var script_obj := GDScript.new()
	var props := script_obj.get_property_list()
	for p in props:
		if p["name"] == "script":
			print("Script.script.usage=%d" % p["usage"])

	var node_obj := Node.new()
	var props2 := node_obj.get_property_list()
	for p in props2:
		if p["name"] == "script":
			print("Node.script.usage=%d" % p["usage"])
	node_obj.free()

	var res_obj := Resource.new()
	var props3 := res_obj.get_property_list()
	for p in props3:
		if p["name"] == "script":
			print("Resource.script.usage=%d" % p["usage"])

	print("DONE")
	quit()
