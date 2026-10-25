use krilla::tagging::fmt::Indent;

fn main() {
    let indent = Indent(usize::MAX);
    let _ = indent.to_string();
}
