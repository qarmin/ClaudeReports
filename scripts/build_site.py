#!/usr/bin/env python3
"""Build the GitHub Pages site.

One entry per project, with equal tabs for every kind of material held on it: audit
report (<Name>_YYYYMMDD.html), bughunt / perf summaries (investigations/) and fuzz
crashes (fuzz-targets/). Summaries and crashes are parsed into JSON and rendered
client-side (scripts/site/); patches, reproducers etc. stay on GitHub and are linked.
"""
import html
import json
import posixpath
import re
import shutil
import sys
from collections import Counter, defaultdict
from pathlib import Path

import markdown

REPORT_RE = re.compile(r"^(?P<name>.+)_(?P<date>\d{8})\.html$")
REPO_URL = "https://github.com/qarmin/ClaudeReports"
KINDS = [("___BUGHUNT___", "bughunt", "Bughunt"), ("___OPTIMIZATIONS___", "perf", "Perf")]
ASSETS = Path(__file__).resolve().parent / "site"
HREF_RE = re.compile(r'href="([^"]*)"')
JSON_BLOCK = r'<script type="application/json" id="{}">(.*?)</script>'
MAX_CODE = 20_000
MAX_TRACE = 4_000


def github_url(repo_path: str) -> str:
    leaf = repo_path.rsplit("/", 1)[-1]
    return f"{REPO_URL}/{'blob' if '.' in leaf else 'tree'}/master/{repo_path}"


def render_markdown(text: str, repo_dir: str) -> str:
    body = markdown.markdown(text, extensions=["tables", "fenced_code", "sane_lists"])

    def fix(m: re.Match) -> str:
        href = m[1]
        if re.match(r"^([a-z]+:|#|//)", href, re.I):
            return m[0]
        path, _, frag = href.partition("#")
        target = posixpath.normpath(posixpath.join(repo_dir, path))
        if target.startswith(".."):
            return m[0]
        return f'href="{github_url(target)}{"#" + frag if frag else ""}"'

    return HREF_RE.sub(fix, body)


def inline_markdown(text: str) -> str:
    text = text.replace("\\`", "`").replace("\\|", "|").strip()
    out = markdown.markdown(text)
    return re.sub(r"^<p>(.*)</p>$", r"\1", out, flags=re.S)


def split_row(line: str) -> list[str]:
    cells = re.split(r"(?<!\\)\|", line.strip())
    return cells[1:-1] if line.strip().endswith("|") else cells[1:]


def parse_table(text: str) -> tuple[str, list[str], list[list[str]], str]:
    lines = text.splitlines()
    start = next((i for i, l in enumerate(lines) if l.startswith("|") and i + 1 < len(lines)
                  and re.match(r"^\|[\s:|-]+\|?$", lines[i + 1])), None)
    if start is None:
        return text, [], [], ""
    end = start + 2
    while end < len(lines) and lines[end].startswith("|"):
        end += 1
    header = [c.strip() for c in split_row(lines[start])]
    rows = [split_row(l) for l in lines[start + 2:end]]
    return "\n".join(lines[:start]), header, rows, "\n".join(lines[end:])


def fix_lines(text: str) -> tuple[int, int] | None:
    m = re.search(r"\+(\d+)\s*/\s*-(\d+)", text)
    return (int(m[1]), int(m[2])) if m else None


def summary_data(ws: Path, repo_dir: str) -> dict:
    chosen = next((n for n in ("SUMMARY-SHORT.md", "SUMMARY.md", "README.md") if (ws / n).is_file()), None)
    links = {n.removesuffix(".md"): github_url(f"{repo_dir}/{n}")
             for n in ("SUMMARY.md", "README.md", "IDEAS.md") if (ws / n).is_file()}
    data = {"workspace": github_url(repo_dir), "files": links, "rows": [], "columns": [], "intro": "", "outro": ""}
    if chosen is None:
        return data
    text = (ws / chosen).read_text(errors="replace")
    intro, header, rows, outro = parse_table(text)
    data["intro"] = render_markdown(intro, repo_dir)
    data["outro"] = render_markdown(outro, repo_dir) if outro.strip() else ""
    if not rows or len(header) < 3:
        data["intro"] = render_markdown(text, repo_dir)
        return data
    lower = [h.lower() for h in header]
    title_col = lower.index("candidate") if "candidate" in lower else 2
    fix_col = lower.index("fix lines") if "fix lines" in lower else None
    skip = {0, 1, title_col, fix_col}
    data["columns"] = [h for i, h in enumerate(header) if i not in skip]
    candidates = {d.name[:3]: d.name for d in ws.iterdir() if d.is_dir() and re.match(r"^\d{3}\.", d.name)}
    for cells in rows:
        cells += [""] * (len(header) - len(cells))
        ident = re.sub(r"\D", "", cells[0]) or cells[0].strip()
        folder = candidates.get(f"{int(ident):03d}") if ident.isdigit() else None
        data["rows"].append({
            "id": cells[0].strip(),
            "cat": cells[1].strip(),
            "title": inline_markdown(cells[title_col]),
            "fix": fix_lines(cells[fix_col]) if fix_col is not None else None,
            "extra": [inline_markdown(c) for i, c in enumerate(cells) if i not in skip],
            "url": github_url(f"{repo_dir}/{folder}") if folder else None,
        })
    return data


def parse_readme(text: str) -> dict:
    fields = dict(re.findall(r"^- ([^:]+): (.*)$", text, re.M))
    msg = re.search(r"## Message\s+```\n(.*?)\n```", text, re.S)
    return {
        "title": text.splitlines()[0].lstrip("# ").strip() if text else "",
        "kind": fields.get("kind", "?").strip(),
        "loc": fields.get("location", "").strip("` "),
        "api": fields.get("last API call", "").strip("` "),
        "miri": fields.get("miri", "").split(" ")[0].strip(),
        "msg": msg[1] if msg else "",
    }


def normalize_kind(kind: str) -> str:
    return "exit status" if kind.startswith("exit status") else kind


def read_limited(path: Path, limit: int) -> str:
    if not path.is_file():
        return ""
    text = path.read_text(errors="replace")
    return text if len(text) <= limit else text[:limit] + f"\n... truncated ({len(text) - limit} more characters)"


def fuzz_data(target: Path, repo_dir: str) -> dict:
    toml = (target / "fuzz.toml").read_text(errors="replace") if (target / "fuzz.toml").is_file() else ""
    source = re.search(r'^git = "([^"]+)"', toml, re.M)
    rev = re.search(r'^rev = "([^"]+)"', toml, re.M)
    crates = re.search(r'^crate = "([^"]+)"', toml, re.M)
    crashes = []
    for bucket in sorted((target / "crashes").iterdir()):
        if not bucket.is_dir():
            continue
        readme = bucket / "README.md"
        info = parse_readme(readme.read_text(errors="replace")) if readme.is_file() else {}
        info.update(
            id=bucket.name,
            kind=normalize_kind(info.get("kind", "?")),
            code=read_limited(bucket / "src" / "main.rs", MAX_CODE),
            trace=read_limited(bucket / "crash.txt", MAX_TRACE),
            url=github_url(f"{repo_dir}/crashes/{bucket.name}"),
        )
        info.pop("title", None)
        crashes.append(info)
    return {
        "crate": crates[1] if crates else target.name,
        "source": source[1] if source else "",
        "rev": rev[1] if rev else "",
        "reported": (target / "REPORTED").exists(),
        "workspace": github_url(repo_dir),
        "files": {"crashes.md": github_url(f"{repo_dir}/crashes.md"), "fuzz.toml": github_url(f"{repo_dir}/fuzz.toml")},
        "crashes": crashes,
    }


def audit_data(path: Path) -> dict:
    text = path.read_text(errors="replace")
    findings, info = [], {}
    try:
        findings = json.loads(re.search(JSON_BLOCK.format("D"), text, re.S)[1])
        info = json.loads(re.search(JSON_BLOCK.format("PI"), text, re.S)[1])
    except (TypeError, ValueError):
        pass
    rank = {s: i for i, s in enumerate(("critical", "high", "medium", "low", "negligible"))}
    top = sorted(findings, key=lambda f: (rank.get(f.get("severity"), 9), f.get("difficulty") != "easy"))[:12]
    return {
        "n": len(findings),
        "sev": dict(Counter(f.get("severity", "?") for f in findings)),
        "diff": dict(Counter(f.get("difficulty", "?") for f in findings)),
        "types": dict(Counter(f.get("type_label") or f.get("type", "?") for f in findings)),
        "top": [{"id": f.get("id"), "sev": f.get("severity"), "diff": f.get("difficulty"), "type": f.get("type_label"),
                 "text": re.sub(r"\s+", " ", f.get("description", ""))[:260]} for f in top],
        "info": {k: info.get(k, "") for k in ("description", "repo_url", "branch", "commit_short", "commit_url", "generated")},
    }


def safe_json(obj) -> str:
    return json.dumps(obj, ensure_ascii=False, separators=(",", ":")).replace("</", "<\\/")


def write_page(out: Path, rel: str, page: str, title: str, payload: dict) -> str:
    depth = rel.count("/")
    text = (SHELL.replace("{title}", html.escape(title)).replace("{root}", "../" * depth)
            .replace("{page}", page).replace("{data}", safe_json(payload)))
    target = out / rel
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(text)
    return rel


def main(src: Path, out: Path) -> None:
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    (out / "latest").mkdir()
    (out / ".nojekyll").touch()
    shutil.copytree(ASSETS, out / "assets")

    projects: dict[str, dict] = {}

    def project(name: str) -> dict:
        return projects.setdefault(name.lower(), {"name": name, "pages": []})

    reports = defaultdict(list)
    for f in [*src.glob("*.html"), *(src / "reports").glob("*.html")]:
        m = REPORT_RE.match(f.name)
        if m:
            shutil.copy2(f, out / f.name)
            reports[m["name"]].append((m["date"], f.name, f.stat().st_size, f))
    for name, files in reports.items():
        files.sort(key=lambda r: r[:3], reverse=True)
        date, fname, size, _ = files[0]
        (out / "latest" / f"{name}.html").write_text(
            f'<!doctype html><meta charset="utf-8"><meta http-equiv="refresh" content="0; url=../{fname}">'
            f'<link rel="canonical" href="../{fname}"><a href="../{fname}">{html.escape(fname)}</a>\n')
        entry = project(name)
        entry["name"] = name
        audit = audit_data(files[0][3])
        info = audit.pop("info")
        entry.update(desc=info["description"], repo=info["repo_url"], commit=info)
        entry["audit"] = {
            "url": fname, "date": f"{date[:4]}-{date[4:6]}-{date[6:]}", "kb": round(size / 1024),
            "older": [{"url": f, "date": f"{d[:4]}-{d[4:6]}-{d[6:]}"} for d, f, _, _ in files[1:]], **audit,
        }

    base = src / "investigations"
    for dirname, slug, _ in KINDS:
        for ws in sorted(base.glob(f"**/{dirname}")):
            rel = ws.relative_to(base).parts[:-1]
            page_name = "__".join(rel)
            data = summary_data(ws, ws.relative_to(src).as_posix())
            entry = project(rel[0])
            url = f"{slug}/{page_name}.html"
            data.update(project=entry["name"], kind=slug, sub="/".join(rel[1:]), _url=url)
            entry["pages"].append((slug, data))
            entry.setdefault(slug, []).append({
                "url": url, "sub": data["sub"], "n": len(data["rows"]),
                "cats": dict(Counter(r["cat"] for r in data["rows"])),
                "rows": [{k: r[k] for k in ("id", "cat", "title", "fix")} for r in data["rows"]],
                "workspace": data["workspace"]})

    fuzz_base = src / "fuzz-targets"
    if fuzz_base.is_dir():
        for target in sorted(fuzz_base.iterdir()):
            if not (target / "crashes").is_dir():
                continue
            data = fuzz_data(target, target.relative_to(src).as_posix())
            entry = project(target.name)
            url = f"fuzz/{target.name}.html"
            data.update(project=entry["name"], _url=url)
            entry["pages"].append(("fuzz", data))
            crashes = data["crashes"]
            entry["fuzz"] = {
                "url": url, "n": len(crashes), "kinds": dict(Counter(c["kind"] for c in crashes)),
                "reported": data["reported"], "source": data["source"], "rev": data["rev"], "workspace": data["workspace"],
                "files": Counter(re.sub(r":\d+(:\d+)?$", "", c["loc"]) for c in crashes).most_common(12),
                "miri": sum(c["miri"] == "clean" for c in crashes)}

    index = []
    for key in sorted(projects, key=str.lower):
        entry = projects[key]
        slug_name = entry["name"]
        tabs = {"overview": f"projects/{slug_name}.html", "audit": entry.get("audit", {}).get("url"),
                "fuzz": entry.get("fuzz", {}).get("url")}
        for _, slug, _ in KINDS:
            tabs[slug] = entry[slug][0]["url"] if slug in entry else None
        common = {"tabs": tabs, "name": entry["name"], "repo": entry.get("repo", ""), "desc": entry.get("desc", "")}
        for slug, data in entry.pop("pages"):
            write_page(out, data.pop("_url"), slug, f"{entry['name']} - {slug}", {**data, **common})
        write_page(out, tabs["overview"], "project", f"{entry['name']} - overview", {**entry, **common})
        entry["overview"] = tabs["overview"]
        for slug, _, _ in KINDS:
            for t in entry.get(slug, []):
                t.pop("rows", None)
                t.pop("workspace", None)
        entry.get("audit", {}).pop("top", None)
        entry.pop("commit", None)
        entry["fuzz"] = {k: v for k, v in entry["fuzz"].items() if k in ("url", "n", "kinds", "reported")} if "fuzz" in entry else None
        if entry["fuzz"] is None:
            del entry["fuzz"]
        index.append(entry)

    (out / "index.html").write_text(
        SHELL.replace("{title}", "Claude Code audit reports").replace("{root}", "")
        .replace("{page}", "index").replace("{data}", safe_json({"projects": index, "repo": REPO_URL})))
    print(f"{len(index)} projects written to {out}")


SHELL = """<!doctype html>
<html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="color-scheme" content="dark">
<title>{title}</title>
<link rel="stylesheet" href="{root}assets/style.css">
</head>
<body data-page="{page}" data-root="{root}">
<div id="app" class="app"></div>
<script type="application/json" id="data">{data}</script>
<script src="{root}assets/site.js"></script>
</body></html>
"""

if __name__ == "__main__":
    main(Path(sys.argv[1] if len(sys.argv) > 1 else "."),
         Path(sys.argv[2] if len(sys.argv) > 2 else "_site"))
