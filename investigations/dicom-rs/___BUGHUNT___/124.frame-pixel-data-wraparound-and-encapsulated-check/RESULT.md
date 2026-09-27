# frame-pixel-data-wraparound-and-encapsulated-check

Direct follow-up/deeper verification of candidate `117` (`encoding-frame-pixel-data-native-offset-
overflow`), which fixed `encoding/src/adapters.rs`'s `PixelDataObject::frame_pixel_data` default
method's native-pixel-data branch (`start = frame as usize * frame_size`, `end = start +
frame_size`, both plain unchecked `usize` arithmetic, bounds-checked only *after* the fact) with
`checked_mul`/`checked_add` returning `None` on overflow. Candidate 117's own "What's not done"
section left two gaps explicitly open, matching the identical caveat already documented by
candidates `086`/`087` (the sibling bug class in `pixeldata/src/lib.rs`): (1) no genuine 64-bit
wraparound-into-bounds demonstration was ever built (only a 32-bit demonstration from the upstream
issue, and a debug-mode panic re-derivation), and (2) the encapsulated-pixel-data branches of the
same function, which use `offset_table` entries summed via ordinary `+=`, were never independently
checked for a comparable overflow. This candidate answers both.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| **Part 1** - modular-arithmetic search (Python, exact big-integer arithmetic, brute-force/CRT over legal `Rows`/`Columns`/`SamplesPerPixel`/`BitsAllocated` combinations and a 2-adic-valuation-based search for a `u32` `frame` value) for a `frame`/`frame_size` pair whose true product exceeds `2**64` by a small remainder | Static reasoning, no build needed | Found: `rows=32768, columns=20000, samples_per_pixel=7, bits_allocated=2` (bytes_per_sample=1) gives `frame_size=4,587,520,000`; `frame=4,021,071,096` (< `u32::MAX`) gives `true_product = frame*frame_size = 18,446,744,074,321,920,000 = 1*2**64 + 612,368,384` - a genuine wraparound, with the wrapped `start=612,368,384` and `end=start+frame_size=5,199,888,384` both small enough that a real ~4.84 GiB buffer can actually hold `end` bytes (see root-cause section below for why `frame_size` must itself be roughly `>= 2**32` for any `u32` `frame` to be able to overflow `usize` at all on a 64-bit target - this is not a free/arbitrary choice) |
| **Part 1** - standalone `repro/src/wraparound.rs`, own `Cargo.toml`/`[workspace]`, built against a fresh clone of pristine `df2b9e42` (pre-117-fix code, dependency path `../clone/encoding`), driving the real public `PixelDataObject::frame_pixel_data` default trait method end-to-end on a real implementor with the values above; `pixel_data` is a real `vec![0u8; 5_199_888_384]` (zero-filled, lazily committed on Linux so the ~4.84 GiB virtual allocation stays cheap) with an 8-byte marker (`b"WRAPPED!"`) planted at the predicted wrapped `start` offset | `--release` (default profile, overflow-checks off, no workspace override), 3 separate runs | Reproduced 3/3: `Some(4587520000 bytes)`, first 8 bytes exactly match the planted marker at offset 612,368,384 - proving the returned bytes really were read from the predicted usize-wrapped offset, not merely "some Some(..) came back by coincidence". Exit code 1 ("WRAPAROUND CONFIRMED") every run |
| Same repro/same inputs, `--dev` (debug) build | 1 run | Panics at `encoding/src/adapters.rs:272:33: attempt to multiply with overflow` - the exact same panic site candidate 117 already documented, on the exact same input that produces a genuine wraparound in release, tying the two manifestations (debug panic / release silent-wrong-data) together as the same defect on the same input, not two unrelated observations |
| Same repro, **117-fix.patch** (verbatim copy of candidate 117's own `patch.diff`) applied to the clone | `--release`, 3 separate runs | All 3: `OK: no panic, returned None` (exit 0) - the fix rejects the overflowing computation instead of wrapping into a spuriously in-bounds offset. Confirmed both ways (bug present before `apply`, gone after) against the same clone |
| **Part 2** - direct code reading of the "Other cases of multi-frame objects" match arm (`Some(number_of_fragments) =>` when `number_of_fragments != number_of_frames`, i.e. the actual offset-table-walking branch, distinct from the `Some(n) if n == number_of_frames` 1:1 shortcut arm just above it) | n/a | No multiplication of two large attacker-controlled factors anywhere in this branch: `base_offset`/`next_offset` are `u32` offset-table entries widened (lossless on 64-bit) to `usize`, never multiplied; the loop's accumulator `offset` only ever gains `fragment.len() + 8` per real, actually-fetched fragment (`self.fragment(idx)?` - `?` aborts the whole call as soon as a claimed fragment index doesn't actually exist, so `number_of_fragments` lying upward doesn't let the loop run past real data) |
| **Part 2** - standalone `repro/src/encapsulated_offset_table.rs` exercising the real `frame_pixel_data` entry point on 4 deliberately adversarial encapsulated objects (a Basic Offset Table entry of `u32::MAX` for both a `frame != 0` and `frame == 0` request; a `frame` near `u32::MAX` against a tiny real offset table; a `number_of_fragments` of `u32::MAX` against 1 real fragment) - each object's `number_of_frames` deliberately set `!=` its `number_of_fragments` so the match actually takes the offset-table-walking arm, not the 1:1 shortcut (an earlier version of this file got that guard condition wrong and silently tested the shortcut arm instead for 2 of the 4 cases - caught by inspecting the actual output before trusting it) | `--release`, run against the same pristine clone and again against the same clone with `117-fix.patch` applied | No panics, no overflow, in either state (byte-identical output before/after, as expected since candidate 117's patch never touches this branch): 2 cases return `Some(0 bytes)` (a minor, separate correctness quirk - a frame whose offset-table entry can never be reached by real accumulated offsets synthesizes an empty result instead of `None` - not the overflow class being checked for), 2 return `None` |

## Status

**Confirmed** - this status answers Part 1's sub-question: the release-mode "silent frame
corruption" the original issue (`Enet4/dicom-rs#817`) alleged but candidate 117 never
independently re-derived on a 64-bit target is real, reproduced 3/3 runs against pristine
`df2b9e42`, and candidate 117's existing fix (re-verified here, not re-invented) closes it 3/3 runs
against the same clone with the patch applied. This is **not a new, distinct bug** - it is a
positive, complete verification that (a) the exact defect candidate 117 already fixed does have a
genuine 64-bit wraparound-into-bounds manifestation (not just a 32-bit one or a debug-mode panic),
and (b) candidate 117's fix is sufficient to close that manifestation too, not just the panic. Part
2's own sub-question is answered separately as **Not reproducible**: no comparable overflow was
found reachable in the encapsulated-pixel-data branch on a 64-bit target, for the reasons in the
table above - if a future session wants a single `SUMMARY.md` disposition for this whole candidate,
Part 1's `Confirmed` (re-)verification is the operative one; Part 2 found nothing new to add.

**Fully reproducible**: Yes, for both parts. Both repros drive the real, public
`PixelDataObject::frame_pixel_data` default trait method end-to-end via real implementors of the
crate's own public trait - the same public entry point any real caller (a pixel-data adapter, a
higher-level `DecodedPixelData` built on top of this trait) would use. There is no second "peer"
process here: the untrusted input this bug is about is the object's own declared attribute values
and pixel data bytes, supplied through the exact same public trait implementation a real object
would use - CONFIRMATION.md section 11's real-second-participant requirement does not apply (same
reasoning candidate 117's own `RESULT.md` already gave).

## Class

`security` (Part 1: a genuine, deterministically-constructed 64-bit `usize` wraparound that makes
the pre-fix `frame_pixel_data` silently return `Some(wrong_bytes)` - real attacker-influenced-length
image bytes sliced from an unintended offset - instead of erroring or panicking, for an
individually-legal-but-adversarially-combined DICOM object requesting an out-of-range frame index;
no `unsafe`, no memory-safety implication beyond returning data the caller did not ask for and has
no way to detect is wrong). Candidate 117 itself classified the same underlying defect as `crash`
(the debug-mode panic manifestation); this candidate additionally confirms the non-crash,
silent-wrong-data manifestation the original issue's title actually emphasized.

## Root cause

**Part 1** (native branch, same root cause candidate 117 already documented and fixed): `frame as
usize * frame_size` / `start + frame_size`, both plain `usize` arithmetic, computed before the
existing `end <= pixel_data.len()` check runs. This candidate adds the missing piece: solving for a
concrete legal input that makes the wraparound land back *inside* a real, allocatable buffer rather
than merely proving the multiplication can panic. The search confirms an inherent constraint worth
recording for any future work on this bug class: since `frame` is a `u32` (< 2**32), `frame *
frame_size` can only exceed `2**64` at all if `frame_size` itself is roughly `>= 2**32` (~4.3 GB) -
meaning any *release-mode, into-bounds* demonstration of this specific overflow fundamentally
requires a multi-gigabyte "pixel data" buffer to exist (the 32-bit case in the original issue had no
such floor, since `usize` was already only 32 bits there). `determine_bytes_per_native_frame`'s own
internal multiplication (`rows * columns * samples_per_pixel * bytes_per_sample`) does not itself
overflow at any legal attribute values (max ~2.3x10^18, under `u64::MAX`) - only the subsequent
`frame *` multiply does, and only for large `frame_size` values that are themselves legal but
unusual (a single frame taking multiple gigabytes, which is unrealistic content-wise but not
excluded by any attribute's own VR range).

**Part 2** (encapsulated branch): no root cause to report - the accumulation shape (`offset +=
fragment.len() + 8`, compared via `>=` against `usize`-widened `u32` offset-table entries, bounded
by `self.fragment(idx)?` refusing to fabricate fragments beyond what's actually present) does not
have the "two large attacker-controlled factors multiplied together" shape that both this bug class
and 117's own fix target. The loop can be walked a declared-but-lied-about number of times, but each
step's own arithmetic stays bounded by real, already-allocated fragment data.

## Introduced by

Part 1: same commit candidate 117 already identified, `61e43337` "[encoding] Add function to access
frame pixeldata" (the commit that introduced `frame_pixel_data`'s native-pixel-data branch with this
exact unchecked arithmetic shape - already fixed by candidate 117's unmerged patch, re-verified here
against a genuine wraparound input rather than only a panic).
Part 2: N/A - no new defect found.

## The fix

No new fix from this candidate - candidate 117's existing `patch.diff` (replacing the plain
multiply/add with `checked_mul`/`checked_add`, short-circuiting to `None` on overflow) is
independently re-verified here to close the genuine 64-bit wraparound as well as the debug-mode
panic. A verbatim copy is included in this folder as `117-fix.patch` (attributed to candidate 117,
not authored by this session) purely so this folder's own `justfile`/`repro/` can independently
drive the same before/after cycle without depending on the `___BUGHUNT___/117.../` folder existing
alongside it. **No `patch.diff` was produced by this candidate** - see "What's not done" below for
why (nothing new to fix).

No workspace-wide test suite run was performed this session: no tracked file in this worktree was
modified (the fix under test lives only inside the disposable `clone/` subfolder created for this
candidate's own repro), so there is nothing to regress. Candidate 117's own `RESULT.md` already
recorded `cargo test -p dicom-encoding` (40 passed/4 doctests passed, before and after, zero
regressions) and a full `cargo test --workspace --exclude dcms` (0 failures before/after) for this
exact patch.

## What's not done / open questions

- No new `patch.diff` from this candidate - Part 1 re-verifies candidate 117's existing fix against
  a harder input (not a new defect to fix); Part 2 found no defect to fix.
- Part 2's minor observation (a `frame` whose own offset-table entry can never be reached by real
  accumulated fragment offsets returns `Some(0 bytes)` instead of `None`; a `number_of_fragments`
  that overstates the real fragment count discards already-accumulated frame data instead of
  returning it, because `self.fragment(idx)?`'s `None` propagates through the whole function) is a
  logic/correctness quirk, not the arithmetic-overflow class this candidate was scoped to - not
  investigated further or written up as its own candidate this session; left as a possible future
  `IDEAS.md` entry for the orchestrator to triage if it's judged worth its own candidate.
- The `~4.84 GiB` buffer this candidate's Part 1 repro allocates is real (not simulated/mocked) but
  large; it relies on Linux's lazy anonymous-page commit behavior for `vec![0u8; N]` to stay cheap
  in practice (only the 8 marker bytes and the returned slice's own bytes are ever touched) - this
  was empirically confirmed to run without issue in this session's sandbox (29 GiB total, ~3.8 GiB
  free physical at the time) but a memory-constrained environment (e.g. a tight `ulimit -v` or a
  container with a low memory limit well under ~5 GiB) may not be able to run this specific repro;
  this is inherent to the bug's own 64-bit `u32`-frame-index constraint (see root cause above), not
  a repro-construction shortcut that could be tightened further.
