use percent_encoding::AsciiSet;

fn main() {
    let ascii_set = AsciiSet::EMPTY;
    let _ = ascii_set.remove(255u8);
}
