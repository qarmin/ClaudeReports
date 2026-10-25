extends SceneTree

func _initialize() -> void:
	ProjectSettings.load_resource_pack("res://nonexistent.pck")
	print("normal_dir file exists: ", FileAccess.file_exists("res://normal_dir/keep.txt"))
	print("excluded dir file exists: ", FileAccess.file_exists("res://big_excluded_dir/d0/f0.txt"))
	print("project.godot exists: ", FileAccess.file_exists("res://project.godot"))
	quit()
