use harfrust::Scale;

fn main() {
    let scale = Scale::new(Some((0i32, 1000000000i32)), 1i32);
    let _ = scale.scale_y(-1000000000i32);
}
