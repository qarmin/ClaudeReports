// Standalone reproduction for candidate 051: `Encoding::swap_last_path_tags` underflows
// `path_tags.len() - 2` when the path-tag stream has fewer than 2 entries.
//
// This exercises the real, public `vello_encoding` API exactly as any external consumer of the
// (published) `vello_encoding` crate could: `Encoding::new()` + the public `path_tags` field +
// the public `swap_last_path_tags` method, with no internal/private access and no participant
// being faked - `swap_last_path_tags` is a plain library function, not a peer/wire boundary, so
// calling it directly *is* the real trigger path.

use vello_encoding::{Encoding, PathTag};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let case = args.get(1).map(String::as_str).unwrap_or("0");

    match case {
        "0" => {
            // A freshly constructed Encoding has 0 path tags.
            let mut encoding = Encoding::new();
            println!("case len=0: path_tags.len() before = {}", encoding.path_tags.len());
            encoding.swap_last_path_tags();
            println!("case len=0: no panic, path_tags.len() after = {}", encoding.path_tags.len());
        }
        "1" => {
            // Only a single path tag pushed.
            let mut encoding = Encoding::new();
            encoding.path_tags.push(PathTag::TRANSFORM);
            println!("case len=1: path_tags.len() before = {}", encoding.path_tags.len());
            encoding.swap_last_path_tags();
            println!("case len=1: no panic, path_tags.len() after = {}", encoding.path_tags.len());
        }
        "2" => {
            // Sanity check: the intended len>=2 case still swaps correctly.
            let mut encoding = Encoding::new();
            encoding.path_tags.push(PathTag::PATH);
            encoding.path_tags.push(PathTag::TRANSFORM);
            encoding.swap_last_path_tags();
            assert_eq!(encoding.path_tags[0].0, PathTag::TRANSFORM.0);
            assert_eq!(encoding.path_tags[1].0, PathTag::PATH.0);
            println!("case len=2: swapped correctly, tags = [{}, {}]", encoding.path_tags[0].0, encoding.path_tags[1].0);
        }
        other => panic!("unknown case {other}"),
    }
    println!("ALL OK");
}
