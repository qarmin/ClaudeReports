extends SceneTree

# Usage: --headless --path harness/gdbench --script textedit_probe.gd -- <line_len>
# Confirmed O(n) per single-character insert, dominated by TextServerAdvanced fully re-shaping
# the whole line on every edit (see ___OPTIMIZATIONS___/README.md's Instrumentation section) -
# not a redundant-call bug, not turned into a candidate.

func _initialize() -> void:
	var args := OS.get_cmdline_user_args()
	var line_len := 20000 if args.is_empty() else int(args[0])
	var te := TextEdit.new()
	te.text = "a".repeat(line_len)

	var iterations := 20
	var start := Time.get_ticks_usec()
	for i in range(iterations):
		te.insert_text("x", 0, 0)
	var elapsed := Time.get_ticks_usec() - start
	print("line_len=%d us_total=%d us_per_op=%f" % [line_len, elapsed, elapsed / float(iterations)])

	te.queue_free()
	quit()
