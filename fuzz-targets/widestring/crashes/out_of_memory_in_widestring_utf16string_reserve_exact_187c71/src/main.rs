use widestring::Utf16String;

fn main() {
    let mut utf16_string = Utf16String::new();
    utf16_string.reserve_exact(4294967295usize);
}
