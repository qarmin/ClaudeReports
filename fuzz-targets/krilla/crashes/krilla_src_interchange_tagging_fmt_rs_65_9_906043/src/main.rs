use krilla::tagging::fmt::Indent;

fn main() {
    let indent = Indent(1000000000000000000usize);
    let _ = indent.to_string();
}
