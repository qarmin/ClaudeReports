extends SceneTree
func _initialize():
	var fail := false
	var root_holder := Node.new()
	get_root().add_child(root_holder)
	var left: Node = root_holder
	var right: Node = root_holder
	for i in range(5):
		var nl := Node.new()
		nl.name = "L%d" % i
		var nr := Node.new()
		nr.name = "R%d" % i
		left.add_child(nl)
		right.add_child(nr)
		left = nl
		right = nr
	var p1 := left.get_path_to(right)
	if String(p1) != "../../../../../R0/R1/R2/R3/R4":
		fail = true
		print("cross-branch path FAIL: ", p1)

	var p2 := right.get_path_to(left)
	if String(p2) != "../../../../../L0/L1/L2/L3/L4":
		fail = true
		print("reverse cross-branch path FAIL: ", p2)

	var p3 := root_holder.get_path_to(left)
	if String(p3) != "L0/L1/L2/L3/L4":
		fail = true
		print("root->deep path FAIL: ", p3)

	var p4 := left.get_path_to(root_holder)
	if String(p4) != "../../../../..":
		fail = true
		print("deep->root path FAIL: ", p4)

	var p5 := left.get_path_to(left)
	if String(p5) != ".":
		fail = true
		print("self path FAIL: ", p5)

	# Not-in-tree node (falls back to old HashSet path unconditionally).
	var orphan := Node.new()
	var orphan_child := Node.new()
	orphan.add_child(orphan_child)
	var p6 := orphan.get_path_to(orphan_child)
	if String(p6) != String(orphan_child.name):
		fail = true
		print("orphan path FAIL: ", p6)
	orphan.free()

	print("ALL_TESTS_" + ("FAIL" if fail else "PASS"))
	quit()
