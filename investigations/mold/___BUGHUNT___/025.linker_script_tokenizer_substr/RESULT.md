# linker_script_tokenizer_substr

`Script<E>::tokenize()` in `src/linker-script.cc` (~lines 47-94) was flagged by a grep sweep for
single-argument `.substr(pos)` calls, the same throwing-overload pattern (`std::out_of_range` when
`pos > size()`) already confirmed buggy elsewhere in this project (candidates 008/014/017/022/023).
Two call sites were flagged specifically: the quoted-string branch (line 78) and the general-token
branch (line 92). Not yet independently verified before this candidate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the current `tokenize()` function end-to-end (lines 47-94) against the exact current source, not just the audit's paraphrase | code reading only, no build | Confirmed the audit's description matches current source line-for-line (modulo the line numbers shifting slightly: quote branch is lines 73-80, general-token branch is lines 82-92) |
| Traced both flagged `pos` values back to their producing expression and the surrounding loop invariant | code reading only | Both are provably bounded by `input.size()` before the `substr(pos)` call runs - see Root cause below |
| Checked the other two `substr(pos+n)` sites in the same function (`/*...*/` comment close, `#` line-comment skip) for the same pattern, since they weren't explicitly named by the audit | code reading only | Also safe by the identical `find()`-returned-and-checked-index construction; no new lead |
| Checked whether `Fatal(ctx)` (invoked via `error()`) actually terminates the process rather than returning, since the "unclosed string literal"/"unclosed comment" branches call `error()` then fall through to the next line in the source text | read `src/mold.h:120-133`, `Fatal<E>::~Fatal()` declared `[[noreturn]]` | Confirmed `error()` never returns - the `pos == npos` branches are dead ends, not live fallthrough paths, so no bug hides there either |
| Traced reachability of the tokenizer from archive `INPUT()`/`GROUP()` content, per the task's specific ask | read `src/main.cc` `read_file()` (lines 106-132), `read_archive_member()` (lines 80-98), `read_input_files()` (lines ~230-280) | `read_file()`'s top-level dispatch does route `FileType::TEXT` to `Script(...).parse_linker_script()` (line 123, 279) - so a file named inside a `GROUP()`/`INPUT()` directive of an already-parsed script, or any top-level file argument, that the file-type sniffer classifies as `TEXT` does reach the tokenizer. But **archive members specifically do not**: `read_archive_member()` (used for every member of a `.a`/thin archive, both the eager top-level-archive path and the parallel `ReaderJob` path) has a `default: break;` for `FileType::TEXT`, silently ignoring a text/linker-script-shaped archive member instead of tokenizing it. This corrects the task's stated hypothesis - the tokenizer is reachable from a file path (directly on the command line, or named via `GROUP()`/`INPUT()` inside another already-open script) but **not** by smuggling a linker-script-shaped member inside an untrusted `.a` archive. |

No build or crafted-input repro was constructed - the code reading below establishes the two
flagged call sites are safe by construction, so there is no crash to reproduce.

## Status

**Not reproducible.** Both flagged `substr(pos)` calls in `Script<E>::tokenize()` are proven safe
by static invariants that hold on every path through the function; no build/instrumentation/fuzzing
was needed to reach this conclusion, and none would change it since the argument is exhaustive over
all inputs, not empirical over a sample.

**Fully reproducible**: N/A (no repro built; nothing to reproduce).

## Root cause (why this is safe, not a bug)

The whole function body is wrapped in `while (!input.empty())` (line 51), so every branch inside the
loop - including both flagged call sites - executes only when `input.size() >= 1`.

**Call site 1 - quoted string (current lines 73-80):**
```cpp
if (input[0] == '"') {
  i64 pos = input.find('"', 1);
  if (pos == std::string_view::npos)
    error(input, "unclosed string literal");   // [[noreturn]] Fatal(), never returns (see below)
  tokens.push_back(input.substr(0, pos + 1));
  input = input.substr(pos + 1);
  continue;
}
```
`std::string_view::find` either returns `npos` (handled by the `error()` call, which is
`[[noreturn]]` - see the `Fatal<E>::~Fatal()` declaration at `src/mold.h:124` - so execution never
falls through to the `substr` calls on that path) or returns a valid index `< input.size()`. In the
latter case `pos + 1 <= input.size()`, and `substr(pos)` only throws when `pos > size()` (not when
`pos == size()`), so both `substr(0, pos+1)` and `substr(pos+1)` are always in-bounds.

**Call site 2 - general token (current lines 82-92):**
```cpp
i64 pos = input.find_first_not_of("...allowed charset...");

if (pos == 0)
  pos = 1;
else if (pos == input.npos)
  pos = input.size();

tokens.push_back(input.substr(0, pos));
input = input.substr(pos);
```
Three cases, all bounded:
- `pos == 0` -> forced to `1`. Since `input.size() >= 1` (loop invariant), `1 <= input.size()`.
- `pos == npos` (every remaining character is in the allowed set) -> forced to `input.size()`.
  `substr(size())` is legal (produces an empty view), not an error.
- otherwise, `find_first_not_of` returned a genuine index that is neither `0` nor `npos`, so
  `0 < pos < input.size()` by definition of what `find_first_not_of` returns.

In every case `pos <= input.size()` holds before the `substr(pos)` call, so neither call site can
throw `std::out_of_range`. This is the same safe pattern the task description already anticipated;
code reading confirms it holds in the real current source with no gap.

As a side check, the two other single-argument `substr(pos+n)` calls in the same function (the
`/* ... */` comment-close at line 61, and the `#`-line-comment skip at line 69) follow the identical
"index came from a checked `find()` result" shape and are safe for the same reason - not just the
two the audit flagged.

## Introduced by

N/A - not applicable to a Not reproducible result (no bug to attribute).

## The fix

Not applicable - no bug found, no patch produced.

## What's not done / open questions

- No build, ASAN run, or crafted-input repro was performed for this candidate - the safety argument
  above is exhaustive over the domain of `input` (any `std::string_view` with `size() >= 1`
  entering these branches), so a dynamic test would not add information beyond what the static
  argument already establishes. If a future session doubts this reasoning, the fastest way to
  double-check is a small standalone C++ unit exercising `tokenize()` directly (bypassing `mold`'s
  file-type sniffing) with adversarial strings (all-quote, single-char, all-allowed-charset,
  all-disallowed-charset) rather than crafting an ELF/archive.
- Confirmed (see the reachability row in the table above) that `Script<E>::tokenize()` is reachable
  from a file argument directly on the command line, or from a file path named inside another
  already-open linker script's `GROUP()`/`INPUT()` directive, if `get_file_type()` classifies that
  file as `FileType::TEXT` - but **not** from a text/linker-script-shaped member embedded inside an
  untrusted `.a`/thin archive (`read_archive_member()` silently ignores `FileType::TEXT` members,
  `src/main.cc:95-96`). This narrows (but given the safety proof above, does not change the
  conclusion for) what "untrusted input" actually means for this specific function - worth noting so
  a future session doesn't need to re-derive this reachability fact from scratch.
- Also checked `unquote()` (line 106, `s.substr(1, s.size() - 2)` guarded only by
  `assert(s[s.size() - 1] == '"')`) as a possible sibling issue, since every one of its 5 call sites
  (lines 137, 229-230, 312-315, 390) passes a `tokens[]` entry produced exclusively by `tokenize()`.
  This is also safe: the only branch of `tokenize()` that can ever produce a token whose first
  character is `"` is the quoted-string branch itself (the general-token branch's allowed charset at
  lines 83-84 excludes `"`, and any input starting with `"` is intercepted by the dedicated branch
  at line 73 before the general-token branch ever runs), and that branch's token is always
  `input.substr(0, pos + 1)` with `pos = input.find('"', 1) >= 1`, i.e. size always `>= 2`. So
  `unquote()`'s `assert` can never actually fire and `s.size() - 2` can never underflow, even in a
  release build with assertions compiled out. Not a lead for a future session.
- This file's tokenizer (and `unquote()`, its only direct consumer of raw token boundaries) looks
  fully hardened against the single-arg-`substr` bug class. A future session looking at
  `linker-script.cc` for new leads should look further up the parser instead - `parse_linker_script()`,
  `read_group()`, `read_output_format()`, and the various `tok[N]`-indexing call sites in
  `Script<E>::parse_*` for possible out-of-range `tok` indexing (a different bug shape - vector/span
  indexing, not `substr` - not investigated this session).
