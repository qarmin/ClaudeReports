extends Node2D

var accum := 0.0

func _process(_delta: float) -> void:
	accum += position.x
	accum += position.y
