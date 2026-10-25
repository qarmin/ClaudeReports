/* Two `install m` / `remove m` lines in the configuration: kmod_module_get_install_commands() and
 * kmod_module_get_remove_commands() are documented to return "all install commands separated by semicolons". */
#include <stdio.h>
#include <string.h>
#include <libkmod.h>

int main(int argc, char **argv)
{
	const char *cfg[] = { argv[2], NULL };
	struct kmod_ctx *ctx = kmod_new(argv[1], cfg);
	struct kmod_module *mod;
	int bad = 0;

	if (!ctx || kmod_module_new_from_name(ctx, "m", &mod) < 0)
		return 2;

	const char *inst = kmod_module_get_install_commands(mod);
	const char *rem = kmod_module_get_remove_commands(mod);

	printf("install commands: '%s'\nremove commands:  '%s'\n", inst ? inst : "(null)", rem ? rem : "(null)");
	if (!inst || !strstr(inst, "echo one") || !strstr(inst, "echo two"))
		bad = 1;
	if (!rem || !strstr(rem, "echo rone") || !strstr(rem, "echo rtwo"))
		bad = 1;
	kmod_module_unref(mod);
	kmod_unref(ctx);
	return bad;
}
