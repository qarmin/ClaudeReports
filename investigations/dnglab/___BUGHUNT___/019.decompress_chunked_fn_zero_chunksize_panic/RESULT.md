# decompress_chunked_fn_zero_chunksize_panic

`rawler/src/decompressors/mod.rs:284-294` `decompress_chunked_fn` calls
`.pixels_mut().par_chunks_mut(chunksize)` with no zero-check, the same shape that made candidate
004's four sibling helpers (`decompress_lines_fn`/`decompress_lines`/`decompress_strips_fn`/
`decompress_strips`) panic unconditionally on a file-tag-derived `width==0`/`stripsize==0`. This
candidate was pre-flagged as needing confirmation of a real, reachable zero-`chunksize` caller
before it could be treated the same way (candidate 004's own "What's not done" section explicitly
left this open, since `decompress_chunked_fn`'s closure signature has no `Result` return path,
making a same-shaped fix a bigger, unverified API change).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct unit-level call to the real public `decompress_chunked_fn::<u16, _>(width, height, 0, ...)` API with a no-op closure, added as a temporary `#[test]` in `rawler/src/decompressors/mod.rs`'s existing `#[cfg(test)] mod tests` | debug build, `cargo test -p rawler --lib`, 4 repeat runs | Panics every time, deterministic: `"chunk_size must not be zero"` at `rawler/src/decompressors/mod.rs:290:20` (the `par_chunks_mut(chunksize)` call), 4/4 runs. Confirms the function's shape is exactly as suspected in isolation - see full output in `output.txt` section 1 |
| Whole-repo `grep -rn "decompress_chunked_fn"` (excluding `target/`) to find every real caller | - | Exactly one caller exists in the entire codebase: `rawler/src/decoders/rw2/v5decompressor.rs`'s `decode_panasonic_v5` (`decompress_chunked_fn`'s definition itself is the only other match) - see `output.txt` section 3 |
| Reading/tracing that one caller's `chunksize` argument (`pixels_per_block`) back to its inputs, to determine whether any file-controlled value can make it 0 | - | `pixels_per_block = V5_PACKETS_PER_BLOCK * pixels_per_packet`. `V5_PACKETS_PER_BLOCK` is `V5_BLOCK_SIZE / V5_BYTES_PER_PACKET = 0x4000 / 16 = 1024`, a fixed compile-time constant with no dependency on `width`/`height`/any file tag. `pixels_per_packet` is matched on the function's `bps` parameter: `12 => 10`, `14 => 9`, and `_ => unreachable!()` for any other value - so `pixels_per_block` (the `chunksize` argument) can only ever be exactly `10240` or `9216`. Any `bps` value other than 12/14 panics via `unreachable!()` *before* `decompress_chunked_fn` is ever called, rather than producing a 0. Traced one level further up: `bps` itself comes straight from a file's `PanasonicTag::BitsPerSample` TIFF tag (`rw2.rs:133`, fully attacker-controlled), but that only ever changes *which* of the two fixed nonzero constants is used, or triggers the separate `unreachable!()` panic - it can never make `chunksize` become 0. See `output.txt` sections 4-6 for the traced source and the two computed constants |

## Status

**Not reproducible** - not through any real, reachable caller. The function itself does panic on
`chunksize == 0` when called directly (confirmed 4/4), but the entire codebase has exactly one real
caller of `decompress_chunked_fn`, and that caller's `chunksize` argument is provably restricted to
one of two fixed, nonzero compile-time-derived values (`10240` or `9216`) regardless of any file
content - `width`, `height`, and every other field of a RW2 file have no path into `chunksize` at
all. This isn't "no input was found that reproduces it despite trying" (which would be
Inconclusive/weak signal); it's a closed-form proof from reading the single call site that no input
*can* reach `chunksize == 0` through this function's only current caller.

**Fully reproducible**: N/A - no `Confirmed` bug is being reported. The isolated unit-level call
(section 1 of `output.txt`) is a direct, hand-constructed call to the suspect function standing in
for a caller that provably does not exist anywhere in the current codebase, so per `CONFIRMATION.md`
section 11 it does not qualify as demonstrating a real, reachable bug - it only demonstrates the
function's documented-by-`rayon` precondition (`chunksize != 0`) is unenforced, which is a latent
API footgun, not a reachable defect this session could confirm end-to-end.

## Class

N/A (Not reproducible).

## Root cause

Not a reachable bug as things stand. `decompress_chunked_fn` is `pub fn`, exported from a `pub mod
decompressors`, so it is part of `rawler`'s public API surface and nothing stops a future or
external caller from passing an untrusted/computed `chunksize` of 0 to it - the same class of
mistake candidate 004 found real, reachable instances of in the four sibling helpers. But as of the
commit checked (see below), the crate's own single internal caller (`decode_panasonic_v5`) never
computes `chunksize` from anything file-controlled; it is always one of exactly two hardcoded
constants selected by a `bps` value that is itself restricted to `{12, 14}` by an `unreachable!()`
panic on every other value. So the panic shape candidate 004 found in `decompress_lines_fn` and its
three siblings does not currently have an analogous reachable trigger for `decompress_chunked_fn`.

`decompress_chunked_fn` and its sole caller (`v5decompressor.rs`) were both introduced together in
commit a5e8e2e3 ("Improve and reorganize packed decompressor") - the constant-only `chunksize`
shape has been present since that same commit, not something that regressed from an earlier,
reachable form.

## Introduced by

N/A - no bug is being confirmed here.

## The fix

None applicable - no bounded fix is proposed, since no reachable trigger for the suspected panic
exists in the current codebase to fix against. Hardening `decompress_chunked_fn` itself (an explicit
`chunksize == 0` guard, mirroring candidate 004's fix to the other four helpers) would still be a
reasonable defensive change given it's public API with an unenforced precondition, but doing so
without a confirmed reachable trigger would be speculative rather than a verified fix - left for
`IDEAS.md` (see below) rather than applied here.

- Project test suite: not run before/after, since no source change was made this candidate (N/A -
  no fix to verify). The temporary unit-test addition used to check the function's isolated
  behavior was reverted (`git checkout --`) before this candidate was written up; the working tree
  used for this investigation is clean.

## What's not done / open questions

- `decompress_chunked_fn` remains an unguarded public API (unlike the four siblings candidate 004
  fixed) - if a future decoder adds a second caller that derives `chunksize` from file-controlled
  data (the same mistake `decode_strips` made for the other four helpers), it would reintroduce
  exactly this panic. Worth a defensive `chunksize == 0` guard even without a currently-reachable
  trigger, but that's a proactive hardening change, not a bug fix for a confirmed reachable defect -
  noted for `IDEAS.md` rather than done speculatively this session.
- A different, real panic was noticed in the same function while tracing this candidate but not
  independently investigated: `decode_panasonic_v5`'s `match bps { 12 => 10, 14 => 9, _ =>
  unreachable!() }` (`rawler/src/decoders/rw2/v5decompressor.rs`, around line 19-22) hard-panics via
  `unreachable!()` for any `BitsPerSample` tag value other than 12 or 14, when a RW2 file's
  `RawFormat` tag selects format 5 (`rw2.rs:375`). `bps` is read straight from the file's own
  `PanasonicTag::BitsPerSample` tag (`rw2.rs:133`) with no validation before being passed through -
  this looks like the same "file-tag-derived value hits an incomplete match/panic macro" bug class
  as candidate 011 (NEF LensData) and candidate 012 (CFA pattern length), and unlike this
  candidate's own suspect, it does not require proving any additional reachability - `RawFormat=5`
  plus an arbitrary `BitsPerSample` looks directly file-controllable. Not confirmed or root-caused
  this session; left as a new backlog item.
