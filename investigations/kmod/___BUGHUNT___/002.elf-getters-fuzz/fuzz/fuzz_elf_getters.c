// libFuzzer harness: every libkmod ELF getter reachable from kmod_module_get_info / _get_versions /
// _get_symbols / _get_dependency_symbols / insmod --force, plus the module signature trailer parser.
// Every returned string is strlen()'d and every returned buffer is read in full so that ASAN catches
// over-reads past the (exact-size) input buffer.
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "libkmod/libkmod-internal.h"

static const void *cur_data;
static off_t cur_size;

// Overrides libkmod-file.c's version (harness object is linked first, --allow-multiple-definition).
int kmod_file_get_contents(const struct kmod_file *file, const void **contents, off_t *size)
{
	(void)file;
	*contents = cur_data;
	*size = cur_size;
	return 0;
}

static volatile size_t sink;

static void touch(const void *p, size_t n)
{
	const volatile uint8_t *b = p;
	size_t acc = 0;
	for (size_t i = 0; i < n; i++)
		acc += b[i];
	sink += acc;
}

static uint64_t rd(const uint8_t *d, size_t off, size_t n, int msb)
{
	uint64_t v = 0;
	for (size_t i = 0; i < n; i++)
		v |= (uint64_t)d[off + i] << (8 * (msb ? n - 1 - i : i));
	return v;
}

// Known bug 001 (unchecked e_shstrndx aborts in kmod_elf_new): skip such inputs to get deeper.
static int shstrndx_ok(const uint8_t *d, size_t s)
{
	int msb, x32;
	uint64_t shnum, shstrndx;

	if (s < 6 || memcmp(d, "\177ELF", 4) != 0)
		return 1;
	x32 = d[4] == 1;
	if (s <= (x32 ? 52u : 64u))
		return 1;
	msb = d[5] == 2;
	shnum = rd(d, x32 ? 48 : 60, 2, msb);
	shstrndx = rd(d, x32 ? 50 : 62, 2, msb);
	return shstrndx != 0 && shstrndx < shnum;
}

static void use_versions(struct kmod_modversion *v, int n)
{
	for (int i = 0; i < n; i++)
		sink += strlen(v[i].symbol) + v[i].crc + v[i].bind;
	free(v);
}

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	struct kmod_elf *elf;
	struct kmod_modversion *v;
	struct kmod_signature_info *si;
	uint64_t off, size;
	const void *p;
	size_t len;
	char **arr;
	int n;

	cur_data = d;
	cur_size = s;
	if (kmod_module_signature_info((const struct kmod_file *)1, &si)) {
		if (si->signer)
			touch(si->signer, si->signer_len);
		if (si->key_id)
			touch(si->key_id, si->key_id_len);
		if (si->sig)
			touch(si->sig, si->sig_len);
		sink += strlen(si->hash_algo) + strlen(si->id_type);
		free(si);
	}

	if (!shstrndx_ok(d, s))
		return 0;
	if (kmod_elf_new(d, s, &elf) < 0)
		return 0;

	if (kmod_elf_get_section(elf, ".modinfo", &off, &size) >= 0)
		touch(d + off, size);
	n = kmod_elf_get_modinfo_strings(elf, &arr);
	if (n > 0) {
		for (int i = 0; i < n; i++)
			sink += strlen(arr[i]);
		free(arr);
	}
	if (kmod_elf_get_build_id(elf, &p, &len) == 0)
		touch(p, len);
	n = kmod_elf_get_modversions(elf, &v);
	if (n > 0)
		use_versions(v, n);
	n = kmod_elf_get_symbols(elf, &v);
	if (n > 0)
		use_versions(v, n);
	n = kmod_elf_get_dependency_symbols(elf, &v);
	if (n > 0)
		use_versions(v, n);
	if (kmod_elf_strip(elf, KMOD_INSERT_FORCE_MODVERSION | KMOD_INSERT_FORCE_VERMAGIC, &p) == 0)
		free((void *)p);

	kmod_elf_unref(elf);
	return 0;
}
