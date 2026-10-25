use rusty_chromaprint::Configuration;
use rusty_chromaprint::Segment;

fn main() {
    let configuration = Configuration::preset_test3();
    let configuration2 = configuration.clone().with_frame_overlap(10000usize);
    let configuration3 = configuration2.clone().with_interpolation(false);
    let segment = Segment { offset1: 0usize, offset2: 0usize, items_count: 0usize, score: 0.0f64 };
    let _ = segment.start1(&configuration3);
}
