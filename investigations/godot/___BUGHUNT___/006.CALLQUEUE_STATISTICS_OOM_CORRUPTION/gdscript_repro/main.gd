extends Node
## Reproduces heap corruption in CallQueue::push_callablep()'s out-of-memory
## branch (core/object/message_queue.cpp:94-99), which calls statistics()
## (:347-440). statistics() destructs every pending Message/Variant in place
## (calling ~Callable()/~Variant()/~Message() on all of them) but does NOT
## reset pages_used/page_bytes the way flush()/clear() do - so the queue is
## left believing it still holds pages_used pages of VALID messages that have
## actually already been destructed. The very next push (or an engine-internal
## per-frame flush()) then operates on already-destructed Callables/Variants.
## Single-threaded is enough: this isn't fundamentally a race, just reachable
## faster under concurrency (see ../RESULT.md).

func _noop() -> void:
	pass

func _ready() -> void:
	print("Flooding the deferred-call queue past its configured 1MB cap.")
	var i := 0
	# Each push_callable() needs room for a Message struct; with 0 args that's
	# small, so a few hundred thousand calls comfortably exceeds 1MB / 4096-byte
	# pages before the engine gets a chance to flush (flush only happens once
	# per frame, and this whole loop runs synchronously within _ready()).
	while i < 2_000_000:
		call_deferred("_noop")
		i += 1
	print("Done queuing. If we get here without crashing, flush next frame.")
	# Give the engine a couple of real frames to flush the (still-overflowing)
	# queue - that's where a corrupted queue would crash if the fix didn't hold.
	await get_tree().create_timer(1.0).timeout
	print("No crash within the time limit (did not race this time).")
	get_tree().quit()
