//! Times the parley stages Slint runs for every paragraph of a document, with the same style
//! setup as `i-slint-core`'s `LayoutWithoutLineBreaksBuilder` for `wrap: no-wrap`:
//!
//!   1. `RangedBuilder::build`  - text analysis (ICU line/word segmentation) + shaping
//!   2. `Layout::break_all_lines` - line breaking, which no-wrap text still needs for alignment
//!   3. keeping the `Layout` alive - what makes a document cost ~5 KB per line
//!
//! Run: cargo run --release [lines]

use std::time::Instant;

use parley::{FontContext, LayoutContext, StyleProperty};

/// Exact live-bytes accounting: RSS deltas are useless here because the allocator reuses the
/// memory freed by the previous variant.
mod live {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicI64, Ordering};

    pub static LIVE: AtomicI64 = AtomicI64::new(0);

    pub struct Tracking;

    unsafe impl GlobalAlloc for Tracking {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            LIVE.fetch_add(layout.size() as i64, Ordering::Relaxed);
            unsafe { System.alloc(layout) }
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            LIVE.fetch_sub(layout.size() as i64, Ordering::Relaxed);
            unsafe { System.dealloc(ptr, layout) }
        }
        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            LIVE.fetch_add(new_size as i64 - layout.size() as i64, Ordering::Relaxed);
            unsafe { System.realloc(ptr, layout, new_size) }
        }
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            LIVE.fetch_add(layout.size() as i64, Ordering::Relaxed);
            unsafe { System.alloc_zeroed(layout) }
        }
    }

    pub fn bytes() -> i64 {
        LIVE.load(Ordering::Relaxed)
    }
}

#[global_allocator]
static ALLOCATOR: live::Tracking = live::Tracking;

const WORDS: [&str; 12] = [
    "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit", "sed",
    "eiusmod", "tempor", "incididunt",
];

fn corpus(lines: usize, chars_per_line: usize) -> Vec<String> {
    (0..lines)
        .map(|index| {
            let mut line = format!("{index:07}.");
            let mut word = index * 7;
            while line.len() < chars_per_line {
                line.push(' ');
                line.push_str(WORDS[word % WORDS.len()]);
                word += 1;
            }
            line
        })
        .collect()
}

fn main() {
    let lines: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(100_000);
    let chars_per_line: usize =
        std::env::args().nth(2).and_then(|a| a.parse().ok()).unwrap_or(100);
    let paragraphs = corpus(lines, chars_per_line);
    let bytes: usize = paragraphs.iter().map(|p| p.len()).sum();

    let mut font_cx = FontContext::default();
    let mut layout_cx: LayoutContext<[u8; 4]> = LayoutContext::new();

    // Stage 1+2 with the layout dropped immediately: what the work costs.
    let start = Instant::now();
    let mut build_time = std::time::Duration::ZERO;
    let mut break_time = std::time::Duration::ZERO;
    let mut width_sum = 0.0f32;
    for paragraph in &paragraphs {
        let build_start = Instant::now();
        let mut builder = layout_cx.ranged_builder(&mut font_cx, paragraph, 1.0, false);
        builder.push_default(StyleProperty::FontSize(13.0));
        builder.push_default(StyleProperty::WordBreak(parley::style::WordBreak::KeepAll));
        builder.push_default(StyleProperty::OverflowWrap(parley::style::OverflowWrap::Normal));
        builder.push_default(StyleProperty::TextWrapMode(parley::style::TextWrapMode::NoWrap));
        let mut layout = builder.build(paragraph);
        build_time += build_start.elapsed();

        let break_start = Instant::now();
        layout.break_all_lines(Some(f32::MAX));
        break_time += break_start.elapsed();

        width_sum += layout.width();
    }
    let total_dropped = start.elapsed();

    // Same again, but keeping every layout alive - the way TextParagraph does.
    let live_before = live::bytes();
    let start = Instant::now();
    let mut retained = Vec::with_capacity(lines);
    for paragraph in &paragraphs {
        let mut builder = layout_cx.ranged_builder(&mut font_cx, paragraph, 1.0, false);
        builder.push_default(StyleProperty::FontSize(13.0));
        builder.push_default(StyleProperty::WordBreak(parley::style::WordBreak::KeepAll));
        builder.push_default(StyleProperty::OverflowWrap(parley::style::OverflowWrap::Normal));
        builder.push_default(StyleProperty::TextWrapMode(parley::style::TextWrapMode::NoWrap));
        let mut layout = builder.build(paragraph);
        layout.break_all_lines(Some(f32::MAX));
        retained.push(layout);
    }
    let total_retained = start.elapsed();
    let live_after = live::bytes();

    println!(
        "{lines} paragraphs x ~{chars_per_line} chars, {:.2} MB of text (width sum {width_sum:.0})",
        bytes as f64 / 1e6
    );
    println!(
        "build (analysis+shape)  {build_time:.2?}  ({:.1} us per paragraph)",
        build_time.as_nanos() as f64 / lines as f64 / 1000.0
    );
    println!(
        "break_all_lines         {break_time:.2?}  ({:.1} us per paragraph)",
        break_time.as_nanos() as f64 / lines as f64 / 1000.0
    );
    println!(
        "total, layout dropped   {total_dropped:.2?}  ({:.1} us per paragraph)",
        total_dropped.as_nanos() as f64 / lines as f64 / 1000.0
    );
    println!(
        "total, layouts retained {total_retained:.2?}  ({:.1} us per paragraph)",
        total_retained.as_nanos() as f64 / lines as f64 / 1000.0
    );
    let retained_count = retained.len();
    drop(retained);

    // Variant: chunk several paragraphs into one parley Layout (parley handles the mandatory
    // breaks at '\n' itself). chunk = lines means one Layout for the whole document.
    for chunk in [1usize, 10, 100, lines] {
        let text: Vec<String> =
            paragraphs.chunks(chunk).map(|group| group.join("\n")).collect();
        let live_start = live::bytes();
        let start = Instant::now();
        let mut layouts = Vec::with_capacity(text.len());
        for block in &text {
            let mut builder = layout_cx.ranged_builder(&mut font_cx, block, 1.0, false);
            builder.push_default(StyleProperty::FontSize(13.0));
            builder.push_default(StyleProperty::WordBreak(parley::style::WordBreak::KeepAll));
            builder.push_default(StyleProperty::OverflowWrap(parley::style::OverflowWrap::Normal));
            builder.push_default(StyleProperty::TextWrapMode(parley::style::TextWrapMode::NoWrap));
            let mut layout = builder.build(block);
            layout.break_all_lines(Some(f32::MAX));
            layouts.push(layout);
        }
        let elapsed = start.elapsed();
        let live_end = live::bytes();
        let lines_total: usize = layouts.iter().map(|l| l.len()).sum();
        println!(
            "chunk {chunk:>6} paragraphs/layout: {elapsed:>9.2?} ({:>5.1} us per paragraph), {:>4} layouts, {lines_total} lines, {:>7.1} MB retained",
            elapsed.as_nanos() as f64 / lines as f64 / 1000.0,
            layouts.len(),
            (live_end - live_start) as f64 / (1024.0 * 1024.0)
        );
        drop(layouts);
    }

    println!(
        "retained {retained_count} layouts, one per paragraph: {:.1} MB live ({:.2} KB per paragraph)",
        (live_after - live_before) as f64 / (1024.0 * 1024.0),
        (live_after - live_before) as f64 / lines as f64 / 1024.0
    );
}


