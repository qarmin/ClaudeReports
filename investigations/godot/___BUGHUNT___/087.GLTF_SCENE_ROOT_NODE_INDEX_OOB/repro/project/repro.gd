extends SceneTree

func _init():
	# The default scene lists node 0 as a root, but the file has no "nodes" array at all.
	var gltf := '{"asset":{"version":"2.0"},"scene":0,"scenes":[{"nodes":[0]}]}'
	var doc := GLTFDocument.new()
	var state := GLTFState.new()
	var err := doc.append_from_buffer(gltf.to_utf8_buffer(), "", state)
	print("append_from_buffer returned: ", error_string(err))
	if err == OK:
		var scene := doc.generate_scene(state)
		print("generate_scene returned: ", scene)
		if scene:
			scene.free()
	quit()
