// Standalone repro for 021.RESIZE_EXACT_CATMULLROM_REFERENCE_IMAGE_DRIFT.
//
// `DynamicImage::resize_exact(.., FilterType::CatmullRom)` on `resize-with-alpha-original.png`
// (129x66 RGBA) down to 65x33 does not bit-exact-match the crate's own committed reference image
// `resize-with-alpha-original-half-size.png` - the same assertion `tests/regression.rs::resizing_with_catmul`
// makes (and fails) on a fully clean checkout.
use image::GenericImageView as _;

fn main() {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("images");

    let image = image::ImageReaderOptions::open(base.join("resize-with-alpha-original.png"))
        .unwrap()
        .decode()
        .unwrap();
    let expected =
        image::ImageReaderOptions::open(base.join("resize-with-alpha-original-half-size.png"))
            .unwrap()
            .decode()
            .unwrap();

    let (w, h) = image.dimensions();
    let (nw, nh) = (w.div_ceil(2), h.div_ceil(2));
    println!("source {w}x{h} -> {nw}x{nh}");

    let mut resizable = image.clone();
    resizable.resize_exact(nw, nh, image::imageops::FilterType::CatmullRom);

    let got = resizable.to_rgba8();
    let exp = expected.to_rgba8();
    assert_eq!((got.width(), got.height()), (exp.width(), exp.height()));

    let mut max_diff = 0i32;
    let mut n_diff = 0usize;
    let mut examples = vec![];
    for (p1, p2) in got.as_raw().chunks(4).zip(exp.as_raw().chunks(4)) {
        for c in 0..4 {
            let d = (p1[c] as i32 - p2[c] as i32).abs();
            if d > 0 {
                n_diff += 1;
                max_diff = max_diff.max(d);
                if examples.len() < 10 {
                    examples.push((p1.to_vec(), p2.to_vec(), d));
                }
            }
        }
    }

    let total_channels = (got.width() * got.height() * 4) as usize;
    println!(
        "differing channel values: {n_diff} of {total_channels} ({:.3}%), max |diff|={max_diff}",
        100.0 * n_diff as f64 / total_channels as f64
    );
    for (a, b, d) in &examples {
        println!("  got={a:?} expected={b:?} |diff|={d}");
    }

    if n_diff == 0 {
        println!("RESULT: bit-exact match (bug not reproduced on this build)");
    } else {
        println!("RESULT: reproduced - resize_exact/CatmullRom does not bit-exact-match the committed reference image");
    }

    assert_eq!(resizable, expected, "full DynamicImage equality (as the real test asserts) also fails");
}
