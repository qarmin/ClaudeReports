# collector-source-parser-unreachable-safe

`object/src/collector.rs:468`, inside `CollectionSource::set_parser_with_ts`, reassigns
`*self = CollectionSource::Parser(...)` and then immediately re-matches `self` with
`let CollectionSource::Parser(parser) = self else { unreachable!(); };` to get a mutable
reference into the just-constructed variant. Flagged as an unchecked `unreachable!()` site left
over from the prior session's sweep (`projects/dicom-rs.md`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: check for any re-entrancy/mutation between the assignment and the re-match | Static code reading, no build needed | The assignment (`*self = CollectionSource::Parser(...)`) and the `let ... else { unreachable!() }` re-match are two directly sequential statements on the same `&mut self` with no intervening call, no `await` point (the crate is synchronous), and no way for another reference to `self` to exist simultaneously (Rust's aliasing rules forbid it - `self` is `&mut`, exclusively borrowed for this whole function). There is no path by which `self` could hold anything other than the `Parser` variant immediately after being set to it |

## Status

**Not reproducible** - ruled out by reading; this is a same-scope, single-threaded, no-reentrancy
TOCTOU-shaped pattern where the "time of check" and "time of use" are actually the same
statement's-worth of exclusive-borrow-guaranteed state, not a real gap.

## Class

n/a

## Root cause

Not applicable (no bug) - Rust's `&mut` exclusivity guarantee makes this pattern (assign a known
variant, then immediately re-destructure to get a typed handle into it) sound by construction;
there is no synchronous or asynchronous code that could run between the two statements and change
`*self` again.

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

None - the surrounding function is small and fully read (`object/src/collector.rs:441-473`).
