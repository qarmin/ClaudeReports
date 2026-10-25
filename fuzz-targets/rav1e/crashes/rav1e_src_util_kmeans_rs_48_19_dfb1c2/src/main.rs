fn main() {
    let _ = rav1e::bench::util::kmeans::<i64, 4>(&[0i64, -1000000000000000000i64, 0i64]);
}
