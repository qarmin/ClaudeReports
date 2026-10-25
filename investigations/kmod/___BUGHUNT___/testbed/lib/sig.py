"""Failure-signature extraction shared by all testbed components."""
import re

_SAN = re.compile(r"ERROR: (AddressSanitizer|LeakSanitizer|UndefinedBehaviorSanitizer): ?([\w-]+)?")
_UBSAN = re.compile(r"(\S+\.c):(\d+):\d+: runtime error: (.*)")
_ASSERT = re.compile(r"(\S+\.c):(\d+): .*Assertion `(.*)' failed")
_FRAME = re.compile(r"#\d+ 0x[0-9a-f]+ in (\S+) (\S*?(?:libkmod|shared|tools)/[\w./-]+\.c):(\d+)")
_LIBFUZZER = re.compile(r"ERROR: libFuzzer: ([\w-]+)")
_MARK = re.compile(r"^(INVARIANT|DIFF) ([\w-]+)", re.M)


def _short(path):
    m = re.search(r"((?:libkmod|shared|tools)/[\w./-]+)", path)
    return m.group(1) if m else path


def signature(text):
    """Stable one-line signature for a crash/sanitizer/assert/invariant report, or None."""
    m = _MARK.search(text)
    if m:
        return f"{m.group(1).lower()}:{m.group(2)}"
    m = _ASSERT.search(text)
    if m:
        return f"assert:{_short(m.group(1))}:{m.group(2)}"
    m = _UBSAN.search(text)
    if m:
        kind = re.sub(r"[0-9]+|0x[0-9a-f]+|'[^']*'", "N", m.group(3))[:60]
        return f"ubsan:{_short(m.group(1))}:{m.group(2)}:{kind}"
    m = _SAN.search(text)
    if m:
        kind = m.group(2) or m.group(1)
        frame = _FRAME.search(text[m.start():])
        where = f"{frame.group(1)}@{_short(frame.group(2))}:{frame.group(3)}" if frame else "?"
        if m.group(1) == "LeakSanitizer":
            kind = "leak"
        return f"asan:{kind}:{where}"
    m = _LIBFUZZER.search(text)
    if m:
        frame = _FRAME.search(text[m.start():])
        where = f"{frame.group(1)}@{_short(frame.group(2))}:{frame.group(3)}" if frame else "?"
        return f"libfuzzer:{m.group(1)}:{where}"
    return None


def first_line(text):
    for line in text.splitlines():
        if any(k in line for k in ("ERROR:", "runtime error", "Assertion", "INVARIANT", "DIFF")):
            return line.strip()[:200]
    return text.strip().splitlines()[0][:200] if text.strip() else ""
