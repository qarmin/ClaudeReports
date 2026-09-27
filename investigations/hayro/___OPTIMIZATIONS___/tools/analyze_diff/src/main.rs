//! Analyzes the magnitude of a hayro-tests `[reference | diff | actual]` diff
//! triptych PNG: how many pixels differ and by how much (max channel
//! deviation). Useful for characterizing whether a change that trips an
//! exact-pixel snapshot test is a real regression or a tiny, arguably
//! acceptable rounding difference. Usage: `analyze_diff <path/to/diff.png>`.
use image::GenericImageView;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let img = image::open(&path).unwrap();
    let (w3, h) = img.dimensions();
    let w = w3 / 3;
    let mut max_r = 0u8;
    let mut count = 0u64;
    let mut total = 0u64;
    for y in 0..h {
        for x in w..2 * w {
            total += 1;
            let px = img.get_pixel(x, y);
            let r = px.0[0];
            if r > 0 {
                count += 1;
                max_r = max_r.max(r);
            }
        }
    }
    println!(
        "{path}: {w}x{h}, {count}/{total} pixels differ, max channel deviation = {max_r}"
    );
}
