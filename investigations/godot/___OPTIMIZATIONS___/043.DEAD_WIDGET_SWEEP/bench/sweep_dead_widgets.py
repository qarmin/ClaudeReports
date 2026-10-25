#!/usr/bin/env python3
"""Find member widgets that are constructed but never shown.

Run from the root of a Godot checkout: python3 sweep_dead_widgets.py

Reports every `member = memnew(<DialogClass>)` whose member has no call that could make it visible.
Note the three indirection patterns documented in ../RESULT.md - this script deliberately reports
them as hits rather than filtering them, because filtering them automatically is what makes the
result untrustworthy. Read each hit; as of 9552dfb685 exactly one (file_templates) was real.
"""
import re, subprocess, sys

CLASSES = ['EditorFileDialog', 'ConfirmationDialog', 'AcceptDialog', 'FileDialog',
           'EditorHelpSearch', 'PopupPanel', 'PopupMenu']
SEARCH_DIRS = 'editor/ scene/ modules/ main/'

files = subprocess.run(['bash', '-c', "grep -rl 'memnew(' editor/ scene/gui/ --include=*.cpp"],
                       capture_output=True, text=True).stdout.split()
if not files:
    sys.exit("no sources found - run this from the root of a Godot checkout")

assign_re = re.compile(r'^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*memnew\(\s*(' + '|'.join(CLASSES) + r')\s*\)', re.M)
cands = {}
for f in files:
    try:
        src = open(f).read()
    except OSError:
        continue
    for m in assign_re.finditer(src):
        cands.setdefault((m.group(1), m.group(2)), f)

noshow = []
for (member, cls), f in cands.items():
    hits = subprocess.run(
        ['bash', '-c', f"grep -rn '\\b{member}\\b' --include=*.cpp --include=*.h {SEARCH_DIRS} 2>/dev/null"],
        capture_output=True, text=True).stdout
    shown = (re.search(rf'{member}\s*->\s*(popup\w*|show|set_visible)\s*\(', hits)
             or re.search(rf'\(\s*Window\s*\*\s*\)\s*{member}\b', hits)
             or re.search(rf'{member}\b[^\n]*(Window::popup|AcceptDialog::popup|popup_centered)', hits)
             or re.search(rf'return\s+{member}\s*;', hits))
    if not shown:
        noshow.append((len([l for l in hits.split('\n') if l.strip()]), cls, member, f))

print(f"{len(cands)} member dialog constructions scanned; {len(noshow)} with no direct show call.\n")
print("Read every hit - see ../RESULT.md for the 3 indirection patterns that produce false positives")
print("(submenu attachment, callable_mp bind, dialog-passed-as-argument helpers).\n")
for n, cls, member, f in sorted(noshow):
    print(f"  {n:3d} refs  {cls:20s} {member:34s} {f}")
