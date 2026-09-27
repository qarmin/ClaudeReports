# read_value_ob: skip zero-init for a buffer immediately fully overwritten by read_exact

A fresh discovery this session (not from the prior session's notes): profiling `parse` mode on
`pydicom/liver.dcm` (a small, non-JPEG, monochrome 3-frame bitmap - no encapsulated pixel data)
with callgrind showed **18.28% of total instructions in `__memset_avx2_unaligned_erms`**, called
exactly once per top-level parse (matching the iteration count exactly). Reading
`parser/src/stateful/decode.rs`'s `read_value_ob` (used for `OB`-VR values, including native
`PixelData`) showed why: `let mut buf = smallvec![0u8; len]; self.from.read_exact(&mut buf)...` -
the buffer is zero-filled, then immediately, unconditionally overwritten in full by `read_exact`.
For a value with a large declared length (a native pixel data element, ~96KB for this fixture),
that initial zero-fill is pure waste - `read_exact` either fully succeeds (every byte gets
overwritten) or fails (the buffer is dropped without ever being read by safe code).

## What was tried

Added a private `uninit_smallvec<T: Copy, A: Array<Item = T>>(len: usize) -> SmallVec<A>` helper
in `parser/src/stateful/decode.rs` that allocates via `SmallVec::with_capacity` and calls
`set_len` without writing anything, skipping the zero-fill. Sound because: `T: Copy` with no
`Drop` glue means dropping an under-written `SmallVec<A>` never reads its elements, and every bit
pattern is already a valid value for `u8` (or any of the other primitive types this file uses),
so there is no way to observe or misuse an "uninitialized" byte as unsound - the only actual
safety requirement is that safe code must never read an element before it's been written, which
`read_exact`'s contract (full success or the buffer is discarded) guarantees.

**Scope note - this candidate ended up narrower than initially attempted.** The same
`vec![0; n]` + `decode_*_into` anti-pattern exists at 8 more call sites in this same file
(`read_value_ss`/`fl`/`od`/`ul`/`us`/`uv`/`sl`/`sv`, one per numeric VR) and applying the same fix
there was tried first - but that broader version was a **real, reproducible +14.2% regression**
on `liver.dcm`'s `parse` (9/9, `RUNS=9`, consistent), the opposite of the intended effect. Bisecting
by isolating just the `read_value_ob` site (this candidate's final scope) recovered a real win with
none of the regression; isolating just the 8 numeric sites alone reproduced the full +14.2%
regression on its own, confirming the numeric sites - not `read_value_ob` - were the actual cause.
**This candidate's `patch.diff` includes only the `read_value_ob` fix**, per SKILL.md's "spend a
bounded amount of time on a regression, then write it up" guidance - the numeric-site regression is
recorded as its own open lead in `IDEAS.md` rather than chased further this session.

**Verified with Miri** (`cargo +nightly miri test -p dicom-parser --lib`, `MIRIFLAGS=-Zmiri-disable-isolation`):
all 49 tests pass with no undefined-behavior report, including tests that directly exercise
`read_value_ob` (`stateful::decode::tests::decode_data_elements` and others) - strong evidence the
unsafe usage is sound, not just "the existing tests still pass."

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `parse` mode against `pydicom/liver.dcm`
(102KB, native/non-encapsulated, the exact fixture the callgrind profile was taken on) and
`color3d_jpeg_baseline.dcm` (6.14MB, JPEG-encapsulated - `read_value_ob` isn't on this file's
dominant cost path, since encapsulated pixel fragments go through the item-value reading path
instead, not `read_value_ob`).

| Variant | liver.dcm parse p50 | color3d parse p50 |
|---|---|---|
| baseline (zero-init) | 0.025/0.025 ms (2 rounds) | 1.839/1.859 ms (2 rounds) |
| patched (uninit) | 0.024/0.024 ms (2 rounds) | no consistent change |

## Result

**liver.dcm: -3.2%/-3.8% across two independent rounds, 9/9 paired wins both times, consistent.**
A real, modest win, smaller in relative terms than the raw 18.28%-of-instructions profile figure
might suggest (memset on a well-predicted, cache-resident ~96KB buffer is itself quite fast in
wall-clock terms relative to everything else `parse` does - the instruction-count share
overstates its wall-clock share, a real instance of MEASUREMENT.md's general Ir-vs-wall-clock
caveat). **color3d: no consistent change** (+1.4%/-0.4% across two rounds, both not consistent),
as expected since this file's dominant PixelData is read via the encapsulated item-value path, not
`read_value_ob`.

## Does it change behavior/output?

- **Bit-exact?** Yes. `dump` output (full `write_all` of the parsed object) is byte-identical
  between baseline and patched builds on both fixtures.
- **Is this a breaking change?** No public API/type changes - `uninit_smallvec` is a private
  function in `parser/src/stateful/decode.rs`.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures. Additionally verified with
  **Miri** (49/49 `dicom-parser` tests pass, no UB reported).

## Why the result looks the way it does

`read_value_ob` is called once per OB-VR element - for a file whose largest element is a sizeable
native pixel data buffer (liver.dcm's ~96KB, 3-frame packed 1-bit bitmap), that one call's
zero-fill is a real, non-negligible fraction of total parse work, and removing it is a clean,
narrowly-scoped win. Why the 8 numeric-array sites regressed instead of also winning (not fully
root-caused, bounded time) - the leading unconfirmed hypothesis is that those sites are each called
many times per file for small, header-sized elements (a handful of bytes/elements each), where the
generic `uninit_smallvec::<T, A>` function call (even `#[inline]`) and its internal
capacity-vs-inline-threshold branching in `SmallVec::with_capacity` may cost more per call than the
`smallvec![val; n]` macro's specialized small-N codegen saved - i.e. the fixed per-call overhead of
the more general uninitialized-allocation path may dominate over many cheap small calls, even
though it wins decisively for the one expensive large call (`read_value_ob`). This mirrors this
workspace's own candidate-006 finding that a seemingly-equivalent code change can behave very
differently depending on call frequency and per-call cost, and reinforces MEASUREMENT.md's general
warning against assuming one micro-optimization technique's win at one site generalizes to
superficially-similar sites elsewhere without re-measuring each one.

## What's not done / open questions

- The numeric-site regression's root cause was not confirmed (no disassembly comparison attempted,
  per this candidate's own bounded-time scope) - left as an open lead in `IDEAS.md` for a future
  session that wants to dig further (e.g. a hand-specialized non-generic `uninit_u16_smallvec`/
  `uninit_u32_smallvec`/etc. per type, avoiding the generic function entirely, might recover the
  win without the regression - untested).
- `object/src/meta.rs`'s `read_str_body` and several other `.resize(len, 0)` sites follow the exact
  same zero-then-fully-overwrite anti-pattern (file-meta-group UID/string reading) - not touched
  this candidate, since they're small (file-meta values are short strings) and unlikely to matter
  much, but not measured directly either; left as a low-priority open lead.
- Not benchmarked: `write-only`/`roundtrip` modes - this fix only touches the read path, so no
  effect is expected there and none was checked for regression (a gap, not a claim of "verified no
  effect").

## Follow-up: the numeric-sites regression did NOT reproduce on a later re-check - unresolved contradiction

While investigating candidates 005/016's allocator-specific regressions with the `diag-mimalloc`
harness feature (see those candidates' own "Follow-up" sections), the numeric-sites-only
comparison from this file's main section above (originally: **+14.2% regression, 0/9 paired wins,
reproduced twice**) was re-run as a side check, using the same fixture and mode
(`parse testdata/liver.dcm 2000`). **It came back the opposite way: -1.2% (a small win), 9/9 then
15/15 paired wins, fully consistent both times, under both glibc's default allocator and
`mimalloc`.** This directly contradicts the original finding in this same file.

**This is reported honestly as an unresolved contradiction, not silently reconciled either way.**
No confirmed explanation - possibilities include (in no particular order, none confirmed): a
mistake in the original bisection (e.g. a stale binary, though the original session took care to
rebuild explicitly each time); some change in this box's state between the original session and
this follow-up that happens to matter specifically for this comparison; or a genuinely flaky
effect that happened to land consistently in one direction three times, then the other direction
twice more - possible but statistically uncomfortable given the pair counts involved (0/9, 0/9,
then 9/9, 15/15). **The numeric sites remain excluded from this candidate's shipped `patch.diff`**
(only `read_value_ob` is included) - that decision is unaffected either way, since the OB-only
patch is a confirmed, real, reproducible win regardless of what the numeric sites do. Whoever
revisits the numeric-sites idea in `IDEAS.md` should treat *both* findings as suspect until
re-derived fresh, rather than trusting either this file's original section or this follow-up.

**Update (2026-08-26): a fresh, independent re-derivation was done as its own candidate, see
[`025.UNINIT_NUMERIC_ARRAY_SITES/RESULT.md`](../025.UNINIT_NUMERIC_ARRAY_SITES/RESULT.md).** Four
more rounds (including a `--features diag-mimalloc` check) all came back "no consistent change" -
a *third*, different outcome from both measurements above. The leading explanation there is that
`liver.dcm`'s ~24-microsecond total `parse` time is simply too fast for this workspace's
process-spawn-based paired timing to resolve reliably at all, for any of the three attempts -
i.e. the contradiction may be an artifact of measuring something below this technique's noise
floor, not evidence of a real effect that keeps flipping direction. Still not resolved which
direction (if any) is real; see that candidate's "What's not done" for what a future attempt would
need (a bigger/more numeric-VR-dense fixture, or a lower-noise measurement technique).
