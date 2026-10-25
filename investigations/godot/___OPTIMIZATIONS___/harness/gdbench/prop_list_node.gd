@tool
extends Node

# Node with a `_get_property_list()` override exposing many Dictionary properties, matching
# PR godotengine/godot#120100's "Scenario B" (100 member empty dictionaries in _get_property_list).
# Used by ___OPTIMIZATIONS___/harness/gdbench/packed_scene_instantiate_probe.gd (candidate 051).

const PROP_COUNT := 100

var _data: Dictionary = {}

func _get_property_list() -> Array[Dictionary]:
	var props: Array[Dictionary] = []
	for i in range(PROP_COUNT):
		props.append({
			"name": "dict_prop_%d" % i,
			"type": TYPE_DICTIONARY,
			"usage": PROPERTY_USAGE_DEFAULT,
		})
	return props

func _get(property: StringName):
	if _data.has(property):
		return _data[property]
	return null

func _set(property: StringName, value) -> bool:
	if String(property).begins_with("dict_prop_"):
		_data[property] = value
		return true
	return false

func _property_can_revert(property: StringName) -> bool:
	return String(property).begins_with("dict_prop_")

func _property_get_revert(property: StringName):
	return {}
