#!/usr/bin/env python3
"""Collect bug-hunt and optimization workspaces into this repo.

The HTML reports gathered by `find_reports.py` are the *summary* of an audit.
This script gathers the *details*: the `___BUGHUNT___` and `___OPTIMIZATIONS___`
folders that the bug-hunt / perf-optimize skills build next to a project, each
holding per-candidate `RESULT.md`, `patch.diff`, `repro/`, and a `justfile` that
reproduces the finding from a fresh clone.

Usage:
    scripts/find_details.py <source-folder> [--dry-run] [--max-file-size 100k]

    # everything under ~/Projekty/Obce
    scripts/find_details.py ~/Projekty/Obce
    # a single project
    scripts/find_details.py ~/Projekty/Obce/hayro

Layout:
    Source folders are mirrored under `investigations/` by their path relative to
    <source-folder>, so nested workspaces keep their context and cannot collide:

        <src>/hayro/___BUGHUNT___          -> investigations/hayro/___BUGHUNT___
        <src>/hayro/vello/___OPTIMIZATIONS___ -> investigations/hayro/vello/___OPTIMIZATIONS___
        <src>/vello/___OPTIMIZATIONS___    -> investigations/vello/___OPTIMIZATIONS___

    When <source-folder> is itself a project (or a workspace folder), its own
    name is used as the leading component, so the project name is never lost.

What is left behind:
    The bug-hunt / perf-optimize skills name things by prefix: `_name` is kept,
    `__name` (two underscores) is scratch - build trees, source checkouts and
    worktrees, header trees, corpora, raw issue dumps, per-run results. Every file
    and directory whose name starts with `__` is skipped, at any depth. Also
    skipped:

      - legacy directory names the candidate justfiles still create: clone/,
        target/, .worktrees/, .git/, node_modules/, .locks/, scratch/, probe/
      - `*.zip` (output of `just pack`, a copy of what is being collected anyway),
        `*.pid`
      - `.gitignore` - these guard the *source* project against its own build
        caches. Copied over they would only make this repo silently skip files it
        did collect (`repro/Cargo.lock` above all, which pins the versions a repro
        needs to still build later).
      - symlinks, so no dangling links land in the repo
      - anything else over --max-file-size (default 100 KB)

    The size cap never applies to the deliverables themselves: files that carry
    the finding - `*.md`, `*.diff`, `*.patch`, source files, `justfile` - are
    always copied whole. Some `patch.diff` and `SUMMARY.md` files run past
    300 KB, and truncating those would defeat the point of collecting them.
    Small binaries stay under the cap on purpose: a malformed 2 KB PDF is often
    the repro itself, without which the candidate folder is useless.

Updating:
    The source is the only truth. A destination folder is deleted and written
    fresh, so renamed and dropped candidates are reflected instead of piling up.
    Only the (project, workspace) pairs actually found in the source are touched,
    so running against a narrower source folder never deletes the rest of
    investigations/. A destination that already matches its source is left alone.
"""

import argparse
import os
import shutil
import sys
from pathlib import Path

REPO_DIR = Path(__file__).resolve().parent.parent
DEST_DIR = REPO_DIR / "investigations"

KIND_DIRS = ("___BUGHUNT___", "___OPTIMIZATIONS___")

SKIP_DIRS = {
    "clone",
    "target",
    ".worktrees",
    ".git",
    "node_modules",
    ".locks",
    "scratch",
    "probe",
}

# Deliverables - copied whole, whatever their size.
ALWAYS_SUFFIXES = {
    ".md", ".diff", ".patch", ".toml", ".rs", ".sh", ".py",
    ".c", ".h", ".cpp", ".hpp", ".json", ".yml", ".yaml",
}
ALWAYS_NAMES = {"justfile", "Justfile"}
NEVER_SUFFIXES = {".zip", ".pid"}
NEVER_NAMES = {".gitignore"}

DEFAULT_MAX_FILE_SIZE = 100_000


def parse_size(text: str) -> int:
    units = {"k": 1_000, "m": 1_000_000, "g": 1_000_000_000}
    value = text.strip().lower().removesuffix("b")
    factor = units.get(value[-1:], 1)
    if factor != 1:
        value = value[:-1]
    try:
        return int(float(value) * factor)
    except ValueError:
        raise argparse.ArgumentTypeError(f"not a size: {text}") from None


def human(size: int) -> str:
    if size >= 1_000_000:
        return f"{size / 1_000_000:.1f} MB"
    if size >= 1_000:
        return f"{size / 1_000:.1f} KB"
    return f"{size} B"


def find_workspaces(root: Path) -> list[Path]:
    if root.name in KIND_DIRS:
        return [root]
    found = []
    for dirpath, dirnames, _ in os.walk(root):
        here = Path(dirpath)
        dirnames[:] = sorted(
            d for d in dirnames if d not in SKIP_DIRS and not scratch(d) and not (here / d).is_symlink()
        )
        for kind in KIND_DIRS:
            if kind in dirnames:
                found.append(here / kind)
                dirnames.remove(kind)
    return found


def dest_for(workspace: Path, root: Path) -> Path:
    if workspace == root:
        return DEST_DIR / root.parent.name / root.name
    rel = workspace.relative_to(root)
    # <root>/___BUGHUNT___ - the root is the project, keep its name.
    if len(rel.parts) == 1:
        rel = Path(root.name) / rel
    return DEST_DIR / rel


def wanted(path: Path, size: int, max_file_size: int) -> bool:
    suffix = path.suffix.lower()
    if suffix in NEVER_SUFFIXES or path.name in NEVER_NAMES:
        return False
    return suffix in ALWAYS_SUFFIXES or path.name in ALWAYS_NAMES or size <= max_file_size


def scratch(name: str) -> bool:
    return name.startswith("__") and not name.startswith("___")


def collect(workspace: Path, max_file_size: int) -> tuple[list[tuple[Path, Path, int]], int]:
    files: list[tuple[Path, Path, int]] = []
    links = 0
    for dirpath, dirnames, filenames in os.walk(workspace):
        here = Path(dirpath)
        keep = []
        for name in sorted(dirnames):
            if scratch(name) or name in SKIP_DIRS:
                continue
            if (here / name).is_symlink():
                links += 1
                continue
            keep.append(name)
        dirnames[:] = keep
        for name in sorted(filenames):
            path = here / name
            if scratch(name):
                continue
            if path.is_symlink():
                links += 1
                continue
            try:
                size = path.stat().st_size
            except OSError:
                continue
            if wanted(path, size, max_file_size):
                files.append((path.relative_to(workspace), path, size))
    return files, links


def fingerprint(dest: Path) -> dict[Path, tuple[int, int]]:
    """size + mtime of everything already at the destination.

    `shutil.copy2` preserves mtime, so an untouched destination fingerprints
    identically to its source. A false mismatch only costs a re-copy.
    """
    out: dict[Path, tuple[int, int]] = {}
    for dirpath, _, filenames in os.walk(dest):
        here = Path(dirpath)
        for name in filenames:
            path = here / name
            try:
                st = path.stat()
            except OSError:
                continue
            out[path.relative_to(dest)] = (st.st_size, st.st_mtime_ns)
    return out


def candidates(paths) -> set[str]:
    return {rel.parts[0] for rel in paths if len(rel.parts) > 1}


def describe(added: set[str], removed: set[str]) -> str:
    def plural(n: int) -> str:
        return f"{n} candidate{'' if n == 1 else 's'}"

    parts = []
    if added:
        parts.append(f"+{plural(len(added))}")
    if removed:
        names = f" ({', '.join(sorted(removed))})" if len(removed) <= 3 else ""
        parts.append(f"-{plural(len(removed))}{names}")
    return ", ".join(parts)


def sync(workspace: Path, dest: Path, max_file_size: int, dry_run: bool) -> tuple[bool, int, int, int]:
    files, links = collect(workspace, max_file_size)
    total = sum(size for _, _, size in files)

    before = fingerprint(dest)
    after = {rel: (size, src.stat().st_mtime_ns) for rel, src, size in files}
    label = dest.relative_to(REPO_DIR)

    if before == after:
        print(f"{label}\n    unchanged (skipped)")
        return False, len(files), total, links

    print(label)
    change = describe(candidates(after) - candidates(before), candidates(before) - candidates(after))
    if change:
        print(f"    {change}")
    print(f"    {len(files)} files, {human(total)}")

    if not dry_run:
        if dest.exists():
            shutil.rmtree(dest)
        for rel, src, _ in files:
            target = dest / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(src, target)
    return True, len(files), total, links


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Collect ___BUGHUNT___ / ___OPTIMIZATIONS___ workspaces into investigations/."
    )
    parser.add_argument("source", help="folder to search for workspaces")
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="report what would be copied and replaced, without touching the disk",
    )
    parser.add_argument(
        "--max-file-size",
        type=parse_size,
        default=DEFAULT_MAX_FILE_SIZE,
        metavar="SIZE",
        help=f"cap for files outside the always-copy list, e.g. 500k (default {DEFAULT_MAX_FILE_SIZE})",
    )
    args = parser.parse_args()

    root = Path(args.source).expanduser().resolve()
    if not root.is_dir():
        sys.exit(f"Not a directory: {root}")

    workspaces = find_workspaces(root)
    if not workspaces:
        sys.exit(f"No {' / '.join(KIND_DIRS)} folders found under {root}")

    if args.dry_run:
        print("dry run - nothing will be written\n")

    files = total = links = changed = 0
    for workspace in sorted(workspaces):
        updated, n, size, skipped = sync(
            workspace, dest_for(workspace, root), args.max_file_size, args.dry_run
        )
        files += n
        total += size
        links += skipped
        changed += updated

    print(f"\n{changed}/{len(workspaces)} workspaces updated, {files} files, {human(total)}")
    if links:
        print(f"{links} symlinks skipped")


if __name__ == "__main__":
    main()
