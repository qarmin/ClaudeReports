extends SceneTree

func _init():
	var src := FileAccess.open("/tmp/bughunt_083_file.txt", FileAccess.WRITE)
	src.store_string("hello")
	src.close()
	var packer := PCKPacker.new()
	packer.pck_start("/tmp/bughunt_083.pck")
	packer.add_file("res://a/b.txt", "/tmp/bughunt_083_file.txt")
	packer.flush()

	var bytes := FileAccess.get_file_as_bytes("/tmp/bughunt_083.pck")
	# Header: magic, version, major, minor, patch, flags (u32 each), file_base (u64), directory offset (u64).
	var dir_offset := bytes.decode_u64(32)
	print("directory offset: ", dir_offset, ", file count: ", bytes.decode_u32(dir_offset))
	bytes.encode_u32(dir_offset, 0x7fffffff)
	var f := FileAccess.open("/tmp/bughunt_083.pck", FileAccess.WRITE)
	f.store_buffer(bytes)
	f.close()

	print("loading a ", bytes.size(), " byte .pck whose directory claims 2147483647 files")
	print("load_resource_pack: ", ProjectSettings.load_resource_pack("/tmp/bughunt_083.pck"))
	quit()
