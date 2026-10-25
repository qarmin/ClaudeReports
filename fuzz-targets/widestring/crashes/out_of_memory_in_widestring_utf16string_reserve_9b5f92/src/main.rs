use widestring::Utf16String;

fn main() {
    let mut utf16_string = Utf16String::with_capacity(0usize);
    utf16_string.reserve(2147483648usize);
}
