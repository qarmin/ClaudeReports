use rspolib::FileOptions;

fn main() {
    let file_options = FileOptions { path_or_content: "\"".to_string(), wrapwidth: 0usize, byte_content: None };
    let _ = rspolib::pofile::<rspolib::FileOptions>(file_options.clone());
}
