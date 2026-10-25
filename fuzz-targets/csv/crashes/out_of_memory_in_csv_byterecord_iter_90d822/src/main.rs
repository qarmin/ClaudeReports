use csv::ByteRecord;

fn main() {
    let byte_record = ByteRecord::with_capacity(0usize, 1000000000usize);
    let _ = byte_record.iter();
}
