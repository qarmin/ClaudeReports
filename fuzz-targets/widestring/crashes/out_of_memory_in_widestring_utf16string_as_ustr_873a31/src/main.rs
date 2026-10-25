use widestring::Utf16String;

fn main() {
    let utf16_string = Utf16String::with_capacity(2147483648usize);
    let _ = utf16_string.as_ustr();
}
