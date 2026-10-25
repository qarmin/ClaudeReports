extends SceneTree

func _init():
	var args := OS.get_cmdline_user_args()
	var mode := args[0] if args.size() > 0 else "trailing"
	var raw := PackedByteArray()
	raw.resize(1000)
	for i in raw.size():
		raw[i] = i % 7
	var gz := raw.compress(FileAccess.COMPRESSION_GZIP)
	if mode == "trailing":
		gz.append(0) # a single padding byte after a complete gzip stream
	print("input bytes: ", gz.size(), " mode: ", mode)
	var out := gz.decompress_dynamic(1 << 20, FileAccess.COMPRESSION_GZIP)
	print("decompress_dynamic returned ", out.size(), " bytes")
	quit()
