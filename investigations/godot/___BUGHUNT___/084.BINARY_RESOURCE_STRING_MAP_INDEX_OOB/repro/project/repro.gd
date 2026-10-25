extends SceneTree

func _init():
	var gradient := Gradient.new()
	gradient.set_meta("m", Curve.new()) # an internal sub-resource with properties
	var path := "/tmp/bughunt_084.res"
	ResourceSaver.save(gradient, path)
	var bytes := FileAccess.get_file_as_bytes(path)
	# The sub-resource record starts with its class name (u32 length incl. NUL + bytes), then the property count (u32),
	# then for each property: name (u32 index into the file's string table) followed by the value.
	var needle := PackedByteArray([6, 0, 0, 0]) + "Curve".to_utf8_buffer() + PackedByteArray([0])
	var pos := -1
	for i in range(bytes.size() - needle.size()):
		if bytes.slice(i, i + needle.size()) == needle:
			pos = i
	assert(pos >= 0)
	var name_index_pos := pos + needle.size() + 4
	print("first property name index of the Curve record: ", bytes.decode_u32(name_index_pos), " (string table has few entries)")
	bytes.encode_u32(name_index_pos, 0x7fffffff)
	var f := FileAccess.open(path, FileAccess.WRITE)
	f.store_buffer(bytes)
	f.close()
	print("loading corrupted .res")
	var res = ResourceLoader.load(path, "", ResourceLoader.CACHE_MODE_IGNORE)
	print("loaded: ", res)
	quit()
