# `Data`/`DataRef::DateTimeIso`/`DurationIso`: `String` -> `Box<str>`, shrinks the whole enum 32 -> 24 bytes

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

Follow-up to candidate 006's own "what's not done" note: `size_of::<Data>()` stayed 32 bytes even
after `Data::String` shrank from `String` (24B) to `Arc<str>` (16B), because `DateTimeIso`/
`DurationIso` (still plain `String`, 24B) became the new largest variant. This candidate tests
that lever **in isolation**, on top of plain `HEAD` (candidate 006's `Arc<str>` change is not
applied here - reverted per this workspace's own rule, so `Data::String` is back to plain `String`
in this tree). The question was whether shrinking just `DateTimeIso`/`DurationIso` alone (leaving
`Data::String` untouched) does anything, given `Data::String` is *also* 24 bytes at baseline and
would seemingly still set the enum's size on its own.

## What was tried

Changed `Data::DateTimeIso(String)` -> `Data::DateTimeIso(Box<str>)` (and the same for
`DurationIso`), in both the `Data` and `DataRef` enums (`src/datatype.rs`). `Box<str>` was chosen
over `Arc<str>` since these values are read-only ISO date/duration strings sourced only from ODS
(`office:date-value`/`office:time-value` attributes) and XLSX's rare `t="d"` cell-type path
(`src/xlsx/cells_reader.rs`) - never deduplicated via a shared-string table the way candidate 006's
`String` variant is, so there's no sharing benefit to justify `Arc`'s extra refcount overhead;
`Box<str>` is the same 16 bytes as `Arc<str>` with none of that overhead. Four call sites needed a
mechanical `String`/`&str` -> `Box<str>` conversion (`.into()`), all via `From`, no logic change:
`src/ods.rs` (2 sites), `src/xlsx/cells_reader.rs` (1 site), plus one `Display`/pattern-match
adjustment in `examples/excel_to_csv.rs` (an or-pattern binding `Data::String(s) |
Data::DateTimeIso(s) | Data::DurationIso(s)` no longer type-checks once the bound type differs
across arms - split into two arms, same behavior). `tests/test.rs` needed 21 mechanical
`.to_string()` -> `.into()` edits at literal-construction call sites, no assertions changed.

**Isolated size check first** (`std::mem::size_of`, both real crate types and a standalone
synthetic mockup to cross-check): baseline `size_of::<Data>() == size_of::<DataRef>() == 32`
(confirms candidate 006's note - `String`, 24B, plus an 8-byte discriminant tag, since 9 variants
is too many to niche-pack into `String`'s single-null-value niche). Patched (only `DateTimeIso`/
`DurationIso` shrunk, `Data::String` untouched, still plain `String`): `size_of::<Data>() ==
size_of::<DataRef>() == 24` - an 8-byte-per-value reduction, **even though `Data::String` is still
24 bytes and unchanged**. This was not the expected outcome going in (see "Why the result looks
the way it does" below for why it happens anyway) - worth a synthetic cross-check since it initially
looked surprising: a standalone `rustc -O` mockup with the same variant shapes (`i64`/`f64`/
`String`/`bool`/16B-struct/`Box<str>`/`Box<str>`/8-variant-enum/unit) reproduced the same 32 -> 24
transition, confirming this isn't a real-crate-specific artifact.

Since this shrinks the whole `Data`/`DataRef` payload (not just `DateTimeIso`/`DurationIso`
values), it affects **every** dense-grid cell slot regardless of content type - measured on two
real fixtures via `___OPTIMIZATIONS___/harness/rss_probe` (`range` mode, release build,
`debug = true` in the harness's own profile), 6 runs each (3 `taskset -c 3` pinned + 3 unpinned,
both directions), all consistent to within ~0.5%:

| Variant | XLSX real file (`big_300k_rows.xlsx`, 20MB, 2.4M cells, ~75% shared-string, no `DateTimeIso`/`DurationIso` cells at all) peak RSS | XLS real file (`adhocallbabynames1996to2016.xls`, 18MB, 1.47M cells) peak RSS | XLSX wall-clock (median) | XLS wall-clock (median) |
|---|---|---|---|---|
| baseline (`HEAD` 5c02e86), 6 runs | 171,860-172,624 KB | 150,916-151,404 KB | ~621 ms | ~150 ms |
| patched, 6 runs | 134,456-135,104 KB | 132,860-133,524 KB | ~592 ms | ~129 ms |

`sig` (the harness's own correctness checksum over every cell's row/col/value) identical baseline
vs. patched on every run, both files.

**Note the XLSX fixture has zero `DateTimeIso`/`DurationIso` cells** (all shared strings/numbers/
dates via the `DateTime`/binary-serial path, not ISO-string cells) - the peak-RSS drop there is
entirely from the smaller `Data` payload applying to *every* cell slot in the dense
`Vec<Cell<Data>>` grid, not from any `DateTimeIso`-specific savings. This is the mechanism the
candidate's own framing predicted ("every dense-grid slot pays 32 bytes... even for Int/Float/
Empty-heavy sheets").

## Result

Metric: **peak RSS** (primary, per this workspace's own focus) and **wall-clock**, both measured;
**end RSS** (retained memory) also recorded, see the anomaly noted below.

- **XLSX real file: peak RSS -21.8%** (172.0-172.6 MB -> 134.5-135.1 MB, consistent across 6 runs
  each direction, single-core and unpinned). Wall-clock **-4.4%** (~621 ms -> ~592 ms median).
- **XLS real file: peak RSS -11.9%** (150.9-151.4 MB -> 132.9-133.5 MB). Wall-clock **-14%**
  (~150 ms -> ~129 ms median).
- Both real-file results are consistent with the isolated `size_of` finding (24/32 = 0.75, i.e. a
  predicted 25% reduction in the raw per-cell payload cost; observed reduction is somewhat lower
  than 25% for both files since `Vec<Cell<Data>>` also carries per-`Cell` row/col overhead
  (`Cell<T>` wraps `Data`/`DataRef` with position fields - see `src/lib.rs`) that this change
  doesn't touch, plus the sparse-buffer-then-dense-buffer overlap noted in candidates 001/002/004
  contributes a baseline "floor" this change doesn't address either).
- **End RSS (retained memory at process exit) moved in the OPPOSITE direction on the XLS fixture**:
  78.1-78.3 MB -> 91.2-91.4 MB (**+16.7%**), reproduced identically even in `open` mode (no cell
  iteration at all - XLS's `open_workbook_auto` parses the whole workbook upfront regardless of
  mode, so this isn't a `Range`-specific effect): baseline `open` end RSS 35.6 MB, patched 55.1 MB.
  XLSX showed no such effect (end RSS ~3.6-4.1 MB both ways, noise-level, since XLSX's `open` is
  lazy and the harness drops the `Range` per-sheet during the loop). See "Why the result looks the
  way it does" for the likely explanation (not fully confirmed).

## Does it change behavior/output?

- **Bit-exact?** Yes - `sig` (harness's cell-by-cell checksum) identical baseline vs. patched on
  every run, both real fixtures, both directions (pinned and unpinned).
- **What changes, for whom:** `Data::DateTimeIso`/`Data::DurationIso` (and the same two `DataRef`
  variants) change their inner type from `String` to `Box<str>`. Any downstream code
  pattern-matching these variants expecting `String` (e.g. taking ownership, calling
  `String`-specific methods like `.push_str()`), or an or-pattern combining these with
  `Data::String(s)` in one arm (as `examples/excel_to_csv.rs` did), needs updating - same shape of
  breakage as candidate 006's `Arc<str>` change, smaller blast radius (2 variants, not the whole
  `String` variant). Code using `DataType` trait accessors (`get_datetime_iso() -> Option<&str>`,
  `get_duration_iso() -> Option<&str>`, `as_string()`, `Display`) is unaffected - `Box<str>` derefs
  to `str` the same way `String` did, and implements `Display` directly.
- **Is this a breaking change?** Yes, a public field-type change on 4 enum variants (2 each in
  `Data`/`DataRef`) - same class of breaking change as candidate 006, smaller in scope. Would need
  a semver-relevant bump if taken for real.
- Project test suite: **45/45 unit tests, 165/166 integration tests (1 pre-existing `#[ignore]`,
  unrelated), 67/67 doc tests - all passing both before and after** (`cargo test --all-features`,
  baseline via `git stash`, patched from the worktree). `cargo clippy --all-features --all-targets`
  clean, no warnings introduced. The project's own `tests/test.rs` needed 21 mechanical
  `.to_string()` -> `.into()` edits at literal-construction sites (included in `patch.diff`, no
  assertion values changed) to compile against the new type.

## Why the result looks the way it does

**The `size_of` shrink from 32 to 24, despite `Data::String` staying 24-byte `String` and
unchanged, is the counterintuitive part - worth spelling out.** Naively, since `Data::String` is
still a 24-byte variant after this change, one might expect it alone to still force the enum to at
least 24+tag = 32 bytes, the same as baseline. That's not what happens, and the standalone
synthetic mockup (see above) reproduces the same transition, so this isn't a real-crate quirk:
at baseline, **three** variants tie at 24 bytes (`String`, `DateTimeIso(String)`,
`DurationIso(String)`) - with three same-sized "large" variants and 6 smaller ones, rustc's enum
layout algorithm apparently doesn't find a niche-packing scheme that avoids an explicit tag,
landing on 24 (payload) + 8 (tag, alignment-padded) = 32. Once only **one** variant (`String`) is
the unique largest, rustc's niche-filling optimization can exploit `String`'s inner `Vec`'s
non-null pointer niche (or an equivalent scheme) to pack all 8 other variants' discriminants into
that single field's spare bit-pattern space without needing a separate tag byte at all, landing on
24 total. This is a compiler-layout implementation detail (observed empirically via `size_of`,
`rustc 1.98.0`; not derived from a `-Zprint-type-sizes` layout dump, which needs nightly and
wasn't run this session) - it means the *number of variants tied for the maximum payload size*, not
just the maximum size itself, matters to `Data`'s total footprint. This also means **this change
alone (without candidate 006's `Arc<str>` for `String`) is a real, independent, already-effective
lever** - it doesn't need candidate 006 applied first to matter, contrary to what the candidate's
own framing initially assumed ("did not shrink despite candidate 006's change" - true on top of
006, but this session found shrinking *works standalone too*, via the tie-breaking mechanism above,
not because 006 needs to be present).

**Why XLSX's peak-RSS win (-21.8%) is bigger than the raw 25% payload reduction might suggest, and
XLS's (-11.9%) is smaller:** XLSX's peak is dominated by its `Vec<Cell<Data>>` dense-grid buffer
(2.4M cells, mostly `Data::Int`/`Data::Float`/shared-string references, no `DateTimeIso` cells at
all in this fixture) with comparatively little else contributing to the peak at the same moment;
shrinking that one buffer's per-element cost dominates the whole peak. XLS's peak includes more
non-`Data`-buffer overhead relative to its cell count (BIFF8 parsing state, an internal
shared-strings `Vec<String>` untouched by this change, workbook/font/format tables, etc.), so the
same per-cell-payload reduction moves peak proportionally less.

**The end-RSS increase on XLS (not reproduced on XLSX) is a real, reproducible, but not fully
root-caused finding - reported factually, not explained away.** It reproduces even in `open` mode
(zero cells iterated), so it's not specific to `Range`/dense-grid construction - something about
XLS's overall parse allocates differently once the per-value payload shrinks. The most plausible
mechanism (not independently verified this session, e.g. by swapping the global allocator or
running under `strace`/`malloc_trim` instrumentation, which would be needed to confirm rather than
speculate): glibc's default allocator uses different retention behavior for `brk`-heap allocations
(not eagerly returned to the OS on free) vs. `mmap`-backed ones (returned via `munmap` on free) -
shrinking many small per-cell allocations can shift some of them across that `M_MMAP_THRESHOLD`
boundary, trading a lower peak for more heap fragmentation/retention afterward. This is a *shape*
of explanation consistent with the observation (peak down, end up, XLS-specific where XLS's parse
path does more heap churn than XLSX's), not a confirmed root cause - flagged in "What's not done"
below rather than asserted as fact.

## What's not done / open questions

- **Not applied for real** - like every candidate in this workspace, measured, verified, and
  reverted; applying it (with a semver-appropriate bump) is a separate decision for the project
  owner.
- **The end-RSS-on-XLS anomaly (+16.7%) was not root-caused**, only characterized and reproduced
  (see above) - would need allocator-level instrumentation (a custom allocator wrapper counting
  bytes by size class, or `strace -e trace=mmap,munmap,brk`) to confirm the `brk`/`mmap` threshold
  hypothesis rather than assume it. Worth revisiting if a future session builds that
  instrumentation for another candidate anyway.
- **Combining this with candidate 006's `Arc<str>` change was not measured together in this
  session** (each candidate tests one change against pristine `HEAD`, per this workspace's "one fix
  per candidate" rule) - the candidate's own originating idea predicted these two would "compound";
  given this session's finding that the size win doesn't actually need 006 as a prerequisite, the
  compound case is still worth checking (does stacking both get all the way to `Arc<str>`'s 16-byte
  floor, or does the same tie-breaking-variant-count effect cap it lower?) - a natural next
  candidate, not attempted here.
- **XLSB was not independently measured** - no large real-world `.xlsb` fixture with `DateTimeIso`/
  `DurationIso` usage was available this session (same gap noted in candidate 006). The mechanism
  (dense `Vec<Cell<Data>>`, same `Data` type) is identical to XLSX's, so a similar peak-RSS
  reduction is expected on any large XLSB file regardless of whether it uses ISO-string cells, but
  this is not confirmed with a real fixture.
- **`formula_stress_test.xlsx` (74KB, many formulas) was not separately measured** - too small
  relative to noise floor to add a meaningful data point for a per-cell-payload change; the two
  larger fixtures already establish the effect clearly.
