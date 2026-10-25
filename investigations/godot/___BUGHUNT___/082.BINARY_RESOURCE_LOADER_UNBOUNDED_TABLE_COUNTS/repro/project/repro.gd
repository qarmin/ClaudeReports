extends SceneTree

# Binary .res header after the class name: importmd offset (8), flags (4), uid (8), 11 reserved ints (44),
# then the string table size (u32), the external resource count (u32) and the internal resource count (u32).
func _init():

	var path := "/tmp/bughunt_082.res"
	ResourceSaver.save(Gradient.new(), path)
	var bytes := FileAccess.get_file_as_bytes(path)
	var needle := PackedByteArray([9, 0, 0, 0]) + "Gradient".to_utf8_buffer() + PackedByteArray([0])
	var pos := -1
	for i in range(bytes.size() - needle.size()):
		if bytes.slice(i, i + needle.size()) == needle:
			pos = i
			break
	assert(pos >= 0)
	var table := pos + needle.size() + 8 + 4 + 8 + 44
	print("string table size field: ", bytes.decode_u32(table))
	bytes.encode_u32(table, 0x7fffffff)
	var f := FileAccess.open(path, FileAccess.WRITE)
	f.store_buffer(bytes)
	f.close()
	print("loading a ", bytes.size(), " byte .res whose string table claims 2147483647 entries")
	var res = ResourceLoader.load(path, "", ResourceLoader.CACHE_MODE_IGNORE)
	print("loaded: ", res)
	quit()
