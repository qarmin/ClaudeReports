#!/usr/bin/env python3
"""Differential-test every corpus/*.rs program under 2+ rustc configurations
that must agree for any UB-free Rust program, in parallel.

For each program this:
  1. Compiles it once per configuration (e.g. `-C opt-level=0` vs `=3`).
  2. Runs the resulting binary (if it compiled) and captures stdout+exit code.
  3. Compares every configuration's (compile-ok?, stdout, exit code) against
     the first configuration (the "baseline").
  4. Separately compares the baseline's stdout against the program's own
     `.expected` file (the generator's independently-computed oracle).

A real finding is case 3: two configurations that are each individually valid
rustc invocations on UB-free code disagreeing with each other. That gets
copied into `findings/<progname>/` with both outputs and the exact commands.

A mismatch caught only by case 4 (all configs agree with each other but not
with `.expected`) is a bug in the *generator's* prediction, not evidence of a
compiler bug - it's reported separately (`--self-check-failures`) and should
get the template fixed, not filed as a finding.

Usage:
    ./diff_runner.py --corpus corpus/ --findings findings/ --jobs 24
    ./diff_runner.py --corpus corpus/ --configs "O0=-C opt-level=0" "O3=-C opt-level=3"
    ./diff_runner.py --corpus corpus/ --rustc "rustc +nightly"
"""

from __future__ import annotations

import argparse
import concurrent.futures
import dataclasses
import difflib
import os
import shlex
import shutil
import subprocess
import sys
from pathlib import Path

DEFAULT_CONFIGS = [
    ("O0", ["-C", "opt-level=0"]),
    ("O3", ["-C", "opt-level=3"]),
]

# Pinned identically across every configuration so a config difference can
# only come from the flags we're deliberately varying (e.g. opt-level), never
# from debug-assertions/overflow-checks defaults silently tracking opt-level.
COMMON_FLAGS = ["-C", "overflow-checks=off", "-C", "debug-assertions=off", "--edition", "2021"]

COMPILE_TIMEOUT_S = 60
RUN_TIMEOUT_S = 10


@dataclasses.dataclass
class ConfigResult:
    name: str
    compile_ok: bool
    compile_cmd: list[str]
    compile_stderr: str
    run_ok: bool
    stdout: str
    exit_code: int | None
    timed_out: bool


@dataclasses.dataclass
class ProgramResult:
    name: str
    rs_path: str
    expected: str
    configs: list[ConfigResult]
    divergence: bool
    self_check_failure: bool


def run_one_config(rustc_argv: list[str], rs_path: Path, config_name: str, flags: list[str], build_dir: Path) -> ConfigResult:
    build_dir.mkdir(parents=True, exist_ok=True)
    out_bin = build_dir / rs_path.stem
    compile_cmd = [*rustc_argv, str(rs_path), "-o", str(out_bin), *COMMON_FLAGS, *flags]
    try:
        cp = subprocess.run(
            compile_cmd,
            capture_output=True,
            text=True,
            timeout=COMPILE_TIMEOUT_S,
        )
    except subprocess.TimeoutExpired:
        return ConfigResult(config_name, False, compile_cmd, "COMPILE TIMEOUT", False, "", None, True)

    if cp.returncode != 0:
        return ConfigResult(config_name, False, compile_cmd, cp.stderr, False, "", None, False)

    try:
        rp = subprocess.run(
            [str(out_bin)],
            capture_output=True,
            text=True,
            timeout=RUN_TIMEOUT_S,
        )
        return ConfigResult(config_name, True, compile_cmd, "", True, rp.stdout, rp.returncode, False)
    except subprocess.TimeoutExpired:
        return ConfigResult(config_name, True, compile_cmd, "", False, "", None, True)
    finally:
        try:
            out_bin.unlink()
        except FileNotFoundError:
            pass


def process_program(rs_path_str: str, rustc_argv: list[str], configs: list[tuple[str, list[str]]], build_root_str: str) -> ProgramResult:
    rs_path = Path(rs_path_str)
    build_root = Path(build_root_str)
    expected_path = rs_path.with_suffix(".expected")
    expected = expected_path.read_text() if expected_path.exists() else ""

    results = []
    for name, flags in configs:
        build_dir = build_root / name / str(os.getpid())
        results.append(run_one_config(rustc_argv, rs_path, name, flags, build_dir))

    baseline = results[0]
    divergence = False
    for other in results[1:]:
        if (baseline.compile_ok, baseline.run_ok, baseline.stdout, baseline.exit_code) != (
            other.compile_ok,
            other.run_ok,
            other.stdout,
            other.exit_code,
        ):
            divergence = True

    self_check_failure = (
        not divergence and baseline.compile_ok and baseline.run_ok and baseline.stdout != expected
    )

    return ProgramResult(
        name=rs_path.stem,
        rs_path=str(rs_path),
        expected=expected,
        configs=results,
        divergence=divergence,
        self_check_failure=self_check_failure,
    )


def write_finding(findings_dir: Path, result: ProgramResult, rustc_argv: list[str]) -> Path:
    out_dir = findings_dir / result.name
    out_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy(result.rs_path, out_dir / Path(result.rs_path).name)
    (out_dir / "expected_by_generator.txt").write_text(result.expected)

    lines = [f"# Divergence: {result.name}", "", f"Source: `{result.rs_path}`", ""]
    for cfg in result.configs:
        lines.append(f"## Config `{cfg.name}`")
        lines.append("")
        lines.append("Compile command:")
        lines.append("```")
        lines.append(shlex.join(cfg.compile_cmd))
        lines.append("```")
        if not cfg.compile_ok:
            lines.append("")
            lines.append("**Did not compile.** stderr:")
            lines.append("```")
            lines.append(cfg.compile_stderr)
            lines.append("```")
        elif cfg.timed_out:
            lines.append("")
            lines.append("**Run timed out.**")
        else:
            lines.append("")
            lines.append(f"Exit code: `{cfg.exit_code}`")
            lines.append("")
            lines.append("stdout:")
            lines.append("```")
            lines.append(cfg.stdout)
            lines.append("```")
        lines.append("")
        (out_dir / f"stdout.{cfg.name}.txt").write_text(cfg.stdout)
        (out_dir / f"stderr.{cfg.name}.txt").write_text(cfg.compile_stderr)

    lines.append("## Generator-predicted expected stdout")
    lines.append("```")
    lines.append(result.expected)
    lines.append("```")

    # Pinpointing which sub-case diverged: for a batched corpus file (see
    # README.md "Batching for throughput"), every stdout line is already
    # self-identifying ("case_<n>:..."), so a plain line-by-line diff between
    # the baseline config and each other config's stdout is enough to show
    # exactly which sub-case's line actually differs - no need to re-run or
    # bisect the file by hand to find it. Still useful (just less pinpointed)
    # for a non-batched single-case file.
    baseline = result.configs[0]
    diff_sections = []
    for other in result.configs[1:]:
        if other.stdout == baseline.stdout:
            continue
        udiff = list(
            difflib.unified_diff(
                baseline.stdout.splitlines(),
                other.stdout.splitlines(),
                fromfile=f"stdout.{baseline.name}",
                tofile=f"stdout.{other.name}",
                lineterm="",
            )
        )
        diff_sections.append((baseline.name, other.name, udiff))

    if diff_sections:
        lines.append("")
        lines.append("## stdout diff (which line/sub-case actually differs)")
        for base_name, other_name, udiff in diff_sections:
            lines.append("")
            lines.append(f"### `{base_name}` vs `{other_name}`")
            lines.append("```diff")
            lines.extend(udiff if udiff else ["(stdout differs but produced no line-level diff - check exit code/compile status above)"])
            lines.append("```")

    (out_dir / "REPORT.md").write_text("\n".join(lines))
    return out_dir


def parse_configs(raw: list[str] | None) -> list[tuple[str, list[str]]]:
    if not raw:
        return DEFAULT_CONFIGS
    parsed = []
    for item in raw:
        name, _, flag_str = item.partition("=")
        if not name or not flag_str:
            raise ValueError(f"bad --configs entry {item!r}, expected NAME=FLAGS")
        parsed.append((name, shlex.split(flag_str)))
    return parsed


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--corpus", type=Path, default=Path("corpus"))
    ap.add_argument("--findings", type=Path, default=Path("findings"))
    ap.add_argument("--build-dir", type=Path, default=None, help="scratch dir for compiled binaries (default: temp under findings/.build)")
    ap.add_argument("--jobs", type=int, default=os.cpu_count() or 4)
    ap.add_argument("--rustc", type=str, default="rustc", help="rustc invocation, e.g. 'rustc' or 'rustc +nightly'")
    ap.add_argument(
        "--configs",
        nargs="+",
        default=None,
        help='configs as NAME=FLAGS, e.g. "O0=-C opt-level=0" "O3=-C opt-level=3" (default: those two)',
    )
    ap.add_argument(
        "--report-self-check-failures",
        action="store_true",
        help="also print programs where all configs agree with each other but not with the generator's .expected",
    )
    args = ap.parse_args()

    configs = parse_configs(args.configs)
    rustc_argv = shlex.split(args.rustc)
    build_root = args.build_dir or (args.findings / ".build")

    rs_files = sorted(args.corpus.glob("*.rs"))
    if not rs_files:
        print(f"no .rs files found under {args.corpus}", file=sys.stderr)
        return 1

    print(f"diffing {len(rs_files)} programs across configs {[c[0] for c in configs]} with {args.jobs} workers...")

    divergences: list[ProgramResult] = []
    self_check_failures: list[ProgramResult] = []
    compile_failures = 0

    with concurrent.futures.ProcessPoolExecutor(max_workers=args.jobs) as pool:
        futs = {
            pool.submit(process_program, str(rs), rustc_argv, configs, str(build_root)): rs
            for rs in rs_files
        }
        done = 0
        total = len(futs)
        for fut in concurrent.futures.as_completed(futs):
            rs = futs[fut]
            done += 1
            try:
                result = fut.result()
            except Exception as e:  # noqa: BLE001 - surface any worker crash as a loud failure, not a silent skip
                print(f"[{done}/{total}] WORKER ERROR on {rs}: {e}", file=sys.stderr)
                continue

            if not all(c.compile_ok for c in result.configs):
                # Not a divergence by itself unless compile status *differs*
                # between configs - that case is still caught below.
                if len({c.compile_ok for c in result.configs}) == 1:
                    compile_failures += 1

            if result.divergence:
                divergences.append(result)
                print(f"[{done}/{total}] DIVERGENCE: {result.name}")
            elif result.self_check_failure:
                self_check_failures.append(result)
                if args.report_self_check_failures:
                    print(f"[{done}/{total}] self-check mismatch (harness bug, not a finding): {result.name}")

    print()
    print(f"done. {len(rs_files)} programs, {compile_failures} failed to compile (same on all configs),")
    print(f"{len(self_check_failures)} self-check mismatches (generator-oracle bugs, not findings),")
    print(f"{len(divergences)} real cross-config divergences.")

    if divergences:
        args.findings.mkdir(parents=True, exist_ok=True)
        print()
        print("Divergences written to:")
        for result in divergences:
            out_dir = write_finding(args.findings, result, rustc_argv)
            print(f"  {out_dir}")

    if self_check_failures and not args.report_self_check_failures:
        names = ", ".join(r.name for r in self_check_failures[:10])
        more = "" if len(self_check_failures) <= 10 else f" (+{len(self_check_failures) - 10} more)"
        print()
        print(f"Self-check mismatches (rerun with --report-self-check-failures for detail): {names}{more}")

    return 1 if divergences else 0


if __name__ == "__main__":
    raise SystemExit(main())
