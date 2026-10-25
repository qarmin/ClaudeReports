use cpp_demangle::DemangleOptions;

fn main() {
    let demangle_options = DemangleOptions::default();
    let _ = demangle_options.recursion_limit(0u32);
}
