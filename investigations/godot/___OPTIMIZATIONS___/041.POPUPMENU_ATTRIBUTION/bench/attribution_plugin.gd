@tool
extends EditorPlugin

func _enter_tree() -> void:
	call_deferred("_run")

func _collect(n: Node, cls: String, out: Array) -> void:
	if n.is_class(cls):
		out.append(n)
	for ch in n.get_children(true):
		_collect(ch, cls, out)

func _chain(n: Node) -> String:
	# Ancestor class chain, nearest first, stopping at the first "interesting" owner.
	var parts := []
	var p := n.get_parent()
	var depth := 0
	while p != null and depth < 6:
		parts.append(p.get_class())
		p = p.get_parent()
		depth += 1
	return " < ".join(parts)

func _run() -> void:
	var pops := []
	_collect(get_tree().root, "PopupMenu", pops)
	var direct_owner := {}
	var owner_kind := {}
	for pm in pops:
		var parent_cls: String = pm.get_parent().get_class() if pm.get_parent() else "<none>"
		direct_owner[parent_cls] = direct_owner.get(parent_cls, 0) + 1
		var kind := "other"
		if parent_cls == "MenuButton" or parent_cls == "OptionButton":
			kind = parent_cls
		owner_kind[kind] = owner_kind.get(kind, 0) + 1

	print("ATTR041 total_PopupMenu=", pops.size())
	print("ATTR041 by_kind=", owner_kind)
	print("ATTR041 --- direct parent class histogram (all) ---")
	var keys := direct_owner.keys()
	keys.sort_custom(func(a, b): return direct_owner[a] > direct_owner[b])
	for k in keys:
		print("ATTR041   %5d  %s" % [direct_owner[k], k])

	print("ATTR041 --- non-MenuButton/OptionButton popups, with ancestor chains ---")
	var chain_count := {}
	for pm in pops:
		var parent_cls: String = pm.get_parent().get_class() if pm.get_parent() else "<none>"
		if parent_cls == "MenuButton" or parent_cls == "OptionButton":
			continue
		var c := _chain(pm)
		chain_count[c] = chain_count.get(c, 0) + 1
	var ck := chain_count.keys()
	ck.sort_custom(func(a, b): return chain_count[a] > chain_count[b])
	for k in ck:
		print("ATTR041   %5d  %s" % [chain_count[k], k])

	# Same for the two classes candidate 027 flagged as already-lazy, as a control.
	for cls in ["LineEdit", "TextEdit", "Tree", "ItemList"]:
		var arr := []
		_collect(get_tree().root, cls, arr)
		print("ATTR041 control %s count=%d" % [cls, arr.size()])

	print("ATTR041 DONE")
	get_tree().quit()
