#!/usr/bin/env python3
# Generates the synthetic large-project repro used to measure candidate 009:
# a project.godot plus a `.gdignore`-marked directory tree simulating a bloated
# VCS/build-output directory (matches godotengine/godot#122438's repro shape),
# and a small `normal_dir` to confirm normal resources are unaffected.
import os
import sys

base = sys.argv[1] if len(sys.argv) > 1 else "."
os.makedirs(base, exist_ok=True)
with open(os.path.join(base, "project.godot"), "w") as f:
    f.write('; test project\nconfig_version=5\n\n[application]\n\nconfig/name="pcktest"\n'
            'config/features=PackedStringArray("4.8")\n')

excluded = os.path.join(base, "big_excluded_dir")
os.makedirs(excluded, exist_ok=True)
open(os.path.join(excluded, ".gdignore"), "w").close()

n_dirs = 50
n_files_per_dir = 300
for d in range(n_dirs):
    dp = os.path.join(excluded, f"d{d}")
    os.makedirs(dp, exist_ok=True)
    for i in range(n_files_per_dir):
        open(os.path.join(dp, f"f{i}.txt"), "w").close()

normal = os.path.join(base, "normal_dir")
os.makedirs(normal, exist_ok=True)
with open(os.path.join(normal, "keep.txt"), "w") as f:
    f.write("hello\n")

print(f"Generated repro project at {base}: {n_dirs * n_files_per_dir} files under "
      f"big_excluded_dir/ (marked with .gdignore), 1 file under normal_dir/")
