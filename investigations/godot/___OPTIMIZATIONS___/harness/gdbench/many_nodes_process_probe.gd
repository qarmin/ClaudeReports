extends SceneTree

# Measures per-frame overhead of a large number of Node2D each running a GDScript _process().
# Parametrized by mover script (mover_empty/mover_simple/mover_whole_read/mover_component_read/
# mover.gd) via the first cmdline user arg, to isolate which part of a typical "move a node"
# script costs what - see IDEAS.md's "native property access via OPCODE_GET_NAMED" lead
# (candidate 048's discovery pass).
#
# IMPORTANT: headless mode applies OS's default `low_processor_usage_mode_sleep_usec` (6900us,
# see core/core_bind.cpp ADD_PROPERTY_DEFAULT) to EVERY frame regardless of actual work, via
# OS::add_frame_delay()'s `!p_can_draw` branch - this swamps any real per-node timing signal
# unless disabled first. Confirmed by comparing an empty _process() against a real one and
# seeing identical ~6900us/frame in both, which is a hardcoded default, not a coincidence.

const NODE_COUNT := 5000
const WARMUP_FRAMES := 10
const MEASURE_FRAMES := 300

var frame_count := 0
var t_start := 0
var measuring := false
var mover_script: Script

func _initialize() -> void:
	OS.set_low_processor_usage_mode_sleep_usec(0)
	var args := OS.get_cmdline_user_args()
	var mover_name := "mover.gd" if args.is_empty() else args[0]
	mover_script = load("res://%s" % mover_name)

	for i in range(NODE_COUNT):
		var node: Node2D = mover_script.new()
		node.position = Vector2(randf_range(-1000, 1000), randf_range(-1000, 1000))
		get_root().add_child(node)
	print("nodes=%d mover=%s" % [NODE_COUNT, mover_name])

func _process(_delta: float) -> bool:
	frame_count += 1
	if frame_count == WARMUP_FRAMES:
		measuring = true
		t_start = Time.get_ticks_usec()
	elif measuring and frame_count == WARMUP_FRAMES + MEASURE_FRAMES:
		var elapsed := Time.get_ticks_usec() - t_start
		print("many_nodes_process_us_per_frame=%f" % (elapsed / float(MEASURE_FRAMES)))
		quit()
	return false
