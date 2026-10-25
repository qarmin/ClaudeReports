#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/syscall.h>
#include <errno.h>
#include <linux/io_uring.h>

long io_uring_setup(unsigned entries, struct io_uring_params *p) {
    return syscall(__NR_io_uring_setup, entries, p);
}
long io_uring_register(int fd, unsigned opcode, void *arg, unsigned nr_args) {
    return syscall(__NR_io_uring_register, fd, opcode, arg, nr_args);
}

int main(void) {
    struct io_uring_params params;
    memset(&params, 0, sizeof(params));
    int fd = io_uring_setup(8, &params);
    if (fd < 0) { perror("io_uring_setup"); return 1; }
    printf("io_uring_setup fd=%d\n", fd);

    // Test PROBE
    size_t probe_sz = sizeof(struct io_uring_probe) + 256 * sizeof(struct io_uring_probe_op);
    unsigned char *buf = malloc(probe_sz);
    memset(buf, 0xFF, probe_sz);
    long ret = io_uring_register(fd, IORING_REGISTER_PROBE, buf, 256);
    printf("PROBE ret=%ld errno=%d (%s)\n", ret, errno, strerror(errno));
    if (ret >= 0) {
        struct io_uring_probe *p = (struct io_uring_probe*)buf;
        printf("  last_op=%u ops_len=%u (poisoned was 0xFF=255)\n", p->last_op, p->ops_len);
        int changed = 0;
        for (size_t i = 0; i < probe_sz; i++) if (buf[i] != 0xFF) changed = 1;
        printf("  buffer changed from poison: %s\n", changed ? "YES" : "no");
    }
    free(buf);
    return 0;
}
