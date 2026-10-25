#!/usr/bin/env python3
"""Generate the STATIC (deterministic, combinatorial) corpus - the second,
complementary generator mode alongside `generator.py`'s random/seeded one.

No RNG anywhere: each category in `templates_static/` declares an explicit,
bounded scenario matrix (a dict of `{axis_name: [allowed values]}`) and this
enumerates its full cartesian product. Re-running with no arguments always
regenerates the exact same corpus, byte for byte - there is no seed, because
there is nothing random to seed. To add coverage, add a value to an axis list
in the relevant `templates_static/*.py` module; you do not need to touch this
file or re-run with different flags to pick it up.

Usage:
    ./static_generator.py --out corpus_static/
    ./static_generator.py --out corpus_static/ --categories union_ref_fields_static
    ./static_generator.py --list-categories

Batching (throughput) and noise injection - same meaning as generator.py's
flags (see README.md "Batching for throughput" / "Noise injection"); noise
here is still fully deterministic, seeded from `(category, idx)` rather than
from a `--seed` flag (static mode has none):
    ./static_generator.py --out corpus_static/ --cases-per-file 20
    ./static_generator.py --out corpus_static/ --cases-per-file 20 --noise-level 2
"""

from __future__ import annotations

import argparse
import random
import sys
from pathlib import Path

from templates_static import STATIC_TEMPLATES
from templates import batching, noise


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", type=Path, default=Path("corpus_static"), help="output directory")
    ap.add_argument(
        "--categories",
        type=str,
        default=None,
        help="comma-separated subset of static template categories to use (default: all)",
    )
    ap.add_argument("--list-categories", action="store_true", help="print available categories and exit")
    ap.add_argument(
        "--cases-per-file",
        type=int,
        default=1,
        help="pack this many independent sub-cases into one .rs file to amortize rustc "
        "process-startup overhead (default 1 = today's one-program-per-file behavior, unchanged)",
    )
    ap.add_argument(
        "--noise-level",
        type=int,
        choices=[0, 1, 2],
        default=0,
        help="0 = none (default, today's behavior), 1 = light, 2 = heavy provably-inert filler "
        "woven around each sub-case's real logic - see README.md 'Noise injection'",
    )
    args = ap.parse_args()

    if args.cases_per_file < 1:
        print("--cases-per-file must be >= 1", file=sys.stderr)
        return 1

    if args.list_categories:
        for name in sorted(STATIC_TEMPLATES):
            print(name)
        return 0

    if args.categories:
        names = [c.strip() for c in args.categories.split(",") if c.strip()]
        unknown = [n for n in names if n not in STATIC_TEMPLATES]
        if unknown:
            print(f"unknown categories: {unknown}; available: {sorted(STATIC_TEMPLATES)}", file=sys.stderr)
            return 1
        category_names = names
    else:
        category_names = sorted(STATIC_TEMPLATES)

    args.out.mkdir(parents=True, exist_ok=True)

    if args.cases_per_file == 1 and args.noise_level == 0:
        generated = 0
        for category_name in category_names:
            programs = STATIC_TEMPLATES[category_name]()
            for program in programs:
                rs_path = args.out / f"{program.name}.rs"
                expected_path = args.out / f"{program.name}.expected"
                rs_path.write_text(program.source)
                expected_path.write_text(program.expected_stdout)
                generated += 1
            print(f"  {category_name}: {len(programs)} programs (full declared scenario matrix)")

        print(f"generated {generated} programs across {len(category_names)} static categories into {args.out}")
        return 0

    # Batched and/or noisy path. Each category's full declared scenario
    # matrix is generated exactly as today (same `generate_all()`, nothing
    # about the matrix itself changes); only how many of those programs share
    # one rustc invocation (and how much inert filler surrounds each one)
    # changes. Noise here is seeded from `(category_name, idx)` rather than a
    # `--seed` flag, matching static mode's "no RNG, same input -> same
    # corpus" philosophy - the enumeration order itself is still the sole
    # source of variation.
    generated = 0
    files_written = 0
    chunk: list[tuple] = []
    chunk_idx = 0

    def flush(chunk, chunk_idx):
        nonlocal files_written
        if not chunk:
            return
        batch_name = f"batch_static_{chunk_idx}_n{len(chunk)}"
        batch_file = batching.assemble_batch(chunk, batch_name, category="mixed_static")
        (args.out / f"{batch_file.name}.rs").write_text(batch_file.source)
        (args.out / f"{batch_file.name}.expected").write_text(batch_file.expected_stdout)
        files_written += 1

    for category_name in category_names:
        programs = STATIC_TEMPLATES[category_name]()
        for idx, program in enumerate(programs):
            noise_bundle = None
            if args.noise_level:
                noise_rng = random.Random(f"static-noise:{category_name}:{idx}")
                noise_bundle = noise.generate(noise_rng, args.noise_level)
            chunk.append((program, noise_bundle))
            generated += 1
            if len(chunk) >= args.cases_per_file:
                flush(chunk, chunk_idx)
                chunk_idx += 1
                chunk = []
        print(f"  {category_name}: {len(programs)} programs (full declared scenario matrix)")

    flush(chunk, chunk_idx)

    print(
        f"generated {generated} sub-cases across {len(category_names)} static categories "
        f"into {files_written} file(s) in {args.out} "
        f"(--cases-per-file {args.cases_per_file}, --noise-level {args.noise_level})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
