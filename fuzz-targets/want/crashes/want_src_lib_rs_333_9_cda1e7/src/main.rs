fn main() {
    let mut taker = { want::new().1 };
    taker.cancel();
    taker.want();
}
