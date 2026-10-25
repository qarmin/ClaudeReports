use png::Filter;

fn main() {
    png::benchable_apis::unfilter(Filter::MinEntropy, 0u8, &vec![], &mut vec![]);
}
