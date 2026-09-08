#!/usr/bin/env python3
"""Shared helpers for the report-refresh pipeline.

Holds the pieces that extract_report.py, verify_report.py and difficulty_pass.py
would otherwise each reimplement:

  - the enums the renderer (code-audit-structured/toml_to_report.py) understands;
  - reading and writing a renderer-ready findings.toml (inline [project] table
    plus [[finding]] tables, order preserved so a finding's index is a stable
    handle across the pipeline);
  - snippet handling: stripping the "  123 | code" prefixes the renderer adds,
    whitespace normalization, and locating a normalized block inside a file.

Requires Python 3.11+ (stdlib tomllib).
"""
from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path

# The schema code-audit-structured writes today; anything else pollutes the type
# filter and goes to the type pass instead of being guessed at.
TYPES = {
    "panic", "overflow_underflow", "memory_leak", "use_after_free", "copy_paste",
    "logic_error", "invalid_comment", "dead_code", "performance",
    "incorrect_error_handling", "api_misuse", "duplicate_code", "architecture",
    "other", "custom",
}
SEVERITIES = {"critical", "high", "medium", "low", "negligible"}
DIFFICULTIES = {"very_easy", "easy", "medium", "hard"}

FINDING_KEY_ORDER = ("type", "severity", "difficulty", "description", "locations", "fix")

_LINE_PREFIX_RE = re.compile(r"^\s*\d+\s\|\s?")
_WS_RE = re.compile(r"[ \t]+")


# ---------- findings.toml I/O ------------------------------------------------

def _toml_str(value: str) -> str:
    out = ["\""]
    for ch in value:
        if ch == "\\":
            out.append("\\\\")
        elif ch == "\"":
            out.append("\\\"")
        elif ch == "\n":
            out.append("\\n")
        elif ch == "\r":
            out.append("\\r")
        elif ch == "\t":
            out.append("\\t")
        elif ord(ch) < 0x20 or ord(ch) == 0x7F:
            out.append(f"\\u{ord(ch):04X}")
        else:
            out.append(ch)
    out.append("\"")
    return "".join(out)


def _toml_value(value) -> str:
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float)):
        return str(value)
    if isinstance(value, (list, tuple)):
        return "[" + ", ".join(_toml_value(v) for v in value) + "]"
    return _toml_str(str(value))


def dump_findings(path: Path, project: dict, findings: list[dict]) -> None:
    """Write a findings.toml the renderer can consume directly.

    Single-line escaped strings only: descriptions routinely contain quotes,
    backticks and triple-quote sequences, and a literal block would need
    per-value escaping analysis to stay valid.
    """
    lines: list[str] = []
    if project:
        lines.append("[project]")
        for key, value in project.items():
            if value not in (None, ""):
                lines.append(f"{key} = {_toml_value(value)}")
        lines.append("")
    for finding in findings:
        lines.append("[[finding]]")
        for key in FINDING_KEY_ORDER:
            if key in finding and finding[key] not in (None, ""):
                lines.append(f"{key} = {_toml_value(finding[key])}")
        lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def load_findings(path: Path) -> tuple[dict, list[dict]]:
    with open(path, "rb") as fh:
        data = tomllib.load(fh)
    project = data.get("project") or {}
    findings = data.get("finding") or []
    return project, findings


# ---------- snippet handling --------------------------------------------------

def parse_loc(loc: str) -> tuple[str, int | None, int | None]:
    """'src/foo.rs:10-25' -> ('src/foo.rs', 10, 25); 'src/foo.rs:10' -> (..., 10, 10).

    A location may legitimately name a whole file with no line part - the
    renderer accepts that - so those come back with `None` line numbers rather
    than raising. Same for anything that does not parse as a line range.
    """
    loc = loc.strip()
    if ":" not in loc:
        return loc, None, None
    file_part, lines = loc.rsplit(":", 1)
    file_part = file_part.strip() or loc
    try:
        if "-" in lines:
            start, end = lines.split("-", 1)
            return file_part, int(start), int(end)
        n = int(lines)
        return file_part, n, n
    except ValueError:
        return loc, None, None


def strip_line_prefixes(code: str) -> list[str]:
    """Drop the renderer's '  123 | ' gutter, but only when it is really there.

    A report's snippet is written by toml_to_report.py as f"{lineno:5d} | {line}",
    so the gutter is present in every published report. Bailing out when it is
    not uniformly present keeps this safe for hand-made or future snippets whose
    code could legitimately start with "12 |".
    """
    raw = code.splitlines()
    nonempty = [ln for ln in raw if ln.strip()]
    if nonempty and all(_LINE_PREFIX_RE.match(ln) for ln in nonempty):
        return [_LINE_PREFIX_RE.sub("", ln) for ln in raw]
    return raw


def normalize(lines: list[str]) -> list[str]:
    """Collapse whitespace runs and strip ends, so reindentation is not a change."""
    return [_WS_RE.sub(" ", ln).strip() for ln in lines]


def trim_blank_edges(lines: list[str]) -> list[str]:
    start, end = 0, len(lines)
    while start < end and not lines[start].strip():
        start += 1
    while end > start and not lines[end - 1].strip():
        end -= 1
    return lines[start:end]


def find_block(haystack: list[str], needle: list[str]) -> list[int]:
    """0-based indices where `needle` occurs as a contiguous run in `haystack`."""
    if not needle or len(needle) > len(haystack):
        return []
    hits = []
    first = needle[0]
    span = len(needle)
    for i in range(len(haystack) - span + 1):
        if haystack[i] == first and haystack[i:i + span] == needle:
            hits.append(i)
    return hits


def read_text(path: Path) -> str | None:
    try:
        return path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return None


def die(msg: str) -> None:
    print(f"ERROR: {msg}", file=sys.stderr)
    raise SystemExit(2)
