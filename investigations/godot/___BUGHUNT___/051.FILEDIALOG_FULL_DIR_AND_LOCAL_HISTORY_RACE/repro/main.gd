extends Node

var running := true
var dialog: FileDialog

func _ready() -> void:
	dialog = FileDialog.new()
	dialog.access = FileDialog.ACCESS_RESOURCES
	add_child(dialog)

	var writers: Array[Thread] = []
	var readers: Array[Thread] = []

	for i in range(3):
		var t := Thread.new()
		t.start(_writer_thread)
		writers.append(t)

	for i in range(3):
		var t := Thread.new()
		t.start(_reader_thread)
		readers.append(t)

	await get_tree().create_timer(3.0).timeout
	running = false

	for t in writers:
		t.wait_to_finish()
	for t in readers:
		t.wait_to_finish()

	print("survived")
	get_tree().quit()

func _writer_thread() -> void:
	var toggle := true
	while running:
		if toggle:
			dialog.current_dir = "res://a"
		else:
			dialog.current_dir = "res://b"
		toggle = !toggle

func _reader_thread() -> void:
	while running:
		var s: String = dialog.current_dir
		var _l = s.length()
