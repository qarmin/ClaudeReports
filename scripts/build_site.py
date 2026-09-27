#!/usr/bin/env python3
"""Build the GitHub Pages site: copy reports, generate index and stable links."""
import html
import re
import shutil
import sys
from collections import defaultdict
from pathlib import Path

REPORT_RE = re.compile(r"^(?P<name>.+)_(?P<date>\d{8})\.html$")
REPO_URL = "https://github.com/qarmin/ClaudeReports"
INVESTIGATION_KINDS = [("___BUGHUNT___", "bughunt"), ("___OPTIMIZATIONS___", "perf")]


def investigation_badges(src: Path, name: str) -> str:
    base = src / "investigations" / name
    if not base.is_dir():
        return ""
    badges = []
    for dirname, label in INVESTIGATION_KINDS:
        if (base / dirname).is_dir():
            url = f"{REPO_URL}/tree/master/investigations/{name}/{dirname}"
            badges.append(f'<a class="badge" href="{url}">{label}</a>')
    return " ".join(badges)


def main(src: Path, out: Path) -> None:
    out.mkdir(parents=True, exist_ok=True)
    (out / "latest").mkdir(exist_ok=True)
    (out / ".nojekyll").touch()

    projects = defaultdict(list)
    for f in src.glob("*.html"):
        m = REPORT_RE.match(f.name)
        if not m:
            continue
        shutil.copy2(f, out / f.name)
        projects[m["name"]].append((m["date"], f.name, f.stat().st_size))

    rows = []
    for name in sorted(projects, key=str.lower):
        reports = sorted(projects[name], reverse=True)
        date, fname, size = reports[0]
        # Stable URL that always points to the newest report of the project
        (out / "latest" / f"{name}.html").write_text(
            f'<!doctype html><meta charset="utf-8">'
            f'<meta http-equiv="refresh" content="0; url=../{fname}">'
            f'<link rel="canonical" href="../{fname}">'
            f'<a href="../{fname}">{html.escape(fname)}</a>\n')
        older = " ".join(
            f'<a class="old" href="{f}">{d[:4]}-{d[4:6]}-{d[6:]}</a>'
            for d, f, _ in reports[1:])
        rows.append(
            f'<tr><td><a href="{fname}">{html.escape(name)}</a></td>'
            f'<td>{date[:4]}-{date[4:6]}-{date[6:]}</td>'
            f'<td class="num">{size / 1024:.0f}&nbsp;KB</td><td>{older}</td>'
            f'<td>{investigation_badges(src, name)}</td></tr>')

    (out / "index.html").write_text(INDEX.replace("{count}", str(len(rows)))
                                    .replace("{rows}", "\n".join(rows)))
    print(f"{len(rows)} projects written to {out}")


INDEX = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Claude Code audit reports</title>
<style>
:root{color-scheme:light dark;--fg:#1f2328;--bg:#fff;--muted:#656d76;--line:#d0d7de;--link:#0969da}
@media (prefers-color-scheme:dark){:root{--fg:#e6edf3;--bg:#0d1117;--muted:#8d96a0;--line:#30363d;--link:#4493f8}}
body{font:15px/1.5 system-ui,sans-serif;color:var(--fg);background:var(--bg);max-width:900px;margin:0 auto;padding:1.5rem 1rem}
a{color:var(--link);text-decoration:none}a:hover{text-decoration:underline}
input{width:100%;box-sizing:border-box;padding:.5rem .7rem;font:inherit;color:inherit;background:transparent;border:1px solid var(--line);border-radius:6px;margin:1rem 0}
table{width:100%;border-collapse:collapse}th,td{text-align:left;padding:.35rem .5rem;border-bottom:1px solid var(--line)}
th{color:var(--muted);font-weight:600}.num{text-align:right;white-space:nowrap}.old{color:var(--muted);font-size:.9em;margin-right:.5em}
p{color:var(--muted)}
.badge{display:inline-block;font-size:.8em;padding:.1em .5em;margin-right:.3em;border:1px solid var(--line);border-radius:1em;color:var(--muted)}
.badge:hover{color:var(--link);border-color:var(--link);text-decoration:none}
</style>
</head>
<body>
<h1>Claude Code audit reports</h1>
<p>{count} projects. Source: <a href="https://github.com/qarmin/ClaudeReports">qarmin/ClaudeReports</a>.
Stable link to the newest report of a project: <code>latest/&lt;Project&gt;.html</code>.
<code>bughunt</code> / <code>perf</code> badges link to the supporting
<a href="https://github.com/qarmin/ClaudeReports/tree/master/investigations">investigations/</a> workspace on GitHub.</p>
<input id="q" type="search" placeholder="Filter projects..." autofocus>
<table>
<thead><tr><th>Project</th><th>Date</th><th class="num">Size</th><th>Older</th><th>Investigations</th></tr></thead>
<tbody id="t">
{rows}
</tbody>
</table>
<script>
const q = document.getElementById('q'), rows = [...document.querySelectorAll('#t tr')];
q.addEventListener('input', () => {
  const s = q.value.toLowerCase();
  rows.forEach(r => r.hidden = !r.cells[0].textContent.toLowerCase().includes(s));
});
</script>
</body>
</html>
"""

if __name__ == "__main__":
    main(Path(sys.argv[1] if len(sys.argv) > 1 else "."),
         Path(sys.argv[2] if len(sys.argv) > 2 else "_site"))
