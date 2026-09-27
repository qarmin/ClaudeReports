use std::hint::black_box;

fn main() {
    let v0: i128 = black_box(0i128 as i128);
    let v1: i128 = black_box(1i128 as i128);
    let v2: i128 = black_box(-1i128 as i128);
    let v3: i128 = black_box(-170141183460469231731687303715884105728i128 as i128);
    let v4: i128 = black_box(-170141183460469231731687303715884105727i128 as i128);
    let v5: i128 = black_box(170141183460469231731687303715884105727i128 as i128);
    let v6: i128 = black_box(170141183460469231731687303715884105726i128 as i128);
    println!("i128->f32/0: {}", v0 as f32);
    println!("i128->f64/0: {}", v0 as f64);
    println!("i128->f32/1: {}", v1 as f32);
    println!("i128->f64/1: {}", v1 as f64);
    println!("i128->f32/2: {}", v2 as f32);
    println!("i128->f64/2: {}", v2 as f64);
    println!("i128->f32/3: {}", v3 as f32);
    println!("i128->f64/3: {}", v3 as f64);
    println!("i128->f32/4: {}", v4 as f32);
    println!("i128->f64/4: {}", v4 as f64);
    println!("i128->f32/5: {}", v5 as f32);
    println!("i128->f64/5: {}", v5 as f64);
    println!("i128->f32/6: {}", v6 as f32);
    println!("i128->f64/6: {}", v6 as f64);
}
