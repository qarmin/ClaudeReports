//! Does the image cache actually keep an image?
//!
//! `IMAGE_CACHE` is a `clru` cache bounded at 5 MB of *decoded* bytes, and an insert that exceeds
//! the bound is discarded (`put_with_weight(..).ok()`). A 1920x1080 RGBA8 image decodes to 8.3 MB,
//! so on that reading it can never be stored. This measures it instead of arguing about it: load,
//! drop, load again. A cached image comes back instantly; an uncached one is decoded again.

use std::path::Path;
use std::time::Instant;

use slint::Image;

fn timed(path: &Path) -> (f64, Option<Image>) {
    let start = Instant::now();
    let image = Image::load_from_path(path).ok();
    (start.elapsed().as_secs_f64() * 1000.0, image)
}

fn decoded_bytes(image: &Image) -> f64 {
    let size = image.size();
    // Everything here decodes to RGB8 or RGBA8; 4 bytes is the upper bound and what the cache
    // weighs for an image with an alpha channel.
    size.width as f64 * size.height as f64 * 3.0 / (1024.0 * 1024.0)
}

// FNV-1a over the decoded pixel bytes, used to confirm a cache hit returns bit-identical pixels
// to a fresh decode rather than just being fast.
fn pixel_hash(image: &Image) -> u64 {
    let buffer = image.to_rgba8().expect("decoded image should be convertible to rgba8");
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in buffer.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn probe(name: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets").join(name);
    if !path.exists() {
        println!("{name}: missing - run `cargo run --bin make_assets` first");
        return;
    }

    let (cold, first) = timed(&path);
    let Some(first) = first else {
        println!("{name}: failed to load");
        return;
    };
    let mb = decoded_bytes(&first);
    let size = first.size();
    let cold_hash = pixel_hash(&first);

    // Still holding a reference: this is the deduplication path and must be instant either way.
    let (deduped, second) = timed(&path);
    drop(second);
    drop(first);

    // Nothing holds it now, so only the cache can serve this one.
    let (after_drop, third) = timed(&path);
    let after_drop_hash = pixel_hash(&third.expect("second load should also succeed"));

    let megapixels = size.width as f64 * size.height as f64 / 1e6;
    let file_mb = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0) as f64 / (1024.0 * 1024.0);
    println!(
        "{name:<20} {mb:>5.1} MB rgb8   cold {cold:>7.2} ms   while held {deduped:>6.2} ms   \
         after drop {after_drop:>7.2} ms   {:>6.1} Mpx/s  {:>6.1} MB/s in   {}   {}",
        megapixels / (cold / 1000.0),
        file_mb / (cold / 1000.0),
        if after_drop > cold / 4.0 { "RE-DECODED" } else { "cached" },
        if cold_hash == after_drop_hash { "pixels match" } else { "PIXEL MISMATCH" }
    );
}

/// Same load/hold/drop/reload sequence as `probe()`, but for an SVG source (parse cost, not
/// decoded-pixel bytes) - added for candidate 081.IMAGE_CACHE_WEAK_SVG_DEDUP, which adds a weak
/// side table so a still-held `Svg`-backed `Image` dedupes even once its parsed tree is heavier
/// than `IMAGE_CACHE`'s bound (see `internal/core/graphics/image/svg.rs`'s
/// `source_size*16+8192` weight formula).
fn probe_svg(name: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets").join(name);
    if !path.exists() {
        println!("{name}: missing - run `cargo run --bin make_assets` first");
        return;
    }

    let file_kb = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0) as f64 / 1024.0;
    let approx_weight_mb = (file_kb * 1024.0 * 16.0 + 8192.0) / (1024.0 * 1024.0);

    let (cold, first) = timed(&path);
    let Some(first) = first else {
        println!("{name}: failed to load");
        return;
    };
    let cold_hash = pixel_hash(&first);

    // Still holding a reference: this is the deduplication path this candidate targets.
    let (while_held, second) = timed(&path);
    drop(second);
    drop(first);

    // Nothing holds it now - a weak side table cannot and should not help here either.
    let (after_drop, third) = timed(&path);
    let after_drop_hash = pixel_hash(&third.expect("second load should also succeed"));

    println!(
        "{name:<20} {file_kb:>7.1} KB src  ~{approx_weight_mb:>6.1} MB weight   cold {cold:>7.3} ms   \
         while held {while_held:>7.3} ms   after drop {after_drop:>7.3} ms   {}   {}",
        if while_held > cold / 4.0 { "RE-PARSED while held" } else { "deduped while held" },
        if cold_hash == after_drop_hash { "pixels match" } else { "PIXEL MISMATCH" }
    );
}

fn main() {
    println!("image cache bound: 5 MB of decoded bytes (core/graphics/image/cache.rs)\n");
    for name in [
        "small.png",
        "medium.png",
        "medium.jpg",
        "large.png",
        "large.jpg",
        "large_alpha.png",
    ] {
        probe(name);
    }

    println!("\nlocating the cliff (RGB8, 3 bytes per pixel):");
    for name in [
        "smooth_medium.png",
        "smooth_medium.jpg",
        "smooth_large.png",
        "smooth_large.jpg",
        "cliff_800x600.png",
        "cliff_1152x864.png",
        "cliff_1280x800.png",
        "cliff_1360x850.png",
        "cliff_1440x900.png",
        "cliff_1600x1000.png",
    ] {
        probe(name);
    }

    println!("\nSVG parse dedup (candidate 081.IMAGE_CACHE_WEAK_SVG_DEDUP):");
    for name in ["shapes.svg", "large_shapes.svg"] {
        probe_svg(name);
    }
}
