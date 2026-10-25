#!/usr/bin/env python3
"""Run testbed components with per-component timeouts; write results/<stamp>/failures.tsv.

  run.py [--long] [--seed N] [--only a,b] [--list]
Every component runs in its own subprocess, so one hang/crash never stops the others.
"""
import argparse
import json
import multiprocessing
import queue
import sys
import time
from datetime import datetime
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent / "lib"))
from common import Ctx, TB, failure  # noqa: E402
from components import COMPONENTS  # noqa: E402

TIMEOUTS = {"suite_clang": 900, "suite_gcc": 900, "corpus_run": 900, "modinfo_diff": 900}


def _child(name, results, long, seed, q):
    ctx = Ctx(results=results, long=long, seed=seed)
    try:
        q.put(COMPONENTS[name](ctx))
    except Exception as e:  # a crashing component is a testbed bug, report it as such
        import traceback
        q.put([failure(f"testbed:{name}:exception", name, "", traceback.format_exc()[-300:])])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--long", action="store_true")
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--only", default="")
    ap.add_argument("--list", action="store_true")
    a = ap.parse_args()
    if a.list:
        print("\n".join(COMPONENTS))
        return 0
    names = [n for n in COMPONENTS if not a.only or n in a.only.split(",")]
    stamp = datetime.now().strftime("%Y-%m-%d_%H%M%S")
    results = TB / "results" / stamp
    results.mkdir(parents=True)
    rows, summary = [], []
    t_all = time.time()
    for name in names:
        t0 = time.time()
        q = multiprocessing.Queue()
        p = multiprocessing.Process(target=_child, args=(name, results, a.long, a.seed, q))
        p.start()
        limit = TIMEOUTS.get(name, 600) * (4 if a.long else 1) + (600 if a.long and name.startswith("fuzz_") else 0)
        # Read before join: a child blocks on exit until its queued result has been consumed.
        try:
            fails = q.get(timeout=limit)
        except queue.Empty:
            fails = [failure(f"testbed:{name}:component-timeout", name, "", f">{limit}s")]
        p.join(30)
        if p.is_alive():
            p.kill()
        dt = time.time() - t0
        rows += fails
        sigs = sorted({f["signature"] for f in fails})
        summary.append(dict(component=name, seconds=round(dt), failures=len(fails), signatures=sigs))
        print(f"{name:20s} {dt:6.0f}s  {len(fails):3d} failure(s)  {', '.join(sigs)[:150]}", flush=True)
    with open(results / "failures.tsv", "w") as fh:
        fh.write("signature\tcomponent\tinput\tdetail\n")
        for r in rows:
            fh.write(f"{r['signature']}\t{r['component']}\t{r['input']}\t{r['detail']}\n")
    meta = dict(stamp=stamp, long=a.long, seed=a.seed, seconds=round(time.time() - t_all),
                commit=(TB / "_src" / ".commit").read_text().strip(), components=summary)
    (results / "summary.json").write_text(json.dumps(meta, indent=1))
    print(f"total {meta['seconds']}s, {len(rows)} failure(s), {len({r['signature'] for r in rows})} signature(s)"
          f" -> {results}/failures.tsv")
    return 0


if __name__ == "__main__":
    sys.exit(main())
