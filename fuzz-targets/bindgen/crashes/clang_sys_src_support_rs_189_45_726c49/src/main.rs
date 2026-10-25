use bindgen::Builder;

fn main() {
    let builder = Builder::default();
    let builder2 = builder.clone().header::<String>("".to_string());
    let builder3 = builder2.clone().clang_arg::<String>("\0".to_string());
    let _ = builder3.clone().generate();
}
