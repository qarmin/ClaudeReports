extends SceneTree

const WARMUP_FRACTION := 0.1

func _timed(iterations: int, fn: Callable) -> float:
	var warmup := int(iterations * WARMUP_FRACTION)
	for i in range(warmup):
		fn.call()
	var start := Time.get_ticks_usec()
	for i in range(iterations):
		fn.call()
	var elapsed := Time.get_ticks_usec() - start
	return elapsed / float(iterations)

func bench_string_concat() -> void:
	var n := 200000
	var per := _timed(n, func():
		var s := ""
		for i in range(20):
			s += "x"
	)
	print("string_concat_us_per_iter=%f" % per)

func bench_string_format() -> void:
	var n := 200000
	var per := _timed(n, func():
		var s := "value: %d, name: %s" % [42, "test"]
	)
	print("string_format_us_per_iter=%f" % per)

func bench_array_append() -> void:
	var n := 50000
	var per := _timed(n, func():
		var a := []
		for i in range(50):
			a.append(i)
	)
	print("array_append_us_per_iter=%f" % per)

func bench_array_cow_pass() -> void:
	var base := []
	for i in range(200):
		base.append(i)
	var n := 200000
	var per := _timed(n, func():
		var a = base
		a.append(999)
	)
	print("array_cow_pass_us_per_iter=%f" % per)

func bench_dict_insert() -> void:
	var n := 50000
	var per := _timed(n, func():
		var d := {}
		for i in range(50):
			d[i] = i
	)
	print("dict_insert_us_per_iter=%f" % per)

func bench_packed_array() -> void:
	var n := 20000
	var per := _timed(n, func():
		var a := PackedFloat32Array()
		for i in range(200):
			a.append(float(i))
	)
	print("packed_array_us_per_iter=%f" % per)

func bench_vector_math() -> void:
	var n := 500000
	var per := _timed(n, func():
		var v := Vector3(1.0, 2.0, 3.0)
		for i in range(20):
			v = v.normalized() + Vector3(0.1, 0.2, 0.3)
	)
	print("vector_math_us_per_iter=%f" % per)

func bench_variant_call() -> void:
	var n := 200000
	var obj := Node.new()
	var per := _timed(n, func():
		for i in range(20):
			obj.set_name("n%d" % i)
	)
	obj.free()
	print("variant_call_us_per_iter=%f" % per)

func bench_if_logic_baseline() -> void:
	var n := 20000000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		pass
	var elapsed := Time.get_ticks_usec() - t0
	print("if_logic_baseline_us_per_iter=%f" % (elapsed / float(n)))

func bench_if_logic_and() -> void:
	var a := false
	var b := false
	var n := 20000000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		if a and b:
			pass
	var elapsed := Time.get_ticks_usec() - t0
	print("if_logic_and_us_per_iter=%f" % (elapsed / float(n)))

func bench_if_logic_or() -> void:
	var a := false
	var b := false
	var n := 20000000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		if a or b:
			pass
	var elapsed := Time.get_ticks_usec() - t0
	print("if_logic_or_us_per_iter=%f" % (elapsed / float(n)))

func bench_if_logic_nested() -> void:
	var a := false
	var b := true
	var c := true
	var d := false
	var n := 20000000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		if (a and b) or (c and d):
			pass
	var elapsed := Time.get_ticks_usec() - t0
	print("if_logic_nested_us_per_iter=%f" % (elapsed / float(n)))

var _c0 := 0
var _c1 := 1
var _c2 := 2
var _c3 := 3
var _c4 := 4
var _c5 := 5
var _c6 := 6
var _c7 := 7
var _c8 := 8
var _c9 := 9

func _chained_add() -> int:
	return _c0 + _c1 + _c2 + _c3 + _c4 + _c5 + _c6 + _c7 + _c8 + _c9

func bench_lambda_create_captures() -> void:
	var a := 1
	var b := 2.0
	var c := "three"
	var d := Vector2(4, 5)
	var e := true
	var n := 2000000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		var f := func(): return a + b + c.length() + d.x + (1 if e else 0)
	var elapsed := Time.get_ticks_usec() - t0
	print("lambda_create_captures_us_per_iter=%f" % (elapsed / float(n)))

func bench_lambda_create_self_captures() -> void:
	var n := 2000000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		var f := func(): return self
	var elapsed := Time.get_ticks_usec() - t0
	print("lambda_create_self_captures_us_per_iter=%f" % (elapsed / float(n)))

func bench_callable_bindv_large() -> void:
	var args := []
	for i in range(2000):
		args.append(i)
	var f := func(a: Array): pass
	var n := 20000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		var bound := f.bindv(args)
	var elapsed := Time.get_ticks_usec() - t0
	print("callable_bindv_large_us_per_iter=%f" % (elapsed / float(n)))

func bench_callable_get_bound_arguments_large() -> void:
	var args := []
	for i in range(2000):
		args.append(i)
	var f := func(a: Array): pass
	var bound := f.bindv(args)
	var n := 20000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		var got := bound.get_bound_arguments()
	var elapsed := Time.get_ticks_usec() - t0
	print("callable_get_bound_arguments_large_us_per_iter=%f" % (elapsed / float(n)))

func bench_dict_string_key_read() -> void:
	var d := {"the_key_name": 42, "other_key": 1, "third_key": 2}
	var n := 5000000
	var t0 := Time.get_ticks_usec()
	var acc := 0
	for i in range(n):
		acc += d["the_key_name"]
	var elapsed := Time.get_ticks_usec() - t0
	print("dict_string_key_read_us_per_iter=%f (acc=%d)" % [(elapsed / float(n)), acc])

func bench_dict_string_key_write() -> void:
	var d := {"the_key_name": 42, "other_key": 1, "third_key": 2}
	var n := 5000000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		d["the_key_name"] = i
	var elapsed := Time.get_ticks_usec() - t0
	print("dict_string_key_write_us_per_iter=%f" % (elapsed / float(n)))

func bench_dict_string_key_contains() -> void:
	var d := {"the_key_name": 42, "other_key": 1, "third_key": 2}
	var n := 5000000
	var t0 := Time.get_ticks_usec()
	var hits := 0
	for i in range(n):
		if d.has("the_key_name"):
			hits += 1
	var elapsed := Time.get_ticks_usec() - t0
	print("dict_string_key_contains_us_per_iter=%f (hits=%d)" % [(elapsed / float(n)), hits])

func bench_while_logic_and() -> void:
	var n := 20000000
	var i := 0
	var a := true
	var b := true
	var t0 := Time.get_ticks_usec()
	while i < n and a and b:
		i += 1
	var elapsed := Time.get_ticks_usec() - t0
	print("while_logic_and_us_per_iter=%f" % (elapsed / float(n)))

func bench_while_logic_or() -> void:
	var n := 20000000
	var i := 0
	var a := false
	var b := false
	var t0 := Time.get_ticks_usec()
	while a or b or i < n:
		i += 1
	var elapsed := Time.get_ticks_usec() - t0
	print("while_logic_or_us_per_iter=%f" % (elapsed / float(n)))

func bench_get_property_list() -> void:
	var n := 300000
	var obj := Node2D.new()
	var per := _timed(n, func():
		obj.get_property_list()
	)
	obj.free()
	print("get_property_list_us_per_iter=%f" % per)

func bench_get_path_to_deep() -> void:
	var depth := 300
	var shallow := Node.new()
	get_root().add_child(shallow)
	var deep_node: Node = shallow
	for i in range(depth - 1):
		var child := Node.new()
		deep_node.add_child(child)
		deep_node = child

	var n := 20000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		shallow.get_path_to(deep_node)
	var elapsed := Time.get_ticks_usec() - t0
	print("get_path_to_deep_us_per_iter=%f" % (elapsed / float(n)))

func bench_get_path_to_cross_branch() -> void:
	var depth := 150
	var root_holder := Node.new()
	get_root().add_child(root_holder)
	var left: Node = root_holder
	var right: Node = root_holder
	for i in range(depth):
		var nl := Node.new()
		var nr := Node.new()
		left.add_child(nl)
		right.add_child(nr)
		left = nl
		right = nr

	var n := 20000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		left.get_path_to(right)
	var elapsed := Time.get_ticks_usec() - t0
	print("get_path_to_cross_branch_us_per_iter=%f" % (elapsed / float(n)))

func bench_chained_operator_call() -> void:
	var n := 5000000
	var t0 := Time.get_ticks_usec()
	for i in range(n):
		_chained_add()
	var elapsed := Time.get_ticks_usec() - t0
	print("chained_operator_call_us_per_iter=%f" % (elapsed / float(n)))

func bench_json_from_native_packed() -> void:
	var n := 1000000
	var arr := PackedVector3Array()
	arr.resize(n)
	for i in range(n):
		arr[i] = Vector3(float(i), float(i) * 0.5, float(i) * 0.25)
	var t0 := Time.get_ticks_usec()
	var result: Variant = JSON.from_native(arr)
	var elapsed := Time.get_ticks_usec() - t0
	print("json_from_native_packed_us_total=%f (n=%d)" % [elapsed, n])

func bench_resource_set_path() -> void:
	var counter := 0
	var n := 50000
	var per := _timed(n, func():
		var res := Resource.new()
		res.resource_path = "res://bench_resource_%d.tres" % counter
		counter += 1
	)
	print("resource_set_path_us_per_iter=%f" % per)

func bench_regex() -> void:
	var re := RegEx.new()
	re.compile("(\\w+)@(\\w+)\\.com")
	var n := 50000
	var per := _timed(n, func():
		re.search("contact us at hello@example.com for info")
	)
	print("regex_us_per_iter=%f" % per)

func bench_json() -> void:
	var data := {"a": 1, "b": [1, 2, 3], "c": {"nested": true, "s": "hello world"}}
	var s := JSON.stringify(data)
	var n := 50000
	var per := _timed(n, func():
		JSON.stringify(data)
	)
	print("json_stringify_us_per_iter=%f" % per)

func bench_json_parse() -> void:
	var data := {"a": 1, "b": [1, 2, 3], "c": {"nested": true, "s": "hello world"}}
	var s := JSON.stringify(data)
	var n := 50000
	var per := _timed(n, func():
		JSON.parse_string(s)
	)
	print("json_parse_us_per_iter=%f" % per)

func bench_node_instance() -> void:
	var n := 20000
	var per := _timed(n, func():
		var node := Node.new()
		node.free()
	)
	print("node_instance_us_per_iter=%f" % per)

func bench_signal_emit() -> void:
	var n := 200000
	var node := Node.new()
	var count := 0
	var handler := func(): count += 1
	node.tree_entered.connect(handler)
	var per := _timed(n, func():
		for i in range(10):
			node.tree_entered.emit()
	)
	node.free()
	print("signal_emit_us_per_iter=%f" % per)

var MODES := {
	"string_concat": bench_string_concat,
	"string_format": bench_string_format,
	"array_append": bench_array_append,
	"array_cow_pass": bench_array_cow_pass,
	"dict_insert": bench_dict_insert,
	"packed_array": bench_packed_array,
	"vector_math": bench_vector_math,
	"variant_call": bench_variant_call,
	"if_logic_baseline": bench_if_logic_baseline,
	"if_logic_and": bench_if_logic_and,
	"if_logic_or": bench_if_logic_or,
	"if_logic_nested": bench_if_logic_nested,
	"resource_set_path": bench_resource_set_path,
	"json_from_native_packed": bench_json_from_native_packed,
	"chained_operator_call": bench_chained_operator_call,
	"get_property_list": bench_get_property_list,
	"lambda_create_captures": bench_lambda_create_captures,
	"lambda_create_self_captures": bench_lambda_create_self_captures,
	"callable_bindv_large": bench_callable_bindv_large,
	"callable_get_bound_arguments_large": bench_callable_get_bound_arguments_large,
	"dict_string_key_read": bench_dict_string_key_read,
	"dict_string_key_write": bench_dict_string_key_write,
	"dict_string_key_contains": bench_dict_string_key_contains,
	"while_logic_and": bench_while_logic_and,
	"while_logic_or": bench_while_logic_or,
	"get_path_to_deep": bench_get_path_to_deep,
	"get_path_to_cross_branch": bench_get_path_to_cross_branch,
	"regex": bench_regex,
	"json_stringify": bench_json,
	"json_parse": bench_json_parse,
	"node_instance": bench_node_instance,
	"signal_emit": bench_signal_emit,
}

func _initialize() -> void:
	var args := OS.get_cmdline_user_args()
	if args.is_empty():
		print("modes: %s" % [MODES.keys()])
		print("usage: godot --headless --script bench.gd -- <mode> [<mode> ...]")
	else:
		for mode in args:
			if MODES.has(mode):
				MODES[mode].call()
			else:
				print("unknown mode: %s" % mode)
	quit()
