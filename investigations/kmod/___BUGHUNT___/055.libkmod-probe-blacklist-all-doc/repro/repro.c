/* m depends on d, d is blacklisted. KMOD_PROBE_APPLY_BLACKLIST_ALL is documented as: "prior to probe, apply KMOD_FILTER_BLACKLIST
 * filter to this module and its dependencies. If any of them are blacklisted and the blacklisted module is not live in the kernel,
 * the function returns early with this enum". */
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <libkmod.h>

static int printed;
static void show(struct kmod_module *m, bool install, const char *options)
{
	printf("  action: %s\n", kmod_module_get_name(m));
	printed++;
}

int main(int argc, char **argv)
{
	const char *cfg[] = { argv[2], NULL };
	struct kmod_ctx *ctx = kmod_new(argv[1], cfg);
	struct kmod_module *mod;
	int r;

	if (!ctx || kmod_module_new_from_name(ctx, "m", &mod) < 0)
		return 2;
	kmod_load_resources(ctx);
	r = kmod_module_probe_insert_module(mod, KMOD_PROBE_DRY_RUN | KMOD_PROBE_IGNORE_LOADED | KMOD_PROBE_APPLY_BLACKLIST_ALL,
					    NULL, NULL, NULL, show);
	printf("return value %d (KMOD_PROBE_APPLY_BLACKLIST_ALL is %d), %d action(s) taken\n", r, KMOD_PROBE_APPLY_BLACKLIST_ALL, printed);
	return r == KMOD_PROBE_APPLY_BLACKLIST_ALL ? 0 : 1;
}
