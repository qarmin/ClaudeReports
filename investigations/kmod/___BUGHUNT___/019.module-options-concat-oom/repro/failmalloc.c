// LD_PRELOAD allocation-failure injector: FAILMALLOC_N=k fails the k-th malloc/calloc/realloc/strdup/strndup (1-based);
// FAILMALLOC_COUNT=1 prints the total number of calls to stderr at exit.
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void *(*real_malloc)(size_t);
static void *(*real_calloc)(size_t, size_t);
static void *(*real_realloc)(void *, size_t);
static unsigned long calls, fail_at;
static int count_mode, resolving;
static char boot[4096];

static void init(void)
{
	if (real_malloc || resolving)
		return;
	resolving = 1;
	real_malloc = dlsym(RTLD_NEXT, "malloc");
	real_calloc = dlsym(RTLD_NEXT, "calloc");
	real_realloc = dlsym(RTLD_NEXT, "realloc");
	resolving = 0;
	if (getenv("FAILMALLOC_N"))
		fail_at = strtoul(getenv("FAILMALLOC_N"), NULL, 10);
	count_mode = getenv("FAILMALLOC_COUNT") != NULL;
}

static int should_fail(void)
{
	return fail_at && ++calls == fail_at;
}

void *malloc(size_t n)
{
	init();
	if (!real_malloc)
		return boot;
	if (should_fail() || (!fail_at && count_mode && ++calls && 0)) {
		errno = ENOMEM;
		return NULL;
	}
	if (!fail_at)
		calls++;
	return real_malloc(n);
}

void *calloc(size_t a, size_t b)
{
	if (resolving)
		return memset(boot, 0, sizeof(boot)), boot;
	init();
	if (should_fail()) {
		errno = ENOMEM;
		return NULL;
	}
	if (!fail_at)
		calls++;
	return real_calloc(a, b);
}

void *realloc(void *p, size_t n)
{
	init();
	if (should_fail()) {
		errno = ENOMEM;
		return NULL;
	}
	if (!fail_at)
		calls++;
	return real_realloc(p, n);
}

char *strdup(const char *s)
{
	size_t n = strlen(s) + 1;
	char *r = malloc(n);

	return r ? memcpy(r, s, n) : NULL;
}

char *strndup(const char *s, size_t max)
{
	size_t n = strnlen(s, max);
	char *r = malloc(n + 1);

	if (!r)
		return NULL;
	memcpy(r, s, n);
	r[n] = '\0';
	return r;
}

__attribute__((destructor)) static void report(void)
{
	if (count_mode)
		fprintf(stderr, "FAILMALLOC_TOTAL=%lu\n", calls);
}
