// libFuzzer harness for both kmod index readers (libkmod/libkmod-index.c).
// Input: [u8 keylen][key bytes][modules.*.bin bytes]
// The mm reader is driven over an exact-size heap copy (not mmap) so ASAN sees every byte read
// past the end of the "file"; the stdio reader is driven over fmemopen() of the same bytes.
#include "libkmod/libkmod-index.c"

#include <stdint.h>
#include <stdio.h>

static int devnull = -1;

static void run_mm(const uint8_t *data, size_t size, const char *key)
{
	struct index_mm idx;
	struct index_value *vals;
	const void *p;
	void *copy;

	copy = malloc(size);
	if (!copy)
		return;
	memcpy(copy, data, size);
	p = copy;
	if (read_u32_mm(&p) != INDEX_MAGIC || read_u32_mm(&p) >> 16 != INDEX_VERSION_MAJOR)
		goto out;
	idx.ctx = NULL;
	idx.mm = copy;
	idx.root_offset = read_u32_mm(&p);
	idx.size = size;

	free(index_mm_search(&idx, key));
	vals = index_mm_searchwild(&idx, key);
	index_values_free(vals);
	index_mm_dump(&idx, devnull, true);
out:
	free(copy);
}

static void run_stdio(const uint8_t *data, size_t size, const char *key)
{
	struct index_file *idx;
	struct index_value *vals;
	uint32_t hdr[3];
	FILE *f;

	f = fmemopen((void *)data, size, "r");
	if (!f)
		return;
	if (!read_u32s(f, hdr, 3) || hdr[0] != INDEX_MAGIC || hdr[1] >> 16 != INDEX_VERSION_MAJOR) {
		fclose(f);
		return;
	}
	idx = calloc(1, sizeof(*idx));
	idx->file = f;
	idx->root_offset = hdr[2];

	free(index_search(idx, key));
	vals = index_searchwild(idx, key);
	index_values_free(vals);
	index_dump(idx, devnull, true);
	index_file_close(idx);
}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
	char key[256];
	size_t klen;

	if (devnull < 0)
		devnull = open("/dev/null", O_WRONLY | O_CLOEXEC);
	if (size < 1)
		return 0;
	klen = data[0];
	if (size < 1 + klen + 12)
		return 0;
	memcpy(key, data + 1, klen);
	key[klen] = '\0';
	data += 1 + klen;
	size -= 1 + klen;

	run_mm(data, size, key);
	run_stdio(data, size, key);
	return 0;
}
