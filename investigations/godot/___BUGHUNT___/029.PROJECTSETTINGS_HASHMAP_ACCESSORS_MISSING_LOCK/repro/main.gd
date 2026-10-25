extends Node
## Reproduces a missing lock on ProjectSettings' `custom_prop_info` HashMap
## (core/config/project_settings.h:103, `HashMap<StringName, PropertyInfo> custom_prop_info`).
##
## `set_custom_property_info()` (core/config/project_settings.cpp:1389) - reached from GDScript
## via the public, ClassDB-bound `ProjectSettings.add_property_info()` - writes
## `custom_prop_info[prop_name] = p_info` with **no lock at all**: a real HashMap insert, which
## can trigger a rehash (reallocating/freeing the backing bucket array).
##
## `_get_property_list()` (core/config/project_settings.cpp:455) DOES start with
## `_THREAD_SAFE_METHOD_` and reads the same `custom_prop_info` map (`.has()` + `operator[]`,
## lines 516-546) - but a lock held by only one side of a shared-container access is no mutual
## exclusion at all: the unlocked writer can mutate/rehash the map while the "locked" reader is
## mid-iteration, racing exactly the same way candidate 010's `props` RBMap did (locked `_set()`
## vs. unlocked `clear()`/`get_order()`).
##
## Both sides are reached here purely through the public, ClassDB-bound GDScript API:
## ProjectSettings.add_property_info() (-> set_custom_property_info(), unlocked) and
## Object.get_property_list() (-> ProjectSettings::_get_property_list(), locked).

var stop_flag := false
var pool_size := 200

func _ready() -> void:
	print("Reproducing missing lock on ProjectSettings' `custom_prop_info` HashMap.")

	# Pre-populate real settings so add_property_info()'s ERR_FAIL_COND(!props.has(name))
	# doesn't just bail out immediately, and so _get_property_list() has real rows to iterate.
	for i in pool_size:
		ProjectSettings.set_setting("race/setting_%d" % i, i)

	var threads: Array[Thread] = []

	# Unlocked-writer threads: ProjectSettings.add_property_info() -> set_custom_property_info()
	# -> custom_prop_info[name] = info, no lock at all.
	for i in 6:
		var t := Thread.new()
		threads.push_back(t)
		t.start(_writer_thread.bind(i))

	# "Locked"-reader threads: Object.get_property_list() -> _get_property_list(), which DOES
	# take _THREAD_SAFE_METHOD_, but that can't protect it from the writer threads above since
	# those never take the same lock.
	for i in 4:
		var t := Thread.new()
		threads.push_back(t)
		t.start(_reader_thread)

	await get_tree().create_timer(8.0).timeout
	stop_flag = true
	for t in threads:
		t.wait_to_finish()
	print("No crash within the time limit (did not race this time).")
	get_tree().quit()

func _writer_thread(seed_i: int) -> void:
	var i := seed_i
	while not stop_flag:
		var key := "race/setting_%d" % (i % pool_size)
		ProjectSettings.add_property_info({
			"name": key,
			"type": TYPE_INT,
			"hint": PROPERTY_HINT_RANGE,
			"hint_string": "0,100",
		})
		i += 1

func _reader_thread() -> void:
	while not stop_flag:
		var props = ProjectSettings.get_property_list()
		# Touch the result so the compiler/VM can't optimize the call away.
		if props.size() < 0:
			print("unreachable")
