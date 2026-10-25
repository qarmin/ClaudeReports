extends Node
## Reproduces the fully-unlocked CameraServer::feeds Vector<Ref<CameraFeed>>
## (servers/camera/camera_server.cpp) via the public GDScript API:
## CameraServer.add_feed()/remove_feed() (writers, no lock) racing
## CameraServer.get_feeds()/get_feed_count()/get_feed() (readers, no lock).
## CameraServer declares a _THREAD_SAFE_CLASS_ mutex but never actually uses
## it anywhere in camera_server.cpp.

var stop_flag := false

func _ready() -> void:
	print("Reproducing unlocked CameraServer.feeds via add_feed/remove_feed/get_feeds.")
	print("8 writer threads, 8 reader threads.")

	CameraServer.monitoring_feeds = true

	var threads: Array[Thread] = []
	for i in 8:
		var t := Thread.new()
		t.start(_writer_thread)
		threads.push_back(t)
	for i in 8:
		var t := Thread.new()
		t.start(_reader_thread)
		threads.push_back(t)

	await get_tree().create_timer(15.0).timeout
	stop_flag = true
	for t in threads:
		t.wait_to_finish()
	print("No crash within the time limit (did not race this time).")
	get_tree().quit()

func _writer_thread() -> void:
	while not stop_flag:
		var feed := CameraFeed.new()
		CameraServer.add_feed(feed)
		CameraServer.remove_feed(feed)

func _reader_thread() -> void:
	while not stop_flag:
		var _count := CameraServer.get_feed_count()
		var _feeds := CameraServer.feeds()
