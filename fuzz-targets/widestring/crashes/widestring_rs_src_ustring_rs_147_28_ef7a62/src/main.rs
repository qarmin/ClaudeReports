use widestring::U32String;

fn main() {
    let mut u32_string = U32String::from_chars(vec![]);
    u32_string.reserve(10000000000000000000usize);
}
