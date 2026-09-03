# WebP anim disposal: does image-webp PR #171's fix have a gap for lossy+alpha frames, or is this repo's own reference PNG just stale?

Follow-up to candidate 014 (`___BUGHUNT___/014.WEBP_ANIM_DISPOSAL_NOT_CLEARED/RESULT.md`,
Confirmed), which left one question open: patching `image-webp` to a git rev containing upstream
PR image-rs/image-webp#171 ("fix: dispose transparent animated frames") fixes candidate 014's own
minimal repro, but flips this repo's own `cargo test --test reference_images` from 189/0 to 188/1
on the real-world fixture `tests/images/webp/extended_images/anim.webp`, frame 2 (0-indexed frame
1, an `ALPH`+`VP8` lossy-with-alpha frame). Two hypotheses were left unresolved: (a) the checked-in
reference PNG is simply stale, produced against the pre-#171 buggy "canvas never cleared" behavior,
or (b) PR #171's fix has its own undocumented gap specific to lossy+alpha frames.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Parsed `tests/images/webp/extended_images/anim.webp`'s real RIFF/VP8X/ANIM/ANMF structure by hand (Python, byte-level) to get ground truth on frame shapes | n/a | All 6 ANMF frames are full-canvas (200x200 at 0,0), every frame has `dispose_to_bg_after=1` and `blend=1` (alpha blending), and every frame is `ALPH`+`VP8` (has alpha). This matters: image-webp's `image-rs/image-webp#178` follow-up fix ("canvas corruption in dispose-to-background for non-alpha frames") only affects frames *without* alpha (`frame_has_alpha=false`) - since every frame here has alpha, that follow-up fix is provably irrelevant to this fixture, ruling it out as the explanation before spending time on it. |
| Read `image-webp`'s git history (`image-rs/image-webp` cloned locally) between `v0.2.4` and the git rev candidate 014 patched to, for every commit touching `src/decoder.rs`/`src/extended.rs` | n/a | Confirmed the rev used by candidate 014 (`f4d80bd9`, main tip) contains PR #171 plus two later fixes: #178 (non-alpha-frame canvas-clear stride bug, ruled out above) and #179 (VP8L dimension-parsing off-by-one, unrelated to disposal/compositing entirely). No other disposal/compositing-relevant commit exists between `0.2.4` and that rev. |
| Differential testing against a real, independent, trusted third-party WebP decoder: decoded the same `anim.webp` frame index 1 via Python Pillow's bundled `libwebp` (`pip install pillow` in a venv; confirmed `features.check('webp')` is `True`), compared pixel-for-pixel against (1) this repo's checked-in reference PNG and (2) this repo's own decode via `image::codecs::webp::WebPDecoder`, both against pristine pinned `image-webp 0.2.4` and against the `[patch.crates-io]` override to the PR #171-containing rev | Ad hoc Python/numpy scripts during investigation; then formalized into the standalone `repro/` (see below) | **Decisive**: pristine `0.2.4`'s decode matches this repo's checked-in reference PNG byte-for-byte (0 mismatches) but diverges sharply from the trusted libwebp decode (4267/40000 = 10.67% of pixels have a wrong alpha value). The PR #171-patched build's decode is the reverse: 0 alpha mismatches against the trusted libwebp decode (exact match) and only ±1 RGB rounding noise for genuinely visible (alpha>0 on both sides) pixels - the same ≤1 rounding noise already present, unrelated to this bug, on frame 0 (which isn't affected by any disposal logic at all and already matches the currently-passing reference). See `output.txt`. |
| Built and ran a standalone `repro/` (own `Cargo.toml`, bare `[workspace]`, real public `image::ImageReader`/`into_frames()` streaming entry point) that performs the same differential comparison, with the trusted libwebp reference and this repo's checked-in reference baked in as fixed comparison PNGs, run via `just test` against a fresh clone at `base_commit` (both the pristine and `Cargo.patched.toml`-overridden builds) | Release build, real clone via `just clone`, `just test` run twice (in-place sanity check, then a fresh standalone run) | Reproduced identically both times - see `output.txt` for the real captured `just test` output. |

## Status

**Not reproducible** (for the suspected bug: "PR #171 has an undocumented gap for lossy+alpha
frames that causes it to produce wrong output for `anim.webp` frame 2"). Ran through 3 escalating
approaches (RIFF/ANMF byte-level ground-truthing, upstream git-history archaeology, and
differential testing against a real independent third-party decoder) without finding any gap in
PR #171 itself for this fixture - the third approach's evidence is strong enough (0 alpha
mismatches, ≤1 RGB rounding noise on visible pixels, matching the pre-existing rounding-noise floor
already present on the currently-passing frame 0) that no further escalation was warranted per
`SKILL.md`'s soft cap: three approaches, the last one a clean, well-supported negative on the
suspected-bug hypothesis, not just "it didn't crash."

**Root cause of the original observation (candidate 014's `reference_images` regression) is
resolved**: hypothesis (a) - the checked-in reference PNG `tests/reference/webp/extended_images/anim.webp.anim_02.png`
is stale, produced under the pre-#171 buggy image-webp behavior (canvas never cleared on "dispose
to background," so frame 0's opaque content silently keeps showing through frame 1's transparent
regions) - not hypothesis (b).

**Fully reproducible**: **Yes**. The differential-testing repro decodes the real, unmodified
`anim.webp` fixture through this repo's real public streaming-reader entry point
(`image::ImageReader::new(...).into_frames()`), the same path any real caller decoding a real
`.webp` file uses - no second participant is being faked (per `CONFIRMATION.md` section 11, a file
handed to a parser's real public entry point *is* the genuine external input). The trusted
third-party comparison target is a real, independent decode by Pillow's bundled `libwebp`, captured
once and checked in as a fixed file (not hand-authored bytes standing in for what a decoder would
produce).

## Class

n/a (Not reproducible - no code bug found this session; see "Root cause" below for what the
investigation did establish).

## Root cause

The regression candidate 014 observed is not caused by any gap in `image-webp` PR #171. It's caused
by the checked-in reference fixture itself predating the fix:

- Before PR #171 (pristine, pinned `0.2.4`): `clear_color` for a "dispose to background" transition
  stays `None` whenever no caller has explicitly called `set_background_color()` (candidate 014's
  original finding), so `extended::composite_frame` never actually clears the canvas region occupied
  by the disposed frame. For `anim.webp`, every frame disposes-to-background and every frame is
  alpha-blended, so frame 0's opaque pixels keep showing through wherever frame 1 is meant to be
  transparent. This repo's checked-in reference PNG for frame 2 was generated against exactly this
  buggy behavior, so it byte-matches pristine `0.2.4`'s output today - and is *wrong* relative to
  what a real WebP renderer/decoder produces for this file.
- After PR #171 (patched to the git rev containing #171/#178/#179): `clear_color` correctly falls
  back to transparent black when no explicit background color is set and the incoming frame has
  alpha (`frame_has_alpha=true`, true for every frame in this fixture), so the canvas is genuinely
  cleared before frame 1 is composited. This output's alpha channel is bit-for-bit identical to an
  independent `libwebp` decode of the same frame, and its RGB channels agree with `libwebp` to
  within ±1 per channel for every pixel where both decoders consider the pixel visible (alpha>0) -
  the same small rounding-noise floor already present, and already accepted, on frame 0 (which
  passes today's test suite and involves no disposal logic at all).

So `cargo test --test reference_images` regressing from 189/0 to 188/1 when PR #171 is applied is
the *expected, correct* consequence of fixing candidate 014's bug: the newly-correct output no
longer matches an old, buggy reference image. It is not evidence of a new bug in PR #171.

## The fix

No source-code fix applies here (there is nothing wrong in `image-webp` PR #171 to fix, and, per
candidate 014, this repo has no in-repo WebP compositing code of its own to patch). The correct
housekeeping action - regenerating `tests/reference/webp/extended_images/anim.webp.anim_02.png`
(and checking the other 5 frames of this fixture for the same staleness, not just frame 2) - was
deliberately **not** performed against the live `Cargo.toml` this session: `image-webp` is still
pinned at `0.2.4` (PR #171 has not shipped in a published release as of this session, matching
candidate 014's finding), so regenerating the reference now, while the pinned dependency still has
the original bug, would make the test suite fail today rather than pass. The correct sequencing is
to regenerate the reference fixture (all 6 `anim.webp` frames, plus a scan for any other animated
WebP fixture with the same "disposes to background with no explicit background color" shape) in the
same change that bumps `image-webp` past the version containing PR #171, not before.

- Project test suite (`cargo test --test reference_images`, pristine tree, `image-webp 0.2.4`):
  **189 passed, 0 failed** (unchanged - no fix was applied to the main project's working tree this
  session).

## What's not done / open questions

- The other 5 frames of `anim.webp` (frames 1, 3, 4, 5, 6) were not individually differential-tested
  against `libwebp` the way frame 2 was - only frame 2 (the one candidate 014 found regressing) was
  checked. Given all 6 ANMF frames share the same disposal/alpha shape, the other frames plausibly
  have the same pre-#171 staleness baked into their own checked-in references, but this wasn't
  empirically confirmed.
- Whether any *other* animated WebP fixture in `tests/images/webp/` has the same
  "dispose-to-background, no explicit background color, alpha-blended following frame" shape (and
  therefore the same stale-reference risk) was not swept this session - only `anim.webp` (the one
  candidate 014 already flagged) was investigated.
- The actual reference-PNG regeneration was intentionally deferred (see "The fix" above) until
  `image-webp` publishes a release containing PR #171 and this repo's `Cargo.lock` can be bumped to
  it for real, rather than via a temporary `[patch.crates-io]` override.
- The ±1-per-channel RGB rounding noise between this repo's `image-webp`-based decode and `libwebp`
  (present on both the pre- and post-#171 builds, and already present on the currently-passing
  frame 0) was not itself investigated - it's small and consistent enough to be ordinary
  floating-point/rounding-mode divergence between two independent YUV->RGB or alpha-blend
  implementations, not flagged as a suspect.
