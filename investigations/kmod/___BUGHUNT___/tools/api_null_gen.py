#!/usr/bin/env python3
"""api_null_gen.py <libkmod.h> > api.c: C driver calling every public kmod_* function in a forked child with all-zero arguments, and with a valid ctx/module as first argument. Build: clang -fsanitize=address -shared-libasan api.c -L<build> -lkmod; run: api <some.ko>. Prints SIGNAL lines; ASAN reports go to stderr."""
import re
import sys

src = open(sys.argv[1]).read()
protos = re.findall(r"^((?:const\s+)?[A-Za-z_][\w\s\*]*?)\s*(kmod_\w+)\s*\(([^;{]*?)\)\s*;", src, re.M | re.S)
out = ['#define _GNU_SOURCE', '#include <libkmod.h>', '#include <stdio.h>', '#include <stdlib.h>', '#include <sys/wait.h>',
       '#include <unistd.h>', 'static struct kmod_ctx *ctx0;', 'static struct kmod_module *mod0;',
       '#define RUN(name, ...) do { fflush(stdout); pid_t p = fork(); if (!p) { alarm(10); __VA_ARGS__; _exit(0);} int st; '
       'waitpid(p,&st,0); if (WIFSIGNALED(st)) printf("SIGNAL %d: %s\\n", WTERMSIG(st), name); } while (0)',
       'int main(int argc, char **argv){ ctx0 = kmod_new(NULL, NULL); kmod_module_new_from_path(ctx0, argv[1], &mod0);']
z = lambda a: "NULL" if ("*" in a or "[" in a) else "0"
for ret, name, args in protos:
    args = re.sub(r"\s+", " ", args).strip()
    parts = [a.strip() for a in args.split(",")]
    if args in ("void", "") or name in ("kmod_set_log_fn", "kmod_module_probe_insert_module"):
        continue
    out.append(f'RUN("{name}(zeros)", {name}({", ".join(z(a) for a in parts)}));')
    if "struct kmod_ctx" in parts[0]:
        out.append(f'RUN("{name}(ctx,zeros)", {name}({", ".join(["ctx0"] + [z(a) for a in parts[1:]])}));')
    elif "struct kmod_module" in parts[0] and "kmod_list" not in parts[0]:
        out.append(f'RUN("{name}(mod,zeros)", {name}({", ".join(["(void*)mod0"] + [z(a) for a in parts[1:]])}));')
out.append('puts("api-null-done"); return 0; }')
print("\n".join(out))
