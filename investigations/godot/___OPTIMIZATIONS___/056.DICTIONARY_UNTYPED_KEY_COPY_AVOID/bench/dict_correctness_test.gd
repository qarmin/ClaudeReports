extends SceneTree

func _initialize() -> void:
	var fail := false

	# Untyped dictionary: basic read/write/has/erase.
	var d := {}
	d["a"] = 1
	d["b"] = 2
	if d["a"] != 1 or d["b"] != 2:
		fail = true
		print("untyped read/write FAIL")
	if not d.has("a") or d.has("c"):
		fail = true
		print("untyped has FAIL")
	if not d.has_all(["a", "b"]):
		fail = true
		print("untyped has_all FAIL")
	d.erase("a")
	if d.has("a"):
		fail = true
		print("untyped erase FAIL")
	if d.get("nonexistent", "default_val") != "default_val":
		fail = true
		print("untyped get-default FAIL")
	if d.get_or_add("new_key", 99) != 99 or d["new_key"] != 99:
		fail = true
		print("untyped get_or_add FAIL")

	# Untyped dict with non-existent key via operator[] read should error (not tested to avoid
	# crashing); getptr()-based get() with default is the safe check above.

	# Typed dictionary: key coercion (int key declared, float passed should coerce to int).
	var td: Dictionary[int, String] = {}
	td[5] = "five"
	if td[5] != "five":
		fail = true
		print("typed dict basic FAIL")
	# Access via a float key that should coerce to int 5.
	if not td.has(5.0):
		fail = true
		print("typed dict coercion FAIL (has with float key)")

	# Typed dict with String key type.
	var td2: Dictionary[String, int] = {}
	td2["x"] = 10
	td2["y"] = 20
	if td2["x"] != 10 or td2["y"] != 20:
		fail = true
		print("typed string-key dict FAIL")
	td2.erase("x")
	if td2.has("x"):
		fail = true
		print("typed string-key erase FAIL")

	# Dictionary with Array/Object-like reference-typed keys (exercises the refcount path).
	var d3 := {}
	var key_arr := ["complex", "key"]
	d3[key_arr] = "value_for_array_key"
	if d3[key_arr] != "value_for_array_key":
		fail = true
		print("array-key dict FAIL")

	# Large dict stress (hashing collisions etc.)
	var big := {}
	for i in range(1000):
		big["key_%d" % i] = i
	var ok := true
	for i in range(1000):
		if big["key_%d" % i] != i:
			ok = false
	if not ok:
		fail = true
		print("large dict FAIL")

	print("ALL_TESTS_" + ("FAIL" if fail else "PASS"))
	quit()
