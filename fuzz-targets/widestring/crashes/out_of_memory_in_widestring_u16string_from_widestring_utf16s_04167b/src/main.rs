use widestring::U16String;
use widestring::Utf16String;

fn main() {
    let _ = U16String::from(Utf16String::with_capacity(2147483648usize));
}
