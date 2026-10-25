use rspolib::FileOptions;

fn main() {
    let _ = rspolib::pofile::<rspolib::FileOptions>(FileOptions { path_or_content: "\"\u{5cac8}\"".to_string(), wrapwidth: 0usize, byte_content: None });
}
