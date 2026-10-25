use widestring::Utf32String;

fn main() {
    let _ = Utf32String::from(Utf32String::with_capacity(1000000000usize));
}
