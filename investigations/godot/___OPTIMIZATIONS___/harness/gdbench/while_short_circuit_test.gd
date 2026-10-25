extends SceneTree

func _initialize() -> void:
	var fail := false

	# Basic and/or while loops.
	var i := 0
	var j := 10
	var iters := 0
	while i < 5 and j > 5:
		i += 1
		j -= 1
		iters += 1
	if iters != 5 or i != 5 or j != 5:
		fail = true
		print("basic and-while FAIL: iters=%d i=%d j=%d" % [iters, i, j])

	var k := 0
	iters = 0
	while k >= 10 or k < 3:
		k += 1
		iters += 1
	if iters != 3 or k != 3:
		fail = true
		print("basic or-while FAIL: iters=%d k=%d" % [iters, k])

	# continue inside a short-circuited while.
	var sum := 0
	var n := 0
	var loop_count := 0
	while n < 10 and true:
		n += 1
		loop_count += 1
		if n % 2 == 0:
			continue
		sum += n
	if sum != 25 or loop_count != 10: # 1+3+5+7+9 = 25
		fail = true
		print("continue-in-while FAIL: sum=%d loop_count=%d" % [sum, loop_count])

	# break inside a short-circuited while.
	var m := 0
	var break_hit := false
	while m < 100 or true:
		m += 1
		if m == 7:
			break_hit = true
			break
	if not break_hit or m != 7:
		fail = true
		print("break-in-while FAIL: break_hit=%s m=%d" % [break_hit, m])

	# Nested while loops, both short-circuited.
	var outer := 0
	var inner_total := 0
	while outer < 3 and true:
		var inner := 0
		while inner < 2 or false:
			inner_total += 1
			inner += 1
		outer += 1
	if inner_total != 6 or outer != 3:
		fail = true
		print("nested-while FAIL: inner_total=%d outer=%d" % [inner_total, outer])

	# while with side-effecting short-circuit (right operand must NOT evaluate when left is false for `and`).
	var call_count := 0
	var side_effect_check := func() -> bool:
		call_count += 1
		return true
	var p := 0
	while p < 3 and side_effect_check.call():
		p += 1
	if call_count != 3:
		fail = true
		print("and-short-circuit-eval-count FAIL: call_count=%d (expected 3, not 4)" % call_count)

	# while with a non-and/or condition (falls back to _parse_expression, must still work).
	var q := 0
	while q < 5:
		q += 1
	if q != 5:
		fail = true
		print("plain-while FAIL: q=%d" % q)

	# 3-operand chain in while condition.
	var r := 0
	iters = 0
	while r < 20 and r >= 0 and true:
		r += 1
		iters += 1
		if iters > 25:
			fail = true
			print("3-operand-while INFINITE LOOP guard triggered")
			break
	if r != 20 or iters != 20:
		fail = true
		print("3-operand-while FAIL: r=%d iters=%d" % [r, iters])

	print("ALL_TESTS_" + ("FAIL" if fail else "PASS"))
	quit()
