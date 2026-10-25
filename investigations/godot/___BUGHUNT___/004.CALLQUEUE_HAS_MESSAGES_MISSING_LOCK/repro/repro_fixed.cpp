// Same as repro.cpp, but has_messages()/get_max_buffer_usage() now take the
// mutex too, mirroring the real fix applied to
// core/object/message_queue.cpp (see ../patch.diff).
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

	// Mirrors the fixed CallQueue::has_messages(): now locked.
	bool has_messages() const {
		MutexLock lock(mutex);
		if (pages_used == 0) {
			return false;
		}
		if (pages_used == 1 && page_bytes[0] == 0) {
			return false;
		}
		return true;
	}

	// Mirrors the fixed CallQueue::get_max_buffer_usage(): now locked.
	int get_max_buffer_usage() const {
		MutexLock lock(mutex);
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
	printf("Verifying the fix for CallQueue::has_messages()/get_max_buffer_usage()\n");
	printf("12 writers growing pages/page_bytes, 12 readers calling the now-locked\n");
	printf("accessors concurrently.\n");
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
	printf("No crash within the time limit.\n");
	return 0;
}
