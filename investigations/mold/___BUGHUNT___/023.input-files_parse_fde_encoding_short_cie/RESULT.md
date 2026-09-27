# 023.input-files_parse_fde_encoding_short_cie

`parse_fde_encoding()` (`src/input-files.cc`, ~line 533), reached from `parse_ehframe()` for every
CIE record in an object's `.eh_frame` section, unconditionally reads `data[8]` and then does
`data.remove_prefix(9)` on a `std::string_view` whose length comes straight from the CIE's own
untrusted, attacker-controlled `size` field (`CieRecord<E>::size()`/`get_contents()`, `src/mold.h`
~line 436). A CIE that declares an implausibly small `size` (1, 2, or 3) produces a 5-, 6-, or
7-byte view, which is too short for either operation - this candidate was seeded from a prior
session's analysis of neighboring code (candidate 008, `___BUGHUNT___/008.../patch.diff`) as a
"different, still-unfixed path" reachable even after 008's own fix, since 008's fix only rejects a
`size` that's inconsistent with the section's *remaining* bytes, not one that's merely very small.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read current source of `parse_fde_encoding()`/`CieRecord<E>` against the hypothesized mechanism | Source reading only | Confirmed base commit `c340272e` has **neither** candidate 008's outer-loop fix nor any other guard on this path - `parse_ehframe()`'s outer loop is completely unvalidated in this pristine checkout. |
| Hand-crafted x86_64 `ET_REL` object, one `.eh_frame` CIE with declared `size=1` (5-byte record), fed to the real `mold` binary CLI | `build-plain` (`-DCMAKE_BUILD_TYPE=Debug`, which mold's own `CMakeLists.txt` compiles with `-D_GLIBCXX_ASSERTIONS`) | **Reproduces**: `std::string_view::operator[]`'s own `__glibcxx_assert(__pos < this->_M_len)` fires on the `data[8]` read (step 3 in the task's numbered mechanism) and aborts (SIGABRT), 4/4 identical runs. |
| Second, isolated crafted object (`size=3`) designed so the version byte doesn't also corrupt the *next* record's reinterpreted size field into an implausibly huge value (avoiding accidental interference with candidate 008's separate, still-unfixed unbounded-`substr(pos)` bug in the outer loop) | Same `build-plain` (Debug) | Reproduces identically - same `operator[]` assertion, same crash, confirming the finding is not an artifact of the specific `size=1` byte layout. |
| Same isolated (`size=3`) object against a **Release** build (`-DCMAKE_BUILD_TYPE=Release`, no `_GLIBCXX_ASSERTIONS`) | `build-release` | **No externally visible symptom** - mold exits 0, links successfully. Confirms by direct observation (not just code reading) that the task's hypothesized "step 4 (`remove_prefix(9)` underflow) walks the read arbitrarily far" concern does **not** materialize: nothing downstream of the underflow in this function ever consults the corrupted `data.size()` to bound anything (the augmentation string is built via a NUL-terminated-C-string constructor, independent of `data`'s own reported size; every later `remove_prefix`/`read_uleb` call advances by a small literal or `read_uleb`-derived count that trivially satisfies the (corrupted, huge) size precondition). |
| Fix: reject `data.size() < 9` up front with `Fatal(ctx)`, rebuild, re-run both crafted objects | `build-plain` (Debug), patched | Both objects now produce a clean `mold: fatal: ...: corrupted CIE record: too short` / exit 1, no crash - 3/3 runs for the `size=1` repro, 1/1 for the isolated `size=3` variant. |
| Full project test suite, before and after the patch | `ctest --test-dir build-plain -j4` | 476/476 passed both before and after (see below). |

## Status

**Confirmed.**

Reproduces deterministically: **4/4 runs** with the `size=1` repro against the pristine Debug
build (`build-plain`, `_GLIBCXX_ASSERTIONS` on via mold's own `CMakeLists.txt`), plus an
independent `size=3` variant crafted to rule out interference from the neighboring candidate-008
bug, which reproduces the identical assertion 1/1 (checked with a second repeat, also see
`output.txt` entries 4 and 6-7 for the patched-vs-unpatched pairing). Also directly observed (not
just inferred) that the same crafted input produces **zero externally visible symptom** in a
Release build without `_GLIBCXX_ASSERTIONS` - see "Root cause" for why, and note the bug is
therefore currently only *observably* a crash under a hardened/debug build, even though the
underlying UB (an out-of-bounds `string_view::operator[]` read, immediately followed by an
out-of-bounds `remove_prefix()`) is real and unconditional regardless of build flags.

**Fully reproducible**: **Yes.** The repro runs the real `mold` binary's real CLI entry point
end-to-end against a hand-crafted `.o` file - this is a single-participant case (a malformed
object file handed to the linker's own public "link this file" entry point *is* the genuine
external input a real attacker/build system would supply), so `CONFIRMATION.md` section 11's
two-participant bar doesn't apply here.

## Class

memory-safety (unchecked `std::string_view::operator[]`/`remove_prefix()` past the view's declared
length; UB per the C++ standard regardless of whether a given build happens to observe a crash).

## Root cause

**Correction to the task's hypothesized mechanism, established by direct testing rather than code
reading alone**: the task description flagged `data.remove_prefix(9)`'s size_t underflow (step 4)
as "the more promising lead" and speculated that `u8 version = data[8]` (step 3) was merely a
harmless adjacent read. Testing shows the opposite ordering of consequence:

- Under any build with `_GLIBCXX_ASSERTIONS` (mold's own `CMakeLists.txt` enables this
  automatically for `CMAKE_BUILD_TYPE=Debug`), **step 3's own bounds assertion in
  `operator[]` fires first**, for *every* `size` value that would also make step 4 dangerous. This
  is not a coincidence: `data[8]` requires `data.size() > 8` to be safe, and `remove_prefix(9)`
  requires `data.size() >= 9` to be safe - these are the same condition (`data.size() >= 9`), so
  whenever `data.size() <= 8` (i.e. CIE `size` field <= 4), `operator[]`'s assertion at `data[8]`
  necessarily fires *before* execution ever reaches `remove_prefix(9)`. **Step 4's underflow is
  therefore structurally unreachable in any build that has the hardening to catch it** - it's
  fully shadowed by step 3's own equivalent check on the exact same precondition.
- Under a build *without* that hardening (plain Release), both `data[8]` and `remove_prefix(9)`
  execute as silent, unchecked UB: `data[8]` reads a real, in-bounds byte of the mmap'd section
  (harmless per this project's established mmap-page lesson - see `___BUGHUNT___/README.md`), and
  `remove_prefix(9)`'s internal `_M_len -= 9` underflows `data`'s reported size to a huge wrapped
  value. Directly confirmed (isolated `size=3` repro, Release build, `output.txt` entry 5) that
  this underflow has **no observable effect**: nothing later in `parse_fde_encoding()` uses
  `data.size()` to bound anything - the augmentation string `aug` is constructed via
  `std::string_view aug = data.data();`, the C-string constructor, which scans for a NUL
  terminator independent of `data`'s own (corrupted) size, and every subsequent `remove_prefix`/
  `read_uleb` call advances by a small, independently-bounded amount that trivially satisfies even
  a huge corrupted precondition. The function completes and returns a (possibly semantically wrong,
  but memory-safe) `fde_ptr_size`.

So the real, reachable bug is: **`parse_fde_encoding()` never validates that the CIE's own declared
`size` leaves at least 9 bytes before touching `data[8]`/`remove_prefix(9)`, both of which have an
implicit "at least 9 bytes" precondition that plain (non-hardened) `string_view` does not enforce.**
This is attacker-reachable via any relocatable object file with a `.eh_frame` section containing a
CIE whose length field is 1-4 (5-8 with the 4-byte length prefix), independent of every other CIE
field. Introduced by the same commit that introduced this whole function, so there was never a
version of this code that validated the record's minimum length.

## Introduced by

5f71cbf98864b306f9e6ccb815ec778a6ade3675 "Write tombstones to .eh_frame_hdr for zero-length FDEs"
(2026-08-13) - this commit added `parse_fde_encoding()` in its entirety (`git show 5f71cbf9 --
src/input-files.cc` shows the whole function as new code, not a modification of pre-existing
lines); confirmed via `git blame` on the base commit, which attributes every line in the function
(including `data[8]` and `remove_prefix(9)`) to this same commit.

## The fix

Added a single bounds check right after `data = cie.get_contents()`, before any of the fixed-offset
reads/`remove_prefix` calls that assume at least 9 bytes:

```cpp
if (data.size() < 9)
  Fatal(ctx) << isec << ": corrupted CIE record: too short";
```

This matches the style of the project's other recently-added bounds checks in the same file (e.g.
candidates 014/017's `Fatal(ctx) << ... : corrupted ...` pattern). Verified against **two**
independently-crafted inputs in the same bug class (per `CONFIRMATION.md` section 7): the original
`size=1` repro and the isolated `size=3` variant - both now produce a clean
`corrupted CIE record: too short` `Fatal()` and exit 1, no crash, 3/3 and 1/1 runs respectively.

- Project test suite (`ctest --test-dir build-plain -j4`, same `build-plain` directory rebuilt
  before/after via `git checkout -- src/input-files.cc` / `git apply patch.diff`): **476/476
  passed before, 476/476 passed after** - no regressions, no new failures.

## What's not done / open questions

- The neighboring, still-unfixed candidate-008 bug (`parse_ehframe()`'s outer loop using the
  single-arg `substr(pos)` with a fully unvalidated `size`) is orthogonal to this candidate and was
  deliberately *not* re-fixed here - this candidate's repro was specifically designed (the isolated
  `size=3` variant) to avoid triggering it, to keep this finding's evidence clean. A future session
  applying both this patch and 008's should re-run both candidates' repros together to confirm no
  interaction.
- Did not check whether the same "small declared size, big-enough section to keep going" shape
  exists in the `.eh_frame_hdr`/FDE-processing code that runs after CIE parsing (this function only
  handles CIEs) - not investigated this session.
- The introducing commit (5f71cbf9, 2026-08-13) is very recent relative to the rest of this
  project's `.eh_frame` parsing code (most of which traces to 2022-2023 per prior sessions' notes)
  - worth flagging that new `.eh_frame`-adjacent code added in this timeframe may be worth a
  systematic re-read for the same "unchecked minimum-length assumption" shape, independent of this
  specific function.
