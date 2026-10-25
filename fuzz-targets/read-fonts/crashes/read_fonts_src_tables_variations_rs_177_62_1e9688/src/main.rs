use read_fonts::FontData;
use read_fonts::tables::variations::TupleVariationHeader;

fn main() {
    let vec = vec![127u8, 127u8, 127u8, 127u8];
    let tuple_variation_header = TupleVariationHeader::read(FontData::new(&vec), 1u16).unwrap();
    let tuple_variation_header2 = tuple_variation_header.shape();
    let tuple_variation_header3 = tuple_variation_header2.shape();
    let _ = tuple_variation_header3.intermediate_start_tuple();
}
