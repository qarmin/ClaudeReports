#!/usr/bin/env python3
"""One-shot schema retrofit: bring SUMMARY.md / SUMMARY-SHORT.md up to the current templates.

This workspace was created under an older SUMMARY template, so its tables lack the
"Introduced by"/"Fixed by" (SUMMARY.md) and "Fix lines"/"Introduced by" (SUMMARY-SHORT.md)
columns even though every candidate's RESULT.md already records the introducing commit.
Reads the per-candidate RESULT.md/patch.diff and rewrites both tables in place.
"""

import pathlib
import re
import sys

OPT = pathlib.Path(__file__).resolve().parent.parent
HASH_RE = re.compile(r"\b([0-9a-f]{7,40})\b")
NO_COMMIT_RE = re.compile(r"^\s*(n/?a|not applicable|not determined)", re.I)


def introduced_by(num: str) -> str:
    folders = sorted(OPT.glob(f"{num}.*"))
    if not folders:
        return "not determined"
    result = folders[0] / "RESULT.md"
    if not result.is_file():
        return "not determined"
    body, capture = [], False
    for line in result.read_text().splitlines():
        if line.startswith("## "):
            capture = line.strip().lower().startswith("## introduced by")
            continue
        if capture:
            body.append(line)
    text = " ".join(body).strip()
    if not text:
        return "not determined"
    if NO_COMMIT_RE.match(text):
        return "n/a"
    m = HASH_RE.search(text.replace("`", ""))
    return m.group(1) if m else "not determined"


def fix_lines(num: str) -> str:
    folders = sorted(OPT.glob(f"{num}.*"))
    if not folders:
        return "n/a"
    patch = folders[0] / "patch.diff"
    if not patch.is_file():
        return "n/a"
    added = deleted = 0
    for line in patch.read_text(errors="replace").splitlines():
        if line.startswith("+++") or line.startswith("---"):
            continue
        if line.startswith("+"):
            added += 1
        elif line.startswith("-"):
            deleted += 1
    return f"+{added}/-{deleted}"


def split_row(line: str) -> list[str]:
    return [c.strip() for c in line.strip().strip("|").split("|")]


def rewrite(path: pathlib.Path, header: str, transform) -> int:
    lines = path.read_text().splitlines()
    out, touched = [], 0
    for line in lines:
        if line.startswith("| # |"):
            out.append(header)
            out.append("|" + "---|" * (header.count("|") - 1))
            continue
        if re.match(r"^\|---", line):
            continue  # emitted alongside the header above
        m = re.match(r"^\| (\d{3}) \|", line)
        if not m:
            out.append(line)
            continue
        out.append(transform(m.group(1), split_row(line)))
        touched += 1
    path.write_text("\n".join(out) + "\n")
    return touched


def summary_row(num: str, cells: list[str]) -> str:
    # old: # | Status | Class | Fully reproducible | Candidate | Root cause | Link
    _, status, cls, fully, cand, root, link = cells
    intro = introduced_by(num) if status == "Confirmed" else "n/a"
    return f"| {num} | {status} | {cls} | {fully} | {cand} | {root} | {intro} |  | {link} |"


def short_row(num: str, cells: list[str]) -> str:
    # old: # | Class | Fully reproducible | Candidate | Root cause
    _, cls, _fully, cand, root = cells
    return f"| {num} | {cls} | {cand} | {fix_lines(num)} | {root} | {introduced_by(num)} |"


if __name__ == "__main__":
    n1 = rewrite(
        OPT / "SUMMARY.md",
        "| # | Status | Class | Fully reproducible | Candidate | Root cause (short) | Introduced by | Fixed by | Link |",
        summary_row,
    )
    n2 = rewrite(
        OPT / "SUMMARY-SHORT.md",
        "| # | Class | Candidate | Fix lines | Root cause | Introduced by |",
        short_row,
    )
    print(f"SUMMARY.md: {n1} rows rewritten, SUMMARY-SHORT.md: {n2} rows rewritten", file=sys.stderr)
