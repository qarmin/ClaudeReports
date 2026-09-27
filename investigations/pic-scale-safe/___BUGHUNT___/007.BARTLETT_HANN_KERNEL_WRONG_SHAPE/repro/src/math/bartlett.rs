// Same trick as consts.rs: literally includes the real `src/math/bartlett.rs` from the cloned
// project, so `crate::math::bartlett::bartlett_hann` in main.rs is the *actual* project function
// (still `pub(crate)`, but "crate" now means this repro crate, which is why this works without
// modifying the source's visibility).
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../clone/src/math/bartlett.rs"
));
