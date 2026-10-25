use percent_encoding::AsciiSet;

fn main() {
    let ascii_set = <percent_encoding::AsciiSet as ::core::ops::Not>::not(AsciiSet::EMPTY);
    let _ = ascii_set.add(255u8);
}
