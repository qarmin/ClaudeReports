# PNG DEFLATE compression level: `Compression::Fastest` instead of `Compression::Fast`

Follow-up to [`../002.png_compression_level/RESULT.md`](../002.png_compression_level/RESULT.md),
ported from `IDEAS.md`'s backlog. That candidate already measured `Fastest` "for comparison, not
applied" (isolated 1.187ms vs. `Fast`'s 1.554ms; end-to-end 3.92ms vs. `Fast`'s 4.32ms, both vs. a
13.708/8.40ms `Balanced`/current-default baseline). This candidate re-measures `Fast` vs. `Fastest`
fresh this session (own baseline pair, `taskset` + unpinned) and formally applies the one-line
change to real source, builds it for real, and re-runs the existing bench's decode-and-compare
correctness check against the actually-patched code path.

`Pixmap::into_png` currently calls no `set_compression(...)` at all (candidate 002's `Fast` fix
was measured and written up but, per this skill's rule, never left applied in the working tree) -
so the real-source comparison here is against that same unmodified default (`Balanced`), with
`Fast` and `Fastest` both reported for the actual three-way comparison requested by the idea.

## What was tried

Reused [`../002.png_compression_level/bench/`](../002.png_compression_level/bench/) unmodified (a
standalone crate, path-dependency on `vello_common`, real rendered `img.pdf` page as input) -
copied into this folder's own `bench/` per the "self-contained candidate folder" rule, no source
changes needed to the bench itself since it already runs all 5 `Compression` variants including
`Fastest` and decode-compares every one of them against the `Balanced` baseline every run.

`taskset -c 2`, 60 iterations, 2 repeated runs (noise-floor check), then 2 more unpinned runs
(realistic wall-clock; PNG encoding here is single-threaded, so "multi-core" here means "unpinned,
system free to schedule/preempt" rather than parallel work):

| Variant | single-core p50 (run 1 / run 2) | unpinned p50 (run 1 / run 2) | notes |
|---|---|---|---|
| `Balanced` (current default) | 13.928 / 13.145 ms | 13.507 / 13.341 ms | baseline |
| `Fast` | 1.961 / 1.930 ms | 1.993 / 1.960 ms | previously written up in candidate 002, never left applied |
| `Fastest` (this candidate) | 1.538 / 1.539 ms | 1.581 / 1.558 ms | this patch |
| `NoCompression` | 1.203 / 1.204 ms | 1.217 / 1.203 ms | for reference, not this candidate |
| `High` | 39.486 / 39.462 ms | 39.765 / 39.563 ms | for reference, not this candidate |

Then applied the real one-line patch to `sparse_strips/vello_common/src/pixmap.rs`
(`encoder.set_compression(png::Compression::Fastest);` in `into_png`), built it for real
(`cargo build -p vello_common --release --features png`, succeeds cleanly) to confirm the change
compiles in the actual crate context and not just the bench's local reimplementation of the same
4-line encode body - then reverted per this skill's rule (patch never left applied).

## Result

Averaging the 4 `Fast` p50s (1.961, 1.930, 1.993, 1.960 -> 1.961 ms) against the 4 `Fastest` p50s
(1.538, 1.539, 1.581, 1.558 -> 1.554 ms): **`Fastest` is ~20.7% faster than `Fast`** for the
isolated PNG-encode step, consistent across both pinned and unpinned runs (no meaningful
difference between the two, as expected for a single-threaded, CPU-bound loop with no lock/
memory-bandwidth contention). Against the current unmodified default (`Balanced`, ~13.5ms average
p50), `Fastest` is ~88.5% faster.

Candidate 002's own previously-measured full render+encode end-to-end numbers (`img.pdf`,
`taskset -c 2`): `Balanced` 8.40ms -> `Fast` 4.32ms -> `Fastest` 3.92ms, i.e. `Fastest` saves an
additional ~9.3% of `Fast`'s already-reduced total (matching the ~20% isolated encode-only delta
measured here, diluted by the render phases that don't change). That end-to-end script was a
temporary instrumentation pass in a previous session and wasn't preserved standalone, so it is
cited here rather than re-run - the isolated bench above is the number actually reproduced fresh
this session.

## Does it change behavior/output?

- **Bit-exact?** yes for decoded pixels. DEFLATE is lossless at every compression level; only the
  encoded byte size changes. Verified by the bench's own decode-and-compare, run fresh this
  session (4 times, once per timing run above):
  ```
  Fast          : decoded pixels identical to Balanced = true
  Fastest       : decoded pixels identical to Balanced = true
  NoCompression : decoded pixels identical to Balanced = true
  High          : decoded pixels identical to Balanced = true
  ```
- **Is this a breaking change?** no pixel-level change. Output PNG file size increases from
  163,294 bytes (`Fast`) to 210,363 bytes (`Fastest`) for this 595x841 test page - **+28.8% larger
  than `Fast`**, or **+81.7% larger than the current `Balanced` default**.
- Project test suite: not re-run against hayro's own 268-test isolation set for this candidate
  (unlike candidate 003 in hayro's own workspace, which wired vello's pixel-changing
  `take_unpremultiplied` fix in for that reason) - a lossless compression-level change cannot
  affect any exact-pixel-diff test's outcome by construction, and this was already empirically
  confirmed for `Fastest` specifically in candidate 002's own 268/268-pass run.

## Why the result looks the way it does

Same mechanism as candidate 002: `fdeflate`'s `Fastest`-equivalent settings do less match-finding
work than its `Fast` settings, trading more of the (already-poor, vs. `Balanced`/`High`)
compression ratio for a further constant-factor speedup. The two runs (pinned/unpinned) agreeing
this closely confirms the operation is neither memory-bandwidth- nor scheduler-sensitive at this
buffer size (595x841 RGBA8, ~2MB).

## What's not done / open questions

- Not wired into hayro (no `[patch]` section added to hayro's `Cargo.toml`) - hayro's actual
  downstream pipeline encodes JPEG via a separate crate, not through this PNG path (see hayro's
  own `___OPTIMIZATIONS___/README.md` scoping note), so an end-to-end hayro-side measurement would
  not reflect the user's real workload the way it does for stages that are actually on that path.
- Did not re-run the full render+encode end-to-end timing fresh this session (see "Result" above)
  - only cited candidate 002's prior measurement of it. Anyone wanting a session-fresh end-to-end
    number would need to rebuild that temporary instrumentation (not preserved in this workspace).
- Did not implement the idea's alternative framing ("expose the compression level as a
  configurable knob") - this candidate only tested the fixed-default swap, per "one fix per
  candidate folder."
- The exact pinned commit hayro's `Cargo.toml` specifies (`dc23e93841d20060743e490ef49c1e0d0d22cf40`)
  could not be checked out in this local clone (no `git-lfs` available - see `../README.md`); this
  clone's HEAD (`af32d531`) is 2 commits ahead, same confound already documented for candidates
  001/002/hayro's own 003.
