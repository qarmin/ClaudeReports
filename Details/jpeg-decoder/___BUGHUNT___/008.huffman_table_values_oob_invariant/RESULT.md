# huffman_table_values_oob_invariant

`HuffmanDecoder::decode` (src/huffman.rs:31-58) computes `index = (code + table.delta[i]) as usize`
and reads `table.values[index]` with no bounds check at the use site (src/huffman.rs:51-52). This
looked suspicious because `table.delta`/`table.maxcode` (src/huffman.rs:191-207) are built from a
fully attacker-controlled DHT segment (bit-length counts and value bytes, src/parser.rs:536-589,
parsed with no cross-validation against `delta`/`maxcode` beyond the classic
"is the code set over-subscribed" check in `derive_huffman_codes`), and correctness of the index
math relies entirely on that construction always keeping `index` within `[0, table.values.len())` -
an invariant that is assumed, not asserted, at the point it's used.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning about the canonical-Huffman-table construction (Figure C.2 / F.15 / F.16 from ITU-T T.81, which this code implements directly) | N/A (pen-and-paper) | The `code >= (1 << size)` check in `derive_huffman_codes` (src/huffman.rs:276-278) rejects any over-subscribed bit-length-count set. For a non-over-subscribed (possibly incomplete) canonical code, `mincode[i] <= code` is expected to hold whenever `code <= maxcode[i]` matches at the first length `i` that isn't rejected by a shorter length's `maxcode` check - this is the standard property canonical Huffman decode tables rely on. Reasoning alone wasn't enough to rule out a corner case (e.g. very skewed bit-length distributions), so escalated to actual computation. |
| Standalone Python re-implementation of `derive_huffman_codes` + the `delta`/`maxcode` build, exhaustively swept over all 65536 possible 16-bit bitstream prefixes per table, across ~2000 bits[] configurations (random totals 1-40 spread over the 16 length classes, plus hand-picked adversarial patterns concentrating codes at length 1 and length 16 to maximize `delta[]` skew) | `python3 sim.py`, ~2s | 0 out-of-range indices found (`tested 2006 found= False`) |
| In-crate exhaustive Rust test using the actual production `HuffmanTable::new`/`derive_huffman_codes` (temporarily added to src/huffman.rs as `#[cfg(test)] mod oob_probe_tests`, then reverted with `git checkout -- src/huffman.rs`; see `output.txt`), building 3005 table configurations (5 hand-picked adversarial + 3000 pseudo-random via a fixed-seed xorshift, same distribution shape as the Python sweep) and, for each, exhaustively checking all 65536 16-bit bitstream prefixes against the real `lut`/`maxcode`/`delta` fields using the exact index arithmetic `decode()` uses (`cargo +stable test --lib oob_probe_tests -- --nocapture`, ~5s, ~197M simulated decode attempts total) | 0 out-of-range indices found; test passed |

`HuffmanTable::values` is a `Vec<u8>`, indexed with ordinary `values[index]` (never
`get_unchecked`) - confirmed unchanged across the file's whole history (`git log -p -- src/huffman.rs
| grep 'table.values\['`, only ever the checked form). So even in the hypothetical case this
invariant were violated, the failure mode would be a bounds-checked panic (a crash/DoS), not
silent memory-unsafety/UB - noted here in case a future session revisits this with a different
angle, since it changes which class this would be if ever confirmed.

## Status

**Not reproducible.** ~197 million simulated `decode()` index computations (in Rust, against the
real `HuffmanTable::new`/`derive_huffman_codes`/`lut`/`maxcode`/`delta` construction, across 3005
distinct DHT-derived table shapes including several hand-picked adversarial ones designed to
maximize the `delta[]` skew that would be needed to push `index` negative or past
`values.len()`), plus an independent ~2000-configuration cross-check in a from-scratch Python
model of the same construction, found zero out-of-bounds indices. This matches the expected
mathematical property of canonical Huffman decode tables (ITU-T T.81 Annex F/C): as long as
`derive_huffman_codes`'s over-subscription check (`code >= 1 << size`) rejects a bits[] set, the
resulting `mincode`/`maxcode`/`delta` triple keeps every reachable `index` within
`[0, values.len())` for every possible bitstream, including incomplete (not fully Kraft-saturated)
code sets. No live thread to keep digging on was found (3 clean negatives), per `SKILL.md`'s soft
cap.

**Fully reproducible**: N/A (not applicable to a Not-reproducible result).

## Class

N/A (not applicable to a Not-reproducible result).

## Root cause

Not applicable - the suspected invariant violation could not be produced. The actual reason the
suspect turned out to be safe: `derive_huffman_codes`'s single validation
(`code >= (1 << size) => Err`, src/huffman.rs:276-278) is sufficient on its own to guarantee the
`delta`/`maxcode` pair built in `HuffmanTable::new` (src/huffman.rs:197-207) keeps
`code + delta[i]` within `[0, values.len())` for every `code` that can ever reach that computation
in `decode()` - there is no gap between "code passes `code <= maxcode[i]` at the first
non-rejected length" and "code is actually `>= mincode[i]`" for a non-over-subscribed canonical
code, so `index` never goes negative (which is the failure mode that would matter here, since a
negative `i32` cast to `usize` would produce a huge value) and never exceeds `values.len() - 1`.

## Introduced by

N/A - no bug was confirmed, so there is nothing to trace to an introducing commit.

## The fix

No fix - nothing to fix, per Not-reproducible status.

- Project test suite: not run before/after (no change was made to the working tree; the temporary
  probe test was reverted with `git checkout -- src/huffman.rs` before this candidate was closed
  out, main tree confirmed clean via `git status --porcelain`). N/A.

## What's not done / open questions

- The probe covers `HuffmanDecoder::decode`'s slow-path index computation only (the `else` branch
  for codes of length 9-16, src/huffman.rs:43-53). The `lut` (fast path, lengths <=8) and `ac_lut`
  (`decode_fast_ac`) tables were not separately probed for the same class of issue - they don't use
  the `delta`/`maxcode`/index-arithmetic pattern this candidate targeted (they index by the raw
  8-bit LUT position, which is always in `[0, 256)` by construction), so they look structurally
  safe by inspection, but that inspection wasn't backed by the same exhaustive-probe rigor applied
  here.
- The probe used pseudo-random (fixed-seed) and hand-picked bits[] distributions, not a true
  exhaustive sweep of the entire bits[] input space (which is combinatorially far too large - up to
  256 codes spread over 16 length classes). A dedicated `fuzz-bug-repro` campaign against
  `parse_dht` + `HuffmanDecoder::decode` together (mentioned as a possible follow-up in the original
  candidate note) would give broader coverage than this session's bounded probe, but is a
  meaningfully larger effort than this candidate's scope; not started this session.
- This candidate did not check whether an out-of-bounds `index` is reachable through some path
  other than a malformed DHT segment (e.g. `fill_default_mjpeg_tables`'s hardcoded tables, which
  are fixed at compile time and not attacker-controlled, so were not considered in scope).
