extends SceneTree

# Uncompressed 24-bit TGA that claims 65535 x 32767 pixels but only carries 300 bytes of pixel data.
func _init():
	var b := StreamPeerBuffer.new()
	b.put_u8(0)        # id length
	b.put_u8(0)        # color map type
	b.put_u8(2)        # image type: uncompressed true-color
	b.put_u16(0)
	b.put_u16(0)
	b.put_u8(0)
	b.put_u16(0)       # x origin
	b.put_u16(0)       # y origin
	b.put_u16(65535)   # width
	b.put_u16(32767)   # height
	b.put_u8(24)       # pixel depth
	b.put_u8(0)        # descriptor: bottom-left origin
	for i in 300:
		b.put_u8(0xff)
	var img := Image.new()
	var err := img.load_tga_from_buffer(b.data_array)
	print("load_tga_from_buffer returned: ", error_string(err))
	quit()
