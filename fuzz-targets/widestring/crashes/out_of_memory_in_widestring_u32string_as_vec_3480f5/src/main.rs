use widestring::U32String;

fn main() {
    let u32_string = U32String::with_capacity(1000000000usize);
    let _ = u32_string.as_vec();
}
