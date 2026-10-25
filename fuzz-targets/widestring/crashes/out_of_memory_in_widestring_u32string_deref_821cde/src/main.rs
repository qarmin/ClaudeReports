use ::core::ops::Deref;
use widestring::U32String;

fn main() {
    let u32_string = U32String::with_capacity(1000000000000000000usize);
    let _ = Deref::deref(&u32_string);
}
