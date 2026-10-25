extends Node
## Debuggee side of candidate 028's repro. Nothing special needed here - the engine's own
## main-loop debugger polling (RemoteDebugger::poll_events(), called every frame while
## `--remote-debug` is active) already races the background RemoteDebuggerPeerTCP `_poll()`
## thread via the unlocked `has_message()`/`is_peer_connected()` accessors. Just stay alive
## long enough for the real server process to flood real wire traffic at us.

func _ready() -> void:
	print("[client] running, waiting for remote debugger traffic")
	await get_tree().create_timer(14.0).timeout
	print("[client] done")
	get_tree().quit(0)
