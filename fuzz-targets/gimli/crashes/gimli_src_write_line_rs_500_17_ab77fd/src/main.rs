use gimli::Encoding;
use gimli::Format;
use gimli::LineEncoding;
use gimli::write::LineProgram;
use gimli::write::LineString;
use gimli::write::LineStringTable;

fn main() {
    let format = Format::Dwarf64;
    let line_encoding = LineEncoding { minimum_instruction_length: 0u8, maximum_operations_per_instruction: 0u8, default_is_stmt: false, line_base: 0i8, line_range: 1u8 };
    let mut line_program = LineProgram::new(Encoding { address_size: 0u8, format: format, version: 0u16 }, line_encoding, LineString::LineStringRef(LineStringTable::default().add::<Vec<u8>>(vec![])), None, LineString::new::<Vec<u8>>(vec![], Encoding { address_size: 0u8, format: format, version: 0u16 }, &mut LineStringTable::default()), None);
    line_program.end_sequence(0u64);
}
