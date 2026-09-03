use std::hint::black_box;

fn main() {
    { let v: u8 = black_box(0u8 as u8); let r = v as i32; println!("direct/u8->i32/0: {}", r); }
    { let v: u8 = black_box(0u8 as u8); let reinterpreted: i8 = unsafe { std::mem::transmute(v) }; let back: u8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/u8->i32/0: {}", r); }
    { let v: u8 = black_box(1u8 as u8); let r = v as i32; println!("direct/u8->i32/1: {}", r); }
    { let v: u8 = black_box(1u8 as u8); let reinterpreted: i8 = unsafe { std::mem::transmute(v) }; let back: u8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/u8->i32/1: {}", r); }
    { let v: u8 = black_box(255u8 as u8); let r = v as i32; println!("direct/u8->i32/2: {}", r); }
    { let v: u8 = black_box(255u8 as u8); let reinterpreted: i8 = unsafe { std::mem::transmute(v) }; let back: u8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/u8->i32/2: {}", r); }
    { let v: u8 = black_box(254u8 as u8); let r = v as i32; println!("direct/u8->i32/3: {}", r); }
    { let v: u8 = black_box(254u8 as u8); let reinterpreted: i8 = unsafe { std::mem::transmute(v) }; let back: u8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/u8->i32/3: {}", r); }
    { let v: u8 = black_box(128u8 as u8); let r = v as i32; println!("direct/u8->i32/4: {}", r); }
    { let v: u8 = black_box(128u8 as u8); let reinterpreted: i8 = unsafe { std::mem::transmute(v) }; let back: u8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/u8->i32/4: {}", r); }
    { let v: u8 = black_box(127u8 as u8); let r = v as i32; println!("direct/u8->i32/5: {}", r); }
    { let v: u8 = black_box(127u8 as u8); let reinterpreted: i8 = unsafe { std::mem::transmute(v) }; let back: u8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/u8->i32/5: {}", r); }
    { let v: u8 = black_box(129u8 as u8); let r = v as i32; println!("direct/u8->i32/6: {}", r); }
    { let v: u8 = black_box(129u8 as u8); let reinterpreted: i8 = unsafe { std::mem::transmute(v) }; let back: u8 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/u8->i32/6: {}", r); }
}
