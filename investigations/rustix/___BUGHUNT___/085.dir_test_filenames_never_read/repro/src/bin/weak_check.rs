//! Replicates the real, currently-shipping `test_dir_seek` test
//! (tests/fs/dir.rs) exactly as written upstream: it builds `filenames` but
//! never reads it back, and only ever compares two listings
//! (`entries`/`entries2`) of the SAME real directory, read through the real
//! `rustix::fs::Dir` public API, against each other.
//!
//! Run against a deliberately mutated build of rustix (see `../mutant.patch`,
//! applied by this folder's `just mutate`) this still prints PASS, proving
//! the weak check cannot see a bug that corrupts every parsed name the same
//! way on every read.

use std::io::Write as _;

fn main() {
    let tempdir = tempfile::tempdir().unwrap();

    let count = 200;
    let prefix = "file_with_a_very_long_name_to_make_sure_that_we_fill_up_the_buffer";
    let test_string = "This is a test string.";
    let mut filenames = Vec::<String>::with_capacity(count);
    for i in 0..count {
        let filename = format!("{prefix}-{i}.txt");
        let mut file = std::fs::File::create(tempdir.path().join(&filename)).unwrap();
        filenames.push(filename);
        file.write_all(test_string.as_bytes()).unwrap();
    }

    let t = rustix::fs::open(
        tempdir.path(),
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .unwrap();

    let mut dir = rustix::fs::Dir::read_from(&t).unwrap();

    for _ in 0..count / 2 {
        dir.read().unwrap().unwrap();
    }
    let offset: i64 = dir.read().unwrap().unwrap().offset();

    let mut entries = Vec::new();
    while let Some(entry) = dir.read() {
        let entry = entry.unwrap();
        entries.push(entry.file_name().to_string_lossy().into_owned());
    }
    assert!(entries.len() >= count / 2);

    dir.seek(offset).unwrap();

    let mut entries2 = Vec::new();
    while let Some(entry) = dir.read() {
        let entry = entry.unwrap();
        entries2.push(entry.file_name().to_string_lossy().into_owned());
    }

    // This is the ONLY check the real, currently-shipping test performs.
    assert_eq!(entries, entries2);

    // `filenames` was built above (exactly like upstream), but is never
    // read here, mirroring the real test's `collection_is_never_read` gap.
    let _ = &filenames;

    println!("weak_check: PASS (entries == entries2) -- but never checked against the real filenames created, so a bug that corrupts every parsed name identically on every read is invisible to this check");
}
