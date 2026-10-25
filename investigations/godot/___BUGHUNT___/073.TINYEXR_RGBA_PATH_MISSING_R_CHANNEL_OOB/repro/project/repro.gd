extends SceneTree

func _init():
	var img := Image.create(7, 5, false, Image.FORMAT_RGBA8)
	img.fill(Color(0.25, 0.5, 0.75, 1.0))
	var exr := img.save_exr_to_buffer()
	# Rename channel "R" to "X": file now has channels A, B, G, X (no R, no Y)
	var p := 0
	for i in range(exr.size() - 1):
		if i > 20 and exr[i] == 0x52 and exr[i + 1] == 0 and exr[i - 1] == 0:
			p = i
			break
	assert(p != 0)
	exr[p] = 0x58
	var loaded := Image.new()
	var err := loaded.load_exr_from_buffer(exr)
	print("load_exr_from_buffer returned: ", error_string(err))
	quit()
