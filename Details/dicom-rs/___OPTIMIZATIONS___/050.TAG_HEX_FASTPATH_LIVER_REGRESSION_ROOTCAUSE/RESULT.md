# Root-causing candidate 027's liver.dcm regression (hex-table Tag serialization)

Follow-up to `027.JSON_TAG_HEX_FASTPATH` (mandatory reading first): that candidate found a
`format!`-to-hand-written-hex-table rewrite of `DicomJson<Tag>::serialize` gave large, consistent
wins on element-heavy fixtures (`seq_10k.dcm` -25.4%/-26.1%, `rtplan.dcm` -28.2%) but a small,
fully-consistent *regression* on `liver.dcm` (+4.8%/+5.4%, 2 rounds, 0/9 both) - a file dominated
by one large `PixelData` value and only ~7 top-level tags, where the tag-hex change should be
negligible either way. 027's own follow-up ruled out the simple "patched binary got bigger"
version of the code-layout hypothesis (total binary size and `pad_integral` instantiation count
both near-identical) and left a full `objdump -d -C` comparison as the prescribed next step -
this candidate does that comparison.

## What was tried

Reused `___OPTIMIZATIONS___/harness/bench_pipeline`'s `json-encode` mode against `liver.dcm`, in
a fresh git worktree at the same `base_commit` 027 used
(`df2b9e4236298efa3fa37a1f360fc4bf284d586a`) with 027's own `patch.diff` re-applied unmodified (no
new source change this candidate). Built baseline and patched release binaries, then:

1. Re-ran the paired timing comparison 3 independent times (different core pinning / order each
   time) to see if the regression reproduces at all in a fresh session.
2. Confirmed the build is fully deterministic on this machine (`cargo clean` + full rebuild of
   the unpatched tree reproduces a byte-identical binary, md5 match) - ruling out "the binary
   candidate 027 actually measured differs from what a clean checkout produces today" before
   trusting any disassembly comparison.
3. `objdump -d -C` on both binaries, located the actually-executed hot symbols via `nm -S`
   (`collect_map::<...DicomJson<Tag>...>` and its callee `serialize_entry::<...DicomJson<Tag>...>`
   - the two functions on the per-element JSON-key-serialization path) and diffed their sizes and
   disassembly (`asm_out/` in this folder).

| Variant | single-core p50 (taskset -c 3/5) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline (unpatched) | 0.139 ms (round 1, A) | 0.139 ms (round 2) | 3 rounds, all consistent with each other |
| patched (027's patch) | 0.125 ms, -9.8%, 9/9 (round 1) | 0.125 ms, -9.9%, 9/9 (round 2) | round 3 (order swapped, A=patched/B=baseline): median 0.125->0.139, +11.4%, "B(baseline) faster in 0 of 9" - i.e. patched still faster in all 9 pairs |

All 3 rounds agree: patched is **faster**, not slower, on this exact `liver.dcm` fixture, this
exact `base_commit`, and this exact unmodified `patch.diff` - the opposite direction from
candidate 027's own documented +4.8%/+5.4% regression.

## Result

**The regression does not reproduce.** Three independent paired-comparison rounds this session
(9/9, 9/9, and 0/9-with-swapped-A/B-meaning-the-same-direction - 27 total pairs) all show the
patched build ~9.8-11.4% *faster* than baseline on `liver.dcm`'s `json-encode` mode, consistent in
both single-core (`taskset`) and unpinned multi-core measurement. This is a real, measured
disagreement with 027's own two rounds (also each internally consistent, 0/9 both, also paired via
the same `tools/paired.sh` discipline) on the identical source diff and identical base commit -
not a case of "the effect is small and noisy," but two sessions each getting a fully consistent
paired-run signal pointing in opposite directions.

Confirmed *not* explained by:
- **A different binary.** `cargo clean` + full rebuild of the unpatched tree reproduces the exact
  same md5 as the binary used for these timings, both before and after this candidate's own
  `cargo clean` - the build is deterministic on this machine, so whatever binary 027 measured
  (same source, same commit, same harness `Cargo.toml`) would objdump identically to what's
  compared here.
- **The patch being mis-applied.** `git apply --check` succeeded, `md5sum` confirms baseline and
  patched binaries differ, and both produce the bit-exact same JSON checksum as each other and as
  027's own reported values.

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum=0xa3cf17d7e8cc5c7a` identical between baseline and patched builds
  on `liver.dcm` (also matches 027's own reported bit-exactness).
- **Is this a breaking change?** No - same as 027 (this candidate made no new source change,
  `patch.diff` here is a byte-for-byte copy of 027's own).
- Project test suite: `cargo test --release -p dicom-json -p dicom-dump`: **25 passed / 0 failed**
  for both baseline and patched builds (4+1+12+3+5, matching 027's own reported count), no new
  failures either way.

## Why the result looks the way it does

The prescribed `objdump -d -C` comparison **did** find a real, concrete code-layout difference,
just not one that explains a slowdown - if anything it points the other way:

- `DicomJson<Tag>::serialize` (the function 027's patch rewrites) exists as a **separate,
  out-of-line function** in the baseline binary (`nm -C` finds it as its own symbol, called via a
  single `call` instruction from `serialize_entry`). In the **patched** binary, no standalone
  symbol for it survives at all - LLVM's inliner folded the entire hex-nibble-extraction-and-write
  logic directly into its one caller, `serialize_entry`.
- This makes `serialize_entry::<...DicomJson<Tag>...>` itself grow from **0xca (202) bytes in
  baseline to 0x16e (366) bytes in patched** (+81%, see `asm_out/base_serialize_entry.asm` vs.
  `asm_out/patch_serialize_entry.asm`) - baseline's version is 5 instructions (`mov`, `mov`,
  `call`, `test`, `je`) plus whatever happens inside the separate call; patched's version has the
  8-nibble table-lookup sequence (~30 `shr`/`and`/`movzx`/`shl`/`or` instructions reading a static
  16-byte lookup table) inlined directly at the call site, then falls through into the existing
  `format_escaped_str` call for the JSON string-quoting.
- Its caller, `collect_map::<...DicomJson<Tag>...>`, stays **exactly the same size in both**
  (0x338/824 bytes) - it always reaches the tag-serialization logic via one `call` to
  `serialize_entry` either way, so this particular caller's own code doesn't grow; the size
  increase is contained entirely inside `serialize_entry`.

This is a genuine, verifiable instance of candidate 027's own "inlining changes what LLVM places
where" hypothesis - the patch does measurably change the shape of the compiled hot path, not just
its total byte count. But it does not, on its own, explain a *slowdown*: for `liver.dcm`'s ~7
top-level tags, trading one out-of-line `call` (whose callee itself calls further out into
`core::fmt`'s `Formatter`/`pad_integral` machinery, i.e. is *more* instructions overall, just
spread across more functions) for ~30 inlined arithmetic instructions plus the same
`format_escaped_str` call should be a wash-to-win at this call count, matching what this session's
own fresh timing actually shows (a ~10% win, smaller than the element-heavy fixtures' 25-28% but
still a win, not a loss).

**Leading conclusion:** candidate 027's own `liver.dcm` regression appears to have been a
measurement artifact specific to that session's run - a paired-run discipline that happened to
land on a consistent 0/9 both times it was checked, but did not hold up on a third and fourth
independent re-check in a later session, on the identical patch and commit. `MEASUREMENT.md`'s
noise-floor guidance treats N/9-or-1/9 as decisive under the assumption that whatever's driving a
consistent split (a real code effect, or some environmental state) stays constant for the
alternating-run window it was measured in; this case suggests that assumption can still fail
across *separate sessions* (different underlying machine/thermal/scheduler state at the time),
even when it held within any one session's own alternating window - a caveat not previously
documented for this project's own paired-comparison tooling.

## What's not done / open questions

- **Not identified**: what specific environmental factor differed between candidate 027's
  original session and this one (CPU governor state, thermal history, background load pattern,
  ASLR base address interacting with an unrelated page-alignment effect) that could flip a
  consistent 0/9 signal to a consistent 9/9 signal in the opposite direction. `uptime` load average
  was 1.8-1.9 during 027's own runs and 2.4-8.7 during this candidate's runs (see the three
  paired-run invocations above) - if anything, this session had *more* background load, yet still
  measured the patch as faster, which argues against "027's regression was real and this session's
  measurement is the one that's wrong" as an explanation, but doesn't identify the true mechanism.
- **Not attempted**: re-running today with the process pinned to the exact same core 027 used, or
  probing CPU frequency-scaling/turbo state directly (`cpupower frequency-info` or similar) during
  both sessions' windows - would help narrow whether this is thermal/governor-related.
- Given the regression does not currently reproduce, no further code change is proposed here -
  this candidate is a measurement/investigation only, and `SUMMARY.md`'s row for 027 should be
  read alongside this candidate's finding rather than acted on as if the liver.dcm regression is
  still an open, currently-reproducible cost of that patch.
