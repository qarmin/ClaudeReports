// Not a transcription: this literally pulls in the real project file at build time, so the
// repro always tracks whatever `gaussian.rs`/`consts.rs` currently say in `../clone` (pristine
// before `just apply`, patched after) rather than a hand-copied snapshot that could drift.
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../clone/src/math/consts.rs"
));
