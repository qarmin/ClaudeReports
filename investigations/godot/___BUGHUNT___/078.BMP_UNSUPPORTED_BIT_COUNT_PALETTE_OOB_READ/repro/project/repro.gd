extends SceneTree

# BMP with an unsupported bits-per-pixel value (5): the loader treats it as "indexed", allocates a
# 32-entry palette (1 << 5) but never fills the index buffer, then uses the (uninitialized) bytes as palette indices.
func _make_bmp(bit_count: int) -> PackedByteArray:
	var table_entries := 1 << bit_count
	var offset := 14 + 40 + table_entries * 4
	var pixels := 8
	var b := StreamPeerBuffer.new()
	b.put_u16(0x4d42)
	b.put_u32(offset + pixels)
	b.put_u32(0)
	b.put_u32(offset)
	b.put_u32(40)
	b.put_32(2)
	b.put_32(2)
	b.put_u16(1)
	b.put_u16(bit_count)
	b.put_u32(0)
	b.put_u32(pixels)
	b.put_u32(0)
	b.put_u32(0)
	b.put_u32(table_entries)
	b.put_u32(0)
	for i in table_entries:
		b.put_u32(0x00ffffff)
	for i in pixels:
		b.put_u8(0xff)
	return b.data_array

func _init():
	var bits := int(OS.get_cmdline_user_args()[0]) if OS.get_cmdline_user_args().size() > 0 else 5
	var img := Image.new()
	var err := img.load_bmp_from_buffer(_make_bmp(bits))
	print("bit_count=", bits, " load_bmp_from_buffer returned: ", error_string(err), " size=", img.get_size())
	quit()
