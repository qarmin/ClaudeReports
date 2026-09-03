# AVIF_ICC_ERR_SWALLOWED

`src/codecs/avif/decoder.rs:113-116` in `AvifDecoder::new` does
`ctx.icc_colour_information().map(|x| x.ok().unwrap_or_default())`, which silently turns any
genuine `Err` from mp4parse's `icc_colour_information()` (type `Option<Result<&[u8]>>`) into an
empty `Vec<u8>`, unlike the nearby `image_rotation()`/`image_mirror_ptr()` calls which propagate
their `Err` via `.map_err(error_map)?`. The suspicion (from `IDEAS.md`) was that a malformed AVIF
with duplicate/invalid `colr` item-property associations could drive mp4parse's
`icc_colour_information()` into its `Err` branch, which this code would then swallow into
`Some(vec![])` and hand to callers as if a valid-but-empty ICC profile were present.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of mp4parse-0.17.0's `icc_colour_information()`/`get_multiple()` (the exact dependency version pinned in this project's `Cargo.lock`) | Static read of `~/.cargo/registry/.../mp4parse-0.17.0/src/lib.rs` lines 1685-1704 and 3464-3482 | `get_multiple()` never validates "multiple associations" as an error - it just collects every matching `ItemProperty` for the item into a `TryVec` and returns `Ok(values)` unconditionally, filtering out non-matching/`Unsupported` properties via a plain `HashMap::get` (no panic path, no bounds error). `icc_colour_information()` then takes `values[0]` (logging `warn!("Multiple ICC profiles...")` if `values.len() > 1`) and returns `Some(Ok(bytes))`. The **only** way `get_multiple()` returns `Err` is `values.push(property)?` failing inside `fallible_collections::TryVec`, i.e. a real allocation failure (OOM) - not anything a crafted `colr`/`ipma` structure can trigger. This directly contradicts the candidate's premise that duplicate/invalid colr-property associations reach the `Err` branch. |
| Manual boundary-value construction: built a real, valid minimal AVIF (`ffmpeg -f lavfi -i color=red:size=4x4 ... -c:v libaom-av1 -still-picture 1`, using the system's real libaom encoder), then hex-patched its ISOBMFF boxes to append a second, malformed/inconsistent ICC `colr` property (`rICC`, distinct payload) into `ipco` and add a second association for it to the primary item in `ipma` (association_count 4->5, appended index-5 byte) - i.e. exactly the "duplicate/inconsistent colr item-property associations" scenario the candidate named - with all box sizes (`ipco`/`iprp`/`meta`) and the `iloc` extent's absolute file offset corrected for the inserted bytes so the file stays a structurally valid AVIF. Then ran a standalone Rust probe calling the real, unmodified `mp4parse::read_avif()` + `AvifContext::icc_colour_information()` (the exact same calls `AvifDecoder::new` makes) against this file, `mp4parse = "0.17.0"` pulled from crates.io like the main project, release build | `read_avif: Ok`; `icc_colour_information: Ok(len=19) bytes=[70, 65, 75, 69, ...]` ("FAKEICCPROFILEBYTES", the *first* of the two ICC profiles) - **not** `Err`. Reproduces the same result on a second run (deterministic, no randomness/seed involved in this code path). Confirms mp4parse handles duplicate colr-property associations by silently picking the first and logging a warning, exactly matching the code-reading conclusion above - there is no `Err` to swallow in this scenario. |
| Attempted end-to-end check through `image::codecs::avif::AvifDecoder::new` itself (`avif-native` feature) against the same mutated file | `cargo run --no-default-features --features avif-native` | Build failure unrelated to the finding: `dav1d-sys`'s build script requires the `dav1d` pkg-config file/dev headers, which are not installed in this environment (only the runtime `libdav1d.so.7` is present). Not pursued further since the load-bearing question (whether mp4parse's `Err` branch is reachable from a crafted file) was already answered directly against the real `mp4parse::icc_colour_information()` call in the previous step. |

## Status

**Not reproducible.**

The code pattern described (`.map(|x| x.ok().unwrap_or_default())` swallowing an `Err` into an
empty, `Some`-wrapped ICC profile) is real and present at `src/codecs/avif/decoder.rs:113-116`,
but the specific trigger the candidate proposed - a malformed AVIF with duplicate/inconsistent
`colr` item-property associations - does not drive mp4parse 0.17.0's `icc_colour_information()`
into its `Err` branch; that branch handles duplicates by returning `Ok` with the first match plus
a warning log. A hand-crafted AVIF exercising exactly that scenario, run through the real
`mp4parse::read_avif`/`icc_colour_information()` entry points, confirms this (2/2 identical runs).
Within `mp4parse` 0.17.0, `icc_colour_information()`'s `Err` branch is reachable only via an
allocation failure inside `fallible_collections::TryVec::push` (a real OOM condition), which is not
a "malformed input" bug in the classic sense and wasn't further pursued (would require exhausting
process memory, not a bounded crafted file).

## Class

N/A (not reproduced).

## Root cause

Not applicable to the specific trigger proposed - see "Status" above. The `.map(|x| x.ok().unwrap_or_default())`
pattern at `decoder.rs:113-116` is still worth noting as a latent inconsistency (unlike
`image_rotation()`/`image_mirror_ptr()` a few lines below, it silently discards any `Err` mp4parse
*could* someday return from this call, e.g. if a future mp4parse version tightens `get_multiple()`
to treat multiple/conflicting colour-property associations as a hard error the way `get_single()`
already does elsewhere in the same file via `Status::IprpConflict`), but with the currently pinned
mp4parse version it is dead code from a practical-attacker-input standpoint: there is no reachable
crafted-file path that produces `Err` here today.

## The fix

No fix applied - nothing to fix relative to the proposed trigger, since it doesn't reproduce
against the current `mp4parse` version. (A defensive `.map_err(error_map)?` matching the
`image_rotation()`/`image_mirror_ptr()` style would still be a reasonable robustness improvement
against a hypothetical future mp4parse behavior change, but that's a preventive style change, not
a bug fix for a demonstrated defect, so it wasn't made this session - see `SKILL.md`'s "the one
rule": every candidate gets the same treatment, and this one didn't reproduce.)

- Project test suite: not run (no source change made).

## What's not done / open questions

- Did not attempt to trigger the `TryVec::push` OOM path in `get_multiple()` - would require
  driving the process to genuine allocation failure, not a bounded crafted file, and is not really
  "malformed input" in the sense the candidate proposed.
- Did not get a full end-to-end run through `image::codecs::avif::AvifDecoder` (the `avif-native`
  feature) because `dav1d` pkg-config/dev headers aren't installed in this environment; the direct
  `mp4parse::read_avif`/`icc_colour_information()` probe already answers the load-bearing question
  (mp4parse's own `Err` branch is unreachable from this crafted input) without needing the AV1
  decode step, since `AvifDecoder::new` calls the exact same two mp4parse functions unmodified.
- The `.map(|x| x.ok().unwrap_or_default())` vs. `.map_err(error_map)?` style inconsistency at
  `decoder.rs:113-116` still exists in the source and could be revisited if a future mp4parse
  upgrade changes `get_multiple()`'s error behavior (e.g. if it starts treating multiple/duplicate
  colour-property associations as `Status::IprpConflict` the way `get_single()` already does for
  other property types elsewhere in the same file) - worth a quick recheck after any mp4parse
  version bump.
