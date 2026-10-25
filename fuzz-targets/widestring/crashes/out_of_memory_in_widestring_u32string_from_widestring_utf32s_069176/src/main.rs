use widestring::U32String;
use widestring::Utf32String;

fn main() {
    let _ = U32String::from(Utf32String::with_capacity(1000000000usize));
}
