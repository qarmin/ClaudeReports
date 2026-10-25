/*
 * modules.*.bin readers: the mmap reader (normal path after kmod_load_resources) and the stdio
 * reader (fallback path) on the same crafted file, plus a differential check that both readers
 * return the same answers. Input layout: [klen:1][key:klen][index file bytes].
 */
#include <sys/mman.h>

#include "common.h"
#include "libkmod/libkmod-index.h"

static char *read_all(int fd, size_t *len)
{
	off_t end = lseek(fd, 0, SEEK_END);
	char *buf = malloc(end + 1);
	pread(fd, buf, end, 0);
	buf[end] = '\0';
	*len = end;
	return buf;
}

static void values_to_str(struct index_value *v, char *out, size_t outsz)
{
	size_t o = 0;
	out[0] = '\0';
	for (; v && o + 1 < outsz; v = v->next)
		o += snprintf(out + o, outsz - o, "%u:%s;", v->priority, v->value);
}


static int cmp_lines(const void *a, const void *b)
{
	return strcmp(*(char *const *)a, *(char *const *)b);
}

/* Dumps list values of one key in reader-specific order (stdio add_value sorts by priority): compare as line sets. */
static char *sorted_lines(const char *buf, size_t len)
{
	char *copy = malloc(len + 1), **lines = malloc(sizeof(char *) * (len + 1)), *out = malloc(len + 1);
	size_t n = 0, o = 0;

	out[0] = '\0';

	memcpy(copy, buf, len);
	copy[len] = '\0';
	for (char *p = copy; p < copy + len; p += strlen(p) + 1) {
		lines[n++] = p;
		char *nl = memchr(p, '\n', copy + len - p);
		if (!nl)
			break;
		*nl = '\0';
	}
	qsort(lines, n, sizeof(char *), cmp_lines);
	for (size_t i = 0; i < n; i++)
		o += sprintf(out + o, "%s\n", lines[i]);
	free(copy);
	free(lines);
	return out;
}

int LLVMFuzzerTestOneInput(const uint8_t *d, size_t s)
{
	static struct kmod_ctx *ctx;
	static char path[300];
	if (!ctx) {
		ctx = quiet_ctx("/nonexistent-kmod-fuzz-moddir", NULL);
		snprintf(path, sizeof(path), "%s/fuzz.bin", scratch_dir());
	}
	if (s < 1)
		return 0;
	size_t klen = d[0] % 33;
	if (s < 1 + klen)
		return 0;
	char key[34];
	memcpy(key, d + 1, klen);
	key[klen] = '\0';
	if (strlen(key) != klen)
		return 0;
	if (write_file(path, d + 1 + klen, s - 1 - klen) < 0)
		return 0;

	struct index_mm *mm = NULL;
	unsigned long long stamp;
	if (index_mm_open(ctx, path, &stamp, &mm) < 0)
		mm = NULL;
	struct index_file *fi = index_file_open(path);

	char *r_mm = NULL, *r_fi = NULL;
	char w_mm[4096] = "", w_fi[4096] = "";
	if (mm) {
		r_mm = index_mm_search(mm, key);
		struct index_value *v = index_mm_searchwild(mm, key);
		values_to_str(v, w_mm, sizeof(w_mm));
		index_values_free(v);
	}
	if (fi) {
		r_fi = index_search(fi, key);
		struct index_value *v = index_searchwild(fi, key);
		values_to_str(v, w_fi, sizeof(w_fi));
		index_values_free(v);
	}

	if (mm && fi) {
		/* Differential: both readers parse the same on-disk format. */
		if ((r_mm == NULL) != (r_fi == NULL) || (r_mm && strcmp(r_mm, r_fi) != 0)) {
			fprintf(stderr, "DIFF index-search key='%s' mm='%s' file='%s'\n", key,
				r_mm ? r_mm : "(null)", r_fi ? r_fi : "(null)");
			abort();
		}
		if (strcmp(w_mm, w_fi) != 0) {
			fprintf(stderr, "DIFF index-searchwild key='%s' mm='%s' file='%s'\n", key, w_mm,
				w_fi);
			abort();
		}
		int fd1 = memfd_create("mm", 0), fd2 = memfd_create("fi", 0);
		index_mm_dump(mm, fd1, true);
		index_dump(fi, fd2, true);
		size_t l1, l2;
		char *b1 = read_all(fd1, &l1), *b2 = read_all(fd2, &l2);
		char *s1 = sorted_lines(b1, l1), *s2 = sorted_lines(b2, l2);
		if (l1 != l2 || (!memchr(b1, 0, l1) && strcmp(s1, s2) != 0)) {
			fprintf(stderr, "DIFF index-dump mm=%zu bytes file=%zu bytes\n", l1, l2);
			if (getenv("KMOD_FUZZ_SHOW_DIFF")) {
				FILE *f1 = fopen("/tmp/diff_mm.txt", "w"), *f2 = fopen("/tmp/diff_fi.txt", "w");
				fwrite(b1, 1, l1, f1);
				fwrite(b2, 1, l2, f2);
				fclose(f1);
				fclose(f2);
			}
			abort();
		}
		free(b1);
		free(b2);
		free(s1);
		free(s2);
		close(fd1);
		close(fd2);
	} else if (mm || fi) {
		int fd = open("/dev/null", O_WRONLY);
		if (mm)
			index_mm_dump(mm, fd, false);
		else
			index_dump(fi, fd, false);
		close(fd);
	}

	free(r_mm);
	free(r_fi);
	if (mm)
		index_mm_close(mm);
	if (fi)
		index_file_close(fi);
	return 0;
}
