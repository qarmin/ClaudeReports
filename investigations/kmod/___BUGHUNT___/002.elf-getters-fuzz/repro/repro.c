/*
 * Standalone repro for the heap-buffer-overflow (over-read) in elf_strip_vermagic().
 *
 * It drives kmod_elf_new() + kmod_elf_strip(KMOD_INSERT_FORCE_VERMAGIC) - the exact internal
 * path do_init_module() (insmod/modprobe --force) takes - on a crafted module read into an
 * EXACT-SIZE heap buffer, so ASAN sees the read past the end.
 *
 * A .modinfo section whose bytes are not NUL-terminated and reach the end of the module buffer
 * makes elf_strip_vermagic() call strlen() past the buffer. With a real file this manifests only
 * for a COMPRESSED module (kmod decompresses into a malloc'd, exact-size buffer); run.sh also
 * exercises the real insmod binary on an xz module to show the end-to-end crash. For an
 * uncompressed module the over-read lands in mmap page padding and is usually masked.
 *
 * Exit 0 = bug not present (clean error return), non-zero / ASAN abort = bug reproduced.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "libkmod/libkmod-internal.h"

int main(int argc, char **argv)
{
	const char *path = argc > 1 ? argv[1] : "min_elf.bin";
	FILE *f = fopen(path, "rb");
	if (!f) {
		perror("fopen");
		return 2;
	}
	fseek(f, 0, SEEK_END);
	long n = ftell(f);
	fseek(f, 0, SEEK_SET);
	if (n <= 0)
		return 2;

	/* exact-size heap buffer: mirrors kmod's decompression buffer */
	uint8_t *buf = malloc(n);
	if (!buf || fread(buf, 1, n, f) != (size_t)n)
		return 2;
	fclose(f);

	struct kmod_elf *elf;
	if (kmod_elf_new(buf, n, &elf) < 0) {
		fprintf(stderr, "kmod_elf_new rejected the module\n");
		free(buf);
		return 0;
	}

	const void *stripped = NULL;
	int err = kmod_elf_strip(elf, KMOD_INSERT_FORCE_VERMAGIC, &stripped);
	fprintf(stderr, "kmod_elf_strip returned %d\n", err);
	free((void *)stripped);
	kmod_elf_unref(elf);
	free(buf);
	return 0;
}
