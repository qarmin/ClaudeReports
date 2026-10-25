use widestring::U32String;

fn main() {
    let mut u32_string = U32String::new();
    u32_string.reserve_exact(1000000000usize);
}
