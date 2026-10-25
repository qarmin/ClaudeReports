// Standalone reproduction of the missing lock in Color::find_named_color()
// (core/math/color.cpp:412-431), using Godot's REAL core/templates/hash_map.h
// directly from this checkout.
//
// mini_find_named_color() below mirrors the real function's exact body: same
// static HashMap<..., int>, same is_empty()-guarded lazy-populate loop, same
// ABSENCE of a lock. One deliberate simplification (noted in RESULT.md, same
// as candidate 002's MiniResourceUID): the key type is CharString, not String
// - String::to_upper()/ustring.cpp pulls in Object/OS/Variant/TranslationServer
// to link standalone, which is irrelevant to this bug (a concurrent-mutation
// race on HashMap's internal resize/rehash, identical for either CowData-backed
// string type). The real named_colors table (core/math/color_names.inc) is
// replaced with a small synthetic list for the same reason real color values
// aren't part of the race.
#include "core/os/mutex.h"
#include "core/string/ustring.h"
#include "core/templates/hash_map.h"

#include <atomic>
#include <cstdio>
#include <thread>

static const char *mini_named_colors[] = {
	"ALICEBLUE", "ANTIQUEWHITE", "AQUA", "AQUAMARINE", "AZURE",
	"BEIGE", "BISQUE", "BLACK", "BLANCHEDALMOND", "BLUE",
	"BLUEVIOLET", "BROWN", "BURLYWOOD", "CADETBLUE", "CHARTREUSE",
	"CHOCOLATE", "CORAL", "CORNFLOWERBLUE", "CORNSILK", "CRIMSON",
	"CYAN", "DARKBLUE", "DARKCYAN", "DARKGOLDENROD", "DARKGRAY",
	"DARKGREEN", "DARKKHAKI", "DARKMAGENTA", "DARKOLIVEGREEN", "DARKORANGE",
};
static constexpr int NUM_COLORS = sizeof(mini_named_colors) / sizeof(mini_named_colors[0]);

// FIXED: mirrors the patched Color::find_named_color - a plain static Mutex +
// MutexLock around the same bare static HashMap, same shape as patch.diff.
static int mini_find_named_color(const CharString &p_name) {
	static Mutex named_colors_mutex;
	static HashMap<CharString, int> named_colors_hashmap;
	MutexLock lock(named_colors_mutex);
	if (unlikely(named_colors_hashmap.is_empty())) {
		for (int i = 0; i < NUM_COLORS; i++) {
			named_colors_hashmap[CharString(mini_named_colors[i])] = i;
		}
	}

	const HashMap<CharString, int>::ConstIterator E = named_colors_hashmap.find(p_name);
	if (E) {
		return E->value;
	}
	return -1;
}

static std::atomic<bool> stop_flag{ false };

static void looker_thread() {
	int i = 0;
	while (!stop_flag.load()) {
		const char *name = mini_named_colors[i % NUM_COLORS];
		int idx = mini_find_named_color(CharString(name));
		(void)idx;
		i++;
	}
}

int main() {
	printf("Reproducing missing lock in Color::find_named_color() (core/math/color.cpp:412-431)\n");
	printf("10 threads calling the lazy-init lookup concurrently on first use.\n");
	printf("Expecting ASan to abort shortly with a heap-use-after-free/overflow in HashMap internals.\n");
	fflush(stdout);

	constexpr int NUM_THREADS = 10;
	std::thread threads[NUM_THREADS];
	for (int i = 0; i < NUM_THREADS; i++) {
		threads[i] = std::thread(looker_thread);
	}

	std::this_thread::sleep_for(std::chrono::seconds(15));
	stop_flag.store(true);
	for (auto &t : threads) {
		t.join();
	}
	printf("No crash within the time limit (did not race this time).\n");
	return 0;
}
