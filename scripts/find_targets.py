#!/usr/bin/env python3
"""Collect rust-fuzzer (rfuzz) targets with crashes into this repo.

Usage:
    scripts/find_targets.py [<rust-fuzzer-folder>] [--dry-run]

    # default: ~/Projekty/Rust/rust-fuzzer
    scripts/find_targets.py
    scripts/find_targets.py --dry-run

Layout:
    <rust-fuzzer>/targets/<name>          -> fuzz-targets/<name>
    <rust-fuzzer>/targets_reported/<name> -> fuzz-targets/<name> + an empty REPORTED file

    A name present in both is taken from `targets/` (the live one).

What is copied (per target that has at least one crash):
    fuzz.toml, crashes.md and, for every crashes/<bucket>/: README.md, crash.txt,
    Cargo.toml, Cargo.lock and the whole src/ (main.rs plus the data files it
    include_bytes!-es - without them the reproducer does not build).

What is left behind:
    assets/ (hundreds of MB of sample inputs taken from the upstream projects),
    work/ (generated harness and builds), trace.ron / panic.json / call_names.json
    (rfuzz internals, the biggest files by far), *.zip, .rfuzz-auto-seen, *.tsv,
    and targets that found no crashes.

Updating:
    The source is the only truth. A destination folder is deleted and written
    fresh when it differs from its source, and folders of targets that are gone
    from the source are removed. An unchanged destination is left alone.
"""

import argparse
import os
import shutil
import sys
from pathlib import Path

REPO_DIR = Path(__file__).resolve().parent.parent
DEST_DIR = REPO_DIR / "fuzz-targets"
DEFAULT_SOURCE = Path("~/Projekty/Rust/rust-fuzzer").expanduser()
REPORTED_MARKER = "REPORTED"

TARGET_FILES = ("fuzz.toml", "crashes.md")
BUCKET_FILES = ("README.md", "crash.txt", "Cargo.toml", "Cargo.lock")
BUCKET_DIRS = ("src",)


def human(size: int) -> str:
    if size >= 1_000_000:
        return f"{size / 1_000_000:.1f} MB"
    if size >= 1_000:
        return f"{size / 1_000:.1f} KB"
    return f"{size} B"


def buckets(target: Path) -> list[Path]:
    crashes = target / "crashes"
    if not crashes.is_dir():
        return []
    return sorted(p for p in crashes.iterdir() if p.is_dir() and not p.is_symlink())


def collect(target: Path) -> list[tuple[Path, Path, int]]:
    files: list[tuple[Path, Path, int]] = []

    def add(path: Path) -> None:
        if path.is_symlink() or not path.is_file():
            return
        files.append((path.relative_to(target), path, path.stat().st_size))

    for name in TARGET_FILES:
        add(target / name)
    for bucket in buckets(target):
        for name in BUCKET_FILES:
            add(bucket / name)
        for dirname in BUCKET_DIRS:
            for dirpath, dirnames, filenames in os.walk(bucket / dirname):
                dirnames[:] = sorted(d for d in dirnames if not (Path(dirpath) / d).is_symlink())
                for name in sorted(filenames):
                    add(Path(dirpath) / name)
    return files


def fingerprint(dest: Path) -> dict[Path, tuple[int, int]]:
    out: dict[Path, tuple[int, int]] = {}
    for dirpath, _, filenames in os.walk(dest):
        for name in filenames:
            path = Path(dirpath) / name
            st = path.stat()
            out[path.relative_to(dest)] = (st.st_size, st.st_mtime_ns)
    return out


def discover(root: Path) -> dict[str, tuple[Path, bool]]:
    found: dict[str, tuple[Path, bool]] = {}
    for dirname, reported in (("targets_reported", True), ("targets", False)):
        base = root / dirname
        if not base.is_dir():
            continue
        for target in sorted(base.iterdir()):
            if target.is_dir() and not target.is_symlink() and buckets(target):
                found[target.name] = (target, reported)
    return found


def sync(name: str, target: Path, reported: bool, dry_run: bool) -> tuple[bool, int, int, int]:
    files = collect(target)
    total = sum(size for _, _, size in files)
    crashes = len(buckets(target))
    dest = DEST_DIR / name

    before = fingerprint(dest) if dest.exists() else {}
    was_reported = before.pop(Path(REPORTED_MARKER), None) is not None
    after = {rel: (size, src.stat().st_mtime_ns) for rel, src, size in files}
    if before == after and was_reported == reported:
        return False, crashes, len(files), total

    print(f"{dest.relative_to(REPO_DIR)}{' (reported)' if reported else ''}: {crashes} crashes, "
          f"{len(files)} files, {human(total)}")
    if not dry_run:
        if dest.exists():
            shutil.rmtree(dest)
        for rel, src, _ in files:
            out = dest / rel
            out.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(src, out)
        if reported:
            (dest / REPORTED_MARKER).touch()
    return True, crashes, len(files), total


def main() -> None:
    parser = argparse.ArgumentParser(description="Collect rust-fuzzer targets with crashes into fuzz-targets/.")
    parser.add_argument("source", nargs="?", default=str(DEFAULT_SOURCE), help="rust-fuzzer checkout")
    parser.add_argument("--dry-run", action="store_true", help="report changes without touching the disk")
    args = parser.parse_args()

    root = Path(args.source).expanduser().resolve()
    found = discover(root)
    if not found:
        sys.exit(f"No targets with crashes found under {root}")

    if args.dry_run:
        print("dry run - nothing will be written\n")

    changed = crashes = files = total = 0
    for name, (target, reported) in found.items():
        updated, n_crashes, n_files, size = sync(name, target, reported, args.dry_run)
        changed += updated
        crashes += n_crashes
        files += n_files
        total += size

    stale = sorted(p.name for p in DEST_DIR.iterdir() if p.is_dir() and p.name not in found) if DEST_DIR.is_dir() else []
    for name in stale:
        print(f"{(DEST_DIR / name).relative_to(REPO_DIR)}: gone from source, removed")
        if not args.dry_run:
            shutil.rmtree(DEST_DIR / name)

    print(f"\n{changed}/{len(found)} targets updated, {len(stale)} removed; "
          f"{crashes} crashes, {files} files, {human(total)} in total")


if __name__ == "__main__":
    main()
