#!/usr/bin/env python3
import re, sys, collections

SUFFIX = re.compile(r"_(normal2|normal3|boundary_(min|max|zero|allones|mixed)|asym_min_max|asym_max_min|imm_\w+)$")

def base(name):
    prev = None
    while prev != name:
        prev = name
        name = SUFFIX.sub("", name)
    return name

all_tests = [l.strip() for l in open(sys.argv[2] if len(sys.argv) > 2 else "all_tests.txt") if l.strip()]
log = re.sub(r"\x1b\[[0-9;]*m", "", open(sys.argv[1], encoding="utf-8", errors="replace").read())
problems = dict()
for name, kind in re.findall(r"isolate\.rs:\d+ -   (\S+)\s+(CRASHED|TIMEOUT|PANIC|FAIL.*)$", log, re.M):
    problems[name] = "wrong" if kind.startswith("FAIL") else "trap"

groups = collections.defaultdict(list)
for t in all_tests:
    groups[base(t)].append(t)

ok = trap = wrong = partial = 0
partial_names = []
for g, tests in groups.items():
    kinds = {problems.get(t, "ok") for t in tests}
    if kinds == {"ok"}:
        ok += 1
    elif "wrong" in kinds:
        wrong += 1
    elif kinds == {"trap"}:
        trap += 1
    else:
        partial += 1
        partial_names.append(g)

total = len(groups)
print(f"distinct intrinsics (test groups): {total}")
print(f"  fully OK          : {ok:>6}  ({100*ok/total:.1f}%)")
print(f"  all scenarios trap: {trap:>6}  ({100*trap/total:.1f}%)")
print(f"  wrong result      : {wrong:>6}  ({100*wrong/total:.1f}%)")
print(f"  partial (mixed)   : {partial:>6}  ({100*partial/total:.1f}%)")
if partial_names:
    print("  mixed:", ", ".join(sorted(partial_names)[:20]))
