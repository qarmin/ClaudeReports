#!/usr/bin/env python3
"""Regenerate the class-filtered summary derivatives from SUMMARY.md/SUMMARY-SHORT.md.

    gen_summaries.py <bughunt_dir>

Writes SUMMARY-SHORT-CRASHES.md / SUMMARY-SHORT-LOGIC.md (filtered from SUMMARY-SHORT.md) and
SUMMARY-CRASHES.md / SUMMARY-LOGIC.md (filtered from SUMMARY.md, full detail). All four are pure
generated output - never hand-edit them, edit SUMMARY.md/SUMMARY-SHORT.md instead and re-run this
(every `just pack*` recipe in the root justfile already does, so this is rarely run directly).

Split rule: a Confirmed row whose Class is exactly "logic" goes to the LOGIC file; every other
Confirmed row (Class "crash", "memory-safety", "security", or any combination including one of
those, e.g. "crash/logic") goes to the CRASHES file. Never both, never neither.

The two full-detail files additionally annotate Root-cause cells that mention another candidate's
number ("candidate 049", "candidates 006/007") when that candidate isn't itself a row in the same
filtered file (wrong class, or not Confirmed) - appends one short note instead of leaving a
dangling reference, since the crash-only/logic-only zip a reader gets via `just pack-crash`/
`just pack-logic` won't contain that other candidate's folder either.
"""
import re
import sys
from pathlib import Path

NUM_RE = re.compile(r'candidates?\s+((?:\d{2,4}(?:\s*[/,]\s*\d{2,4})*))', re.IGNORECASE)


def split_cells(line):
    protected = line.rstrip('\n').replace('\\|', '\x00')
    parts = protected.split('|')
    return [p.strip().replace('\x00', '\\|') for p in parts[1:-1]]


def find_table(lines):
    for i, line in enumerate(lines):
        if line.startswith('|') and i + 1 < len(lines) and re.match(r'^\|[\s|:-]+\|$', lines[i + 1].strip()):
            header = line
            sep = lines[i + 1]
            rows = []
            j = i + 2
            while j < len(lines) and lines[j].startswith('|'):
                rows.append(lines[j])
                j += 1
            return header, sep, rows
    raise SystemExit('no markdown table found')


def col_index(header_cells, name_prefix):
    for i, c in enumerate(header_cells):
        if c == name_prefix or c.startswith(name_prefix):
            return i
    return None


def find_refs(text):
    nums = set()
    for m in NUM_RE.finditer(text):
        for n in re.split(r'[/,]\s*', m.group(1)):
            n = n.strip()
            if n:
                nums.add(n)
    return nums


def is_crash_bucket(cls):
    return cls.strip().lower() != 'logic'


def render(cells):
    return '| ' + ' | '.join(cells) + ' |'


def write_file(out_path, title_kind, source_name, sibling_note, header, sep, rows):
    out_path.write_text(
        '\n'.join(
            [
                f'# Confirmed {title_kind}-class bugs (Status: Confirmed only)',
                '',
                'GENERATED FILE - do not hand-edit. Regenerate with `just gen-summaries` (or any `just',
                f'pack*` recipe, which always regenerates this first) after changing {source_name}.',
                '',
                sibling_note,
                '',
                header,
                sep,
            ]
            + rows
        )
        + '\n'
    )


def gen_full(src_path, crashes_out, logic_out):
    lines = src_path.read_text().splitlines()
    header, sep, rows = find_table(lines)
    hc = split_cells(header)
    idx_num = col_index(hc, '#')
    idx_status = col_index(hc, 'Status')
    idx_class = col_index(hc, 'Class')
    idx_root = col_index(hc, 'Root cause')
    if None in (idx_num, idx_status, idx_class, idx_root):
        raise SystemExit(f'{src_path}: could not locate required columns in header {hc}')

    parsed = [split_cells(r) for r in rows]
    confirmed = [c for c in parsed if c[idx_status] == 'Confirmed']
    crashes = [c for c in confirmed if is_crash_bucket(c[idx_class])]
    logic = [c for c in confirmed if not is_crash_bucket(c[idx_class])]

    def annotated_rows(bucket):
        nums = {c[idx_num] for c in bucket}
        out = []
        for c in bucket:
            cells = list(c)
            refs = find_refs(cells[idx_root])
            excluded = sorted(n for n in refs if n not in nums and n != c[idx_num])
            if excluded:
                cells[idx_root] += (
                    f" *(also references candidate {', '.join(excluded)} - not in this file, "
                    f"see the project's full `SUMMARY.md`.)*"
                )
            out.append(render(cells))
        return out

    write_file(
        crashes_out, 'crash', '`SUMMARY.md`',
        'Filtered from `SUMMARY.md` - full detail. Pure `logic`-class Confirmed rows are in '
        '`SUMMARY-LOGIC.md` instead. A Root cause cell mentioning another candidate is annotated '
        'when that candidate is not itself a row in this same file.',
        header, sep, annotated_rows(crashes),
    )
    write_file(
        logic_out, 'logic', '`SUMMARY.md`',
        'Filtered from `SUMMARY.md` - full detail. Crash/security/memory-safety-class Confirmed '
        'rows (including a mixed class like `crash/logic`) are in `SUMMARY-CRASHES.md` instead. A '
        'Root cause cell mentioning another candidate is annotated when that candidate is not '
        'itself a row in this same file.',
        header, sep, annotated_rows(logic),
    )
    return len(crashes), len(logic)


def gen_short(src_path, crashes_out, logic_out):
    lines = src_path.read_text().splitlines()
    header, sep, rows = find_table(lines)
    hc = split_cells(header)
    idx_class = col_index(hc, 'Class')
    if idx_class is None:
        raise SystemExit(f'{src_path}: could not locate Class column in header {hc}')

    parsed = [split_cells(r) for r in rows]
    crashes = [c for c in parsed if is_crash_bucket(c[idx_class])]
    logic = [c for c in parsed if not is_crash_bucket(c[idx_class])]

    write_file(
        crashes_out, 'crash', '`SUMMARY-SHORT.md`',
        'Filtered from `SUMMARY-SHORT.md`. Pure `logic`-class rows are in '
        '`SUMMARY-SHORT-LOGIC.md` instead. See `SUMMARY.md`/`SUMMARY-CRASHES.md` for every '
        'candidate investigated (including Not-reproducible/Inconclusive ones) and full detail.',
        header, sep, [render(c) for c in crashes],
    )
    write_file(
        logic_out, 'logic', '`SUMMARY-SHORT.md`',
        'Filtered from `SUMMARY-SHORT.md`. Crash/security/memory-safety-class rows (including a '
        'mixed class like `crash/logic`) are in `SUMMARY-SHORT-CRASHES.md` instead. See '
        '`SUMMARY.md`/`SUMMARY-LOGIC.md` for every candidate investigated (including '
        'Not-reproducible/Inconclusive ones) and full detail.',
        header, sep, [render(c) for c in logic],
    )
    return len(crashes), len(logic)


def main():
    if len(sys.argv) != 2:
        raise SystemExit(f'usage: {sys.argv[0]} <bughunt_dir>')
    d = Path(sys.argv[1])
    summary = d / 'SUMMARY.md'
    summary_short = d / 'SUMMARY-SHORT.md'
    if not summary.exists() or not summary_short.exists():
        raise SystemExit(f'{summary} / {summary_short} must both exist')

    nc, nl = gen_full(summary, d / 'SUMMARY-CRASHES.md', d / 'SUMMARY-LOGIC.md')
    print(f'SUMMARY-CRASHES.md: {nc} rows, SUMMARY-LOGIC.md: {nl} rows')
    nc, nl = gen_short(summary_short, d / 'SUMMARY-SHORT-CRASHES.md', d / 'SUMMARY-SHORT-LOGIC.md')
    print(f'SUMMARY-SHORT-CRASHES.md: {nc} rows, SUMMARY-SHORT-LOGIC.md: {nl} rows')


if __name__ == '__main__':
    main()
