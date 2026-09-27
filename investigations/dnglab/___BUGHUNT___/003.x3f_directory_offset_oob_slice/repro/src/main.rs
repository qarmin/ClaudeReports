// Standalone reproduction: X3F decoder panics on a truncated file via an out-of-range slice.
//
// This is a genuinely malformed/truncated raw file (5 bytes: the "FOVb" X3F magic plus one
// trailing byte) handed to rawler's real, public, top-level entry point `rawler::get_decoder` -
// the exact first step every rawler-based tool (including the `dnglab` CLI) takes when opening
// any raw file. No internals of the suspect function are called directly, and no second
// participant needs to be faked - this is a single external artifact (a file) parsed by its
// real public entry point, per CONFIRMATION.md section 11.
//
// This exact 5-byte input was found by cargo-fuzz's fuzz_get_decoder target within its first
// ~25 minutes of unseeded fuzzing (no starting corpus) - see ../../README.md.

use rawler::get_decoder;
use rawler::rawsource::RawSource;

fn main() {
    // "FOVb" (X3F magic) + one trailing byte - not even a full directory-offset footer.
    let data: &[u8] = b"FOVb\n";

    let out_path = std::env::temp_dir().join("bughunt_003_x3f_truncated.x3f");
    std::fs::write(&out_path, data).expect("write x3f file");
    println!("wrote {} bytes to {:?}", data.len(), out_path);

    let source = RawSource::new(&out_path).expect("open raw source");
    match get_decoder(&source) {
        Ok(_) => println!("get_decoder returned Ok (bug not present)"),
        Err(e) => println!("get_decoder returned an error, no panic (bug not present): {e}"),
    }
}
