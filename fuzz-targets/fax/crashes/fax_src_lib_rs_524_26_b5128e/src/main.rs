use fax::Bits;

fn main() {
    let bits = Bits { data: 0u16, len: 26u8 };
    let _ = bits.to_string();
}
