# resync_github_issues_prs_past_watermark_1682

Meta-candidate, not a single suspect code path: re-run the "GitHub issue/PR inspiration" discovery
step (`SKILL.md`) to catch up this workspace's own open-issues/open-PRs backlog past its last
recorded watermark, then triage every newly-visible item for a bug-shaped lead not yet covered by
an existing `SUMMARY.md` row. Recommended ahead of more cold code reading because, per this
project's `projects/<name>.md` notes, the open-issue/open-PR backlog was the highest-yield source
in the previous session (7 of that session's confirmed bugs traced back to it).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Checked `IDEAS.md`'s watermark line | `Last GitHub issue/PR checked for inspiration: #1693 (repo: bytecodealliance/rustix)` | Already past this candidate's nominal starting point (#1682) - `__issues_raw/issues/` and `__issues_raw/pulls/` already contained every open issue/PR numbered 1683-1693, fetched earlier the same day (file mtimes ~11:59-12:33 local, before this candidate started), i.e. the resync itself had already run as part of this session's/batch's own setup |
| Re-ran the resync script to confirm nothing newer exists | `python3 ~/.claude/skills/gh-issues-prs-check/fetch_issues.py --dir ___BUGHUNT___/__issues_raw raw-sync --since 1693` (gh CLI authenticated as account `qarmin`) | `Raw-synced 0 issue(s), 0 PR(s) numbered > 1693` - `MAX_NUMBER_SEEN=1693`, confirming the cache is current as of this run; `IDEAS.md`'s watermark line was left unchanged since nothing new was found (own value already matched) |
| Read every open issue/PR numbered 1683-1693 (6 issues: #1683, #1685, #1686, #1689, #1691, #1693; 4 PRs: #1684, #1687, #1688, #1692) and judged each against "is this a still-live crash/correctness/UB-shaped bug, and does an existing `SUMMARY.md` candidate already cover it" | Direct reading of the cached JSON bodies in `__issues_raw/` | See breakdown below - every bug-shaped item in this range duplicates a candidate already `Confirmed` in this same workspace; every other item is out of scope for this skill (doc-lint warning, feature request) |

### Per-item disposition

| # | Kind | Title | Disposition |
|---|---|---|---|
| #1683 | issue | recvmsg: truncated SCM_RIGHTS control data (MSG_CTRUNC) panics in AncillaryDrain and again in Drop on macOS | Duplicate of already-`Confirmed` candidate 027 (`ancillary_drain_cmsg_len_underflow`) - same mechanism (`cmsg_len` trusted past the buffer's real remaining space), same macOS-only manifestation already noted in 027's own `RESULT.md`. Not a new finding. |
| #1684 | PR (open) | fix(net): don't trust `cmsg_len` past the end of a truncated control buffer | Fixes #1683 - same bug as candidate 027; its description additionally states that with overflow checks off, `cvt_msg` slices past the buffer and yields `OwnedFd`s read from uninitialized memory (a detail not explicitly spelled out in 027's own write-up, but the same root cause). Not a new finding, no action taken on 027's own files per this candidate's scope. |
| #1685 | issue | Docs build with warnings (`RUSTDOCFLAGS="-D warnings" cargo doc`) | Compile-time `rustdoc::broken_intra_doc_links` lint failures (several stale doc links: `crate::process::prlimit`, `OFlags::PATH`, `DupFlags::CLOEXEC`, `mlock_with`, `RecvFlags::TRUNC`) - a documentation/lint gap, not a crash/correctness/UB-shaped bug; out of this skill's scope (see `HEURISTICS.md`'s anti-patterns list: "style-only... warnings with no correctness implication"). Skipped, not added to `IDEAS.md`. |
| #1686 | issue | Add rlimit support for Redox | Feature request (no `rlimit` support exists yet on that target), not a bug. Skipped. |
| #1687 | PR (open) | feat(redox): `getrlimit` and `setrlimit` | Implements the #1686 feature request. Not a bug. Skipped. |
| #1688 | PR (open) | fix(fs): Enable sendfile on Android | Duplicate of already-`Confirmed` candidate 025 (`sendfile_android_linux_kernel_gate`) - identical description (`target_os = "linux"` instead of the `linux_kernel` cfg alias). Not a new finding. |
| #1689 | issue | net feature alone fails to compile in 1.1.5: sockopt uses `crate::timespec`, which `net` does not gate in | Duplicate of already-`Confirmed` candidate 024 (`net_feature_timespec_gating_regression`) - identical root cause and identical `E0433` compiler output. Not a new finding. |
| #1691 | issue | Support `mknodat`/`mkfifoat` on Apple targets | Feature request (no body beyond the title), not a bug. Skipped. |
| #1692 | PR (open) | fix(net): use IPPROTO_IPV6 for IPV6_MULTICAST_HOPS | Duplicate of already-`Confirmed` candidate 003 (`ipv6_multicast_hops_wrong_protocol_level`) - identical root cause (`IPPROTO_IP` used instead of `IPPROTO_IPV6`), and its description adds useful provenance detail (traces the regression to upstream commit 77c4aaea moving the code and obscuring `git blame`, matching candidate 003's own "Introduced by"). Not a new finding. |
| #1693 | issue | xattr size queries panic with `&mut [MaybeUninit<u8>]` buffers | Duplicate of already-`Confirmed` candidate 010 (`getxattr_zero_len_buffer_bogus_length`) - identical trigger (0-length size-query buffer) and identical panic site/message (`src/buffer.rs:176:35: mid > len`). Not a new finding. |

## Status

**Not reproducible** (as a *new* candidate - every bug-shaped item in this backlog range is a
restatement of a finding this workspace already has a `Confirmed` row for; nothing here warranted
opening a new numbered investigation). Coverage: every open issue and open PR numbered 1683-1693
inclusive was read in full (10 items total); the resync itself was independently re-verified
current (0 new items past #1693 as of this run). This is a complete review of the stated range, not
a sample.

## Class

n/a (no new `Confirmed` finding from this candidate).

## Root cause

n/a - this candidate found no new suspect to root-cause. For the record, the mapping from each
bug-shaped item back to its pre-existing candidate's own root cause is the table above; see each
referenced candidate's own `RESULT.md` for the underlying mechanism.

## Introduced by

n/a (no new finding).

## The fix

n/a - no new bug to fix. Five of this workspace's own previously-`Confirmed` candidates (003, 010,
024, 025, 027) now have independent, real-world corroboration: each is either the subject of a
still-open upstream bug report, or already has an open (unmerged, as of this run) fix PR attempting
the same repair this workspace already verified independently. None of those candidates' own
`RESULT.md`/`SUMMARY.md` rows were edited by this candidate (out of scope - see "Fixed by" only
applies once a fix actually lands, and these upstream PRs are still open/unmerged).

- Project test suite: N/A, no code change was made by this candidate.

## What's not done / open questions

- This candidate deliberately does not touch `SUMMARY.md`/`SUMMARY-SHORT.md`/`IDEAS.md` (per this
  session's own batch instructions, those are transcribed later as a separate step).
- If/when PRs #1684, #1688, #1692 are merged upstream, candidates 027, 025, 003 should each get
  their `SUMMARY.md` "Fixed by" cell filled in and a `CORRECTION.md` added, per `SKILL.md`'s "When a
  later candidate invalidates an earlier one's confirmation" - not done here since none of the three
  PRs were merged as of this run (all `state: open`).
- #1684's description of a release-mode (`overflow-checks = false`) consequence of the same bug
  candidate 027 found (`cvt_msg` slicing past the buffer and yielding `OwnedFd`s backed by
  uninitialized memory, rather than only the debug-mode subtraction panic) is worth folding into
  candidate 027's own write-up in a future session, if a stronger memory-safety demonstration is
  wanted there - not done here since it would mean editing another candidate's already-written
  record, out of this candidate's own scope.
- #1685 (doc-lint warnings) was intentionally left out of `IDEAS.md` as out-of-scope for this skill;
  it would be an easy, uncontroversial fix for whoever does triage non-bug maintenance items.
- No new leads were added to `IDEAS.md`'s backlog by this candidate, since none of the 10 reviewed
  items qualified as a new, not-yet-covered bug-shaped lead.
