# What is under test, and how findings are confirmed

Read this first if picking the work up cold. This workspace is a variant of the standard
bug-hunt workflow: it never reads or modifies rustc's own source, never builds rustc from
source, and never attempts a fix (this repo's `AGENTS.md` bans implementation work without a
named reviewer — none was named — and the user explicitly scoped this session to discovery only).
See `../AGENTS.md`.

## The actual goal of this session (read this before anything else)

The user does **not** want re-confirmation of already-known GitHub issues treated as findings.
The deliverable is:

1. A short, bounded recon pass over the open `I-miscompile` issue list
   (`_reference/known_miscompile_issues.md`) — just "does it still reproduce" plus "what pattern
   does it exercise" — used purely as **pattern-mining input**, not as candidates. No candidate
   folder, no `SUMMARY.md` row is ever created just because a known issue still reproduces.
2. **The main deliverable**: `miscompile-fuzzer/`, a standalone, reusable, as-broad-as-possible,
   deliberately UB-free Rust-program generator + differential-testing harness, built as real code
   (not one-off ad-hoc LLM-authored test programs per round) — see
   `miscompile-fuzzer/README.md` once it exists for how it works. It generates many small,
   parameterized Rust programs from template categories informed by step 1's patterns, compiles
   each under two-or-more configurations that MUST agree for UB-free code, runs them, and diffs
   output automatically. This is meant to exist as a lasting base a future session can re-run or
   extend, not a throwaway script.
3. Any genuinely new divergence the harness finds gets triaged (re-verified, checked for
   pre-existing UB in the generated program) and — only if it survives that — gets a real
   candidate folder (`RESULT.md`, `repro/`, `output.txt`, best-effort bisection) and a
   `SUMMARY.md` row. This is the only way a row ever appears in `SUMMARY.md` this session.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `rustup` | yes | 1.29.1, network access confirmed (static.rust-lang.org reachable) |
| `cargo-bisect-rustc` | yes | installed this session via `cargo install cargo-bisect-rustc` (user-approved) — use for best-effort bisection of any confirmed new finding |
| `gh` (GitHub CLI) | yes | authenticated as `qarmin`; read-only (`gh issue view`) during the recon pass — never used to post/comment/close anything |
| building rustc from source | **not used** | out of scope — all testing uses prebuilt toolchains only |

Toolchains already cached before this session started: `stable`, `nightly`,
`nightly-2026-08-04`, `nightly-2026-08-21`, `nightly-2026-08-24`, `nightly-2026-09-11`, `1.98.0`.
Host: `x86_64-unknown-linux-gnu`, AMD Ryzen 9 7900 (Zen4), 24 threads — compiling many small
single-file programs in parallel (`xargs -P$(nproc)` / GNU parallel) is cheap here and the
harness should use that.

## Ground rules used in this workspace

1. No claim without a command that reproduces it — every `Confirmed` candidate has a real
   `repro/` and the exact command that runs it.
2. **No `patch.diff`, ever.** Every candidate's "The fix" section reads "not attempted — out of
   scope for this session (user request; also blocked by AGENTS.md's Reviewer gate, no reviewer
   named)".
3. **A re-confirmed known issue is not a finding.** Only a genuinely new divergence the
   `miscompile-fuzzer/` harness turns up, that survives re-verification and a pre-existing-UB
   review, becomes a candidate folder + `SUMMARY.md` row.
4. Every such `Confirmed` candidate gets a best-effort **Introduced by** via `cargo-bisect-rustc`
   (fallback: manual `rustup toolchain install nightly-YYYY-MM-DD` binary search), then
   `rustc +nightly-DATE -vV` for the commit hash, then `git log <hash> -1` in this checkout
   (read-only) for the subject/date.
5. Per-candidate `justfile`s use `candidate_justfile_template` (rustup-only, no clone/patch —
   nothing here ever touches the rust-lang/rust checkout, so there's nothing to clone or patch).
6. The generator/harness itself lives in `miscompile-fuzzer/` as real, documented code — treat it
   as a durable artifact to extend across sessions, not scratch; keep it working (re-runnable,
   `README.md` accurate) even if a given campaign run finds nothing.
