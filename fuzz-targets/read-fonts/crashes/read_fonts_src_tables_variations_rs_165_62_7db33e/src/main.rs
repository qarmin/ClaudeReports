use read_fonts::FontData;
use read_fonts::tables::variations::TupleIndex;
use read_fonts::tables::variations::TupleVariationHeader;

fn main() {
    let vec = vec![178u8, 120u8, 171u8, 116u8];
    let font_data = FontData::new(&vec);
    let u16_v = TupleIndex::INTERMEDIATE_REGION;
    let tuple_variation_header = TupleVariationHeader::read(font_data, u16_v).unwrap();
    let _ = tuple_variation_header.peak_tuple();
}
