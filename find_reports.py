#!/usr/bin/env python3
"""Collect code-audit reports into this repo.

Recursively searches a source folder for `findings-interactive.html` files
(produced by the code-audit-structured skill) and copies each one here under
the canonical `ProjectName_YYYYMMDD.html` name.

Name resolution:
    .../<Project>/___FINDINGS___/findings-interactive.html -> <Project>
    .../<Project>/findings-interactive.html                -> <Project>
The date is the file's modification time (when the report was generated).

Per-project rules:
    - identical content already here -> skip (never overwrite)
    - older version(s) here          -> delete them, copy the new one in
    - newer version already here     -> skip (don't downgrade)

Usage:
    find_reports.py <source-folder>
"""

import hashlib
import re
import shutil
import sys
from datetime import datetime
from pathlib import Path

REPO_DIR = Path(__file__).resolve().parent
SOURCE_NAME = "findings-interactive.html"
# Canonical repo file: <name>_<YYYYMMDD>.html. The name may itself contain
# underscores (e.g. ik_llama.cpp), so the date anchors the split at the end.
REPO_RE = re.compile(r"^(?P<name>.+)_(?P<date>\d{8})\.html$")


def project_name(report: Path) -> str:
    parent = report.parent
    return (parent.parent if parent.name == "___FINDINGS___" else parent).name


def report_date(report: Path) -> str:
    return datetime.fromtimestamp(report.stat().st_mtime).strftime("%Y%m%d")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def existing_versions(project: str) -> list[tuple[Path, str]]:
    versions = []
    for path in REPO_DIR.glob("*.html"):
        m = REPO_RE.match(path.name)
        if m and m.group("name") == project:
            versions.append((path, m.group("date")))
    return versions


def collect(source_root: Path) -> None:
    reports = sorted(source_root.rglob(SOURCE_NAME))
    if not reports:
        print(f"No {SOURCE_NAME} files found under {source_root}")
        return

    for report in reports:
        project = project_name(report)
        date = report_date(report)
        target = REPO_DIR / f"{project}_{date}.html"
        new_hash = sha256(report)
        versions = existing_versions(project)

        if any(sha256(p) == new_hash for p, _ in versions):
            print(f"skip (identical): {project} ({date})")
            continue

        newer = [(p, d) for p, d in versions if d > date]
        if newer:
            kept = ", ".join(sorted(d for _, d in newer))
            print(f"skip (newer present {kept}): {project} ({date})")
            continue

        for p, d in versions:
            if d < date:
                p.unlink()
                print(f"removed older: {p.name}")

        shutil.copy2(report, target)
        print(f"added: {target.name}  <- {report}")


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit(f"Usage: {Path(sys.argv[0]).name} <source-folder>")
    source_root = Path(sys.argv[1]).expanduser().resolve()
    if not source_root.is_dir():
        sys.exit(f"Not a directory: {source_root}")
    collect(source_root)


if __name__ == "__main__":
    main()
