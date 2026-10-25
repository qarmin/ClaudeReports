# gstxmlhelper_datetime_missing_separator_oob_read

`gst-plugins-bad/ext/dash/gstxmlhelper.c` `gst_xml_helper_get_prop_dateTime()` parses an ISO-8601
`dateTime` DASH-MPD attribute (`availabilityStartTime`, `publishTime`, etc.) using repeated
`pos = strcspn(str, <separator>); str += (pos + 1);` steps (5 of them: year->`-`, month->`-`,
day->`T`, hour->`:`, minute->`:`) with no check that the separator was actually found. Flagged by
`findings.toml` (`OVF_15`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end repro: parse a minimal DASH-MPD-style XML document (`<MPD availabilityStartTime="2024">`) via the real `libxml2` `xmlReadMemory()`/`xmlGetProp()`, then call the real, unmodified `gst_xml_helper_get_prop_dateTime()` on the resulting `xmlNode`, under a scoped `clang -fsanitize=address,undefined` build | Scoped meson build (core+base+bad/dash) | No crash/finding, despite `g_printerr` tracing confirming the exact predicted pointer arithmetic occurred (`str` ends up exactly 24 bytes past the attribute-value string's start, matching `malloc_usable_size()` for that allocation as independently measured with a standalone probe). AddressSanitizer did not flag the resulting 1-byte read. |
| Same repro, same input, run under `valgrind` instead | Same build | **Reproduces**: `Invalid read of size 1` inside glibc's `sscanf`/`strlen`, called from `gst_xml_helper_get_prop_dateTime (gstxmlhelper.c:842)`, "0 bytes after a block of size 24 alloc'd" at `xmlStrndup` (libxml2's own attribute-value duplication), called from the same function at `gstxmlhelper.c:826`. |

## Status

**Confirmed** - reproduces 3/3 runs, identical valgrind report each time. (Note: this is a case
where valgrind caught a real bug that a scoped ASAN build did not - see "What's not done" below
for the likely reason and why valgrind's result is trusted as the primary evidence here rather
than treated as suspect for disagreeing with ASAN.)

**Fully reproducible**: **Yes**. The repro exercises the real, unmodified
`gst_xml_helper_get_prop_dateTime()` through a real `libxml2` XML parse of a crafted DASH-MPD-style
document and the real `xmlGetProp()` attribute-value extraction - no hand-constructed string
pointer stands in for what `libxml2` would actually hand back to a real MPD-parsing caller.

## Class

memory-safety (heap-buffer-overflow read)

## Root cause

`strcspn(str, sep)` returns the length of the initial segment of `str` that contains none of the
characters in `sep`. If `sep`'s character is genuinely present, `str[pos] == sep_char`; if it is
**absent**, `pos == strlen(str)` and `str[pos] == '\0'` - but the code does not distinguish these
two cases, unconditionally doing `str += (pos + 1)`. In the "absent" case this advances `str` to
exactly one byte past the string's own NUL terminator - i.e. one byte past the end of the
underlying allocation `libxml2` made for that attribute value (`xmlStrndup`, sized exactly to the
string's length + 1). The very next `sscanf(str, "%d", ...)` call then reads from that
out-of-bounds address (`sscanf`'s glibc implementation calls `strlen()` on its input first, which
is what actually touches the invalid byte). This is reachable via any DASH MPD manifest whose
`availabilityStartTime` (or any other dateTime-typed attribute parsed through this same function)
omits an expected `-`, `T`, or `:` separator - e.g. a manifest attribute value that is pure digits
with no punctuation, which is exactly what this repro constructs, driven through the real
`libxml2` parse + real `gst_xml_helper_get_prop_dateTime()` call.

## Introduced by

not determined - `git log -S`/`git blame` on this exact function only turn up this project's own
bulk `subprojects/` monorepo-merge commits (`86b251b7d1` "dash: split mpdparser, mpdclient and
xmlhelper", `019971a3c7` "Move files into the subprojects/... subdir"), both of which are pure
file-move/refactor commits, not the original authorship - the real history predates what's
reachable from this checkout (the DASH module's history originates from a separate
`gst-dash`/`gst-plugins-bad` history merged in before this repo's own tracked range starts
meaningfully diverging on this function). Not worth a `git bisect` given the soft time budget for
this best-effort lookup.

## The fix

Add `if (str[pos] == '\0') goto error;` immediately after each of the 5 `pos = strcspn(...)` calls,
before the corresponding `str += (pos + 1)` - if the separator wasn't found, the dateTime string is
malformed and parsing should fail cleanly instead of advancing past the buffer.

Verified: rebuilt with the fix and re-ran the same repro - valgrind now reports 0 errors, and the
function still correctly parses a genuinely well-formed dateTime string
(`"2024-01-15T10:30:00"` -> `gst_date_time_to_iso8601_string()` round-trips to
`"2024-01-15T10:30:00Z"`), confirming the fix doesn't regress valid input.

- Project test suite: `meson test -C build-dash-repro 'gst-plugins-bad:elements_dash_mpd'` -
  1/1 passed both before and after the fix.

## What's not done / open questions

- **Why ASAN missed this but valgrind caught it, precisely**: not fully root-caused. The most
  likely explanation, based on `xmlStrndup`'s call to plain `malloc()` (visible in valgrind's own
  allocation-site backtrace) sized exactly to the string's length, is some interaction between
  glibc's `sscanf`/`strlen` implementation's internal buffering/vectorized scanning and
  `AddressSanitizer`'s interceptors for those specific libc functions in this environment (glibc
  2.42-ish + clang's ASAN runtime) that doesn't trigger the redzone check the way a raw
  user-written loop would (compare the plain `malloc(24); p[24]` sanity check used earlier this
  session, which ASAN caught immediately). This is the same general "small-magnitude
  before/after-allocation read absorbed by something in the toolchain" theme as candidates
  #053/#057, but for a different specific reason (ASAN/libc interaction rather than an allocator
  header) - worth remembering that **valgrind should still be tried even after a clean ASAN run**
  for small (1-a few byte) out-of-bounds reads, they are not fully interchangeable tools.
- Only the "year" separator check (the first of 5) was driven to an actual crash; the other 4
  (month, day, hour, minute) have the exact same pattern and the exact same fix, but weren't each
  independently exercised with their own dedicated crafted input this session.
