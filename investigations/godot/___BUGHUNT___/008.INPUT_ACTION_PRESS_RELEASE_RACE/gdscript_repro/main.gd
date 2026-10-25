extends Node
## Reproduces the missing lock in Input::action_press()/action_release()
## (core/input/input.cpp:1515-1543), which mutate the engine's
## HashMap<StringName, ActionState> action_states with NO lock at all - via
## the public GDScript API Input.action_press()/Input.action_release(),
## called concurrently from many Threads. Uses built-in input actions
## (ui_accept, ui_cancel, ui_up, ui_down, ui_left, ui_right, ui_select,
## ui_home, ui_end, ui_page_up) so no custom input map is needed - each is
## an independent HashMap key, maximizing insert/rehash churn across threads.

var stop_flag := false
var actions := ["ui_accept", "ui_cancel", "ui_up", "ui_down", "ui_left", "ui_right", "ui_select", "ui_home", "ui_end", "ui_page_up"]

func _ready() -> void:
	print("Reproducing missing lock in Input.action_press()/action_release().")
	print("16 threads calling action_press/action_release on 10 built-in actions concurrently.")

	var threads: Array[Thread] = []
	for i in 16:
		var t := Thread.new()
		threads.push_back(t)
	for t in threads:
		t.start(_hammer_thread)

	await get_tree().create_timer(15.0).timeout
	stop_flag = true
	for t in threads:
		t.wait_to_finish()
	print("No crash within the time limit (did not race this time).")
	get_tree().quit()

func _hammer_thread() -> void:
	var i := 0
	while not stop_flag:
		var action: String = actions[i % actions.size()]
		Input.action_press(action)
		Input.action_release(action)
		i += 1
