extends Node
## Reproduces the missing lock in CallQueue::get_max_buffer_usage()
## (core/object/message_queue.cpp:456-458) using only the public GDScript API:
## call_deferred() (writer side, locked) pushes into the same global
## MessageQueue::get_singleton() that Performance.get_monitor(MEMORY_MESSAGE_BUFFER_MAX)
## reads (reader side, unlocked) - both race on CallQueue::pages/page_bytes.

var stop_flag := false
var counter := 0

func _ready() -> void:
	print("Reproducing missing lock in CallQueue::get_max_buffer_usage() via")
	print("Performance.get_monitor(Performance.MEMORY_MESSAGE_BUFFER_MAX).")
	print("4 writer threads calling call_deferred(), 4 reader threads polling the monitor.")

	var threads: Array[Thread] = []
	for i in 4:
		var t := Thread.new()
		t.start(_writer_thread)
		threads.push_back(t)
	for i in 4:
		var t := Thread.new()
		t.start(_reader_thread)
		threads.push_back(t)

	await get_tree().create_timer(20.0).timeout
	stop_flag = true
	for t in threads:
		t.wait_to_finish()
	print("No crash within the time limit (did not race this time).")
	get_tree().quit()

func _noop() -> void:
	counter += 1

func _writer_thread() -> void:
	while not stop_flag:
		call_deferred("_noop")

func _reader_thread() -> void:
	while not stop_flag:
		var _u := Performance.get_monitor(Performance.MEMORY_MESSAGE_BUFFER_MAX)
