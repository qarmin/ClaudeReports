# timespec_checked_sub_premature_tv_sec_overflow

This candidate started as a process/meta lead from `projects/rustix.md`'s "Risk map update,
session 2026-10-01" (code-reading/Miri-sweep/GitHub-issue-mining discovery visibly drying up,
naming a deeper fuzzing campaign - `fuzz-bug-repro`, run separately - as one of two remaining
high-yield angles; `bug-hunt` itself does not call other skills, so this session instead did the
bounded, shallow one-off fuzzing this skill's own scope allows). `rustix` has no `fuzz/` directory
of its own, so a short one-off `cargo-fuzz` harness was written from scratch targeting a handful of
`rustix`'s pure-computation public API entry points (no real syscall in the call path, so fuzzable
directly, unlike most of `linux_raw`'s inline-asm backend): `path::Arg`/`with_c_str`'s
stack/heap-buffer boundary, `net::SocketAddrUnix::new`/`new_abstract_name`, `path::DecInt`, and
`event::Timespec::checked_add`/`checked_sub` (differential-tested against an independent `i128`
reference computation).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo +nightly fuzz` (ASAN + libFuzzer), target `fuzz_arg` | `path::Arg::into_with_c_str` (`&[u8] -> CStr`), `net::SocketAddrUnix::new`/`new_abstract_name`, `path::DecInt::new` over all integer widths; release+ASAN+debug-assertions build; ~181s wall clock, 158,695,972 executions, corpus converged (coverage plateaued at 75/121 features) | No crash (one crash found and fixed was in the harness's own test-value arithmetic, not rustix - see below); clean negative |
| `cargo +nightly fuzz` (ASAN + libFuzzer), target `fuzz_timespec` | `event::Timespec::checked_add`/`checked_sub`, differential-tested against an independent `i128` total-nanoseconds reference, input restricted to the documented `0 <= tv_nsec < 1_000_000_000` precondition; release+ASAN+debug-assertions build; 181s, corpus-seeded re-run with 82,010,033 further executions | **Crash found** (`checked_sub` mismatch) - see below |
| Minimal hand-reduction + standalone public-API repro (`rustix::event::Timespec::checked_sub` directly, no fuzzer/harness assertions) | Two independently-constructed inputs in the same bug class (the fuzzer's own minimized seed, plus a second hand-built one at a different offset from the same boundary), release build | Both reproduce 3/3 runs each (deterministic, pure function - no flakiness expected or observed) |

Note on the `fuzz_arg` target's own crash: the harness's *own* test-value construction
(`i64::MIN + (v as i64 % 4)`) could itself overflow under the fuzz build's debug-assertions,
unrelated to any rustix code (`v as i64 % 4` can be negative, and adding that to `i64::MIN`
underflows). Fixed by switching to `wrapping_add`/`wrapping_sub` in the harness itself before
continuing the `fuzz_arg` run - not a project finding, noted here only so a future session reading
this `RESULT.md` understands why the harness source (visible in this folder's history/description)
looks different from a naive first draft.

## Status

**Confirmed**

Reproduces 3/3 runs via the standalone `repro/` (deterministic - this is a pure, `const fn`
arithmetic function with no I/O, no randomness, no layout dependence; a reproduction is not
expected to ever be flaky). Also reproduces 3/3 re-runs of the original libFuzzer crash artifact
directly.

**Fully reproducible**: **Yes**. `repro/` calls rustix's real, public `rustix::event::Timespec`
type and its real public `checked_sub` method directly - the genuine public entry point a real
caller would use. There is no second participant to fake here: this is a single-participant, pure
arithmetic function operating only on caller-supplied struct fields (both of which are already
`pub`), not a parser/decoder reacting to another participant's wire input - so CONFIRMATION.md
section 11's "real second participant" requirement does not apply (its own text names this as the
normal, unexceptional case: "a malformed file handed to a parser's real public entry point" - here
there isn't even a file, just two caller-constructed `Timespec` values, which is an even more
direct case of "no other side being impersonated").

## Class

logic

## Root cause

`Timespec::checked_sub` (and its mirror-image sibling `checked_add`) checks `self.tv_sec.checked_
sub(rhs.tv_sec)` for `i64` overflow **before** applying the nanosecond borrow/carry adjustment,
instead of after. Concretely (`src/timespec.rs`, pre-fix):

```rust
pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    if let Some(mut tv_sec) = self.tv_sec.checked_sub(rhs.tv_sec) {
        let mut tv_nsec = self.tv_nsec - rhs.tv_nsec;
        if tv_nsec < 0 {
            tv_nsec += 1_000_000_000;
            if let Some(borrowed_sec) = tv_sec.checked_sub(1) {
                tv_sec = borrowed_sec;
            } else {
                return None;
            }
        }
        Some(Self { tv_sec, tv_nsec })
    } else {
        None   // <- reached even when the eventual borrowed value would fit
    }
}
```

When `self.tv_sec - rhs.tv_sec` overflows `i64` by **exactly 1** (i.e. the true mathematical
difference is `i64::MAX + 1`) *and* `self.tv_nsec < rhs.tv_nsec` (so a 1-second nanosecond borrow
is about to be subtracted), the correct final `tv_sec` is `i64::MAX + 1 - 1 = i64::MAX`, which is
perfectly representable - but the function already returned `None` one line earlier, because the
raw `tv_sec` subtraction (before the borrow is known to be needed, let alone applied) overflowed on
its own. The borrow that would have rescued the result into range never gets a chance to run. This
is a false negative: a legitimate, in-range `Timespec` subtraction is reported as "overflow"
(`None`) instead of returning the correct `Some(...)`.

Minimal concrete instance: `Timespec{tv_sec: 0, tv_nsec: 0}.checked_sub(Timespec{tv_sec: i64::MIN,
tv_nsec: 1})`. `0 - i64::MIN` alone is `9223372036854775808`, one past `i64::MAX`
(`9223372036854775807`), so the initial `checked_sub` fails and the function returns `None` -
but the true result, after borrowing 1 second for `0ns - 1ns`, is `Timespec{tv_sec: i64::MAX,
tv_nsec: 999_999_999}`, which fits in `i64` just fine.

`checked_add`'s equivalent code was read and reasoned about for a mirror-image bug (a carry that
could rescue an add-side overflow), but no triggering input was found or expected: the nanosecond
carry for `checked_add` only ever *increases* `tv_sec` by 0 or 1 (never decreases it), so it can
only make an already-overflowing sum *more* overflowed, never rescue one - there is no equivalent
false negative on the add side. The fix below still rewrites `checked_add` the same way (for
symmetry/consistency and because it's equally correct and no more complex), not because a distinct
add-side bug was found.

## Introduced by

b710cf2f "Add addition, subtraction and conversion to/from `core::time::Duration` for Timespec
(#1311)" - this is the commit that originally introduced both `checked_add` and `checked_sub` with
this exact check-before-borrow/carry structure; the bug has been present unchanged since.

## The fix

Widen the `tv_sec` computation to `i128` so the nanosecond borrow/carry is applied to the exact
mathematical difference/sum *before* checking whether the final result fits in `i64`, rather than
checking the raw `tv_sec`-only subtraction/addition for overflow first:

```rust
pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    let mut tv_sec = self.tv_sec as i128 - rhs.tv_sec as i128;
    let mut tv_nsec = self.tv_nsec - rhs.tv_nsec;
    if tv_nsec < 0 {
        tv_nsec += 1_000_000_000;
        tv_sec -= 1;
    }
    if tv_sec < Secs::MIN as i128 || tv_sec > Secs::MAX as i128 {
        return None;
    }
    Some(Self { tv_sec: tv_sec as Secs, tv_nsec })
}
```

(and the equivalent widening for `checked_add`). Both functions remain `const fn` (basic `i128`
arithmetic/comparison/`as` casts are const-stable well below this crate's MSRV of 1.65) and keep
their existing public signatures - a pure bugfix, no API change.

Checked against more than the one triggering input (CONFIRMATION.md section 7): `repro/` contains
both the minimized fuzzer seed and a second, independently-constructed input at a different
offset from the same `i64::MAX`-boundary shape (`tv_sec: 10` vs `tv_sec: -9223372036854775798`) -
both were confirmed to reproduce the bug against the pristine tree and to be fixed against the
patched tree. A third case (the pre-existing doc-tested genuine-overflow example, `i64::MIN -
(0, 1ns)`, which has no valid representable result) was also checked and correctly still returns
`None` after the fix - the fix narrows the false-overflow case without weakening the real one.

Before/after of re-running `repro/` (see `output.txt` for full captured output):
- Before (pristine `287214b8`): exits 1, "REPRODUCED" (2 of 3 cases mismatch).
- After (`patch.diff` applied): exits 0, "did not reproduce" (all 3 cases match the independently
  computed correct result, including the still-correctly-`None` genuine-overflow sanity case).

- Project test suite (`cargo test --workspace --features std,all-apis,use-libc --no-fail-fast`,
  same sandbox): **411 passed; 1 failed; 5 ignored**, identical before and after the patch. The 1
  failure is the pre-existing, environment-caused `ioctl::test_int_setter` (`NotFound` opening
  `/dev/net/tun`, which does not exist in this sandbox - documented in `projects/rustix.md`'s "no
  `/dev/net/tun`" gotcha), unrelated to `timespec.rs`. `Timespec::checked_add`/`checked_sub`'s own
  doc-tests (which include the genuine-overflow `None` cases) pass both before and after.

## What's not done / open questions

- `checked_add`'s own code path was reasoned about but not independently fuzzed to the same depth
  as `checked_sub` within this session's time budget (the `fuzz_timespec` harness exercises both
  functions every run, but the crash it actually found was on the `checked_sub` side; 82M+
  combined executions produced no `checked_add` mismatch). The fix widens both symmetrically, so
  this is a belt-and-suspenders improvement for `checked_add` rather than a confirmed second bug.
- This was a genuinely shallow, one-off campaign (two small custom harnesses, a few minutes of
  wall-clock fuzzing each) - exactly the scope `bug-hunt`'s own `HEURISTICS.md`/`CONFIRMATION.md`
  describe as "deliberately shallow" for this skill. The original meta-lead's own suggestion (a
  real, longer campaign via the separate `fuzz-bug-repro` skill, with a persistent corpus and
  proper harness coverage across more of the public API - `net`, `fs`, `io_uring` struct builders,
  bitflags-heavy types, etc.) remains open and likely still worth doing; this session's two
  harnesses covered only a small, hand-picked slice of the full public surface.
- Other call sites that consume `Timespec::checked_add`/`checked_sub`'s result were not audited in
  this session for how they handle an (even correctly produced) `None` - out of scope here, since
  the bug is in the arithmetic functions themselves, not in any particular caller.
