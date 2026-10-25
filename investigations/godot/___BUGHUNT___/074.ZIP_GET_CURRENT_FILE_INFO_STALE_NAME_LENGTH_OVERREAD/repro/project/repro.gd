extends SceneTree

func _init():
	var path := "/tmp/bughunt_074.zip"
	var zp := ZIPPacker.new()
	zp.open(path)
	zp.start_file("a/")
	zp.close_file()
	zp.start_file("a/b.txt")
	zp.write_file("hello".to_utf8_buffer())
	zp.close_file()
	zp.close()

	var zip := FileAccess.get_file_as_bytes(path)
	# Second central directory header: raise the file name length (offset 28) beyond the end of the file.
	var second := -1
	var from := 0
	while true:
		var i := _find(zip, "PK\u0001\u0002", from)
		if i < 0:
			break
		second = i
		from = i + 1
	zip.encode_u16(second + 28, 92)
	var f := FileAccess.open(path, FileAccess.WRITE)
	f.store_buffer(zip)
	f.close()

	var reader := ZIPReader.new()
	print("open: ", error_string(reader.open(path)))
	print("files: ", reader.get_files())
	quit()

func _find(b: PackedByteArray, s: String, from: int) -> int:
	var n := s.to_utf8_buffer()
	for i in range(from, b.size() - n.size()):
		if b.slice(i, i + n.size()) == n:
			return i
	return -1
