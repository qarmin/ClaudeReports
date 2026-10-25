#define _GNU_SOURCE
#include <dlfcn.h>
#include <sys/statvfs.h>
#include <stdio.h>

typedef int (*statvfs_fn)(const char *, struct statvfs *);
typedef int (*statvfs64_fn)(const char *, struct statvfs64 *);
typedef int (*fstatvfs_fn)(int, struct statvfs *);
typedef int (*fstatvfs64_fn)(int, struct statvfs64 *);

int statvfs(const char *path, struct statvfs *buf) {
    static statvfs_fn real = 0;
    if (!real) real = (statvfs_fn)dlsym(RTLD_NEXT, "statvfs");
    fprintf(stderr, "[shim] statvfs(\"%s\") called\n", path);
    int rc = real(path, buf);
    if (rc == 0) buf->f_favail = buf->f_ffree / 2 + 7;
    return rc;
}

int statvfs64(const char *path, struct statvfs64 *buf) {
    static statvfs64_fn real = 0;
    if (!real) real = (statvfs64_fn)dlsym(RTLD_NEXT, "statvfs64");
    fprintf(stderr, "[shim] statvfs64(\"%s\") called\n", path);
    int rc = real(path, buf);
    if (rc == 0) buf->f_favail = buf->f_ffree / 2 + 7;
    return rc;
}

int fstatvfs(int fd, struct statvfs *buf) {
    static fstatvfs_fn real = 0;
    if (!real) real = (fstatvfs_fn)dlsym(RTLD_NEXT, "fstatvfs");
    fprintf(stderr, "[shim] fstatvfs(%d) called\n", fd);
    int rc = real(fd, buf);
    if (rc == 0) buf->f_favail = buf->f_ffree / 2 + 7;
    return rc;
}

int fstatvfs64(int fd, struct statvfs64 *buf) {
    static fstatvfs64_fn real = 0;
    if (!real) real = (fstatvfs64_fn)dlsym(RTLD_NEXT, "fstatvfs64");
    fprintf(stderr, "[shim] fstatvfs64(%d) called\n", fd);
    int rc = real(fd, buf);
    if (rc == 0) buf->f_favail = buf->f_ffree / 2 + 7;
    return rc;
}
