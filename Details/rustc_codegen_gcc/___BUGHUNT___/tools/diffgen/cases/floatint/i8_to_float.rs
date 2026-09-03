use std::hint::black_box;

fn main() {
    let v0: i8 = black_box(0i8 as i8);
    let v1: i8 = black_box(1i8 as i8);
    let v2: i8 = black_box(-1i8 as i8);
    let v3: i8 = black_box(-128i8 as i8);
    let v4: i8 = black_box(-127i8 as i8);
    let v5: i8 = black_box(127i8 as i8);
    let v6: i8 = black_box(126i8 as i8);
    println!("i8->f32/0: {}", v0 as f32);
    println!("i8->f64/0: {}", v0 as f64);
    println!("i8->f32/1: {}", v1 as f32);
    println!("i8->f64/1: {}", v1 as f64);
    println!("i8->f32/2: {}", v2 as f32);
    println!("i8->f64/2: {}", v2 as f64);
    println!("i8->f32/3: {}", v3 as f32);
    println!("i8->f64/3: {}", v3 as f64);
    println!("i8->f32/4: {}", v4 as f32);
    println!("i8->f64/4: {}", v4 as f64);
    println!("i8->f32/5: {}", v5 as f32);
    println!("i8->f64/5: {}", v5 as f64);
    println!("i8->f32/6: {}", v6 as f32);
    println!("i8->f64/6: {}", v6 as f64);
}
