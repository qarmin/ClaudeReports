/*
 * modules.builtin.modinfo (kernel-provided, read by modinfo/modprobe for built-in modules).
 * Input layout: [name_len:1][module name][modules.builtin.modinfo bytes].
 */
#include "common.h"

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	static char dir[300], file[340];
	if (!dir[0]) {
		snprintf(dir, sizeof(dir), "%s/moddir", scratch_dir());
		mkdir(dir, 0700);
		snprintf(file, sizeof(file), "%s/modules.builtin.modinfo", dir);
	}
	if (s < 1)
		return 0;
	size_t nlen = d[0] % 32;
	if (s < 1 + nlen)
		return 0;
	char name[33];
	memcpy(name, d + 1, nlen);
	name[nlen] = '\0';
	if (nlen == 0 || strlen(name) != nlen)
		strcpy(name, "ext4");
	if (write_file(file, d + 1 + nlen, s - 1 - nlen) < 0)
		return 0;

	struct kmod_ctx *ctx = quiet_ctx(dir, NULL);
	if (!ctx)
		return 0;
	struct kmod_module *mod;
	if (kmod_module_new_from_name(ctx, name, &mod) == 0) {
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
		kmod_module_unref(mod);
	}
	struct kmod_list *list = NULL;
	if (kmod_module_new_from_lookup(ctx, name, &list) == 0)
		kmod_module_unref_list(list);
	kmod_unref(ctx);
	return 0;
}
