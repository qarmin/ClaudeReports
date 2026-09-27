# core-length-arithmetic-overflow-to-undefined

`core/src/header.rs`'s `Length` type reserves the bit pattern `0xFFFF_FFFF` as `Length::UNDEFINED`
- a sentinel meaning "no defined length" (used pervasively for `SQ` elements and encapsulated
pixel data), semantically distinct from any real byte count. Four of `Length`'s arithmetic
operator implementations (`Add<Length>`, `Add<i32>`, `SubAssign<Length>`, `SubAssign<i32>`)
compute their result using plain, wrapping `u32`/`i32` arithmetic and only *assert* (via
`debug_assert!`) that the result doesn't happen to collide with `0xFFFF_FFFF` - they never
actually prevent it. A defined-length computation whose true result is exactly `0xFFFF_FFFF` (or
wraps around to it, e.g. via underflow) silently becomes indistinguishable from
`Length::UNDEFINED` in release builds, or aborts the whole process via an uncatchable panic in
debug/test builds.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology: `git log --all --grep="#768"` (following this session's established "recent fix commits reveal sibling bugs" technique) | Searched all refs, not just the current branch, for commit messages referencing an external bug-report issue number already seen fixing sibling issues (`OVF_6`, `LOOP_1`) elsewhere in this session's investigation | Found `dd6e81aa` ("fix(core): prevent Length arithmetic from silently wrapping to UNDEFINED sentinel", referencing `OVF_3`/`OVF_4`/`OVF_5`) on a **separate, unmerged remote branch** (`origin/fix/length-arithmetic-overflow`) - confirmed via `git merge-base --is-ancestor dd6e81aa df2b9e42` (returned false) that this fix was never merged into the commit this bug-hunt session is auditing |
| Independent reproduction of the defect described in that unmerged commit, from scratch (no dependency on the unmerged branch itself - just the technique it named) | Minimal, hand-built `dicom-core` path-dep binary calling `Length(0xFFFF_FFFE) + Length(1)` and `Length(0) -= Length(1)` directly | Confirmed: in debug builds, both calls panic immediately via the pre-existing `debug_assert!`; in release builds, both silently produce `Length::UNDEFINED` instead of the correct result |
| Auditing every other arithmetic operator on `Length` for the same pattern, since the external fix only mentioned 3 specific call sites | Read all `impl std::ops::* for Length` blocks in `core/src/header.rs` | Found the external fix's coverage was **incomplete**: `Add<i32>` and `SubAssign<Length>` have the exact same unguarded `debug_assert!`-only pattern and were not touched by `dd6e81aa` |
| Reproduction of the two additional operators found | Same technique: `Length(0xFFFF_FFFE) + 1i32` and `Length(0) -= 1i32` (this second one *was* covered by the external fix, included here as a completeness check) | Confirmed both `Add<i32>` and `SubAssign<i32>` exhibit the same debug-panic/release-corruption behavior. Reproduced deterministically: in-tree checks during investigation + 2 independent `just test` runs against a fresh clone of pristine upstream `df2b9e42`, plus a separately captured debug-mode panic (see `backtrace.txt`) |

## Status

**Confirmed**, with a bounded fix applied and verified. Deterministic - reproduces on every run,
for any code path that adds or subtracts `Length`/`i32` values whose true arithmetic result would
be exactly `0xFFFF_FFFF` (overflow) or would need to go below `0` (underflow, which also wraps
around to `0xFFFF_FFFF` in unsigned `u32` arithmetic).

## Class

`crash` in debug/test builds (an uncatchable `debug_assert!` panic on ordinary arithmetic, not
attacker-specific but reachable by any code computing lengths near the `u32` boundary) and `logic`
in release builds (silent corruption to a semantically different sentinel value with no error
raised). Distinct from every other candidate this session - found via git archaeology of an
external, unmerged fix rather than fuzzing, and confirmed independent by verifying the exact
commit this session audits (`df2b9e42`) does not contain the fix.

## Root cause

`Length` wraps a plain `u32`, with `0xFFFF_FFFF` reserved as the `UNDEFINED` sentinel (`0` through
`0xFFFF_FFFE` are the defined range). Four of its five arithmetic operator implementations compute
their result via ordinary wrapping/narrowing `u32`/`i32` arithmetic and rely on a
`debug_assert!(result != UNDEFINED_LEN, ...)` to *notice* a collision - but a `debug_assert!` is
compiled out entirely in release builds (the crate's normal build profile for any downstream user,
and the profile any external fuzzer or production consumer would use), so in release builds
nothing prevents the collision from occurring; in debug/test builds, the assertion **panics the
whole process** instead of returning a `Result::Err` or otherwise handling the situation
gracefully - neither behavior is safe.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Rewrote all four affected operators (`Add<Length>`, `Add<i32>`, `SubAssign<Length>`,
`SubAssign<i32>`) to compute their result in a wider intermediate type (`u64` for the
`Length`+`Length` addition, `i64` for the others, since subtraction can go negative before
clamping) and clamp the result to the valid defined range `[0, UNDEFINED_LEN - 1]`, instead of
letting the narrower `u32`/`i32` arithmetic wrap around or coincidentally land on the reserved
sentinel:

```rust
impl std::ops::Add<Length> for Length {
    fn add(self, rhs: Length) -> Self::Output {
        match (self.0, rhs.0) {
            (UNDEFINED_LEN, _) | (_, UNDEFINED_LEN) => Length::UNDEFINED,
            (l1, l2) => {
                let o = (l1 as u64 + l2 as u64).min(UNDEFINED_LEN as u64 - 1) as u32;
                Length(o)
            }
        }
    }
}
// Add<i32>, SubAssign<Length>, SubAssign<i32> follow the same pattern with i64 + .clamp(0, UNDEFINED_LEN as i64 - 1)
```

Also carried over, by direct analogy (same root cause, same file, same external report), the
already-external-but-unmerged fix to `DataElement::update_value`'s `byte_len as u32` cast, capping
at `u32::MAX - 1` before the cast so a `PrimitiveValue` whose byte length happens to be exactly (or
a multiple of `2^32` away from) `0xFFFF_FFFF` can't silently truncate into the sentinel either.

See `patch.diff` for the exact diff (fix only; the accompanying unit test added during
investigation - `length_arithmetic_never_collides_with_undefined_sentinel` - is not included,
matching this session's convention of keeping `patch.diff` minimal).

- `cargo test -p dicom-core --lib`: baseline (test present, fix removed) - the new test
  **panicked** via the pre-existing `debug_assert!` on the very first assertion (`Add<Length>`),
  exactly as predicted -> with the fix restored - **68 passed, 0 failed**.
- Full workspace (`cargo test --workspace --lib`, all 12 crates with a `#[lib]` target):
  **351 passed / 0 failed / 2 ignored** (pristine, no candidate changes) -> **352 passed / 0
  failed / 2 ignored** with this fix + its 1 test applied - zero regressions anywhere in the
  existing suite.
- Verified both debug and release profiles directly via the standalone `repro/` (see
  `backtrace.txt`): pristine code panics in debug and silently corrupts to `Length::UNDEFINED` in
  release; patched code produces the mathematically correct, clamped result in both profiles.

## What's not done / open questions

- `DataElement::update_value`'s analogous fix (`OVF_3` in the external report) was applied by
  direct analogy but does not have its own dedicated reproduction in this session's `repro/`:
  triggering it for real requires a `PrimitiveValue` whose `calculate_byte_len()` is close to
  4 GiB, which is impractical to construct and allocate for a routine, fast-running repro. The fix
  itself is a trivial, purely defensive one-line change (capping before a cast, no behavior change
  for any realistic value), so it was included anyway rather than left for a future session, but
  readers should be aware only the four operator-level fixes have a from-scratch, independently
  verified reproduction backing them.
- No attempt was made to search for further call sites of `Length` arithmetic within this
  workspace that might be affected by this specific overflow class (as opposed to auditing the
  operator implementations themselves, which was exhaustive - all `impl std::ops::* for Length`
  blocks in `core/src/header.rs` were checked). A workspace-wide search for real-world call sites
  that could feasibly reach the exact overflow/underflow boundary with attacker-controlled input
  was not performed.
- This candidate does not evaluate or draw on any other content of the unmerged
  `origin/fix/length-arithmetic-overflow` branch beyond the one commit examined here; whether that
  branch contains other relevant, still-unmerged fixes was not investigated.
