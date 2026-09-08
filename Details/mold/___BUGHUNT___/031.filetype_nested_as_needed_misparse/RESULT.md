# filetype_nested_as_needed_misparse

`Script<E>::get_script_output_type()` (`src/linker-script.cc:252-258`) - the function
`get_machine_type()` in `src/filetype.cc:204` dispatches to for `FileType::TEXT` - peeks into a
linker script to work out its machine type. It unwraps exactly **one** level of `AS_NEEDED` with an
`if`, so a `GROUP(AS_NEEDED(AS_NEEDED(libfoo.so)))` script makes it try to open a library literally
named `AS_NEEDED`. Handed on as a side observation by candidate 028, which saw it while fixing an
unrelated recursion bug in the same functions and explicitly left it alone.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `get_script_output_type()` and the main parser `read_group()` side by side, and trace every caller of `get_machine_type()` | code reading, source at c340272e | Found the asymmetry: `read_group()` (:194-197) *recurses* on `AS_NEEDED` and handles any depth; `get_script_output_type()` (:254-255) unwraps with a single `if`. Also found the bug is **not** confined to the `-m`-inference path - `open_library()` (`src/main.cc:163`) calls `get_machine_type()` on every `-l`-resolved file regardless of `-m` |
| Establish the reference behavior: 8 nesting/wrapping shapes through GNU ld 2.46 | `/usr/bin/ld`, `scratch/ldtest2.sh` | GNU ld accepts nested `AS_NEEDED` at 2, 3 and 5 levels, with comments and quoted names. It **rejects** `AS_NEEDED()`, `GROUP(GROUP(...))` and `INPUT(INPUT(...))` as syntax errors, so those wrapper shapes are out of scope - `AS_NEEDED` is the only directive in ld's grammar that legitimately nests here |
| Differential sweep: same 8 scripts through mold in 3 link modes vs GNU ld in 2 | plain Debug build (`-DCMAKE_BUILD_TYPE=Debug`), `repro/run.sh` | **6/8 shapes: mold fails, GNU ld links.** mold's failure is `mold: fatal: <script>:1: ... ^ library not found: AS_NEEDED`. Mode B (positional script + explicit `-m`) succeeds for every shape, confirming the main parser is fine and only the inference path is broken |
| Check the `-l` path specifically, with `-m elf_x86_64` explicitly on the command line | same build | Fails identically. `-m` does **not** avoid the bug when the script arrives via `-l` - which is how real scripts (`/usr/lib/x86_64-linux-gnu/libc.so`, GCC 16's `libatomic_asneeded.so`) are found |
| Compare the resulting binaries, not just exit status, where both linkers succeed | `readelf -d`, in `repro/run.sh` | mold's and GNU ld's `DT_NEEDED` sets agree on every shape (basenames compared; ld records `./libfoo.so`, mold records `libfoo.so` - an unrelated pre-existing spelling difference) |
| Real-toolchain reachability: add a nested-`AS_NEEDED` case to the project's own `test/linker-script-group-as-needed.sh`, which links via `gcc -B. -L$t -lscript2` | ctest, pristine `src/linker-script.cc` | **Test #224 fails** on pristine source with `library not found: AS_NEEDED`. `cc -###` confirms gcc passes `-m elf_x86_64` on that link, so this is the `-l` path failing with `-m` present, driven by a real compiler driver |
| Repeat the repro 3x on pristine and 3x on the patched build | same build | 3/3 fail (exit 1) pristine, byte-identical output each time; 3/3 pass (exit 0) patched, byte-identical. Fully deterministic |
| Same-bug-class inputs beyond the repro (CONFIRMATION.md section 7) | `repro/extra_checks.sh` | Nesting depths 10/100/500/5000: all fail pristine, all link patched (GNU ld itself gives up at 5000 with an error, mold's iterative unwrap does not). 7 degenerate/truncated nestings: rejected cleanly with exit 1 both before and after, matching GNU ld's rejection - no hang, no crash. An i386-only nested script (`mold -r`): pristine fails, patched infers `elf_i386` and emits a correct `ELF 32-bit LSB relocatable, Intel i386` |
| Project test suite before and after | `ctest --test-dir build-plain -j4` | 476/476 before, 476/476 after (16 skipped by ctest for missing zstd/hwasan/apx features, identically both times) |
| `git log -S'AS_NEEDED'` / `-S'resolve_path(tok[2]'` on the file | git history | Traced the three commits that shaped this code; see "Introduced by" |

## Status

**Confirmed**.

Reproduction rate: **3/3** full `repro/run.sh` runs against the pristine Debug build, with 6 of the
8 shapes failing in every run and byte-identical output across runs; **3/3** runs against the
patched build with all 8 shapes passing. Deterministic - no seed, layout or timing dependence
(it is a parser branch, not a memory bug). Build-independent: nothing here depends on `Debug` vs
`Release`; Debug was used only for consistency with this workspace's other candidates.

**Fully reproducible**: **Yes**. `repro/run.sh` runs the real `mold` binary and the real
system `/usr/bin/ld` from the command line on plain-text linker scripts and real `.o`/`.so` files
produced by the real `cc` - the exact artifacts a real user/toolchain supplies. There is no second
participant being stood in for (CONFIRMATION.md section 11 does not apply). The project's own
ctest case additionally drives it through the real `gcc` driver.

## Class

logic

## Root cause

`src/linker-script.cc:252-258` at c340272e:

```cpp
  if (tok.size() >= 3 && (tok[0] == "INPUT" || tok[0] == "GROUP") &&
      tok[1] == "(") {
    if (tok.size() >= 5 && tok[2] == "AS_NEEDED" && tok[3] == "(")   // <-- `if`, not `while`
      tok = tok.subspan(2);
    if (MappedFile *mf = resolve_path(tok[2], false))
      return get_machine_type(ctx, rctx, mf);
  }
```

The tokenizer emits `AS_NEEDED` and `(` as separate tokens, so one level of `AS_NEEDED(` is
two tokens and the single `subspan(2)` peels exactly one. For
`GROUP ( AS_NEEDED ( AS_NEEDED ( libfoo.so ) ) )` the peel leaves
`AS_NEEDED ( AS_NEEDED ( libfoo.so ...`, whose `tok[2]` is still the literal string `AS_NEEDED`.
That gets handed to `resolve_path()`, which searches the script's directory, the cwd and every
`-L` directory for a file called `AS_NEEDED`, finds nothing, and ends in
`error(tok, "library not found: " + str)` - a `Fatal`, so the link aborts. Nested `AS_NEEDED` is
legal GNU ld syntax (ld's grammar makes `AS_NEEDED '(' input_list1 ')'` itself an `input_list1`
item, so it composes with itself), and GNU ld 2.46 links all of these.

**Why the main parser does not share the bug** (the asymmetry that makes this specifically an
inference-path defect): `Script<E>::read_group()` at :193-200 handles `AS_NEEDED` by *recursing*
into itself, so it peels arbitrarily many levels. mold therefore links the identical script
correctly whenever it takes the main-parser path. The two functions duplicate the same "skip past
AS_NEEDED wrappers" logic with different fidelity, and only the copy in
`get_script_output_type()` is single-level.

**Reachability - two paths, and the second one is not about `-m` at all:**

1. `src/main.cc:136 detect_machine_type()` -> `Script(...).get_script_output_type()`. Only
   reached when `-m` is absent *and* no non-TEXT positional input already answered the question.
2. `src/main.cc:163 open_library()` -> `get_machine_type()` -> (`FileType::TEXT`) ->
   `Script(...).get_script_output_type()`, `src/filetype.cc:204`. This runs on **every** file
   found through `-l<name>`, whether or not `-m` was given. Since text linker scripts are routinely
   installed under library names (`/usr/lib/x86_64-linux-gnu/libc.so`, and GCC 16's
   `libatomic_asneeded.so`, which is what motivated e0a6f8e1 in the first place), this is the path a
   real toolchain would hit. `Script<E>::resolve_path(..., check_target=true)` at :145 reaches the
   same function for scripts named inside another script.

The visible symptom in both cases is a hard `mold: fatal: library not found: AS_NEEDED`, i.e. the
error names a directive keyword as if it were a missing library - so a user hitting it gets a
diagnostic that points nowhere useful.

## Introduced by

e0a6f8e1 "Fix getting output type from linker scripts that use AS_NEEDED" (2026-02-28) is the
commit that introduced the current single-level unwrap - it added exactly the `if` that should have
been a `while`, to make GCC 16's `libatomic_asneeded.so` work (mold issue #1545). Before it, *any*
leading `AS_NEEDED` broke this function, so e0a6f8e1 is an incomplete fix rather than the origin of
the underlying assumption.

For completeness, the two earlier commits that set the stage, both found via `git log -S` on the
file:

- cf8cce6d "-r: Infer machine type even if all input files are given via linker script"
  (2022-11-28) added the `INPUT`/`GROUP` branch and its "`tok[2]` is a filename" assumption. It used
  `MappedFile::open()`, which returns null on failure, so back then a nested (or any)
  `AS_NEEDED` merely made inference return "unknown", ending in `-m option is missing`.
- 4eeebf02 "Define Script class and move linker script-related code there" (2024-07-14) replaced
  that `open()` with `resolve_path(tok[2], false)`, which turns a miss into the hard
  `library not found:` `Fatal` seen today.

## The fix

`patch.diff`, two files:

- `src/linker-script.cc`: `if` -> `while` on the `AS_NEEDED` unwrap. The loop is **iterative, not
  recursive** - deliberately, since candidate 028 had just fixed an unbounded-recursion crash in the
  adjacent `read_group()`/`get_script_output_type()` code and a recursive unwrap here would need the
  same depth cap. It is also inherently bounded: each iteration requires `tok.size() >= 5` and then
  consumes 2 tokens, so it runs at most `(n-3)/2` times and always leaves `tok.size() >= 3`, keeping
  the following `tok[2]` in range on every path (the same invariant the original `if` relied on).
  This patch stands alone against pristine c340272e and does not include 028's changes.
- `test/linker-script-group-as-needed.sh`: extends the project's existing test with the nested
  `GROUP(AS_NEEDED(AS_NEEDED("$t/libB.so")))` case, exercised both as `-lscript2` and as
  `-l:libscript2.a`. Verified to **fail** against pristine `src/linker-script.cc` and pass with the
  fix, so it is a real regression test and not a tautology.

Why it is sufficient rather than tuned to the one triggering input (CONFIRMATION.md section 7) -
checked against, beyond the 6 failing shapes in `repro/run.sh`: nesting depths 10, 100, 500 and
5000 (all now link; GNU ld itself errors out at 5000, mold does not); `INPUT` as well as `GROUP`;
comments and quoted filenames inside the nesting; a multi-line nesting; nesting that is not the
first item in the group; a plain `OUTPUT_FORMAT` script; 7 degenerate or truncated nestings
(`AS_NEEDED(AS_NEEDED())`, unterminated `AS_NEEDED(AS_NEEDED`, a nested reference to a genuinely
missing file, an unbalanced group) which must stay rejected and do, cleanly, with exit 1 and no hang
or crash; and an i386-only nested script linked with `mold -r`, which now correctly infers
`elf_i386` and emits `ELF 32-bit LSB relocatable, Intel i386` where pristine mold aborted - the
positive proof that the function now does the job it exists for, not just that it stopped erroring.

- Project test suite: **476/476 passing before -> 476/476 passing after**
  (`ctest --test-dir build-plain -j4`, 0 failures either way; 16 tests skipped by ctest for missing
  zstd/hwasan/apx/etc. features, identically in both runs). Covers the project's own
  `linker-script.sh` .. `linker-script6.sh`, `linker-script-group-as-needed.sh` (the one extended
  here), `linker-script-defsym.sh`, `linker-script-error.sh`, `linker-script-relocatable.sh` and
  `linker-script-version.sh`.

The patch was reverted from the worktree after `patch.diff` was captured; the tree is back at
pristine c340272e.

## What's not done / open questions

- **Duplicated logic, not just a duplicated bug.** The real design smell is that
  `get_script_output_type()` re-implements a fragment of `read_group()`'s directive handling by
  index arithmetic instead of sharing it. The patch keeps the existing shape (minimal, matches how
  the maintainer wrote it in e0a6f8e1); a maintainer may prefer to factor the "skip `AS_NEEDED`
  wrappers" step out so the two can never drift again.
- **Other wrapper shapes were checked and are genuinely out of scope**, so this is not a
  half-audit: GNU ld 2.46 rejects `GROUP(GROUP(...))`, `INPUT(INPUT(...))` and `AS_NEEDED()` as
  syntax errors, and `AS_NEEDED` is the only construct in ld's `input_list1` grammar that nests
  inside itself. mold rejecting those is correct behavior, not the same bug class.
- **`get_script_output_type()` only ever inspects the *first* item of the group, and only an
  `OUTPUT_FORMAT`/`INPUT`/`GROUP` that is the very first token of the file.** Both limitations are
  pre-existing and unchanged by this patch. Checked, and *not* a divergence in the obvious case:
  `GROUP(AS_NEEDED(missing.so) main.o libfoo.so)` fails under both mold and GNU ld 2.46 (ld:
  `cannot find missing.so`), so the first-item-only shortcut is not visibly wrong there. Whether a
  script that leads with something else entirely (a `SECTIONS` block, a second `OUTPUT_FORMAT`
  line) is mis-inferred was not investigated - it is a different question from the nesting one.
- **`resolve_path()` being fatal on the inference path at all is arguably wrong.** This function's
  job is a best-effort peek; failing to identify the machine type could reasonably return `""` and
  let the caller fall through to its own `-m option is missing` diagnostic, rather than aborting the
  link with a misleading `library not found:`. Changing that is a maintainer's behavioral call, so
  the patch does not touch it.
- No fuzzing was run. This finding came entirely from code reading plus a hand-written differential
  sweep against GNU ld - 8 shapes in `repro/run.sh` and ~14 more in `repro/extra_checks.sh`, all
  hand-written, none generated. A differential fuzzer over linker-script text with mold and GNU ld
  as the two oracles would be a cheap and obvious way to find more of this class (the input is plain
  text, no ELF crafting needed), and is the natural next step for anyone wanting real coverage here.
