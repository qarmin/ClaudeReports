extends SceneTree
func _initialize() -> void:
	var n := 200000
	var before := OS.get_static_memory_usage()
	var keep := []
	keep.resize(n)
	for i in range(n):
		keep[i] = Object.new()
	var after := OS.get_static_memory_usage()
	print("static_memory_delta_bytes=%d" % (after - before))
	print("bytes_per_object=%f" % ((after - before) / float(n)))
	quit()
