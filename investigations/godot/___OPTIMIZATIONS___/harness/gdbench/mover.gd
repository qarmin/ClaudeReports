extends Node2D

var velocity := Vector2(randf_range(-50, 50), randf_range(-50, 50))

func _process(delta: float) -> void:
	position += velocity * delta
	if position.x > 1000 or position.x < -1000:
		velocity.x = -velocity.x
	if position.y > 1000 or position.y < -1000:
		velocity.y = -velocity.y
