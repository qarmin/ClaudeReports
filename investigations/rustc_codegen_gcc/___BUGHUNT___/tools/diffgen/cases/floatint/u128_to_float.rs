use std::hint::black_box;

fn main() {
    let v0: u128 = black_box(0u128 as u128);
    let v1: u128 = black_box(1u128 as u128);
    let v2: u128 = black_box(340282366920938463463374607431768211455u128 as u128);
    let v3: u128 = black_box(340282366920938463463374607431768211454u128 as u128);
    let v4: u128 = black_box(170141183460469231731687303715884105728u128 as u128);
    let v5: u128 = black_box(170141183460469231731687303715884105727u128 as u128);
    let v6: u128 = black_box(170141183460469231731687303715884105729u128 as u128);
    println!("u128->f32/0: {}", v0 as f32);
    println!("u128->f64/0: {}", v0 as f64);
    println!("u128->f32/1: {}", v1 as f32);
    println!("u128->f64/1: {}", v1 as f64);
    println!("u128->f32/2: {}", v2 as f32);
    println!("u128->f64/2: {}", v2 as f64);
    println!("u128->f32/3: {}", v3 as f32);
    println!("u128->f64/3: {}", v3 as f64);
    println!("u128->f32/4: {}", v4 as f32);
    println!("u128->f64/4: {}", v4 as f64);
    println!("u128->f32/5: {}", v5 as f32);
    println!("u128->f64/5: {}", v5 as f64);
    println!("u128->f32/6: {}", v6 as f32);
    println!("u128->f64/6: {}", v6 as f64);
}
