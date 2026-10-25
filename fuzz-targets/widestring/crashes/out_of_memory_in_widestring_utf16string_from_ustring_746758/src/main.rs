use widestring::U16String;
use widestring::Utf16String;

fn main() {
    let _ = Utf16String::from_ustring(U16String::with_capacity(2147483648usize));
}
