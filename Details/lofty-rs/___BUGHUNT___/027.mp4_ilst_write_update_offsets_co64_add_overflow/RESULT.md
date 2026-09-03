# mp4_ilst_write_update_offsets_co64_add_overflow

Noticed while working candidate 026: that candidate's 120s post-fix corpus-driven fuzz burst
(`___BUGHUNT___/_reference/026_second_crash_overflow_add.bin`, corpus and harness also under
`_reference/026_*`) turned up a second, distinct panic in
`lofty/src/mp4/ilst/write.rs::update_offsets` at line 491 (`co64`, 64-bit chunk offset rewrite),
separate from candidate 026's own `atom_info.rs` off-by-8 finding, and was deliberately left
uninvestigated there (one bug per candidate). This candidate investigates it directly.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Standalone reproduction of the saved crash artifact (`026_second_crash_overflow_add.bin`), using the project's own public API (`Mp4File::read_from` -> `set_ilst`/`Ilst::default()` -> `save_to`) | Debug build (default `cargo build` profile, debug-assertions on), 3 repeat runs against the main project's own pristine (unpatched) working tree | Deterministic panic, identical message and panic site (`write.rs:491:49`, `attempt to add with overflow`) all 3/3 runs |
| Manual boundary-value reasoning about the arithmetic itself (`(read_offset as i64 + difference) as u64` where `read_offset: u64` comes straight from file bytes with no bound relative to the real file size, and `difference: i64` is a small realistic size delta from the tag rewrite) | Code reading, no build needed | Confirms the bug class: reinterpreting an arbitrary attacker/corruption-controlled `u64` as `i64` via `as` (not a checked conversion) produces a value with no relation to `difference`'s small realistic magnitude, so the following `+` is expected to overflow for any `read_offset` roughly in the upper half of `u64`'s range - matches exactly what the crash input does |

Only two approaches were needed (well under the soft cap of 3) since the first attempt reproduced
deterministically and the arithmetic reasoning fully explained why, with no need to escalate to
`miri`/ASAN or a fresh fuzz harness.

## Status

**Confirmed**

Reproduces deterministically 3/3 runs (debug build, standalone `repro/` against the project's
public API, run directly against the main project's working tree) and again via the fully
standalone `just test` clone-based reproduction (fresh `git clone` of the real repo at
`base_commit`, unmodified working tree) - see `output.txt`.

**Fully reproducible: Yes.** This is a single-participant finding (the library's own write path
mishandling a file it itself already accepted at read time) - no second participant/peer is
involved, so `CONFIRMATION.md` section 11's "real second participant" bar doesn't apply; `repro/`
runs the real, public `Mp4File::read_from` -> `set_ilst` -> `save_to` API end-to-end against a real
crafted file, exactly as any real caller adding/editing MP4 metadata on that file would.

## Class

crash

## Root cause

`lofty/src/mp4/ilst/write.rs::update_offsets` rewrites every chunk/sample-group byte offset stored
in `stco` (32-bit), `co64` (64-bit), and `tfhd` (64-bit `base-data-offset`) atoms that lies at or
past the rewritten `ilst` atom, by adding `difference` (an `i64`: how many bytes the file grew or
shrank because of the tag edit, a small, realistic value bounded by the actual size of the tag data
being written).

For the 64-bit cases (`co64` at line 491, `tfhd` at the equivalent `base_data_offset` branch), the
existing code computed the new offset as:

```rust
write_handle.write_u64::<BigEndian>((read_offset as i64 + difference) as u64)?;
```

`read_offset: u64` is read directly from the file's `co64`/`tfhd` atom content with **no validation
that it's a sane offset relative to the file's actual size** - a `co64` entry can declare any
64-bit value at all, since the read path only ever uses these as opaque per-sample data, never
checking or clamping them against the file length. `read_offset as i64` is a raw bit
reinterpretation (not a checked/lossless conversion): for any `read_offset` with its top bit set
(i.e. roughly the upper half of the `u64` range, which includes any value at or above
`i64::MAX + 1`), the cast produces a value near `i64::MIN` or otherwise numerically unrelated to
`read_offset`'s true magnitude. Adding `difference` (small, in the realistic range this code
expects) to that reinterpreted value overflows `i64`'s range whenever `read_offset`'s cast lands
close enough to `i64::MAX`/`i64::MIN`'s boundary - and in debug-assertions builds (which check `+`
for overflow), that overflow panics instead of silently wrapping.

In the crashing input, at least one `co64` entry declares a `read_offset` whose `i64` reinterpretation
sits close enough to `i64::MAX` that adding the tag rewrite's `difference` (a small positive value)
crosses the boundary, panicking with `attempt to add with overflow` at `write.rs:491:49` -
reachable purely by writing a metadata tag to a real, already-`read_from`-accepted MP4 file whose
`co64` table contains an out-of-range chunk offset (a form of corruption the read path has no reason
to reject, since it never needs to dereference these offsets itself).

The `stco` (32-bit) loop a few lines above uses the analogous `i64::from(read_offset) + difference`
pattern, but `read_offset` there is a `u32`, so `i64::from(read_offset)` is always within
`0..=u32::MAX` - nowhere near `i64`'s overflow boundary for any realistic `difference` - so it was
left unchanged; see "What's not done" below for why this wasn't fully ruled out rather than just
deprioritized.

## Introduced by

304feb94 "MP4: Properly handle offset updates for shrinking tags" (2024-01-20) - `git log -S"read_offset as i64 + difference"` finds this as the commit that introduced `update_offsets`'s `co64`/`tfhd` 64-bit offset-rewriting logic (and the `stco` 32-bit sibling candidate 036 also targets) from scratch.

## The fix

Replaced the raw `read_offset as i64 + difference` reinterpret-and-add in both the `co64` and
`tfhd` loops with a shared helper, `apply_offset_difference`, that performs the addition through a
real checked conversion and checked addition instead of a lossy bit-reinterpretation cast:

```rust
fn apply_offset_difference(
	ident: &AtomIdent<'static>,
	read_offset: u64,
	difference: i64,
) -> Result<u64, FileEncodingError> {
	i64::try_from(read_offset)
		.ok()
		.and_then(|offset| offset.checked_add(difference))
		.and_then(|offset| u64::try_from(offset).ok())
		.ok_or_else(|| {
			FileParseError::from(AtomParseError::message(
				Some(ident.clone()),
				"offset update would overflow",
			))
			.into()
		})
}
```

This is sufficient because it removes the exact mechanism that caused the panic: any `read_offset`
that doesn't fit losslessly into `i64` (or whose sum with `difference` doesn't fit back into `u64`)
now produces a `FileEncodingError` through the same `AtomParseError`/`FileParseError` error path the
file already uses elsewhere in this function (e.g. the adjacent "found an extended `stco`/`tfhd`
atom" checks), instead of panicking - consistent with how this codebase already treats other
malformed-atom conditions as recoverable errors rather than crashes. Checked, not just for the one
triggering `read_offset` value: the helper handles every `u64`/`i64` combination generically (any
`read_offset` failing the `i64::try_from`, any `difference` causing `checked_add` to overflow, and
any resulting negative or too-large sum failing the final `u64::try_from`), not a special case for
the specific crash input.

- Project test suite (`cargo test -p lofty --lib --tests -- --test-threads=1`, same invocation
  candidate 026 used, chosen to avoid a pre-existing, unrelated parallel-test flakiness in
  `probe::tests::probe_{aac,vorbis,wav}` on this checkout): **646 passed, 0 failed, 73 ignored**
  both before and after the fix (identical per-binary breakdown: 256/122/56/2/15/191/4 passed) - no
  new failures, no change.
- `repro/` (the exact crashing input): panics (`attempt to add with overflow`) before the fix,
  gracefully returns `FileEncodingError { format: Some(Mp4), .. }` from `save_to` after the fix
  (3/3 runs each way against the main tree, plus a clean `just test` run from a fresh standalone
  clone - see `output.txt`).

## What's not done / open questions

- The `stco` (32-bit) loop's analogous `(i64::from(read_offset) + difference) as u32` was *not*
  changed. It's structurally safe from the exact overflow mechanism this candidate found (`u32`
  always fits losslessly in `i64`, so there's no reinterpretation step to go wrong), but the final
  `as u32` truncating cast on the result was not specifically checked for silent wraparound if
  `difference` were ever large enough in magnitude to push the sum outside `u32`'s range (this
  would silently corrupt an offset rather than panic, a different, `logic`-class bug, not the
  `crash`-class one this candidate targeted) - worth a follow-up candidate if `difference` can ever
  realistically be large.
- `difference`'s realistic range was reasoned about, not exhaustively verified - it's derived from
  `atom_size_difference` in `write_to`, itself bounded by the size of the tag data actually being
  written (see candidate 026's own note that a longer fuzz campaign would likely find more in this
  area). This fix doesn't depend on `difference` staying small (it's fully generic, see above), but
  the *absence* of a similar bug for `stco` does implicitly rely on that reasoning holding.
- Not re-run through a fresh fuzz burst against this specific fix (unlike candidate 026, which had
  a fuzz harness already set up and warm from the prior session) - the fix was verified only via
  the exact crash input plus the project's existing test suite, per the "bounded fix, verified
  against the confirmed input + test suite" bar; a longer follow-up campaign (candidate 026's own
  harness, kept at `___BUGHUNT___/_reference/026_fuzz_harness/`) would be worth pointing at this
  patched code to look for further issues in the same function.
