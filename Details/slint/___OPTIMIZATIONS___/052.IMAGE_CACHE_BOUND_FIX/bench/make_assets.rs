//! Generates the image assets used by the `images` scene.
//!
//! Run with `cargo run --profile rdebug --bin make_assets`. The output is deterministic, so the
//! files can be regenerated at any time instead of being committed - a benchmark asset nobody can
//! reproduce is a benchmark nobody can check.

use std::path::Path;

use image::{ImageBuffer, Rgb, Rgba};

/// A smooth gradient - the compressible end of the range. High-frequency noise is the worst case for
/// JPEG and the best case for PNG (little to inflate), so decode throughput measured on one pattern
/// says nothing about the other.
fn smooth(x: u32, y: u32, w: u32, h: u32) -> [u8; 3] {
    [(255 * x / w.max(1)) as u8, (255 * y / h.max(1)) as u8, ((x + y) * 127 / (w + h).max(1)) as u8]
}

/// A pattern with detail at several scales, so a scaled draw cannot be mistaken for a flat fill and
/// a JPEG of it does not compress to nothing.
fn sample(x: u32, y: u32, seed: u32) -> [u8; 3] {
    let fine = ((x ^ y).wrapping_mul(2654435761) >> 16) as u8;
    let coarse = (((x / 37) + (y / 23)).wrapping_mul(97)) as u8;
    [
        fine.wrapping_add(seed as u8),
        coarse.wrapping_add((x >> 3) as u8),
        ((x.wrapping_mul(3)) ^ (y.wrapping_mul(5))) as u8,
    ]
}

fn write_rgb(dir: &Path, name: &str, w: u32, h: u32, quality: Option<u8>) {
    let buffer = ImageBuffer::from_fn(w, h, |x, y| Rgb(sample(x, y, 0)));
    let path = dir.join(name);
    match quality {
        None => buffer.save(&path).unwrap(),
        Some(q) => {
            let file = std::fs::File::create(&path).unwrap();
            let mut encoder =
                image::codecs::jpeg::JpegEncoder::new_with_quality(std::io::BufWriter::new(file), q);
            encoder.encode(buffer.as_raw(), w, h, image::ExtendedColorType::Rgb8).unwrap();
        }
    }
    report(&path);
}

/// `alpha`: `None` writes a fully opaque alpha channel - the case that should be able to take a copy
/// path and today does not (plan item A3). `Some(f)` varies it.
fn write_rgba(dir: &Path, name: &str, w: u32, h: u32, alpha: Option<fn(u32, u32) -> u8>) {
    let buffer = ImageBuffer::from_fn(w, h, |x, y| {
        // Same seed as `write_rgb`, so `<name>.png` and `<name>_opaque.png` hold identical
        // pixels and differ only in having an alpha channel - which is the whole comparison.
        let [r, g, b] = sample(x, y, 0);
        Rgba([r, g, b, alpha.map_or(255, |f| f(x, y))])
    });
    let path = dir.join(name);
    buffer.save(&path).unwrap();
    report(&path);
}

fn write_svg(dir: &Path, name: &str, paths: usize) {
    let mut svg = String::from(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" width="512" height="512">"#,
    );
    for i in 0..paths {
        let (a, b) = ((i * 37 % 512) as i32, (i * 53 % 512) as i32);
        svg.push_str(&format!(
            r##"<path d="M{a} {b} l40 15 l-15 40 z" fill="#{:02x}{:02x}{:02x}"/>"##,
            (i * 7) as u8,
            (i * 13) as u8,
            (i * 29) as u8
        ));
    }
    svg.push_str("</svg>");
    let path = dir.join(name);
    std::fs::write(&path, svg).unwrap();
    report(&path);
}

fn report(path: &Path) {
    let bytes = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    println!("{:<28} {:>10} bytes", path.file_name().unwrap().to_string_lossy(), bytes);
}

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    std::fs::create_dir_all(&dir).unwrap();

    for (name, w, h) in [("small", 64, 64), ("medium", 512, 512), ("large", 1920, 1080)] {
        write_rgb(&dir, &format!("{name}.png"), w, h, None);
        write_rgb(&dir, &format!("{name}.jpg"), w, h, Some(85));
        write_rgba(&dir, &format!("{name}_opaque.png"), w, h, None);
        write_rgba(&dir, &format!("{name}_alpha.png"), w, h, Some(|x, y| ((x + y) % 256) as u8));
    }

    // The same resolution in a smooth pattern, so decode throughput can be read against content
    // rather than against one arbitrary image.
    for (name, w, h) in [("smooth_medium", 512, 512), ("smooth_large", 1920, 1080)] {
        let buffer = ImageBuffer::from_fn(w, h, |x, y| Rgb(smooth(x, y, w, h)));
        buffer.save(dir.join(format!("{name}.png"))).unwrap();
        report(&dir.join(format!("{name}.png")));
        let file = std::fs::File::create(dir.join(format!("{name}.jpg"))).unwrap();
        image::codecs::jpeg::JpegEncoder::new_with_quality(std::io::BufWriter::new(file), 85)
            .encode(buffer.as_raw(), w, h, image::ExtendedColorType::Rgb8)
            .unwrap();
        report(&dir.join(format!("{name}.jpg")));
    }

    // A series straddling the 5 MB cache bound, to locate the cliff in resolution terms rather
    // than in bytes - "how big may my images be" is the question an application author has.
    for (w, h) in [(800, 600), (1152, 864), (1280, 800), (1360, 850), (1440, 900), (1600, 1000)] {
        write_rgb(&dir, &format!("cliff_{w}x{h}.png"), w, h, None);
    }

    // Nine-slice: a frame with distinct corners so a wrong slice is visible.
    write_rgba(&dir, "nine_slice.png", 96, 96, Some(|x, y| if (24..72).contains(&x) && (24..72).contains(&y) { 128 } else { 255 }));

    write_svg(&dir, "shapes.svg", 400);

    println!("\nassets written to {}", dir.display());
}
