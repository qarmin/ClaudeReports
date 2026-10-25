#!/usr/bin/env python3
"""addrow.py <json>: append a candidate row to SUMMARY.md (and SUMMARY-SHORT.md when Confirmed), then run tidy.
JSON keys: num, status, cls, repro, cand, cause, intro, short_cand, fix, short_cause (last three only if Confirmed)."""
import json
import subprocess
import sys

r = json.loads(sys.argv[1])
esc = lambda s: s.replace("|", "\\|")
cls = r.get("cls", "")
row = (f"| {r['num']} | {r['status']} | {cls} | {r.get('repro', '')} | {r['cand']} | {esc(r['cause'])} | "
       f"{esc(r.get('intro', ''))} | | [`{r['cand']}/RESULT.md`]({r['cand']}/RESULT.md) |\n")
with open("SUMMARY.md", "a") as f:
    f.write(row)
if r["status"] == "Confirmed":
    short = (f"| {r['num']} | {cls.split(' ')[0]} | {esc(r['short_cand'])} | {r['fix']} | {esc(r['short_cause'])} | "
             f"{esc(r['intro']).split(' ')[0]} |\n")
    with open("SUMMARY-SHORT.md", "a") as f:
        f.write(short)
subprocess.run(["just", "tidy"], check=False)
