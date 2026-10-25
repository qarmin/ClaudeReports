use widestring::U16String;

fn main() {
    let mut u16_string = U16String::new();
    u16_string.reserve_exact(10000000000000000000usize);
}
