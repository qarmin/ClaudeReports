extends Node
## Real second participant for candidate 028 (RemoteDebuggerPeerTCP has_message()/connected
## race). Listens on a loopback TCP port, waits for the real debuggee process (started
## separately with `--remote-debug tcp://127.0.0.1:<PORT>`) to connect, then floods it with
## genuine `var_to_bytes()`-encoded, length-prefixed messages - the exact wire format
## `RemoteDebuggerPeerTCP::_read_in()` decodes on the other side. This drives the debuggee's
## own background `_poll()` thread to really push into `in_queue` / flip `connected`, which is
## what the debuggee's main thread races against via the unlocked `has_message()`/
## `is_peer_connected()`.

const PORT := 17891
const DURATION_SEC := 12.0

var _server := TCPServer.new()
var _peer: StreamPeerTCP

func _ready() -> void:
	print("[server] listening on 127.0.0.1:%d" % PORT)
	var err := _server.listen(PORT, "127.0.0.1")
	if err != OK:
		print("[server] listen failed: %d" % err)
		get_tree().quit(1)
		return

	var waited := 0.0
	while not _server.is_connection_available():
		await get_tree().process_frame
		waited += get_process_delta_time()
		if waited > 20.0:
			print("[server] no debuggee connection within timeout")
			get_tree().quit(1)
			return

	_peer = _server.take_connection()
	print("[server] debuggee connected, flooding real wire-format messages")

	var start_msec := Time.get_ticks_msec()
	var sent := 0
	while Time.get_ticks_msec() - start_msec < DURATION_SEC * 1000.0:
		_peer.poll()
		if _peer.get_status() != StreamPeerTCP.STATUS_CONNECTED:
			print("[server] debuggee closed the connection early")
			break

		# Burst several messages per frame to maximize the chance that the debuggee's
		# main thread observes `has_message()`/`is_peer_connected()` mid-write.
		for _i in 50:
			var payload := var_to_bytes([])
			var header := PackedByteArray()
			header.resize(4)
			header.encode_u32(0, payload.size())
			_peer.put_data(header)
			_peer.put_data(payload)
			sent += 1

		# Drain whatever the debuggee writes back so its own _write_out() never stalls.
		var avail := _peer.get_available_bytes()
		if avail > 0:
			_peer.get_partial_data(avail)

		await get_tree().process_frame

	print("[server] sent %d messages, disconnecting to flip `connected` on the debuggee" % sent)
	_peer.disconnect_from_host()
	await get_tree().create_timer(1.0).timeout
	get_tree().quit(0)
