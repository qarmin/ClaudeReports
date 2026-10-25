extends Node2D

var accum := Vector2()

func _process(_delta: float) -> void:
	accum += position
	accum += position
