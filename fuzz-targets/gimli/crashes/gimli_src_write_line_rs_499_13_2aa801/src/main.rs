use gimli::LineEncoding;
use gimli::write::LineProgram;
use gimli::write::LineString;
use gimli::write::StringTable;

fn main() {
    let mut line_program = LineProgram::new(LineProgram::none().encoding(), LineEncoding { minimum_instruction_length: 100u8, maximum_operations_per_instruction: 1u8, default_is_stmt: false, line_base: 0i8, line_range: 1u8 }, LineString::StringRef(StringTable::default().add::<Vec<u8>>(vec![])), None, LineString::StringRef(StringTable::default().add::<Vec<u8>>(vec![])), None);
    line_program.end_sequence(1u64);
}
