/*
 * Real entry point: a module file on disk (raw, xz, zstd or gzip) opened through the public API,
 * like modinfo/modprobe do - decompression, ELF, signature parsing, info/versions/symbols.
 */
#include "common.h"

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	static struct kmod_ctx *ctx;
	static char path[300];
	if (!ctx) {
		ctx = quiet_ctx("/nonexistent-kmod-fuzz-moddir", NULL);
		snprintf(path, sizeof(path), "%s/fuzzmod.ko", scratch_dir());
	}
	if (elf_hits_known_shstrndx_assert(d, s))
		return 0;
	if (write_file(path, d, s) < 0)
		return 0;

	struct kmod_module *mod;
	if (kmod_module_new_from_path(ctx, path, &mod) < 0)
		return 0;

	struct kmod_list *l = NULL, *it;
	l = NULL;
	if (kmod_module_get_info(mod, &l) >= 0) {
		kmod_list_foreach(it, l) {
			(void)strlen(kmod_module_info_get_key(it));
			const char *v = kmod_module_info_get_value(it);
			if (v)
				(void)strlen(v);
		}
		kmod_module_info_free_list(l);
	}
	l = NULL;
	if (kmod_module_get_versions(mod, &l) >= 0)
		kmod_module_versions_free_list(l);
	l = NULL;
	if (kmod_module_get_symbols(mod, &l) >= 0)
		kmod_module_symbols_free_list(l);
	l = NULL;
	if (kmod_module_get_dependency_symbols(mod, &l) >= 0)
		kmod_module_dependency_symbols_free_list(l);
	kmod_module_unref(mod);
	return 0;
}
