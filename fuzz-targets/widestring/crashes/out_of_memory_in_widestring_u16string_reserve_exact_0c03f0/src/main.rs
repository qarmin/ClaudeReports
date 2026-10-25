use widestring::U16String;

fn main() {
    let mut u16_string = U16String::from(vec![]);
    u16_string.reserve_exact(2147483648usize);
}
