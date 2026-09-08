# 026: input_files_ctor_ubsan_invalid_downcast

`src/input-files.cc`'s `operator<<(std::ostream&, const InputFile<E>&)` (current line ~1490)
does a raw C-style downcast:

```cpp
ObjectFile<E> *obj = (ObjectFile<E> *)&file;
if (obj->archive_name == "")
  ...
```

`InputFile<E>` has a virtual destructor (`src/mold.h:1890`), so it is a real polymorphic base
of `ObjectFile<E>`/`SharedFile<E>` (`src/mold.h:1980`/`2071`). This downcast is only
well-defined once construction of the derived (`ObjectFile<E>`) part has actually completed.
Calling this `operator<<` on `*this` from *inside* `InputFile<E>::InputFile()`'s own base
constructor body - before the derived object's members (e.g. `ObjectFile<E>::archive_name`,
a `std::string`) have even been constructed - is undefined behavior (a downcast through an
object whose most-derived type isn't yet established / an access to a not-yet-existing
member), even though it "worked" in every manual test run below.

**This candidate's original task framing turned out to be based on an inaccurate premise, and
the actual finding is stronger than what was assigned - see "Correction to the task's premise"
below.**

## Correction to the task's premise

The assigning task assumed this hazard is purely *hypothetical* - reachable only if/when
candidate 002's fix (`___BUGHUNT___/002.input-files_InputFile_ctor_oob_shsize/patch.diff`,
which adds a new `Fatal(ctx) << *this << ": e_shoff is out of range";` call) is ever adopted
for real, and that the pristine, currently-tracked `InputFile<E>::InputFile()` "never actually
does this" (only using `mf->name`, never `*this`, in its one existing `Fatal()` call).

**That premise is wrong.** Reading the actual pristine constructor (`src/input-files.cc`,
current lines 78-108, `base_commit` `c340272e6376f9a8e5b5cd6f6634f925f6d54e5c`) shows it
already has **two** such calls, both predating candidate 002 and both present in pristine,
unpatched `main`:

```cpp
InputFile<E>::InputFile(Context<E> &ctx, MappedFile *mf)
  : mf(mf), symbols(ArenaAllocator<ArenaPtr<Symbol<E>>>(ctx.arena)),
    filename(mf->name) {
  if (mf->size < sizeof(ElfEhdr<E>))
    Fatal(ctx) << *this << ": file too small";          // <-- *this, mid-construction
  if (memcmp(mf->data, "\177ELF", 4))
    Fatal(ctx) << *this << ": not an ELF file";          // <-- *this, mid-construction
  ...
```

(`git log -S` on this exact text traces both calls back through this project's earliest
"temporary"-tagged commits from ~2021 - effectively as old as the `InputFile`/`ObjectFile`
class split itself; see "Introduced by" below.)

Of the two, the **"file too small" check is directly reachable via the real `mold` CLI on a
genuinely truncated but ELF-magic'd `ET_REL` object file** - `src/filetype.cc`'s
`get_file_type()` classifies a file as `FileType::ELF_OBJ` from just its first ~20 bytes
(`e_ident` + `e_type` + `e_machine`) without ever checking the file's total size, so a file
smaller than `sizeof(Elf64_Ehdr)` (64 bytes) sails straight through into
`new_object_file()` -> `ObjectFile<E>`'s constructor -> the base `InputFile<E>::InputFile()`
constructor body above, where `mf->size < sizeof(ElfEhdr<E>)` fires *before* `is_dso` or any
derived member exists. (The "not an ELF file" check is very likely dead code from the real CLI
entry point specifically - `get_file_type()` gates on the `"\177ELF"` magic before ever routing
a file into `ObjectFile<E>`'s constructor - but it shares the exact same hazard and the exact
same fix, so it's fixed alongside the reachable one for free.)

**Net effect: this bug is not hypothetical. It is directly reproducible on pristine,
unpatched `main` (`c340272e6376f9a8e5b5cd6f6634f925f6d54e5c`) right now, via the real `mold`
CLI, with a single crafted 20-byte file - no other candidate's patch needs to be applied at
all.** Candidate 002's patch (applied+reverted temporarily below, exactly as the task
requested) demonstrates only that the *same* hazard would resurface a third time if that
separately-valid, already-Confirmed fix is ever adopted for real - it is not required to
observe the bug.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `operator<<` and `InputFile<E>::InputFile()` against current pristine source | static read | Found the task's premise inaccurate: pristine already has 2 `Fatal(ctx) << *this` calls in the base ctor, not 0 |
| `git log -S` on both existing `*this` call sites | static | Both predate this project's earliest reachable commits ("temporary"-tagged, ~2021-era); not a recent regression |
| Built ASAN+UBSAN (`-DMOLD_USE_ASAN=ON`, g++, `build-asan/`, `-j4`) | one-time config+build | Builds clean; `CMakeLists.txt:109` confirms `-fsanitize=address -fsanitize=undefined` bare (no extra explicit sub-flags), which is enough - g++'s default `-fsanitize=undefined` group includes the vptr/downcast check |
| Scenario A: 20-byte truncated `ET_REL` ELF object (`repro/make_truncated_elf.py`), pristine code, run via real `mold` CLI | ASAN+UBSAN build, 5 runs | **UBSan `runtime error: downcast of address ... which does not point to an object of type 'ObjectFile'` fires every time, immediately before the expected `mold: fatal: ...: file too small` message and exit(1). 5/5 runs.** |
| Scenario B: candidate 002's patch applied temporarily, out-of-range `e_shoff` object (`repro/make_bad_elf_002.py`, reused from 002's own generator) | ASAN+UBSAN build, 3 runs | Same UBSan diagnostic, this time via 002's own new `Fatal(ctx) << *this << ": e_shoff is out of range"` call. 3/3 runs. Confirms the task's originally-hypothesized scenario too, as a secondary/consistent data point |
| Second input in the same class: 48-byte truncated file (different boundary, still < 64) | ASAN+UBSAN build, post-fix | Also clean after the fix (no UBSan diagnostic, correct `Fatal()` message) - fix isn't narrowly tailored to the one exact byte length first tried |
| Full `just test` cycle (fresh `git clone` of `base_commit`, build, both scenarios, apply fix, rebuild, both scenarios again) | `build-asan/`, `-j4` throughout | Reproduces identically from a genuinely fresh clone (not just this worktree) - see `output.txt` |
| `ctest --test-dir build-asan -j4`, before and after the fix | 476 registered tests | 475/476 both before and after (1 pre-existing, unrelated `x86_64-run-clang` ASAN/LD_PRELOAD failure, documented in `___BUGHUNT___/README.md` as a known pre-existing issue on this host, not a regression) |

## Status

**Confirmed.**

Reproduces **5/5 runs** for Scenario A (pristine code, no other patch applied - the actual,
currently-live bug) and **3/3 runs** for Scenario B (candidate 002's patch applied
temporarily, matching the task's original hypothesis). Same UBSan diagnostic text and same
source line class (`operator<<`'s downcast line) in both cases; only the second `mold: fatal:
...` line (and its source line number) differs, matching whichever `Fatal(ctx) << *this` call
was hit.

**Fully reproducible: Yes.** The repro runs the real, unmodified `mold` binary (built from a
genuine `git clone` of `base_commit`, no internal function called directly) via its actual CLI
entry point (`mold <crafted-file> -o <out>`), on a single external artifact (a crafted object
file) that is exactly the kind of input a real user/build system would hand to the linker -
there is no second participant to fake here (see `CONFIRMATION.md` section 11's exception:
"a malformed file handed to a parser's real public entry point *is* the genuine external
input").

## Class

**UB** (undefined behavior - downcast through / member access on an object whose derived part
has not yet been constructed, flagged by UBSan's vptr check; specifically NOT a raw
out-of-bounds memory read like most of this project's other confirmed candidates).

## Root cause

`InputFile<E>::InputFile(Context<E>&, MappedFile*)` is the **base-class** constructor for the
`ObjectFile<E>`/`SharedFile<E>` hierarchy. While this constructor's body is executing, the
object's dynamic type is still `InputFile<E>` - the derived part (which is what actually makes
it an `ObjectFile<E>`) has not been constructed yet. Two `Fatal(ctx) << *this << ...` calls in
this constructor's early validation logic pass `*this` to
`operator<<(std::ostream&, const InputFile<E>&)`, which unconditionally does
`(ObjectFile<E>*)&file` and then reads `obj->archive_name` (a `std::string` member that lives
in the derived, not-yet-constructed part of the object) whenever `file.is_dso` is `false` (which
it always is at this point, since `is_dso`'s default member initializer `= false` already ran
before the constructor body, even though `is_dso` was never in the mem-initializer list).
Reading a not-yet-constructed `std::string` through a downcast pointer, and the downcast itself,
are both undefined behavior - flagged live by g++'s `-fsanitize=undefined` vptr check as
`downcast of address ... which does not point to an object of type 'ObjectFile'`.

Empirically this "worked" (no crash, correct-looking `mold: fatal: ...` message printed) in
every single run of both scenarios above - a textbook case of UB that happens to behave
correctly on this particular compiler/allocator/build, exactly the kind of latent hazard that
only a sanitizer surfaces.

## Introduced by

**Not determined precisely - present since this project's earliest reachable ("temporary"-
tagged) commit history.** `git log -S"Fatal(ctx) << *this << \": file too small\""` (and the
"not an ELF file" sibling) traces back through commits `ebc8a68c`, `8558a98b`, `8963b563`,
`9aac607c`, `2f417ddf`, `ea37097f` (all pre-mold-rename, when the file was `object_file.cc`) -
i.e. this exact `Fatal(ctx) << *this` pattern is essentially as old as the `InputFile`/
`ObjectFile` class split itself, from the project's very first working prototype (long before
`c340272e`). Not worth a full history archaeology session to find one single "introducing"
commit - the class hierarchy (and this constructor's shape) has been foundational and largely
unchanged in this respect for the project's entire life.

## The fix

Changed both existing `Fatal(ctx) << *this` calls in `InputFile<E>::InputFile()`'s base
constructor to `Fatal(ctx) << mf->name`, matching the style the *third*, original neighboring
`Fatal()` call in this same constructor already uses two lines later
(`Fatal(ctx) << mf->name << ": e_shoff or e_shnum corrupted: ..."`) - `mf->name` is available,
correct, and fully constructed at this point (it's in the mem-initializer list, and
`filename(mf->name)` is set before the constructor body runs), so this avoids the hazard
entirely without touching `operator<<` at all, and without changing any user-visible error
message text.

```diff
   InputFile<E>::InputFile(Context<E> &ctx, MappedFile *mf)
     : mf(mf), symbols(ArenaAllocator<ArenaPtr<Symbol<E>>>(ctx.arena)),
       filename(mf->name) {
+  // Use mf->name rather than *this here: operator<<(ostream&, InputFile<E>&)
+  // downcasts to ObjectFile<E>/SharedFile<E>, which is only well-defined once
+  // this base constructor has finished - the derived part doesn't exist yet
+  // while we're still in this constructor's body.
   if (mf->size < sizeof(ElfEhdr<E>))
-    Fatal(ctx) << *this << ": file too small";
+    Fatal(ctx) << mf->name << ": file too small";
   if (memcmp(mf->data, "\177ELF", 4))
-    Fatal(ctx) << *this << ": not an ELF file";
+    Fatal(ctx) << mf->name << ": not an ELF file";
```

This is `patch.diff` in this directory, and is a diff against pristine `base_commit` **with
candidate 002's patch NOT included/needed** - it fixes the bug as it exists in the real,
currently-tracked codebase right now, independent of whether 002's patch is ever adopted.

Verified:
- Re-ran the exact Scenario A repro (20-byte truncated file) against the patched build: UBSan
  diagnostic is gone, `mold: fatal: ...: file too small` message and exit(1) unchanged. 3/3 runs.
- Per CONFIRMATION.md section 7 (vary the input within the same bug class): also tried a
  48-byte truncated file (a different boundary, still `< sizeof(ElfEhdr<E>)`) - same clean
  result, not just the one exact byte length first used.
- Re-applied candidate 002's patch **on top of** this fix (temporarily, for verification only,
  matching the task's request) and re-ran Scenario B: 002's own newly-added
  `Fatal(ctx) << *this << ": e_shoff is out of range"` call is **untouched by this patch** (as
  intended - it doesn't exist in pristine `base_commit`) and **still shows the UBSan diagnostic**
  - this is expected, not a verification failure; it demonstrates the fix is correctly scoped to
  only the calls that exist in pristine code today, while confirming the same fix approach
  (`mf->name` instead of `*this`) would need to be applied to any such future addition too. See
  `output.txt` for the full transcript of all of the above, and both the in-worktree manual runs
  and the standalone `just test` fresh-clone cycle.
- Project test suite: **475/476 before -> 475/476 after** (`ctest --test-dir build-asan -j4`).
  The one failure both before and after is `x86_64-run-clang`, a pre-existing, unrelated
  ASAN/`LD_PRELOAD` collision with mold's own `mold-wrapper.so` trick, documented in
  `___BUGHUNT___/README.md` as present on unpatched `main` regardless of source changes - no new
  failures introduced by this fix.

Both patches (candidate 002's, temporarily, and this candidate's own) were fully reverted from
the assigned worktree before finishing; the worktree's `git status` is clean except for this
`CANDIDATE_OUT/` directory.

## What's not done / open questions

- **The "not an ELF file" check is very likely unreachable from the real `mold` CLI today**
  (`get_file_type()` gates on the ELF magic bytes before ever constructing an `ObjectFile<E>`/
  `SharedFile<E>`), so it was not independently repro'd as its own live bug - it was fixed
  alongside the reachable "file too small" check purely because it shares the identical
  `Fatal(ctx) << *this` hazard and the identical one-line fix, not because a second live trigger
  was demonstrated for it specifically.
- **This is a systemic pattern, not just these two calls**: *any* future `Fatal(ctx) << *this`
  (or any other code that calls `operator<<` on `*this`, directly or transitively) added inside
  `InputFile<E>::InputFile()`'s base-constructor body carries the identical hazard, since the
  derived part is never constructed at that point regardless of which specific check triggers
  it. Candidate 002's own (separately Confirmed, already-fixed, but never actually merged per
  this skill's revert-after-verify rule) added exactly one more instance of this - a future
  session adopting 002's fix for real should apply this same `mf->name` substitution to 002's
  new line at the same time, or re-run this candidate's Scenario B check to confirm.
  `operator<<` itself was deliberately left unchanged (approach (b), not (a), per the task's own
  framing) - a more defensive `operator<<` that's safe to call from anywhere, including
  mid-construction, was considered but judged less clean than simply not calling it during
  construction in the first place, since there's no compile-time enforceable way to make a
  general-purpose `operator<<` safe against "some caller passed an as-yet-incomplete object"
  short of removing the downcast pattern from `operator<<` entirely (a larger, more invasive
  change than justified by two call sites, one of which appears unreachable anyway).
- No `SharedFile<E>`-specific (`ET_DYN`/DSO) trigger was independently constructed - the "file
  too small" check runs identically regardless of which derived type will eventually be
  constructed (the branch happens before `is_dso` is read in `get_file_type`'s caller chain),
  so a DSO-flavored repro would very likely hit the exact same line and diagnostic, but this
  wasn't separately verified since the `ObjectFile<E>` (`ET_REL`) trigger already demonstrates
  the mechanism fully.
