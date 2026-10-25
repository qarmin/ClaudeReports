/*
 * For each stdin line: set MODPROBE_OPTIONS to it, run modprobe's own prepend_options_from_env()
 * and print "<count>" followed by "\x1f<token>" for every token it produced.
 */
#include "tools/modprobe.c"

int main(void)
{
	char *line = NULL;
	size_t cap = 0;
	ssize_t n;
	while ((n = getline(&line, &cap, stdin)) > 0) {
		if (line[n - 1] == '\n')
			line[n - 1] = '\0';
		setenv("MODPROBE_OPTIONS", line, 1);
		char *orig[] = { "modprobe", NULL };
		int argc = 1;
		char **argv = prepend_options_from_env(&argc, orig);
		printf("%d", argc - 1);
		for (int i = 1; i < argc; i++)
			printf("\x1f%s", argv[i]);
		putchar('\n');
		if (argv != orig)
			free(argv);
	}
	free(line);
	return 0;
}
