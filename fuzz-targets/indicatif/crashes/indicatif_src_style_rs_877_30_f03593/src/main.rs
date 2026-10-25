use indicatif::ProgressStyle;

fn main() {
    let progress_style = ProgressStyle::default_bar();
    let progress_style2 = progress_style.clone().template("").unwrap();
    let _ = progress_style2.clone().progress_chars("\u{3ffb6}\"");
}
