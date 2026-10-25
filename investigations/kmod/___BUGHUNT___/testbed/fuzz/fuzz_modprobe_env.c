/* MODPROBE_OPTIONS tokenizer (tools/modprobe.c prepend_options_from_env), compiled in directly. */
#include "tools/modprobe.c"

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	char *env = malloc(s + 1);
	memcpy(env, d, s);
	env[s] = '\0';
	setenv("MODPROBE_OPTIONS", env, 1);
	free(env);

	char *orig[] = { "modprobe", "-n", "dummy", NULL };
	int argc = 3;
	char **argv = prepend_options_from_env(&argc, orig);
	if (argv && argv != orig) {
		for (int i = 0; i < argc; i++)
			(void)strlen(argv[i]);
		if (argv[argc] != NULL)
			abort();
		free(argv);
	}
	return 0;
}
