fn main() {
    let _ = rav1e::bench::util::kmeans::<i64, 4>(&[0i64, 0i64, 0i64, i64::MAX, 0i64]);
}
