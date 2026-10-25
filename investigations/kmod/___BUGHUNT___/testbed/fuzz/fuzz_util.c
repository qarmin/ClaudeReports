/* shared/util.c string/path helpers: freadline_wrapped, path_to_modname, path_ends_with_kmod_ext, alias_normalize, underscores. */
#include "common.h"
#include "shared/util.h"

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	char buf[PATH_MAX];
	size_t len;
	char *line, *str;
	unsigned int ln = 0;
	FILE *f;

	if (s == 0)
		return 0;
	str = malloc(s + 1);
	memcpy(str, d, s);
	str[s] = '\0';
	if (strlen(str) == s) {
		if (path_to_modname(str, buf, &len) && len != strlen(buf))
			__builtin_trap();
		(void)path_ends_with_kmod_ext(str, s);
		if (alias_normalize(str, buf, &len) == 0 && len != strlen(buf))
			__builtin_trap();
		if (underscores(str) >= 0 && strchr(str, '-') != NULL) {
			/* dashes outside [] must be gone */
			int depth = 0;
			for (char *p = str; *p; p++) {
				if (*p == '[')
					depth = 1;
				else if (*p == ']')
					depth = 0;
				else if (*p == '-' && !depth)
					__builtin_trap();
			}
		}
	}
	free(str);

	f = fmemopen((void *)d, s, "r");
	if (f) {
		size_t total = 0;
		while ((line = freadline_wrapped(f, &ln)) != NULL) {
			total += strlen(line);
			free(line);
		}
		if (total > s)
			__builtin_trap();
		fclose(f);
	}
	return 0;
}
