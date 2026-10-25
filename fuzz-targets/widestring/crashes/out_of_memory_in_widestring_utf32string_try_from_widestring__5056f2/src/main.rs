use widestring::U32String;
use widestring::Utf32String;

fn main() {
    let _ = Utf32String::try_from(U32String::with_capacity(1000000000usize));
}
