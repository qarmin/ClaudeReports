use widestring::U16String;

fn main() {
    let u16_string = U16String::with_capacity(2147483648usize);
    let _ = u16_string.as_ustr();
}
