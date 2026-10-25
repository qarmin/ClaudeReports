extends Node2D

var velocity := Vector2(randf_range(-50, 50), randf_range(-50, 50))

func _process(delta: float) -> void:
	position += velocity * delta
