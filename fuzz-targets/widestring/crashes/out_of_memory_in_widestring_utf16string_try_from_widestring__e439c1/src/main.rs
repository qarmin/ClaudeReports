use widestring::U16String;
use widestring::Utf16String;

fn main() {
    let _ = Utf16String::try_from(U16String::with_capacity(4294967296usize));
}
