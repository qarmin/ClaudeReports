use csv::ByteRecord;
use csv::StringRecord;

fn main() {
    let string_record = StringRecord::from_byte_record(ByteRecord::from(StringRecord::with_capacity(2147483648usize, 0usize))).unwrap();
    let _ = string_record.as_byte_record();
}
