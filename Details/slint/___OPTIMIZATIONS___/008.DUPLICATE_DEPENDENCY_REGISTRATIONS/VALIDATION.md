# 08 - Validation of the duplicate-registration skip

Everything below was run with the patch applied to `internal/core/properties.rs`
(`dedupe_registrations.diff`), `rdebug` profile.

## Test suites

| Suite | Command | Result |
| --- | --- | --- |
| `i-slint-core` unit tests (property system, models, item tree) | `cargo test -p i-slint-core` | **291 passed**, 0 failed (+69 doc tests, +1 compile-fail) |
| Rust API integration tests (partial renderer, text layout cache, popups, layouts) | `cargo test -p slint` | **77 passed**, 0 failed |
| Language test cases through the interpreter (property invalidation, layouts, models) | `cd tests && cargo test -p test-driver-interpreter` | **818 passed**, 0 failed |
| The same cases through the Rust code generator | `cd tests && cargo test -p test-driver-rust` | **745 passed**, 0 failed |
| Screenshot comparison suite | `cd tests && cargo test -p test-driver-screenshots` | **could not run**: `softbuffer` fails to compile in this environment (25 pre-existing errors, unrelated to this change) |

The interpreter and Rust driver suites are the relevant ones for a dependency-tracking change:
they are exactly the cases that set a property and assert what recomputes.

## Rendering invalidation, checked directly

A missed dependency would show up as stale pixels: the renderer would not be told to redraw
when something it read changes. Checked with FemtoVG (Xvfb) and the deterministic `rects` scene,
which has no text and therefore no run-to-run noise:

```sh
perf_test --mode rects --count 2000 --frames 5 --scroll-step 37 --snapshot out.ppm
```

- baseline vs patched, after 5 frames of moving content: **byte-identical PPM**.
- the moved frame differs from the static (`--scroll-step 0`) frame, so content really did move
  and the comparison is not vacuous.

For the `scroll` scene (contains text, so it is not bit-deterministic): baseline vs patched
differ in 159 bytes of 3 302 400, while two runs of the *same baseline* binary differ in 291
bytes - the patch is well inside the scene's own noise.

## Numbers that also serve as validation

Allocation counts dropped by exactly the measured duplicate share (57.1% / 47.2% / 0.5% per
scene), which is what the skip is supposed to remove and nothing more.
