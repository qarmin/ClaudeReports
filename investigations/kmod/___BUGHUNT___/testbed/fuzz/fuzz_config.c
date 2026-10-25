/*
 * modprobe.d parsing + kernel command line parsing, then everything a consumer reads back from
 * the config: public iterators, per-module options/softdeps/weakdeps/install commands, and a
 * dry-run probe of modules with install commands ($CMDLINE_OPTS substitution).
 * Input layout: [cmdline_len:1][cmdline bytes][config file bytes]. /proc/cmdline is redirected
 * to the cmdline bytes via -Wl,--wrap=open.
 */
#include <stdarg.h>

#include "common.h"

static char cmdline_path[300];
int __real_open(const char *path, int flags, ...);
int __wrap_open(const char *path, int flags, ...)
{
	mode_t mode = 0;
	if (flags & O_CREAT) {
		va_list ap;
		va_start(ap, flags);
		mode = va_arg(ap, mode_t);
		va_end(ap);
	}
	if (cmdline_path[0] && strcmp(path, "/proc/cmdline") == 0)
		path = cmdline_path;
	return __real_open(path, flags, mode);
}

static int noop_install(struct kmod_module *m, const char *cmdline, void *data)
{
	(void)strlen(cmdline);
	return 0;
}

static void drain(struct kmod_config_iter *it)
{
	if (!it)
		return;
	while (kmod_config_iter_next(it)) {
		(void)strlen(kmod_config_iter_get_key(it));
		const char *v = kmod_config_iter_get_value(it);
		if (v)
			(void)strlen(v);
	}
	kmod_config_iter_free_iter(it);
}

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	static char cfgdir[300], cfgfile[320];
	if (!cfgdir[0]) {
		snprintf(cfgdir, sizeof(cfgdir), "%s/modprobe.d", scratch_dir());
		mkdir(cfgdir, 0700);
		snprintf(cfgfile, sizeof(cfgfile), "%s/fuzz.conf", cfgdir);
		snprintf(cmdline_path, sizeof(cmdline_path), "%s/cmdline", scratch_dir());
	}
	if (s < 1 || s < 1u + d[0])
		return 0;
	if (write_file(cmdline_path, d + 1, d[0]) < 0 ||
	    write_file(cfgfile, d + 1 + d[0], s - 1 - d[0]) < 0)
		return 0;

	const char *paths[] = { cfgdir, NULL };
	struct kmod_ctx *ctx = quiet_ctx("/nonexistent-kmod-fuzz-moddir", paths);
	if (!ctx)
		return 0;

	drain(kmod_config_get_blacklists(ctx));
	drain(kmod_config_get_masks(ctx));
	drain(kmod_config_get_install_commands(ctx));
	drain(kmod_config_get_remove_commands(ctx));
	drain(kmod_config_get_aliases(ctx));
	drain(kmod_config_get_options(ctx));
	drain(kmod_config_get_softdeps(ctx));
	drain(kmod_config_get_weakdeps(ctx));

	/* Per-module views for every name the config mentions in options/install/softdep. */
	struct kmod_config_iter *iters[] = { kmod_config_get_options(ctx),
					     kmod_config_get_install_commands(ctx),
					     kmod_config_get_softdeps(ctx) };
	int budget = 32;
	for (unsigned i = 0; i < 3; i++) {
		struct kmod_config_iter *it = iters[i];
		if (!it)
			continue;
		while (budget-- > 0 && kmod_config_iter_next(it)) {
			struct kmod_module *mod;
			if (kmod_module_new_from_name(ctx, kmod_config_iter_get_key(it), &mod) < 0)
				continue;
			(void)kmod_module_get_options(mod);
			(void)kmod_module_get_install_commands(mod);
			struct kmod_list *pre = NULL, *post = NULL, *weak = NULL;
			if (kmod_module_get_softdeps(mod, &pre, &post) == 0) {
				kmod_module_unref_list(pre);
				kmod_module_unref_list(post);
			}
			if (kmod_module_get_weakdeps(mod, &weak) == 0)
				kmod_module_unref_list(weak);
			if (i == 1)
				kmod_module_probe_insert_module(mod, KMOD_PROBE_DRY_RUN | KMOD_PROBE_IGNORE_LOADED,
								"extra=1", noop_install, NULL, NULL);
			kmod_module_unref(mod);
		}
		kmod_config_iter_free_iter(it);
	}
	kmod_unref(ctx);
	return 0;
}
