use csv::ByteRecord;

fn main() {
    let _ = ByteRecord::with_capacity(4294967296usize, 0usize);
}
