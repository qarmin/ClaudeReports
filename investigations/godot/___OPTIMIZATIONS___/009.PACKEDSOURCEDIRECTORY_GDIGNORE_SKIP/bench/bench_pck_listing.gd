extends SceneTree

func _initialize() -> void:
	ProjectSettings.load_resource_pack("res://nonexistent.pck")
	print("dir listing normal_dir: ", DirAccess.get_files_at("res://normal_dir"))
	print("dir listing big_excluded_dir/d0: ", DirAccess.get_files_at("res://big_excluded_dir/d0"))
	quit()
