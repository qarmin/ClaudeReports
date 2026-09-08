#!/usr/bin/env python3
"""verify_report.py - re-check an extracted report against freshly cloned code.

WHY
---
Re-reading every finding with an LLM costs tokens proportional to the number of
findings, and almost all of that is spent on findings nobody has touched since
the audit. This does the same job mechanically and for free: a finding's own
snippet (embedded in the published report, see extract_report.py) is the baseline,
and the only question is whether that block of code still exists in the current
checkout.

ALGORITHM (per location "file:start-end")
-----------------------------------------
1. Strip the renderer's "  123 | " gutter from the baseline snippet, normalize
   whitespace per line (collapse runs, strip ends) and drop blank edges.
2. Look for that block as a contiguous run of lines in the current file. Line
   numbers do not have to match: code merely shifting down (or being reindented)
   is not a change. On a hit, the location is rewritten to the new line range, so
   the refreshed report links to where the code actually is today.
3. If the file is gone or the block is not in it, search the whole checkout for
   the block. Exactly one file containing it means the code moved - the path is
   rewritten. Zero or several matches mean we cannot tell, and the finding drops.
4. A finding lives or dies with its FIRST location - the site its description
   is about. Later locations that no longer resolve are pruned from it, so a
   multi-site finding is not discarded wholesale because one of its spots was
   refactored away.

Deliberately imprecise in one direction only: an unrelated rewrite that happens
to preserve the exact buggy lines keeps the finding alive, so the output can
contain a few findings that were in fact superseded - but a genuinely untouched
finding is never dropped by accident.

SAFEGUARDS
----------
A wrong git URL (or wrong branch) makes every snippet miss, which would silently
"fix" an entire report. Two thresholds guard against that, and tripping either
one means nothing is written and the exit code is 3:
  --max-drop-ratio     (default 0.90) share of findings dropped
  --min-path-presence  (default 0.50) share of referenced paths that exist, and
                       only when more than half the findings dropped too AND
                       relocation did not explain the survivors - a restructured
                       tree loses its paths while keeping its code
Pass --force to write the result anyway (after checking the repo really is the
right one).

USAGE
-----
  verify_report.py --work DIR --clone DIR [--auto-description] [--description TEXT]
  verify_report.py --work DIR --clone DIR --dry-run
  verify_report.py test          # self-test (creates a temp checkout)

OUTPUT (in --work)
------------------
  findings-verified.toml   surviving findings, locations updated, [project] refreshed
  dropped.md               what dropped and why, for eyeballing
  stats.json               counts, thresholds, and the indices still needing a
                           difficulty/type decision (input to difficulty_pass.py)
"""
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from datetime import date
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from report_toml import (  # noqa: E402
    DIFFICULTIES,
    TYPES,
    die,
    dump_findings,
    find_block,
    load_findings,
    normalize,
    parse_loc,
    read_text,
    strip_line_prefixes,
    trim_blank_edges,
)

# Only files that could plausibly hold source are worth scanning when a path moved.
SKIP_DIRS = {".git", "node_modules", "target", "build", ".venv", "__pycache__"}
SKIP_SUFFIXES = {
    ".png", ".jpg", ".jpeg", ".gif", ".ico", ".bmp", ".webp", ".svg", ".pdf",
    ".zip", ".gz", ".xz", ".bz2", ".zst", ".7z", ".tar", ".jar", ".so", ".a",
    ".o", ".dll", ".exe", ".dylib", ".bin", ".dat", ".woff", ".woff2", ".ttf",
    ".mp3", ".mp4", ".wav", ".ogg", ".avi", ".mov", ".pyc", ".class", ".wasm",
}
MAX_SCAN_FILE_BYTES = 4_000_000
# A one-line or near-empty block matches too much to relocate on; require substance.
MIN_SEARCH_LINES = 2
MIN_SEARCH_CHARS = 40
# Missing paths only count as evidence of a wrong checkout above this drop rate,
# and only when relocation did not already explain them.
PATH_GUARD_DROP_RATIO = 0.50
PATH_GUARD_RELOCATED_SHARE = 0.25


def git(clone: Path, *args: str) -> str:
    try:
        out = subprocess.run(["git", "-C", str(clone), *args],
                             capture_output=True, text=True, timeout=60)
    except (OSError, subprocess.SubprocessError):
        return ""
    return out.stdout.strip() if out.returncode == 0 else ""


def evaluate_guards(total: int, drop_ratio: float, path_presence: float,
                    max_drop_ratio: float, min_path_presence: float,
                    kept: int = 0, moved_paths: int = 0) -> list[str]:
    """Which safeguards a result trips, if any.

    Missing paths on their own mean nothing: a project that reorganized its tree
    (cargo moved almost everything into crates/, vello lifted sparse_strips/* to
    the top level) has hardly any of the recorded paths left, yet its code is all
    there. Two things have to go wrong together before that looks like the wrong
    repository: the paths are gone AND most findings dropped AND relocation did
    not account for the survivors. In a genuinely wrong checkout the blocks are
    found nowhere, so `moved_paths` stays near zero.
    """
    tripped = []
    if total and drop_ratio > max_drop_ratio:
        tripped.append(f"drop_ratio {drop_ratio:.2f} > {max_drop_ratio}")
    relocated_share = moved_paths / kept if kept else 0.0
    if (path_presence < min_path_presence and drop_ratio > PATH_GUARD_DROP_RATIO
            and relocated_share < PATH_GUARD_RELOCATED_SHARE):
        tripped.append(f"path_presence {path_presence:.2f} < {min_path_presence} "
                       f"with drop_ratio {drop_ratio:.2f} and only "
                       f"{moved_paths} relocations")
    return tripped


def baseline_block(code: str) -> list[str]:
    return trim_blank_edges(normalize(strip_line_prefixes(code)))


def make_ref(path: str, start: int, count: int) -> str:
    return f"{path}:{start}" if count <= 1 else f"{path}:{start}-{start + count - 1}"


def iter_repo_files(clone: Path):
    for dirpath, dirnames, filenames in os.walk(clone):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        here = Path(dirpath)
        for name in filenames:
            path = here / name
            if path.suffix.lower() in SKIP_SUFFIXES or path.is_symlink():
                continue
            try:
                if path.stat().st_size > MAX_SCAN_FILE_BYTES:
                    continue
            except OSError:
                continue
            yield path


def search_repo(clone: Path, needles: dict[int, list[str]]) -> dict[int, list[tuple[str, int]]]:
    """One pass over the checkout, matching every pending block at once.

    Called only for locations whose file is gone or no longer holds the block, so
    the common case pays nothing. Bailing out at two hits per needle keeps a
    boilerplate block from making us read the whole tree into memory.
    """
    hits: dict[int, list[tuple[str, int]]] = {key: [] for key in needles}
    by_first: dict[str, list[int]] = {}
    for key, block in needles.items():
        by_first.setdefault(block[0], []).append(key)

    for path in iter_repo_files(clone):
        text = read_text(path)
        if text is None:
            continue
        lines = normalize(text.splitlines())
        present = set(lines)
        candidates = {key for first, keys in by_first.items() if first in present for key in keys}
        if not candidates:
            continue
        rel = str(path.relative_to(clone))
        for key in candidates:
            if len(hits[key]) >= 2:
                continue
            for index in find_block(lines, needles[key]):
                hits[key].append((rel, index + 1))
                if len(hits[key]) >= 2:
                    break
    return hits


def readme_description(clone: Path) -> str:
    """First real paragraph of the README, flattened to one line."""
    for name in ("README.md", "README.rst", "README", "README.txt", "readme.md"):
        text = read_text(clone / name)
        if text is None:
            continue
        paragraph: list[str] = []
        for raw in text.splitlines():
            line = raw.strip()
            if not line:
                if paragraph:
                    break
                continue
            if (line.startswith(("#", "=", "-", "*", ">", "|", "!", "[", "<", "```", ":"))
                    or line.startswith("..")):
                continue
            paragraph.append(line)
        if paragraph:
            text = re.sub(r"\s+", " ", " ".join(paragraph)).strip()
            text = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", text)
            return text[:300].rstrip()
    return ""


def gh_description(repo_url: str) -> str:
    m = re.match(r"https?://github\.com/([^/]+)/([^/.]+)", repo_url or "")
    if not m or not shutil.which("gh"):
        return ""
    try:
        out = subprocess.run(
            ["gh", "repo", "view", f"{m.group(1)}/{m.group(2)}", "--json", "description",
             "-q", ".description"],
            capture_output=True, text=True, timeout=30,
        )
    except (OSError, subprocess.SubprocessError):
        return ""
    return out.stdout.strip() if out.returncode == 0 else ""


def verify(work: Path, clone: Path, args) -> dict:
    project, findings = load_findings(work / "findings.toml")
    baseline = json.loads((work / "baseline.json").read_text(encoding="utf-8"))["findings"]
    if len(baseline) != len(findings):
        die(f"baseline.json ({len(baseline)}) and findings.toml ({len(findings)}) disagree; re-run extract_report.py")

    paths = {parse_loc(loc)[0] for f in findings for loc in f["locations"]}
    present = sum(1 for p in paths if (clone / p).exists())
    path_presence = present / len(paths) if paths else 1.0

    # Pass 1: match in the file the finding names.
    resolved: dict[tuple[int, int], str] = {}
    failed: dict[tuple[int, int], str] = {}
    pending: dict[int, list[str]] = {}
    pending_keys: dict[int, tuple[int, int]] = {}
    unverifiable = 0
    whole_file = 0
    for f_idx, (finding, base) in enumerate(zip(findings, baseline)):
        for l_idx, loc in enumerate(base["locations"]):
            key = (f_idx, l_idx)
            path, start, _ = parse_loc(loc["ref"])
            if start is None:
                # A whole-file location has no lines to match, so the file
                # existing is all we can check - and all we may rewrite.
                whole_file += 1
                if (clone / path).is_file():
                    resolved[key] = loc["ref"]
                else:
                    failed[key] = "file gone (location names no lines)"
                continue
            block = baseline_block(loc["code"])
            if not block:
                unverifiable += 1
                if (clone / path).exists():
                    resolved[key] = loc["ref"]
                else:
                    failed[key] = "no snippet and file gone"
                continue
            text = read_text(clone / path) if (clone / path).is_file() else None
            if text is not None:
                indexes = find_block(normalize(text.splitlines()), block)
                if indexes:
                    best = min(indexes, key=lambda i: abs(i + 1 - start))
                    resolved[key] = make_ref(path, best + 1, len(block))
                    continue
            token = len(pending)
            pending[token] = block
            pending_keys[token] = key

    # Pass 2: one repo-wide sweep for everything pass 1 could not place.
    moved_paths = 0
    if pending:
        searchable = {
            token: block for token, block in pending.items()
            if len(block) >= MIN_SEARCH_LINES and sum(len(x) for x in block) >= MIN_SEARCH_CHARS
        }
        hits = search_repo(clone, searchable) if searchable else {}
        for token, block in pending.items():
            key = pending_keys[token]
            found = hits.get(token, [])
            files = {rel for rel, _ in found}
            if len(found) == 1 or len(files) == 1:
                rel, line = found[0]
                resolved[key] = make_ref(rel, line, len(block))
                moved_paths += 1
            elif not found:
                failed[key] = "block not found in checkout"
            else:
                failed[key] = f"block found in {len(files)} files, ambiguous"

    # Pass 3: a finding lives or dies with its FIRST location - the site the
    # description is actually about. Later locations that no longer resolve are
    # pruned from it, so a multi-site finding (duplicate_code, architecture) is
    # not thrown away wholesale because one of its five spots got refactored.
    kept: list[dict] = []
    dropped: list[dict] = []
    refs_updated = 0
    locations_pruned = 0
    findings_pruned = 0
    for f_idx, (finding, base) in enumerate(zip(findings, baseline)):
        count = len(base["locations"])
        reasons = [failed[(f_idx, l)] for l in range(count) if (f_idx, l) in failed]
        if (f_idx, 0) in failed:
            dropped.append({"type": finding["type"], "severity": finding["severity"],
                            "locations": finding["locations"], "reasons": reasons,
                            "description": finding["description"]})
            continue
        new_locs = [resolved[(f_idx, l)] for l in range(count) if (f_idx, l) in resolved]
        if len(new_locs) < count:
            locations_pruned += count - len(new_locs)
            findings_pruned += 1
        refs_updated += sum(1 for old, new in zip(finding["locations"], new_locs) if old != new)
        updated = dict(finding)
        updated["locations"] = new_locs
        kept.append(updated)

    total = len(findings)
    drop_ratio = len(dropped) / total if total else 0.0
    tripped = evaluate_guards(total, drop_ratio, path_presence,
                              args.max_drop_ratio, args.min_path_presence,
                              kept=len(kept), moved_paths=moved_paths)

    commit = git(clone, "rev-parse", "HEAD")
    branch = git(clone, "rev-parse", "--abbrev-ref", "HEAD")
    stats = {
        "name": project.get("name", ""),
        "total": total,
        "kept": len(kept),
        "dropped": len(dropped),
        "drop_ratio": round(drop_ratio, 4),
        "path_presence": round(path_presence, 4),
        "paths_referenced": len(paths),
        "refs_updated": refs_updated,
        "locations_pruned": locations_pruned,
        "findings_pruned": findings_pruned,
        "moved_paths": moved_paths,
        "unverifiable_locations": unverifiable,
        "whole_file_locations": whole_file,
        "commit": commit,
        "branch": branch,
        "tripped": tripped,
        "forced": bool(args.force),
        "needs_difficulty": [i for i, f in enumerate(kept)
                             if f.get("difficulty") not in DIFFICULTIES],
        "needs_type": [i for i, f in enumerate(kept) if f["type"] not in TYPES],
    }

    if args.dry_run:
        return stats

    (work / "stats.json").write_text(json.dumps(stats, ensure_ascii=False, indent=2),
                                     encoding="utf-8")
    lines = [f"# Dropped findings - {project.get('name', '?')}", "",
             f"checked against {commit or '?'} ({branch or '?'})",
             f"{len(dropped)} of {total} findings dropped", ""]
    for item in dropped:
        lines.append(f"## {item['type']} / {item['severity']}")
        lines.append(f"- locations: {', '.join(item['locations'])}")
        lines.append(f"- reason: {'; '.join(item['reasons'])}")
        lines.append(f"- description: {item['description'][:300]}")
        lines.append("")
    (work / "dropped.md").write_text("\n".join(lines), encoding="utf-8")

    if tripped and not args.force:
        return stats

    project = dict(project)
    if commit:
        project["commit"] = commit
        project["commit_short"] = commit[:10]
        if project.get("repo_url"):
            project["commit_url"] = f"{project['repo_url'].rstrip('/')}/commit/{commit}"
    if branch and branch != "HEAD":
        project["branch"] = branch
    project["generated"] = date.today().isoformat()
    description = args.description or project.get("description") or ""
    if not description and args.auto_description:
        description = gh_description(project.get("repo_url", "")) or readme_description(clone)
    project["description"] = description
    dump_findings(work / "findings-verified.toml", project, kept)
    return stats


# ---------- self-test ---------------------------------------------------------

def _self_test() -> None:
    import tempfile

    from report_toml import load_findings as _load

    def sh(*args, cwd):
        subprocess.run(args, cwd=cwd, check=True, capture_output=True)

    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        clone = tmp_path / "clone"
        (clone / "src").mkdir(parents=True)
        # untouched, shifted, reindented, moved, and genuinely rewritten cases
        (clone / "src" / "keep.rs").write_text(
            "// added line\n// added line\nfn keep() {\n    let a = 1;\n}\n", encoding="utf-8")
        (clone / "src" / "reindent.rs").write_text(
            "fn r() {\n\t\tlet b = 2;\n}\n", encoding="utf-8")
        (clone / "src" / "moved_new.rs").write_text(
            "fn moved(input: &[u8]) -> usize {\n"
            "    let checksum = input.iter().map(|b| *b as usize).sum();\n"
            "    checksum.wrapping_mul(31)\n}\n", encoding="utf-8")
        (clone / "src" / "short.rs").write_text("fn s() {\n    ok();\n}\n", encoding="utf-8")
        (clone / "src" / "gone.rs").write_text(
            "fn other() {\n    let z = 99;\n}\n", encoding="utf-8")
        (clone / "README.md").write_text(
            "# demo\n\n[![badge](x)](y)\n\nA tiny demo project for tests.\n", encoding="utf-8")
        sh("git", "init", "-q", "-b", "main", cwd=clone)
        sh("git", "-c", "user.email=t@t", "-c", "user.name=t", "add", ".", cwd=clone)
        sh("git", "-c", "user.email=t@t", "-c", "user.name=t", "commit", "-qm", "init", cwd=clone)

        work = tmp_path / "work"
        work.mkdir()
        findings = [
            {"type": "panic", "severity": "high", "difficulty": "easy", "description": "shifted",
             "locations": ["src/keep.rs:1-3"], "fix": "f"},
            {"type": "logic_error", "severity": "low", "description": "reindented",
             "locations": ["src/reindent.rs:2"], "fix": "f"},
            {"type": "security", "severity": "low", "description": "moved file",
             "locations": ["src/moved_old.rs:1-3"], "fix": "f"},
            {"type": "dead_code", "severity": "low", "difficulty": "medium",
             "description": "really gone", "locations": ["src/gone.rs:1-2"], "fix": "f"},
            {"type": "performance", "severity": "low", "difficulty": "easy",
             "description": "moved but too generic to relocate on",
             "locations": ["src/short_old.rs:1-2"], "fix": "f"},
            {"type": "duplicate_code", "severity": "low", "difficulty": "easy",
             "description": "main site alive, second site refactored away",
             "locations": ["src/keep.rs:1-3", "src/gone.rs:1-2"], "fix": "f"},
            {"type": "duplicate_code", "severity": "low", "difficulty": "easy",
             "description": "main site gone, only a secondary site left",
             "locations": ["src/gone.rs:1-2", "src/keep.rs:1-3"], "fix": "f"},
            {"type": "architecture", "severity": "low", "difficulty": "easy",
             "description": "whole-file location, file still there",
             "locations": ["src/keep.rs"], "fix": "f"},
            {"type": "architecture", "severity": "low", "difficulty": "easy",
             "description": "whole-file location, file deleted",
             "locations": ["src/vanished.rs"], "fix": "f"},
        ]
        dump_findings(work / "findings.toml",
                      {"name": "demo", "description": "",
                       "repo_url": "https://example.com/demo"}, findings)
        (work / "baseline.json").write_text(json.dumps({"findings": [
            {"idx": 0, "locations": [{"ref": "src/keep.rs:1-3",
                                      "code": "    1 | fn keep() {\n    2 |     let a = 1;\n    3 | }"}]},
            {"idx": 1, "locations": [{"ref": "src/reindent.rs:2",
                                      "code": "    2 |     let b = 2;"}]},
            {"idx": 2, "locations": [{"ref": "src/moved_old.rs:1-3", "code":
                "    1 | fn moved(input: &[u8]) -> usize {\n"
                "    2 |     let checksum = input.iter().map(|b| *b as usize).sum();\n"
                "    3 |     checksum.wrapping_mul(31)"}]},
            {"idx": 3, "locations": [{"ref": "src/gone.rs:1-2",
                                      "code": "    1 | fn removed() {\n    2 |     let y = 7;"}]},
            {"idx": 4, "locations": [{"ref": "src/short_old.rs:1-2",
                                      "code": "    1 | fn s() {\n    2 |     ok();"}]},
            {"idx": 5, "locations": [
                {"ref": "src/keep.rs:1-3",
                 "code": "    1 | fn keep() {\n    2 |     let a = 1;\n    3 | }"},
                {"ref": "src/gone.rs:1-2",
                 "code": "    1 | fn removed() {\n    2 |     let y = 7;"}]},
            {"idx": 6, "locations": [
                {"ref": "src/gone.rs:1-2",
                 "code": "    1 | fn removed() {\n    2 |     let y = 7;"},
                {"ref": "src/keep.rs:1-3",
                 "code": "    1 | fn keep() {\n    2 |     let a = 1;\n    3 | }"}]},
            {"idx": 7, "locations": [{"ref": "src/keep.rs", "code": "    1 | // added line"}]},
            {"idx": 8, "locations": [{"ref": "src/vanished.rs", "code": "    1 | gone"}]},
        ]}), encoding="utf-8")

        class Args:
            max_drop_ratio = 0.9
            min_path_presence = 0.5
            force = False
            dry_run = False
            description = None
            auto_description = True

        stats = verify(work, clone, Args())
        assert stats["total"] == 9 and stats["kept"] == 5, stats
        # gone.rs was rewritten; short.rs moved but its block is too generic to
        # relocate on, which is the deliberate bias: never guess a new path. The
        # last finding dies because its FIRST location is the one that vanished.
        assert stats["dropped"] == 4, stats
        # A location naming a whole file has no lines to match; the file simply
        # has to still be there, and its ref is left alone.
        assert stats["whole_file_locations"] == 2, stats
        assert stats["moved_paths"] == 1, stats
        assert stats["refs_updated"] == 3, stats
        assert stats["findings_pruned"] == 1 and stats["locations_pruned"] == 1, stats
        assert stats["needs_difficulty"] == [1, 2], stats
        assert stats["needs_type"] == [2], stats
        assert stats["branch"] == "main" and len(stats["commit"]) == 40, stats
        assert not stats["tripped"], stats

        project, kept = _load(work / "findings-verified.toml")
        assert [f["locations"] for f in kept] == [
            ["src/keep.rs:3-5"], ["src/reindent.rs:2"], ["src/moved_new.rs:1-3"],
            ["src/keep.rs:3-5"], ["src/keep.rs"]], kept
        assert project["description"] == "A tiny demo project for tests.", project
        assert project["commit_url"].endswith(project["commit"]), project
        assert "really gone" in (work / "dropped.md").read_text(encoding="utf-8")

        # Wrong checkout: every snippet misses, so nothing may be written.
        empty = tmp_path / "wrong"
        (empty / "src").mkdir(parents=True)
        (empty / "src" / "unrelated.rs").write_text("fn nope() {}\n", encoding="utf-8")
        sh("git", "init", "-q", "-b", "main", cwd=empty)
        work2 = tmp_path / "work2"
        shutil.copytree(work, work2)
        (work2 / "findings-verified.toml").unlink()
        stats = verify(work2, empty, Args())
        assert stats["kept"] == 0 and stats["tripped"], stats
        assert not (work2 / "findings-verified.toml").exists(), "tripped run must not write output"

    # The guards themselves: a restructured-but-correct checkout (few paths
    # left, few drops) must pass, a wrong checkout must not.
    assert evaluate_guards(245, 0.18, 0.32, 0.90, 0.50, kept=202, moved_paths=143) == []
    assert evaluate_guards(245, 1.00, 0.32, 0.90, 0.50, kept=0, moved_paths=0), "wrong repo must trip"
    assert evaluate_guards(245, 0.60, 0.20, 0.90, 0.50, kept=98, moved_paths=2), \
        "gone paths with no relocations must trip"
    # vello: the tree moved wholesale, so nearly every survivor was relocated.
    assert evaluate_guards(124, 0.58, 0.09, 0.90, 0.50, kept=52, moved_paths=54) == [], \
        "a relocated tree is not a wrong checkout"
    assert evaluate_guards(245, 0.60, 0.90, 0.90, 0.50, kept=98, moved_paths=0) == [], \
        "heavy drops alone are legitimate"

    print("verify_report.py: self-test OK")


def main() -> None:
    if len(sys.argv) >= 2 and sys.argv[1] == "test":
        _self_test()
        return
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--work", required=True, help="dir holding findings.toml + baseline.json")
    ap.add_argument("--clone", required=True, help="fresh checkout of the project")
    ap.add_argument("--max-drop-ratio", type=float, default=0.90)
    ap.add_argument("--min-path-presence", type=float, default=0.50)
    ap.add_argument("--force", action="store_true", help="write output even if a threshold tripped")
    ap.add_argument("--dry-run", action="store_true", help="print stats, write nothing")
    ap.add_argument("--description", default=None, help="project description to record")
    ap.add_argument("--auto-description", action="store_true",
                    help="fill an empty description from gh repo view, else the README")
    args = ap.parse_args()

    work, clone = Path(args.work).resolve(), Path(args.clone).resolve()
    if not (work / "findings.toml").exists():
        die(f"no findings.toml in {work}")
    if not clone.is_dir():
        die(f"not a directory: {clone}")

    stats = verify(work, clone, args)
    print(json.dumps(stats, ensure_ascii=False, indent=2))
    if stats["tripped"] and not args.force:
        raise SystemExit(3)


if __name__ == "__main__":
    main()
