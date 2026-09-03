#!/usr/bin/env python3
"""Disable one applied fix at a time, so each can be measured on its own.

When several fixes land in the same file, reverting the whole file measures a *bundle*, not a
fix (see MEASUREMENT.md section 18). Each entry in FIXES below is the smallest edit that turns
exactly one fix off while leaving the others in place - usually restoring the loop or call site
it replaced. Write this switch when you write the fix, not afterward - reconstructing it later
means re-deriving the original code from a diff.

    tools/isolate.py list                 # what can be toggled
    tools/isolate.py off <fix>            # write the disabled variant into the tree
    tools/isolate.py restore              # put the tree back

ADAPT: fill in FIXES below (one entry per candidate that needs isolating - not every candidate
does; a single-file, single-change candidate can just be `git checkout`-reverted directly).

`off` saves the originals under `target/isolate_backup` and `restore` puts them back; both touch
the files afterward, because cargo (or an equivalent build tool) skips a rebuild when a restored
file's mtime goes backwards (see MEASUREMENT.md section 1).

Two fixes in the *same* file can be disabled together. Backups are numbered in application order
and restored in reverse, so the file ends up exactly as it started - restoring in name order
instead would leave an intermediate state, which silently poisons every measurement that follows.
"""
import pathlib
import shutil
import subprocess
import sys

# ADAPT: point this at the project root (however many parents up this script actually lives).
REPO = pathlib.Path(__file__).resolve().parents[3]
# Not /tmp - it is not durable across a long session (killed by an unrelated cleanup, a reboot,
# a container recycle) and a wipe between `off` and `restore` leaves the tree disabled with
# nothing to put back. `target/` is gitignored and survives as long as the checkout does - use
# the same rule for any other scratch state (comparison binaries, saved trees) this workspace
# needs to survive between commands.
BACKUP = REPO / "target" / "isolate_backup"

# fix name -> (file relative to REPO, [(applied code, disabled code), ...])
FIXES = {
    # "example-fix-name": (
    #     "src/some_file.rs",
    #     [
    #         ("applied code exactly as it appears in the file", "code to swap it back to"),
    #     ],
    # ),
}


def touch(path: pathlib.Path) -> None:
    subprocess.run(["touch", str(path)], check=True)


def main() -> int:
    if len(sys.argv) < 2 or sys.argv[1] == "list":
        print("toggleable fixes:")
        for name, (path, _) in FIXES.items():
            print(f"  {name:20} {path}")
        return 0

    if sys.argv[1] == "restore":
        if not BACKUP.exists():
            print("nothing to restore")
            return 0
        # Reverse application order: a second `off` on the same file backed up the
        # already-disabled version, so restoring oldest-last is what puts the original back.
        for saved in sorted(BACKUP.glob("*.path"), reverse=True):
            rel = saved.read_text().strip()
            content = (BACKUP / (saved.stem + ".bak")).read_bytes()
            (REPO / rel).write_bytes(content)
            touch(REPO / rel)
            print(f"restored {rel}")
        shutil.rmtree(BACKUP)
        return 0

    if sys.argv[1] != "off" or len(sys.argv) != 3:
        print(__doc__)
        return 2

    fix = sys.argv[2]
    if fix not in FIXES:
        print(f"unknown fix `{fix}`; try `list`")
        return 2

    rel, edits = FIXES[fix]
    path = REPO / rel
    source = path.read_text()

    BACKUP.mkdir(parents=True, exist_ok=True)
    stem = f"{len(list(BACKUP.glob('*.path'))):02d}_{fix.replace('/', '_')}"
    (BACKUP / f"{stem}.path").write_text(rel)
    (BACKUP / f"{stem}.bak").write_bytes(path.read_bytes())

    for applied, disabled in edits:
        if applied not in source:
            print(f"FAILED: the applied form of `{fix}` is not in {rel} - has it been edited?")
            shutil.rmtree(BACKUP)
            return 1
        source = source.replace(applied, disabled)

    path.write_text(source)
    touch(path)
    print(f"{fix} disabled in {rel} (restore with: tools/isolate.py restore)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
