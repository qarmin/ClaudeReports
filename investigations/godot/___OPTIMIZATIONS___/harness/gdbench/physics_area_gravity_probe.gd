extends SceneTree

# Measures GodotBody2D::integrate_forces()'s per-step cost when many bodies overlap one Area2D -
# see IDEAS.md's "areas.sort()" lead (candidate 048's discovery pass). Each body's `areas` Vector
# is kept sorted by ordered_insert()/remove_at() already, so the unconditional `.sort()` in
# integrate_forces() should be pure overhead in the common case (no runtime priority changes).

const BODY_COUNT := 2000
const WARMUP_FRAMES := 10
const MEASURE_FRAMES := 300

var frame_count := 0
var t_start := 0
var bodies_settled := false

func _initialize() -> void:
	var area := Area2D.new()
	var area_shape := CollisionShape2D.new()
	var rect := RectangleShape2D.new()
	rect.size = Vector2(4000, 4000)
	area_shape.shape = rect
	area.add_child(area_shape)
	area.gravity_space_override = Area2D.SPACE_OVERRIDE_COMBINE
	area.gravity = 98.0
	area.priority = 1
	get_root().add_child(area)

	for i in range(BODY_COUNT):
		var body := RigidBody2D.new()
		body.position = Vector2(randf_range(-1900, 1900), randf_range(-1900, 1900))
		var shape := CollisionShape2D.new()
		var circle := CircleShape2D.new()
		circle.radius = 4.0
		shape.shape = circle
		body.add_child(shape)
		get_root().add_child(body)

	print("bodies=%d" % BODY_COUNT)

func _physics_process(_delta: float) -> bool:
	frame_count += 1
	if frame_count == WARMUP_FRAMES:
		bodies_settled = true
		t_start = Time.get_ticks_usec()
	elif bodies_settled and frame_count == WARMUP_FRAMES + MEASURE_FRAMES:
		var elapsed := Time.get_ticks_usec() - t_start
		print("physics_area_gravity_us_per_frame=%f" % (elapsed / float(MEASURE_FRAMES)))
		quit()
	return false
