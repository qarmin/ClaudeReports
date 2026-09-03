# Animated WebP: "dispose to background" never actually clears the canvas by default

Sourced from still-open upstream https://github.com/image-rs/image/issues/2913 ("Decoding
animated WebP images with transparency doesn't work correctly... the current frame is 'drawn on
top' the previous frame"). This repo's own `src/codecs/webp/decoder.rs` does **no frame
compositing of its own at all** - `WebPDecoder::read_image`/`read_frame` just forwards straight
into `image_webp::WebPDecoder::read_frame`, which returns an already-fully-composited canvas. So
the candidate's premise of a "shared compositing path" between GIF/WebP/APNG in this repo's own
source is false for WebP specifically: GIF (`src/codecs/gif.rs`) and APNG
(`src/codecs/png.rs::ApngDecoder`) each implement their own independent disposal/blend logic
in-tree and were spot-checked (code read + this project's own passing `mixed-disposal.gif` /
`any-disposal.gif` / `png/apng/wpt/dispose-*` reference-image tests) with no analogous bug found;
WebP's compositing is 100% inside the external `image-webp` crate (pinned at `0.2.4` in this
repo's `Cargo.lock`), where the actual bug lives.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `src/codecs/webp/decoder.rs`, `src/codecs/gif.rs`, `src/codecs/png.rs` (ApngDecoder) | n/a | Confirmed WebP has zero in-repo compositing code (fully delegated); GIF/APNG each have their own independent, seemingly-correct disposal implementations, not shared with WebP. |
| Run this repo's existing `cargo test --test reference_images` (per-frame byte-exact comparison against reference PNGs) | debug build, default features, `gif/anim/*`, `png/apng/*`, `webp/extended_images/anim.webp` fixtures | 189/189 passed on pristine tree - no *existing* test fixture in this repo currently exercises the specific "no explicit background color + dispose-to-background + following transparent alpha-blended frame" shape that triggers the bug. |
| `gh api`/`gh` against `image-rs/image` and `image-rs/image-webp` (issue #2913, its comments, PR image-rs/image-webp#171, tag/branch history, crates.io sparse index) | n/a | Confirmed upstream root cause and fix location/commit; confirmed the fix (PR #171, merged 2026-03-02) postdates the pinned `image-webp` `0.2.4` (tagged 2025-08-27) and that no newer `image-webp` version has been published to crates.io as of this session (latest published: `0.2.4`). |
| Hand-built, spec-valid 3-frame animated WebP (RIFF/VP8X/ANIM/ANMF, real VP8L lossless sub-bitstreams via `image_webp::WebPEncoder`), decoded through this repo's real public `image::codecs::webp::WebPDecoder` entry point | debug + release build, default webp feature, in-tree example and standalone `repro/` (both in-tree `path = "../../.."` and via a fresh `git clone` at `base_commit` per the candidate `justfile`) | **Bug reproduced, 3/3 runs each in 3 separate contexts** (in-tree example, in-tree `repro/`, fresh-clone `repro/` via `just test`) - see `output.txt`. |
| Attempted fix: `[patch.crates-io]` pinning `image-webp` to the upstream git commit containing PR #171 (tried both the `release-0.2.5` branch tip and `main` tip) | `cargo update -p image-webp`, rebuild, rerun repro + `cargo test --lib` + `cargo test --test reference_images` | Repro's own bug goes away (3/3), and `cargo test --lib` stays 331/0 - **but** `cargo test --test reference_images` regresses from 189/0 to 188/1: `webp/extended_images/anim.webp` frame 2 no longer matches its reference PNG, identically for both candidate git revs. See "The fix" below. |

## Status

**Confirmed**

Reproduces deterministically, 3/3 runs, against the pristine dependency tree (`image-webp 0.2.4`
as currently pinned by this repo's `Cargo.lock`), both in a debug build and a release build, via
the real public `image::codecs::webp::WebPDecoder` API.

**Fully reproducible**: **Yes**. The repro is a single-participant case per `CONFIRMATION.md`
section 11 (a malformed/adversarial file handed to a parser's real public entry point *is* the
genuine external input - no second participant is being impersonated). The animated WebP file
itself, while hand-constructed byte-for-byte, is spec-valid RIFF/VP8X/ANIM/ANMF container syntax
wrapping real VP8L lossless sub-bitstreams (produced once via the real `image_webp::WebPEncoder`,
not hand-authored entropy-coded data), and is decoded through this repo's real, unmodified public
`WebPDecoder::new`/`read_image` entry point exactly as a real caller reading a real `.webp` file
from disk would.

## Class

logic

## Root cause

Entirely inside the `image-webp` dependency (external crate, not this repo's own source), pinned
at `0.2.4` by this repo's `Cargo.lock`/`Cargo.toml` (`image-webp = { version = "0.2.0", optional
= true }`).

In `image_webp::WebPDecoder::read_frame` (`src/decoder.rs`, around the `dispose`/`clear_color`
computation just before the sub-chunk is read), the canvas-clearing color used for a "dispose to
background" frame transition is computed as:

```rust
let clear_color = if self.animation.dispose_next_frame {
    info.background_color
} else {
    None
};
```

`info.background_color` is an `Option<[u8; 4]>` that starts as `None` and is **only** ever set via
the crate's own public `WebPDecoder::set_background_color()` (exposed in this repo as
`image::codecs::webp::WebPDecoder::set_background_color`) - it is never derived from the WebP
file's own `ANIM` chunk background-color-hint field (that hint is parsed separately into
`background_color_hint` and is available via a different accessor, but is not auto-applied). So
for the overwhelming majority of real callers - anyone who doesn't explicitly call
`set_background_color` before decoding, which is the default/undocumented-as-required path -
`clear_color` is always `None` whenever a frame's disposal method is "dispose to background",
meaning **the canvas is silently never actually cleared**. The next frame is then composited (via
`extended::composite_frame`) directly on top of whatever the still-undisposed previous frame left
behind. For a frame that is itself fully or partially transparent, this makes the previous,
supposedly-disposed frame visibly "show through"/"stay drawn" - exactly the "later frames appear
drawn on top of earlier ones instead of the disposal method being respected" symptom from
image-rs/image#2913.

This is already fixed upstream in `image-rs/image-webp` by PR #171 ("fix: dispose transparent
animated frames", merged 2026-03-02), which changes the computation to fall back to fully
transparent black when no explicit background color is set and the frame has an alpha channel:

```rust
let clear_color = if self.animation.dispose_next_frame {
    match (info.background_color, frame_has_alpha) {
        (color @ Some(_), _) => color,
        (_, true) => Some([0, 0, 0, 0]),
        _ => None,
    }
} else {
    None
};
```

As of this session, `image-webp 0.2.4` (tagged 2025-08-27) is still the latest version published
to crates.io (confirmed via `index.crates.io`'s sparse index) - PR #171 (merged 2026-03-02) has
not shipped in any published release yet, matching the maintainers' own comments on #2913 ("Fixed
on `image-webp` `main`... now we just need someone with publish rights to ship it").

## Introduced by

Not determined - bug resides in the external `image-webp` crate's own source (not vendored/tracked in this repo's git history), not in this repo's own code.

## The fix

**No bounded fix verified this session.** The only fix reachable from within this repo's own
source (there is no in-repo WebP compositing code to patch directly - see above) is bumping the
`image-webp` dependency past the point containing PR #171. Since no such version is published to
crates.io yet, the only mechanism available in this repo today is a `[patch.crates-io]` override
pinning `image-webp` to an upstream git revision:

```toml
[patch.crates-io]
image-webp = { git = "https://github.com/image-rs/image-webp", rev = "f4d80bd965df2c81e65b6f43c1f70e0750bd4b0f" }
```

This was tried (with both the `release-0.2.5` branch tip `75f810915d02ae4ff55d3f825bf6a4b07efdf994`
and the `main` branch tip `f4d80bd965df2c81e65b6f43c1f70e0750bd4b0f` - both contain PR #171,
result identical either way) and does make this candidate's own repro pass (3/3 runs, canvas
correctly clears to transparent). **However it regresses this project's own pre-existing
`tests/reference_images.rs` fixture `webp/extended_images/anim.webp`**: frame 2 (0-indexed frame
1, the second of six ANMF frames, itself an `ALPH`+`VP8` lossy-with-alpha frame, blended with
`dispose_next_frame` inherited from frame 0's `dispose=1`) no longer byte-matches its checked-in
reference PNG under the patched dependency. `cargo test --lib` (this repo's own unit tests, 331
tests) is unaffected either way.

This means the "obvious" upstream fix is not a safe drop-in for this repo without further
investigation this session did not have scope for: either (a) this repo's own `anim.webp` /
`anim.webp.anim_02.png` reference fixture pair needs regenerating against a trusted third-party
decoder to confirm which behavior (old or PR #171's) is actually spec-correct for that specific
real-world file, or (b) PR #171's fix has its own edge-case gap (e.g. it hardcodes transparent
black for the no-explicit-background-color case rather than deriving from the file's own
`background_color_hint`, which this specific fixture sets to `[0,0,0,0]` BGRA - so that shouldn't
be the discrepancy, but wasn't fully root-caused this session) that a maintainer picking up
`image-webp`'s next release would want to check before publishing. Per `SKILL.md`'s step 6, this
is recorded as "Confirmed, no bounded fix found this session" rather than shipping a patch that
trades one observable regression for another.

- Project test suite (`cargo test --lib`): **331 passed, 0 failed**, unchanged before and after
  the attempted `[patch.crates-io]` fix.
- Project test suite (`cargo test --test reference_images`): **189 passed, 0 failed before** the
  attempted fix -> **188 passed, 1 failed after** (new failure:
  `image webp/extended_images/anim.webp`, "Frame 2: Reference image does not match").

## What's not done / open questions

- Root-causing *why* PR #171's fix specifically changes `anim.webp` frame 2's output was not
  completed this session - worth a maintainer/upstream look before this repo bumps past
  `image-webp 0.2.4`, since it may indicate PR #171 itself has a gap (e.g. around lossy `VP8`+`ALPH`
  frames specifically, versus the `VP8L` lossless frames this candidate's own repro uses) rather
  than this repo's reference fixture being simply stale.
- Whether the same bug class (clearing skipped by default unless a caller opts in via an
  explicit setter) exists anywhere else in `image-webp`'s own decoder was not swept - only the
  one `dispose_next_frame`/`clear_color` site from #2913/PR #171 was investigated.
- GIF's `blend_and_dispose_pixel` and APNG's `mix_next_frame` dispose-region logic (both in this
  repo's own source) were spot-checked by code reading plus their own existing passing
  disposal-focused reference-image tests, but not independently fuzzed/boundary-tested this
  session the way the WebP path was - see `IDEAS.md` for a follow-up suggestion.
- Once `image-webp` publishes a release containing PR #171 (or a fix for whatever causes the
  `anim.webp` regression), re-running `cargo update -p image-webp` and this candidate's `repro/`
  against the then-current pinned version would be the natural way to re-check whether this is
  fully resolved without needing a `[patch.crates-io]` workaround at all.
