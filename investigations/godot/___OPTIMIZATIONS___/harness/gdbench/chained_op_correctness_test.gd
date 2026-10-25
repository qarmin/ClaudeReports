extends SceneTree

func _initialize() -> void:
	var fail := false

	var a := 1
	var b := 2
	var c := 3
	var d := 4
	var e := 5
	var r1 := a + b + c + d + e
	if r1 != 15:
		fail = true
		print("int chain FAIL: ", r1)

	var f1 := 1.5
	var f2 := 2.5
	var f3 := 3.0
	var r2 := f1 + f2 * f3 - f1 / f2
	if abs(r2 - 8.4) > 0.0001:
		fail = true
		print("mixed float chain FAIL: ", r2)

	var s1 := "a"
	var s2 := "b"
	var s3 := "c"
	var r3 := s1 + s2 + s3 + s1 + s2
	if r3 != "abcab":
		fail = true
		print("string chain FAIL: ", r3)

	var v1 := Vector2(1, 2)
	var v2 := Vector2(3, 4)
	var v3 := Vector2(5, 6)
	var r4 := v1 + v2 + v3 - v1
	if r4 != Vector2(8, 10):
		fail = true
		print("vector chain FAIL: ", r4)

	# Long chain (10 operands) like the PR's own benchmark shape.
	var r5 := 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9
	if r5 != 45:
		fail = true
		print("long int chain FAIL: ", r5)

	# Nested/parenthesized chains.
	var r6 := (a + b) * (c + d) - (e - a)
	if r6 != 17:
		fail = true
		print("nested chain FAIL: ", r6)

	# Comparison chain result feeding into logical ops (mix of 048 and 052 mechanisms).
	var r7 := (a < b) and (c < d) and (d < e + 100)
	if r7 != true:
		fail = true
		print("comparison+logic chain FAIL: ", r7)

	print("ALL_TESTS_" + ("FAIL" if fail else "PASS"))
	quit()
