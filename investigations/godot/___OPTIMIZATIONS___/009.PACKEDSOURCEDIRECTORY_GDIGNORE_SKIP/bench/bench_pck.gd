extends SceneTree

func _initialize() -> void:
	var start := Time.get_ticks_usec()
	ProjectSettings.load_resource_pack("res://nonexistent.pck")
	var elapsed := Time.get_ticks_usec() - start
	print("load_resource_pack_us=%d" % elapsed)
	quit()
