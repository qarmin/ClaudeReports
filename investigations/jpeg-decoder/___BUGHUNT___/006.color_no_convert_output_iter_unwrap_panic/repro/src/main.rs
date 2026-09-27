// Standalone reproduction for candidate 006: color_no_convert_output_iter_unwrap_panic
// (GitHub issue https://github.com/image-rs/jpeg-decoder/issues/289).
//
// Drives the crate's real public API (`jpeg_decoder::Decoder::decode`) on a real, on-disk,
// standard 4:2:0 chroma-subsampled baseline JPEG file (subsampled_420.jpg, copied from this
// project's own tests/reftest/images/extraneous-data.jpg fixture) - a malformed/adversarial file
// handed to a decoder's public entry point is the genuine external input here, so a
// single-process repro is the right shape per CONFIRMATION.md section 11 (there is no second
// participant to fake).
//
// The only thing making this repro's input "adversarial" relative to typical library usage is
// the caller opting into `ColorTransform::None` (documented: "No transform should be applied and
// the data is returned as-is") - a legitimate, supported configuration for callers who want raw
// per-component samples. Once that's configured, *any* baseline JPEG with chroma subsampling
// (h_max/v_max > 1) - not a specially crafted file - is enough to hit the panic.

fn main() {
    let bytes = include_bytes!("../subsampled_420.jpg");

    let result = std::panic::catch_unwind(|| {
        let mut decoder = jpeg_decoder::Decoder::new(std::io::Cursor::new(&bytes[..]));
        decoder.set_color_transform(jpeg_decoder::ColorTransform::None);
        decoder.decode()
    });

    match result {
        Ok(Ok(pixels)) => {
            println!(
                "did not reproduce - decode() completed normally, {} bytes",
                pixels.len()
            );
        }
        Ok(Err(e)) => {
            println!("did not reproduce - decode() returned an Err instead of panicking: {:?}", e);
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - decode() panicked: {}", msg);
            std::process::exit(1);
        }
    }
}
