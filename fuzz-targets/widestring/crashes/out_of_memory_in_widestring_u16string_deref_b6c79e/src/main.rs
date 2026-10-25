use ::core::ops::Deref;
use widestring::U16String;

fn main() {
    let u16_string = U16String::with_capacity(4294967296usize);
    let _ = Deref::deref(&u16_string);
}
