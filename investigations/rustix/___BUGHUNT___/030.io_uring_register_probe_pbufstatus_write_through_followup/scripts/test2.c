#define _GNU_SOURCE
#include <stdio.h>
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

    long ret;
    ret = io_uring_register(fd, IORING_REGISTER_PERSONALITY, NULL, 0);
    printf("PERSONALITY ret=%ld errno=%d (%s)\n", ret, errno, strerror(errno));

    ret = io_uring_register(fd, IORING_REGISTER_ENABLE_RINGS, NULL, 0);
    printf("ENABLE_RINGS ret=%ld errno=%d (%s)\n", ret, errno, strerror(errno));

    ret = io_uring_register(fd, 999, NULL, 0);
    printf("BOGUS(999) ret=%ld errno=%d (%s)\n", ret, errno, strerror(errno));

    return 0;
}
