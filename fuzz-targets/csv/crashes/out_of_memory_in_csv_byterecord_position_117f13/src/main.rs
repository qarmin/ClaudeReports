use csv::ByteRecord;
use csv::StringRecord;

fn main() {
    let byte_record = ByteRecord::from(StringRecord::with_capacity(1000000000000000000usize, 0usize));
    let _ = byte_record.position();
}
