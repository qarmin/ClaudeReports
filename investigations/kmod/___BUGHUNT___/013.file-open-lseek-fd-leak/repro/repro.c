// kmod_module_get_info() on a non-seekable file (a pipe reopened through /proc/self/fd) leaks one fd per call.
#include <dirent.h>
#include <libkmod.h>
#include <stdio.h>
#include <unistd.h>

static int count_fds(void)
{
	int n = 0;
	DIR *d = opendir("/proc/self/fd");

	while (readdir(d))
		n++;
	closedir(d);
	return n;
}

int main(void)
{
	int p[2], before, after;
	char path[64];
	struct kmod_ctx *ctx = kmod_new(NULL, NULL);

	if (!ctx || pipe(p) < 0)
		return 2;
	snprintf(path, sizeof(path), "/proc/self/fd/%d", p[0]);
	before = count_fds();
	for (int i = 0; i < 100; i++) {
		struct kmod_module *mod;
		struct kmod_list *info = NULL;

		if (kmod_module_new_from_path(ctx, path, &mod) < 0)
			return 2;
		kmod_module_get_info(mod, &info);
		kmod_module_info_free_list(info);
		kmod_module_unref(mod);
	}
	after = count_fds();
	printf("open fds before: %d, after 100 failed kmod_module_get_info() calls: %d\n", before, after);
	kmod_unref(ctx);
	return after > before ? 1 : 0;
}
