# encode-element-as-text-date-value-under-ds-is-vr-unreachable

Found via a fresh `grep -rn "unreachable!()" --include='*.rs'` sweep of the whole tracked
workspace this session (the prior two sessions' 23-site inventory was fully exhausted, but new
commits have added new sites since - `SKILL.md`'s own guidance to re-run this sweep fresh each
session). `parser/src/stateful/encode.rs:414`'s `encode_element_as_text` has a bare `unreachable!()`
covering `PrimitiveValue::Date`/`DateTime`/`Time`/`Tags`/`Strs`/`Str`, matching this project's own
recurring "a `DataElement`'s VR and its stored value are never kept in sync by the type system"
bug class (candidates 025/026/092/093/094/095).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading: traced `encode_element_as_text`'s only call site (`encode_primitive_element`'s fallback branch, reached when `de.vr` is `DS`/`IS` and `value` is not already `Str`/`Strs` - those are intercepted earlier in `encode_primitive_element`'s own top-level match) | n/a | Confirmed `Str`/`Strs` genuinely cannot reach the inner `unreachable!()` given the current single call site (they're always intercepted first), but `Date`/`DateTime`/`Time`/`Tags` values reaching here are only prevented by *convention* (real callers "shouldn't" pair `VR::DS`/`VR::IS` with those variants), not by the type system - nothing in the public `DataElement`/`InMemDicomObject` API stops a caller from constructing exactly that mismatch |
| Standalone repro building a real `InMemDicomObject` (via the public `DataElement::new`/`put_element` API) with one element declared `VR::DS` but holding a `PrimitiveValue::Date`, written via the real public `write_all` entry point | debug build, 3 repeat runs, against unpatched `df2b9e42` | Reproduced 3/3: `thread 'main' panicked at .../encode.rs:414:41: internal error: entered unreachable code` - exact match to the suspected line |

No further tools were needed - this is the same well-established bug class as five prior
candidates in this project; a single targeted repro was sufficient to confirm it.

## Status

**Confirmed** - deterministic, pure logic/panic bug with no layout/seed/timing dependency:
reproduces 3/3 runs.

**Fully reproducible**: Yes. The repro drives the real, public `write_all` entry point on a real
`InMemDicomObject` built entirely through the crate's own public element-insertion API - the same
path any real caller (a program constructing/editing a DICOM object before writing it) would use.
There is no second "peer" process here: the untrusted input is the object's own in-memory state,
constructed through the exact same public API a real caller would use to (mistakenly or not) create
a VR/value mismatch - not a hand-constructed stand-in for another process's behavior.

## Class

crash (a bare `unreachable!()` panic reachable from ordinary use of the public `write_all`/dataset-
writing API on an object whose VR/value pairing the type system never enforced to be consistent).

## Root cause

`encode_primitive_element`'s top-level dispatch intercepts `PrimitiveValue::Str`/`Strs` first
(routing them to `encode_text_element`/`encode_texts_element`), then, in its fallback arm, checks
`if let VR::DS | VR::IS = de.vr` and calls `encode_element_as_text(value, de)` for the DS/IS special
case (DS/IS values are sometimes stored as numeric `PrimitiveValue` variants and need textual
re-rendering). By the time control reaches `encode_element_as_text`, `value` can only be a
*non*-`Str`/`Strs` variant - but `encode_element_as_text`'s own `match value` still lists `Str`/
`Strs` alongside `Date`/`DateTime`/`Time`/`Tags` in its final catch-all arm, which is `unreachable!()`.
`Str`/`Strs` genuinely cannot reach it (dead code, but harmless), while `Date`/`DateTime`/`Time`/
`Tags` values absolutely can: nothing in the public API prevents constructing a `DataElement` with
`VR::DS`/`VR::IS` and one of those value variants (the crate does not, and by design cannot, keep a
`DataElement`'s declared VR and its stored `PrimitiveValue` variant in sync once the object is held
in memory and editable via the public API - the same root cause already documented for candidates
025/026/092/093/094/095).

## Introduced by

123f257b "[parser] force encoding DS and IS values as text" (2022-11-22) - the commit that
introduced `encode_element_as_text` and its own `unreachable!()` catch-all arm, evidently written
under the assumption that only `Str`/`Strs`/numeric variants would ever reach this function (true
for `Str`/`Strs`, never actually checked for `Date`/`DateTime`/`Time`/`Tags`).

## The fix

Added a `VrValueMismatch` error variant to `parser::stateful::encode::Error` (tag, VR, and a clone
of the offending value, for a self-explanatory error message), and replaced the `unreachable!()`
catch-all arm with `return VrValueMismatchSnafu { tag: de.tag, vr: de.vr, value: value.clone() }.fail()`.
This makes the encoder return a graceful, descriptive error instead of panicking for exactly the
inputs that can actually reach this arm (`Date`/`DateTime`/`Time`/`Tags`), while also covering the
technically-unreachable `Str`/`Strs` cases defensively at no cost, in case a future refactor of the
caller's dispatch order ever changes that guarantee.

This candidate's own `VrValueMismatch` variant was derived and named independently to match this
project's own established convention from candidate 092 (which added an identically-named variant
for a different, sibling gap in the same function's outer dispatch) - since neither candidate's
patch is ever actually applied to the real tree between sessions (per `SKILL.md`'s "always revert"
rule), both exist only in their own isolated `patch.diff`; if either is ever merged first, the other
would need its own `VrValueMismatch` variant reconciled against whichever one landed, rather than
both being added side by side.

Verified against the repro: before the fix, the write call panics (3/3 runs); after the fix, it
returns `Err("Could not print data set")` (wrapping the new `VrValueMismatch` error) with no panic
- see `output.txt` for the full before/after capture.

- Project test suite: `cargo test -p dicom-parser -p dicom-object` - 75+9+49+22+0 passed, 0 failed
  across all reported groups, before AND after (identical counts, zero regressions - no existing
  test constructs this exact VR/value mismatch). Full workspace `cargo test --workspace --exclude
  dcms` (dcms excluded: untracked scratch crate belonging to a different session, not part of this
  workspace's tracked tree) - 0 failures anywhere before or after.

## What's not done / open questions

- This fix only closes the gap for `VR::DS`/`VR::IS` paired with `Date`/`DateTime`/`Time`/`Tags`
  (the one function this candidate targets). Candidates 092/093/094/095 already covered several
  other VR/value mismatch shapes in the same broader dispatch (`encode_primitive_element`'s outer
  match, and `dicom_encoding`'s `BasicEncode::encode_primitive` default method) - this candidate
  does not re-audit those, only the one `unreachable!()` site the fresh sweep flagged.
- The fresh `unreachable!()` sweep that found this (67 total tracked-file hits minus this session's
  own repro/tooling false-positives, ~31 in real source files) surfaced roughly 20 other sites not
  yet individually checked this session (`core/src/header.rs:94`, `encoding/src/adapters.rs`'s 8
  occurrences, `encoding/src/transfer_syntax/mod.rs`'s 2, `object/src/mem.rs`'s 4,
  `object/src/meta.rs`'s 4 - likely the already-known `FileMetaAttribute` sites from candidates
  023/024, not re-verified line-for-line this session -, `parser/src/dataset/mod.rs`'s 5,
  `findscu/src/main.rs`'s 1 CLI-argument-driven one) - left in `IDEAS.md`'s backlog for a future
  session/discovery round rather than exhausted in this one.
