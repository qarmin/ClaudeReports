use std::hint::black_box;

fn main() {
    let v0: i16 = black_box(0i16 as i16);
    let v1: i16 = black_box(1i16 as i16);
    let v2: i16 = black_box(-1i16 as i16);
    let v3: i16 = black_box(-32768i16 as i16);
    let v4: i16 = black_box(-32767i16 as i16);
    let v5: i16 = black_box(32767i16 as i16);
    let v6: i16 = black_box(32766i16 as i16);
    println!("i16->f32/0: {}", v0 as f32);
    println!("i16->f64/0: {}", v0 as f64);
    println!("i16->f32/1: {}", v1 as f32);
    println!("i16->f64/1: {}", v1 as f64);
    println!("i16->f32/2: {}", v2 as f32);
    println!("i16->f64/2: {}", v2 as f64);
    println!("i16->f32/3: {}", v3 as f32);
    println!("i16->f64/3: {}", v3 as f64);
    println!("i16->f32/4: {}", v4 as f32);
    println!("i16->f64/4: {}", v4 as f64);
    println!("i16->f32/5: {}", v5 as f32);
    println!("i16->f64/5: {}", v5 as f64);
    println!("i16->f32/6: {}", v6 as f32);
    println!("i16->f64/6: {}", v6 as f64);
}
