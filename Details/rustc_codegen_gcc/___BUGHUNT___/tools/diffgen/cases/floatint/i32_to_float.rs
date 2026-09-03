use std::hint::black_box;

fn main() {
    let v0: i32 = black_box(0i32 as i32);
    let v1: i32 = black_box(1i32 as i32);
    let v2: i32 = black_box(-1i32 as i32);
    let v3: i32 = black_box(-2147483648i32 as i32);
    let v4: i32 = black_box(-2147483647i32 as i32);
    let v5: i32 = black_box(2147483647i32 as i32);
    let v6: i32 = black_box(2147483646i32 as i32);
    println!("i32->f32/0: {}", v0 as f32);
    println!("i32->f64/0: {}", v0 as f64);
    println!("i32->f32/1: {}", v1 as f32);
    println!("i32->f64/1: {}", v1 as f64);
    println!("i32->f32/2: {}", v2 as f32);
    println!("i32->f64/2: {}", v2 as f64);
    println!("i32->f32/3: {}", v3 as f32);
    println!("i32->f64/3: {}", v3 as f64);
    println!("i32->f32/4: {}", v4 as f32);
    println!("i32->f64/4: {}", v4 as f64);
    println!("i32->f32/5: {}", v5 as f32);
    println!("i32->f64/5: {}", v5 as f64);
    println!("i32->f32/6: {}", v6 as f32);
    println!("i32->f64/6: {}", v6 as f64);
}
