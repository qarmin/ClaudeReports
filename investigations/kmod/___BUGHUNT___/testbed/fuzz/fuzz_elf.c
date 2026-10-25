/* In-memory ELF parsing: every kmod_elf getter + kmod_elf_strip, with a strip invariant. */
#include "common.h"
#include <inttypes.h>

#include "libkmod/libkmod-internal.h"

static uint64_t rd(const uint8_t *p, int n, int le)
{
	uint64_t v = 0;
	for (int i = 0; i < n; i++)
		v |= (uint64_t)p[le ? i : n - 1 - i] << (8 * i);
	return v;
}

static void shdr_table_range(const uint8_t *d, size_t s, uint64_t *lo, uint64_t *hi)
{
	int is64 = d[4] == 2, le = d[5] == 1;
	*lo = rd(d + (is64 ? 0x28 : 0x20), is64 ? 8 : 4, le);
	uint64_t num = rd(d + (is64 ? 0x3c : 0x30), 2, le);
	uint64_t esz = rd(d + (is64 ? 0x3a : 0x2e), 2, le);
	*hi = *lo + num * esz;
}

static void exercise(const uint8_t *d, size_t s, int depth)
{
	struct kmod_elf *elf;
	if (kmod_elf_new(d, s, &elf) < 0)
		return;

	char **strs;
	int n = kmod_elf_get_modinfo_strings(elf, &strs);
	if (n > 0) {
		for (int i = 0; i < n; i++)
			(void)strlen(strs[i]);
		free(strs);
	}

	struct kmod_modversion *mv;
	int nv = kmod_elf_get_modversions(elf, &mv);
	if (nv > 0) {
		for (int i = 0; i < nv; i++)
			(void)strlen(mv[i].symbol);
		free(mv);
	}
	n = kmod_elf_get_symbols(elf, &mv);
	if (n > 0) {
		for (int i = 0; i < n; i++)
			(void)strlen(mv[i].symbol);
		free(mv);
	}
	n = kmod_elf_get_dependency_symbols(elf, &mv);
	if (n > 0) {
		for (int i = 0; i < n; i++)
			(void)strlen(mv[i].symbol);
		free(mv);
	}

	const void *hash;
	size_t hash_len;
	if (kmod_elf_get_build_id(elf, &hash, &hash_len) == 0 && hash_len > 0) {
		volatile uint8_t sink = 0;
		for (size_t i = 0; i < hash_len; i++)
			sink ^= ((const uint8_t *)hash)[i];
	}

	uint64_t off, size;
	(void)kmod_elf_get_section(elf, ".gnu.linkonce.this_module", &off, &size);

	if (depth == 0) {
		const void *stripped;
		if (kmod_elf_strip(elf, KMOD_INSERT_FORCE_MODVERSION | KMOD_INSERT_FORCE_VERMAGIC,
				   &stripped) == 0) {
			/*
			 * Invariant: stripping only clears SHF_ALLOC in the section header table and
			 * blanks vermagic inside .modinfo - no byte outside those ranges may change.
			 */
			uint64_t mi_off = 0, mi_size = 0;
			(void)kmod_elf_get_section(elf, ".modinfo", &mi_off, &mi_size);
			uint64_t sh_lo, sh_hi;
			shdr_table_range(d, s, &sh_lo, &sh_hi);
			for (size_t i = 0; i < s; i++) {
				if (((const uint8_t *)stripped)[i] == d[i])
					continue;
				if ((i >= mi_off && i < mi_off + mi_size) || (i >= sh_lo && i < sh_hi))
					continue;
				fprintf(stderr, "INVARIANT strip-wrote-outside: byte %zu (modinfo %" PRIu64
						"+%" PRIu64 ", shdrs %" PRIu64 "-%" PRIu64 ")\n",
					i, mi_off, mi_size, sh_lo, sh_hi);
				abort();
			}
			exercise(stripped, s, depth + 1);
			free((void *)stripped);
		}
	}
	kmod_elf_unref(elf);
}

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	if (elf_hits_known_shstrndx_assert(d, s))
		return 0;
	/* kmod_elf_new() keeps a pointer to the buffer; give it an exact-size heap copy so ASAN sees overreads. */
	uint8_t *copy = malloc(s ? s : 1);
	memcpy(copy, d, s);
	exercise(copy, s, 0);
	free(copy);
	return 0;
}
