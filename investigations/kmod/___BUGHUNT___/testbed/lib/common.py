"""Paths, environment and process helpers shared by testbed components."""
import os
import tempfile
import resource
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

TB = Path(__file__).resolve().parent.parent
SRC = TB / "_src"
BUILD_FUZZ = Path(os.environ.get("KMOD_TB_BUILD", TB / "__build-fuzz"))
BUILD_GCC = TB / "__build-gcc"
REALROOT = TB / "__corpus-data" / "realroot"
KVER = "7.0.0-38-generic"
REAL_MODDIR = REALROOT / "usr" / "lib" / "modules" / KVER
SYSTEM_MAP = REALROOT / "boot" / f"System.map-{KVER}"
KHEADERS = TB.parent / "__kheaders" / "root" / "usr" / "src" / f"linux-headers-{KVER}"
ASAN_RT_DIR = Path(subprocess.run(["clang", "-print-file-name=libclang_rt.asan-x86_64.so"],
                                  capture_output=True, text=True).stdout.strip()).parent

SAN_ENV = {
    "LD_LIBRARY_PATH": str(ASAN_RT_DIR),
    "ASAN_OPTIONS": "detect_leaks=1:abort_on_error=0:halt_on_error=1:print_summary=1",
    "UBSAN_OPTIONS": "halt_on_error=1:print_stacktrace=1:print_summary=1",
}


@dataclass
class Ctx:
    results: Path
    long: bool = False
    seed: int = 1
    failures: list = field(default_factory=list)

    def tool(self, name):
        return BUILD_FUZZ / name

    def workdir(self, component):
        d = self.results / component
        d.mkdir(parents=True, exist_ok=True)
        return d


def failure(signature, component, input_path, detail):
    return dict(signature=signature, component=component, input=str(input_path),
                detail=detail.replace("\t", " ").replace("\n", " ")[:300])


MEMCAP = TB.parent / "tools" / "memcap.sh"


OUTPUT_LIMIT = 32 << 20


def _limit_output():
    resource.setrlimit(resource.RLIMIT_FSIZE, (OUTPUT_LIMIT, OUTPUT_LIMIT))


def run(cmd, timeout=60, env=None, stdin=None, cwd=None, cap_mb=1536):
    """Run a sanitizer-instrumented process under a hard RSS cap (exit 137); never raises on non-zero exit or timeout."""
    cmd = [MEMCAP, cap_mb, "timeout", "-s", "KILL", int(timeout), *cmd]
    full_env = dict(os.environ)
    full_env.update(SAN_ENV)
    if env:
        full_env.update(env)
    # Output goes to size-limited disk files: a pipe read by this process would buffer runaway output in uncapped RAM.
    with tempfile.TemporaryFile(dir=TB / "results") as fo, tempfile.TemporaryFile(dir=TB / "results") as fe:
        try:
            p = subprocess.run([str(c) for c in cmd], stdout=fo, stderr=fe, timeout=timeout + 30,
                               env=full_env, input=stdin, cwd=cwd, preexec_fn=_limit_output)
            rc = p.returncode
        except subprocess.TimeoutExpired:
            rc = "timeout"
        outs = []
        for f in (fo, fe):
            f.seek(0)
            outs.append(f.read(4 << 20).decode("utf-8", "replace"))
    return rc, outs[0], outs[1]


def save_input(ctx, component, name, data_or_path):
    d = ctx.workdir(component) / "inputs"
    d.mkdir(exist_ok=True)
    dst = d / name
    if isinstance(data_or_path, (bytes, bytearray)):
        dst.write_bytes(data_or_path)
    elif Path(data_or_path).is_dir():
        shutil.copytree(data_or_path, dst, dirs_exist_ok=True)
    else:
        shutil.copy(data_or_path, dst)
    return dst


def real_modules():
    return sorted(REAL_MODDIR.rglob("*.ko*"))


def sample(items, n, seed):
    """Deterministic stride sample (plus a seed-dependent offset), n=None for all."""
    if n is None or n >= len(items):
        return list(items)
    step = len(items) / n
    off = seed % max(1, int(step))
    return [items[min(len(items) - 1, int(i * step) + off)] for i in range(n)]
