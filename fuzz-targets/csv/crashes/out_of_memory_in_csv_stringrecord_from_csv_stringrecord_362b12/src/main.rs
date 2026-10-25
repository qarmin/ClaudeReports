use csv::ByteRecord;
use csv::StringRecord;

fn main() {
    let _ = StringRecord::from(StringRecord::from_byte_record(ByteRecord::with_capacity(0usize, 1000000000usize)).unwrap());
}
