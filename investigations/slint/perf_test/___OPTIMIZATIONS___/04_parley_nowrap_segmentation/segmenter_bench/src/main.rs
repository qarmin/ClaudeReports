//! Times the two things parley's `analyze_text` does per paragraph, over the same corpus the
//! perf_test text mode generates: ICU line segmentation (useless for NoWrap text) and ICU
//! word segmentation (needed for cursor movement either way).

use std::time::Instant;

use icu_segmenter::options::{LineBreakOptions, LineBreakWordOption, WordBreakInvariantOptions};
use icu_segmenter::{LineSegmenter, WordSegmenter};

const WORDS: [&str; 12] = [
    "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit", "sed",
    "eiusmod", "tempor", "incididunt",
];

fn corpus(lines: usize) -> Vec<String> {
    (0..lines)
        .map(|index| {
            let mut line = format!("{index:07}.");
            let mut word = index * 7;
            while line.len() < 100 {
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
    let paragraphs = corpus(lines);
    let bytes: usize = paragraphs.iter().map(|p| p.len()).sum();

    // Same options parley uses for WordBreak::KeepAll, which is what Slint asks for on no-wrap.
    let mut options = LineBreakOptions::default();
    options.word_option = Some(LineBreakWordOption::KeepAll);
    // Same constructors parley uses without the `complex-scripts` feature.
    let line_segmenter = LineSegmenter::new_for_non_complex_scripts(options);
    let word_segmenter = WordSegmenter::new_for_non_complex_scripts(WordBreakInvariantOptions::default());

    let start = Instant::now();
    let mut line_breaks = 0usize;
    for paragraph in &paragraphs {
        line_breaks += line_segmenter.segment_str(paragraph).count();
    }
    let line_time = start.elapsed();

    let start = Instant::now();
    let mut word_breaks = 0usize;
    for paragraph in &paragraphs {
        word_breaks += word_segmenter.segment_str(paragraph).count();
    }
    let word_time = start.elapsed();

    println!("{lines} paragraphs, {:.2} MB", bytes as f64 / 1e6);
    println!(
        "line segmentation  {line_time:.2?}  ({} breaks, {:.0} ns per paragraph)",
        line_breaks,
        line_time.as_nanos() as f64 / lines as f64
    );
    println!(
        "word segmentation  {word_time:.2?}  ({} breaks, {:.0} ns per paragraph)",
        word_breaks,
        word_time.as_nanos() as f64 / lines as f64
    );
}
