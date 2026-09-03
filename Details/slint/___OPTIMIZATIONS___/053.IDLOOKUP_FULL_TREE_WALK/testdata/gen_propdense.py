#!/usr/bin/env python3
"""Generate a flat .slint file with N sibling Rectangles, each with several
plain-property-identifier bindings (in-scope lookups, not element-id lookups),
to stress IdLookup's full-tree walk before InScopeLookup succeeds."""
import sys

n = int(sys.argv[1])
refs_per_elem = int(sys.argv[2]) if len(sys.argv) > 2 else 4
out = sys.argv[3] if len(sys.argv) > 3 else f"/tmp/propdense_{n}.slint"

lines = []
lines.append("export component AppWindow inherits Window {")
lines.append("    width: 800px;")
lines.append("    height: 600px;")
for i in range(n):
    lines.append("    Rectangle {")
    lines.append("        property <length> p0: 1px;")
    for j in range(1, refs_per_elem):
        lines.append(f"        property <length> p{j}: p{j-1} + 1px;")
    lines.append(f"        x: p{refs_per_elem-1};")
    lines.append("    }")
lines.append("}")

with open(out, "w") as f:
    f.write("\n".join(lines) + "\n")
print(out)
