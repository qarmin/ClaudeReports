/*
 * Dump everything libkmod reports about module files, one canonical text block per path:
 *   == <path>
 *   info\t<key>\t<value>
 *   ver\t<symbol>\t<crc>
 *   sym\t<symbol>\t<crc>
 *   dep\t<symbol>\t<crc>\t<bind>
 * Paths come from argv, or one per line on stdin when argv has none. Used by the modinfo,
 * compression and corpus components. Values have '\n' and '\t' escaped.
 */
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "libkmod/libkmod.h"

static void quiet(void *d, int p, const char *f, int l, const char *fn, const char *fmt, va_list a)
{
}

static void put_esc(const char *s)
{
	for (; s && *s; s++) {
		if (*s == '\n')
			fputs("\\n", stdout);
		else if (*s == '\t')
			fputs("\\t", stdout);
		else if (*s == '\\')
			fputs("\\\\", stdout);
		else
			putchar(*s);
	}
}

static void dump(struct kmod_ctx *ctx, const char *path)
{
	struct kmod_module *mod;
	struct kmod_list *l = NULL, *it;
	int err;

	printf("== %s\n", path);
	err = kmod_module_new_from_path(ctx, path, &mod);
	if (err < 0) {
		printf("error\tnew_from_path\t%d\n", err);
		return;
	}
	l = NULL;
	err = kmod_module_get_info(mod, &l);
	if (err < 0) {
		printf("error\tget_info\t%d\n", err);
	} else {
		kmod_list_foreach(it, l) {
			fputs("info\t", stdout);
			put_esc(kmod_module_info_get_key(it));
			putchar('\t');
			put_esc(kmod_module_info_get_value(it));
			putchar('\n');
		}
		kmod_module_info_free_list(l);
	}
	l = NULL;
	if ((err = kmod_module_get_versions(mod, &l)) >= 0) {
		kmod_list_foreach(it, l)
			printf("ver\t%s\t0x%08" PRIx64 "\n", kmod_module_version_get_symbol(it),
			       kmod_module_version_get_crc(it));
		kmod_module_versions_free_list(l);
	} else
		printf("error\tget_versions\t%d\n", err);
	l = NULL;
	if ((err = kmod_module_get_symbols(mod, &l)) >= 0) {
		kmod_list_foreach(it, l)
			printf("sym\t%s\t0x%08" PRIx64 "\n", kmod_module_symbol_get_symbol(it),
			       kmod_module_symbol_get_crc(it));
		kmod_module_symbols_free_list(l);
	} else
		printf("error\tget_symbols\t%d\n", err);
	l = NULL;
	if ((err = kmod_module_get_dependency_symbols(mod, &l)) >= 0) {
		kmod_list_foreach(it, l)
			printf("dep\t%s\t0x%08" PRIx64 "\t%d\n",
			       kmod_module_dependency_symbol_get_symbol(it),
			       kmod_module_dependency_symbol_get_crc(it),
			       kmod_module_dependency_symbol_get_bind(it));
		kmod_module_dependency_symbols_free_list(l);
	} else
		printf("error\tget_dependency_symbols\t%d\n", err);
	kmod_module_unref(mod);
}

int main(int argc, char *argv[])
{
	const char *no_cfg[] = { "/nonexistent-kmod-testbed", NULL };
	struct kmod_ctx *ctx = kmod_new("/nonexistent-kmod-testbed", no_cfg);
	if (!ctx)
		return 2;
	kmod_set_log_fn(ctx, quiet, NULL);
	if (argc > 1) {
		for (int i = 1; i < argc; i++)
			dump(ctx, argv[i]);
	} else {
		char *line = NULL;
		size_t cap = 0;
		ssize_t n;
		while ((n = getline(&line, &cap, stdin)) > 0) {
			if (line[n - 1] == '\n')
				line[n - 1] = '\0';
			if (line[0])
				dump(ctx, line);
		}
		free(line);
	}
	kmod_unref(ctx);
	return 0;
}
