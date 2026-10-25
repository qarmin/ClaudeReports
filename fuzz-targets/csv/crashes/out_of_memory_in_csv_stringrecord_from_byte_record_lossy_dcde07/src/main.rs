use csv::ByteRecord;
use csv::StringRecord;

fn main() {
    let _ = StringRecord::from_byte_record_lossy(ByteRecord::with_capacity(0usize, 1000000000usize));
}
