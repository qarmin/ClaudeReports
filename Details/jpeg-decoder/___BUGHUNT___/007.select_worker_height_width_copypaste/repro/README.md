## Why this repro is shaped this way

`Decoder::select_worker` (src/decoder.rs) and everything it touches (`FrameInfo`,
`PreferWorkerKind`) are private - `mod parser;` and `mod worker;` are not `pub mod` in
`src/lib.rs`, so nothing outside the crate can name or call them. The bug is a pure internal
arithmetic mistake (`height` computed from `output_size.width` instead of `output_size.height`)
in an internal parallelism heuristic - by design, the two backends it chooses between
(`Immediate` vs `Multithreaded`/`Rayon`) produce bit-identical decoded output, so there is no
externally observable symptom (no crash, no wrong pixel) reachable through the public API alone.
This is not a "two participants" finding in the sense of CONFIRMATION.md section 11 (no
peer/attacker input is being faked) - it is a single deterministic function computing the wrong
thing from a struct field, and section 11's own carve-out for a single external artifact applies:
a real JPEG file decoded through the real public `Decoder::decode()` entry point *is* the genuine
input, there is no "other side" to impersonate.

Because the only way to observe select_worker's actual decision is to call it directly, this
repro has two parts, both run against a pristine clone of the real crate:

1. `test_patch.diff` - adds a `#[cfg(test)] mod bug007_select_worker_tests` block directly to the
   end of `src/decoder.rs` with two `#[test]` functions that call `Decoder::select_worker`
   directly with `FrameInfo` values built from real width/height pairs (999x8 and 10x2000). This
   is the standard, idiomatic Rust way to unit-test a private function - not a stand-in for a
   fake participant.
2. `verify_dims.rs` - an example that exercises the *actual* public API
   (`Decoder::new`/`decode`/`info`) against the real JPEG fixture `fixtures/test_wide.jpg`
   (999x8, made with `convert -size 999x8 xc:gray`), proving those exact dimensions are what a
   real image handed to the real public entry point produces (i.e. what `select_worker` is
   really called with during a real decode at src/decoder.rs:610), and that decode correctness is
   completely unaffected by which worker backend the bug picks.

Run both with `just test` from this candidate's folder (see `../justfile`).
