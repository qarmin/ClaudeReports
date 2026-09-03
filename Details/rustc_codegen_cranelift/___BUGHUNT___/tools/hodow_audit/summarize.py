#!/usr/bin/env python3
"""Aggregate a hodow --isolate log into a per-extension implemented/missing/wrong table."""
import re, sys, collections

log = open(sys.argv[1], encoding="utf-8", errors="replace").read()
log = re.sub(r"\x1b\[[0-9;]*m", "", log)

stats = {}
for k in ("Passed", "Failed", "Panicked", "Timed out", "Crashed", "Not run", "Total"):
    m = re.search(rf"^.*?{re.escape(k)}\s+(\d+)\s*$", log, re.M)
    if m:
        stats[k] = int(m.group(1))

problems = re.findall(r"isolate\.rs:\d+ -   (\S+)\s+(CRASHED|TIMEOUT|PANIC|FAIL: .*)$", log, re.M)

def ext_of(name):
    m = re.match(r"test_(x86_64|x86)_([a-z0-9]+)_", name)
    return f"{m.group(1)}::{m.group(2)}" if m else "?"

# totals per extension come from the LLVM baseline table
base = {}
btxt = re.sub(r"\x1b\[[0-9;]*m", "", open(sys.argv[2], encoding="utf-8", errors="replace").read())
for line in re.findall(r"main\.rs:65 - (\S+)\s+(\d+)\s+\d+\s+\d+\s+\d+", btxt):
    base[line[0].lower()] = int(line[1])

bad = collections.Counter()
wrong = collections.Counter()
for name, kind in problems:
    e = ext_of(name)
    if kind.startswith("FAIL"):
        wrong[e] += 1
    else:
        bad[e] += 1

print(f"{'extension':<24}{'total':>8}{'ok':>8}{'trap':>8}{'wrong':>8}{'ok %':>8}")
print("-" * 64)
tot_all = ok_all = bad_all = wrong_all = 0
for e, total in sorted(base.items(), key=lambda kv: -kv[1]):
    b, w = bad[e], wrong[e]
    ok = total - b - w
    tot_all += total; ok_all += ok; bad_all += b; wrong_all += w
    print(f"{e:<24}{total:>8}{ok:>8}{b:>8}{w:>8}{100*ok/total:>7.1f}%")
print("-" * 64)
print(f"{'TOTAL':<24}{tot_all:>8}{ok_all:>8}{bad_all:>8}{wrong_all:>8}{100*ok_all/tot_all:>7.1f}%")
print()
print("raw isolate stats:", stats)
