#!/usr/bin/env python3
"""extract_report.py - turn a published report back into a findings.toml.

WHY
---
The reports in this repo are the only surviving copy of most audits: the
___FINDINGS___ workspaces they came from are long deleted. Luckily each report
embeds everything the audit produced - a JSON array of findings (script id="D")
and, for newer ones, the project metadata (script id="PI") - including the exact
code snippet each finding was written against. That makes a published report a
complete, self-contained input: we can rebuild a renderer-ready findings.toml
from it, and use the embedded snippets as the baseline for checking whether the
code has changed since (see verify_report.py).

Only today's schema is accepted. A finding with no `difficulty`, or with a type
outside the current list, is reported in the summary as needing the
difficulty/type pass - never guessed at here. A severity outside the current
list is a hard error: unlike the other two there is nothing sensible to do with
it, and silently rewriting it would misreport how bad a finding is.

USAGE
-----
  extract_report.py <report.html> --out-dir DIR
  extract_report.py test          # self-test (no repo access)

OUTPUT
------
  DIR/findings.toml   [project] + [[finding]] tables, ready for toml_to_report.py
  DIR/baseline.json   per-finding location snippets, input for verify_report.py
  stdout              JSON summary (counts, git_url, indices needing a decision)

Finding order is preserved, so a finding's index is a stable handle for the rest
of the pipeline.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from report_toml import (  # noqa: E402
    DIFFICULTIES,
    SEVERITIES,
    TYPES,
    die,
    dump_findings,
    read_text,
)

REPORT_RE = re.compile(r"^(?P<name>.+)_(?P<date>\d{8})\.html$")
_BLOCK_RE = r'<script type="application/json" id="{}">(.*?)</script>'


def _json_block(html: str, block_id: str):
    m = re.search(_BLOCK_RE.format(block_id), html, re.S)
    if not m:
        return None
    text = m.group(1).strip()
    text = re.sub(r"^const\s+\w+\s*=\s*", "", text).rstrip(";").strip()
    try:
        return json.loads(text)
    except json.JSONDecodeError as exc:
        die(f"block id={block_id!r} is not valid JSON: {exc}")


def parse_report_name(path: Path) -> tuple[str, str]:
    m = REPORT_RE.match(path.name)
    if not m:
        die(f"not a canonical report name (Project_YYYYMMDD.html): {path.name}")
    date = m.group("date")
    return m.group("name"), f"{date[:4]}-{date[4:6]}-{date[6:]}"


def extract(report: Path, out_dir: Path) -> dict:
    html = read_text(report)
    if html is None:
        die(f"cannot read {report}")
    name, audit_date = parse_report_name(report)

    raw = _json_block(html, "D")
    if raw is None:
        die(f"{report.name} has no findings block (script id=\"D\")")
    project = _json_block(html, "PI") or {}
    project.setdefault("name", name)
    project.setdefault("description", "")

    findings: list[dict] = []
    baseline: list[dict] = []
    needs_difficulty: list[int] = []
    needs_type: list[int] = []
    unknown_types: dict[str, int] = {}
    no_locations = 0

    for item in raw:
        idx = len(findings)
        want_type = False
        ftype = item.get("type") or "other"
        if ftype not in TYPES:
            want_type = True
            unknown_types[ftype] = unknown_types.get(ftype, 0) + 1

        severity = item.get("severity") or "medium"
        if severity not in SEVERITIES:
            die(f"{report.name}: finding {idx} has severity {severity!r}, "
                f"which is not one of {sorted(SEVERITIES)}")

        finding = {
            "type": ftype,
            "severity": severity,
            "description": item.get("description") or "",
            "locations": [],
            "fix": item.get("fix") or "",
        }

        difficulty = item.get("difficulty")
        want_difficulty = difficulty not in DIFFICULTIES
        if not want_difficulty:
            finding["difficulty"] = difficulty

        locs = []
        for loc in item.get("locations") or []:
            if isinstance(loc, str):
                locs.append({"ref": loc, "code": ""})
            elif loc.get("ref"):
                locs.append({"ref": loc["ref"], "code": loc.get("code") or ""})
        if not locs:
            no_locations += 1
            continue

        finding["locations"] = [loc["ref"] for loc in locs]
        findings.append(finding)
        baseline.append({"idx": idx, "locations": locs})
        if want_difficulty:
            needs_difficulty.append(idx)
        if want_type:
            needs_type.append(idx)

    out_dir.mkdir(parents=True, exist_ok=True)
    dump_findings(out_dir / "findings.toml", project, findings)
    (out_dir / "baseline.json").write_text(
        json.dumps({"report": report.name, "name": name, "findings": baseline},
                   ensure_ascii=False),
        encoding="utf-8",
    )

    return {
        "report": report.name,
        "name": name,
        "audit_date": audit_date,
        "has_pi": bool(_json_block(html, "PI")),
        "git_url": project.get("git_url", ""),
        "repo_url": project.get("repo_url", ""),
        "branch": project.get("branch", ""),
        "audit_commit": project.get("commit", ""),
        "total": len(findings),
        "dropped_without_locations": no_locations,
        "needs_difficulty": needs_difficulty,
        "needs_type": needs_type,
        "unknown_types": unknown_types,
    }


# ---------- self-test ---------------------------------------------------------

def _self_test() -> None:
    import tempfile

    from report_toml import load_findings, strip_line_prefixes

    findings = [
        {"id": "PANIC_1", "type": "panic", "severity": "critical", "difficulty": "hard",
         "description": "a \"quoted\" thing with `code` and\nnewline", "fix": "do X",
         "locations": [{"ref": "src/a.rs:10-11", "code": "   10 | let a = 1;\n   11 | let b = 2;"}]},
        {"id": "L_1", "type": "logic_error", "severity": "negligible",
         "description": "d", "fix": "f", "locations": [{"ref": "src/b.rs:3", "code": "    3 | ok"}]},
        {"id": "S_1", "type": "security", "severity": "low", "difficulty": "easy",
         "description": "d", "fix": "f", "locations": [{"ref": "src/c.rs:1", "code": "    1 | x"}]},
        {"id": "C_1", "type": "custom", "severity": "high", "difficulty": "medium",
         "description": "d", "fix": "f", "locations": [{"ref": "src/d.rs:1", "code": "    1 | y"}]},
        {"id": "X_1", "type": "dead_code", "severity": "low",
         "description": "no locations", "fix": "f", "locations": []},
    ]
    pi = {"name": "demo", "description": "", "git_url": "https://example.com/demo.git",
          "repo_url": "https://example.com/demo", "commit": "abc123", "branch": "main"}
    html = (
        "<html><body>\n"
        f'<script type="application/json" id="D">{json.dumps(findings)}</script>\n'
        f'<script type="application/json" id="PI">{json.dumps(pi)}</script>\n'
        "</body></html>\n"
    )

    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        report = tmp_path / "demo_20260521.html"
        report.write_text(html, encoding="utf-8")
        summary = extract(report, tmp_path / "work")

        assert summary["total"] == 4, summary
        assert summary["dropped_without_locations"] == 1, summary
        assert summary["audit_date"] == "2026-05-21", summary
        assert summary["git_url"] == "https://example.com/demo.git", summary
        assert summary["needs_difficulty"] == [1], summary
        assert summary["needs_type"] == [2], summary
        assert summary["unknown_types"] == {"security": 1}, summary

        project, out = load_findings(tmp_path / "work" / "findings.toml")
        assert project["name"] == "demo", project
        assert out[0]["description"] == "a \"quoted\" thing with `code` and\nnewline", out[0]
        assert out[0]["difficulty"] == "hard", out[0]
        assert out[1]["severity"] == "negligible", out[1]
        assert "difficulty" not in out[1], "a missing difficulty must stay missing, not be guessed"
        assert out[2]["type"] == "security", "an unknown type is kept verbatim for the type pass"
        assert out[3]["type"] == "custom", "the user's own custom type is valid"

        baseline = json.loads((tmp_path / "work" / "baseline.json").read_text())
        assert [f["idx"] for f in baseline["findings"]] == [0, 1, 2, 3], baseline
        assert strip_line_prefixes(baseline["findings"][0]["locations"][0]["code"]) == \
            ["let a = 1;", "let b = 2;"]

    # A severity outside the schema is a hard error rather than a silent rewrite.
    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        report = tmp_path / "bad_20260521.html"
        bad = [{"id": "X", "type": "panic", "severity": "info", "difficulty": "easy",
                "description": "d", "fix": "f",
                "locations": [{"ref": "src/a.rs:1", "code": "    1 | x"}]}]
        report.write_text(
            f'<script type="application/json" id="D">{json.dumps(bad)}</script>',
            encoding="utf-8")
        try:
            extract(report, tmp_path / "work")
        except SystemExit:
            pass
        else:
            raise AssertionError("severity 'info' must not be accepted")

    # A report with no PI block still yields a usable [project] table.
    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        report = tmp_path / "weezl_20260521.html"
        report.write_text(
            f'<script type="application/json" id="D">{json.dumps(findings[:1])}</script>',
            encoding="utf-8",
        )
        summary = extract(report, tmp_path / "work")
        assert summary["has_pi"] is False, summary
        assert summary["git_url"] == "", summary
        project, _ = load_findings(tmp_path / "work" / "findings.toml")
        assert project == {"name": "weezl"}, project

    print("extract_report.py: self-test OK")


def main() -> None:
    if len(sys.argv) >= 2 and sys.argv[1] == "test":
        _self_test()
        return
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("report", help="published report HTML (Project_YYYYMMDD.html)")
    ap.add_argument("--out-dir", required=True, help="where to write findings.toml + baseline.json")
    args = ap.parse_args()

    summary = extract(Path(args.report).resolve(), Path(args.out_dir).resolve())
    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
