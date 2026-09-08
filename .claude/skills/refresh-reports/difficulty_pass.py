#!/usr/bin/env python3
"""difficulty_pass.py - the one step that needs judgement, kept small.

WHY
---
Two things in a report cannot be filled in mechanically: a finding with no
`difficulty`, and a finding whose `type` is outside the schema
code-audit-structured writes today. Those are the only part of a refresh worth
spending tokens on. Everything about this script exists to keep that spend
bounded and resumable:

  - only findings that survived verify_report.py are ever sent out;
  - a batch carries just what a decision needs (type, severity, description,
    fix, location refs) - no code is read, no repo is touched;
  - each batch is one agent call whose answer lands in its own JSON file, so an
    interrupted run resumes by looking at which batch files have answers.

USAGE
-----
  difficulty_pass.py make-batches --toml findings-verified.toml --stats stats.json \
      --out-dir batches [--size 50]
  difficulty_pass.py apply --toml findings-verified.toml --results batches [--dry-run]
  difficulty_pass.py test

ANSWER FORMAT (one file per batch, written next to the batch as <batch>-result.json)
  {"results": [{"idx": 3, "difficulty": "easy"},
               {"idx": 7, "difficulty": "medium", "type": "logic_error"}]}
A bare list is accepted too. Unknown indices and invalid enum values are
reported and ignored, never written.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from report_toml import (  # noqa: E402
    DIFFICULTIES,
    TYPES,
    die,
    dump_findings,
    load_findings,
)

BATCH_FIELDS = ("type", "severity", "description", "fix", "locations")


def make_batches(toml_path: Path, stats_path: Path, out_dir: Path, size: int) -> dict:
    project, findings = load_findings(toml_path)
    stats = json.loads(stats_path.read_text(encoding="utf-8"))
    wanted: dict[int, list[str]] = {}
    for idx in stats.get("needs_difficulty", []):
        wanted.setdefault(idx, []).append("difficulty")
    for idx in stats.get("needs_type", []):
        wanted.setdefault(idx, []).append("type")

    out_dir.mkdir(parents=True, exist_ok=True)
    for stale in out_dir.glob("batch-*.json"):
        stale.unlink()

    indices = sorted(wanted)
    batches = []
    for start in range(0, len(indices), size):
        chunk = indices[start:start + size]
        payload = {
            "toml": str(toml_path),
            "project": project.get("name", ""),
            "findings": [
                {"idx": idx, "needs": wanted[idx],
                 **{k: findings[idx].get(k, "") for k in BATCH_FIELDS}}
                for idx in chunk
            ],
        }
        path = out_dir / f"batch-{len(batches):03d}.json"
        path.write_text(json.dumps(payload, ensure_ascii=False, indent=1), encoding="utf-8")
        batches.append({"file": str(path), "findings": len(chunk),
                        "result_file": str(path.with_name(path.stem + "-result.json"))})

    return {"project": project.get("name", ""), "findings_to_decide": len(indices),
            "batches": batches}


def _load_results(target: Path) -> list[dict]:
    files = sorted(target.glob("batch-*-result.json")) if target.is_dir() else [target]
    results: list[dict] = []
    for path in files:
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as exc:
            die(f"cannot read results {path}: {exc}")
        items = data.get("results") if isinstance(data, dict) else data
        if not isinstance(items, list):
            die(f"{path} has no results list")
        results.extend(items)
    return results


def apply(toml_path: Path, results_path: Path, dry_run: bool) -> dict:
    project, findings = load_findings(toml_path)
    results = _load_results(results_path)

    applied = invalid = unknown = 0
    problems: list[str] = []
    for item in results:
        idx = item.get("idx")
        if not isinstance(idx, int) or not 0 <= idx < len(findings):
            unknown += 1
            problems.append(f"idx {idx!r} out of range")
            continue
        changed = False
        difficulty = item.get("difficulty")
        if difficulty is not None:
            if difficulty in DIFFICULTIES:
                findings[idx]["difficulty"] = difficulty
                changed = True
            else:
                invalid += 1
                problems.append(f"idx {idx}: bad difficulty {difficulty!r}")
        ftype = item.get("type")
        if ftype is not None:
            if ftype in TYPES:
                findings[idx]["type"] = ftype
                changed = True
            else:
                invalid += 1
                problems.append(f"idx {idx}: bad type {ftype!r}")
        applied += int(changed)

    still_missing = [i for i, f in enumerate(findings) if f.get("difficulty") not in DIFFICULTIES]
    unknown_types = [i for i, f in enumerate(findings) if f["type"] not in TYPES]
    if not dry_run:
        dump_findings(toml_path, project, findings)

    return {"results_seen": len(results), "applied": applied, "invalid": invalid,
            "unknown_idx": unknown, "problems": problems[:20],
            "still_missing_difficulty": still_missing, "still_unknown_type": unknown_types}


# ---------- self-test ---------------------------------------------------------

def _self_test() -> None:
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        toml_path = tmp_path / "findings-verified.toml"
        findings = [
            {"type": "panic", "severity": "high", "difficulty": "easy",
             "description": "settled", "locations": ["a.rs:1"], "fix": "f"},
            {"type": "logic_error", "severity": "low", "description": "needs difficulty",
             "locations": ["b.rs:2"], "fix": "f"},
            {"type": "security", "severity": "high", "description": "needs both",
             "locations": ["c.rs:3"], "fix": "f"},
        ]
        dump_findings(toml_path, {"name": "demo"}, findings)
        (tmp_path / "stats.json").write_text(
            json.dumps({"needs_difficulty": [1, 2], "needs_type": [2]}), encoding="utf-8")

        summary = make_batches(toml_path, tmp_path / "stats.json", tmp_path / "batches", 1)
        assert summary["findings_to_decide"] == 2, summary
        assert len(summary["batches"]) == 2, summary
        first = json.loads(Path(summary["batches"][0]["file"]).read_text())
        assert first["findings"][0]["idx"] == 1, first
        assert first["findings"][0]["needs"] == ["difficulty"], first
        assert "code" not in json.dumps(first), "batches must not carry source code"
        second = json.loads(Path(summary["batches"][1]["file"]).read_text())
        assert sorted(second["findings"][0]["needs"]) == ["difficulty", "type"], second

        Path(summary["batches"][0]["result_file"]).write_text(
            json.dumps({"results": [{"idx": 1, "difficulty": "hard"}]}), encoding="utf-8")
        Path(summary["batches"][1]["result_file"]).write_text(
            json.dumps([{"idx": 2, "difficulty": "nope", "type": "api_misuse"},
                        {"idx": 99, "difficulty": "easy"}]), encoding="utf-8")

        report = apply(toml_path, tmp_path / "batches", dry_run=False)
        assert report["applied"] == 2, report
        assert report["invalid"] == 1 and report["unknown_idx"] == 1, report
        assert report["still_missing_difficulty"] == [2], report
        assert report["still_unknown_type"] == [], report

        _, out = load_findings(toml_path)
        assert out[1]["difficulty"] == "hard", out[1]
        assert out[2]["type"] == "api_misuse", out[2]
        assert "difficulty" not in out[2], out[2]

    print("difficulty_pass.py: self-test OK")


def main() -> None:
    if len(sys.argv) >= 2 and sys.argv[1] == "test":
        _self_test()
        return
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)

    mk = sub.add_parser("make-batches", help="write agent-sized batches of undecided findings")
    mk.add_argument("--toml", required=True)
    mk.add_argument("--stats", required=True)
    mk.add_argument("--out-dir", required=True)
    mk.add_argument("--size", type=int, default=50)

    ap_apply = sub.add_parser("apply", help="merge agent answers back into the TOML")
    ap_apply.add_argument("--toml", required=True)
    ap_apply.add_argument("--results", required=True, help="result file or dir of batch results")
    ap_apply.add_argument("--dry-run", action="store_true")

    args = ap.parse_args()
    if args.cmd == "make-batches":
        out = make_batches(Path(args.toml).resolve(), Path(args.stats).resolve(),
                           Path(args.out_dir).resolve(), args.size)
    else:
        out = apply(Path(args.toml).resolve(), Path(args.results).resolve(), args.dry_run)
    print(json.dumps(out, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
