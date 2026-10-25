extends Node
## Reproduces the missing lock in Color::find_named_color() (core/math/color.cpp:412-431)
## using only the public GDScript API - Color.from_string() falls back to named()
## when the string isn't a valid hex color, which calls find_named_color().

var stop_flag := false

func _ready() -> void:
	print("Reproducing missing lock in Color.find_named_color() via Color.from_string().")
	print("10 threads calling Color.from_string(<name>, Color.BLACK) concurrently on first use.")

	var threads: Array[Thread] = []
	for i in 10:
		var t := Thread.new()
		threads.push_back(t)
	for t in threads:
		t.start(_looker_thread)

	await get_tree().create_timer(15.0).timeout
	stop_flag = true
	for t in threads:
		t.wait_to_finish()
	print("No crash within the time limit (did not race this time).")
	get_tree().quit()

func _looker_thread() -> void:
	var names := ["yellow", "cornflowerblue", "darkorange", "chartreuse", "crimson"]
	var i := 0
	while not stop_flag:
		var c := Color.from_string(names[i % names.size()], Color.BLACK)
		i += 1
