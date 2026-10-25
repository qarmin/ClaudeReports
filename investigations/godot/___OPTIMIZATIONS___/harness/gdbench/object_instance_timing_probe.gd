extends SceneTree
func _initialize() -> void:
	var n := 500000
	var warmup := n / 10
	for i in range(warmup):
		var o := Object.new()
		o.free()
	var start := Time.get_ticks_usec()
	for i in range(n):
		var o := Object.new()
		o.free()
	var elapsed := Time.get_ticks_usec() - start
	print("object_instance_us_per_iter=%f" % (elapsed / float(n)))
	quit()
