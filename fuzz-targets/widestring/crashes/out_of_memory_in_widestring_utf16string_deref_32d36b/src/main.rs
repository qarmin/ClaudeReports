use ::core::ops::Deref;
use widestring::Utf16String;

fn main() {
    let utf16_string = Utf16String::with_capacity(4294967295usize);
    let _ = Deref::deref(&utf16_string);
}
