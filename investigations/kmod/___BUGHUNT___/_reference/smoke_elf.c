#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include "libkmod/libkmod-internal.h"
int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	struct kmod_elf *elf;
	if (kmod_elf_new(d, s, &elf) < 0)
		return 0;
	char **arr;
	int n = kmod_elf_get_modinfo_strings(elf, &arr);
	if (n > 0)
		free(arr);
	kmod_elf_unref(elf);
	return 0;
}
