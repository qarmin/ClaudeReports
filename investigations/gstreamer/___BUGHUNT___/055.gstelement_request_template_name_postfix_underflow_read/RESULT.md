# gstelement_request_template_name_postfix_underflow_read

`subprojects/gstreamer/gst/gstelement.c` `gst_element_is_valid_request_template_name()`'s
postfix-matching computes `name_ptr + name_postfix_len - templ_postfix_len` (both `guint`) and
passes the result to `strncmp()`. Flagged by `findings.toml` (`OVF_26`) as a case where, if the
requested pad name's postfix segment is shorter than the template's literal postfix, the unsigned
subtraction wraps around to a huge value, producing a "wildly out-of-bounds pointer."

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Standalone C program: a minimal custom `GstElement` subclass (registered via plain `g_type_register_static`, no plugin needed) with a request pad template `"src_%uXYZ"` (a `%u` specifier immediately followed by 3 literal chars, no separating `_`), requesting pad name `"src_5"` (a 1-char postfix, shorter than the template's 3-char postfix) via the public `gst_element_request_pad_simple()`, run under `valgrind` | Core-only meson build, plain debug | No crash, no valgrind finding. Reading the actual C expression closely: `name_ptr + name_postfix_len - templ_postfix_len` is **not** a single unsigned subtraction reinterpreted as a pointer offset - `+`/`-` are left-associative at the same precedence, so it's evaluated as two sequential pointer operations, `(name_ptr + name_postfix_len) - templ_postfix_len`. Each individual operation uses `name_postfix_len`/`templ_postfix_len`'s *actual* small numeric values (1 and 3 in this test), not a pre-computed wrapped `guint`, so the net result is simply `name_ptr - 2` - a small, nearby (and in this specific test, still in-bounds) pointer, not a multi-gigabyte wild pointer. |
| Same harness, refined: template postfix widened to 64 literal chars (`templ_postfix_len = 64`), requested name's postfix kept at 1 char (`"5"`), and the requested `name` string switched to a `g_strdup()`'d heap allocation (6 bytes) so valgrind's precise heap-redzone tracking applies | Same build | **Reproduces**: valgrind reports "Invalid read of size 1" in `strncmp` at `gst_element_is_valid_request_template_name (gstelement.c:1060)`, called from `gst_element_request_pad_simple (gstelement.c:1236)`, on an address "15 bytes after a block of size 40 free'd" (an unrelated, already-freed heap allocation adjacent to the 6-byte `name` string) - a genuine heap-buffer-underflow read, `templ_postfix_len` (64) bytes reading backward from just past the `name` allocation. |

## Status

**Confirmed** - reproduces 3/3 runs, identical valgrind finding each time.

**Fully reproducible**: **Yes**. The repro drives the real, unmodified
`gst_element_is_valid_request_template_name()`/`gst_element_request_pad_simple()` purely through
the public `gst_element_request_pad_simple()` API against a real (if minimal, custom-registered)
`GstElement` subclass and a real pad template - the vulnerable comparison logic itself is entirely
untouched/real.

## Class

memory-safety (heap-buffer-underflow read)

## Root cause

**Correcting `findings.toml`'s stated mechanism**: this is *not* an unsigned-integer-wraparound-
turned-huge-pointer-offset bug (a single `guint` subtraction producing e.g. `0xFFFFFFFE`, then
added to a pointer as a ~4GB offset) - the C expression `name_ptr + name_postfix_len -
templ_postfix_len` is two sequential, individually well-defined pointer arithmetic operations due
to left-to-right operator associativity, not one combined unsigned computation. The real,
narrower bug is still genuine and still memory-unsafe: when the requested pad name's postfix
segment (`name_postfix_len`, e.g. `strlen("5")` = 1) is shorter than the request template's
literal postfix (`templ_postfix_len`, e.g. `strlen("XXXX...")` = 64), the resulting pointer
(`name_ptr + name_postfix_len - templ_postfix_len`) lands up to `templ_postfix_len` bytes *before*
the start of the caller-supplied `name` string, and `strncmp()` then reads `templ_postfix_len`
bytes starting from there - a heap-buffer-underflow read whose maximum backward distance is
controlled by the **pad template's own literal postfix length** (something the element
implementation, not an external attacker, controls) but whose *trigger* (a caller/application
requesting a pad name with a too-short postfix) is fully public-API-reachable and outside the
element's control. Practically, the blast radius is bounded by however long a literal postfix a
real pad template happens to declare (real GStreamer templates rarely go far past `"_%u"` with a
short suffix), which is presumably why this has gone unnoticed in thirteen years of use - but it
is a real out-of-bounds read regardless of typical severity, and the earlier `name_postfix_len <
templ_postfix_len` case is never checked before the subtraction.

## Introduced by

f80dfc9b06d7affe54945cb1486b1c1521f1cb95 "element: Allow multiple conversion specifiers for
request pads" (2016-04-29) - this rewrote `gst_element_is_valid_request_template_name()` to
support multiple `%u`/`%d` specifiers and introduced the postfix-length comparison/`strncmp` call
that lacks the length guard.

## The fix

Add `if (name_postfix_len < templ_postfix_len) return FALSE;` immediately before the `strncmp()`
call - if the requested name's postfix is shorter than the template's required literal postfix, it
can never match, so reject it up front instead of computing an out-of-bounds pointer.

Verified: rebuilt with the fix and re-ran the same repro - valgrind now reports 0 errors, and
`gst_element_request_pad_simple()` still correctly returns `NULL` for the same mismatched request
(behavior is unchanged for every input that previously worked correctly; only the specific
out-of-bounds-before-computing-the-answer case is now short-circuited safely).

- Project test suite: `meson test -C build-core-repro gstreamer:gst_gstelement
  gstreamer:gst_gstelementfactory gstreamer:gst_gstghostpad gstreamer:gst_gstpad` - 4/4 passed
  both before and after the fix.

## What's not done / open questions

- Only the `next_specifier == FALSE` (single/last-specifier) branch of the postfix-length
  computation was exercised; the `next_specifier == TRUE` branch (multiple `%u`/`%d` specifiers
  separated by `_`) computes `templ_postfix_len`/`name_postfix_len` slightly differently
  (`templ_name - (templ_name_ptr + 2)` / `name - name_ptr`) but funnels into the exact same
  unguarded `strncmp` call two lines later, so the same fix covers it - not independently
  re-verified with its own dedicated repro this session.
