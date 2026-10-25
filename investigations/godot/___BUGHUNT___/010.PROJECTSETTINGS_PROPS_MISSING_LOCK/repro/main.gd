extends Node
## Reproduces a missing lock in ProjectSettings' `props` RBMap
## (core/config/project_settings.h:101, `RBMap<StringName, VariantContainer> props`).
##
## `_set()`/`_get()`/`has_setting()` (core/config/project_settings.cpp:293,386,919) all start
## with `_THREAD_SAFE_METHOD_` and mutate/read `props` under the class-wide mutex - inserting a
## brand new key via `props[p_name] = VariantContainer(...)` triggers a real red-black-tree
## insert/rebalance (core/templates/rb_map.h).
##
## But `clear()`, `get_order()`, `set_order()`, `set_builtin_order()`, `is_builtin_setting()`
## (project_settings.cpp:1056-1082) touch the exact same `props` tree with **no lock at all** -
## `clear()` in particular calls `props.erase()`, a second, fully unsynchronized tree mutation
## (not just a read) that can run concurrently with `_set()`'s locked insert/erase on the very
## same node.
##
## All five are public, ClassDB-bound methods, directly callable from GDScript:
## ProjectSettings.set_setting() -> _set() (locked), ProjectSettings.clear()/get_order()/
## set_order()/is_builtin_setting() (unlocked).

var stop_flag := false
var pool_size := 200

func _ready() -> void:
	print("Reproducing missing lock on ProjectSettings' `props` RBMap.")

	# Pre-populate the key pool so clear()/get_order() have something to race against
	# from the very first iteration.
	for i in pool_size:
		ProjectSettings.set_setting("race/setting_%d" % i, i)

	var threads: Array[Thread] = []

	# Locked-writer threads: churn insert/erase on the SAME keys via the public,
	# _THREAD_SAFE_METHOD_-guarded set_setting() (-> ProjectSettings::_set()).
	for i in 6:
		var t := Thread.new()
		threads.push_back(t)
		t.start(_writer_thread.bind(i))

	# Unlocked-mutator threads: ProjectSettings.clear() -> props.erase(), no lock at all,
	# racing directly against the writer threads' locked inserts on the same keys.
	for i in 6:
		var t := Thread.new()
		threads.push_back(t)
		t.start(_clearer_thread.bind(i))

	# Unlocked-reader threads: get_order()/is_builtin_setting() walk the same tree with no
	# lock, from a different key than the writers/clearers touch (a builtin setting that
	# always exists), to catch corruption from tree-wide rotations, not just same-key races.
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
		ProjectSettings.set_setting(key, i)
		i += 1

func _clearer_thread(seed_i: int) -> void:
	var i := seed_i
	while not stop_flag:
		var key := "race/setting_%d" % (i % pool_size)
		if ProjectSettings.has_setting(key):
			ProjectSettings.clear(key)
		i += 1

func _reader_thread() -> void:
	while not stop_flag:
		if ProjectSettings.has_setting("application/config/name"):
			ProjectSettings.get_order("application/config/name")
