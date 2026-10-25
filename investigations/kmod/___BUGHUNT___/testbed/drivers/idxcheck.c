/*
 * Index reader differential on a real depmod output dir:
 *   idxcheck dump <moddir> <type 0..4>        -> kmod_dump_index() to stdout
 *   idxcheck lookup <moddir>  (names on stdin) -> for each name, the module list returned by
 *       kmod_module_new_from_lookup() with the mmap'd indexes (kmod_load_resources) and with the
 *       stdio fallback reader; prints "MISMATCH" lines and exits 1 if they ever differ.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "libkmod/libkmod.h"

static void quiet(void *d, int p, const char *f, int l, const char *fn, const char *fmt, va_list a)
{
}

static struct kmod_ctx *ctx_for(const char *dir)
{
	const char *no_cfg[] = { "/nonexistent-kmod-testbed", NULL };
	struct kmod_ctx *ctx = kmod_new(dir, no_cfg);
	if (ctx)
		kmod_set_log_fn(ctx, quiet, NULL);
	return ctx;
}

static void lookup_str(struct kmod_ctx *ctx, const char *name, char *out, size_t outsz)
{
	struct kmod_list *list = NULL, *it;
	size_t o = 0;
	int err = kmod_module_new_from_lookup(ctx, name, &list);
	o += snprintf(out + o, outsz - o, "err=%d:", err < 0 ? err : 0);
	kmod_list_foreach(it, list) {
		struct kmod_module *m = kmod_module_get_module(it);
		const char *p = kmod_module_get_path(m);
		if (o < outsz)
			o += snprintf(out + o, outsz - o, "%s(%s);", kmod_module_get_name(m),
				      p ? p : "-");
		kmod_module_unref(m);
	}
	kmod_module_unref_list(list);
}

int main(int argc, char *argv[])
{
	if (argc >= 4 && strcmp(argv[1], "dump") == 0) {
		struct kmod_ctx *ctx = ctx_for(argv[2]);
		if (kmod_load_resources(ctx) < 0)
			return 2;
		int r = kmod_dump_index(ctx, atoi(argv[3]), 1);
		kmod_unref(ctx);
		return r < 0 ? 3 : 0;
	}
	if (argc >= 3 && strcmp(argv[1], "lookup") == 0) {
		struct kmod_ctx *mm = ctx_for(argv[2]), *fi = ctx_for(argv[2]);
		if (kmod_load_resources(mm) < 0)
			return 2;
		static char a[65536], b[65536];
		char *line = NULL;
		size_t cap = 0;
		ssize_t n;
		long checked = 0, bad = 0;
		while ((n = getline(&line, &cap, stdin)) > 0) {
			if (line[n - 1] == '\n')
				line[n - 1] = '\0';
			lookup_str(mm, line, a, sizeof(a));
			lookup_str(fi, line, b, sizeof(b));
			checked++;
			if (strcmp(a, b) != 0) {
				bad++;
				printf("MISMATCH\t%s\tmm=%s\tfile=%s\n", line, a, b);
			}
		}
		printf("checked=%ld mismatches=%ld\n", checked, bad);
		free(line);
		kmod_unref(mm);
		kmod_unref(fi);
		return bad ? 1 : 0;
	}
	fprintf(stderr, "usage: idxcheck dump <dir> <type> | lookup <dir>\n");
	return 2;
}
