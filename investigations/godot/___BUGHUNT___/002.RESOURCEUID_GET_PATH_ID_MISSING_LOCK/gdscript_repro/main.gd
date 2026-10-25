extends Node
## Reproduces the missing lock in ResourceUID.get_path_id() (core/io/resource_uid.cpp:221-227)
## using only public, documented GDScript API - no C++ needed.
##
## Writer thread:  ResourceUID.add_id() / ResourceUID.remove_id()       (locked, real engine code)
## Reader thread:  ResourceLoader.get_resource_uid() -> ResourceUID.get_path_id()  (UNLOCKED - the bug)
##
## Run with an ASan-instrumented editor/export build (scons ... use_asan=yes) to see
## AddressSanitizer abort the process with a heap-use-after-free/heap-buffer-overflow
## inside HashMap::getptr()/_resize_and_rehash() - the exact same crash as the
## standalone C++ repro in ___BUGHUNT___/002.../repro/, but reached purely through the
## scripting API a normal game/plugin would use.

var stop_flag := false

func _ready() -> void:
	print("Reproducing missing lock in ResourceUID.get_path_id() via the public GDScript API.")
	print("Writer thread: ResourceUID.add_id()/remove_id() (locked, real engine code path)")
	print("Reader thread: ResourceLoader.get_resource_uid() -> ResourceUID.get_path_id() (UNLOCKED, the bug)")
	print("Expecting ASan to abort the process shortly if this build has use_asan=yes.")

	var writer := Thread.new()
	var reader := Thread.new()
	writer.start(_writer_loop)
	reader.start(_reader_loop)

	await get_tree().create_timer(20.0).timeout
	stop_flag = true
	writer.wait_to_finish()
	reader.wait_to_finish()
	print("No crash within the time limit (did not race this time).")
	get_tree().quit()

func _writer_loop() -> void:
	var id := 0
	while not stop_flag:
		var path := "res://item_%d.tres" % (id % 5000)
		ResourceUID.add_id(id, path)
		if id > 2000:
			ResourceUID.remove_id(id - 2000)
		id += 1

func _reader_loop() -> void:
	var id := 0
	while not stop_flag:
		var path := "res://item_%d.tres" % (id % 5000)
		ResourceLoader.get_resource_uid(path) # no lock taken on this path - the bug
		id += 1
