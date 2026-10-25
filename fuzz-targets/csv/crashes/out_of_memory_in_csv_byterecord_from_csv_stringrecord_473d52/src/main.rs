use csv::ByteRecord;
use csv::StringRecord;

fn main() {
    let _ = ByteRecord::from(StringRecord::with_capacity(0usize, 1000000000usize));
}
