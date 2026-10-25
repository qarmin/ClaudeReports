use widestring::Utf32String;

fn main() {
    let utf32_string = Utf32String::with_capacity(1000000000usize);
    let _ = utf32_string.as_ustr();
}
