#!/usr/bin/env python3
"""Generate a corpus of small, parameterized, UB-free Rust programs for
differential testing against rustc optimization bugs.

Each program is produced by a template category (see `templates/`), driven by
a `random.Random` seeded deterministically from `--seed` and the program's
index, so a given `(seed, index)` always reproduces the exact same source -
useful for isolating/minimizing a specific finding later.

Usage:
    ./generator.py --count 200 --seed 1 --out corpus/
    ./generator.py --count 50 --seed 1 --out corpus/ --categories union_ref_fields,packed_struct_fields
    ./generator.py --list-categories

Batching (throughput) and noise injection:
    ./generator.py --count 5000 --seed 1 --out corpus/ --cases-per-file 100
    ./generator.py --count 5000 --seed 1 --out corpus/ --cases-per-file 100 --noise-level 2

See README.md "Batching for throughput" and "Noise injection" for what these
do and why; `--cases-per-file 1 --noise-level 0` (the defaults) reproduce
today's one-program-per-file output exactly, byte for byte.
"""

from __future__ import annotations

import argparse
import random
import sys
from pathlib import Path

from templates import NOT_DEFAULT, TEMPLATES
from templates import batching, identity_chain, noise


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--count", type=int, default=100, help="number of programs to generate")
    ap.add_argument("--seed", type=int, default=0, help="base RNG seed (combined with each program's index)")
    ap.add_argument("--out", type=Path, default=Path("corpus"), help="output directory")
    ap.add_argument(
        "--categories",
        type=str,
        default=None,
        help="comma-separated subset of template categories to use (default: all)",
    )
    ap.add_argument("--list-categories", action="store_true", help="print available categories and exit")
    ap.add_argument(
        "--start-index",
        type=int,
        default=0,
        help="first index to generate (lets you append to an existing corpus without colliding names)",
    )
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
    ap.add_argument(
        "--depth",
        type=int,
        default=None,
        help="identity_chain_safe/identity_chain_unsafe only: max nesting depth of the generated "
        "identity tree (module default 8; ignored by every other category)",
    )
    ap.add_argument(
        "--breadth",
        type=int,
        default=None,
        help="identity_chain_safe/identity_chain_unsafe only: max match-arm branching per match "
        "node (module default 3; ignored by every other category)",
    )
    args = ap.parse_args()

    if args.depth is not None or args.breadth is not None:
        identity_chain.configure(depth=args.depth, breadth=args.breadth)

    if args.cases_per_file < 1:
        print("--cases-per-file must be >= 1", file=sys.stderr)
        return 1

    if args.list_categories:
        for name in sorted(TEMPLATES):
            suffix = "  (opt-in - not in the default rotation; pass --categories to use)" if name in NOT_DEFAULT else ""
            print(f"{name}{suffix}")
        return 0

    if args.categories:
        names = [c.strip() for c in args.categories.split(",") if c.strip()]
        unknown = [n for n in names if n not in TEMPLATES]
        if unknown:
            print(f"unknown categories: {unknown}; available: {sorted(TEMPLATES)}", file=sys.stderr)
            return 1
        category_names = names
    else:
        # NOT_DEFAULT categories (e.g. identity_chain_unsafe) are opt-in only -
        # they never silently join a run that didn't name them explicitly.
        category_names = sorted(n for n in TEMPLATES if n not in NOT_DEFAULT)

    args.out.mkdir(parents=True, exist_ok=True)

    indices = range(args.start_index, args.start_index + args.count)

    # --cases-per-file 1 with --noise-level 0 (both defaults) is a byte-for-byte
    # passthrough of today's output - no mod-wrapping, no println! rewriting, so
    # nothing existing can regress just by this flag's presence in the codebase.
    if args.cases_per_file == 1 and args.noise_level == 0:
        generated = 0
        for i in indices:
            category_name = category_names[i % len(category_names)]
            template_fn = TEMPLATES[category_name]
            rng = random.Random(f"{args.seed}:{i}")
            program = template_fn(rng, i)

            rs_path = args.out / f"{program.name}.rs"
            expected_path = args.out / f"{program.name}.expected"
            rs_path.write_text(program.source)
            expected_path.write_text(program.expected_stdout)
            generated += 1

        print(f"generated {generated} programs across {len(category_names)} categories into {args.out}")
        return 0

    # Batched and/or noisy path: each sub-case's (Program, NoiseBundle) is
    # derived exactly the way a non-batched run would derive it - same
    # per-index category pick, same `random.Random(f"{seed}:{i}")` template
    # rng - so batching/noise only changes how many sub-cases share one
    # rustc invocation, never what's being tested. Noise draws come from a
    # separate, independently-seeded rng so they can never perturb (consume
    # extra draws from) the real template's own rng stream.
    generated = 0
    files_written = 0
    chunk: list[tuple] = []
    chunk_start = None

    def flush(chunk, chunk_start):
        nonlocal files_written
        if not chunk:
            return
        batch_name = f"batch_seed{args.seed}_start{chunk_start}_n{len(chunk)}"
        batch_file = batching.assemble_batch(chunk, batch_name, category="mixed")
        (args.out / f"{batch_file.name}.rs").write_text(batch_file.source)
        (args.out / f"{batch_file.name}.expected").write_text(batch_file.expected_stdout)
        files_written += 1

    for i in indices:
        category_name = category_names[i % len(category_names)]
        template_fn = TEMPLATES[category_name]
        rng = random.Random(f"{args.seed}:{i}")
        program = template_fn(rng, i)

        noise_bundle = None
        if args.noise_level:
            noise_rng = random.Random(f"noise:{args.seed}:{i}")
            noise_bundle = noise.generate(noise_rng, args.noise_level)

        if chunk_start is None:
            chunk_start = i
        chunk.append((program, noise_bundle))
        generated += 1

        if len(chunk) >= args.cases_per_file:
            flush(chunk, chunk_start)
            chunk = []
            chunk_start = None

    flush(chunk, chunk_start)

    print(
        f"generated {generated} sub-cases across {len(category_names)} categories "
        f"into {files_written} file(s) in {args.out} "
        f"(--cases-per-file {args.cases_per_file}, --noise-level {args.noise_level})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
