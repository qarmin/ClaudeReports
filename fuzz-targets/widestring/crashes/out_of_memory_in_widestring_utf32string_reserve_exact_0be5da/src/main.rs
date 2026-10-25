use widestring::Utf32String;

fn main() {
    let mut utf32_string = Utf32String::from(Utf32String::default());
    utf32_string.reserve_exact(1000000000usize);
}
