use gimli::write::LineString;
use gimli::write::LineStringTable;
use gimli::write::StringTable;

fn main() {
    let line_string = LineString::LineStringRef(LineStringTable::default().add::<Vec<u8>>(vec![]));
    let _ = line_string.get(&StringTable::default(), &LineStringTable::default());
}
