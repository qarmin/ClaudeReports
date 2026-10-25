# Bug-hunt results — rustc miscompilation discovery via an automated differential fuzzer

Scope note for this workspace: **every row here is a genuinely NEW miscompilation found by the
differential-fuzzer generator built this session** (`miscompile-fuzzer/`), not a re-confirmation
of an already-known GitHub issue. The user was explicit: the 33 open `I-miscompile` issues
checked for pattern-mining purposes (`_reference/known_miscompile_issues.md`) are informational
only and never get a row here just for still reproducing — only something new does. All testing
uses **prebuilt toolchains fetched via `rustup`/`cargo-bisect-rustc` only** on the host
architecture (`x86_64-unknown-linux-gnu`, AMD Zen4) — the rust-lang/rust checkout itself is never
built from source and never modified. No fixes are attempted in this workspace: out of scope per
the user's request, and AGENTS.md's Reviewer gate (no named reviewer for this session) would
block implementation work regardless. See `README.md` for the full methodology.

**Status**: **Confirmed** (still reproduces on current nightly, a real run was observed) —
**Not reproducible** (checked, no longer reproduces, or never did on this target — see the
candidate's `RESULT.md`) — **Inconclusive** (signal seen but short of `CONFIRMATION.md`'s bar).
**Class**: always `miscompile` in this workspace (occasionally `crash` if the issue is actually
a compiler crash mislabeled, noted when it happens) — purely descriptive, not a priority
ranking. **Fully reproducible** (only when Confirmed): **Yes** if a real `rustc`/`cargo` run on
this host reproduces the wrong behavior end-to-end. **Introduced by** (only when Confirmed,
best-effort): the nightly date and/or commit hash found via bisection (`cargo-bisect-rustc` or
manual `rustup toolchain install nightly-YYYY-MM-DD`), cross-referenced against the local git
history for a commit subject — or `not determined` with the reason. **Fixed by**: blank unless
a later session finds it's since been fixed upstream (the GitHub issue being open is a strong
prior that it hasn't, but nightly can move faster than issue triage).

Rows are kept sorted by the **#** column (ascending) — run `just tidy` after appending.

| # | Status | Class | Fully reproducible | Candidate | Root cause (short) | Introduced by | Fixed by | Link |
|---|---|---|---|---|---|---|---|---|
