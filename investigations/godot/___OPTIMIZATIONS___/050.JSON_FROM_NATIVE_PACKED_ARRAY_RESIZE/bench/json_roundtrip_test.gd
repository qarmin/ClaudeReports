extends SceneTree
func _initialize():
	var fail := false
	var v3arr := PackedVector3Array([Vector3(1,2,3), Vector3(4,5,6)])
	var native: Variant = JSON.from_native(v3arr)
	var back: Variant = JSON.to_native(native)
	if back != v3arr:
		fail = true
		print("PackedVector3Array roundtrip FAIL: ", back)

	var colarr := PackedColorArray([Color(0.1,0.2,0.3,0.4), Color(0.5,0.6,0.7,0.8)])
	var native2: Variant = JSON.from_native(colarr)
	var back2: Variant = JSON.to_native(native2)
	if back2 != colarr:
		fail = true
		print("PackedColorArray roundtrip FAIL: ", back2)

	var dict := {"a": 1, "b": [1,2,3]}
	var native3: Variant = JSON.from_native(dict)
	var back3: Variant = JSON.to_native(native3)
	if back3 != dict:
		fail = true
		print("Dictionary roundtrip FAIL: ", back3)

	var i32arr := PackedInt32Array([1,2,3,4,5])
	var native4: Variant = JSON.from_native(i32arr)
	var back4: Variant = JSON.to_native(native4)
	if back4 != i32arr:
		fail = true
		print("PackedInt32Array roundtrip FAIL: ", back4)

	var strarr := PackedStringArray(["a","b","c"])
	var native5: Variant = JSON.from_native(strarr)
	var back5: Variant = JSON.to_native(native5)
	if back5 != strarr:
		fail = true
		print("PackedStringArray roundtrip FAIL: ", back5)

	print("ALL_TESTS_" + ("FAIL" if fail else "PASS"))
	quit()
