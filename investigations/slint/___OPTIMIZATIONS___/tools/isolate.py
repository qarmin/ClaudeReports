#!/usr/bin/env python3
"""Disable one applied fix at a time, so each can be measured on its own.

Ported on 2026-08-23 from `perf_test/___OPTIMIZATIONS___/tools/isolate.py` (see `../SUMMARY.md`
header for provenance) - only the REPO path resolution changed (this file now lives one directory
level closer to the repo root, since `___OPTIMIZATIONS___` sits at the repo root directly instead
of inside `perf_test/`). The FIXES table below is unchanged and still describes the real patches
from that session.

Several of that session's fixes live in the same files (03, 19 and 23 all touch `sharedparley/`),
so reverting a *file* measures a bundle rather than a fix (see MEASUREMENT.md section 18). Each
entry below is the smallest edit that turns exactly one fix off while leaving the others in place.

    tools/isolate.py list                 # what can be toggled
    tools/isolate.py off <fix>            # write the disabled variant into the tree
    tools/isolate.py restore              # put the tree back

`off` saves the originals under `target/isolate_backup` and `restore` puts them back; both touch
the files afterward, because cargo skips a rebuild when a restored file's mtime goes backwards
(see MEASUREMENT.md section 1).

Two fixes in the *same* file can be disabled together (08 and 14 both live in `properties.rs`).
The backups are numbered in application order and restored in reverse, so the file ends up as it
started; restoring them in name order restores an intermediate state instead, which is a silent
way to poison every measurement that follows.

NOTE: these edits reference candidates 08/09/14/19/23/26/29/41 - none of them are currently
applied to this working tree (this port only restates the historical record; it does not apply
anything). `off`/`restore` only make sense after you've actually applied the relevant patch.diff
from this workspace to try isolating one part of it.
"""
import pathlib
import shutil
import subprocess
import sys

# This file lives at <repo_root>/___OPTIMIZATIONS___/tools/isolate.py - two parents up is the repo root.
REPO = pathlib.Path(__file__).resolve().parents[2]
# Not /tmp: it is not durable across a long session, and a wipe between `off` and `restore`
# leaves the tree in the disabled state with nothing to put back. `target/` is gitignored and
# survives as long as the checkout does.
BACKUP = REPO / "target" / "isolate_backup"

# fix -> (file, [(what the applied code looks like, what it replaces it with)])
FIXES = {
    "14-node-pooling": (
        "internal/core/properties.rs",
        [
            (
                """                unsafe {
                    node_pool::with(|pool| {
                        (*(*binding_ptr).dep_nodes.get()).recycle_into(pool, |node| node.remove())
                    })
                };""",
                """                unsafe { *(*binding_ptr).dep_nodes.get() = Default::default() };""",
            ),
            (
                """        unsafe {
            node_pool::with(|pool| {
                (*self.holder.dep_nodes.get()).recycle_into(pool, |node| node.remove())
            })
        };""",
                """        unsafe { *self.holder.dep_nodes.get() = Default::default() };""",
            ),
        ],
    ),
    # From an earlier session, included here because it lives in the same file as 14 and the two
    # interact by construction: 08 avoids a registration, 14 makes a registration cheap.
    "08-skip-duplicates": (
        "internal/core/properties.rs",
        [
            (
                """        if unsafe { (*property_that_will_notify).front_is(self_ptr) } {
            #[cfg(feature = "dep-stats")]
            dep_stats::record_skipped();
            return;
        }
""",
                "",
            ),
        ],
    ),
    "19-selection-band": (
        "internal/core/textlayout/sharedparley/selection.rs",
        [
            (
                """        let visible = self.visible_paragraphs();
        let first = visible
            .partition_point(|p| {
                self.y_offset + p.y + PhysicalLength::new(p.metrics.height) < visible_band.start
            })
            .saturating_sub(1);

        for paragraph_index in first..visible.len() {""",
                """        for paragraph_index in 0..self.visible_paragraphs().len() {""",
            ),
            (
                """            if paragraph_top - line_pad > visible_band.end {
                break;
            }
            if paragraph_top + paragraph_height + line_pad < visible_band.start {
                continue;
            }""",
                """            if paragraph_top + paragraph_height + line_pad < visible_band.start
                || paragraph_top - line_pad > visible_band.end
            {
                continue;
            }""",
            ),
        ],
    ),
    "23-glyph-cull": (
        "internal/core/textlayout/sharedparley/draw.rs",
        [
            (
                """        let mut glyphs_it = glyphs_it.filter(move |glyph| {
            columns.as_ref().is_none_or(|columns| {""",
                """        let mut glyphs_it = glyphs_it.filter(move |glyph| {
            let _ = &columns;
            true || columns.as_ref().is_none_or(|columns| {""",
            ),
        ],
    ),
    "08b-cull-geometry-read": (
        "internal/core/partial_renderer.rs",
        [
            (
                """        if !draw && let Some(offset) = cached_offset {""",
                """        let _ = &cached_offset;
        if false && let Some(offset) = cached_offset {""",
            ),
        ],
    ),
    "09-conditional-state-stack": (
        "internal/core/item_rendering.rs",
        [
            (
                """            if renders {
                renderer.save_state();
            }""",
                """            if true {
                renderer.save_state();
            }""",
            ),
            (
                """            if renders {
                renderer.restore_state();
            } else {
                renderer.translate(-item_origin.to_vector());
            }""",
                """            if true {
                renderer.restore_state();
            } else {
                renderer.translate(-item_origin.to_vector());
            }""",
            ),
        ],
    ),
    "41-svg-per-frame-cache": (
        "internal/renderers/software/lib.rs",
        [
            (
                """                let cached = cache_key
                    .as_ref()
                    .and_then(|key| self.scalable_source_cache.get(key))
                    .cloned();""",
                """                let cached: Option<SharedImageBuffer> = {
                    let _ = &self.scalable_source_cache;
                    None
                };""",
            ),
        ],
    ),
    "26-text-cache-sweep": (
        "internal/core/textlayout/sharedparley/cache.rs",
        [
            (
                """    if cache.live.get() > ENTRY_LIMIT {
        cache.sweep();
    }""",
                """    if false && cache.live.get() > ENTRY_LIMIT {
        cache.sweep();
    }""",
            ),
        ],
    ),
    "29-sort-append": (
        "internal/core/model/adapters.rs",
        [
            (
                """        let appended_at_end = index >= self.wrapped_model.row_count().saturating_sub(count);
        if !appended_at_end {
            for row in self.mapping.borrow_mut().iter_mut() {
                if *row >= index {
                    *row += count;
                }
            }
        }""",
                """        for row in self.mapping.borrow_mut().iter_mut() {
            if *row >= index {
                *row += count;
            }
        }""",
            ),
        ],
    ),
}

# 03 is the whole `sharedparley` refactor and has no one-line off switch: reverting it means
# reverting the directory, which takes 19 and 23 with it. Measure it as "textlayout reverted"
# against "textlayout with 19 and 23 already disabled" so the comparison is still one change.
NOTE_03 = """03-lazy-shaping has no toggle: it is the split of `TextParagraph` into metrics and glyphs,
which every other change in `sharedparley/` builds on. To measure it alone, disable 19 and 23
first, build that as the baseline, then `git checkout -- internal/core/textlayout`."""


def touch(path: pathlib.Path) -> None:
    subprocess.run(["touch", str(path)], check=True)


def main() -> int:
    if len(sys.argv) < 2 or sys.argv[1] == "list":
        print("toggleable fixes:")
        for name, (path, _) in FIXES.items():
            print(f"  {name:20} {path}")
        print()
        print(NOTE_03)
        return 0

    if sys.argv[1] == "restore":
        if not BACKUP.exists():
            print("nothing to restore")
            return 0
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
