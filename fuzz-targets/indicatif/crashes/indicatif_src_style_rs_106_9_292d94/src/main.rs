use indicatif::ProgressStyle;

fn main() {
    let progress_style = ProgressStyle::with_template("").unwrap();
    let _ = progress_style.clone().tick_chars("");
}
