use widestring::U16String;

fn main() {
    let _ = U16String::try_from(U16String::with_capacity(4294967295usize));
}
