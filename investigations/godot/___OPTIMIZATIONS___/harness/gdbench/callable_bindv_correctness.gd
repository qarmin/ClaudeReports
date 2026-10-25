extends SceneTree

func _initialize() -> void:
	var fail := false

	# Basic bindv + call roundtrip.
	var f := func(a: int, b: int, c: int) -> int:
		return a + b + c
	var bound := f.bindv([1, 2, 3])
	var result = bound.call()
	if result != 6:
		fail = true
		print("basic bindv+call FAIL: result=%s" % result)

	# get_bound_arguments roundtrip.
	var got := bound.get_bound_arguments()
	if got != [1, 2, 3]:
		fail = true
		print("get_bound_arguments FAIL: got=%s" % got)

	# Large array bindv + call, verifying every element survives the O(1) share correctly.
	var big_args := []
	for i in range(2000):
		big_args.append(i)
	var sum_func := func(arr: Array) -> int:
		var s := 0
		for v in arr:
			s += v
		return s
	# bindv binds arguments individually (prepended before call-time args), not as one array -
	# so bind 3 separate ints instead to keep this a realistic bindv shape, and separately check
	# a huge single-array-argument bind for the array-copy path.
	var single_arr_bind := (func(arr: Array) -> int:
		var s := 0
		for v in arr:
			s += v
		return s
	).bindv([big_args])
	var big_result = single_arr_bind.call()
	var expected_sum := 0
	for i in range(2000):
		expected_sum += i
	if big_result != expected_sum:
		fail = true
		print("large-array bindv+call FAIL: got=%s expected=%s" % [big_result, expected_sum])

	var big_got := single_arr_bind.get_bound_arguments()
	if big_got.size() != 1 or big_got[0] != big_args:
		fail = true
		print("large-array get_bound_arguments FAIL: size=%d" % big_got.size())

	# Empty bindv (should return same callable per the early-out).
	var empty_bound = f.bindv([])
	if empty_bound.get_bound_arguments().size() != 0:
		fail = true
		print("empty bindv FAIL")

	# Mutating the original array after bindv must NOT affect the already-bound arguments
	# (bindv should have taken its own copy/share at bind time, not a live reference).
	var mutable_args := [10, 20]
	var mut_bound := (func(x: int, y: int) -> int: return x + y).bindv(mutable_args)
	mutable_args[0] = 999
	var mut_result = mut_bound.call()
	if mut_result != 30:
		fail = true
		print("mutation-independence FAIL: result=%s (expected 30, mutable_args changed after bind)" % mut_result)

	print("ALL_TESTS_" + ("FAIL" if fail else "PASS"))
	quit()
