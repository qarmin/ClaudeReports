//! Reproduces panics in `vello_common::pico_svg::PicoSvg::load` on malformed, untrusted
//! SVG attribute text, via the crate's real public `PicoSvg::load` entry point.
//!
//! Three distinct trigger sites are checked (all reachable purely by choosing the SVG text
//! handed to the real public API - no internal scaffolding, no second participant to fake):
//! 1. A malformed `viewBox` attribute (not a valid float in one of its 4 fields) -
//!    `f64::from_str(s).unwrap()` in `PicoSvg::load` (pico_svg.rs line 94).
//! 2. A `matrix(...)` transform attribute with a non-float argument - `.expect(...)` in
//!    `parse_transform` (pico_svg.rs line 242).
//! 3. A `matrix(...)` transform attribute with the wrong argument count (must be exactly 6) -
//!    a second `.expect(...)` in `parse_transform` (pico_svg.rs line 245).

use vello_common::pico_svg::PicoSvg;

fn try_load(label: &str, svg: &str) -> bool {
    let result = std::panic::catch_unwind(|| PicoSvg::load(svg, 1.0));
    match result {
        Ok(Ok(_)) => {
            println!("[{label}] did not reproduce - PicoSvg::load returned Ok");
            false
        }
        Ok(Err(e)) => {
            println!("[{label}] did not reproduce - PicoSvg::load returned Err: {e}");
            false
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| panic_payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("[{label}] REPRODUCED - panicked: {msg}");
            true
        }
    }
}

fn main() {
    let mut any_reproduced = false;

    // 1. Malformed viewBox: non-numeric token in one of the 4 whitespace-separated fields.
    let bad_viewbox = r#"<svg viewBox="0 0 100 NaNpx"><path d="M0 0 L10 10"/></svg>"#;
    any_reproduced |= try_load("viewBox non-float token", bad_viewbox);

    // 2. matrix(...) with a non-float argument.
    let bad_matrix_nonfloat =
        r#"<svg><g transform="matrix(1,0,0,1,0,notanumber)"><path d="M0 0 L10 10"/></g></svg>"#;
    any_reproduced |= try_load("matrix non-float argument", bad_matrix_nonfloat);

    // 3. matrix(...) with the wrong argument count (5 instead of 6).
    let bad_matrix_argcount =
        r#"<svg><g transform="matrix(1,0,0,1,0)"><path d="M0 0 L10 10"/></g></svg>"#;
    any_reproduced |= try_load("matrix wrong argument count", bad_matrix_argcount);

    if any_reproduced {
        std::process::exit(1);
    } else {
        println!("no reproduction - all inputs handled gracefully");
    }
}
