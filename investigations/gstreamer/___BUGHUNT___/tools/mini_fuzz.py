#!/usr/bin/env python3
"""A small, dependency-free mutation fuzzer for the ci/fuzzing/ localfuzzer-driven binaries.

No afl-fuzz/honggfuzz/radamsa is installed in this container and there's no root to apt-install
them - this script is the replacement. It's not a real coverage-guided fuzzer (no instrumentation
feedback), just random mutation + a corpus - but combined with an ASAN/UBSAN-instrumented target
(see tools/build_asan_fuzz.sh) it's enough to catch shallow crashes cheaply, which is this skill's
whole point (a deep, coverage-guided campaign belongs to the fuzz-bug-repro skill instead).

Usage:
    mini_fuzz.py <target binary> <seed corpus dir> <crashes out dir> \
        [--seconds N] [--iterations N] [--timeout SECS] [--jobs N] [--seed N]

Target convention: invoked as `<target> <input-file>` (matches ci/fuzzing/localfuzzer.c, which
takes a single file or directory argument and runs LLVMFuzzerTestOneInput on its contents once).
Exit code non-zero (a signal death shows up as negative returncode from subprocess, or a
sanitizer's abort()) counts as a crash. Set ASAN_OPTIONS/UBSAN_OPTIONS with abort_on_error=1 (see
this script's own env defaults below) so a sanitizer finding always turns into a non-zero exit
instead of being silently swallowed by GStreamer's own error handling.

Dedup: crashes are bucketed by the first "interesting" stack frame line found in stderr (skips
libc/sanitizer runtime frames) - good enough to avoid saving thousands of copies of the same bug,
not a substitute for real triage. Each unique bucket's first sample is saved; later ones are
counted but not written again unless --keep-all is passed.
"""
import argparse
import hashlib
import os
import random
import subprocess
import sys
import time
from pathlib import Path

SKIP_FRAME_SUBSTRINGS = (
    "asan_", "ubsan_", "sanitizer_common", "libc.so", "ld-linux",
    "__libc_start", "__interceptor_", "in main ", "abort_message",
)


def load_corpus(corpus_dir: Path) -> list[bytes]:
    files = [f for f in corpus_dir.rglob("*") if f.is_file()]
    if not files:
        sys.exit(f"no seed files found under {corpus_dir}")
    return [f.read_bytes() for f in files]


def mutate(data: bytes, rng: random.Random, corpus: list[bytes]) -> bytes:
    if not data:
        data = b"\x00"
    data = bytearray(data)
    n_ops = rng.randint(1, 6)
    for _ in range(n_ops):
        op = rng.randint(0, 5)
        if op == 0 and data:  # bit flip
            i = rng.randrange(len(data))
            data[i] ^= 1 << rng.randint(0, 7)
        elif op == 1 and data:  # byte set to random / extreme value
            i = rng.randrange(len(data))
            data[i] = rng.choice([0x00, 0xFF, 0x7F, 0x80, rng.randint(0, 255)])
        elif op == 2 and len(data) > 4:  # delete a chunk
            i = rng.randrange(len(data))
            length = rng.randint(1, min(32, len(data) - i))
            del data[i:i + length]
        elif op == 3:  # insert random bytes
            i = rng.randrange(len(data) + 1)
            chunk = bytes(rng.randint(0, 255) for _ in range(rng.randint(1, 16)))
            data[i:i] = chunk
        elif op == 4 and len(data) >= 4:  # duplicate a chunk
            i = rng.randrange(len(data))
            length = rng.randint(1, min(32, len(data) - i))
            chunk = data[i:i + length]
            j = rng.randrange(len(data) + 1)
            data[j:j] = chunk
        elif op == 5 and corpus:  # splice in a chunk from another seed
            other = rng.choice(corpus)
            if other:
                i = rng.randrange(len(other))
                length = rng.randint(1, min(32, len(other) - i))
                chunk = other[i:i + length]
                j = rng.randrange(len(data) + 1)
                data[j:j] = chunk
        if len(data) > 1_000_000:  # keep inputs bounded
            del data[1_000_000:]
    return bytes(data)


def crash_bucket(stderr: str) -> str:
    for line in stderr.splitlines():
        line = line.strip()
        if not line.startswith("#"):
            continue
        if any(s in line for s in SKIP_FRAME_SUBSTRINGS):
            continue
        return line
    # No usable frame - fall back to the first non-empty line (still better than nothing).
    for line in stderr.splitlines():
        if line.strip():
            return line.strip()
    return "<no output>"


def run_target(target: str, data: bytes, timeout: float, env: dict) -> tuple[int, str]:
    path = f"/tmp/mini_fuzz_input_{os.getpid()}"
    with open(path, "wb") as f:
        f.write(data)
    try:
        result = subprocess.run([target, path], capture_output=True, timeout=timeout, env=env)
        return result.returncode, result.stderr.decode("utf-8", "replace")
    except subprocess.TimeoutExpired:
        return -9999, "<timeout>"
    finally:
        try:
            os.unlink(path)
        except OSError:
            pass


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("target")
    ap.add_argument("corpus_dir", type=Path)
    ap.add_argument("crashes_dir", type=Path)
    ap.add_argument("--seconds", type=float, default=None)
    ap.add_argument("--iterations", type=int, default=None)
    ap.add_argument("--timeout", type=float, default=5.0, help="per-run timeout in seconds")
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--keep-all", action="store_true", help="save every crash, not just first-per-bucket")
    args = ap.parse_args()

    if args.seconds is None and args.iterations is None:
        args.seconds = 120  # a bounded default so a bare invocation doesn't run forever

    args.crashes_dir.mkdir(parents=True, exist_ok=True)
    corpus = load_corpus(args.corpus_dir)
    rng = random.Random(args.seed)

    env = os.environ.copy()
    # detect_leaks=0: the first run of any ci/fuzzing/ target forks a "gst-plugin-scanner" helper
    # to build the plugin registry cache, which leaks ~500 bytes through fontconfig/pango
    # one-time init (confirmed 2026-09-15, unrelated to the fuzzed input) - leak detection isn't
    # this workspace's goal anyway (memory-safety crashes are), and a leak finding here would
    # otherwise need the same noise-filtering treatment as the UBSAN "function"/alignment checks.
    env.setdefault("ASAN_OPTIONS", "abort_on_error=1:halt_on_error=1:exitcode=99:detect_leaks=0")
    # UBSAN is deliberately NOT halt_on_error here (2026-09-15): even with the build's own
    # -fno-sanitize=function (see tools/build_asan_fuzz.sh), this codebase's GST_READ_UINT32-style
    # macros trip UBSan's alignment/type-punning checks constantly on ordinary buffer reads that
    # work fine on x86 - halting on those would abort every single run on boilerplate before the
    # fuzzed input is meaningfully exercised, and would make ASAN's real memory-safety errors
    # (which already abort on their own via ASAN_OPTIONS above) indistinguishable from noise in
    # the crash bucket count. UBSAN still prints its report to stderr either way, so a real,
    # interesting UBSAN-only finding (no accompanying ASAN abort) is still visible in a saved
    # crash's .stderr.txt if one happens to line up with a real ASAN-detected crash - just not
    # used as the crash signal on its own.
    env.setdefault("UBSAN_OPTIONS", "print_stacktrace=1")
    # G_DEBUG=fatal-warnings/fatal-criticals is deliberately NOT set by default: GStreamer's
    # parsers routinely g_warning()/g_critical() on malformed input as part of normal, correct
    # error handling, so forcing those fatal would flood crashes_dir with non-bugs. Set it in the
    # caller's environment explicitly (it's honored via env.setdefault above) if hunting
    # specifically for "should never happen" assertion violations.

    seen_buckets: dict[str, Path] = {}
    start = time.monotonic()
    execs = 0
    crashes = 0

    def time_up() -> bool:
        if args.iterations is not None and execs >= args.iterations:
            return True
        if args.seconds is not None and (time.monotonic() - start) >= args.seconds:
            return True
        return False

    print(f"fuzzing {args.target} with {len(corpus)} seed(s), "
          f"budget: {args.iterations or ''}{' iterations' if args.iterations else ''}"
          f"{args.seconds or ''}{'s' if args.seconds else ''}")

    while not time_up():
        seed = rng.choice(corpus)
        data = mutate(seed, rng, corpus)
        rc, stderr = run_target(args.target, data, args.timeout, env)
        execs += 1

        if rc != 0:
            bucket = crash_bucket(stderr)
            digest = hashlib.sha1(bucket.encode("utf-8", "replace")).hexdigest()[:12]
            if digest not in seen_buckets or args.keep_all:
                crashes += 1
                fname = f"crash_{digest}_{crashes:04d}"
                (args.crashes_dir / fname).write_bytes(data)
                (args.crashes_dir / (fname + ".stderr.txt")).write_text(
                    f"exit code: {rc}\nbucket key: {bucket}\n\n{stderr}", encoding="utf-8")
                seen_buckets[digest] = args.crashes_dir / fname
                print(f"[{execs}] NEW crash bucket {digest}: {bucket[:100]}")

        if execs % 500 == 0:
            elapsed = time.monotonic() - start
            print(f"  ... {execs} execs, {elapsed:.0f}s, {execs / max(elapsed, 0.001):.0f} execs/s, "
                  f"{len(seen_buckets)} unique crash bucket(s)")

    elapsed = time.monotonic() - start
    print(f"\ndone: {execs} execs in {elapsed:.1f}s ({execs / max(elapsed, 0.001):.0f} execs/s), "
          f"{len(seen_buckets)} unique crash bucket(s) in {args.crashes_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
