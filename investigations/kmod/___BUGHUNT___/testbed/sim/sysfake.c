/*
 * LD_PRELOAD simulator of the kernel module interface for modprobe/rmmod/lsmod/insmod without root:
 * redirects /sys/module/... and /proc/modules to $SYSFAKE, fakes delete_module/init_module/finit_module on that state
 * and logs every call to $SYSFAKE/log. State: $SYSFAKE/sys/module/<name>/{initstate,refcnt,holders/<holder>,coresize},
 * $SYSFAKE/proc/modules (rewritten from the tree after every change).
 */
#define _GNU_SOURCE
#include <errno.h>
#include <dirent.h>
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <ftw.h>

static const char *root;
static void regen(void);

static const char *map(const char *path, char *buf, size_t sz)
{
	if (!root || !path)
		return path;
	if (strncmp(path, "/sys/module", 11) == 0 || strcmp(path, "/proc/modules") == 0 || strcmp(path, "/proc/cmdline") == 0) {
		snprintf(buf, sz, "%s%s", root, path);
		return buf;
	}
	return path;
}

#define REAL(ret, name, args) static ret (*real_##name) args
REAL(int, open, (const char *, int, ...));
REAL(int, open64, (const char *, int, ...));
REAL(int, openat, (int, const char *, int, ...));
REAL(FILE *, fopen, (const char *, const char *));
REAL(FILE *, fopen64, (const char *, const char *));
REAL(DIR *, opendir, (const char *));
REAL(int, stat, (const char *, struct stat *));
REAL(int, stat64, (const char *, struct stat64 *));

static void syms(void)
{
	if (real_open)
		return;
	root = getenv("SYSFAKE");
#define L(n) real_##n = dlsym(RTLD_NEXT, #n)
	L(open); L(open64); L(openat); L(fopen); L(fopen64); L(opendir); L(stat); L(stat64);
}

__attribute__((constructor)) static void init(void)
{
	syms();
	if (root && strstr(program_invocation_short_name, "lsmod"))
		regen();
}

#define MODE_ARG() mode_t mode = 0; if (flags & (O_CREAT | O_TMPFILE)) { va_list ap; va_start(ap, flags); mode = va_arg(ap, mode_t); va_end(ap); }
int open(const char *p, int flags, ...) { syms(); char b[4096]; MODE_ARG(); return real_open(map(p, b, sizeof b), flags, mode); }
int open64(const char *p, int flags, ...) { syms(); char b[4096]; MODE_ARG(); return real_open64(map(p, b, sizeof b), flags, mode); }
int openat(int d, const char *p, int flags, ...) { syms(); char b[4096]; MODE_ARG(); return real_openat(d, p && p[0] == '/' ? map(p, b, sizeof b) : p, flags, mode); }
FILE *fopen(const char *p, const char *m) { syms(); char b[4096]; return real_fopen(map(p, b, sizeof b), m); }
FILE *fopen64(const char *p, const char *m) { syms(); char b[4096]; return real_fopen64(map(p, b, sizeof b), m); }
DIR *opendir(const char *p) { syms(); char b[4096]; return real_opendir(map(p, b, sizeof b)); }
int stat(const char *p, struct stat *s) { syms(); char b[4096]; return real_stat(map(p, b, sizeof b), s); }
int stat64(const char *p, struct stat64 *s) { syms(); char b[4096]; return real_stat64(map(p, b, sizeof b), s); }

static void logf_(const char *fmt, ...)
{
	char p[4096];
	snprintf(p, sizeof p, "%s/log", root);
	FILE *f = real_fopen(p, "a");
	va_list ap;
	va_start(ap, fmt);
	vfprintf(f, fmt, ap);
	va_end(ap);
	fclose(f);
}

static int rm_cb(const char *path, const struct stat *sb, int flag, struct FTW *ftw) { return remove(path); }
static void rm_rf(const char *p) { nftw(p, rm_cb, 16, FTW_DEPTH | FTW_PHYS); }

static int read_int(const char *path)
{
	FILE *f = real_fopen(path, "r");
	int v = -1;
	if (f) {
		if (fscanf(f, "%d", &v) != 1)
			v = -1;
		fclose(f);
	}
	return v;
}

static void write_int(const char *path, int v)
{
	FILE *f = real_fopen(path, "w");
	if (f) {
		fprintf(f, "%d\n", v);
		fclose(f);
	}
}

/* /proc/modules is regenerated from the tree */
static void regen(void)
{
	char dir[4096], p[4096];
	snprintf(dir, sizeof dir, "%s/sys/module", root);
	DIR *d = real_opendir(dir);
	snprintf(p, sizeof p, "%s/proc/modules", root);
	FILE *out = real_fopen(p, "w");
	struct dirent *e;
	while (d && (e = readdir(d))) {
		if (e->d_name[0] == '.')
			continue;
		char q[4096];
		snprintf(q, sizeof q, "%s/%s/refcnt", dir, e->d_name);
		int rc = read_int(q);
		snprintf(q, sizeof q, "%s/%s/holders", dir, e->d_name);
		DIR *h = real_opendir(q);
		char hl[2048] = "";
		struct dirent *he;
		while (h && (he = readdir(h))) {
			if (he->d_name[0] == '.')
				continue;
			strcat(hl, he->d_name);
			strcat(hl, ",");
		}
		if (h)
			closedir(h);
		fprintf(out, "%s 4096 %d %s Live 0x0000000000000000\n", e->d_name, rc < 0 ? 0 : rc, hl[0] ? hl : "-");
	}
	if (d)
		closedir(d);
	fclose(out);
}

long delete_module(const char *name, unsigned int flags)
{
	char p[4096], q[4096];
	if (!root) {
		errno = EPERM;
		return -1;
	}
	logf_("delete %s flags=%u\n", name, flags);
	snprintf(p, sizeof p, "%s/sys/module/%s", root, name);
	struct stat st;
	if (real_stat(p, &st) != 0) {
		errno = ENOENT;
		return -1;
	}
	snprintf(q, sizeof q, "%s/refcnt", p);
	int rc = read_int(q);
	if (rc > 0 && !(flags & 0200000 /* O_TRUNC? KMOD force */) && !(flags & O_TRUNC)) {
		errno = (flags & O_NONBLOCK) ? EWOULDBLOCK : EBUSY;
		return -1;
	}
	/* drop this module from the holders of the modules it uses, and decrement their refcnt */
	char dir[4096];
	snprintf(dir, sizeof dir, "%s/sys/module", root);
	DIR *d = real_opendir(dir);
	struct dirent *e;
	while (d && (e = readdir(d))) {
		if (e->d_name[0] == '.')
			continue;
		snprintf(q, sizeof q, "%s/%s/holders/%s", dir, e->d_name, name);
		if (access(q, F_OK) == 0) {
			unlink(q);
			snprintf(q, sizeof q, "%s/%s/refcnt", dir, e->d_name);
			int r = read_int(q);
			if (r > 0)
				write_int(q, r - 1);
		}
	}
	if (d)
		closedir(d);
	rm_rf(p);
	regen();
	return 0;
}

static int do_insert(const char *name, const char *args, long flags)
{
	char p[4096], q[4096];
	logf_("insert %s args=[%s] flags=%ld\n", name, args ? args : "", flags);
	snprintf(p, sizeof p, "%s/sys/module/%s", root, name);
	struct stat st;
	if (real_stat(p, &st) == 0) {
		errno = EEXIST;
		return -1;
	}
	snprintf(q, sizeof q, "%s/sys", root);
	mkdir(q, 0755);
	snprintf(q, sizeof q, "%s/sys/module", root);
	mkdir(q, 0755);
	mkdir(p, 0755);
	snprintf(q, sizeof q, "%s/holders", p);
	mkdir(q, 0755);
	snprintf(q, sizeof q, "%s/initstate", p);
	FILE *f = real_fopen(q, "w");
	if (!f) {
		logf_("cannot create %s: %s\n", q, strerror(errno));
		errno = EIO;
		return -1;
	}
	fputs("live\n", f);
	fclose(f);
	snprintf(q, sizeof q, "%s/refcnt", p);
	write_int(q, 0);
	regen();
	return 0;
}

long init_module(const void *mem, unsigned long len, const char *args)
{
	static int n;
	char p[4096];
	if (!root) {
		errno = EPERM;
		return -1;
	}
	snprintf(p, sizeof p, "%s/init_%d.bin", root, n++);
	FILE *f = real_fopen(p, "w");
	if (f) {
		fwrite(mem, 1, len, f);
		fclose(f);
	}
	logf_("init_module len=%lu args=[%s] dump=%s\n", len, args ? args : "", p);
	return 0;
}

long syscall(long no, ...)
{
	static long (*real_syscall)(long, ...);
	va_list ap;
	va_start(ap, no);
	long a1 = va_arg(ap, long), a2 = va_arg(ap, long), a3 = va_arg(ap, long), a4 = va_arg(ap, long);
	va_end(ap);
	if (root && no == 313 /* __NR_finit_module x86_64 */) {
		char link[64], path[4096];
		snprintf(link, sizeof link, "/proc/self/fd/%ld", a1);
		ssize_t n = readlink(link, path, sizeof path - 1);
		if (n < 0)
			return -1;
		path[n] = 0;
		char *b = strrchr(path, '/');
		b = b ? b + 1 : path;
		char *dot = strstr(b, ".ko");
		if (dot)
			*dot = 0;
		for (char *c = b; *c; c++)
			if (*c == '-')
				*c = '_';
		return do_insert(b, (const char *)a2, a3);
	}
	if (!real_syscall)
		real_syscall = dlsym(RTLD_NEXT, "syscall");
	return real_syscall(no, a1, a2, a3, a4);
}
