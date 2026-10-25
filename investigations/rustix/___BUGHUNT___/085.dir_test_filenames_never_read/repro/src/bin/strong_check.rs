//! Same real scenario as `weak_check.rs` (same real `rustix::fs::Dir` public
//! API, same real temp directory with real files), but additionally checks
//! every entry name actually read back against the real set of filenames
//! that were created -- the assertion the upstream test is missing. This is
//! `patch.diff` applied to the test.
//!
//! Against a pristine rustix build this prints PASS. Against the mutated
//! build (`just mutate`) that `weak_check` cannot distinguish from correct,
//! this panics, proving the stronger check catches what the shipping test
//! misses.

use std::collections::HashSet;
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

    assert_eq!(entries, entries2);

    // The assertion the real test is missing: check against ground truth,
    // not just self-consistency.
    let real_names: HashSet<&str> = filenames.iter().map(String::as_str).collect();
    for name in entries.iter().chain(entries2.iter()) {
        assert!(
            name == "." || name == ".." || real_names.contains(name.as_str()),
            "unexpected directory entry name: {name:?} (not among the real files created)"
        );
    }

    println!("strong_check: PASS (entries == entries2, AND every name matches a real created file)");
}
