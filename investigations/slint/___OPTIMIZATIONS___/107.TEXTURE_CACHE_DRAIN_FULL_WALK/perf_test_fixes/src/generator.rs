use std::fmt::Write;

use slint::SharedString;

use crate::RecordItem;

/// Minimum number of characters of lorem ipsum per generated line.
pub const LINE_LEN: usize = 100;

const WORDS: [&str; 40] = [
    "lorem",
    "ipsum",
    "dolor",
    "sit",
    "amet",
    "consectetur",
    "adipiscing",
    "elit",
    "sed",
    "eiusmod",
    "tempor",
    "incididunt",
    "labore",
    "dolore",
    "magna",
    "aliqua",
    "enim",
    "minim",
    "veniam",
    "quis",
    "nostrud",
    "exercitation",
    "ullamco",
    "laboris",
    "nisi",
    "aliquip",
    "commodo",
    "consequat",
    "duis",
    "irure",
    "reprehenderit",
    "voluptate",
    "velit",
    "esse",
    "cillum",
    "fugiat",
    "nulla",
    "pariatur",
    "excepteur",
    "occaecat",
];

/// Appends one line of at least [`LINE_LEN`] characters, unique per `index`: the
/// index prefix plus a per-line rotation of the word list keeps every line's
/// glyph run different, so text/glyph caches cannot serve them all from one entry.
pub fn push_line(out: &mut String, index: usize) {
    push_line_of(out, index, LINE_LEN);
}

/// Same, with an explicit target length. A paragraph is the unit of lazy shaping, so a document
/// made of a few very long lines behaves completely differently from one made of many short ones -
/// minified JSON, CSV rows and log lines are all the former.
pub fn push_line_of(out: &mut String, index: usize, line_len: usize) {
    let start = out.len();
    let _ = write!(out, "{index:07}.");

    let mut word = index.wrapping_mul(7);
    while out.len() - start < line_len {
        out.push(' ');
        let w = WORDS[word % WORDS.len()];
        if word.is_multiple_of(11) {
            let mut chars = w.chars();
            if let Some(first) = chars.next() {
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
            }
        } else {
            out.push_str(w);
        }
        word += 1;
    }

    let _ = write!(out, " #{}", index.wrapping_mul(2654435761) % 100_000);
    out.push('\n');
}

/// Loads a generated asset by file name. Panics rather than falling back: a benchmark that
/// silently measured a different image than the one asked for would be worse than one that stops.
pub fn load_asset(name: &str) -> slint::Image {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets").join(name);
    slint::Image::load_from_path(&path).unwrap_or_else(|_| {
        panic!("cannot load {}; run `cargo run --bin make_assets` first", path.display())
    })
}

pub fn make_record(index: usize) -> RecordItem {
    let mut second = String::with_capacity(48);
    let mut word = index.wrapping_mul(13);
    while second.len() < 40 {
        if !second.is_empty() {
            second.push(' ');
        }
        second.push_str(WORDS[word % WORDS.len()]);
        word += 1;
    }
    let _ = write!(second, " #{}", index.wrapping_mul(2654435761) % 100_000);

    RecordItem {
        first: format!("item {index:06}").into(),
        second: second.into(),
        third: format!("{}.{:02} MB", index % 977, index % 100).into(),
    }
}

pub fn selection_info(text: &str, cursor: i32, anchor: i32) -> SharedString {
    let cursor = clamp_to_boundary(text, cursor);
    let anchor = clamp_to_boundary(text, anchor);
    let (start, end) = (cursor.min(anchor), cursor.max(anchor));

    if start == end {
        return format!("no selection - cursor at byte {start} of {}", text.len()).into();
    }

    let selected = &text[start..end];
    let lines = selected.bytes().filter(|byte| *byte == b'\n').count() + 1;
    format!("selected {lines} line(s), {} bytes (offsets {start}..{end})", selected.len()).into()
}

fn clamp_to_boundary(text: &str, offset: i32) -> usize {
    let mut offset = (offset.max(0) as usize).min(text.len());
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn lines_are_long_enough_and_all_different() {
        let mut text = String::new();
        for index in 0..500 {
            push_line(&mut text, index);
        }

        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 500);
        assert!(
            lines.iter().all(|line| line.len() >= LINE_LEN),
            "every line has at least LINE_LEN chars"
        );
        assert_eq!(lines.iter().collect::<HashSet<_>>().len(), 500, "no two lines are identical");
    }

    #[test]
    fn records_differ_per_index() {
        let first = make_record(0);
        let second = make_record(1);
        assert_ne!(first.first, second.first);
        assert_ne!(first.second, second.second);
        assert_ne!(first.third, second.third);
    }

    #[test]
    fn selection_spanning_three_lines() {
        let text = "aaa\nbbb\nccc\n";
        let info = selection_info(text, 0, 9);
        assert!(info.contains("selected 3 line(s)"), "{info}");
        assert!(info.contains("9 bytes"), "{info}");
    }

    #[test]
    fn empty_selection_and_out_of_range_offsets() {
        let text = "aaa\nbbb";
        assert!(selection_info(text, 4, 4).starts_with("no selection"));
        assert!(selection_info(text, -5, 9999).contains("selected 2 line(s)"));
    }

    #[test]
    fn offsets_are_clamped_to_char_boundaries() {
        let text = "zażółć\ngęślą";
        // 2 is inside the 2-byte `ż`, so it must be clamped instead of panicking.
        let info = selection_info(text, 2, 3);
        assert!(info.starts_with("no selection"), "{info}");
    }
}

/// A procedurally generated RGBA image. Generated rather than loaded so the scene needs no asset
/// file and so `--image-distinct` can produce genuinely different sources cheaply.
pub fn make_image(seed: u32, size: u32) -> slint::Image {
    let mut buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(size, size);
    let width = buffer.width();
    for (index, pixel) in buffer.make_mut_slice().iter_mut().enumerate() {
        let x = index as u32 % width;
        let y = index as u32 / width;
        // Cheap, non-uniform, and different per seed, so nothing can dedupe the buffers.
        *pixel = slint::Rgba8Pixel {
            r: (x.wrapping_mul(7).wrapping_add(seed.wrapping_mul(31))) as u8,
            g: (y.wrapping_mul(5).wrapping_add(seed.wrapping_mul(17))) as u8,
            b: ((x ^ y).wrapping_add(seed)) as u8,
            a: 255,
        };
    }
    slint::Image::from_rgba8(buffer)
}

/// Writes (or reuses) `count` genuinely distinct on-disk PNG files under `assets/gallery/` and
/// loads each with `Image::load_from_path`, so every one gets a real, persistent
/// `ImageCacheKey::Path` - unlike `make_image`'s in-memory buffers, which get
/// `ImageCacheKey::Invalid` (see `ImageCacheKey::new`'s doc comment: "no image cache key can be
/// created ... programmatically created images") and so never enter a renderer's per-source GPU
/// texture cache at all, recreating their texture from scratch every single frame regardless of
/// any cache. That made `--image-distinct`/`--image-unique` unsuitable, until this function, for
/// measuring anything about that cache (e.g. `TextureCache::drain()`'s per-frame walk in
/// `i-slint-renderer-femtovg`) - only for measuring the uncached-path cost itself. Files are
/// skipped if already present (deterministic content keyed by index+size), so repeated
/// paired-comparison runs only pay the encode cost once.
pub fn make_unique_gallery_images(count: usize, size: u32) -> Vec<slint::Image> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets").join("gallery");
    std::fs::create_dir_all(&dir).expect("create perf_test gallery asset dir");
    (0..count)
        .map(|index| {
            let path = dir.join(format!("g{size}_{index}.png"));
            if !path.exists() {
                let seed = index as u32;
                let buffer = image::ImageBuffer::from_fn(size, size, |x, y| {
                    image::Rgba([
                        (x.wrapping_mul(7).wrapping_add(seed.wrapping_mul(31))) as u8,
                        (y.wrapping_mul(5).wrapping_add(seed.wrapping_mul(17))) as u8,
                        ((x ^ y).wrapping_add(seed)) as u8,
                        255,
                    ])
                });
                buffer.save(&path).unwrap_or_else(|e| {
                    panic!("cannot write gallery image {}: {e}", path.display())
                });
            }
            slint::Image::load_from_path(&path)
                .unwrap_or_else(|_| panic!("cannot load gallery image {}", path.display()))
        })
        .collect()
}
