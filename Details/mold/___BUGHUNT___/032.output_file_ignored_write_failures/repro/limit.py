#!/usr/bin/env python3
"""Run a command with RLIMIT_FSIZE capped and its stdout redirected to a file.

SIG_IGN dispositions survive execve, so ignoring SIGXFSZ here makes the child
see a plain EFBIG write error instead of being killed by the signal - which is
exactly the "write() failed but the process keeps going" situation this repro
is about.

usage: limit.py <byte-limit> <stdout-file> <cmd> [args...]
"""
import os
import resource
import signal
import sys

limit = int(sys.argv[1])
outpath = sys.argv[2]
cmd = sys.argv[3:]

fd = os.open(outpath, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o644)
os.dup2(fd, 1)
if fd != 1:
    os.close(fd)
signal.signal(signal.SIGXFSZ, signal.SIG_IGN)
resource.setrlimit(resource.RLIMIT_FSIZE, (limit, limit))
os.execvp(cmd[0], cmd)
