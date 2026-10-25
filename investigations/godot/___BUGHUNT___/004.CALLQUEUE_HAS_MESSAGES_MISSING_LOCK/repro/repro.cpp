// Standalone reproduction of the missing lock in CallQueue::has_messages()/
// get_max_buffer_usage() (core/object/message_queue.cpp:445-458), using
// Godot's REAL core/templates/local_vector.h and core/os/mutex.h directly
// from this checkout.
//
// MiniCallQueue below mirrors the real CallQueue's relevant members and
// methods exactly: a Mutex-guarded LocalVector<Page*> "pages" and
// LocalVector<uint32_t> "page_bytes", grown via a locked _add_page()-style
// mutator (push_back, matching the real growth condition at
// message_queue.cpp:59-64), and the real has_messages()/get_max_buffer_usage()
// bodies verbatim - both of which read "pages"/"page_bytes" with NO lock.
#include "core/os/mutex.h"
#include "core/templates/local_vector.h"

#include <atomic>
#include <cstdio>
#include <thread>

struct MiniPage {
	int dummy;
};

class MiniCallQueue {
	mutable Mutex mutex;
	LocalVector<MiniPage *> pages;
	LocalVector<uint32_t> page_bytes;
	uint32_t pages_used = 0;

	void _add_page() {
		if (pages_used == page_bytes.size()) {
			pages.push_back(memnew(MiniPage));
			page_bytes.push_back(0);
		}
		page_bytes[pages_used] = 0;
		pages_used++;
	}

public:
	// Mirrors the real push_callp()-style mutator: locked, grows pages/page_bytes.
	// LocalVector::reserve() grows geometrically (1.5x), so after the first dozen
	// calls no further memrealloc() happens at all - to keep demonstrating the
	// race for the whole run (not just the first few calls), periodically reset
	// back to empty under the lock, forcing a fresh alloc/free/grow cycle every
	// time, widening the window a concurrent unlocked reader could land in.
	void add_entry() {
		MutexLock lock(mutex);
		if (pages_used > 20) {
			for (uint32_t i = 0; i < pages_used; i++) {
				memdelete(pages[i]);
			}
			pages.reset();
			page_bytes.reset();
			pages_used = 0;
		}
		if (pages_used == 0 || page_bytes[pages_used - 1] > 0) {
			_add_page();
		}
		page_bytes[pages_used - 1] += 1;
	}

	// Mirrors CallQueue::has_messages() (message_queue.cpp:445-453): no lock at all.
	bool has_messages() const {
		if (pages_used == 0) {
			return false;
		}
		if (pages_used == 1 && page_bytes[0] == 0) {
			return false;
		}
		return true;
	}

	// Mirrors CallQueue::get_max_buffer_usage() (message_queue.cpp:456-458): no lock.
	int get_max_buffer_usage() const {
		return pages.size() * 4096;
	}
};

static MiniCallQueue queue;
static std::atomic<bool> stop_flag{ false };

static void writer_thread() {
	while (!stop_flag.load()) {
		queue.add_entry();
	}
}

static void reader_thread() {
	while (!stop_flag.load()) {
		volatile bool b = queue.has_messages();
		volatile int u = queue.get_max_buffer_usage();
		(void)b;
		(void)u;
	}
}

int main() {
	printf("Reproducing missing lock in CallQueue::has_messages()/get_max_buffer_usage()\n");
	printf("(core/object/message_queue.cpp:445-458). 5 writers growing pages/page_bytes,\n");
	printf("5 readers calling the unlocked accessors concurrently.\n");
	fflush(stdout);

	constexpr int NUM_WRITERS = 12;
	constexpr int NUM_READERS = 12;
	std::thread writers[NUM_WRITERS];
	std::thread readers[NUM_READERS];
	for (auto &t : writers) {
		t = std::thread(writer_thread);
	}
	for (auto &t : readers) {
		t = std::thread(reader_thread);
	}

	std::this_thread::sleep_for(std::chrono::seconds(15));
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
