use gimli::Encoding;
use gimli::Format;
use gimli::LineEncoding;
use gimli::write::LineProgram;
use gimli::write::LineString;
use gimli::write::LineStringTable;

fn main() {
    let mut line_program = LineProgram::new(Encoding { address_size: 0u8, format: Format::Dwarf64, version: 0u16 }, LineEncoding { minimum_instruction_length: 1u8, maximum_operations_per_instruction: 10u8, default_is_stmt: false, line_base: 0i8, line_range: 1u8 }, LineString::LineStringRef(LineStringTable::default().add::<Vec<u8>>(vec![])), None, LineString::String(vec![]), None);
    line_program.end_sequence(10000000000000000000u64);
}
