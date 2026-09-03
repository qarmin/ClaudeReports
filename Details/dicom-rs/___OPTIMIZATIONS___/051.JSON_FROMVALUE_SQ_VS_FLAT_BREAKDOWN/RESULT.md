# `dicom_json::from_value`'s own SQ-recursion-vs-flat-element split, vs. `from_str`'s (candidate 036)

Candidate 037 measured candidate 026's `RawValue`+`from_str` fix through the `from_value` entry
point (a `serde_json::Value` in, not raw text) on `seq_10k.dcm` alone, and found a real but
smaller/noisier win than `from_str`'s own -59 to -66%-class number on the same fixture - leaving
open whether `from_value`'s win, like `from_str`'s (candidate 036), is dominated by the `SQ`
recursion path rather than the flat multi-element path. This candidate reuses candidate 036's own
`gen-flat`/`gen-seq`-built fixtures (`flat_20k.dcm`, `seq_10k.dcm` - both 20,000 total elements,
differing only in whether they sit inside one recursive `SQ` or flat at the top level) run through
the `json-decode-fromvalue` harness mode (candidate 037) instead of `json-decode`, to isolate the
same two paths for the `from_value` entry point.

## What was tried

No new code - candidate 026's `patch.diff` re-applied unchanged to `json/Cargo.toml` and
`json/src/de/mod.rs`; two binaries (baseline, patched) compared via `tools/paired.sh` (`RUNS=9`,
15 iterations/run) on the `json-decode-fromvalue <file> <iters>` harness mode, on both fixtures,
both single-core (`taskset -c 0`) and multi-core (unpinned):

| Fixture | mode | round 1 | round 2 (re-check) |
|---|---|---|---|
| `seq_10k.dcm` (10,000 SQ items x 2 elements = 20,000 total) | multi-core | -16.6%, 9/9 | -16.4%, 8/9 |
| `seq_10k.dcm` | single-core | -16.1%, 7/9 (not consistent - see note) | -18.3%, 9/9 |
| `flat_20k.dcm` (20,000 flat elements, no SQ) | multi-core | +50.1%, 0/9 | +49.7%, 0/9 |
| `flat_20k.dcm` | single-core | +51.2%, 0/9 | +44.3%, 0/9 |

The single-core round-1 `seq_10k.dcm` result (7/9, "NOT consistent") happened during a period of
very high system load (`uptime` showed `load average: 36.88` against 24 cores, from sibling
candidates running concurrently on this box); round 2, re-run once load dropped to ~9, was clean
9/9 and consistent with the multi-core numbers. The `flat_20k.dcm` regression, by contrast, was
9/9-consistent (in the regression direction, i.e. "B" - patched - slower every single pair) across
all 4 rounds regardless of load, single-core or multi-core - not a noise artifact.

## Result

**`from_value`'s SQ-vs-flat split does NOT match `from_str`'s own split (candidate 036) - it goes
in the opposite direction for the flat path.** `from_str` showed a real win on both paths (a
smaller -12/-14/-18%-class win on flat, a larger -58/-66%-class win on SQ). `from_value` instead
shows:

- **SQ-recursion path: a real, consistent win, `-16 to -18%`** (multi-core and single-core, once
  system noise is controlled for) - smaller than `from_str`'s `-58 to -66%` on the same fixture,
  consistent with candidate 037's own finding that `from_value` benefits less from this fix
  overall.
- **Flat-element path: a real, consistent REGRESSION, `+44 to +51%`** (0/9 in the patch's favor,
  every round, both single- and multi-core) - the opposite sign from `from_str`'s own flat-path
  result on the identical fixture.

Time only was measured (no peak-RSS follow-up in this candidate).

## Does it change behavior/output?

- **Bit-exact?** Yes - checksums `0xd583b32455a46900` (`seq_10k.dcm`) and `0xfdf3ca526d3d7843`
  (`flat_20k.dcm`) are identical baseline vs. patched, and match candidates 036/037's own checksums
  on the same fixtures exactly.
- **Is this a breaking change?** No - same private internal `json` crate deserializer change as
  candidates 026/036/037, unaffected by this candidate; `from_value`'s public signature is
  untouched.
- Project test suite: `cargo test --release -p dicom-json`: **12 + 5 = 17 passed / 0 failed**, both
  baseline and patched (identical counts), matching candidates 026/036/037's own recorded counts.
  (Note on how this was obtained: an initial attempt to temporarily revert the patch for the
  baseline test run via `git stash`/`git stash pop` inside this candidate's own worktree
  accidentally swapped stashed changes with a concurrently-running sibling candidate's worktree -
  `git stash` uses a stash ref stored in the shared `.git` directory, which is **not** isolated
  per-worktree the way the working tree itself is. This was caught immediately via `git diff
  --stat` showing an unexpected `encoding/src/text.rs` change instead of this candidate's own
  `json/` change; the tree was restored with `git checkout --` + a fresh `git apply` of this
  candidate's own known `patch.diff` rather than trusting the stash, and both the baseline and
  patched test runs above were re-executed cleanly this way. A message was sent to the orchestrating
  session flagging that whichever sibling candidate was using `encoding/src/text.rs` around the
  same time should double-check its own worktree's diff actually matches what it intended to test.
  No `git stash` was used for anything reported in this candidate's own numbers above - all
  measurements were taken with the tree in a known, directly-verified state at the time of
  measurement.)

## Why the result looks the way it does

Candidate 037's own "why" section hypothesized that `serde_json::Value`'s `Deserializer` impl,
when asked for a `RawValue`, must internally re-serialize the relevant sub-`Value` back into text
(since a `Value` never had raw JSON text to begin with) - a real cost `from_str` never pays. This
candidate's flat-vs-SQ split makes that cost's shape concrete: a flat element pays this
serialize-then-reparse round trip once per element with nothing to amortize it against, and for
this fixture's many small, cheap-to-convert-directly values (short UI strings), that overhead
apparently exceeds the double-pass (`Value` tree + `from_value`) cost the original patch removes -
a net loss. The `SQ` path's per-item cost is dominated by the recursive re-entry into
`DataElementVisitor`/`visit_map` for each nested object (the same compounding effect candidate 036
identified for `from_str`), which is large enough that even with the same per-element
serialize-then-reparse tax paid on the way in, the net effect is still a win, just a smaller one
than `from_str` sees on the identical fixture (where there's no such tax to begin with).

## What's not done / open questions

- This closes the open question left by candidates 036/037's own "what's not done" sections (does
  `from_value`'s split match `from_str`'s) - the answer is "no, and the flat path actually
  regresses," itself worth flagging as a reason not to assume `from_value` uniformly benefits from
  candidate 026's fix the way `from_str`/`from_reader` do, if `from_value` is ever used on
  flat-element-heavy documents (e.g. a document with many top-level elements and little/no nested
  `SQ` content).
- The exact re-serialization mechanism inside `serde_json`'s `Value`-as-`Deserializer` raw-value
  support (candidate 037's hypothesis for *why* this happens) was still not traced in `serde_json`'s
  own source in this candidate either - the flat-path regression is consistent with that hypothesis
  but doesn't independently confirm the mechanism.
- Not cross-checked against `rtplan.dcm` or `liver.dcm` (real-world mixed fixtures) through
  `json-decode-fromvalue` - only the two synthetic, path-isolating fixtures were used here,
  matching candidate 036's own scope for the equivalent `from_str` question.
