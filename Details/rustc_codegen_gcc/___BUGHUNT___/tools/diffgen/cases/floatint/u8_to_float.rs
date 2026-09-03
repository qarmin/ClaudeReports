use std::hint::black_box;

fn main() {
    let v0: u8 = black_box(0u8 as u8);
    let v1: u8 = black_box(1u8 as u8);
    let v2: u8 = black_box(255u8 as u8);
    let v3: u8 = black_box(254u8 as u8);
    let v4: u8 = black_box(128u8 as u8);
    let v5: u8 = black_box(127u8 as u8);
    let v6: u8 = black_box(129u8 as u8);
    println!("u8->f32/0: {}", v0 as f32);
    println!("u8->f64/0: {}", v0 as f64);
    println!("u8->f32/1: {}", v1 as f32);
    println!("u8->f64/1: {}", v1 as f64);
    println!("u8->f32/2: {}", v2 as f32);
    println!("u8->f64/2: {}", v2 as f64);
    println!("u8->f32/3: {}", v3 as f32);
    println!("u8->f64/3: {}", v3 as f64);
    println!("u8->f32/4: {}", v4 as f32);
    println!("u8->f64/4: {}", v4 as f64);
    println!("u8->f32/5: {}", v5 as f32);
    println!("u8->f64/5: {}", v5 as f64);
    println!("u8->f32/6: {}", v6 as f32);
    println!("u8->f64/6: {}", v6 as f64);
}
