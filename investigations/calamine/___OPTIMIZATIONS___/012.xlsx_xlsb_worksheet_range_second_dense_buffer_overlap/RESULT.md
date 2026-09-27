# XLSX/XLSB `worksheet_range()`: attempt an in-place `DataRef -> Data` conversion, find there was no second buffer to remove

`rustc 1.98.0`, `cargo 1.98.0`.

Candidate 007's own RESULT.md flags, as its own "What's not done" item #1, that
`worksheet_range()` (`src/xlsx/mod.rs`, `src/xlsb/mod.rs`) calls `worksheet_range_ref()` (one
dense `Range<DataRef>`, sparse-buffer-free since 007) and then does
`rge.inner.into_iter().map(Into::into).collect()` to build a *second* full dense `Range<Data>` -
reasoning that the old `Range<DataRef>` buffer is only dropped once the new `Range<Data>` buffer
has fully grown, so "both dense buffers... are briefly alive together". This candidate implements
the never-attempted fix that note calls for: an explicit in-place, cell-by-cell conversion that
writes each `Data` value back into the same `Vec<DataRef>` allocation instead of collecting into a
fresh `Vec`, replacing the `.into_iter().map(Into::into).collect()` call in both `worksheet_range()`
implementations with `Range<DataRef>::into_data_in_place()` (`src/lib.rs`), an `unsafe`
`ptr::read`/`ptr::write`/`Vec::from_raw_parts` loop guarded by `const` size/align assertions
(`size_of::<DataRef>() == size_of::<Data>()`, both 32 bytes per candidate 002's own measurement).

**Before measuring the real pipeline, an isolated diagnostic (see "Why the result looks the way it
does" below) showed this was very likely to be a no-op**: a standalone example using a
byte-counting global allocator around exactly `Vec<DataRef>::into_iter().map(Into::into).collect::<Vec<Data>>()`
(no calamine parsing involved at all) showed the *plain* `collect()` **already reuses the same
underlying allocation** (`before_ptr == after_ptr` on the outer `Vec`, confirmed via pointer
comparison) and allocates **zero** extra bytes for a `DataRef::Int`-only input, and *exactly* the
owned-`String` cost (55.8 MB for a synthetic 2.4M-cell / 75%-`SharedString` input, matching
candidate 002's own ~56MB estimate almost exactly) for a `SharedString`-heavy input - not the cost
of a second dense buffer on top of that. This candidate's real end-to-end measurement (below)
confirms that isolated finding.

## What was tried

Isolated diagnostic (`examples/inplace_check.rs` / `inplace_check2.rs` in the worktree, not part
of `patch.diff` - throwaway, not shipped): custom `#[global_allocator]` tracking current/peak
allocated bytes, run around `Vec<DataRef<'static>>::into_iter().map(Into::into).collect::<Vec<Data>>()`
in isolation.

| Input | peak bytes during `collect()` (isolated) | same allocation reused (outer `Vec`)? |
|---|---|---|
| 2,000,000x `DataRef::Int` | 0 | yes |
| 2,400,000x (~75% `DataRef::SharedString`, 25% `DataRef::Int`) | 55,800,000 | yes |

End-to-end (`___OPTIMIZATIONS___/harness/rss_probe`, `range` mode = `worksheet_range()`, peak RSS
= `/proc/self/status` `VmHWM`, release build). Because this candidate's own target (the
`DataRef -> Data` conversion) sits immediately downstream of candidate 007's fix, "baseline" here
means candidate 007's patch applied but NOT this candidate's - isolating this candidate's own
effect from 007's already-measured, separate sparse-buffer removal - rather than plain `HEAD`.

| Variant | `big_300k_rows.xlsx` (2.4M cells, ~75% shared-string) peak RSS | notes |
|---|---|---|
| baseline (007 applied only), `taskset -c 3`, 2 runs | 134,740 / 134,712 KB | |
| baseline (007 applied only), unpinned, 2 runs | 134,524 / 135,016 KB | |
| patched (007 + this candidate), `taskset -c 3`, 2 runs | 134,784 / 134,708 KB | first build |
| patched (007 + this candidate), unpinned, 2 runs | 134,572 / 134,248 KB | first build |
| patched (007 + this candidate), `taskset -c 3`, 2 runs | 134,668 / 134,740 KB | fresh rebuild, re-verification |
| patched (007 + this candidate), unpinned, 2 runs | 134,428 / 134,516 KB | fresh rebuild, re-verification |
| `just test` (fresh clone, independent of dev worktree) | baseline 135,016 / patched 134,588 KB | matches the ranges above |

Baseline mean (4 runs): **134,748 KB**. Patched mean (8 runs across two independent
builds): **134,564 KB**. Difference: ~184 KB (~0.14%) - smaller than the run-to-run spread already
observed within the baseline alone (134,524-135,016 KB, a ~500 KB range), i.e. within noise, not a
measured improvement.

`sig`/`cells` (2,400,000) were bit-identical across every baseline and patched run, and on the
small `formula_stress_test.xlsx` fixture (`sig=16125784316220351567`, `cells=4542`, matching
candidate 007's own recorded value for that file).

Elapsed time: no attempt made to draw a conclusion from it here - both variants sit in the
575-705ms range seen for this file throughout this workspace's prior candidates, i.e. unaffected,
as expected for a change whose entire premise was about memory, not compute.

## Result

**No change.** Metric measured: peak RSS (`VmHWM`), corroborated by an isolated allocation-byte
count around the exact operation in question. The ~184 KB difference between baseline and patched
means is within this fixture's own measured run-to-run noise (~500 KB spread) - not a real signal
in either direction. The manual in-place conversion this candidate implements is functionally
redundant: `rustc`'s standard `Vec::from_iter` specialization for `Map<vec::IntoIter<T>, F>`
already performs the identical in-place reuse (same allocation, no extra bytes) whenever `T`/`U`
share size and alignment, which is exactly the case here (`DataRef`/`Data`, both 32 bytes, 8-byte
aligned).

## Does it change behavior/output?

- **Bit-exact?** Yes - `sig`/`cells` identical to baseline on both fixtures used
  (`big_300k_rows.xlsx`, `formula_stress_test.xlsx`), across every run.
- **Is this a breaking change?** No public API change. `Range::into_data_in_place()` is
  `pub(crate)`, invisible outside the crate; `worksheet_range()`'s documented behavior is
  unchanged.
- Project test suite, `cargo test --release --all-features`: baseline (candidate 007 applied
  only) and patched (007 + this candidate), identical both times: **45/45 unit tests, 165/166
  integration tests (1 pre-existing `#[ignore]`, unrelated), 67/67 doc tests**. `cargo clippy
  --release --all-features --all-targets` clean on the patched tree.
- This is a real change to the tested project's own source (`src/lib.rs`, `src/xlsx/mod.rs`,
  `src/xlsb/mod.rs`) - `patch.diff` is included per this workspace's rules, even though the
  measured result is "no change", because a `pub(crate)` function and two call-site edits were
  genuinely applied and measured, not merely investigated.

## Why the result looks the way it does

Candidate 007's own "Why the result looks the way it does" section treated the `DataRef -> Data`
conversion as *two* separate, additive sources of overlap: (1) "a second dense buffer" from
`.into_iter().map(Into::into).collect()`, and (2) the owned-`String` allocation cost of converting
`DataRef::SharedString(&str)` into `Data::String(String)`. This candidate's isolated diagnostic
(see "What was tried") shows these were never two separate things - they are the **same**
phenomenon, mis-attributed as two. `rustc`'s in-place-collect specialization (an established,
longstanding optimization in `liballoc`'s `Vec::from_iter` for a `Map` over a `Vec`'s own
`IntoIter`, active whenever the source and destination item types share size and alignment) already
reuses the exact same backing allocation for `Vec<DataRef>::into_iter().map(Into::into).collect::<Vec<Data>>()`
- confirmed directly via pointer identity and an allocator byte-counter, not inferred from
`size_of` equality alone. There never was a second dense buffer materializing before the first one
dropped; the ~56 MB this candidate (and candidate 002, and candidate 007) all separately noticed on
this fixture is entirely accounted for by item (2) alone - the unavoidable-as-currently-implemented
cost of turning each borrowed shared-string reference into an owned `String`. That remaining cost
is candidate 006's territory (`Arc<str>` interning), not this candidate's.

This also means the hand-written `unsafe` `into_data_in_place()` this candidate adds is strictly
redundant with what the compiler already does for free via safe code - it does not regress
correctness (verified bit-exact and via the full test suite) but it buys nothing either, at the
cost of an `unsafe` block that safe code didn't need.

## What's not done / open questions

- Did not re-verify whether the same in-place-collect specialization still applies if `Data`'s
  size ever changes relative to `DataRef`'s (e.g. if a future change shrinks one but not the
  other) - the `const` assertions added in `into_data_in_place()` would at least turn a future
  mismatch into a compile error rather than silent UB, but since this candidate's own conclusion
  is "don't take this patch, the compiler already does it", that safeguard has no real target to
  protect once this patch itself isn't applied.
- Did not check whether the same in-place-collect specialization also already applies to
  `worksheet_formula()`'s sparse-cell handling or any other `.into_iter().map(...).collect()` site
  in the crate - this candidate only checked the one call site flagged by candidate 007. A quick
  grep-and-check of other `Vec`-to-`Vec` `map().collect()` sites for same-size/align source/dest
  pairs is a plausible (if likely low-yield, given this result) follow-up.
- The real remaining lever this candidate reconfirms (not discovers) is candidate 006's
  `Arc<str>`-interning angle for `Data::String`/`SharedString`, now that candidate 007's own fix
  has been layered in as this candidate's baseline - candidate 006 itself was measured against
  plain `HEAD` (pre-007), where the sparse buffer still dominated the peak and masked any string-
  interning saving; re-measuring 006's patch on top of 007's is still open, flagged again here.
- The isolated diagnostic examples (`examples/inplace_check.rs`, `examples/inplace_check2.rs`)
  used during investigation were not kept in the worktree or `patch.diff` - they were exploratory
  only, not part of the shipped (redundant) fix.
