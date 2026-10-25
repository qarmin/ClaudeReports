fn main() {
    let _ = rav1e::bench::util::kmeans::<i64, 4>(&[0i64, 0i64, -1i64, -9223372036854775807i64, 0i64, 0i64]);
}
