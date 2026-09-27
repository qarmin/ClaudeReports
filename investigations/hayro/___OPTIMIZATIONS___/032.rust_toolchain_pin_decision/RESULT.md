# Should this project pin a `rust-toolchain.toml`?

Candidate 016 traced its own 92-vs-28 call-count discrepancy (two same-methodology sessions
measuring the identical unsplit `irreversible_filter_97i_horizontal_simd::<Avx512>`) to the most
likely explanation left standing after ruling out build method/diagnostic module/dependency
version: unrecorded `rustc`/LLVM version drift between sessions, since this project has no
`rust-toolchain.toml` and `cargo`/`rustc` silently follow whatever `rustup`'s `stable` alias
resolves to at the moment any command runs. This candidate investigates whether pinning fixes
that, and whether pinning has any measurable cost today.

`rustc --version` / `cargo --version` for every measurement below: `rustc 1.98.0 (88d9e12ae
2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)` (this machine's current `stable`, confirmed via
`rustup show` before starting).

## What was tried

The patch adds `rust-toolchain.toml` at the repo root pinning `channel = "1.98.0"` - the exact
version this session's `stable` already resolves to, not the workspace's separately-tracked MSRV
(`rust-version = "1.92"` in the root `Cargo.toml`, already enforced in CI via
`dtolnay/rust-toolchain@1.92.0` for the `min-version`/`checks`/`load_tests` jobs - CI already pins
per-job for its own purposes; this candidate is about *local* dev/session reproducibility, which CI
pinning doesn't cover). Pinning to the currently-installed `stable` version, rather than to the
MSRV, was a deliberate choice: it costs nothing today (same compiler either way) and doesn't
retroactively change the toolchain that produced the last 31 candidates' own codegen-artifact
measurements in this workspace, unlike pinning to 1.92.0 would.

Four things were checked, each build/bench/test step run through the shared `flock` build lock:

1. **Does pinning change the compiler actually used?** No - `rustc --version` is byte-identical
   before and after (`rustup show active-toolchain` confirms the pin resolves to
   `1.98.0-x86_64-unknown-linux-gnu`, same build hash `88d9e12ae` as the unpinned `stable`).
2. **Is a `cargo build --release` binary bit-identical to itself across two independent clean
   rebuilds under the same toolchain?** Yes - built `hayro-bench`'s `render_bench` twice from a
   clean `target/`, same pinned toolchain, and `sha256sum` matched exactly
   (`97eefe6f35661c2ecf1430abb058305ff9f7d4e7d95ec04853c73eade5450e73` both times). This
   independently confirms this project's release builds are otherwise deterministic given a fixed
   toolchain - the pin's job is only to keep that toolchain fixed *across sessions*, not to fix any
   existing nondeterminism (there isn't any observed here).
3. **Is the pinned build's raw binary byte-identical to the unpinned (`stable`) build's binary?**
   No - `cmp` reports the first difference at byte 25, and ~70% of bytes differ
   (`cmp -l | wc -l` = 9,930,857 of 14,149,720 bytes). `strings` on both binaries shows why: the
   unpinned binary embeds `.../toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/...`
   in debug-info (`[profile.release] debug = 1` is set in this workspace's `Cargo.toml`) line-table
   paths, the pinned one embeds `.../toolchains/1.98.0-x86_64-unknown-linux-gnu/lib/rustlib/...`
   instead - two distinct toolchain install directories for the literal same rustc build, since
   rustup materializes a `1.98.0`-named toolchain separately from the pre-existing `stable`-named
   one even though they resolve to the same compiler. Both path strings are coincidentally the same
   length (32 chars), so this isn't simple reflow; the wide diff is consistent with rustc's debug
   line-table/string-interning output being sensitive to the exact path string's *content* (not
   just length), not with any semantic/codegen change.
4. **Does that raw-binary difference actually matter - i.e. is the render *output* the same?** Yes,
   confirmed bit-exact: rendered `img.pdf` (the user's own real test file) with both the unpinned
   and the pinned binary via `render_bench --save-bitmaps`, and the two output PNGs hash identically
   (`sha256 a53a10ce...` both). The raw executable's debug-info bytes differ; the program's actual
   behavior does not.

| Variant | single-core (taskset -c 2), 20 iter | multi-core, 20 iter | notes |
|---|---|---|---|
| unpinned (`stable`), run 1 | 3.607 ms | 3.864 ms | `img.pdf`, `render_bench --backend hayro` |
| unpinned (`stable`), run 2 (repeat) | 5.449 ms | 3.549 ms | same binary re-run for noise floor |
| pinned (`1.98.0`), run 1 | 4.816 ms | 3.462 ms | rebuilt from clean `target/` with the pin |
| pinned (`1.98.0`), run 2 (repeat) | 3.539 ms | 3.460 ms | same pinned binary re-run |

System load during these runs was elevated (`uptime` showed load average 13-16 on a 24-core
machine, other sibling candidates' own build/bench work was concurrently queued on the same shared
`flock`), which is consistent with the wide run-to-run spread seen in both variants - both pinned
and unpinned numbers fall in the same 3.4-5.5ms band, with no consistent directional gap between
them. Given point 4 above already establishes the two binaries produce bit-identical output for
identical input, and point 2 establishes the underlying compiler is unchanged, this spread is
ordinary system noise, not attributable to the patch.

## Result

**No measurable time cost or benefit today** - pinning to the currently-installed `stable` version
changes zero bytes of program *output* (point 4) and uses the identical compiler (point 1); the
render-time numbers overlap within normal system noise, not a directional difference. The value of
this patch is entirely about **eliminating future drift**, not about today's performance: it
converts "whatever `rustup`'s `stable` alias happens to resolve to when a command runs" (which
candidate 016 confirmed already changed at least once during this workspace's history, per that
candidate's toolchain-directory-mtime evidence) into a value fixed in version control, so a future
codegen-artifact measurement (`nm`/`objdump` call counts, instruction counts, inlining/vectorization
observations - anything candidate 016's own "What's not done" section flagged) is reproducible
session-to-session on this machine without anyone having to remember to record `rustc --version` by
hand every time (which is exactly what didn't happen for candidates 013/014, per this template's
own warning).

## Does it change behavior/output?

- **This is a real fix to the tested project's own source** (adds `rust-toolchain.toml` at the repo
  root) - `patch.diff` is present, not a measurement-only investigation.
- **Bit-exact?** Program output: yes, confirmed (point 4 above, matching PNG hash for `img.pdf`).
  Raw compiled binary: no - see point 3 above; the difference is confined to debug-info path
  strings from the toolchain's own install directory name, not to any codegen/instruction content
  that would affect behavior or timing.
- **Is this a breaking change?** No public API, file format, or runtime behavior changes.
  `rust-toolchain.toml`'s only effect is which `rustc`/`cargo` binary `rustup` selects when a
  command is run inside this repo - contributors on `stable` already on `1.98.0` see zero change;
  a contributor on a different `stable` version, or whose `stable` updates later, would have their
  local toolchain silently switched to `1.98.0` for this repo (rustup auto-installs it if missing)
  instead of continuing to float - which is the entire point, but is worth stating plainly as a
  process change for real contributors, not just for this optimization workspace.
- **Project test suite** (`cargo test --release -p hayro-interpret -p hayro-tests --no-fail-fast`,
  run through the shared build lock, both before and after, single-threaded
  (`--test-threads=1`) - see "Why the result looks the way it does" below for why single-threaded
  was necessary for a fair comparison): baseline (unpinned) 430 passed / 1277 failed; patched
  (pinned) 430 passed / 1277 failed - **identical pass/fail counts, and the failing-test-name sets
  are exactly identical** (`diff` of the two sorted `FAILED` name lists produced no output). All
  1277 failures are the same pre-existing environment issue already documented by candidate 024
  (`hayro-tests` loads PDF fixtures from a `WORKSPACE_PATH`-relative corpus directory not present
  in a plain `git worktree`), unrelated to this patch.

## Why the result looks the way it does

Pinning to the *same* version already in use is, by construction, a no-op for anything that depends
on compiler behavior (codegen, optimization decisions, output bytes) - the only thing that changes
is which named directory under `~/.rustup/toolchains/` provides that compiler, which only shows up
in embedded debug-info paths, never in program behavior. That's exactly the intended outcome: this
patch is insurance against *future* drift (the next time `rustup update stable` runs, or a fresh
machine sets up a newer `stable` by default), not a fix for anything currently broken.

One real methodology finding surfaced while producing this candidate, not about the pin itself: the
project's default (multi-threaded) `cargo test` run for `hayro-tests` is **flaky** under load - an
initial default-threaded baseline run (before this candidate settled on `--test-threads=1` for its
final comparison) produced 117 passed / 1590 failed, a materially different count from the stable
430/1277 both variants converge on when run single-threaded. Re-running the *same unpinned* build
single-threaded reproduced 430/1277 directly, so this was test-harness flakiness under concurrent
load (system load average 13-16 during this session, other candidates' builds queued on the shared
lock), not something introduced by the toolchain pin - but it's a real trap for any future
candidate's correctness check on this project: **use `--test-threads=1` for `hayro-tests`'s
correctness comparisons**, or the pass/fail counts themselves become unreliable evidence,
independent of whatever patch is actually being tested. Worth promoting to `projects/hayro.md`.

## What's not done / open questions

- This patch pins to the currently-installed `stable` (`1.98.0`), not to the workspace's MSRV
  (`1.92.0`, already CI-enforced separately). Pinning to MSRV instead would be a more invasive
  choice affecting every contributor's default local toolchain version, not just this optimization
  workspace's own session-to-session reproducibility - a call for the project maintainer, not made
  here.
- This candidate demonstrates the pin is a no-op for the version currently active on this machine;
  it does not (and cannot) demonstrate that the version 013/014 originally used would have produced
  28 rather than 92 calls - that specific historical discrepancy remains characterized, not
  reproduced, per candidate 016's own conclusion.
- The multi-threaded `cargo test` flakiness noted above is a pre-existing harness issue independent
  of this candidate; not investigated further here (root cause not identified - plausibly
  file-descriptor or shared-cache contention under `--test-threads` default parallelism combined
  with this session's elevated system load) and not fixed by this patch.
