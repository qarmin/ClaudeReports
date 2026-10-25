/*
 * LD_PRELOAD shim standing in for a real kernel with a real out-of-tree TCP congestion-control
 * module registered whose `name` field (char name[TCP_CA_NAME_MAX], TCP_CA_NAME_MAX == 16) is
 * exactly 16 bytes long with no trailing NUL (legal: a C string-literal initializer exactly as
 * long as the destination array is explicitly permitted to drop the terminating NUL - see
 * RESULT.md). The real kernel's own getsockopt(TCP_CONGESTION) handler (net/ipv4/tcp.c,
 * do_tcp_getsockopt) does:
 *
 *   char ca_name[TCP_CA_NAME_MAX] = {};
 *   len = min_t(unsigned int, len, TCP_CA_NAME_MAX);
 *   memcpy(ca_name, icsk->icsk_ca_ops->name, sizeof(ca_name));
 *   copy_to_sockptr(optval, ca_name, len);
 *
 * i.e. it copies exactly TCP_CA_NAME_MAX (16) raw bytes out of the algorithm's `name` field,
 * whatever they are - there is no NUL-termination guarantee for a name that fills the whole
 * array. This shim reproduces exactly that shape of response for TCP_CONGESTION only; every
 * other getsockopt(2) call (including every other optname on the exact same socket) is passed
 * through unmodified to the real libc/kernel.
 */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <string.h>
#include <sys/socket.h>

typedef int (*real_getsockopt_t)(int, int, int, void *, socklen_t *);

int getsockopt(int sockfd, int level, int optname, void *optval, socklen_t *optlen) {
    static real_getsockopt_t real_getsockopt = NULL;
    if (!real_getsockopt) {
        real_getsockopt = (real_getsockopt_t)dlsym(RTLD_NEXT, "getsockopt");
    }

    if (level == IPPROTO_TCP && optname == TCP_CONGESTION) {
        /* Exactly 16 bytes, no NUL anywhere - what a real out-of-tree module's full-length
         * `name` field would hand back verbatim. */
        static const char name[16] = {
            's', 'u', 'p', 'e', 'r', 'c', 'o', 'n',
            'g', 'e', 's', 't', 'i', 'o', 'n', '1',
        };
        socklen_t want = *optlen;
        socklen_t give = want < sizeof(name) ? want : sizeof(name);
        memcpy(optval, name, give);
        *optlen = give;
        return 0;
    }

    return real_getsockopt(sockfd, level, optname, optval, optlen);
}
