# `PluralRules::_cache_plural_tests()` recurses forever (stack overflow) on a plural rule with `?` but no `:`

Found by the GDScript text-format fuzz loop (`.po` seed, `ResourceLoader.load()`), ASan editor build, seed 1.
Crashing input: a `Plural-Forms` header with a mutated rule (`... ?2);`, the `:` removed).

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of text formats (json, xml, ConfigFile, Expression, RegEx, .tres, .tscn, shader, GDScript, str_to_var, BBCode, csv) | ASan editor build `b8420aa3a6`, seed 1, 800-1500 iterations each | all clean |
| Same loop on `.po` | seed 1 | SIGSEGV (stack overflow, no ASan report: gdb shows ~thousands of nested `PluralRules::_cache_plural_tests`) |
| Minimal repro (`repro/project/repro.gd`): `.po` with `plural=n ? 1;` | unfixed build | SIGSEGV, 10/10 |
| Fix verification | `patch.diff`, rebuilt | repro loads fine, 10/10; variants `?`, `:`, `n : 1 ? 2`, `n ? : 1`, `(n ? 1)`, `n ? 1 ? 2 : 3` do not crash; real rules (English, Arabic-style chain, Polish) still select the right plural forms (Polish n=0,1,2,5,21 -> forms 2,0,1,2,0) |

## Status

**Confirmed.** Deterministic, 10/10 crashes before, 0/10 after.

**Fully reproducible**: Yes - real `ResourceLoader.load()` of a `.po` file (the same path as the editor/project translation import).

## Class

`crash` (unbounded recursion -> stack overflow from a malformed translation file)

## Root cause

`core/string/plural_rules.cpp`, `_cache_plural_tests()`:

```cpp
int first_ques_mark = _find_unquoted(rule, '?');
int first_colon = _find_unquoted(rule, ':');
if (first_ques_mark == -1) { /* leaf */ return; }
...
_cache_plural_tests(rule.substr(first_ques_mark + 1, first_colon - first_ques_mark - 1)..., p_node->left);
_cache_plural_tests(rule.substr(first_colon + 1)..., p_node->right);
```

Only a missing `?` is handled. If a `?` exists but there is no `:` (`first_colon == -1`), the right-hand
recursion is `rule.substr(0)` - the whole rule again - so it recurses with identical input forever. A `:`
located before the `?` gives a negative length / bogus ranges as well.

## Introduced by

ebb96e2303 Move plural rules logic into a separate class

## The fix

`patch.diff` (+1/-1): treat the rule as a leaf when `first_colon < first_ques_mark` (covers `-1`). The malformed
rule then simply fails to evaluate like any other invalid expression instead of recursing. Applied to the main
tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone (full ASan scons build per variant); `repro/run.sh` run against the
unfixed and fixed binaries built from the main tree. Nested ternaries are split at the *first* `:`
(`a ? b ? c : d : e` is mis-parsed) - existing precedence limitation, not changed or investigated.
