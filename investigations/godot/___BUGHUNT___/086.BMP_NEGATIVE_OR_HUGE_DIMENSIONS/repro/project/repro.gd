extends SceneTree

# 24-bit BMP whose header width is -1 (0xffffffff). The loader casts it to size_t.
func _make_bmp(width: int, height: int) -> PackedByteArray:
	var bit_count := 24
	var table_entries := 0
	var offset := 14 + 40 + table_entries * 4
	var pixels := 16
	var b := StreamPeerBuffer.new()
	b.put_u16(0x4d42)
	b.put_u32(offset + pixels)
	b.put_u32(0)
	b.put_u32(offset)
	b.put_u32(40)
	b.put_32(width)
	b.put_32(height)
	b.put_u16(1)
	b.put_u16(bit_count)
	b.put_u32(0)
	b.put_u32(pixels)
	b.put_u32(0)
	b.put_u32(0)
	b.put_u32(table_entries)
	b.put_u32(0)
	for i in pixels:
		b.put_u8(0xff)
	return b.data_array

func _init():
	var args := OS.get_cmdline_user_args()
	var width := int(args[0]) if args.size() > 0 else -1
	var height := int(args[1]) if args.size() > 1 else 2
	var img := Image.new()
	var err := img.load_bmp_from_buffer(_make_bmp(width, height))
	print("width=", width, " height=", height, " load_bmp_from_buffer returned: ", error_string(err), " size=", img.get_size())
	quit()
