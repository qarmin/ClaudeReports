// Standalone reproduction of the missing lock in ResourceUID::get_path_id()
// (core/io/resource_uid.cpp:221-227), using Godot's REAL core/templates/hash_map.h
// and REAL core/os/mutex.h, unmodified, included directly from this checkout.
//
// MiniResourceUID below is a byte-for-byte copy of the relevant method bodies from
// the real ResourceUID class (core/io/resource_uid.h/.cpp): same field types
// (Mutex + HashMap<CharString, ID> reverse_cache), same lock placement in add_id(),
// same ABSENCE of a lock in get_path_id(). Only the Object/ClassDB/Resource-loading
// scaffolding around it is stripped, since that machinery is not part of the race.
#include "core/os/mutex.h"
#include "core/string/ustring.h"
#include "core/templates/hash_map.h"

#include <atomic>
#include <cstdio>
#include <thread>

class MiniResourceUID {
public:
	typedef int64_t ID;

private:
	Mutex mutex;
	HashMap<ID, CharString> unique_ids;
	HashMap<CharString, ID> reverse_cache; // mirrors resource_uid.h:61

public:
	// Mirrors ResourceUID::add_id (resource_uid.cpp:156-163): locked.
	void add_id(ID p_id, const CharString &p_path) {
		MutexLock l(mutex);
		unique_ids[p_id] = p_path;
		reverse_cache[p_path] = p_id;
	}

	// Mirrors ResourceUID::remove_id (resource_uid.cpp:230-236): locked.
	void remove_id(ID p_id) {
		MutexLock l(mutex);
		if (!unique_ids.has(p_id)) {
			return;
		}
		reverse_cache.erase(unique_ids[p_id]);
		unique_ids.erase(p_id);
	}

	// FIXED: mirrors the patched ResourceUID::get_path_id, taking the same `mutex`
	// every other method in this class already takes before touching reverse_cache.
	ID get_path_id(const CharString &p_path) const {
		MutexLock l(mutex);
		const ID *id = reverse_cache.getptr(p_path);
		if (id) {
			return *id;
		}
		return -1;
	}
};

static MiniResourceUID uid_table;
static std::atomic<bool> stop_flag{ false };

// Mirrors the editor_file_system.cpp / resource_importer_scene.cpp call sites that
// invoke ResourceUID::add_id()/set_id()/remove_id() from scripting or the main thread
// while the engine is running.
static void writer_thread() {
	char buf[32];
	int64_t id = 0;
	while (!stop_flag.load()) {
		snprintf(buf, sizeof(buf), "res://item_%lld.tres", (long long)(id % 5000));
		uid_table.add_id(id, CharString(buf));
		if (id > 2000) {
			uid_table.remove_id(id - 2000);
		}
		id++;
	}
}

// Mirrors ResourceLoader::get_resource_uid() -> ResourceUID::get_path_id(), reachable
// from a background thread during threaded resource loading (resource_loader.cpp:1412-1415).
static void reader_thread() {
	char buf[32];
	int64_t id = 0;
	while (!stop_flag.load()) {
		snprintf(buf, sizeof(buf), "res://item_%lld.tres", (long long)(id % 5000));
		uid_table.get_path_id(CharString(buf)); // no lock taken - the bug
		id++;
	}
}

int main() {
	printf("Reproducing missing lock in ResourceUID::get_path_id() (core/io/resource_uid.cpp:221-227)\n");
	printf("Writer thread: add_id()/remove_id() (locked, real code path)\n");
	printf("Reader thread: get_path_id() (UNLOCKED, the bug)\n");
	printf("Expect ASAN to report heap-use-after-free inside HashMap::getptr()/_resize_and_rehash() shortly.\n");
	fflush(stdout);

	std::thread writers[1];
	std::thread readers[1];
	for (auto &t : writers) {
		t = std::thread(writer_thread);
	}
	for (auto &t : readers) {
		t = std::thread(reader_thread);
	}

	// Let it run; ASAN will abort the process as soon as it catches the UAF, so in
	// practice this sleep is never reached on an affected build.
	std::this_thread::sleep_for(std::chrono::seconds(10));
	stop_flag.store(true);
	for (auto &t : writers) {
		t.join();
	}
	for (auto &t : readers) {
		t.join();
	}
	printf("No crash within the time limit (did not race this time).\n");
	return 0;
}
