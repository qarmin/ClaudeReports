// libFuzzer harness: kmod_elf_new() header validation (no pre-filter), plus a section lookup and
// the modinfo-strings getter on whatever kmod_elf_new() accepts.
#include <stdint.h>
#include <stdlib.h>

#include "libkmod/libkmod-internal.h"

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	struct kmod_elf *elf;
	uint64_t off, size;
	char **arr;

	if (kmod_elf_new(d, s, &elf) < 0)
		return 0;
	(void)kmod_elf_get_section(elf, ".modinfo", &off, &size);
	if (kmod_elf_get_modinfo_strings(elf, &arr) > 0)
		free(arr);
	kmod_elf_unref(elf);
	return 0;
}
