# core-dictionary-tagrange-selector-parsing-safe

`core/src/dictionary/data_element.rs` has two string-slicing operations parsing untrusted,
user/caller-supplied text (`TagRange::from_str`'s `"(gggg,eeee)"` paren-stripping, and
`parse_selector`'s `"Tag[N].Tag2"` item-index extraction) that looked like classic
boundary/off-by-one candidates (`&s[1..s.len() - 1]`, `&part[split_i + 1..part.len() - 1]`) per
HEURISTICS.md's "index/slice arithmetic near a boundary" pattern - neither of these string parsers
had been checked by either bug-hunt session before this one.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value reasoning: for each slice, derived the minimum/maximum string length the preceding guard actually allows, and checked whether the slice's start/end indices can ever be inverted or out of bounds within that allowed range | Static code reading, no build needed | `TagRange::from_str`'s `s[1..s.len()-1]` is only reached after `s.starts_with('(') && s.ends_with(')')` both hold - since `'('` and `')'` are different characters, a 1-character string can satisfy at most one of these two conditions, never both; therefore any `s` reaching the slice has `len() >= 2`, making `1..len()-1` always valid (empty range at worst, for `s == "()"`). `parse_selector`'s `part[split_i+1..part.len()-1]` is only reached after `part.ends_with(']')` (so the last byte is always `']'`, never `'['`) and `split_i = part.find('[')` succeeded (returns `None`, safely handled via `.context(..)?`, if no `'['` exists) - since the last byte can never be `'['`, any found `'['` is at an index `<= len()-2`, making `split_i+1 <= len()-1` always hold, so the slice's start never exceeds its end |

## Status

**Not reproducible** - ruled out by proving, for each slice, that the preceding string-shape guard
structurally guarantees the slice's start index can never exceed its end index or the string's
length, for every input that reaches it.

## Class

n/a

## Root cause

Not applicable (no bug) - both slices are guarded by a `starts_with`/`ends_with` check on a
*different* character than what's being sliced away, which happens to make the slice length
implication airtight (a 2-character-minimum floor falls out of the two characters being distinct,
rather than being asserted directly) - a subtle but correct piece of reasoning in the original
code, not an accident.

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

- Only `TagRange::from_str` and `parse_selector` were checked in this file (508 lines total) -
  `core/src/dictionary/data_element.rs`'s other parsing entry points (`parse_tag`, referenced but
  not shown in the excerpts read) and the rest of the dictionary module (`stub.rs`, `mod.rs`) were
  not exhaustively re-read for unrelated issues.
