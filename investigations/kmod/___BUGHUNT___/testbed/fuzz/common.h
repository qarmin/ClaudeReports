/* Shared helpers for kmod libFuzzer targets. */
#pragma once
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "libkmod/libkmod.h"

static void quiet_log(void *data, int priority, const char *file, int line, const char *fn,
		      const char *format, va_list args)
{
}

/* Per-process scratch dir: libFuzzer may run several jobs in parallel. */
static const char *scratch_dir(void)
{
	static char dir[256];
	if (!dir[0]) {
		const char *base = getenv("TMPDIR") ? getenv("TMPDIR") : "/tmp";
		snprintf(dir, sizeof(dir), "%s/kmodfuzz.%d", base, getpid());
		mkdir(dir, 0700);
	}
	return dir;
}

static int write_file(const char *path, const uint8_t *data, size_t size)
{
	int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
	if (fd < 0)
		return -1;
	size_t off = 0;
	while (off < size) {
		ssize_t w = write(fd, data + off, size - off);
		if (w <= 0) {
			close(fd);
			return -1;
		}
		off += (size_t)w;
	}
	close(fd);
	return 0;
}

static struct kmod_ctx *quiet_ctx(const char *dirname, const char *const *config_paths)
{
	static const char *const no_cfg[] = { "/nonexistent-kmod-fuzz-config", NULL };
	struct kmod_ctx *ctx = kmod_new(dirname, config_paths ? config_paths : no_cfg);
	if (ctx)
		kmod_set_log_fn(ctx, quiet_log, NULL);
	return ctx;
}

/*
 * Known bug 001 (elf-shstrndx-assert): kmod_elf_new() aborts on an out-of-range e_shstrndx.
 * Skip such inputs so fuzzing reaches deeper code; set KMOD_FUZZ_NO_FILTER=1 to disable.
 */
static int elf_hits_known_shstrndx_assert(const uint8_t *d, size_t s)
{
	if (getenv("KMOD_FUZZ_NO_FILTER"))
		return 0;
	if (s < 6 || memcmp(d, "\x7f" "ELF", 4) != 0)
		return 0;
	int is64 = d[4] == 2, le = d[5] == 1;
	if (s < (is64 ? 0x40u : 0x34u))
		return 0;
#define RD16(o) (le ? (uint16_t)(d[o] | d[(o) + 1] << 8) : (uint16_t)(d[o] << 8 | d[(o) + 1]))
	uint16_t shnum = is64 ? RD16(0x3c) : RD16(0x30);
	uint16_t shstrndx = is64 ? RD16(0x3e) : RD16(0x32);
#undef RD16
	return shstrndx == 0 || shstrndx >= shnum;
}
