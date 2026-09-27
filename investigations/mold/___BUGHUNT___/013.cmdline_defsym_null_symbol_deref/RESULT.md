# cmdline_defsym_null_symbol_deref

`parse_defsym_value` in `src/cmdline.cc` (~line 590-604) parses the value half of a
`--defsym=SYMBOL=VALUE` argument. Flagged by a prior static-audit pass as returning a
default-constructed `std::variant<Symbol<E> *, u64>` (whose first alternative,
`Symbol<E> *`, default-constructs to null) on a malformed value expression, with that null
later dereferenced when marking GC roots - a plausible "silent crash instead of diagnostic"
shape worth a real repro.

## Important environment note before the findings

This candidate's stated `base_commit` is `c340272e6376f9a8e5b5cd6f6634f925f6d54e5c`, but this
worktree's actual checked-out `HEAD` at the start of the session was `d3cf4a23` - several
commits *ahead* of the stated base, on a branch that (it turns out) already contains a real
upstream commit fixing this exact bug (`a129618d`, dated after `c340272e`). To test against the
scope this candidate was actually assigned, I did **not** modify this worktree's own checkout;
instead I made a separate local clone pinned to the literal `c340272e` commit
(`scratch/base_c340272e/`, not committed, deleted with the rest of this disposable worktree) and
did all building/reproduction/patching there. `patch.diff` in this folder is a diff against that
`c340272e` clone, matching the `base_commit`/`repo_url` recorded in `justfile`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `parse_defsym_value` + its caller (`--defsym` handling, ~line 913-918) and the GC-root-marking loop (~line 1709-1712, dereference at line 1745) in the current source at `c340272e` | Static read only | Confirmed the exact mechanism the audit described: the hex-parse branch (`s.starts_with("0x")`) does `return {};` (a default-constructed, i.e. null-`Symbol<E>*`-holding, variant) when `stoull`'s `nread` doesn't consume the whole string - e.g. trailing garbage after the hex digits. The decimal-digits branch and the symbol-name fallback (`get_symbol()`) can't produce this - `get_symbol()` always returns a real, non-null pointer (inserts into a symbol map if not already present). So only the malformed-hex path is actually reachable. |
| `git log -S'parse_defsym_value'` / `git show` on the fix | git history | Found the exact fix already applied upstream: commit `a129618d` "Report an error for a malformed hexadecimal --defsym value" (2026-09-01), whose commit message describes this precise bug ("returned a default-constructed variant on a bad hex string such as 0xz... dereferenced when marking GC roots"). Not an ancestor of the stated `c340272e` base (`c340272e` is 2026-08-13, earlier). |
| Build a pristine clone pinned to `c340272e` (`-DCMAKE_BUILD_TYPE=Debug`, default g++/gcc, `cmake --build -j4`), compile a one-line `int main(void){return 0;}` with `gcc -c`, run `mold -o /tmp/out --defsym=foo=0x12z valid.o` | Plain debug build, `-j4` | **Segfault, exit 139, 5/5 repeat runs.** `gdb -ex "set follow-fork-mode child" -ex "set detach-on-fork on" -ex run -ex bt` pinpoints the fault to `src/cmdline.cc:1745` (`sym->gc_root = true;`) inside `parse_nonpositional_args<X86_64>`, called from `mold_main` (`main.cc:338`) / `main` (`entry.cc:42`). Crash happens in the parent process during argument parsing, before mold's usual fork-to-child-for-linking step, so the standard `gdb`-attaches-parent gotcha noted in the project's own `README.md` doesn't apply here. |
| Sanity-check valid `--defsym` values (`0x1234`, `1234`, `main`) against the same unpatched build | same build | All succeed normally (warn about undefined `_start`, exit 0) - confirms the crash is specific to the malformed-hex path, not a general `--defsym` regression. |
| Apply the same one-line fix upstream used (`a129618d`: `Fatal(ctx) << "-defsym: not a number: " << s;` instead of `return {};`), rebuild (`cmdline.cc` only, fast incremental build), re-run the exact repro | Same build dir, patched | No crash: `mold: fatal: -defsym: not a number: 0x12z`, exit 1. Re-ran 5x (see `output.txt`) plus a second malformed variant (`0xZZ`, no valid hex digits at all) - also a clean diagnostic, exit 1. Re-ran the three valid-value sanity checks - unaffected, still exit 0. |
| `ctest --test-dir build-plain -j4` (476 native x86_64 tests, ~13 skipped) before and after the patch | Full suite | **476/476 pass, both before and after** (no existing test exercises this crash path, and the fix doesn't regress anything). First attempt at capturing this accidentally wrote to a generic `/tmp/ctest_after.log` and got clobbered by a concurrently-running sibling candidate in another worktree (same host, shared `/tmp`) - re-ran into worktree-private log paths and got consistent, trustworthy 476/476 numbers both times; see the note in `output.txt`. |

## Status

**Confirmed.** Reproduces 5/5 runs (fully deterministic, no ASLR/layout dependency - the null
pointer is a compile-time-fixed default value, not a heap-layout-dependent address) against a
plain debug build at the stated `base_commit`. Also independently confirmed via a real `gdb`
backtrace pinpointing the exact null-dereference line, and via the standalone `repro/run.sh`
script re-run against a freshly rebuilt binary both before and after the fix.

**Fully reproducible**: **Yes.** The repro runs the real `mold` binary's actual CLI entry point
end-to-end (`gcc -c` a trivial valid object, then `mold -o ... --defsym=foo=0x12z valid.o`) - no
hand-constructed internal state, no crafted ELF needed at all. This is a single-participant
finding (a CLI argument string is the entire external input; there is no second participant
whose reaction is being faked), so `CONFIRMATION.md` section 11's two-participant bar does not
apply.

## Class

crash (memory-safety: null pointer dereference)

## Root cause

`parse_defsym_value<E>(Context<E>&, std::string_view s)` in `src/cmdline.cc` parses the
right-hand side of `--defsym=SYMBOL=VALUE`. When `s` starts with `0x`/`0X`, it calls
`std::stoull(std::string(s), &nread, 16)` and checks `s.size() != nread` to detect trailing
garbage after the hex number (e.g. `"0x12z"` - `stoull` parses `"0x12"`, sets `nread = 4`,
leaving `s.size() = 5 != nread`). On that mismatch the function does `return {};` - a
default-constructed `std::variant<Symbol<E> *, u64>`. Because `Symbol<E> *` is the variant's
*first* alternative, a default-constructed variant holds that alternative default-constructed,
i.e. a **null `Symbol<E> *`**, not any kind of error marker - the type system gives no static
signal that this differs from a legitimately-parsed null-valued anything.

That value flows: `parse_nonpositional_args`'s `--defsym` handling stores it unexamined into
`ctx.arg.defsyms` (`arg.emplace_back(get_symbol(ctx, name), parse_defsym_value(ctx, value))`).
Later in the same function (~line 1710-1712), a loop copies every `Symbol<E> *` alternative out
of `ctx.arg.defsyms` into `ctx.arg.undefined` via `std::get_if<Symbol<E> *>(&val)` - this
correctly narrows to the `Symbol<E> *` alternative (the malformed-hex case *did* produce that
alternative, just with a null value inside it, so the `get_if` succeeds and copies the null
pointer straight through). A few lines later (line 1745), an unconditional loop over
`ctx.arg.undefined` does `sym->gc_root = true;` for every entry to mark GC roots - dereferencing
the null pointer, SIGSEGV. The actual syntax error (`0x12z` is not a valid hex number) is
silently swallowed instead of ever reaching a diagnostic.

## Introduced by

dc9573989cdca595b6ce24d21fd2c654ac5528fb "Refactor" (2022-03-01), which introduced
`parse_defsym_value` (in `elf/cmdline.cc` at the time, before the later directory-reorganization
commit that moved it to `src/cmdline.cc`) with `return {};` on this exact malformed-hex path from
the very first version of the function - confirmed via `git log -S'parse_defsym_value' --all`
and reading that commit's diff directly.

Separately, and worth recording precisely: this exact bug was **already found and fixed
upstream**, in commit a129618d11ada3723f87e5133319e988039cebb8 "Report an error for a malformed
hexadecimal --defsym value" (2026-09-01, authored by Rui Ueyama with a `Co-Authored-By: Claude
Fable 5` trailer suggesting AI-assisted discovery, possibly from an earlier run of this same
skill or a similar audit). That commit is not an ancestor of the stated `base_commit` (`c340272e`,
2026-08-13, i.e. three weeks earlier) - so the bug was live at the assigned base_commit and is
fixed only in later upstream history, which happens to already be present on this specific
worktree's own stray `HEAD` (see the environment note above).

## The fix

Reproduced the identical one-line change already applied upstream (`a129618d`), independently
derived by reading the malformed-hex branch and reasoning about the swallowed error path before
finding that commit:

```diff
     u64 addr = std::stoull(std::string(s), &nread, 16);
     if (s.size() != nread)
-      return {};
+      Fatal(ctx) << "-defsym: not a number: " << s;
     return addr;
```

`ctx` is already in scope at this exact point (it's the function's first parameter), so no
propagation via `std::optional`/similar was needed - the smallest possible change, matching how
every other parse error in this same function (`Fatal(ctx) << "-defsym: syntax error: ..."` at
the call site, and further up in `parse_section_order`) already reports errors.

Verified: re-ran `repro/run.sh` against the rebuilt patched binary - malformed-hex input now
prints `mold: fatal: -defsym: not a number: 0x12z` and exits 1 (no crash), 5/5 repeat runs. Also
checked a second malformed variant in the same class (`0xZZ`, no valid hex digits at all
following the prefix) - same clean diagnostic, not just the one exact triggering string. All
three valid-value forms (`0x1234` hex, `1234` decimal, `main` symbol-name) still work unchanged
after the fix.

- Project test suite: 476/476 passing before -> 476/476 passing after (no new failures, no
  existing test exercised this crash path either way).

## What's not done / open questions

- Did not check whether any other malformed-value shape can reach the same or a similar null-
  pointer path through a different route (e.g. is there any way for `get_symbol()` itself to
  ever return null under some other code path not visited here) - reading `get_symbol()`
  (`src/input-files.cc:26-39`) shows it unconditionally inserts-or-finds and returns a real
  pointer, so this looks closed, but wasn't independently fuzzed.
- Did not attempt a longer fuzzing campaign against `--defsym`'s value grammar generally (a
  handful of hand-picked boundary strings were tried: `0x12z`, `0xZZ`, `0x1234`, `1234`, `main`) -
  a `fuzz-bug-repro`-style short harness feeding `parse_defsym_value` directly could sweep more
  of the grammar cheaply if a future session wants extra assurance, though the function's logic
  is simple enough (3 branches, one already shown buggy and now fixed) that this seems low-value.
- Since the real fix is already merged upstream past this candidate's assigned `base_commit`,
  `patch.diff` here is redundant with `a129618d` rather than a novel contribution - it's included
  anyway per this skill's own rule (every `Confirmed` candidate with a bounded fix gets a real,
  verified `patch.diff`), and because the assigned `base_commit` genuinely doesn't have the fix
  yet, so from that snapshot's point of view this is a real, applicable patch.
