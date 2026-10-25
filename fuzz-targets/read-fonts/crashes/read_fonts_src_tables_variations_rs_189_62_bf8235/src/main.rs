use read_fonts::FontData;
use read_fonts::tables::variations::TupleVariationHeader;

fn main() {
    let vec = vec![81u8, 0u8, 93u8, 0u8];
    let font_data = FontData::new(&vec);
    let tuple_variation_header = TupleVariationHeader::read(font_data, 1u16).unwrap();
    let _ = tuple_variation_header.intermediate_end_tuple();
}
