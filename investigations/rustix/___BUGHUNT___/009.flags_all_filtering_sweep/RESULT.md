# flags_all_filtering_sweep

Follow-up to candidate 007 (`statx`'s `mask & StatxFlags::all()` forward-compat filter became a
silent no-op once `StatxFlags::all()` became `u32::MAX` via bitflags' externally-defined-flags
support, commit 7ff96318). That commit applied the same `const _ = !0;` opt-in crate-wide across
~44 files, so the backlog suspected other `mask & SomeFlags::all()`-shaped filtering guards might
share the identical silent-no-op bug.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `grep -rn "Flags::all()" src/ --include=*.rs` | Whole crate, both backends | Exactly 2 matches, both `src/backend/{libc,linux_raw}/fs/syscalls.rs`'s `mask & StatxFlags::all()` - the same two call sites already confirmed and root-caused by candidate 007 (one per backend, same bug) |
| `grep -rn "::all()" src/ --include=*.rs \| grep -v StatxFlags` | Whole crate | Zero other `SomeFlags::all()` call sites anywhere in the crate at all, filtering or otherwise |
| `grep -rn "from_bits_truncate\|from_bits_retain\|bits() &\|& mask\|contains(.*Flags::all" src/backend --include=*.rs \| grep -i "mask\|filter"` | Whole crate | Found `umask`'s `Mode::from_bits_retain` (not a filtering guard - it's just decoding the kernel's returned mode bits, no masking-against-`all()` shape) and the same two `STATX__RESERVED` lines candidate 007 already covers - no new pattern |

## Status

**Not reproducible** (no new instance of the bug class found). The crate has exactly two
`SomeFlags::all()` call sites total, both already covered by candidate 007's confirmed bug and
fix. Despite commit 7ff96318 opting ~44 files' flag types into external-bits support (`const _ =
!0;`), essentially none of the other files actually use `.all()` as a runtime filtering mask the
way `statx` does - most just use a flags type as an opaque bitflag parameter passed straight to a
syscall, with no local "strip unknown bits before use" guard to break.

## Fully reproducible

N/A (no bug found to reproduce).

## Root cause

N/A - the suspected additional instances don't exist. The `Flags::all()`-as-filter pattern
candidate 007 found is apparently unique to `statx`'s forward-compatibility design (a mask meant
to let callers request a superset of known flags and have the wrapper silently drop anything the
kernel wouldn't understand) - most of the crate's other bitflags-typed parameters are just passed
through to a syscall without any local masking step at all, so there's no `all()` call to become a
no-op in the first place.

## Introduced by

N/A - not applicable, no bug confirmed.

## The fix

N/A - no bug found, nothing to fix.

## What's not done / open questions

- This sweep only checked for the *exact* `Flags::all()` filtering shape (or close textual
  variants: `from_bits_truncate`, `bits() & mask`, `& mask`). A semantically equivalent guard
  written differently (e.g. manually listing every known flag's numeric value in a local constant
  instead of calling `.all()`) wouldn't be caught by this grep and wasn't otherwise audited.
- Not re-checked: whether any *other* bitflags type in the crate has its own local
  "forward-compat mask" pattern that predates or postdates 7ff96318 entirely (i.e. never used
  `.all()` for it) - out of scope for this candidate, which was specifically about the `.all()`
  filtering shape from candidate 007.
