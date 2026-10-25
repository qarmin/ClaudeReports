use rspolib::FileOptions;

fn main() {
    let _ = rspolib::mofile::<rspolib::FileOptions>(FileOptions { path_or_content: "".to_string(), wrapwidth: 0usize, byte_content: None });
}
