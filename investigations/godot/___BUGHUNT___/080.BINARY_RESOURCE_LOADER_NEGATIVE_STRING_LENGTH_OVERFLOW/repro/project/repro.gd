extends SceneTree

func _init():
	var path := "/tmp/bughunt_080.res"
	ResourceSaver.save(Gradient.new(), path)
	var bytes := FileAccess.get_file_as_bytes(path)
	# The file stores the resource class name as <u32 length incl. NUL><bytes>. Corrupt that length to -1.
	var needle := PackedByteArray([9, 0, 0, 0]) + "Gradient".to_utf8_buffer() + PackedByteArray([0])
	var pos := -1
	for i in range(bytes.size() - needle.size()):
		if bytes.slice(i, i + needle.size()) == needle:
			pos = i
			break
	assert(pos >= 0)
	bytes.encode_s32(pos, -1)
	var f := FileAccess.open(path, FileAccess.WRITE)
	f.store_buffer(bytes)
	f.close()
	print("loading corrupted .res (string length field = -1)")
	var res = ResourceLoader.load(path, "", ResourceLoader.CACHE_MODE_IGNORE)
	print("loaded: ", res)
	quit()
