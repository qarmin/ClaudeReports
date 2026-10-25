fn main() {
    let _ = rav1e::bench::util::kmeans::<i64, 4>(&[0i64, i64::MIN, -1000000000000000000i64]);
}
