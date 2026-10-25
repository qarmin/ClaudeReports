#include <sys/socket.h>
#include <sys/un.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <unistd.h>

int main() {
    // Test 1: bind abstract name with addrlen = sizeof(sockaddr_un) (max, 110),
    // with trailing garbage bytes after the intended name (abstract names are
    // NOT re-clamped by the kernel's unix_mkname - only filesystem paths are).
    {
        int fd = socket(AF_UNIX, SOCK_DGRAM, 0);
        struct sockaddr_un addr;
        memset(&addr, 0xAA, sizeof(addr)); // garbage fill
        addr.sun_family = AF_UNIX;
        addr.sun_path[0] = 0; // abstract
        memcpy(&addr.sun_path[1], "probe1", 6);
        // leave rest as 0xAA garbage
        socklen_t len = sizeof(addr); // 110, the max unix_mkname allows
        int r = bind(fd, (struct sockaddr*)&addr, len);
        printf("test1 bind(len=%zu) = %d (errno=%d %s)\n", (size_t)len, r, errno, strerror(errno));
        if (r == 0) {
            struct sockaddr_storage st;
            socklen_t rlen = sizeof(st);
            memset(&st, 0, sizeof(st));
            r = getsockname(fd, (struct sockaddr*)&st, &rlen);
            printf("  getsockname -> r=%d reported len=%u (sizeof(sockaddr_un)=%zu)\n", r, rlen, sizeof(struct sockaddr_un));
        }
        close(fd);
    }

    // Test 2: bind with addrlen deliberately > sizeof(sockaddr_un) (128, full
    // sockaddr_storage) to see if the kernel's own unix_mkname rejects it.
    {
        int fd = socket(AF_UNIX, SOCK_DGRAM, 0);
        char buf[128];
        memset(buf, 0xBB, sizeof(buf));
        struct sockaddr_un *addr = (struct sockaddr_un*)buf;
        addr->sun_family = AF_UNIX;
        addr->sun_path[0] = 0;
        memcpy(&addr->sun_path[1], "probe2", 6);
        socklen_t len = 128;
        int r = bind(fd, (struct sockaddr*)buf, len);
        printf("test2 bind(len=%u, > sizeof(sockaddr_un)) = %d (errno=%d %s)\n", len, r, errno, strerror(errno));
        close(fd);
    }

    // Test 3: filesystem path bind with garbage after NUL + oversized addrlen,
    // check if kernel re-clamps (per libc backend's own comment, Linux should).
    {
        int fd = socket(AF_UNIX, SOCK_STREAM, 0);
        struct sockaddr_un addr;
        memset(&addr, 0xCC, sizeof(addr));
        addr.sun_family = AF_UNIX;
        const char *path = "/tmp/sockaddr_probe/test3.sock";
        unlink(path);
        strcpy(addr.sun_path, path);
        // addrlen covers full struct incl trailing garbage after NUL
        socklen_t len = sizeof(addr);
        int r = bind(fd, (struct sockaddr*)&addr, len);
        printf("test3 bind(fs path, len=%zu) = %d (errno=%d %s)\n", (size_t)len, r, errno, strerror(errno));
        if (r == 0) {
            struct sockaddr_storage st;
            socklen_t rlen = sizeof(st);
            memset(&st, 0, sizeof(st));
            r = getsockname(fd, (struct sockaddr*)&st, &rlen);
            printf("  getsockname -> r=%d reported len=%u (strlen+1+2=%zu)\n", r, rlen, strlen(path)+1+2);
        }
        close(fd);
        unlink(path);
    }
    return 0;
}
