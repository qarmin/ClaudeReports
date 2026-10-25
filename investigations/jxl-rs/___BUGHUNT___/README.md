# ___BUGHUNT___ workspace for jxl-rs

Session 2026-10-12 (14:49-16:49 Europe/Warsaw), base commit 7f539c8.

## Tooling that works here
- `cargo +nightly fuzz` (debug-assertions on) - the repo's own `jxl/fuzz` targets work as is.
- ASAN: `RUSTFLAGS="-Zsanitizer=address -Cdebug-assertions -Coverflow-checks" ASAN_OPTIONS=detect_leaks=0:allocator_may_return_null=1 cargo +nightly test --release -p jxl --target x86_64-unknown-linux-gnu -- --skip huge_image`; the big `compare_*` tests need ~3 minutes on an idle machine and look hung when the machine is busy with fuzzers.
- i686 works via `cargo zigbuild --target i686-unknown-linux-gnu` (zig cc), binaries run natively; aarch64 via `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build` + `qemu-aarch64 -L /usr/aarch64-linux-gnu`.
- wasm32-wasip1 + node 22 WASI works (`harnesses/replay32`), but node segfaults when stdout is redirected, so run one file per process and use the exit code.
- i686-unknown-linux-gnu does not link here (no 32-bit libgcc). miri was not used (SIMD intrinsics everywhere).

## harnesses/
Standalone cargo crates (path dependencies point at the repo checkout two levels up):
- `fuzz_formats` - cargo-fuzz crate (`cargo +nightly fuzz run --fuzz-dir . <target>`): `formats`, `chunked` (pixel format + options + chunking), `seek`, `seekdiff`, `chunkdiff`.
- `diff_formats` - output-format / orientation / premultiply differential (subcommands: default, `or`, `pm`, `try`, `ec`).
- `simdvar` - dumps decoded f32 per SIMD feature set for comparison.
- `replay32` - fuzz-corpus replay (native or wasm32-wasip1).
- `img_int` - replays inputs through jxl-image-rs-integration.

Candidate 001's fix was applied to a git worktree (`_reference/wt`, not shipped) while fuzzing continued, so the main tree stayed clean.
