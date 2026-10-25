/*
 * libkmod public API as a state machine: random sequences of lookups, dependency/softdep/weakdep queries,
 * info/versions/symbols getters, dry-run probes with callbacks, filters, resource reloads and out-of-order
 * unrefs, over a fixed synthetic module tree (make_api_tree.sh, path in $KMOD_API_TREE) and a fuzzed modprobe.d file.
 * Input layout: [cfg_len:2][config bytes][op bytes].
 */
#include "common.h"

#define NMOD 8
#define NLIST 4

static const char *const names[] = { "m0", "m1", "m2", "m3", "m4", "m5", "m6", "m7", "a0", "a1", "pci:v1d3",
				     "m-1", "builtin0", "nonexistent", "", "intern" };

static int show_cb(struct kmod_module *m, bool install, const char *options)
{
	(void)kmod_module_get_name(m);
	if (options)
		(void)strlen(options);
	return 0;
}

static int install_cb(struct kmod_module *m, const char *cmd, void *data)
{
	(void)strlen(cmd);
	return 0;
}

static void show_wrap(struct kmod_module *m, bool install, const char *options)
{
	show_cb(m, install, options);
}

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	static char cfgdir[300], cfgfile[320];
	const char *tree = getenv("KMOD_API_TREE");
	if (!tree || s < 2)
		return 0;
	if (!cfgdir[0]) {
		snprintf(cfgdir, sizeof(cfgdir), "%s/modprobe.d", scratch_dir());
		mkdir(cfgdir, 0700);
		snprintf(cfgfile, sizeof(cfgfile), "%s/fuzz.conf", cfgdir);
	}
	size_t cl = (d[0] | d[1] << 8) % (s - 1);
	if (write_file(cfgfile, d + 2, cl) < 0)
		return 0;
	const uint8_t *ops = d + 2 + cl;
	size_t nops = s - 2 - cl;

	char moddir[400];
	snprintf(moddir, sizeof(moddir), "%s/lib/modules/1.0", tree);
	const char *paths[] = { cfgdir, NULL };
	struct kmod_ctx *ctx = quiet_ctx(moddir, paths);
	if (!ctx)
		return 0;
	kmod_load_resources(ctx);

	struct kmod_module *mods[NMOD] = { 0 };
	struct kmod_list *lists[NLIST] = { 0 };
	int nmods = 0;

	for (size_t i = 0; i + 1 < nops; i += 2) {
		uint8_t op = ops[i] % 20, a = ops[i + 1];
		struct kmod_module *m = mods[a % NMOD];
		struct kmod_list **lp = &lists[a % NLIST];
		const char *name = names[a % (sizeof(names) / sizeof(names[0]))];
		switch (op) {
		case 0: { /* lookup into a list, remember the first module */
			struct kmod_list *l = NULL;
			if (kmod_module_new_from_lookup(ctx, name, &l) >= 0 && l) {
				kmod_module_unref_list(*lp);
				*lp = l;
				struct kmod_module *f = kmod_module_get_module(l);
				if (mods[nmods % NMOD])
					kmod_module_unref(mods[nmods % NMOD]);
				mods[nmods++ % NMOD] = f;
			}
			break;
		}
		case 1: {
			struct kmod_module *n = NULL;
			if (kmod_module_new_from_name(ctx, name, &n) >= 0) {
				if (mods[nmods % NMOD])
					kmod_module_unref(mods[nmods % NMOD]);
				mods[nmods++ % NMOD] = n;
			}
			break;
		}
		case 2:
			if (m) {
				kmod_module_unref_list(*lp);
				*lp = kmod_module_get_dependencies(m);
			}
			break;
		case 3:
			if (m) {
				struct kmod_list *pre = NULL, *post = NULL;
				if (kmod_module_get_softdeps(m, &pre, &post) >= 0) {
					kmod_module_unref_list(*lp);
					*lp = pre;
					kmod_module_unref_list(lists[(a + 1) % NLIST]);
					lists[(a + 1) % NLIST] = post;
				}
			}
			break;
		case 4:
			if (m) {
				struct kmod_list *w = NULL;
				if (kmod_module_get_weakdeps(m, &w) >= 0) {
					kmod_module_unref_list(*lp);
					*lp = w;
				}
			}
			break;
		case 5:
			if (m) {
				struct kmod_list *l = NULL, *it;
				if (kmod_module_get_info(m, &l) >= 0) {
					kmod_list_foreach(it, l) {
						(void)strlen(kmod_module_info_get_key(it));
						(void)strlen(kmod_module_info_get_value(it));
					}
					kmod_module_info_free_list(l);
				}
			}
			break;
		case 6:
			if (m) {
				struct kmod_list *l = NULL;
				if (kmod_module_get_versions(m, &l) >= 0)
					kmod_module_versions_free_list(l);
				l = NULL;
				if (kmod_module_get_symbols(m, &l) >= 0)
					kmod_module_symbols_free_list(l);
				l = NULL;
				if (kmod_module_get_dependency_symbols(m, &l) >= 0)
					kmod_module_dependency_symbols_free_list(l);
			}
			break;
		case 7:
			if (m) {
				unsigned f = (ops[i + 1] * 0x9e3779b1u) >> 8;
				unsigned flags = KMOD_PROBE_DRY_RUN | (f & 0x2f) |
						 ((f & 0x40) ? KMOD_PROBE_APPLY_BLACKLIST : 0) |
						 ((f & 0x80) ? KMOD_PROBE_APPLY_BLACKLIST_ALL : 0) |
						 ((f & 0x100) ? KMOD_PROBE_APPLY_BLACKLIST_ALIAS_ONLY : 0);
				kmod_module_probe_insert_module(m, flags, (f & 0x200) ? "x=1 y=\"a b\"" : NULL,
								install_cb, NULL, (f & 0x400) ? show_wrap : NULL);
			}
			break;
		case 8: {
			int k = a % NMOD;
			if (mods[k]) {
				kmod_module_unref(mods[k]);
				mods[k] = NULL;
			}
			break;
		}
		case 9:
			kmod_module_unref_list(*lp);
			*lp = NULL;
			break;
		case 10:
			switch (a % 4) {
			case 0: kmod_unload_resources(ctx); break;
			case 1: kmod_load_resources(ctx); break;
			case 2: (void)kmod_validate_resources(ctx); break;
			default: kmod_unload_resources(ctx); kmod_load_resources(ctx);
			}
			break;
		case 11:
			if (m) {
				(void)kmod_module_get_initstate(m);
				(void)kmod_module_get_refcnt(m);
				(void)kmod_module_get_size(m);
				struct kmod_list *h = kmod_module_get_holders(m);
				kmod_module_unref_list(h);
				struct kmod_list *sc = kmod_module_get_sections(m);
				kmod_module_section_free_list(sc);
			}
			break;
		case 12:
			if (m) {
				const char *o = kmod_module_get_options(m);
				const char *c = kmod_module_get_install_commands(m);
				const char *r = kmod_module_get_remove_commands(m);
				if (o) (void)strlen(o);
				if (c) (void)strlen(c);
				if (r) (void)strlen(r);
			}
			break;
		case 13:
			if (*lp) {
				struct kmod_list *out = NULL;
				if (kmod_module_apply_filter(ctx, (a & 1) ? KMOD_FILTER_BLACKLIST : KMOD_FILTER_BUILTIN,
							     *lp, &out) >= 0)
					kmod_module_unref_list(out);
			}
			break;
		case 14:
			if (m) {
				(void)kmod_module_get_name(m);
				(void)kmod_module_get_path(m);
				(void)kmod_module_get_initstate(m);
			}
			break;
		case 15: {
			struct kmod_config_iter *(*gs[])(const struct kmod_ctx *) = {
				kmod_config_get_blacklists, kmod_config_get_install_commands, kmod_config_get_masks,
				kmod_config_get_remove_commands, kmod_config_get_aliases, kmod_config_get_options,
				kmod_config_get_softdeps, kmod_config_get_weakdeps };
			struct kmod_config_iter *it = gs[a % 8](ctx);
			while (it && kmod_config_iter_next(it)) {
				(void)strlen(kmod_config_iter_get_key(it));
				const char *v = kmod_config_iter_get_value(it);
				if (v) (void)strlen(v);
			}
			kmod_config_iter_free_iter(it);
			break;
		}
		case 16:
			if (m) {
				kmod_module_ref(m);
				kmod_module_unref(m);
				kmod_module_unref(kmod_module_ref(m));
			}
			break;
		case 17:
			if (*lp) {
				struct kmod_module *f = kmod_module_get_module(*lp);
				kmod_module_unref(f);
			}
			break;
		case 18: { /* module from a file path in the tree */
			char p[500];
			struct kmod_module *n = NULL;
			snprintf(p, sizeof(p), "%s/kernel/%sm%d.ko", moddir, (a % 3 == 0) ? "sub/" : "", a % 8);
			if (kmod_module_new_from_path(ctx, p, &n) >= 0) {
				if (mods[nmods % NMOD])
					kmod_module_unref(mods[nmods % NMOD]);
				mods[nmods++ % NMOD] = n;
			}
			break;
		}
		case 19: { /* drop the ctx reference early; modules and lists keep it alive */
			kmod_ref(ctx);
			kmod_unref(ctx);
			break;
		}
		}
	}
	for (int k = 0; k < NLIST; k++)
		kmod_module_unref_list(lists[k]);
	for (int k = 0; k < NMOD; k++)
		kmod_module_unref(mods[k]);
	kmod_unref(ctx);
	return 0;
}
