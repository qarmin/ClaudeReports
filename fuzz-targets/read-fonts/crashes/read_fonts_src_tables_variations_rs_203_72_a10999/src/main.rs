use read_fonts::FontData;
use read_fonts::tables::variations::TupleVariationHeader;

fn main() {
    let vec = vec![92u8, 17u8, 227u8, 177u8];
    let font_data = FontData::new(&vec);
    let tuple_variation_header = TupleVariationHeader::read(font_data, 1u16).unwrap();
    let _ = tuple_variation_header.intermediate_tuples();
}
