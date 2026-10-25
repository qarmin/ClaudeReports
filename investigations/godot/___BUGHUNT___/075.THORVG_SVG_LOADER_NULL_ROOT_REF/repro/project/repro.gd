extends SceneTree

func _init():
	# A truncated SVG: root element without a width/height/viewBox followed by a dangling '<'
	var svg := "<svg><".to_utf8_buffer()
	var img := Image.new()
	var err := img.load_svg_from_buffer(svg)
	print("load_svg_from_buffer returned: ", error_string(err))
	quit()
