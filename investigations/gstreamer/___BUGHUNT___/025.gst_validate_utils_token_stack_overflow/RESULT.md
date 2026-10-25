# gst_validate_utils_token_stack_overflow

`subprojects/gst-devtools/validate/gst/validate/gst-validate-utils.c`'s expression parser
(`_read_double()` and `_read_builtin()`, both called from the public
`gst_validate_utils_parse_expression()`) copies digits/identifier characters one at a time into a
fixed `gchar token[PARSER_MAX_TOKEN_SIZE]` (256 bytes) stack buffer with no bounds check on the
write index `pos`. Flagged by a prior structured audit (`findings.toml` OVF_9).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + read-through of `_read_double`/`_read_builtin`/`_read_token`-style loops | Read the source directly; identified 8 unchecked `token[pos++] = _next(parser)` write sites across the two functions | Confirmed no bounds check exists anywhere on `pos` before a write, for either a numeric literal or an identifier token |
| Clang ASAN+UBSan debug build of the real `libgstvalidate-1.0.so`, standalone C harness calling the real public `gst_validate_utils_parse_expression()` with a crafted 4096-char expression | `meson setup --buildtype=debugoptimized -Db_sanitize=address,undefined -Db_lundef=false -Dc_args=-fno-sanitize=function` (clang 21.1.8), `-Ddevtools=enabled -Dgst-devtools:validate=enabled`, minimal core+base scope | Reproduced immediately: `AddressSanitizer: stack-buffer-overflow` in `_read_double` (4096-digit numeric literal) and separately in `_read_builtin` (4096-char identifier), 5/5 runs each |

Only 2 of the soft-cap-of-3 approaches were needed: the second approach (real ASAN build + real
public-API harness) produced an immediate, deterministic, unambiguous signal on the first attempt
for both code paths named in the candidate description, so no third tool was required.

## Status

**Confirmed**

Reproduces 5/5 runs for both the numeric-literal path (`_read_double`, line 112 of the pristine
file) and the identifier/builtin-name path (`_read_builtin`, line 369 of the pristine file),
under a clang ASAN+UBSan `debugoptimized` build. Each run aborts with a distinct
`AddressSanitizer: stack-buffer-overflow` diagnostic pinpointing the exact write and the exact
overflowed stack variable (`token`).

**Fully reproducible**: **Yes**. The repro calls the real, unmodified, exported
`gst_validate_utils_parse_expression()` from the actual built `libgstvalidate-1.0.so` (via
`pkg-config gstreamer-validate-1.0` against the project's own meson-uninstalled devenv, no
symbols stubbed or reimplemented). The finding is about a single participant (this function)
reacting to a single external input (the expression string) it is handed directly - there is no
second participant to fake here: a crafted `expr(...)` string is the entire external input, the
same way a malformed file handed to a parser's own public `open`/`parse` entry point would be
(see `CONFIRMATION.md` section 11's "single-process, single-participant repro remains completely
fine" carve-out). In the real project this string ultimately comes from a `.validatetest`
scenario file's `expr(...)` field, resolved via
`gst_validate_replace_variables_in_string()` -> `gst_validate_utils_parse_expression()`
(gst-validate-utils.c around line 1354) - the harness calls the same public function with the
same shape of input directly, skipping only the scenario-file-loading boilerplate around it.

## Class

memory-safety (stack buffer overflow / OOB write)

## Root cause

`_read_double()` (originally line ~100-140) and `_read_builtin()` (originally line ~359-410) each
declare `gchar token[PARSER_MAX_TOKEN_SIZE]` (256 bytes) and grow it one character at a time via
patterns like `token[pos++] = _next(parser);` inside `while (isdigit(...))` /
`while (isalpha(...) || isdigit(...) || c=='_' || c=='$')` loops, with `pos` never compared
against `PARSER_MAX_TOKEN_SIZE`. `_next()` only checks that `parser->pos` (the read cursor into
the *input* string) hasn't run past the end of the expression string - it enforces nothing about
`pos` (the *write* cursor into the local `token` buffer). Because `gst_validate_utils_parse_expression()`
strips all whitespace from the input before parsing (`g_strsplit`/`g_strjoinv` with `" "`/`""`) and
imposes no length limit of its own, any numeric literal or bare identifier longer than 255
characters inside the expression string overflows `token` by exactly `(length - 255)` bytes,
smashing adjacent stack frame contents (in this build: the caller's own local `token` buffer for
`_read_builtin`, since `_read_double` is called from inside `_read_builtin`'s stack frame via
`_read_parenthesis` -> `_read_unary` -> `_read_power` -> `_read_term` -> `_read_expr` -> `_parse`).

## Introduced by

a89f32725b "utils: Add util functions to parse simple mathematical expressions" (2013-09-28).
This commit added `gst-validate-utils.c`'s entire math-expression parser, including
`PARSER_MAX_TOKEN_SIZE`, `_read_double()`, and `_read_builtin()`, already with the unchecked
`token[pos++] = _next(parser)` pattern in both functions from the very first version - confirmed
via `git log -S "_read_double"` (single match) and reading that commit's own diff, which shows the
unbounded writes present in the original 467-line addition. The bug has been present, unchanged in
this respect, in every commit since.

## The fix

Added a single choke-point helper, `_token_push()`, that all 8 `token[pos++] = ...` write sites in
`_read_double()` and `_read_builtin()` now go through; it calls the parser's existing `_error()`
(a `longjmp`-based error path already used throughout this file for malformed input) once `pos`
would reach `PARSER_MAX_TOKEN_SIZE - 1` (reserving room for the trailing `'\0'`), instead of
writing out of bounds. This is a minimal, mechanical change - no new global state, no change to
any function signature, using the same error-reporting mechanism (`_error()` -> `*error` string
returned to the caller) the parser already uses for every other malformed-input case (e.g.
"Expected ')'!", "Unknown operation!").

Verified:
- Re-running both `repro/repro_numeric.c` (4096-digit literal) and `repro/repro_identifier.c`
  (4096-char identifier) against the patched, rebuilt `libgstvalidate-1.0.so`: 3/3 runs each, no
  crash, clean `error="Token is too long!"` returned instead (see `output.txt`, "AFTER" section).
- Varied the input within the same bug class per `CONFIRMATION.md` section 7: both the
  numeric-literal path (`_read_double`) and the separate identifier path (`_read_builtin`) were
  checked, not just the one that happened to be tried first - both are fixed by the same
  `_token_push()` choke point.
- `repro/repro_sanity.c` (4 ordinary expressions: `"1 + 2 * 3"`, `"min(4, 5) + max(1, 2)"`,
  `"-3.5e2 + 1"`, `"(1 + 2) * (3 - 4)"`) still parses to the correct numeric results after the fix,
  confirming the bounds check doesn't reject legitimate short tokens.
- Project test suite (`gst-devtools:validate_utilities`, `gst-devtools:validate_expression_parser`
  - the only existing checked-in tests that exercise this file): **2/2 passing before the patch,
  2/2 passing after the patch** (neither test happens to exercise a >255-character token, so
  neither one changes outcome - this is stated explicitly rather than implied to be broader
  coverage than it is).

## What's not done / open questions

- `PARSER_MAX_ARGUMENT_COUNT` (defined alongside `PARSER_MAX_TOKEN_SIZE` in the same file) was not
  investigated this session - it's a separate constant guarding argument-count, not token length,
  and wasn't part of this candidate's scope; worth a follow-up look if the same
  unchecked-fixed-buffer pattern appears there too.
- No other `PARSER_MAX_TOKEN_SIZE`-sized stack buffer exists elsewhere in this file (grepped for
  `PARSER_MAX_TOKEN_SIZE` across the whole file: only the two declarations inside `_read_double`
  and `_read_builtin`, both fixed here) - this class of bug does not appear to recur elsewhere in
  this specific file.
- Not checked: whether any *other* GstValidate code (`gst-validate-scenario.c`, etc.) has a
  similar fixed-size-stack-buffer-fed-by-untrusted-file pattern; this session's scope was limited
  to the specific functions named in the candidate description.
