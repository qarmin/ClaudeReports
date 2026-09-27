// Standalone reproduction for candidate 049 (AFFINE_REFLECT_ZERO_DIRECTION_NAN).
//
// Drives kurbo's real public entry point (`Affine::reflect`) directly with a zero-length
// direction vector - a real, single-artifact geometric input constructed the same way any caller
// would call this API (no second participant is being faked here; there is nothing analogous to
// a peer/wire input in this finding - see CONFIRMATION.md section 11).
//
// Root cause: `Affine::reflect` (kurbo/src/affine.rs) computes
// `Vec2 { x: direction.y, y: -direction.x }.normalize()` unconditionally. `Vec2::normalize` is
// documented (kurbo/src/vec2.rs) to produce NaN when the vector's magnitude is 0. When `direction`
// itself has zero length, the perpendicular vector built from it is also zero-length, so
// `normalize()` yields NaN, and every coefficient of the returned `Affine` becomes NaN. Unlike the
// analogous `Affine::inverse`/`TranslateScale::inverse` (which both explicitly document "Produces
// NaN values when ..." on their own doc comments), `reflect`'s doc comment said nothing about this
// precondition before the fix - a caller reading only the public docs had no signal that a
// zero-length `direction` is unsupported.
//
// This is a documentation-gap finding, not a behavioral one: there is no well-defined "correct"
// reflection for a degenerate (zero-length) direction vector, so the fix documents the NaN
// footgun (matching the project's own existing convention for this exact situation elsewhere in
// the same file) rather than inventing new fallback behavior. Accordingly this repro checks two
// independent things against the same clone:
//   1. The runtime behavior (NaN coefficients) - this is real, and does NOT change after the fix;
//      it is the fact the finding is about, not something meant to be papered over.
//   2. Whether the crate's own doc comment for `Affine::reflect` documents this NaN behavior -
//      this DOES change after `patch.diff` is applied, and is what "fixed" actually means here.
//
// Exit code convention: 0 = fix confirmed present (doc mentions NaN) AND behavior still correctly
// produces NaN (sanity: the underlying math wasn't silently changed). Non-zero = the doc is still
// missing the precondition (matches the pristine, pre-patch tree).

use kurbo::{Affine, Point};

fn main() {
    // 1. Runtime behavior: reproduce the exact repro from the finding.
    let aff = Affine::reflect((1.0, 0.0), (0.0, 0.0));
    let coeffs = aff.as_coeffs();
    println!("Affine::reflect((1.0,0.0),(0.0,0.0)).as_coeffs() = {coeffs:?}");
    let p = aff * Point::new(2.0, 2.0);
    println!("aff * Point::new(2.0, 2.0) = {p:?}");
    let all_nan = coeffs.iter().all(|c| c.is_nan());
    if all_nan {
        println!("[behavior] confirmed: zero-length direction produces an all-NaN Affine (expected, documented behavior)");
    } else {
        println!("[behavior] UNEXPECTED: coefficients are not all NaN - underlying math changed unexpectedly: {coeffs:?}");
    }

    // 2. Documentation check: does the crate's own source doc-comment for `reflect` mention NaN?
    // Reads the same source file this binary was just compiled against (via the path dependency
    // into ../clone), so "before" (pristine clone) and "after" (patched clone) give different
    // answers across the two `cargo run` invocations in this candidate's justfile.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let affine_rs_path = format!("{manifest_dir}/../clone/kurbo/src/affine.rs");
    let doc_documents_nan = match std::fs::read_to_string(&affine_rs_path) {
        Ok(src) => {
            // Find the doc comment block directly above `pub fn reflect`.
            if let Some(fn_pos) = src.find("pub fn reflect(") {
                let doc_block = &src[..fn_pos];
                let doc_start = doc_block.rfind("/// Create an affine transform that represents reflection").unwrap_or(0);
                let doc_text = &doc_block[doc_start..];
                doc_text.contains("NaN")
            } else {
                eprintln!("could not find `pub fn reflect(` in {affine_rs_path} - is the clone set up correctly?");
                false
            }
        }
        Err(e) => {
            eprintln!("could not read {affine_rs_path}: {e}");
            false
        }
    };

    if doc_documents_nan {
        println!("[docs] `Affine::reflect`'s doc comment documents the zero-length-direction NaN footgun - FIXED");
    } else {
        println!("[docs] `Affine::reflect`'s doc comment does NOT document the zero-length-direction NaN footgun - REPRODUCED (matches pristine tree)");
    }

    if all_nan && doc_documents_nan {
        println!("did not reproduce - behavior is the documented NaN footgun, and it IS documented");
        std::process::exit(0);
    } else if all_nan && !doc_documents_nan {
        eprintln!("REPRODUCED - undocumented NaN footgun: zero-length direction silently produces NaN with no doc warning");
        std::process::exit(1);
    } else {
        eprintln!("REPRODUCED (different symptom) - underlying NaN-producing behavior itself changed unexpectedly, see [behavior] line above");
        std::process::exit(1);
    }
}
