#!/usr/bin/env python3
"""refresh.py - drive the mechanical part of refreshing published reports.

WHY
---
Everything a refresh needs except judgement is deterministic: pick the next
reports, clone the project shallowly, rebuild a findings.toml from the report,
check every finding's snippet against the fresh checkout, re-render, replace the
file in the repo. Doing that here (instead of in the agent) keeps a batch of ten
projects at a handful of tool calls, and makes the run resumable: state is
written after every single project, so an interrupted run never repeats work or
loses more than the project it died on.

The agent only steps in twice: to fill in a git URL that cannot be resolved, and
to decide `difficulty` (plus any off-schema type) for findings that survived
verification - see difficulty_pass.py.

COMMANDS
--------
  refresh.py list     [--limit 10]            what the next batch would be
  refresh.py prepare  [--limit 10]            clone, extract, verify, render what it can
  refresh.py finish   [--only NAME ...]       apply agent answers, render, install
  refresh.py status                           progress across the whole repo
  refresh.py test                             self-test (no network)

A project whose findings all carry a difficulty is rendered and installed by
`prepare` alone. Otherwise `prepare` leaves it at `awaiting-difficulty` with
batches written and the checkout kept, and `finish` completes it.

STATE
-----
  tmp/refresh-state.json   per-run progress (gitignored, safe to delete)
  projects.toml            durable per-project facts, committed: git URL, branch,
                           first audit date, last refresh, verified commit
"""
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tomllib
from dataclasses import dataclass
from datetime import date, timedelta, datetime
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import difficulty_pass  # noqa: E402
import verify_report  # noqa: E402
from extract_report import REPORT_RE, extract  # noqa: E402
from report_toml import dump_findings, load_findings  # noqa: E402

DEFAULT_RENDERER = Path.home() / ".claude/skills/code-audit-structured/toml_to_report.py"
CLONE_TIMEOUT = 3600
FRESH_DAYS = 30
BATCH_SIZE = 50


@dataclass
class Ctx:
    repo: Path
    renderer: Path = DEFAULT_RENDERER

    @property
    def reports_dir(self) -> Path:
        return self.repo / "reports"

    def report_files(self) -> list[Path]:
        """Reports live in reports/; pinned old ones (projects.toml `keep`) stay in the repo root."""
        return sorted([*self.repo.glob("*.html"), *self.reports_dir.glob("*.html")])

    @property
    def tmp(self) -> Path:
        return self.repo / "tmp"

    @property
    def state_path(self) -> Path:
        return self.tmp / "refresh-state.json"

    @property
    def projects_path(self) -> Path:
        return self.repo / "projects.toml"

    def work(self, name: str) -> Path:
        return self.tmp / "work" / name

    def clone(self, name: str) -> Path:
        return self.tmp / "clones" / name


# ---------- durable project facts --------------------------------------------

def _toml_key(name: str) -> str:
    return '"' + name.replace("\\", "\\\\").replace('"', '\\"') + '"'


def load_projects(ctx: Ctx) -> dict[str, dict]:
    if not ctx.projects_path.exists():
        return {}
    with open(ctx.projects_path, "rb") as fh:
        return tomllib.load(fh)


def save_projects(ctx: Ctx, projects: dict[str, dict]) -> None:
    lines = [
        "# Durable per-project facts for .claude/skills/refresh-reports.",
        "# git_url is what a refresh clones; fill it in by hand when a report",
        "# predates embedded project metadata and the name is ambiguous.",
        "",
    ]
    for name in sorted(projects, key=str.lower):
        lines.append(f"[{_toml_key(name)}]")
        for key in ("git_url", "repo_url", "branch", "first_audit", "last_refresh",
                    "verified_commit", "note"):
            value = projects[name].get(key)
            if value:
                lines.append(f'{key} = "{value}"')
        keep = projects[name].get("keep")
        if keep:
            lines.append("keep = [" + ", ".join(f'"{k}"' for k in keep) + "]")
        lines.append("")
    ctx.projects_path.write_text("\n".join(lines), encoding="utf-8")


def load_state(ctx: Ctx) -> dict:
    if not ctx.state_path.exists():
        return {"projects": {}}
    try:
        return json.loads(ctx.state_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {"projects": {}}


def save_state(ctx: Ctx, state: dict) -> None:
    state["updated"] = datetime.now().isoformat(timespec="seconds")
    ctx.state_path.parent.mkdir(parents=True, exist_ok=True)
    ctx.state_path.write_text(json.dumps(state, ensure_ascii=False, indent=1), encoding="utf-8")


def set_status(ctx: Ctx, state: dict, name: str, status: str, **fields) -> None:
    entry = state["projects"].setdefault(name, {})
    entry["status"] = status
    entry["at"] = datetime.now().isoformat(timespec="seconds")
    entry.update(fields)
    save_state(ctx, state)


# ---------- report discovery --------------------------------------------------

def reports(ctx: Ctx) -> dict[str, tuple[Path, str]]:
    """name -> (newest report path, its date as YYYY-MM-DD)."""
    found: dict[str, tuple[Path, str]] = {}
    for path in ctx.report_files():
        m = REPORT_RE.match(path.name)
        if not m:
            continue
        name, raw = m.group("name"), m.group("date")
        iso = f"{raw[:4]}-{raw[4:6]}-{raw[6:]}"
        if name not in found or iso > found[name][1]:
            found[name] = (path, iso)
    return found


def days_since(iso: str) -> int:
    try:
        return (date.today() - date.fromisoformat(iso)).days
    except ValueError:
        return 10**6


def select(ctx: Ctx, limit: int | None, only: list[str], fresh_days: int) -> list[str]:
    projects = load_projects(ctx)
    state = load_state(ctx)
    picked = []
    for name, (_, report_date) in sorted(reports(ctx).items(), key=lambda kv: kv[0].lower()):
        if only:
            if name in only:
                picked.append(name)
            continue
        entry = state["projects"].get(name, {})
        if entry.get("status") == "awaiting-difficulty":
            continue
        last = projects.get(name, {}).get("last_refresh") or report_date
        if days_since(last) < fresh_days:
            continue
        if limit is not None and len(picked) >= limit:
            break
        picked.append(name)
    return picked


# ---------- steps -------------------------------------------------------------

def run(cmd: list[str], timeout: int) -> tuple[int, str]:
    try:
        out = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return 124, f"timeout after {timeout}s"
    except OSError as exc:
        return 127, str(exc)
    return out.returncode, (out.stderr or out.stdout).strip()[-2000:]


_SSH_URL_RE = re.compile(r"^(?:ssh://)?git@([^:/]+)[:/](.+?)(?:\.git)?/?$")


def normalize_git_url(url: str) -> str:
    """SSH remotes to HTTPS.

    Audits run from a developer checkout record whatever remote that checkout
    used, and an `git@github.com:owner/repo.git` is unusable here - there are no
    keys and no known_hosts, so the clone dies on host key verification.
    """
    m = _SSH_URL_RE.match(url.strip())
    return f"https://{m.group(1)}/{m.group(2)}.git" if m else url


def init_submodules(dest: Path) -> None:
    """Fetch submodules when the project keeps its code in them.

    st-mems-rust-drivers is one sensor crate per submodule, so a plain shallow
    clone looks like a project whose every file vanished.
    """
    if (dest / ".gitmodules").exists():
        run(["git", "-C", str(dest), "submodule", "update", "--init",
             "--recursive", "--depth", "1"], CLONE_TIMEOUT)


def clone_repo(ctx: Ctx, name: str, git_url: str, branch: str | None) -> tuple[bool, str]:
    dest = ctx.clone(name)
    if dest.exists():
        shutil.rmtree(dest)
    dest.parent.mkdir(parents=True, exist_ok=True)
    git_url = normalize_git_url(git_url)
    cmd = ["git", "clone", "--depth", "1", "--single-branch"]
    if branch:
        cmd += ["--branch", branch]
    cmd += [git_url, str(dest)]
    code, err = run(cmd, CLONE_TIMEOUT)
    if code != 0 and branch:
        # The recorded branch may have been renamed or deleted; the default one is
        # still a fair basis for verification.
        code, err = run(["git", "clone", "--depth", "1", "--single-branch", git_url, str(dest)],
                        CLONE_TIMEOUT)
    if code == 0:
        init_submodules(dest)
    return code == 0, err


def render(ctx: Ctx, name: str, toml_path: Path, clone: Path) -> tuple[Path | None, str]:
    code, err = run([sys.executable, str(ctx.renderer), str(toml_path), "--root", str(clone)], 900)
    out = toml_path.with_name(toml_path.stem + "-interactive.html")
    if code != 0 or not out.exists():
        return None, err or "renderer produced no output"
    return out, ""


def install(ctx: Ctx, name: str, html: Path | None, keep: list[str] | None = None) -> str | None:
    """Put the refreshed report in the repo under today's date, dropping older ones.

    A report listed in the project's `keep` is left alone: some old versions are
    linked to from outside this repo, so replacing them would break those links.
    """
    protected = set(keep or ())
    for old in ctx.report_files():
        m = REPORT_RE.match(old.name)
        if m and m.group("name") == name and old.name not in protected:
            old.unlink()
    if html is None:
        return None
    ctx.reports_dir.mkdir(exist_ok=True)
    target = ctx.reports_dir / f"{name}_{date.today().strftime('%Y%m%d')}.html"
    shutil.copy2(html, target)
    return target.name


def finalize(ctx: Ctx, state: dict, projects: dict, name: str, work: Path,
             stats: dict, audit_date: str, keep_clone: bool) -> dict:
    toml_path = work / "findings-verified.toml"
    _, kept = load_findings(toml_path)
    clone = ctx.clone(name)

    keep = projects.get(name, {}).get("keep") or []
    if kept:
        html, err = render(ctx, name, toml_path, clone)
        if html is None:
            set_status(ctx, state, name, "render-failed", error=err)
            return {"name": name, "status": "render-failed", "error": err}
        installed = install(ctx, name, html, keep)
        status = "done"
    else:
        install(ctx, name, None, keep)
        installed = None
        status = "done-empty"

    entry = projects.setdefault(name, {})
    entry.setdefault("first_audit", audit_date)
    entry["last_refresh"] = date.today().isoformat()
    if stats.get("commit"):
        entry["verified_commit"] = stats["commit"]
    if stats.get("branch") and stats["branch"] != "HEAD":
        entry["branch"] = stats["branch"]
    entry.pop("note", None)
    save_projects(ctx, projects)

    if not keep_clone and clone.exists():
        shutil.rmtree(clone)
    set_status(ctx, state, name, status, report=installed,
               kept=stats.get("kept"), total=stats.get("total"),
               commit=stats.get("commit"))
    return {"name": name, "status": status, "report": installed,
            "total": stats.get("total"), "kept": stats.get("kept"),
            "dropped": stats.get("dropped"), "refs_updated": stats.get("refs_updated"),
            "moved_paths": stats.get("moved_paths"),
            "locations_pruned": stats.get("locations_pruned"),
            "findings_pruned": stats.get("findings_pruned")}


class VerifyArgs:
    def __init__(self, **kw):
        self.max_drop_ratio = kw.get("max_drop_ratio", 0.90)
        self.min_path_presence = kw.get("min_path_presence", 0.50)
        self.force = kw.get("force", False)
        self.dry_run = False
        self.description = kw.get("description")
        self.auto_description = kw.get("auto_description", True)


def prepare_one(ctx: Ctx, name: str, args) -> dict:
    state, projects = load_state(ctx), load_projects(ctx)
    report, report_date = reports(ctx)[name]
    work = ctx.work(name)
    if work.exists():
        shutil.rmtree(work)

    summary = extract(report, work)
    known = projects.get(name, {})
    git_url = known.get("git_url") or summary["git_url"]
    if not git_url:
        entry = projects.setdefault(name, {})
        entry.setdefault("first_audit", summary["audit_date"])
        entry.setdefault("note", "git_url unknown - fill in by hand, then re-run prepare")
        save_projects(ctx, projects)
        set_status(ctx, state, name, "needs-url", report=report.name)
        return {"name": name, "status": "needs-url", "report": report.name,
                "findings": summary["total"]}

    git_url = normalize_git_url(git_url)
    repo_url = known.get("repo_url") or summary["repo_url"] or re.sub(r"\.git$", "", git_url)
    branch = known.get("branch") or summary["branch"] or None
    entry = projects.setdefault(name, {})
    entry.update({"git_url": git_url, "repo_url": repo_url})
    entry.setdefault("first_audit", summary["audit_date"])
    if branch:
        entry["branch"] = branch
    save_projects(ctx, projects)

    # Reports predating embedded metadata carry no repo URL, and without it the
    # renderer emits no permalinks and no repo link in the header.
    project, findings = load_findings(work / "findings.toml")
    project.update({"name": name, "git_url": git_url, "repo_url": repo_url})
    if branch:
        project["branch"] = branch
    dump_findings(work / "findings.toml", project, findings)

    ok, err = clone_repo(ctx, name, git_url, branch)
    if not ok:
        set_status(ctx, state, name, "clone-failed", error=err, git_url=git_url)
        return {"name": name, "status": "clone-failed", "git_url": git_url, "error": err}

    stats = verify_report.verify(work, ctx.clone(name),
                                 VerifyArgs(force=args.force,
                                            max_drop_ratio=args.max_drop_ratio,
                                            min_path_presence=args.min_path_presence))
    if stats["tripped"] and not args.force:
        shutil.rmtree(ctx.clone(name), ignore_errors=True)
        note = "; ".join(stats["tripped"])
        entry["note"] = f"verification tripped: {note}"
        save_projects(ctx, projects)
        set_status(ctx, state, name, "needs-decision", tripped=stats["tripped"],
                   git_url=git_url, kept=stats["kept"], total=stats["total"])
        return {"name": name, "status": "needs-decision", "git_url": git_url,
                "tripped": stats["tripped"], "total": stats["total"],
                "kept": stats["kept"], "path_presence": stats["path_presence"],
                "report": report.name}

    undecided = stats["needs_difficulty"] or stats["needs_type"]
    if not undecided:
        return finalize(ctx, state, projects, name, work, stats,
                        summary["audit_date"], args.keep_clone)

    batches = difficulty_pass.make_batches(work / "findings-verified.toml", work / "stats.json",
                                           work / "batches", args.size)
    set_status(ctx, state, name, "awaiting-difficulty", total=stats["total"],
               kept=stats["kept"], to_decide=batches["findings_to_decide"],
               audit_date=summary["audit_date"])
    return {"name": name, "status": "awaiting-difficulty", "total": stats["total"],
            "kept": stats["kept"], "dropped": stats["dropped"],
            "refs_updated": stats["refs_updated"], "moved_paths": stats["moved_paths"],
            "locations_pruned": stats["locations_pruned"],
            "findings_pruned": stats["findings_pruned"],
            "to_decide": batches["findings_to_decide"], "batches": batches["batches"]}


def missing_batch_results(work: Path) -> list[str]:
    """Batches that were never answered.

    Finalizing with a batch still unanswered silently renders those findings at
    the default difficulty, which looks like a real label but is not one.
    """
    batches = work / "batches"
    if not batches.is_dir():
        return []
    return [b.name for b in sorted(batches.glob("batch-*.json"))
            if not b.name.endswith("-result.json")
            and not b.with_name(b.stem + "-result.json").exists()]


def finish_one(ctx: Ctx, name: str, args) -> dict:
    state, projects = load_state(ctx), load_projects(ctx)
    entry = state["projects"].get(name, {})
    if entry.get("status") != "awaiting-difficulty":
        return {"name": name, "status": entry.get("status", "unknown"),
                "error": "not awaiting difficulty"}
    work = ctx.work(name)
    pending = missing_batch_results(work)
    if pending and not getattr(args, "force", False):
        return {"name": name, "status": "awaiting-difficulty",
                "error": f"unanswered batches: {', '.join(pending)}"}
    applied = difficulty_pass.apply(work / "findings-verified.toml", work / "batches", False)
    stats = json.loads((work / "stats.json").read_text(encoding="utf-8"))
    out = finalize(ctx, state, projects, name, work, stats,
                   entry.get("audit_date", ""), args.keep_clone)
    out["difficulty"] = {k: applied[k] for k in
                         ("applied", "invalid", "unknown_idx", "still_missing_difficulty")}
    return out


def dir_size(path: Path) -> int:
    total = 0
    for dirpath, _, filenames in os.walk(path):
        for name in filenames:
            try:
                total += (Path(dirpath) / name).stat().st_size
            except OSError:
                pass
    return total


# ---------- commands ----------------------------------------------------------

def cmd_list(ctx: Ctx, args) -> dict:
    all_reports = reports(ctx)
    picked = select(ctx, args.limit, args.only, args.fresh_days)
    return {"reports_total": len(all_reports), "selected": picked}


def cmd_prepare(ctx: Ctx, args) -> dict:
    results = []
    for name in select(ctx, args.limit, args.only, args.fresh_days):
        try:
            results.append(prepare_one(ctx, name, args))
        except (SystemExit, Exception) as exc:
            state = load_state(ctx)
            set_status(ctx, state, name, "failed", error=str(exc))
            results.append({"name": name, "status": "failed", "error": str(exc)})
    clones = ctx.tmp / "clones"
    return {"processed": len(results), "results": results,
            "clones_on_disk_mb": round(dir_size(clones) / 1e6, 1) if clones.exists() else 0.0}


def cmd_finish(ctx: Ctx, args) -> dict:
    state = load_state(ctx)
    names = args.only or [n for n, e in state["projects"].items()
                          if e.get("status") == "awaiting-difficulty"]
    return {"results": [finish_one(ctx, name, args) for name in names]}


def cmd_status(ctx: Ctx, args) -> dict:
    state = load_state(ctx)
    projects = load_projects(ctx)
    counts: dict[str, int] = {}
    for entry in state["projects"].values():
        counts[entry.get("status", "?")] = counts.get(entry.get("status", "?"), 0) + 1
    refreshed = [n for n, p in projects.items() if p.get("last_refresh")]
    stale = select(ctx, None, [], args.fresh_days)
    return {"reports_total": len(reports(ctx)), "refreshed_ever": len(refreshed),
            "pending": len(stale), "by_status": counts,
            "awaiting_difficulty": [n for n, e in state["projects"].items()
                                    if e.get("status") == "awaiting-difficulty"],
            "needs_url": [n for n, e in state["projects"].items()
                          if e.get("status") == "needs-url"],
            "needs_decision": [n for n, e in state["projects"].items()
                               if e.get("status") == "needs-decision"]}


# ---------- self-test ---------------------------------------------------------

def _self_test() -> None:
    import tempfile

    from report_toml import dump_findings

    with tempfile.TemporaryDirectory() as tmp:
        ctx = Ctx(repo=Path(tmp))
        (ctx.repo / "tmp").mkdir()
        ctx.reports_dir.mkdir()

        for name in ("weezl_20260521.html", "weezl_20260101.html", f"Zed_{(date.today() - timedelta(days=5)).strftime('%Y%m%d')}.html",
                     "alpha_20260301.html", "not-a-report.html"):
            (ctx.reports_dir / name).write_text("x", encoding="utf-8")

        found = reports(ctx)
        assert set(found) == {"weezl", "Zed", "alpha"}, found
        assert found["weezl"][1] == "2026-05-21", found
        # Alphabetical, case-insensitive, and today's fresh reports skipped.
        assert select(ctx, 10, [], FRESH_DAYS) == ["alpha", "weezl"], select(ctx, 10, [], FRESH_DAYS)
        assert select(ctx, 1, [], FRESH_DAYS) == ["alpha"]
        assert select(ctx, 10, ["weezl"], FRESH_DAYS) == ["weezl"]

        projects = {"weezl": {"git_url": "https://example.com/w.git",
                              "last_refresh": date.today().isoformat()},
                    "ik_llama.cpp": {"git_url": "https://example.com/i.git"}}
        save_projects(ctx, projects)
        again = load_projects(ctx)
        assert again == projects, again
        assert select(ctx, 10, [], FRESH_DAYS) == ["alpha"], "a fresh last_refresh must skip"

        state = load_state(ctx)
        set_status(ctx, state, "weezl", "awaiting-difficulty", kept=3)
        assert load_state(ctx)["projects"]["weezl"]["kept"] == 3
        assert select(ctx, 10, [], 0) == ["alpha", "Zed"], "awaiting-difficulty is not re-picked"

        # install() replaces every older file for the project, whatever its date.
        work = ctx.work("weezl")
        work.mkdir(parents=True)
        dump_findings(work / "findings-verified.toml", {"name": "weezl"}, [])
        html = work / "findings-verified-interactive.html"
        html.write_text("<html>fresh</html>", encoding="utf-8")
        installed = install(ctx, "weezl", html)
        assert installed == f"weezl_{date.today().strftime('%Y%m%d')}.html", installed
        assert (ctx.reports_dir / installed).read_text() == "<html>fresh</html>"
        assert not (ctx.reports_dir / "weezl_20260521.html").exists()
        assert not (ctx.reports_dir / "weezl_20260101.html").exists()
        assert (ctx.reports_dir / "alpha_20260301.html").exists(), "other projects untouched"

        # An unanswered batch must block finalizing, or its findings quietly
        # render at the default difficulty.
        batches = work / "batches"
        batches.mkdir()
        (batches / "batch-000.json").write_text("{}", encoding="utf-8")
        (batches / "batch-001.json").write_text("{}", encoding="utf-8")
        assert missing_batch_results(work) == ["batch-000.json", "batch-001.json"]
        (batches / "batch-000-result.json").write_text("{}", encoding="utf-8")
        assert missing_batch_results(work) == ["batch-001.json"]
        (batches / "batch-001-result.json").write_text("{}", encoding="utf-8")
        assert missing_batch_results(work) == []

        # An empty report is removed from the repo instead of being published.
        assert install(ctx, "alpha", None) is None

        assert normalize_git_url("git@github.com:owner/repo.git") == "https://github.com/owner/repo.git"
        assert normalize_git_url("ssh://git@gitlab.gnome.org/GNOME/glib") == \
            "https://gitlab.gnome.org/GNOME/glib.git"
        assert normalize_git_url("https://github.com/owner/repo.git") == \
            "https://github.com/owner/repo.git"
        assert not (ctx.reports_dir / "alpha_20260301.html").exists()

    print("refresh.py: self-test OK")


def main() -> None:
    if len(sys.argv) >= 2 and sys.argv[1] == "test":
        _self_test()
        return

    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--repo", default=str(Path(__file__).resolve().parents[3]),
                    help="report repo root (default: this skill's repo)")
    ap.add_argument("--renderer", default=str(DEFAULT_RENDERER),
                    help="path to code-audit-structured/toml_to_report.py")
    sub = ap.add_subparsers(dest="cmd", required=True)

    for name, help_text in (("list", "show the next batch"),
                            ("prepare", "clone, extract, verify, render what it can"),
                            ("finish", "apply agent answers and install"),
                            ("status", "progress across the repo")):
        p = sub.add_parser(name, help=help_text)
        p.add_argument("--only", nargs="*", default=[], metavar="NAME")
        p.add_argument("--fresh-days", type=int, default=FRESH_DAYS)
        if name in ("list", "prepare"):
            p.add_argument("--limit", type=int, default=10)
        if name in ("prepare", "finish"):
            p.add_argument("--keep-clone", action="store_true")
        if name == "finish":
            p.add_argument("--force", action="store_true",
                           help="finalize even with unanswered difficulty batches")
        if name == "prepare":
            p.add_argument("--size", type=int, default=BATCH_SIZE,
                           help="findings per difficulty batch")
            p.add_argument("--force", action="store_true",
                           help="write results even when a safeguard trips")
            p.add_argument("--max-drop-ratio", type=float, default=0.90)
            p.add_argument("--min-path-presence", type=float, default=0.50)

    args = ap.parse_args()
    ctx = Ctx(repo=Path(args.repo).resolve(), renderer=Path(args.renderer).resolve())
    if not ctx.repo.is_dir():
        sys.exit(f"not a directory: {ctx.repo}")

    handler = {"list": cmd_list, "prepare": cmd_prepare,
               "finish": cmd_finish, "status": cmd_status}[args.cmd]
    print(json.dumps(handler(ctx, args), ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
