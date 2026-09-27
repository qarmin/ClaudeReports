# Ideas

Unchecked suspects only - see `SUMMARY.md` for what's already been investigated.

Last GitHub issue/PR checked for inspiration: **#1620** (repo: rui314/mold)

## Backlog (not yet investigated)

Consumed top-to-bottom by default when a session isn't given a specific target - see `SKILL.md`.

| Added | Suspect | Why it looks suspicious | Source |
|---|---|---|---|
| 2026-09-05 | **TSAN data-race hunt (resume note - candidate 033 was stopped mid-investigation)** | The user stopped this candidate before it wrote any `RESULT.md`/`output.txt`, so **nothing here is verified** - but it got further than any prior attempt and left two concrete artifacts. **(1) The TSAN build works on this host**: `-DMOLD_USE_TSAN=ON` with `-DCMAKE_CXX_COMPILER=g++` (gcc 15) configured and **built a working `mold` binary** - so the "never configured here" caveat in `README.md` is now resolved, and a future session can skip straight to exercising it. **(2) Two drafted, UNVERIFIED race fixes** are saved at `___BUGHUNT___/_reference/033_tsan_drafted_unverified.diff`: (a) `SharedFile<E>::resolve_symbols` (`src/input-files.cc:~1736`) reads `sym.skip_dso` *before* taking `sym.mu`, and since `skip_dso` is a one-bit bitfield it shares a memory location with flags written under that lock - a textbook bitfield data race; the draft moves the read inside the lock. (b) `Context::has_error` is a plain `bool` written by `Error`/`Warn` from TBB worker threads; the draft makes it `Atomic<bool>`. **Critically unknown: whether either came from an actual TSAN report or only from code reading** - no TSAN output survived, so a resuming session must re-derive the evidence rather than trust these. Both look independently plausible on inspection and are worth confirming properly (build TSAN, exercise a large parallel link, check whether these specific sites are flagged). | interrupted candidate 033, 2026-09-05 |
| 2026-09-05 | GitHub issue #1211 \"mold randomly segfaults\" (open since 2024-03-01, musl-based from-scratch distro) | Real, still-open upstream crash report with no reproducer attached - the reporter only provided a screenshot, and says bfd works fine on the same inputs. \"Randomly\" suggests either a data race (mold is heavily TBB-parallel) or an uninitialized/heap-layout-dependent read, both of which this project has NOT yet looked for (every confirmed bug so far is deterministic input parsing). Would need reconstructing a musl-based link environment; the payoff is that a confirmed repro here would close a real user-facing upstream issue. | GitHub issue sync, 2026-09-05 |
| 2026-09-05 | GitHub issues #1305 (sparc64) / #1306 (x32): large blocks of arch-specific ctest failures on Debian buildd | Both still open. #1305 lists ~19 failing sparc64 tests, #1306 says most x32 tests fail. These are real, reproducible-by-construction correctness failures on non-x86_64 targets that this host cannot run natively (no QEMU cross tests registered here), but some may be diagnosable by reading the failure mode in the linked buildd logs plus targeted code reading. **Note the possible connection to candidate 030** (SPARC64 large-PLT size mismatch) - if that candidate confirms, check whether any of #1305's failures share the root cause. | GitHub issue sync, 2026-09-05 |
| 2026-09-05 | src/mold.h:2199 (`MappedFile *mf = mf;` - self-initializing NSDMI) | Noticed while investigating candidate 028 - a default member initializer that initializes a member from itself. Currently dead code (no constructor path reaches it), so not a live bug, but it would become a real uninitialized-pointer read the moment a second constructor is added to that class. Worth a trivial fix (`= nullptr`) even though it isn't currently reproducible - a future session could either raise it as a code-quality/robustness patch or verify whether any newer upstream commit made it reachable. | noticed while investigating candidate 028 |

## Discovery notes (2026-09-05)

**The original `code-audit-structured` audit's high-value vein is now essentially exhausted.** A full
re-triage of all `___FINDINGS___/findings-*.toml` (301 findings) filtered to
severity critical/high + difficulty easy/medium + type panic/overflow_underflow/security/
memory_safety/incorrect_error_handling yields 28 findings, of which **27 map to candidates this
project has already investigated** (001-028) - only `src/mold.h:1701-1725` was unclaimed, and it is
now a backlog row above. What remains unreviewed in that audit is overwhelmingly
`low`/`medium`-severity `logic_error`/`performance`/`invalid_comment` material, which needs
differential testing against GNU ld rather than crash reproduction and is a different (more
expensive) kind of work.

**Implication for future sessions**: stop treating `___FINDINGS___` as the primary backlog source.
The productive sources from here are (a) fresh code-reading in never-audited areas, especially the
output/layout side (see the `passes.cc`/`output-chunks.cc` row above), (b) real upstream GitHub
issues (the sync is now watermarked at #1620 - 168 open issues, 16 open PRs cached in
`issues_raw/`), and (c) bug classes this project has never attempted at all: **data races**
(mold is heavily TBB-parallel and nothing here has run TSAN yet - `README.md` notes
`-DMOLD_USE_TSAN=ON` exists but has never been configured) and **differential correctness testing
against GNU ld** (candidates 028/031 both used ld as an oracle successfully, so the harness pattern
now exists).

**Already-fixed-upstream check (2026-09-05)**: GitHub issue #994 ("crash: assertion failed
elf/input-sections.cc:445", `assert(sym.file)` in `record_undef_error`, open since 2023 against
mold 1.10.1) no longer applies to current code - that assertion has been replaced by a real
`if (!sym.file && &sym != discarded_comdat_sym<E>)` check (`src/input-sections.cc:318`). Recorded so
a future session doesn't re-chase it from the still-open issue.
