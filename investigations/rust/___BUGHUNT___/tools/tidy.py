#!/usr/bin/env python3
"""Tidy a bug-hunt / perf-optimize workspace: delete debug litter, sort the SUMMARY tables.

Both jobs are mechanical and must be repeatable, so they live here instead of being
re-derived by an agent every session (which costs tokens and drifts).

  --clean   delete core dumps and profiler output (see DEBRIS below)
  --sort    sort the markdown tables in SUMMARY.md / SUMMARY-SHORT.md by their `#` column

With neither flag, does BOTH. Safe to re-run at any point; both jobs are idempotent.

WHY --clean SCANS THE PROJECT ROOT, NOT JUST THE WORKSPACE
----------------------------------------------------------
A repro that segfaults dumps `core.<pid>` into the *current working directory*, which for
a repro run against the main tree is the PROJECT ROOT, not `___BUGHUNT___/`. The justfile's
own `clean` recipe only ever looked inside the workspace, so those cores were never
collected - and a crash-hunting session that runs a crashing repro 10x per candidate leaves
hundreds of MB (20 MB per dump is typical) sitting in the project root indefinitely.
That is how a disk fills up without anyone noticing.

SAFETY
------
Deletion is by EXACT name pattern, never a wildcard sweep:
  * `core.<digits>` / `vgcore.<digits>`     - unambiguous, no source file looks like this
  * bare `core`                             - ONLY if it is a regular file whose first four
                                              bytes are the ELF magic `\\x7fELF`, so a `core`
                                              *directory* (extremely common: `core/`, `src/core/`)
                                              or a text file called `core` is never touched
  * `perf.data`, `perf.data.old`, `gmon.out`
  * `cachegrind.out.*`, `callgrind.out.*`, `massif.out.*`, `*.stackdump`, `*.profraw`
`.git`, `target`, `node_modules`, `clone` and `.venv` are never descended into, and the
project-root scan is depth-limited (`--max-depth`, default 3) so this never walks a huge
tree. Use `--dry-run` to see what would go without deleting anything.

USAGE
-----
  python3 tidy.py                         # workspace = cwd, project root = its parent
  python3 tidy.py --workspace ___BUGHUNT___ --project-root .
  python3 tidy.py --clean --dry-run
  python3 tidy.py --sort --desc           # newest-first ledgers instead of ascending
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

SKIP_DIRS = {".git", "target", "node_modules", "clone", ".venv", "venv", "__pycache__"}

# (compiled pattern, needs_elf_check)
DEBRIS = [
    (re.compile(r"^core\.\d+$"), False),
    (re.compile(r"^vgcore\.\d+$"), False),
    (re.compile(r"^core$"), True),
    (re.compile(r"^perf\.data(\.old)?$"), False),
    (re.compile(r"^gmon\.out$"), False),
    (re.compile(r"^(cachegrind|callgrind|massif)\.out\..+$"), False),
    (re.compile(r".+\.stackdump$"), False),
    (re.compile(r".+\.profraw$"), False),
]


def is_elf(path: Path) -> bool:
    try:
        with open(path, "rb") as fh:
            return fh.read(4) == b"\x7fELF"
    except OSError:
        return False


def is_debris(path: Path) -> bool:
    name = path.name
    for pat, needs_elf in DEBRIS:
        if pat.match(name):
            # A bare `core` is only litter when it really is an ELF core dump - never a
            # `core/` directory or a hand-written file that happens to be called `core`.
            return (not needs_elf) or (path.is_file() and is_elf(path))
    return False


def walk(root: Path, max_depth: int):
    """Yield files under root, skipping SKIP_DIRS and stopping at max_depth."""
    stack = [(root, 0)]
    while stack:
        d, depth = stack.pop()
        try:
            entries = list(d.iterdir())
        except OSError:
            continue
        for e in entries:
            if e.is_symlink():
                continue
            if e.is_dir():
                if e.name not in SKIP_DIRS and depth < max_depth:
                    stack.append((e, depth + 1))
            else:
                yield e


def clean(targets: list[tuple[Path, int]], dry_run: bool) -> tuple[int, int]:
    seen: set[Path] = set()
    n = total = 0
    for root, max_depth in targets:
        if not root.is_dir():
            continue
        for f in walk(root, max_depth):
            rp = f.resolve()
            if rp in seen or not is_debris(f):
                continue
            seen.add(rp)
            try:
                size = f.stat().st_size
            except OSError:
                continue
            print(f"  {'would remove' if dry_run else 'removed'}  {f}  ({size / 1048576:.1f} MB)")
            if not dry_run:
                try:
                    f.unlink()
                except OSError as e:
                    print(f"  FAILED to remove {f}: {e}", file=sys.stderr)
                    continue
            n += 1
            total += size
    return n, total


_SEP_RE = re.compile(r"^\|[\s:|-]+\|$")


def first_cell(line: str) -> str:
    """Text of the first table cell, honouring `\\|` escapes inside cells."""
    body = line.strip()
    if body.startswith("|"):
        body = body[1:]
    out = []
    i = 0
    while i < len(body):
        if body[i] == "\\" and i + 1 < len(body):
            out.append(body[i + 1])
            i += 2
            continue
        if body[i] == "|":
            break
        out.append(body[i])
        i += 1
    return "".join(out).strip()


def sort_key(line: str):
    """Sort by the leading `#` column as a NUMBER, so 2 sorts before 12 (a plain string
    sort is what produces the unreadable 1, 10, 11, 2 ordering when rows aren't
    zero-padded). Rows whose first cell isn't a number keep their relative order and are
    parked at the end."""
    cell = first_cell(line)
    m = re.match(r"^`?(\d+)`?$", cell)
    return (0, int(m.group(1))) if m else (1, 0)


def sort_tables(path: Path, desc: bool, dry_run: bool) -> int:
    """Sort every markdown table in `path` by its first column. Returns rows moved."""
    if not path.is_file():
        return 0
    lines = path.read_text(encoding="utf-8").splitlines()
    out: list[str] = []
    moved = 0
    i = 0
    while i < len(lines):
        # A table is: header row | separator row | one or more data rows.
        if (lines[i].lstrip().startswith("|")
                and i + 1 < len(lines) and _SEP_RE.match(lines[i + 1].strip())):
            header, sep = lines[i], lines[i + 1]
            j = i + 2
            while j < len(lines) and lines[j].lstrip().startswith("|"):
                j += 1
            rows = lines[i + 2:j]
            numbered = [r for r in rows if sort_key(r)[0] == 0]
            other = [r for r in rows if sort_key(r)[0] != 0]
            ordered = sorted(numbered, key=sort_key, reverse=desc) + other
            moved += sum(1 for a, b in zip(rows, ordered) if a != b)
            out.extend([header, sep, *ordered])
            i = j
            continue
        out.append(lines[i])
        i += 1

    # Never lose or invent a line: the output must be a permutation of the input.
    if sorted(out) != sorted(lines):
        print(f"  ERROR: refusing to write {path.name} - content would change, not just reorder",
              file=sys.stderr)
        return -1
    if moved and not dry_run:
        try:
            path.write_text("\n".join(out) + "\n", encoding="utf-8")
        except OSError as e:
            print(f"  ERROR: cannot write {path}: {e}", file=sys.stderr)
            return -1
    if moved:
        print(f"  {'would reorder' if dry_run else 'reordered'}  {path.name}: {moved} row(s)")
    else:
        print(f"  already sorted  {path.name}")
    return moved


def main() -> int:
    ap = argparse.ArgumentParser(description="Clean debug litter and sort SUMMARY tables.")
    ap.add_argument("--workspace", type=Path, default=Path("."),
                    help="___BUGHUNT___ / ___OPTIMIZATIONS___ dir (default: cwd)")
    ap.add_argument("--project-root", type=Path, default=None,
                    help="Main project root to also scan for stray core dumps "
                         "(default: the workspace's parent)")
    ap.add_argument("--clean", action="store_true", help="Delete debug litter")
    ap.add_argument("--sort", action="store_true", help="Sort the SUMMARY tables")
    ap.add_argument("--desc", action="store_true", help="Sort descending (newest first)")
    ap.add_argument("--max-depth", type=int, default=3,
                    help="Depth limit for the project-root scan (default 3)")
    ap.add_argument("--dry-run", action="store_true", help="Show what would happen, change nothing")
    args = ap.parse_args()

    do_clean = args.clean or not (args.clean or args.sort)
    do_sort = args.sort or not (args.clean or args.sort)

    ws = args.workspace.resolve()
    root = (args.project_root or ws.parent).resolve()
    rc = 0

    if do_clean:
        print(f"clean: workspace {ws}, project root {root} (max depth {args.max_depth})")
        # The workspace itself is walked deeply (candidate folders nest a few levels);
        # the project root only shallowly, since that is just where a crashing repro's
        # cwd happens to be.
        n, total = clean([(ws, 12), (root, args.max_depth)], args.dry_run)
        print(f"  {n} file(s), {total / 1048576:.1f} MB"
              f"{' (dry run, nothing deleted)' if args.dry_run else ' reclaimed'}")

    if do_sort:
        print(f"sort: {'descending' if args.desc else 'ascending'} by the '#' column")
        for name in ("SUMMARY.md", "SUMMARY-SHORT.md"):
            if sort_tables(ws / name, args.desc, args.dry_run) < 0:
                rc = 1
    return rc


if __name__ == "__main__":
    raise SystemExit(main())
