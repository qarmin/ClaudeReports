extends SceneTree

func _initialize() -> void:
	var proj := Projection(
		Vector4(1, 5, 9, 13),
		Vector4(2, 6, 11, 15),
		Vector4(4, 7, 11, 15),
		Vector4(4, 8, 12, 16)
	)
	var n := 2000000
	var start := Time.get_ticks_usec()
	var det := 0.0
	for i in range(n):
		det = proj.determinant()
	var elapsed := Time.get_ticks_usec() - start
	print("determinant=%f n=%d total_us=%d us_per_call=%f" % [det, n, elapsed, elapsed / float(n)])
	quit()
