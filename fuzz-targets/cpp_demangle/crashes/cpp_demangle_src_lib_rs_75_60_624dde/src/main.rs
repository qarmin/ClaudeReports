use cpp_demangle::ParseOptions;

fn main() {
    let _ = ParseOptions::default().recursion_limit(0u32);
}
