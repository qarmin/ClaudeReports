# select_worker_height_width_copypaste

`Decoder::select_worker` (src/decoder.rs:245-259) is the heuristic that decides whether decoding
should use the `Multithreaded`/`Rayon` worker backend or the single-threaded `Immediate` one,
based on whether the image's pixel area (`width * height`) exceeds `PARALLELISM_THRESHOLD`
(128*128 = 16384). Direct code reading showed the `height` local is computed from
`frame.output_size.width` instead of `frame.output_size.height` - a copy-paste mistake that makes
the heuristic square the width instead of computing width*height.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (direct unit test of the private function) | `cargo +stable test --lib`, two `#[test]`s added to `src/decoder.rs` calling `Decoder::<std::io::Empty>::select_worker` with `FrameInfo`s for 999x8 and 10x2000 | Both fail against pristine code, both pass after the one-line fix - deterministic, reproduces every run (5/5) |
| Real end-to-end decode via the public API | `cargo +stable run --example verify_dims` against a real 999x8 JPEG fixture (`fixtures/test_wide.jpg`, made with `convert -size 999x8 xc:gray`) | Decode succeeds and produces exactly 999*8=7992 bytes regardless of the bug - confirms `select_worker` really is called with these exact `output_size` values during a real decode (src/decoder.rs:610), and that the bug has no effect on decode correctness (see "Root cause" for why) |

A third escalating tool (fuzzing/sanitizers) was not needed - the arithmetic mistake is fully
deterministic and boundary-value construction plus a real end-to-end dimension check already gave
a full, repeatable signal.

## Status

**Confirmed**. Reproduces 5/5 runs (deterministic pure-function arithmetic, debug build,
`cargo +stable test --lib`, no sanitizer needed).

**Fully reproducible**: Yes, with a stated deviation from the usual public-API-only shape - see
below.

`Decoder::select_worker`, `FrameInfo`, `Dimensions` and `PreferWorkerKind` are all private
(`mod parser;` / `mod worker;` are not `pub mod` in `src/lib.rs`), so nothing outside the crate can
name or call them, and - by design - the two backends `select_worker` chooses between produce
bit-identical decoded output (see "Root cause"), so there is no externally observable symptom
reachable through the public API alone (no crash, no wrong pixel). This is not a "two participants"
finding under CONFIRMATION.md section 11 (no peer/attacker input is being faked) - it is a single
deterministic function computing the wrong thing from a struct field, and section 11's own
carve-out for a single external artifact applies: a real JPEG file decoded through the real public
`Decoder::decode()` entry point *is* the genuine input, there is no "other side" to impersonate.
Because the only channel to observe `select_worker`'s actual decision is to call it directly, the
repro adds two `#[test]`s straight into `src/decoder.rs` (the standard, idiomatic way to unit-test
a private Rust function) with `FrameInfo` values built from real width/height pairs, plus a
separate real end-to-end decode (`examples/verify_dims.rs`, using only the crate's actual public
`Decoder::new`/`decode`/`info` API) against a real JPEG fixture with those exact same dimensions,
proving those dimensions are exactly what a real image handed to the real public entry point
produces and that the bug's effect is confined to the internal worker-backend choice. See
`repro/README.md` for the full rationale.

## Class

logic (internal performance heuristic mis-selection; no crash, no incorrect decoded output)

## Root cause

```rust
PreferWorkerKind::Multithreaded => {
    let width: u64 = frame.output_size.width.into();
    let height: u64 = frame.output_size.width.into();   // should be .height
    if width * height > PARALLELISM_THRESHOLD {
        PreferWorkerKind::Multithreaded
    } else {
        PreferWorkerKind::Immediate
    }
}
```

`height` is assigned from `frame.output_size.width` instead of `frame.output_size.height`, so the
heuristic computes `width * width` instead of `width * height`. For any non-square image this
picks the wrong branch of the threshold comparison:

- Wide/short images (e.g. 999x8, true area 7992 < 16384): `width*width` = 998001 > 16384, so the
  heavier `Multithreaded`/`Rayon` backend gets picked for a genuinely tiny image.
- Narrow/tall images (e.g. 10x2000, true area 20000 > 16384): `width*width` = 100 < 16384, so the
  cheaper `Immediate` backend gets picked even though the image is large enough that parallelism
  was the intended choice.

Both directions were confirmed with the two unit tests in the repro. Because
`compute_image_parallel`'s `Immediate`, `Multithreaded` (mpsc), and `Rayon` backends
(src/worker/mod.rs, src/worker/rayon.rs, src/worker/immediate.rs, src/worker/multithreaded.rs) are
all functionally equivalent by design (same output, different execution strategy), this
mis-selection has no effect on decode correctness - only on which internal code path/overhead
profile gets used, for non-square images whose width alone (squared) crosses the threshold on the
wrong side of where the real area does.

## Introduced by

cfa7b497781023b6c8c3801a58f3bf30c77e7a4e "Sketch: choose worker late" (2022-05-14)

The line was introduced as-is in this commit (confirmed via `git blame`); a later commit,
49139b73 "Adjust constants for preferred worker", only changed the threshold constant, not this
line.

## The fix

One-line change, restoring the second field access to `.height`:

```diff
-                let height: u64 = frame.output_size.width.into();
+                let height: u64 = frame.output_size.height.into();
```

Verified against both repro unit tests (both go from FAILED to ok - see `output.txt`), and checked
against a second, opposite-direction boundary value (10x2000, not just the original 999x8) per
CONFIRMATION.md section 7, so the fix isn't narrowly tailored to the one input that first found
it.

- Project test suite (`cargo +stable test`, full suite: lib unit tests, `tests/lib.rs`,
  `tests/rayon.rs`, `tests/rayon-0.rs`, `tests/rayon-1.rs`, `tests/rayon-2.rs`, doctests): 23
  passed, 0 failed before the fix -> 23 passed, 0 failed after the fix. No new failures, no
  existing test happened to depend on the buggy selection.

## What's not done / open questions

- No other call sites of `select_worker` share this specific mistake - it's called from exactly
  two places (src/decoder.rs:458 and :610), both routing through the same single buggy function,
  so there is only one instance of this bug, not several.
- This is a pure internal scheduling-heuristic bug with no correctness or memory-safety
  consequence; a maintainer may reasonably consider it low-impact (wrong backend choice, not wrong
  output), but it is a real, deterministic logic bug and the fix is a trivial one-line correction
  with no behavioral downside.
