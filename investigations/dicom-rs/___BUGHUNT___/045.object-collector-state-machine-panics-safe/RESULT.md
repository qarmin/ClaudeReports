# object-collector-state-machine-panics-safe

`object/src/collector.rs`'s `DicomCollector` (a streaming, step-by-step DICOM file reader with
public methods like `read_preamble`/`read_file_meta`/`read_dataset_to_end`/`read_next_fragment`,
meant to be called in a specific order) has an internal `CollectionSource` enum with two
`panic!()`/`unreachable!()` sites reachable only when the collector is used out of the order its
private state machine expects: `raw_reader_mut()` panics if called after the reader has already
transitioned to a `Parser`, and `parser()` panics if called before a parser has been set up
(`"parser transfer syntax not set"`). Since candidate `030` (2026-08-25 session) only checked one
of this file's `unreachable!()` sites (the `set_parser_with_ts` internal re-match, ruled safe by
`&mut` exclusivity), the file's broader public-API-misuse-shaped panic surface had not yet been
checked - HEURISTICS.md's "state-machine invariant that isn't checked, only assumed" pattern,
matching the general class already confirmed multiple times this session (`023`/`024` from the
prior session, `038` this session).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: read every public method on `DicomCollector` (`read_preamble`, `read_file_meta`, `take_file_meta`, `read_dataset_to_end`, `read_dataset_up_to`, `read_dataset_up_to_pixeldata`, `read_next_fragment`, `read_basic_offset_table`) tracing whether any publicly-reachable call sequence could invoke `parser()`/`raw_reader_mut()` before the corresponding state transition (`set_parser_with_ts`) has happened | Static code reading, no build needed | `DicomCollector` maintains its own explicit `CollectorState` field (`Start`/`Preamble`/...) with `ensure!(self.state == CollectorState::Start, IllegalStateStartSnafu)`-style checks at entry to state-sensitive methods, returning a proper `Result::Err` (not a panic) for out-of-order calls at the *collector* level. Crucially, `read_dataset_to_end`/`read_dataset_up_to` (the methods that reach `CollectionSource::parser()`) explicitly check `!self.source.has_parser()` first and self-initialize the parser (via `set_parser_with_ts`, using a guessed transfer syntax hint if `read_file_meta` was never called, itself gracefully erroring via `.context(GuessTransferSyntaxSnafu)?` if no hint can be guessed) *before* ever falling into the `else` branch that calls the panicking `parser()` - so that branch is only reached when `has_parser()` is already confirmed true, making the panic genuinely unreachable through this call path. `raw_reader_mut()`'s panic (called after the source has already become a `Parser`) was similarly checked: its only caller found, `read_preamble`, is itself guarded by `ensure!(self.state == CollectorState::Start, ..)`, and `CollectorState::Start` cannot coexist with a `CollectionSource::Parser` (the transition to `Parser` only happens inside `read_file_meta`, which requires having already progressed past `Start`/`Preamble`) |

## Status

**Not reproducible** - ruled out by tracing every public call path to both panic sites; the
collector's own `CollectorState` tracking (a proper, `Result`-returning state machine at the public
API layer) prevents any call sequence from reaching either `panic!()`/`unreachable!()` site inside
the lower-level `CollectionSource` enum.

## Class

n/a

## Root cause

Not applicable (no bug) - unlike the `034` (`ul` P-Data strict-mode gap) or `038`
(`PixelDecoder` default-vs-override drift) findings from this same session, where a public method
genuinely lacked the validation its sibling had, `DicomCollector`'s public methods consistently
self-check state (either via the explicit `CollectorState` field or via `has_parser()`) before
touching the lower-level enum whose own panics assume a specific internal invariant. This is a
well-defended internal state machine, not a coincidentally-safe one.

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

- Only the two `panic!()`/`unreachable!()` sites and the public methods that could plausibly reach
  them were traced - the rest of this 1896-line file (in particular `read_next_fragment`/
  `read_basic_offset_table`'s pixel-data-fragment-specific logic, and the `Self::collect_to_object`
  helper shared by the dataset-reading methods) was not exhaustively re-read line by line for
  unrelated issues beyond this specific state-machine question.
