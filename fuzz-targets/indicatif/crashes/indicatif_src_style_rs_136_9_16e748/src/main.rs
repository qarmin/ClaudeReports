use indicatif::HumanFloatCount;
use indicatif::ProgressStyle;

fn main() {
    let human_float_count = HumanFloatCount(1.0f64);
    let string = human_float_count.to_string();
    let _ = ProgressStyle::with_template(&string).unwrap().progress_chars("");
}
