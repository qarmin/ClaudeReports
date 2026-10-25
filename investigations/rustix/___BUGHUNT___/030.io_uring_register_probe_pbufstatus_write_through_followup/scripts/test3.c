#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <errno.h>
#include <linux/io_uring.h>
#include <stdint.h>

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

    // register a provided buffer ring (bgid=42, 8 entries)
    size_t ring_size = 8 * sizeof(struct io_uring_buf);
    void *ring = mmap(NULL, ring_size, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    if (ring == MAP_FAILED) { perror("mmap"); return 1; }

    struct io_uring_buf_reg reg;
    memset(&reg, 0, sizeof(reg));
    reg.ring_addr = (unsigned long)(uintptr_t)ring;
    reg.ring_entries = 8;
    reg.bgid = 42;

    long ret = io_uring_register(fd, IORING_REGISTER_PBUF_RING, &reg, 1);
    printf("PBUF_RING register ret=%ld errno=%d (%s)\n", ret, errno, strerror(errno));

    struct io_uring_buf_status status;
    memset(&status, 0xAB, sizeof(status));
    status.buf_group = 42;
    ret = io_uring_register(fd, IORING_REGISTER_PBUF_STATUS, &status, 1);
    printf("PBUF_STATUS ret=%ld errno=%d (%s)\n", ret, errno, strerror(errno));
    printf("  status.head after call = 0x%x (poison was 0xABABABAB)\n", status.head);

    return 0;
}
