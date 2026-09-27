# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `cargo +nightly fuzz run <target>` | works, but hangs unless disarmed | ASan/libFuzzer's on-crash/on-new-coverage symbolizer subprocess misbehaves in this sandbox (blocked subprocess/ptrace permissions). Prefix every invocation with `ASAN_OPTIONS=symbolize=0` (add `external_symbolizer_path=/bin/false` if it still hangs). If a run doesn't finish, `pkill -9 -f "release/<target> -"` to reap the stuck fork - it never returns on its own. Same env var needed for `-minimize_crash=1` and single-artifact replay. |
| `miri` | not installed as of this session | `rustup +nightly component add miri` needed first; not yet verified to work on this codebase (unsafe-heavy crates: check `pixeldata` decoders first if reached for). |
| debug-assertions build | works normally | `cargo build` (dev profile) already has overflow checks on; no special setup. |
| ASAN/UBSAN outside cargo-fuzz (`-Z sanitizer=address` on a plain bin/test) | not yet tried this session | note here once tried. |
| disk space | tight | 87G free / 1.8T, 96% used at session start - `cargo clean` on crates not part of the in-progress candidate if a sanitizer/fuzz build run gets close to filling it. |

## Existing project harnesses worth reusing

Official upstream fuzz targets (tracked in git, `cargo fuzz run <name>` from inside the crate's
`fuzz/` dir):
- `object/fuzz/fuzz_targets/open_file.rs`
- `pixeldata/fuzz/fuzz_targets/decode_image_file.rs`
- `pixeldata/fuzz/fuzz_targets/decode_simple_image.rs`
- `ul/fuzz/fuzz_targets/pdu_roundtrip.rs`

Leftover, **untracked** directories from a prior `fuzz-bug-repro` campaign session (not part of
this repo's git history, still present on disk in this checkout): `core/fuzz/`, `parser/fuzz/`,
`json/fuzz/` - only `Cargo.lock`/`corpus/` survived, the actual `fuzz_targets/*.rs` files from that
campaign are gone from this checkout. If a candidate needs a `core`/`parser`/`json`-level fuzz
harness, write a fresh short one-off (`templates/fuzz_harness_template.rs`) rather than assuming
these directories are ready to use - check what's actually in them first.

`dcms/` at the repo root (untracked, left from the same campaign): ~13 real-world `.dcm` sample
files (JPEG2000, RGB, YBR, palette, overlay, waveform, DICOMDIR, RT dose) - good corpus seeds or
manual round-trip test inputs.

`___FUZZ_FINDINGS___/` and `___FINDINGS___/` at the repo root (both untracked): records from prior
sessions (a `fuzz-bug-repro` campaign and a `code-audit-structured` pass). Read-only references for
this session, not part of this workspace - see `IDEAS.md` for the two leads pulled from
`___FUZZ_FINDINGS___/`'s campaign notes into this session's backlog.

## Instrumentation/tooling set up for this workspace

Nothing added yet this session - update this table as tools get set up for real.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every `Confirmed` candidate has a real,
   standalone `repro/` and the exact command that runs it.
2. Every candidate with a fix gets a real `patch.diff`, applied and built for real, plus the
   project's own tests where they cover the code.
3. `Not reproducible`/`Inconclusive` results are written down with the same care as confirmed
   bugs - a result with no file is indistinguishable from "never checked".
4. If a later candidate changes an earlier one's confirmation status, don't silently rewrite that
   candidate's `RESULT.md` - add a `CORRECTION.md` in its folder and update its `SUMMARY.md` row
   to point at it.
