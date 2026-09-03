#!/usr/bin/env python3
# Generates a .slint fixture of N distinct, trivial custom components (Card0..CardN-1, each a
# bare Rectangle+Text with no accessible-role/geometry overrides), all instantiated once inside
# one root Window. Each Card becomes its own generated Rust struct+impl (a distinct
# "sub_component" in the compiler's LLR), which is what makes the per-component method-skeleton
# cost in generator/rust.rs scale with N instead of staying constant - a fixture that only
# repeats *instances* of the same component (e.g. a `for` loop) would not exercise this, since
# the codegen for a repeated component is only emitted once regardless of instance count.
import sys

N = int(sys.argv[1]) if len(sys.argv) > 1 else 400
out = sys.argv[2] if len(sys.argv) > 2 else f"fixture_{N}.slint"

lines = []
for i in range(N):
    lines.append(f"component Card{i} inherits Rectangle {{")
    lines.append("    width: 10px; height: 10px;")
    lines.append(f'    Text {{ text: "hi{i}"; }}')
    lines.append("}")
lines.append("export component MainWindow inherits Window {")
for i in range(N):
    lines.append(f"    Card{i} {{ x: {i}px; y: 0px; }}")
lines.append("}")

with open(out, "w") as f:
    f.write("\n".join(lines))
print(f"wrote {out} ({N} components)")
