#!/usr/bin/env python3
"""Shrink a crashing input to (close to) the smallest one that still reproduces the bug.

    minimize.py <target binary> <input file> [-- <extra target args>]

Ddmin-style: repeatedly tries removing chunks of the input, keeping any removal that still
reproduces, shrinking the chunk size as it goes. Writes the minimized input next to the original
as `<input>.min`. Prefer `cargo fuzz tmin` (see COMMANDS.md) when the target is a cargo-fuzz
harness - it does the same thing with corpus-aware tooling. Use this script for anything else
(a target binary invoked directly, not through cargo-fuzz).

ADAPT: `reproduces()` below assumes the target binary follows repro_template.rs's exit-code
convention (non-zero = reproduced) and is invoked as `<target> <input file>`. Change it if the
target reads from stdin instead, or signals differently.
"""
import subprocess
import sys


def reproduces(target: str, data: bytes, extra_args: list[str]) -> bool:
    path = "/tmp/minimize_candidate_input"
    with open(path, "wb") as f:
        f.write(data)
    result = subprocess.run([target, path, *extra_args], capture_output=True, timeout=10)
    return result.returncode != 0


def ddmin(data: bytes, test) -> bytes:
    chunk_size = len(data) // 2
    while chunk_size >= 1:
        changed = True
        while changed:
            changed = False
            i = 0
            while i < len(data):
                candidate = data[:i] + data[i + chunk_size:]
                if candidate and test(candidate):
                    data = candidate
                    changed = True
                else:
                    i += chunk_size
        chunk_size = chunk_size // 2 if chunk_size > 1 else 0
    return data


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__)
        return 2

    target = sys.argv[1]
    input_path = sys.argv[2]
    extra_args = sys.argv[4:] if len(sys.argv) > 3 and sys.argv[3] == "--" else []

    with open(input_path, "rb") as f:
        original = f.read()

    if not reproduces(target, original, extra_args):
        print(f"the ORIGINAL input does not reproduce against {target} - nothing to minimize")
        return 1

    print(f"original size: {len(original)} bytes")
    minimized = ddmin(original, lambda d: reproduces(target, d, extra_args))
    print(f"minimized size: {len(minimized)} bytes")

    out_path = input_path + ".min"
    with open(out_path, "wb") as f:
        f.write(minimized)
    print(f"wrote {out_path} - verify it still reproduces before using it as the candidate's repro input")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
