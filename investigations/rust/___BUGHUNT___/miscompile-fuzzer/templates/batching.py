"""Shared batched-file assembly, reused by generator.py and
static_generator.py (`--cases-per-file`, `--noise-level`). See README.md
"Batching for throughput" and "Noise injection" for the full rationale.

Batching packs many independent sub-cases - each one already a complete,
self-contained `.rs` program produced unmodified by an existing category
template in `templates/` or `templates_static/` - into ONE `.rs` file, so a
single rustc invocation amortizes process-startup overhead across all of
them. This module doesn't know or care which generator mode produced a given
sub-case's `Program` (see `common.py`); it only needs one `Program` plus an
optional `NoiseBundle` per sub-case.

How a sub-case gets isolated: every sub-case's entire original source
(everything it would have been, as a standalone `.rs` file) is wrapped in its
own `mod case_<n> { ... }`. This namespaces all of its top-level items
automatically, including the handful of templates that intentionally reuse a
fixed, non-`idx`-qualified name across invocations (e.g. `cell_aliasing.py`'s
`struct Holder`) - those would otherwise collide if two sub-cases of the same
category shared a batch file. `mod` isolation makes this correct regardless
of any individual template's naming convention, so no template file needs to
change for batching to be safe.

Attributing a divergence to one sub-case: every `println!(...)` call in a
sub-case's body gets a `print!("case_<n>:")` spliced immediately before it
(a verbatim, uninterpreted text substitution - no parsing of the format
string), and the corresponding `.expected` lines get the same `case_<n>:`
prefix. Because sub-cases run strictly in sequence from `main()`, this makes
every line of stdout (and of `.expected`) self-identifying, so a plain
line-by-line diff of two stdout captures (see `diff_runner.py`'s REPORT.md
diff section) immediately shows which sub-case's line actually differs.
"""

from __future__ import annotations

import dataclasses

from .common import Program
from .noise import NoiseBundle

_MAIN_MARKER = "fn main() {"


def split_program_source(source: str) -> tuple[str, str]:
    """-> (preamble_before_fn_main, body_inside_fn_mains_braces).

    Relies on an invariant true of every template in this tool (both
    `templates/` and `templates_static/`): `fn main() {` is always the final
    top-level item in the generated source, so its closing brace is the very
    last `}` in the file - this lets us split without a full brace-matching
    parser.
    """
    idx = source.index(_MAIN_MARKER)
    preamble = source[:idx].rstrip("\n")
    rest = source[idx + len(_MAIN_MARKER) :].rstrip()
    if not rest.endswith("}"):
        raise ValueError("generated source's fn main() has no matching closing brace at end of file")
    return preamble, rest[:-1]


def _attribute_prints(body: str, case_n: int) -> str:
    return body.replace("println!(", f'print!("case_{case_n}:"); println!(')


def assemble_case_module(program: Program, case_n: int, noise: NoiseBundle | None) -> tuple[str, str]:
    """-> (rust_source_for_this_mod, this_sub-case's_attributed_expected_stdout)."""
    preamble, body = split_program_source(program.source)

    attr_line, _, real_items = preamble.partition("\n")
    if not attr_line.startswith("#!["):
        raise ValueError(
            f"expected every template's preamble to start with an inner attribute, got: {attr_line!r}"
        )

    noise = noise or NoiseBundle()
    noise_items_src = "\n\n".join(noise.items)
    noise_pre_src = "\n        ".join(noise.pre_stmts)
    noise_post_src = "\n        ".join(noise.post_stmts)
    body = _attribute_prints(body, case_n)

    mod_src = f"""mod case_{case_n} {{
    {attr_line}

{noise_items_src}

{real_items}

    pub fn run() {{
        {noise_pre_src}
        {{
{body}
        }}
        {noise_post_src}
    }}
}}
"""
    expected = "".join(f"case_{case_n}:{line}\n" for line in program.expected_stdout.splitlines())
    return mod_src, expected


@dataclasses.dataclass
class BatchFile:
    name: str
    source: str
    expected_stdout: str
    category: str


def assemble_batch(
    cases: list[tuple[Program, NoiseBundle | None]],
    batch_name: str,
    category: str,
) -> BatchFile:
    """`cases` is one (Program, NoiseBundle-or-None) per sub-case, in the
    exact order they'll run (and print) in the assembled file's `main()`.
    """
    mod_blocks = []
    expected_blocks = []
    calls = []
    for n, (program, noise) in enumerate(cases):
        mod_src, expected = assemble_case_module(program, n, noise)
        mod_blocks.append(mod_src)
        expected_blocks.append(expected)
        calls.append(f"    case_{n}::run();")

    header = (
        "// Batched corpus file: multiple independent sub-cases, each isolated in its\n"
        "// own `mod case_N`, called in sequence from main() so a single rustc\n"
        "// invocation amortizes process-startup overhead across all of them.\n"
        "// See README.md \"Batching for throughput\". Do not hand-edit - regenerate\n"
        "// via generator.py/static_generator.py --cases-per-file instead.\n\n"
    )
    source = header + "\n".join(mod_blocks) + "\nfn main() {\n" + "\n".join(calls) + "\n}\n"
    return BatchFile(name=batch_name, source=source, expected_stdout="".join(expected_blocks), category=category)
