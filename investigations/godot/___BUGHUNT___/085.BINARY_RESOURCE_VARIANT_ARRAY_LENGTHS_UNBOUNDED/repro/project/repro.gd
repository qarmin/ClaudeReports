extends SceneTree

func _init():
	var gradient := Gradient.new()
	gradient.set_meta("a", [0x11111111, 0x22222222, 0x33333333])
	var path := "/tmp/bughunt_085.res"
	ResourceSaver.save(gradient, path)
	var bytes := FileAccess.get_file_as_bytes(path)
	# An Array is stored as <u32 element count> followed by that many Variants; an int Variant is <u32 type = 3><u32 value>.
	var needle := PackedByteArray([3, 0, 0, 0, 3, 0, 0, 0, 0x11, 0x11, 0x11, 0x11])
	var pos := -1
	for i in range(bytes.size() - needle.size()):
		if bytes.slice(i, i + needle.size()) == needle:
			pos = i
			break
	assert(pos >= 0)
	bytes.encode_u32(pos, 0x7fffffff)
	var f := FileAccess.open(path, FileAccess.WRITE)
	f.store_buffer(bytes)
	f.close()
	print("loading a ", bytes.size(), " byte .res whose Array claims 2147483647 elements")
	var res = ResourceLoader.load(path, "", ResourceLoader.CACHE_MODE_IGNORE)
	print("loaded: ", res)
	quit()
